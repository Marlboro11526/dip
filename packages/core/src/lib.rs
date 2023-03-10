//! Shared resources across platforms

pub mod schedule;
pub mod ui_state;

pub mod config {
    pub use dip_macro::ConfigPlugin;
}
pub use dip_task as task;

pub mod prelude {
    pub use crate::{
        schedule::{DipStage, DipStartupStage, UiSchedulePlugin},
        task::prelude::*,
        ui_state::{NoRootProps, NoUiAction, NoUiState, UiStateHandler},
    };
}
