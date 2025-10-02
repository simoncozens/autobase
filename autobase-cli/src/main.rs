use autobase::{
    base::{BaseTable, MinMax},
    base_script,
    cjk::{self, compute_bounds},
    config, utils,
};

use anyhow::Context;
use clap::Parser;
use fontheight::{Report, Reporter};
use rayon::{iter::ParallelIterator, prelude::*};
use skrifa::raw::TableProvider;
use std::{collections::BTreeMap, fs, iter, path::PathBuf, process::ExitCode};
use write_fonts::FontBuilder;

use crate::utils::supported_scripts;
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Output TTF
    #[arg(short = 'o', long, requires = "binary")]
    output: Option<PathBuf>,

    /// The TTF(s) to analyze; if more than one is given, a single BASE table will be generated
    #[arg(required = true)]
    font_path: Vec<PathBuf>,

    /// Add min-max records for experimental Android multiscript vertical metrics
    #[arg(short = 'm', long = "min-max")]
    min_max: bool,

    /// Use hhea ascent/descent as font default min/max; otherwise use OS/2 sTypoAscender/sTypoDescender
    #[arg(short = 'u', long = "use-hhea", requires = "min_max")]
    use_hhea: bool,

    /// The number of words from each list to test
    #[arg(short = 'k', long = "words", default_value_t = 1000)]
    words_per_list: usize,

    /// Write new BASE table into font binary
    #[arg(short = 'b', long = "binary")]
    binary: bool,

    /// Configuration file
    #[arg(short = 'c', long = "config")]
    config: Option<PathBuf>,

    #[command(flatten)]
    verbosity: clap_verbosity::Verbosity<clap_verbosity::InfoLevel>,
}

fn main() -> anyhow::Result<ExitCode> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();

    let config = if let Some(config_path) = args.config.as_deref() {
        config::load_config(config_path).context("failed to load config")?
    } else {
        config::Config::default()
    };

    if args.binary && args.font_path.len() > 1 && args.output.is_some() {
        anyhow::bail!("The -o option only makes sense with a single input font");
    }

    let bases = args
        .font_path
        .iter()
        .map(|path| {
            let font_bytes = fs::read(path).context("failed to read font file")?;
            generate_base_for_font(&args, config.clone(), font_bytes)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let base = collate_bases(bases, config.tolerance);

    if args.binary {
        for font_path in args.font_path {
            let font_bytes = fs::read(&font_path).context("failed to read font file")?;
            let font = skrifa::FontRef::new(&font_bytes).context("failed to parse font file")?;
            let mut new_font = FontBuilder::new();
            new_font.add_table(&base.to_skrifa()?)?;
            new_font.copy_missing_tables(font.clone());
            let binary = new_font.build();
            let output_path = args.output.clone().unwrap_or(font_path);
            fs::write(&output_path, binary).context("failed to write font file")?;
            log::info!("Wrote font to {:?}", output_path);
        }
    } else {
        println!("{}", base.to_fea());
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::SUCCESS)
}

fn generate_base_for_font(
    args: &Args,
    config: config::Config,
    font_bytes: Vec<u8>,
) -> Result<BaseTable, anyhow::Error> {
    let reporter = Reporter::new(&font_bytes)?;
    let font = reporter.fontref();
    let locations = reporter.interesting_locations();
    let instances = locations
        .par_iter()
        .map(|location| reporter.instance(location))
        .collect::<Result<Vec<_>, _>>()
        .context("failed to initialise instances for testing")?;
    let supported = supported_scripts(font);
    log::info!(
        "Supported scripts: {}",
        supported.iter().cloned().collect::<Vec<_>>().join(", ")
    );
    let wordlists = static_lang_word_lists::LOOKUP_TABLE
        .values()
        .filter(|word_list| {
            // Filter out word lists that don't have a script in the font
            word_list
                .script()
                .map(|x| supported.contains(x))
                .unwrap_or(false)
        });
    let reports = wordlists
        // Cartesian product relevant word lists with instances
        .flat_map(|word_list| instances.iter().zip(iter::repeat(word_list)))
        .par_bridge()
        .map(|(reporter, word_list)| reporter.par_check(word_list, Some(args.words_per_list), 1))
        .collect::<Result<Vec<_>, _>>()?;
    let mut reports_by_script: BTreeMap<String, Vec<Report>> = BTreeMap::new();
    for report in reports.into_iter() {
        if let Some(script) = report.word_list.script() {
            reports_by_script
                .entry(script.to_string())
                .or_default()
                .push(report);
        }
    }
    let font_minmax = get_font_minmax(font, args.use_hhea);
    log::info!(
        "Font default min {} max {}",
        font_minmax.lowest.unwrap_or_default(),
        font_minmax.highest.unwrap_or_default(),
    );
    let mut base_script_records = if args.min_max {
        reports_by_script
            .iter()
            .flat_map(|(script, reports)| {
                base_script::base_script_record(script, reports, &config, &font_minmax)
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // If we are not writing into the binary (ie. just outputting FEA), we
    // can't use NULL MinMax values, because FEA doesn't support them. So we
    // need to replace them with the font's default min/max values.
    if !args.binary {
        for script in base_script_records.iter_mut() {
            if let Some(script_minmax) = &script.default_minmax {
                if script_minmax.is_empty() {
                    continue;
                }
                // Replace any nulls in the script default min/max
                script.default_minmax =
                    Some(script_minmax.clone().with_nulls_replaced(&font_minmax));
            }
            for (_baseline, lang) in script.languages.iter_mut() {
                *lang = lang.clone().with_nulls_replaced(&font_minmax);
            }
        }
    }

    let mut base = BaseTable::new(
        base_script_records,
        vec![], // No vertical today
    );
    let needs_cjk = supported.iter().any(|s| cjk::is_cjk_script(s));
    if needs_cjk {
        log::info!("CJK scripts detected, adding CJK BASE records");
        let cjk_bounds = compute_bounds(font)?;
        let upem = font.head()?.units_per_em() as f32;
        cjk_bounds.insert_into_base(upem, &supported, &mut base);
    }
    if !needs_cjk && !args.min_max {
        log::info!("No CJK BASE table needed, -m was not given");
    }
    Ok(base)
}

fn collate_bases(bases: Vec<BaseTable>, tolerance: Option<u16>) -> BaseTable {
    let base_iter = bases.into_iter();
    let mut first = match base_iter.clone().next() {
        Some(b) => b,
        None => return BaseTable::new(vec![], vec![]),
    };
    for b in base_iter {
        first.merge(&b, tolerance);
    }
    // Simplify the BASE table to remove redundant entries
    first.simplify(tolerance); // 5 units tolerance
    first
}

fn get_font_minmax(font: &skrifa::FontRef, use_hhea: bool) -> MinMax {
    let (ascender, descender) = if use_hhea {
        let hhea = font.hhea().unwrap();
        (hhea.ascender().to_i16(), hhea.descender().to_i16())
    } else {
        let os2 = font.os2().unwrap();
        (os2.s_typo_ascender(), os2.s_typo_descender())
    };
    MinMax::new_min_max(descender, ascender)
}
