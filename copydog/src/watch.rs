use std::fs;

use notify::{RecommendedWatcher, immediate_watcher, Watcher as _, RecursiveMode, EventKind};
use thiserror::Error;

use crate::config::Config;
use std::fmt::{Debug, Formatter};

pub struct Watcher{
    watcher: Option<RecommendedWatcher>,
    config: Config,
}

impl Watcher {
    /// Creates new watcher
    pub fn new(config: Config) -> Self {
        Watcher {
            watcher: None,
            config,
        }
    }

    /// Starts watching source folder
    ///
    /// Only watches nothing for now
    pub fn start(&mut self) -> Result<(), WatchError>{
        let config = self.config.to_owned();
        let mut watcher = immediate_watcher(move |res: Result<notify::Event, notify::Error>| {
            match res {
                Ok(event) => {
                    //TODO handle error
                    handle_file(event, &config);
                },
                Err(_e) => {}
            }
        }).unwrap();
        watcher.watch(&self.config.source, RecursiveMode::Recursive)?;
        self.watcher = Some(watcher);
        Ok(())
    }

    /// Stops watching given folder
    ///
    /// Will return `Err` if called before `start`
    pub fn stop(&mut self) -> Result<(), WatchError> {
        match &mut self.watcher {
            Some(_) => {
                let mut watcher = self.watcher.take().unwrap();
                watcher.unwatch(&self.config.source)?;
                Ok(())
            },
            None => Err(WatchError::NoWatching)
        }
    }
}

impl Debug for Watcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Watcher")
            .field("config", &self.config)
            .field("watcher (is_some)", &self.watcher.is_some())
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Called stop before calling start.")]
    NoWatching,
    #[error("Watch error.")]
    WatchError{
        #[source]
        source: notify::Error
    },
    #[error("Error while copying file.")]
    CopyError{
        #[source]
        source: std::io::Error
    }
}

impl From<notify::Error> for WatchError {
    fn from(source: notify::Error) -> Self {
        WatchError::WatchError{source}
    }
}

impl From<std::io::Error> for WatchError {
    fn from(source: std::io::Error) -> Self {
        WatchError::CopyError{source}
    }
}

fn handle_file(event: notify::Event, config: &Config) -> Result<(), WatchError> {
    match event.kind{
        EventKind::Create(_) | EventKind::Modify(_) => {},
        _ => return Ok(())
    }

    if event.paths.len() == 0 {
        return Ok(());
    }

    let path = &event.paths[0];

    // ignore files with no extension
    if let None = path.extension() {
        return Ok(());
    }
    let ext = path.extension().unwrap().to_str();
    // ignore files with non UTF-8 extension
    if let None = ext {
        return Ok(());
    }
    let ext = ext.unwrap();

    let setting = if config.filetypes.contains_key(ext) {
        &config.filetypes[ext]
    } else if config.filetypes.contains_key("*") {
        &config.filetypes["*"]
    } else {
        // no rule to extension
        return Ok(());
    };

    // ignore files
    if let Some(ignored_folders)  = &setting.ignore {
        for ign in ignored_folders {
            if path.starts_with(ign) {
                return Ok(());
            }
        }
    }

    // copy file
    let new_path = setting.target.join(path.file_name().unwrap());

    fs::copy(path, &new_path)?;
    Ok(())
}