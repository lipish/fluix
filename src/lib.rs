//! Fluix - A GPUI-based UI Component Library
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
//! use fluix::prelude::*;
//!
//! let input = TextInput::new(cx)
//!     .placeholder("Enter text...")
//!     .on_change(|value| {
//!         println!("Value: {}", value);
//!     });
//! ```

// Core modules
pub mod ai;
pub mod assets;
pub mod components;
pub mod theme;
pub mod utils;

// Re-exports for convenience
pub use ai::*;
pub use assets::Assets;
// Note: components::* includes layout module, which conflicts with ai::layout
// Import components individually to avoid conflicts
pub use components::basic::*;
pub use components::form::*;
// Advanced module is empty, skip it
// pub use components::advanced::*;
// Layout is imported separately to avoid conflict with ai::layout
pub use components::layout as components_layout;
pub use components::layout::*;
pub use theme::*;
pub use utils::*;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::ai::*;
    // Import components individually to avoid layout name conflict
    pub use crate::components::basic::*;
    pub use crate::components::form::*;
    // Advanced module is empty, skip it
    // pub use crate::components::advanced::*;
    pub use crate::components::layout::*;
    pub use crate::theme::*;
    pub use gpui::*;
}
