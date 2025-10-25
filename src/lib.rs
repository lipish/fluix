//! RUI - A GPUI-based UI Component Library
//!
//! RUI provides a comprehensive set of UI components built on top of GPUI 0.2.
//! It aims to offer a complete, easy-to-use component collection similar to gpui-component.
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

use gpui::*;

// Core modules
pub mod theme;
pub mod components;
pub mod utils;

// Re-exports for convenience
pub use theme::*;
pub use components::*;
pub use utils::*;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::theme::*;
    pub use crate::components::*;
    pub use gpui::*;
}
