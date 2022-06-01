#![warn(missing_docs)]
//! Build desktop app

mod context;
mod converter;
pub mod event;
pub mod hooks;
pub mod plugin;
mod protocol;
mod runner;
pub mod setting;
mod window;

pub mod prelude {
    //! This module includes plugin, settings, events, and hooks.
    pub use crate::{
        event::*,
        hooks::*,
        plugin::DioxusPlugin,
        setting::{DioxusSettings, UpdateMode},
    };
}
