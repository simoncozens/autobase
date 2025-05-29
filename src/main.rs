mod base_script;
mod cjk;
mod utils;

use anyhow::Context;
use clap::Parser;
use fontheight_core::{Reporter, SimpleLocation};
use rayon::{iter::ParallelIterator, prelude::*};
use skrifa::{FontRef, MetadataProvider};
use std::{fs, path::PathBuf, process::ExitCode};
use write_fonts::{
    FontBuilder,
    tables::base::{Axis, Base, BaseScriptList},
};

use crate::{cjk::is_cjk_script, utils::supported_scripts};
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Output TTF
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// The TTF to analyze
    font_path: PathBuf,

    /// The em-box bottom edge value to use for CJK tags; if not specified, the font's OS/2 table will be used
    #[arg(short = 'd', long = "descender")]
    descender: Option<i16>,

    /// Add min-max records for experimental Android multiscript vertical metrics
    #[arg(short = 'm', long = "min-max")]
    min_max: bool,

    /// The number of words from each list to test
    #[arg(short = 'k', long = "words", default_value_t = 1000)]
    words_per_list: usize,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> anyhow::Result<ExitCode> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    let font_bytes = fs::read(&args.font_path).context("failed to read font file")?;

    let reporter = Reporter::new(&font_bytes)?;
    // Because we can't get access to the FontRef, *or* create our own Location, all we can do is
    // look at all the "interesting" locations in the font, and then pick the default one
    // manually. But hey, minimizing visibility in library code is good programming practice,
    // who can fault it?
    let font = FontRef::new(&font_bytes).context("failed to parse font")?;
    let mut user_default_location = SimpleLocation::new();
    for axis in font.axes().iter() {
        user_default_location.insert(axis.tag().to_string(), axis.default_value());
    }
    let interesting = reporter.interesting_locations();
    let default_location = interesting
        .iter()
        .find(|loc| loc.to_simple() == user_default_location)
        .expect("no matching location found");

    let supported = supported_scripts(&font);
    println!(
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
        .par_bridge()
        .map(|word_list| -> anyhow::Result<_> {
            let report = reporter
                .par_check_location(default_location, word_list, None, 1)?
                .to_report(default_location, word_list);
            Ok(report)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut base_script_records = if args.min_max {
        base_script::base_script_records(&reports)
    } else {
        vec![]
    };
    // Add CJK tags
    let cjk_reports = reports
        .into_iter()
        .filter(|r| r.word_list.script().is_some_and(is_cjk_script))
        .collect::<Vec<_>>();
    let (base_tag_list, vertical_axis) = if !cjk_reports.is_empty() {
        cjk::add_cjk_tags(
            &mut base_script_records,
            &cjk_reports,
            &font,
            args.descender,
            &supported,
        )?
    } else {
        (None, None)
    };

    // generate the BASE table
    let base = Base::new(
        Some(Axis::new(
            base_tag_list,
            BaseScriptList::new(base_script_records),
        )),
        vertical_axis,
    );

    let mut new_font = FontBuilder::new();
    new_font.add_table(&base)?;
    new_font.copy_missing_tables(FontRef::new(&font_bytes).unwrap());
    let binary = new_font.build();
    let output_path = args.output.unwrap_or(args.font_path);
    fs::write(&output_path, binary).context("failed to write font file")?;
    println!("Wrote font to {:?}", output_path);
    Ok(ExitCode::SUCCESS)
}
