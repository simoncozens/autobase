//! This module determines base table BaseScriptRecords; that is,
//! script-specific vertical metrics.
<<<<<<< HEAD

use fontheight::{Report, WordExtremes};
use skrifa::Tag;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use write_fonts::tables::base::{BaseCoord, BaseScript, BaseScriptRecord, MinMax};

use crate::utils::iso15924_to_opentype;

fn wordlist_script_and_language(w: &WordList) -> String {
    if let Some(lang) = w.language() {
        format!("{}_{}", w.script().unwrap_or("Zzzz"), lang)
    } else {
        w.script().unwrap_or("Zzzz").to_string()
    }
}
pub fn base_script_records(
    script: &str,
    reports: &[Report],
    config: &Config,
) -> Vec<BaseScriptRecord> {
    let mut records = vec![];
    let Some(ot_script) = iso15924_to_opentype(script) else {
        log::warn!("Script {} does not have an OpenType tag, skipping", script);
        return records;
    };
    log::info!("Writing min-max BASE script records for script {}", script);
    for report in reports.iter() {
        let script_and_language = wordlist_script_and_language(report.word_list);
        let override_ = config.r#override.get(&script_and_language);
        if report.exemplars.is_empty() && override_.is_none() {
            continue;
        }
        let (highest, highest_word) = override_
            .and_then(|o| o.max)
            .map(|x| (x, "<from config>".to_string()))
            .unwrap_or_else(|| {
                let h = report.exemplars.highest().first().unwrap();
                (h.extremes.highest() as i16, h.word.to_string())
            });
        let (lowest, lowest_word) = override_
            .and_then(|o| o.min)
            .map(|x| (x, "<from config>".to_string()))
            .unwrap_or_else(|| {
                let l = report.exemplars.lowest().first().unwrap();
                (l.extremes.lowest() as i16, l.word.to_string())
            });
        log::info!(
            " Script {}: min = {} ({}), max = {} ({})",
            script,
            lowest,
            lowest_word,
            highest,
            highest_word
        );
        records.push(BaseScriptRecord::new(
            ot_script,
            BaseScript::new(
                None,
                Some(MinMax::new(
                    Some(BaseCoord::format_1(lowest)),
                    Some(BaseCoord::format_1(highest)),
                    vec![],
                )),
                vec![],
            ),
        ));
    }
    records.sort_by_key(|r| r.base_script_tag);
    records
>>>>>>> 33801f3 (Add config file, sort reports by script)
}
