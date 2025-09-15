use anyhow::{Context, anyhow};
use skrifa::{
    FontRef, Tag,
    raw::{TableProvider, tables::base::MinMax},
};
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let font = std::env::args()
        .nth(1)
        .expect("Please provide a font file path");
    let font_data = std::fs::read(font).context("Failed to load font file")?;
    let font = FontRef::new(&font_data).context("Failed to parse font file")?;
    let base = font.base().context("Failed to read BASE table")?;

    let horiz_axis = base
        .horiz_axis()
        .ok_or(anyhow!("BASE table must have a horizontal axis"))??;
    let mut axes = vec![("Horizontal", horiz_axis)];
    if let Some(vert_axis) = base.vert_axis().transpose()? {
        axes.push(("Vertical", vert_axis));
    }

    for (axis_name, axis) in axes {
        let script_list = axis
            .base_script_list()
            .context("BASE table must have a horizontal script list")?;
        println!("{} axis", axis_name);
        for script in script_list.base_script_records() {
            let script_tag = script.base_script_tag();
            let base_script = script
                .base_script(script_list.offset_data())
                .context("BASE script record must have a BaseScript")?;
            if let Some(Ok(min_max)) = base_script.default_min_max() {
                dump_min_max(&script_tag, &min_max, None);
            }
            for langsys in base_script.base_lang_sys_records() {
                let lang_tag = langsys.base_lang_sys_tag();
                let min_max = langsys
                    .min_max(base_script.offset_data())
                    .context("BASE langsys record must have a MinMax")?;
                dump_min_max(&script_tag, &min_max, Some(&lang_tag));
            }
        }
    }

    //     let taglist = horiz_axis
    //     .base_tag_list()
    //     .ok_or(anyhow!("BASE table must have a horizontal tag list"))??;
    // let tags = taglist.baseline_tags();

    Ok(ExitCode::SUCCESS)
}

fn dump_min_max(script_tag: &Tag, min_max: &MinMax, lang_tag: Option<&Tag>) {
    println!(" Script {}", script_tag);
    if let Some(lang_tag) = lang_tag {
        println!("  Language {}", lang_tag);
    }
    print!("   ");
    if let Some(min) = min_max
        .min_coord()
        .map(|v| v.expect("Failed to read min coord").coordinate())
    {
        print!("Min: {:?} ", min);
    }
    if let Some(max) = min_max
        .max_coord()
        .map(|v| v.expect("Failed to read max coord").coordinate())
    {
        print!("Max: {:?}", max);
    }
    println!();
}
