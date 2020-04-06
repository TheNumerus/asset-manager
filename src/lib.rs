//! # Copydog
//! Crate for automatic copying files between folders.
//! ## Usage
//! Easiest way to use this library is by loading a toml configuraion such as:
//! ```toml
//!  source = "."
//!
//!  [fbx]
//!  target = "../destination/models"
//!  ignore = ["ignore"]
//!
//!  [png]
//!  target = "../destination/textures"
//!
//!  ["*"]
//!  target = "../destination"
//! ```
//! ```rust
//! use copydog::ConfigBuilder;
//!
//! let toml = "...config here...";
//! let config = ConfigBuilder::new().toml(toml).build();
//! ```
use std::path::{PathBuf, Path, Component};
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};

pub mod config;
pub use config::{ConfigBuilder};

use config::Config;

pub fn watch(config: Config) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    watcher.watch(config.source, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path) => println!("file {:?} created", path),
                    _ => {}
                }
            },
            Err(_e) => eprintln!("Folder watching ended."),
        }
    }
}

/// Function from Cargo for prettier `PathBuf`
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
