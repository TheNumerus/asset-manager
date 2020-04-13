use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::path::PathBuf;

use notify::{RecommendedWatcher, watcher, DebouncedEvent, Watcher as _, RecursiveMode};
use thiserror::Error;

use crate::config::Config;

pub struct Watcher{
    watcher: RecommendedWatcher,
    handle: Option<JoinHandle<()>>,
    rx: Arc<Mutex<Receiver<DebouncedEvent>>>,
    path: PathBuf,
}

impl Watcher {
    /// Creates new watcher
    pub fn new(config: &Config) -> Self {
        let (tx, rx) = channel();

        let rx = Arc::new(Mutex::new(rx));

        let watcher = watcher(tx, Duration::from_secs(5)).unwrap();

        Watcher {
            watcher,
            handle: None,
            rx,
            path: config.source.to_owned(),
        }
    }

    /// Starts watching source folder
    ///
    /// Only watches nothing for now
    pub fn start(&mut self) -> Result<(), WatchError>{
        self.watcher.watch(&self.path, RecursiveMode::Recursive)?;
        let rx = Arc::clone(&self.rx);
        self.handle = Some(thread::spawn(move || {
            loop {
                match rx.lock().unwrap().recv() {
                    Ok(event) => {
                        match event {
                            DebouncedEvent::Create(path) => println!("file {:?} created", path),
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
                self.watcher.unwatch(&self.path)?;
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
    }
}

impl From<notify::Error> for WatchError {
    fn from(source: notify::Error) -> Self {
        WatchError::WatchError{source}
    }
}