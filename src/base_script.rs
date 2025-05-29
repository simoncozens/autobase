//! This module determines base table BaseScriptRecords; that is,
//! script-specific vertical metrics.
use fontheight_core::Report;
use write_fonts::tables::base::{BaseCoord, BaseScript, BaseScriptRecord, MinMax};

use crate::utils::iso15924_to_opentype;
pub fn base_script_records(reports: &[Report]) -> Vec<BaseScriptRecord> {
    let mut records = vec![];
    println!("Writing min-max BASE script records");
    for report in reports.iter() {
        if report.exemplars.is_empty() {
            continue;
        }
        let highest = report.exemplars.highest().first().unwrap();
        let lowest = report.exemplars.lowest().first().unwrap();
        let Some(script) = report.word_list.script() else {
            continue;
        };

        println!(
            " Script {}: min = {} ({}), max = {} ({})",
            script,
            lowest.extremes.lowest(),
            lowest.word,
            highest.extremes.highest(),
            highest.word,
        );
        if let Some(ot_script) = iso15924_to_opentype(script) {
            records.push(BaseScriptRecord::new(
                ot_script,
                BaseScript::new(
                    None,
                    Some(MinMax::new(
                        Some(BaseCoord::format_1(lowest.extremes.lowest() as i16)),
                        Some(BaseCoord::format_1(highest.extremes.highest() as i16)),
                        vec![],
                    )),
                    vec![],
                ),
            ));
        }
    }
    records.sort_by_key(|r| r.base_script_tag);
    records
}
