use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
use toml_edit::Document;

use crate::PROJECT_DIRS;

static DEEPL_ENDPOINT_PRO: &str = "https://api.deepl.com/v2/translate";
static DEEPL_ENDPOINT_FREE: &str = "https://api-free.deepl.com/v2/translate";

pub fn config_file_path() -> PathBuf {
    PROJECT_DIRS.config_dir().join("dpl.toml")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DplConfig {
    pub endpoint: String,
    pub api_key: String,
    pub default_lang: String,
}

impl DplConfig {
    pub fn load_config(config_file_path: &PathBuf) -> Result<Self> {
        if !config_file_path.is_file() {
            return Err(anyhow!(
                "Configuration file not found at: {}",
                config_file_path.to_string_lossy()
            ));
        }

        let toml_content = fs::read_to_string(config_file_path)?;
        let s: Self = toml::from_str(&toml_content)?;

        Ok(s)
    }

    fn load_config_as_str(config_file_path: &PathBuf) -> Option<String> {
        if !config_file_path.is_file() {
            eprintln!(
                "Configuration file not found at: {}",
                config_file_path.to_string_lossy()
            );
            return None;
        }

        match fs::read_to_string(config_file_path) {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("cannot read config file as toml string. {}", e);
                return None;
            }
        }
    }

    pub fn generate_config(config_file_path: &PathBuf) -> Result<()> {
        if config_file_path.is_file() {
            println!(
                "A config file already exists at: {}",
                config_file_path.to_string_lossy()
            );

            println!("Overwrite? (y/n): ");
            io::stdout().flush()?;
            let mut desion = String::new();
            io::stdin().read_line(&mut desion)?;

            if !desion.trim().eq_ignore_ascii_case("Y") {
                return Ok(());
            }
        } else {
            let config_dir = config_file_path.parent();
            match config_dir {
                Some(path) => fs::create_dir_all(path)?,
                None => {
                    return Err(anyhow!(
                        "Unable to write config file to: {}",
                        config_file_path.to_string_lossy()
                    ));
                }
            }
        }

        println!(
            "\nFree and Pro, which option did you go with? <See, https://www.deepl.com/pro-api>
DeepL API Pro: {DEEPL_ENDPOINT_PRO}
DeepL API Free: {DEEPL_ENDPOINT_FREE}"
        );

        println!("pro/free (default) free: ");
        io::stdout().flush()?;
        let mut endpoint = String::new();
        io::stdin().read_line(&mut endpoint)?;
        let endpoint = match endpoint.trim() {
            "pro" | "Pro" => DEEPL_ENDPOINT_PRO,
            "free" | "Free" => DEEPL_ENDPOINT_FREE,
            _ => {
                return Ok(());
            }
        };

        println!(
            "\nRegister your API key. You can get it here <https://www.deepl.com/account/summary>"
        );

        println!("API key: ");
        io::stdout().flush()?;
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key)?;

        println!(
            "\nThe language into which the text should be translated. Options currently available:

BG - Bulgarian
CS - Czech
DA - Danish
DE - German
EL - Greek
EN - English (unspecified variant for backward compatibility; please select EN-GB or EN-US instead)
EN-GB - English (British)
EN-US - English (American)
ES - Spanish
ET - Estonian
FI - Finnish
FR - French
HU - Hungarian
ID - Indonesian
IT - Italian
JA - Japanese
KO - Korean
LT - Lithuanian
LV - Latvian
NB - Norwegian (Bokmål)
NL - Dutch
PL - Polish
PT - Portuguese (unspecified variant for backward compatibility; please select PT-BR or PT-PT instead)
PT-BR - Portuguese (Brazilian)
PT-PT - Portuguese (all Portuguese varieties excluding Brazilian Portuguese)
RO - Romanian
RU - Russian
SK - Slovak
SL - Slovenian
SV - Swedish
TR - Turkish
UK - Ukrainian
ZH - Chinese (simplified)

<See, https://www.deepl.com/docs-api/translate-text>"
        );

        println!("Default target language: ");
        io::stdout().flush()?;
        let mut default_lang = String::new();
        io::stdin().read_line(&mut default_lang)?;

        let config = Self {
            endpoint: endpoint.to_string(),
            api_key: api_key.trim().to_string(),
            default_lang: default_lang.trim().to_string(),
        };
        let toml = toml::to_string(&config)?;

        fs::write(&config_file_path, toml).map_err(|e| {
            anyhow!(
                "\nFailed to create config file at '{}': {}",
                config_file_path.to_string_lossy(),
                e
            )
        })?;

        println!(
            "\nSuccess! Config file written to {}",
            config_file_path.to_string_lossy()
        );

        Ok(())
    }

    pub fn update_config(config_file_path: &PathBuf, key: &str, value: &str) -> Result<()> {
        let toml_content = Self::load_config_as_str(config_file_path);
        let mut doc = toml_content.unwrap_or_default().parse::<Document>()?;
        doc[key] = toml_edit::value(value);
        File::create(config_file_path)
            .and_then(|mut file| file.write_all(doc.to_string().as_ref()))?;

        Ok(())
    }
}
