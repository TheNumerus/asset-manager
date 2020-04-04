use std::path::PathBuf;
use std::env::args;
use std::fs;

use anyhow::{Result, Context, bail};

use toml::{from_str, Value};

use thiserror::Error;

fn main() -> Result<()> {
    let config = get_config()?;

    check_folders(&config)?;

    dbg!(config);

    Ok(())
}

fn get_config() -> Result<Vec<Config>> {
    // get path to config file
    let arg = args().nth(1).context("Invalid number of arguments.")?;

    // read and parse config file
    let path = PathBuf::from(arg);
    let toml = fs::read_to_string(&path).context(format!("Unable to read config file {:?}.", path))?;
    let toml = from_str::<Value>(&toml).context("Unable to parse config file.")?;

    let mut configs = Vec::new();

    if let Value::Table(table) = toml {
        for (extension, params) in &table {
            let config = toml_to_config(extension, params, &path)?;
            configs.push(config);
        }
    } else {
        bail!(AppError::InvalidStruct(String::from("File must be a table.")));
    }
    Ok(configs)
}

fn toml_to_config(extension: &String, params: &Value, config_loc: &PathBuf) -> Result<Config> {
    let target = &params.get("target").context(format!("Key 'target' not found in {}.", extension))?;
    let ignore = &params.get("ignore");

    let config_parent = config_loc.parent().unwrap();
    
    // parse target
    let target = pathbuf_from_value(target, String::from("Target must be a String."))?;
    let target = config_parent.join(target);

    // parse ignore 
    let ignore = match ignore {
        Some(val) => {
            if let Value::Array(arr) = val {
                let mut folders = Vec::new();
                for v in arr {
                    let folder = pathbuf_from_value(&v, String::from("Source must be a String."))?;
                    // check if ignored folder exists
                    let config_parent = config_loc.parent().unwrap();
                    let folder_path = config_parent.join(&folder);
                    folders.push(folder_path);
                }
                Some(folders)
            } else {
                bail!(AppError::InvalidStruct(String::from("Ignored folders must be an array of Strings.")));
            }
        },
        None => None
    };

    let c = Config {
        extension: extension.clone(),
        target,
        ignore
    };
    Ok(c)
}

fn pathbuf_from_value(value: && Value, err_message: String) -> Result<PathBuf> {
    if let Value::String(s) = value {
        Ok(PathBuf::from(s))
    } else {
        bail!(AppError::InvalidStruct(err_message))
    }
}

fn check_folders(configs: &[Config]) -> Result<()> {
    for config in configs {
        // check target folder
        if !config.target.is_dir() {
            // try to create
            fs::create_dir(&config.target).context(format!("Unable to create target folder for {}", config.extension))?;
            eprintln!("Creating folder {:?}", config.target);
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    extension: String,
    target: PathBuf,
    ignore: Option<Vec<PathBuf>>,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Invalid config structure. {0}")]
    InvalidStruct(String),
    #[error("Path {0} is not a valid folder.")]
    NotValidFolder(String),
}