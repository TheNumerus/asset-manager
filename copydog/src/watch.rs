use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

use notify::{RecommendedWatcher, watcher, DebouncedEvent, Watcher as _, RecursiveMode};
use thiserror::Error;

use crate::config::Config;

pub struct Watcher{
    watcher: RecommendedWatcher,
    handle: Option<JoinHandle<Result<(), WatchError>>>,
    rx: Arc<Mutex<Receiver<DebouncedEvent>>>,
    config: Config,
}

impl Watcher {
    /// Creates new watcher
    pub fn new(config: Config) -> Self {
        let (tx, rx) = channel();

        let rx = Arc::new(Mutex::new(rx));

        let watcher = watcher(tx, Duration::from_secs(5)).unwrap();

        Watcher {
            watcher,
            handle: None,
            rx,
            config,
        }
    }

    /// Starts watching source folder
    ///
    /// Only watches nothing for now
    pub fn start(&mut self) -> Result<(), WatchError>{
        self.watcher.watch(&self.config.source, RecursiveMode::Recursive)?;
        let rx = Arc::clone(&self.rx);
        let config = self.config.to_owned();
        self.handle = Some(thread::spawn(move || {
            loop {
                use DebouncedEvent::*;
                match rx.lock().unwrap().recv() {
                    Ok(event) => {
                        match event {
                            Create(path) | Write(path) => {
                                handle_file(&path, &config)?;
                            },
                            // ignore other events
                            _ => {}
                        }
                    },
                    Err(_e) => eprintln!("Folder watching ended."),
                }
            }
        }));
        Ok(())
    }

    /// Stops watching given folder
    ///
    /// Will return `Err` if called before `start`
    pub fn stop(&mut self) -> Result<(), WatchError> {
        match &self.handle {
            Some(_) => {
                self.watcher.unwatch(&self.config.source)?;
                let mut empty = None;
                std::mem::swap(&mut self.handle, &mut empty);
                // TODO fix this
                //handle.unwrap().join().unwrap();
                Ok(())
            },
            None => Err(WatchError::NoWatching)
        }
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

fn handle_file(path: &PathBuf, config: &Config) -> Result<(), WatchError> {
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