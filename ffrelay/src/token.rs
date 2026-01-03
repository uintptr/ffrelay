use std::{fs, io::Write, path::PathBuf};

use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};

const FF_CONFIG_DIR: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize)]
struct TokenFile {
    token: String,
}

fn get_token_file() -> Result<PathBuf> {
    let config_dir = dirs::cache_dir().ok_or_else(|| anyhow!("unable to find cache dir"))?;

    let config_dir = config_dir.join(FF_CONFIG_DIR);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("token.json"))
}

pub fn save_token<T>(token: T) -> Result<()>
where
    T: Into<String>,
{
    let config_file = get_token_file()?;

    let data = TokenFile {
        token: token.into(),
    };

    let token_data = serde_json::to_string_pretty(&data)?;

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(config_file)?;

    f.write_all(token_data.as_bytes())?;

    Ok(())
}

pub fn find_token() -> Result<String> {
    let config_file = get_token_file()?;

    if !config_file.exists() {
        bail!("{} doesn't exist", config_file.display())
    }

    let file_data = fs::read_to_string(&config_file)?;

    let data: TokenFile = serde_json::from_str(&file_data)?;

    Ok(data.token)
}
