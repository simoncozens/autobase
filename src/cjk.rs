use std::{collections::HashSet, iter::once};

use fontheight_core::Report;
use skrifa::{
    MetadataProvider, Tag,
    prelude::{LocationRef, Size},
    raw::TableProvider,
};
use write_fonts::tables::base::{
    Axis, BaseCoord, BaseCoordFormat1, BaseScript, BaseScriptList, BaseScriptRecord, BaseTagList,
    BaseValues,
};

use crate::utils::{is_cjk_codepoint, iso15924_to_opentype};

pub const CJK_SCRIPTS: [&str; 5] = ["Kana", "Hani", "Bopo", "Hira", "Hang"];
pub fn is_cjk_script(s: &str) -> bool {
    CJK_SCRIPTS.iter().any(|&cjk| cjk == s)
}

fn find_lefts_rights(font: &skrifa::FontRef) -> (Option<i16>, Option<i16>) {
    let glyphmetrics = font.glyph_metrics(Size::unscaled(), LocationRef::default());
    let cjk_glyphs = font
        .charmap()
        .mappings()
        .filter(|(codepoint, _glyphid)| {
            let c = char::from_u32(*codepoint);
            c.is_some_and(is_cjk_codepoint)
        })
        .map(|(_codepoint, glyphid)| glyphid);
    let cjk_bounds = cjk_glyphs.flat_map(|gid| glyphmetrics.bounds(gid));
    let lefts = cjk_bounds.clone().map(|b| b.x_min).collect::<Vec<_>>();
    let rights = cjk_bounds.map(|b| b.x_max).collect::<Vec<_>>();
    if lefts.is_empty() || rights.is_empty() {
        return (None, None);
    }
    (
        Some((lefts.iter().copied().sum::<f32>() / lefts.len() as f32) as i16),
        Some((rights.iter().copied().sum::<f32>() / rights.len() as f32) as i16),
    )
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

    println!(
        "Setting horizontal CJK baselines: icfb = {:.0}, icft = {:.0}, ideo = {:.0}",
        icfb, icft, ideo
    );

    let (vert_icfb, vert_icft) = find_lefts_rights(font);
    if let (Some(vert_icfb), Some(vert_icft)) = (vert_icfb, vert_icft) {
        println!(
            "Setting vertical CJK baselines: icfb = {}, icft = {}",
            vert_icfb, vert_icft
        );
    } else {
        println!("No vertical CJK baselines found");
    }

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
        if let (Some(vert_icfb), Some(vert_icft)) = (vert_icfb, vert_icft) {
            vertical_script_records.push(BaseScriptRecord::new(
                tag,
                BaseScript::new(
                    Some(BaseValues::new(
                        default_index,
                        vec![
                            BaseCoord::Format1(BaseCoordFormat1::new(vert_icfb)),
                            BaseCoord::Format1(BaseCoordFormat1::new(vert_icft)),
                            BaseCoord::Format1(BaseCoordFormat1::new(0i16)),
                            BaseCoord::Format1(BaseCoordFormat1::new(-ideo)),
                        ],
                    )),
                    None,
                    vec![],
                ),
            ));
        }
    }
    // Not yet implemented

    let vertical_axis = if vertical_script_records.is_empty() {
        None
    } else {
        Some(Axis::new(
            Some(tags.clone()),
            BaseScriptList::new(vertical_script_records),
        ))
    };

    Ok((Some(tags), vertical_axis))
}
