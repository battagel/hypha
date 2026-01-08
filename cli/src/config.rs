use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::constants::PROJECT_NAME;

/// Config file stored in home directory
fn config_filename() -> String {
    format!(".{}", PROJECT_NAME)
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Root directory for notes (required)
    pub root_dir: Option<PathBuf>,
}

impl Config {
    /// Load config from ~/.hypha
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save config to ~/.hypha
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        Ok(())
    }

    /// Get the config file path (~/.hypha)
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(config_filename()))
    }

    /// Get root_dir, running interactive setup if not configured
    pub fn require_root_dir(&self) -> Result<PathBuf> {
        match &self.root_dir {
            Some(path) => {
                if !path.exists() {
                    println!(
                        "Configured root directory does not exist: {}",
                        path.display()
                    );
                    println!("Creating it now...");
                    fs::create_dir_all(path)?;
                }
                Ok(path.clone())
            }
            None => {
                // This shouldn't happen if ensure_configured was called
                println!("No root directory configured. Running setup...\n");
                let config = interactive_setup()?;
                config.require_root_dir()
            }
        }
    }
}

/// Ensure config exists, running interactive setup if needed
pub fn ensure_configured() -> Result<Config> {
    let config = Config::load()?;

    if config.root_dir.is_none() {
        println!("Welcome to Hypha! Let's get you set up.\n");
        return interactive_setup();
    }

    Ok(config)
}

/// Run interactive setup to create config
pub fn interactive_setup() -> Result<Config> {
    // Suggest a default path
    let default_path = dirs::home_dir()
        .map(|h| h.join(PROJECT_NAME))
        .unwrap_or_else(|| PathBuf::from(format!("./{}", PROJECT_NAME)));

    print!(
        "Where would you like to store your notes? [{}]: ",
        default_path.display()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    let path = if input.is_empty() {
        default_path
    } else {
        let path = PathBuf::from(input);
        if path.is_absolute() {
            path
        } else {
            std::env::current_dir()?.join(path)
        }
    };

    // Create directory if needed
    if !path.exists() {
        fs::create_dir_all(&path)?;
        println!("Created directory: {}", path.display());
    }

    // Create default template file if it doesn't exist
    let template_path = path.join(crate::constants::TEMPLATE_FILE);
    if !template_path.exists() {
        fs::write(&template_path, crate::constants::DEFAULT_TEMPLATE)?;
        println!("Created template: {}", template_path.display());
    }

    let config = Config {
        root_dir: Some(path.clone()),
    };
    config.save()?;

    println!(
        "\nConfiguration saved to: {}",
        Config::config_path()?.display()
    );
    println!("Notes directory: {}", path.display());
    println!();

    Ok(config)
}
