//! Abstract the BASE table into a manageable structure.
//!
//! Handles both reading and writing binary BASE table data, and exporting to AFDKO feature syntax.

use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use skrifa::{FontRef, Tag};
use write_fonts::{tables::base as write_base, FontBuilder};

use crate::error::AutobaseError;

/// A MinMax represents the highest and lowest points of a set of glyphs, along with
/// the word that produced each extreme. This is useful for debugging and for
/// understanding why a particular BASE table was generated.
#[derive(Clone, Debug)]
pub struct MinMax {
    pub highest: Option<i16>,
    pub highest_word: String,
    pub lowest: Option<i16>,
    pub lowest_word: String,
}

impl MinMax {
    pub fn new_min_max(low: i16, high: i16) -> Self {
        Self {
            lowest: Some(low),
            highest: Some(high),
            lowest_word: "<from font>".to_string(),
            highest_word: "<from font>".to_string(),
        }
    }

    /// Convert to a Skrifa MinMax representation for writing to a font.
    pub fn to_skrifa(&self) -> write_base::MinMax {
        write_base::MinMax::new(
            self.lowest.map(write_base::BaseCoord::format_1),
            self.highest.map(write_base::BaseCoord::format_1),
            vec![],
        )
    }

    /// Create a MinMax from a Skrifa MinMax representation read from a font.
    fn from_skrifa(mm: &skrifa::raw::tables::base::MinMax) -> Result<Self, AutobaseError> {
        Ok(Self {
            highest: mm.max_coord().transpose()?.map(|c| c.coordinate()),
            highest_word: "<from font>".to_string(),
            lowest: mm.min_coord().transpose()?.map(|c| c.coordinate()),
            lowest_word: "<from font>".to_string(),
        })
    }

