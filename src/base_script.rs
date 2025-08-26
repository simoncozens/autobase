//! This module determines base table BaseScriptRecords; that is,
//! script-specific vertical metrics.

use fontheight::{Report, WordExtremes};
use skrifa::Tag;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use write_fonts::tables::base::{BaseCoord, BaseScript, BaseScriptRecord, MinMax};

use crate::utils::iso15924_to_opentype;
pub fn base_script_records(reports: &[Report]) -> Vec<BaseScriptRecord> {
    println!("Writing min-max BASE script records");
    // Tag => (lowest, highest)
    let mut script_extremes = BTreeMap::<Tag, (WordExtremes, WordExtremes)>::new();
    reports
        .iter()
        .filter(|report| !report.exemplars.is_empty())
        .for_each(|report| {
            let Some(ot_script) = report.word_list.script().and_then(iso15924_to_opentype) else {
                return;
            };
            let lowest = report.exemplars.lowest().first().unwrap();
            let highest = report.exemplars.highest().first().unwrap();

            match script_extremes.entry(ot_script) {
                Entry::Vacant(entry) => {
                    entry.insert((*lowest, *highest));
                }
                Entry::Occupied(mut entry) => {
                    let (current_lowest, current_highest) = entry.get_mut();
                    *current_lowest = current_lowest.lower(*lowest);
                    *current_highest = current_highest.higher(*highest);
                }
            }
        });
    script_extremes
        .into_iter()
        .map(|(ot_script, (lowest, highest))| {
            println!(
                " Script {}: min = {} ({}), max = {} ({})",
                ot_script,
                lowest.lowest(),
                lowest.word,
                highest.highest(),
                highest.word,
            );
            BaseScriptRecord::new(
                ot_script,
                BaseScript::new(
                    None,
                    Some(MinMax::new(
                        Some(BaseCoord::format_1(lowest.lowest().floor() as i16)),
                        Some(BaseCoord::format_1(highest.highest().ceil() as i16)),
                        vec![],
                    )),
                    vec![],
                ),
            )
        })
        .collect()
}
