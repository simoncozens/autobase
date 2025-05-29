use std::{collections::HashSet, iter::once};

use fontheight_core::Report;
use skrifa::{raw::TableProvider, Tag};
use write_fonts::tables::base::{
    Axis, BaseCoord, BaseCoordFormat1, BaseScript, BaseScriptList, BaseScriptRecord, BaseTagList,
    BaseValues,
};

use crate::utils::iso15924_to_opentype;

pub const CJK_SCRIPTS: [&str; 5] = ["Kana", "Hani", "Bopo", "Hira", "Hang"];
pub fn is_cjk_script(s: &str) -> bool {
    CJK_SCRIPTS.iter().any(|&cjk| cjk == s)
}

pub fn add_cjk_tags(
    base_script_records: &mut Vec<BaseScriptRecord>,
    reports: &[Report],
    font: &skrifa::FontRef,
    descender: Option<i16>,
    supported_scripts: &HashSet<&'static str>,
) -> anyhow::Result<(Option<BaseTagList>, Option<Axis>)> {
    // We need to determine:
    // icfb: Ideographic character face bottom edge - (average deepest)
    let lowests = reports
        .iter()
        .flat_map(|x| x.exemplars.lowest().first())
        .map(|x| x.extremes.lowest() as f32)
        .collect::<Vec<_>>();
    let icfb = lowests.iter().copied().sum::<f32>() / lowests.len() as f32;
    // icft: Ideographic character face top edge - (average highest)
    let heighests = reports
        .iter()
        .flat_map(|x| x.exemplars.highest().first())
        .map(|x| x.extremes.highest() as f32)
        .collect::<Vec<_>>();
    let icft = heighests.iter().copied().sum::<f32>() / heighests.len() as f32;
    // ideo: ideographic em-box bottom edge - (font descender)
    let os2 = font.os2()?;
    let mut ideo = descender.unwrap_or_else(|| os2.s_typo_descender());
    if ideo > 0 {
        ideo = -ideo;
    }

    // XXX
    let vert_icfb = icfb;
    let vert_icft = icft;

    println!(
        "Setting horizontal CJK baselines: icfb = {:.0}, icft = {:.0}, ideo = {:.0}",
        icfb, icft, ideo
    );

    // Now we make baseline tags for all of these
    let tags = BaseTagList::new(vec![
        Tag::new(b"icfb"),
        Tag::new(b"icft"),
        Tag::new(b"ideo"),
        Tag::new(b"romn"),
    ]);
    // And a vertical axis
    let mut vertical_script_records = vec![];

    for script in once("DFLT").chain(supported_scripts.iter().cloned()) {
        let tag = if script == "DFLT" {
            Tag::new(b"DFLT")
        } else {
            iso15924_to_opentype(script)
                .ok_or_else(|| anyhow::anyhow!("No OpenType tag for script: {}", script))?
        };
        // Find or create a BaseScriptRecord for this script

        let record = if let Some(i) = base_script_records
            .iter()
            .position(|b| b.base_script_tag == tag)
        {
            &mut base_script_records[i]
        } else {
            base_script_records.push(BaseScriptRecord::new(tag, BaseScript::default()));
            base_script_records.last_mut().unwrap()
        };
        let default_index = if is_cjk_script(script) || script == "DFLT" {
            2
        } else {
            3
        };
        record.base_script.base_values = Some(BaseValues::new(
            default_index,
            vec![
                BaseCoord::Format1(BaseCoordFormat1::new(icfb as i16)),
                BaseCoord::Format1(BaseCoordFormat1::new(icft as i16)),
                BaseCoord::Format1(BaseCoordFormat1::new(ideo)),
                BaseCoord::Format1(BaseCoordFormat1::new(0i16)),
            ],
        ))
        .into();
        vertical_script_records.push(BaseScriptRecord::new(
            tag,
            BaseScript::new(
                Some(BaseValues::new(
                    default_index,
                    vec![
                        BaseCoord::Format1(BaseCoordFormat1::new(vert_icfb as i16)),
                        BaseCoord::Format1(BaseCoordFormat1::new(vert_icft as i16)),
                        BaseCoord::Format1(BaseCoordFormat1::new(0i16)),
                        BaseCoord::Format1(BaseCoordFormat1::new(-ideo)),
                    ],
                )),
                None,
                vec![],
            ),
        ));
    }
    // Not yet implemented

    // let vertical_axis = Some(Axis::new(
    //     Some(tags.clone()),
    //     BaseScriptList::new(vertical_script_records),
    // ));
    let vertical_axis = None;

    Ok((Some(tags), vertical_axis))
}
