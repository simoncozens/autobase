//! This module determines base table BaseScriptRecords; that is,
//! script-specific vertical metrics.
use crate::{
    config::{Config, ScriptLanguage},
    utils::{iso639_to_opentype, iso15924_to_opentype},
};
use fontheight::{Report, WordList};
use std::collections::HashMap;
use write_fonts::tables::base::{BaseCoord, BaseScript, BaseScriptRecord};

#[derive(Clone, Debug)]
struct MinMax {
    highest: Option<i16>,
    highest_word: String,
    lowest: Option<i16>,
    lowest_word: String,
}

impl MinMax {
    fn from_report(r: Report, config: &Config) -> Option<Self> {
        let script_and_language = wordlist_script_and_language(r.word_list);
        let override_ = config.r#override.get(&script_and_language);
        // If there are no exemplars and no overrides, we can't produce a MinMax
        if r.exemplars.is_empty() && override_.is_none() {
            return None;
        }

        let (mut highest, mut highest_word) = if r.exemplars.is_empty() {
            (None, "<none>".to_string())
        } else {
            let h = r.exemplars.highest().first().unwrap();
            (Some(h.extremes.highest() as i16), h.word.to_string())
        };
        let (mut lowest, mut lowest_word) = if r.exemplars.is_empty() {
            (None, "<none>".to_string())
        } else {
            let l = r.exemplars.lowest().first().unwrap();
            (Some(l.extremes.lowest() as i16), l.word.to_string())
        };
        if let Some(ov) = override_ {
            if let Some(max) = ov.max {
                highest = Some(max);
                highest_word = "<override>".to_string();
            }
            if let Some(min) = ov.min {
                lowest = Some(min);
                lowest_word = "<override>".to_string();
            }
        }
        if highest.is_none() && lowest.is_none() {
            return None;
        }

        Some(MinMax {
            highest,
            highest_word,
            lowest,
            lowest_word,
        })
    }

    fn merge(&mut self, other: &MinMax) {
        if let Some(other_high) = other.highest
            && (self.highest.is_none() || self.highest.unwrap() < other_high)
        {
            self.highest = Some(other_high);
            self.highest_word = other.highest_word.clone();
        }
        if let Some(other_low) = other.lowest
            && (self.lowest.is_none() || self.lowest.unwrap() > other_low)
        {
            self.lowest = Some(other_low);
            self.lowest_word = other.lowest_word.clone();
        }
    }

    fn aggregate(minmaxes: &[MinMax]) -> Option<Self> {
        if minmaxes.is_empty() {
            return None;
        }
        let mut agg = minmaxes[0].clone();
        for mm in &minmaxes[1..] {
            agg.merge(mm);
        }
        Some(agg)
    }

    fn to_base(&self) -> write_fonts::tables::base::MinMax {
        write_fonts::tables::base::MinMax::new(
            self.lowest.map(BaseCoord::format_1),
            self.highest.map(BaseCoord::format_1),
            vec![],
        )
    }
}

impl std::fmt::Display for MinMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinMax<")?;
        if let Some(min) = &self.lowest {
            write!(f, " min: {:?} (from {})", min, self.lowest_word)?;
        }
        if let Some(max) = &self.highest {
            if self.lowest.is_some() {
                write!(f, ",")?;
            }
            write!(f, " max: {:?} (from {})", max, self.highest_word)?;
        }
        write!(f, ">")
    }
}

fn wordlist_script_and_language(w: &WordList) -> ScriptLanguage {
    if let Some(lang) = w.language() {
        ScriptLanguage {
            script: w.script().unwrap_or("Zzzz").to_string(),
            language: Some(lang.to_string()),
        }
    } else {
        ScriptLanguage {
            script: w.script().unwrap_or("Zzzz").to_string(),
            language: None,
        }
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
    log::debug!("Got {} reports", reports.len());
    log::debug!(
        "Reports: {:#?}",
        reports
            .iter()
            .map(|r| r.word_list.name().to_string())
            .collect::<Vec<_>>()
    );

    // We've received multiple reports for the script, which may be distinguished by language.
    // If the config specifies particular languages, we break them out of our computations.
    // (In the future, we might also automatically break out outliers.)
    let mut remaining_langs = vec![];
    let mut lang_specific_minmax: HashMap<String, MinMax> = HashMap::new();
    let mut split_languages: Vec<&String> = config
        .languages
        .iter()
        .filter(|sl| sl.script == script)
        .flat_map(|sl| sl.language.as_ref())
        .collect::<Vec<_>>();
    // Also split out anything manually overridden
    split_languages.extend(
        config
            .r#override
            .keys()
            .filter(|sl| sl.script == script)
            .flat_map(|sl| sl.language.as_ref()),
    );
    split_languages.sort();
    split_languages.dedup();
    log::debug!(" Splitting out languages: {:?}", split_languages);
    for report in reports.iter() {
        let Some(minmax) = MinMax::from_report(report.clone(), config) else {
            continue;
        };
        if let Some(lang) = report.word_list.language()
            && split_languages.contains(&&lang.to_string())
        {
            lang_specific_minmax
                .entry(lang.to_string())
                .and_modify(|existing| existing.merge(&minmax))
                .or_insert(minmax);
        } else {
            remaining_langs.push(minmax);
        }
    }

    let base_langsys_records = lang_specific_minmax
        .iter()
        .map(|(lang, mm)| {
            log::info!(" Language {}: {:?}", lang, mm);
            write_fonts::tables::base::BaseLangSysRecord::new(
                iso639_to_opentype(lang),
                mm.to_base(),
            )
        })
        .collect::<Vec<_>>();

    let script_minmax = MinMax::aggregate(&remaining_langs);
    log::info!(" Script {}: {:?}", script, script_minmax);
    records.push(BaseScriptRecord::new(
        ot_script,
        BaseScript::new(
            None,
            script_minmax.map(|s| s.to_base()),
            base_langsys_records,
        ),
    ));
    records.sort_by_key(|r| r.base_script_tag);
    records
}
