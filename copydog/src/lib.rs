#![warn(missing_debug_implementations)]
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

pub mod config;
pub mod watch;

pub use config::{ConfigBuilder};
