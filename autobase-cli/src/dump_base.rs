use anyhow::Context;
use autobase::base::BaseTable;
use skrifa::{raw::TableProvider, FontRef};
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let font = std::env::args()
        .nth(1)
        .expect("Please provide a font file path");
    let font_data = std::fs::read(font).context("Failed to load font file")?;
    let font = FontRef::new(&font_data).context("Failed to parse font file")?;
    let base = font.base().context("Failed to read BASE table")?;
    let our_base = BaseTable::from_skrifa(&base)?;
    println!("{}", our_base.to_fea());
    Ok(ExitCode::SUCCESS)
}
