use autobase::{
    base::BaseTable,
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
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// The TTF to analyze
    font_path: PathBuf,

    /// Add min-max records for experimental Android multiscript vertical metrics
    #[arg(short = 'm', long = "min-max")]
    min_max: bool,

    /// The number of words from each list to test
    #[arg(short = 'k', long = "words", default_value_t = 1000)]
    words_per_list: usize,

    /// Emit AFDKO feature code to stdout instead of modifying the font
    #[arg(short = 'f', long = "fea")]
    fea: bool,

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

    let font_bytes = fs::read(&args.font_path).context("failed to read font file")?;

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
    // Split reports by script
    let mut reports_by_script: BTreeMap<String, Vec<Report>> = BTreeMap::new();
    for report in reports.into_iter() {
        if let Some(script) = report.word_list.script() {
            reports_by_script
                .entry(script.to_string())
                .or_default()
                .push(report);
        }
    }

    let base_script_records = if args.min_max {
        reports_by_script
            .iter()
            .flat_map(|(script, reports)| {
                base_script::base_script_records(script, reports, &config)
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // generate the BASE table
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
        return Ok(ExitCode::SUCCESS);
    }

    if args.fea {
        println!("{}", base.to_fea());
        return Ok(ExitCode::SUCCESS);
    }
    let mut new_font = FontBuilder::new();
    new_font.add_table(&base.to_skrifa()?)?;
    new_font.copy_missing_tables(font.clone());
    let binary = new_font.build();
    let output_path = args.output.unwrap_or(args.font_path);
    fs::write(&output_path, binary).context("failed to write font file")?;
    log::info!("Wrote font to {:?}", output_path);
    Ok(ExitCode::SUCCESS)
}
