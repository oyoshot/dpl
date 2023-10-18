mod config;
mod deepl;
mod directories;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::{config_file_path, DplConfig};
use deepl::deepl;
use directories::PROJECT_DIRS;

use std::process;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "Do DeepL from CLI",
    arg_required_else_help = true,
)]
struct Cli {
    /// Text to be translated
    text: Option<String>,

    /// Target language
    #[arg(short = 'l', long = "lang")]
    lang: Option<String>,

    #[clap(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Edit configuration file
    #[clap(arg_required_else_help = true)]
    Config {
        /// Initialize the configuration file
        #[clap(long, exclusive = true)]
        init: bool,

        /// Set Endpoint in configuration file
        #[clap(long)]
        endpoint: Option<String>,

        /// Set API Key in configuration file
        #[clap(long)]
        api_key: Option<String>,

        /// Set default target language in configuration file
        #[clap(long)]
        default_lang: Option<String>,
    },
}

fn run() -> Result<bool> {
    let config_file_path = config_file_path();
    let cli = Cli::parse();

    if let Some(text) = cli.text.as_deref() {
        let config = DplConfig::load_config(&config_file_path)?;
        let lang = match cli.lang.as_deref() {
            Some(_) => cli.lang.unwrap(),
            None => config.default_lang,
        };

        deepl(&config.endpoint, &config.api_key, text, &lang)?;
        return Ok(true);
    }

    match &cli.subcommand {
        Some(SubCommands::Config {
            init,
            endpoint,
            api_key,
            default_lang,
        }) => {
            if *init {
                DplConfig::generate_config(&config_file_path)?;
            }
            if let Some(endpoint) = endpoint {
                DplConfig::update_config(&config_file_path, "endpoint", endpoint)?;
            }
            if let Some(api_key) = api_key {
                DplConfig::update_config(&config_file_path, "api_key", api_key)?;
            }
            if let Some(default_lang) = default_lang {
                DplConfig::update_config(&config_file_path, "default_lang", default_lang)?;
            }
        }
        None => {}
    }

    Ok(true)
}

fn main() {
    let result = run();

    match result {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
