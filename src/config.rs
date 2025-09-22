use std::collections::HashMap;

use serde::Deserialize;

use crate::utils::KNOWN_ISO_SCRIPTS;

#[derive(Debug, Default, Deserialize)]
pub struct Override {
    #[serde(default)]
    pub min: Option<i16>,
    #[serde(default)]
    pub max: Option<i16>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ScriptLanguage {
    pub script: String,
    pub language: Option<String>,
}

// "ef_Abcd" -> ("Abcd", Some("ef"))
impl<'de> Deserialize<'de> for ScriptLanguage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('_').rev();

        let script = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing script"))?
            .to_string();
        if !KNOWN_ISO_SCRIPTS.contains(&script.as_str()) {
            return Err(serde::de::Error::custom(format!(
                "unknown ISO 15924 script code: {}",
                script
            )));
        }
        let language = parts.next().map(|s| s.to_string());
        if let Some(lang) = &language
            && (lang.len() != 2 && lang.len() != 3)
        {
            return Err(serde::de::Error::custom(format!(
                "language code must be 2 or 3 letters: {}",
                lang
            )));
        }
        if parts.next().is_some() {
            return Err(serde::de::Error::custom(
                "too many parts, expected format: [language_]script",
            ));
        }
        Ok(ScriptLanguage { script, language })
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub r#override: HashMap<String, Override>,
    pub languages: Vec<ScriptLanguage>,
}

pub fn load_config(path: &std::path::Path) -> anyhow::Result<Config> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
