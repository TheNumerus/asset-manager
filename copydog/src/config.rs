//! Configuration structures
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs;

use thiserror::Error;
use toml::Value;

/// A structure used for saving configuration info
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Root watching folder
    pub source: PathBuf,
    /// File type configurations
    pub filetypes: HashMap<String, FileType>,
}

/// Used for building `Config` from toml format
///
/// `Config` can be also build manually, this serves as a convenience method for creation
pub struct ConfigBuilder {
    working_directory: Option<PathBuf>,
    toml: Option<String>
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder{
            working_directory: None,
            toml: None
        }
    }

    /// Sets the configuration data
    pub fn toml<S: AsRef<str>>(&mut self, toml: S) -> &mut Self{
        self.toml = Some(toml.as_ref().to_string());
        self
    }

    /// Sets the working directory
    ///
    /// If the `source` parameter provided in `toml` is absolute, this function does nothing
    pub fn working_dir<P: AsRef<Path>>(&mut self, wd: P) -> &mut Self {
        self.working_directory = Some(wd.as_ref().to_path_buf());
        self
    }

    /// Finishes creation of `Config`.
    ///
    /// Will create non-existent folders if needed.
    /// If there are multiple settings per extension, the setting further in the input will be used
    pub fn build(&self) -> Result<Config, ConfigError> {
        let toml = toml::from_str::<Value>(&self.toml.as_ref().unwrap_or(&String::from("source = \".\"")))?;

        let mut filetypes = HashMap::new();

        if let Value::Table(table) = toml {
            let source = table.get("source");
            let mut root_path = match source {
                Some(val) => {
                    Self::pathbuf_from_value(&val, String::from("Source must be a string"))?
                },
                None => {
                    return Err(ConfigError::InvalidStruct(String::from("Key 'source' not found.")));
                }
            };

            if !root_path.is_absolute() && self.working_directory.is_some() {
                root_path = self.working_directory.as_ref().unwrap().join(root_path);
            }

            let abs_path = fs::canonicalize(&root_path);
            let source = match abs_path {
                Ok(val) => val,
                Err(_e) => return Err(ConfigError::NotValidFolder(format!("{:?}", root_path)))
            };

            for (extension, params) in &table {
                // skip 'source'
                if extension == "source" {
                    continue;
                }
                let filetype = Self::toml_to_filetype(extension, params)?;

                filetypes.insert(extension.clone(), filetype);
            }

            Self::check_folders(&source, &mut filetypes)?;

            let config = Config{source, filetypes};

            Ok(config)
        } else {
            Err(ConfigError::InvalidStruct(String::from("File must be a table.")))
        }
    }

    fn pathbuf_from_value(value: &Value, err_message: String) -> Result<PathBuf, ConfigError> {
        if let Value::String(s) = value {
            Ok(PathBuf::from(s))
        } else {
            Err(ConfigError::InvalidStruct(err_message))
        }
    }

    fn toml_to_filetype(extension: &str, params: &Value) -> Result<FileType, ConfigError> {
        let target = &params.get("target");
        let target = match target {
            Some(val) => val,
            None => {
                return Err(ConfigError::InvalidStruct(format!("Key 'target' not found in {}.", extension)));
            }
        };
        let ignore = &params.get("ignore");

        // parse target
        let target = Self::pathbuf_from_value(target, String::from("Target must be a String."))?;

        // parse ignore
        let ignore = match ignore {
            Some(val) => {
                if let Value::Array(arr) = val {
                    let mut folders = Vec::new();
                    for v in arr {
                        let folder = Self::pathbuf_from_value(&v, String::from("Source must be a String."))?;
                        folders.push(folder);
                    }
                    Some(folders)
                } else {
                    return Err(ConfigError::InvalidStruct(String::from("Ignored folders must be an array of Strings.")));
                }
            },
            None => None
        };

        let c = FileType {
            target,
            ignore
        };
        Ok(c)
    }

    fn check_folders(source: &PathBuf, filetypes: &mut HashMap<String, FileType>) -> Result<(), ConfigError> {
        for filetype in filetypes.values_mut() {
            let target_path = source.join(&filetype.target);
            if !target_path.exists() {
                fs::create_dir(&target_path)?;
            }
            filetype.target = target_path.canonicalize()?;

            if let Some(ignored_paths) = &mut filetype.ignore {
                for ignore in ignored_paths {
                    let ignore_path = source.join(&ignore);
                    if !ignore_path.exists() {
                        fs::create_dir(&ignore_path)?;
                    }
                    *ignore = ignore_path.canonicalize()?;
                }
            }
        }
        Ok(())
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileType {
    pub target: PathBuf,
    pub ignore: Option<Vec<PathBuf>>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{message}")]
    FileError{
        message: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Unable to parse toml input")]
    ParseError{
        #[source]
        source: toml::de::Error,
    },
    #[error("Invalid config structure. {0}")]
    InvalidStruct(String),
    #[error("Path {0} is not a valid folder.")]
    NotValidFolder(String),
}

impl From<toml::de::Error> for ConfigError {
    fn from(source: toml::de::Error) -> Self {
        ConfigError::ParseError {
            source
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::FileError {
            message: String::from("File error"),
            source
        }
    }
}