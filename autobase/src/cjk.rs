use std::collections::HashSet;

use crate::{
    base::{BaseScript, BaseTable},
    error::AutobaseError,
    utils::iso15924_to_opentype,
};
use skrifa::{
    metrics::BoundingBox,
    prelude::{LocationRef, Size},
    raw::TableProvider,
    GlyphId, MetadataProvider, Tag,
};

// To let the function work with both ISO and OpenType script tags, we include both
pub const CJK_SCRIPTS: [&str; 10] = [
    "Kana", "Hani", "Bopo", "Hira", "Hang", "kana", "hani", "bopo", "hira", "hang",
];
pub fn is_cjk_script(s: &str) -> bool {
    CJK_SCRIPTS.contains(&s)
}

/// CJK vertical metrics, as per the Google Fonts vertical metrics specification.
///
/// See https://googlefonts.github.io/gf-guide/metrics.html#cjk-vertical-metrics for how these are determined.
#[derive(Debug, Clone)]
pub struct CjkMetrics {
    /// Ideographic character face bottom edge
    h_icfb: Option<f32>,
    /// Ideographic character face top edge
    h_icft: Option<f32>,
    /// Ideographic em-box bottom edge
    h_ideo: Option<f32>,
    /// Ideographic em-box top edge
    h_idtp: Option<f32>,
    /// Roman baseline
    h_romn: Option<f32>,

    /// Ideographic character face left edge
    v_icfb: Option<f32>,
    /// Ideographic character face right edge
    v_icft: Option<f32>,
    /// Ideographic em-box left edge
    v_ideo: Option<f32>,
    /// Ideographic em-box right edge (advance width)
    v_idtp: Option<f32>,
    /// Vertical roman baseline
    v_romn: Option<f32>,
}

impl CjkMetrics {
    fn from_bounds(bounds: &[BoundingBox], upem: f32, average_width: f32) -> Self {
        let bbox_y_average = bounds
            .iter()
            .map(|b| (b.y_max + b.y_min) / 2.0)
            .sum::<f32>()
            / bounds.len() as f32;
        let h_idtp = bbox_y_average + upem / 2.0;
        let h_ideo = bbox_y_average - upem / 2.0;
        let average_top = bounds.iter().map(|b| b.y_max).sum::<f32>() / bounds.len() as f32;
        let average_bottom = bounds.iter().map(|b| b.y_min).sum::<f32>() / bounds.len() as f32;
        let average_left = bounds.iter().map(|b| b.x_min).sum::<f32>() / bounds.len() as f32;
        let average_right = bounds.iter().map(|b| b.x_max).sum::<f32>() / bounds.len() as f32;

        CjkMetrics {
            h_icfb: Some(average_bottom),
            h_icft: Some(average_top),
            h_ideo: Some(h_ideo),
            h_idtp: Some(h_idtp),
            h_romn: Some(0.0),
            v_icfb: Some(average_left),
            v_icft: Some(average_right),
            v_ideo: Some(0.0),
            v_idtp: Some(average_width),
            v_romn: Some(-h_ideo),
        }
    }