    pub fn merge(&mut self, other: &MinMax) {
        if let Some(other_high) = other.highest {
            if self.highest.is_none() || self.highest.unwrap() < other_high {
                self.highest = Some(other_high);
                self.highest_word = other.highest_word.clone();
            }
        }
        if let Some(other_low) = other.lowest {
            if self.lowest.is_none() || self.lowest.unwrap() > other_low {
                self.lowest = Some(other_low);
                self.lowest_word = other.lowest_word.clone();
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.highest.is_none() && self.lowest.is_none()
    }

    fn unset_highest(&mut self) {
        self.highest = None;
        self.highest_word = "<none>".to_string();
    }
    fn unset_lowest(&mut self) {
        self.lowest = None;
        self.lowest_word = "<none>".to_string();
    }

    pub fn with_inliers_removed(self, limits: &MinMax) -> MinMax {
        let mut new = self;
        if let (Some(high), Some(limit_high)) = (new.highest, limits.highest) {
            if high < limit_high {
                new.unset_highest();
            }
        }
        if let (Some(low), Some(limit_low)) = (new.lowest, limits.lowest) {
            if low > limit_low {
                new.unset_lowest();
            }
        }
        new
    }

    pub fn with_nulls_replaced(self, defaults: &MinMax) -> MinMax {
        let mut new = self;
        if new.highest.is_none() {
            new.highest = defaults.highest;
            new.highest_word = "<default>".to_string();
        }
        if new.lowest.is_none() {
            new.lowest = defaults.lowest;
            new.lowest_word = "<default>".to_string();
        }
        new
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

/// A BaseScript represents the BASE table data for a particular script, including
/// its default baseline, any other baselines, and MinMax data for the script as a
/// whole and for any languages within the script.
#[derive(Clone, Debug)]
pub struct BaseScript {
    /// The script tag, e.g. 'hani'
    ///
    /// Note that this is an OpenType script tag, not a ISO 15924 code.
    pub script: Tag,
    /// The default baseline tag, e.g. 'romn'
    pub default_baseline: Option<Tag>,
    /// A map of baseline tags to their y-coordinates
    pub baselines: BTreeMap<Tag, i16>,
    /// The default MinMax for the script
    pub default_minmax: Option<MinMax>,
    /// A map of language tags to their MinMax values
    ///
    /// The language tag is a 4-character OpenType language tag.
    pub languages: BTreeMap<Tag, MinMax>,
}

impl BaseScript {
    pub fn new(script: Tag) -> Self {
        Self {
            script,
            default_baseline: None,
            baselines: BTreeMap::new(),
            default_minmax: None,
            languages: BTreeMap::new(),
        }
    }

    /// Convert to a Skrifa BaseScriptRecord representation for writing to a font.
    pub fn to_skrifa(
        &self,
        baseline_tags: &[Tag],
    ) -> Result<write_base::BaseScriptRecord, AutobaseError> {
        let default_minmax = self.default_minmax.as_ref().map(|x| x.to_skrifa());
        let language_minmax: Vec<write_base::BaseLangSysRecord> = self
            .languages
            .iter()
            .map(|(lang, mm)| write_base::BaseLangSysRecord::new(*lang, mm.to_skrifa()))
            .collect();
        let baseline_index = self
            .default_baseline
            .map(|baseline_tag| {
                baseline_tags
                    .iter()
                    .position(|tag| *tag == baseline_tag)
                    .ok_or(AutobaseError::BaselineTagNotFound {
                        script: self.script,
                        tag: baseline_tag,
                    })
            })
            .transpose()?;
        let baselines: Vec<write_base::BaseCoord> = baseline_tags
            .iter()
            .map(|tag| {
                if let Some(y) = self.baselines.get(tag) {
                    write_base::BaseCoord::format_1(*y)
                } else {
                    write_base::BaseCoord::format_1(0)
                }
            })
            .collect();

        let base_values: Option<write_base::BaseValues> = baseline_index
            .map(|baseline_index| write_base::BaseValues::new(baseline_index as u16, baselines));

        Ok(write_base::BaseScriptRecord::new(
            self.script,
            write_base::BaseScript::new(base_values, default_minmax, language_minmax),
        ))
    }

    pub fn simplify(&mut self, tolerance: u16) {
        if let Some(default_minmax) = &self.default_minmax {
            // First, remove entries that are close to the default
            for value in self.languages.values_mut() {
                if let (Some(low), Some(def_low)) = (value.lowest, default_minmax.lowest) {
                    if (low - def_low).unsigned_abs() <= tolerance {
                        value.lowest = None;
                    }
                }
                if let (Some(high), Some(def_high)) = (value.highest, default_minmax.highest) {
                    if (high - def_high).unsigned_abs() <= tolerance {
                        value.highest = None;
                    }
                }
            }
            // Next remove entries that are now empty
            self.languages
                .retain(|_, v| v.lowest.is_some() || v.highest.is_some());
        }
    }

    pub fn merge(&self, other: &BaseScript) -> Self {
        let mut merged = self.clone();
        if let Some(other_def) = &other.default_minmax {
            if let Some(merged_def) = &mut merged.default_minmax {
                merged_def.merge(other_def);
            } else {
                merged.default_minmax = Some(other_def.clone());
            }
        }
        for (lang, other_mm) in &other.languages {
            if let Some(merged_mm) = merged.languages.get_mut(lang) {
                merged_mm.merge(other_mm);
            } else {
                merged.languages.insert(*lang, other_mm.clone());
            }
        }
        merged
    }
}

/// A BaseTable represents the entire BASE table, with horizontal and vertical axes.
#[derive(Clone, Debug, Default)]
pub struct BaseTable {
    /// The horizontal axis BaseScript records
    pub horizontal: Vec<BaseScript>,
    /// The vertical axis BaseScript records
    pub vertical: Vec<BaseScript>,
}

impl BaseTable {
    /// Convert to a Skrifa Base representation for writing to a font.
    pub fn to_skrifa(&self) -> Result<write_base::Base, AutobaseError> {
        let mut baseline_tags: BTreeMap<Tag, ()> = BTreeMap::new();
        for script in self.horizontal.iter().chain(self.vertical.iter()) {
            if let Some(def) = script.default_baseline {
                baseline_tags.insert(def, ());
            }
        }
        let baseline_tags: Vec<Tag> = baseline_tags.into_keys().collect();

        let mut horizontal_scripts: Vec<write_base::BaseScriptRecord> = self
            .horizontal
            .iter()
            .map(|s| s.to_skrifa(&baseline_tags))
            .collect::<Result<Vec<_>, _>>()?;
        let mut vertical_scripts: Vec<write_base::BaseScriptRecord> = self
            .vertical
            .iter()
            .map(|s| s.to_skrifa(&baseline_tags))
            .collect::<Result<Vec<_>, _>>()?;
        horizontal_scripts.sort_by_key(|r| r.base_script_tag);
        vertical_scripts.sort_by_key(|r| r.base_script_tag);

        let horizontal_axis = if !horizontal_scripts.is_empty() {
            Some(write_base::Axis::new(
                Some(write_base::BaseTagList::new(baseline_tags.clone())),
                write_base::BaseScriptList::new(horizontal_scripts),
            ))
        } else {
            None
        };
        let vertical_axis = if !vertical_scripts.is_empty() {
            Some(write_base::Axis::new(
                Some(write_base::BaseTagList::new(baseline_tags)),
                write_base::BaseScriptList::new(vertical_scripts),
            ))
        } else {
            None
        };

        Ok(write_base::Base::new(horizontal_axis, vertical_axis))
    }

    /// Export the BASE table to AFDKO feature syntax.
    pub fn to_fea(&self) -> String {
        let mut fea = "table BASE {\n".to_string();
        for (axis, scripts) in [
            ("HorizAxis", &self.horizontal),
            (" VertAxis", &self.vertical),
        ] {
            if scripts.is_empty() {
                continue;
            }
            // gather all baseline tags
            let mut baseline_tags: BTreeSet<Tag> = BTreeSet::new();
            for script in scripts.iter() {
                if let Some(def) = script.default_baseline {
                    baseline_tags.insert(def);
                }
                for lang in script.baselines.keys() {
                    baseline_tags.insert(*lang);
                }
            }
            let baseline_tags: Vec<Tag> = baseline_tags.into_iter().collect();

            // HorizAxis.BaseTagList <baseline tag>+;
            if !baseline_tags.is_empty() {
                fea.push_str(&format!(
                    " {}.BaseTagList      {};\n",
                    axis,
                    baseline_tags.iter().map(|x| x.to_string()).join(" ")
                ));

                // HorizAxis.BaseScriptList <script record> (, <script record>)*;
                // <script tag> <default baseline tag> <base coord>+
                fea.push_str(&format!(" {}.BaseScriptList ", axis));
                for script_record in scripts.iter() {
                    fea.push_str(&format!(
                        "\n    {} {}               ",
                        script_record.script,
                        script_record
                            .default_baseline
                            .unwrap_or_else(|| Tag::new(b"romn"))
                    ));
                    for tag in baseline_tags.iter() {
                        if let Some(y) = script_record.baselines.get(tag) {
                            fea.push_str(&format!("{:>4} ", y));
                        } else {
                            fea.push_str("0 ");
                        }
                    }
                    fea.pop(); // remove last space
                    fea.push(','); // separate records with commas
                }
                fea.pop(); // remove last comma
                fea.push_str(";\n");
            }
            // HorizAxis.MinMax <minmax record>;
            for script_record in scripts.iter() {
                if let Some(mm) = script_record.default_minmax.as_ref() {
                    fea.push_str(&format!(
                        " {}.MinMax {} dflt {}, {};\n",
                        axis,
                        script_record.script,
                        mm.lowest
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| "NULL".to_string()),
                        mm.highest
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| "NULL".to_string())
                    ));
                    for (lang, coord) in script_record.languages.iter() {
                        fea.push_str(&format!(
                            " {}.MinMax {} {} {}, {};\n",
                            axis,
                            script_record.script,
                            lang,
                            coord
                                .lowest
                                .map(|x| x.to_string())
                                .unwrap_or_else(|| "NULL".to_string()),
                            coord
                                .highest
                                .map(|x| x.to_string())
                                .unwrap_or_else(|| "NULL".to_string())
                        ));
                    }
                }
            }
            fea.push('\n');
        }
        fea.pop();
        fea.push_str("} BASE;\n");
        fea
    }

    fn _axis_to_base_scripts(
        axis: &skrifa::raw::tables::base::Axis,
    ) -> Result<Vec<BaseScript>, AutobaseError> {
        let script_list = axis.base_script_list()?;
        let base_tag_list: Vec<Tag> = axis
            .base_tag_list()
            .transpose()?
            .map(|b| b.baseline_tags().iter().map(|x| x.get()).collect())
            .unwrap_or(vec![]);
        let mut base_scripts = vec![];
        for script_record in script_list.base_script_records() {
            let script_tag = script_record.base_script_tag();
            let base_script = script_record.base_script(script_list.offset_data())?;
            let default_minmax = base_script
                .default_min_max()
                .transpose()?
                .map(|mm| MinMax::from_skrifa(&mm))
                .transpose()?;
            let mut languages = BTreeMap::new();
            for langsys in base_script.base_lang_sys_records() {
                let lang_tag = langsys.base_lang_sys_tag();
                let min_max = langsys.min_max(base_script.offset_data())?;
                languages.insert(lang_tag, MinMax::from_skrifa(&min_max)?);
            }
            let mut baselines = BTreeMap::new();
            let mut default_baseline_index = 0;
            if let Some(base_values) = base_script.base_values().transpose()? {
                baselines = base_values
                    .base_coords()
                    .iter()
                    .flatten()
                    .enumerate()
                    .map(|(i, coord)| (base_tag_list[i], coord.coordinate()))
                    .collect();
                default_baseline_index = base_values.default_baseline_index() as usize;
            }
            base_scripts.push(BaseScript {
                script: script_tag,
                default_baseline: base_tag_list.get(default_baseline_index).cloned(),
                baselines,
                default_minmax,
                languages,
            });
        }
        Ok(base_scripts)
    }

    /// Create a BaseTable from a Skrifa Base representation read from a font.
    pub fn from_skrifa(base: &skrifa::raw::tables::base::Base) -> Result<Self, AutobaseError> {
        Ok(Self {
            horizontal: base
                .horiz_axis()
                .transpose()?
                .map_or(Ok(vec![]), |a| Self::_axis_to_base_scripts(&a))?,
            vertical: base
                .vert_axis()
                .transpose()?
                .map_or(Ok(vec![]), |a| Self::_axis_to_base_scripts(&a))?,
        })
    }

    /// Create a new BASE table
    pub fn new(horizontal: Vec<BaseScript>, vertical: Vec<BaseScript>) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    /// Add the BASE table to a binary font, returning the new binary data.
    pub fn add_to_binary(&self, font: &FontRef) -> Result<Vec<u8>, AutobaseError> {
        let mut new_font = FontBuilder::new();
        new_font.add_table(&self.to_skrifa()?)?;
        new_font.copy_missing_tables(font.clone());
        let binary = new_font.build();
        Ok(binary)
    }

    pub fn merge(&mut self, other: &BaseTable) {
        for (my_axis, their_axis) in [
            (&mut self.horizontal, &other.horizontal),
            (&mut self.vertical, &other.vertical),
        ] {
            // For each script in other, see if we have it already
            for script in their_axis.iter() {
                // Find a matching script in self
                if let Some(my_script) = my_axis.iter().find(|s| s.script == script.script) {
                    my_script.merge(script);
                } else {
                    my_axis.push(script.clone());
                }
            }
        }
    }

    pub fn simplify(&mut self, tolerance: u16) {
        for script in self.horizontal.iter_mut().chain(self.vertical.iter_mut()) {
            script.simplify(tolerance);
        }
    }
}
