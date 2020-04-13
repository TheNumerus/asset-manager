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

pub mod config;
pub use config::{ConfigBuilder};

pub mod watch;

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
