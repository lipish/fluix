//! RUI - A GPUI-based UI Component Library
//!
//!
//! ## Features
//!
//! - **Basic Components**: Buttons, inputs, checkboxes, etc.
//! - **Form Components**: Text inputs, dropdowns, date pickers, etc.
//! - **Layout Components**: Modals, drawers, tabs, etc.
//! - **Advanced Components**: Tables, trees, charts, etc.
//!
//! ## Example
//!
//! ```rust,ignore
//! use rui::prelude::*;
//!
//! let input = TextInput::new(cx)
//!     .placeholder("Enter text...")
//!     .on_change(|value| {
//!         println!("Value: {}", value);
//!     });
//! ```

// Core modules
pub mod components;
pub mod theme;
pub mod utils;

// Re-exports for convenience
pub use components::*;
pub use theme::*;
pub use utils::*;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::components::*;
    pub use crate::theme::*;
    pub use gpui::*;
}