    pub fn insert_into_base(
        &self,
        upem: f32,
        supported_scripts: &HashSet<&str>,
        base: &mut BaseTable,
    ) {
        let average_width = self.v_idtp.unwrap();
        let font_is_square = (average_width - upem).abs() / upem < 0.01;
        // get all the supported scripts; if they're not already in the base table, add them
        // for each script, the default baseline should be ideo if it's a CJK script, romn otherwise
        // we want to add the following baseline: icfb, icft, ideo, romn; idtp only if the font is not square

        // supported_scripts is expected to be ISO scripts, convert them to OT
        for ot_script in supported_scripts
            .iter()
            .flat_map(|s| iso15924_to_opentype(s))
        {
            let default_baseline = if is_cjk_script(&ot_script.to_string()) {
                Tag::new(b"ideo")
            } else {
                Tag::new(b"romn")
            };
            // Find a horizontal basescript record for this script, or create one
            let h_basescript =
                if let Some(bs) = base.horizontal.iter_mut().find(|bs| bs.script == ot_script) {
                    bs
                } else {
                    base.horizontal.push(BaseScript::new(ot_script));
                    base.horizontal.last_mut().unwrap()
                };
            h_basescript.default_baseline = Some(default_baseline);
            let hbaselines = &mut h_basescript.baselines;
            if let Some(icfb) = self.h_icfb {
                hbaselines.insert(Tag::new(b"icfb"), icfb as i16);
            }
            if let Some(icft) = self.h_icft {
                hbaselines.insert(Tag::new(b"icft"), icft as i16);
            }
            if let Some(ideo) = self.h_ideo {
                hbaselines.insert(Tag::new(b"ideo"), ideo as i16);
            }
            if let Some(romn) = self.h_romn {
                hbaselines.insert(Tag::new(b"romn"), romn as i16);
            }
            if !font_is_square {
                if let Some(idtp) = self.h_idtp {
                    hbaselines.insert(Tag::new(b"idtp"), idtp as i16);
                }
            }
            // Find a vertical basescript record for this script, or create one
            let v_basescript =
                if let Some(bs) = base.vertical.iter_mut().find(|bs| bs.script == ot_script) {
                    bs
                } else {
                    base.vertical.push(BaseScript::new(ot_script));
                    base.vertical.last_mut().unwrap()
                };
            v_basescript.default_baseline = Some(default_baseline);
            let vbaselines = &mut v_basescript.baselines;
            if let Some(icfb) = self.v_icfb {
                vbaselines.insert(Tag::new(b"icfb"), icfb as i16);
            }
            if let Some(icft) = self.v_icft {
                vbaselines.insert(Tag::new(b"icft"), icft as i16);
            }
            if let Some(ideo) = self.v_ideo {
                vbaselines.insert(Tag::new(b"ideo"), ideo as i16);
            }
            if let Some(romn) = self.v_romn {
                vbaselines.insert(Tag::new(b"romn"), romn as i16);
            }
            if !font_is_square {
                if let Some(idtp) = self.v_idtp {
                    vbaselines.insert(Tag::new(b"idtp"), idtp as i16);
                }
            }
        }
    }
}

fn cjk_glyphs(f: &skrifa::FontRef) -> Vec<GlyphId> {
    let mut cjk_glyphs = f
        .charmap()
        .mappings()
        .filter(|(cp, _gid)| {
            // We're going to be using this to find the ideographic bounding
            // box, so we're only interesting in Han/Kanji. In some designs,
            // kana, enclosed characters, etc. may be taller than the
            // ideographic bounding box, so we exclude them.
            (0x4E00..0x9FFF).contains(cp)
            || (0x3400..0x4DBF).contains(cp) // CJK Unified Ideographs Extension A
            || (0x20000..0x2A6DF).contains(cp) // CJK Unified Ideographs Extension B
        })
        .map(|(_cp, gid)| gid)
        .collect::<Vec<_>>();
    if cjk_glyphs.is_empty() {
        // Maybe just a Korean or Kana font?
        cjk_glyphs = f
            .charmap()
            .mappings()
            .filter(
                |(cp, _gid)|
                // Korean Hangul syllables
                (0xAC00..=0xD7AF).contains(cp)
                || // Kana characters
                (0x3040..=0x30FF).contains(cp)
                || (0xFF00..=0xFFEF).contains(cp), // Full-width Kana
            )
            .map(|(_cp, gid)| gid)
            .collect();
    }
    cjk_glyphs
}

pub fn compute_bounds(f: &skrifa::FontRef) -> Result<CjkMetrics, AutobaseError> {
    let upem = f.head()?.units_per_em() as f32;
    let glyph_metrics = f.glyph_metrics(Size::unscaled(), LocationRef::default());
    let hmtx = f.hmtx()?;
    let relevant_glyphs = cjk_glyphs(f);
    let average_width = relevant_glyphs
        .iter()
        .map(|&gid| hmtx.advance(gid).map(|x| x as f32).unwrap_or(upem)) // Promote to f32 to avoid overflow
        .sum::<f32>()
        / relevant_glyphs.len() as f32;
    Ok(CjkMetrics::from_bounds(
        &relevant_glyphs
            .iter()
            .filter_map(|&gid| glyph_metrics.bounds(gid))
            .collect::<Vec<_>>(),
        upem,
        average_width,
    ))
}
