//! Root crate library to use for execution and easier testing.
#![deny(unused_extern_crates)]
#![warn(
	trivial_casts,
	trivial_numeric_casts,
	missing_debug_implementations,
	unused_qualifications,
	missing_docs,
	dead_code
)]

mod widgets;

pub use self::widgets::{root_widget, RootUIState};

// TODO:
// - Camera picker combo box
// - Camera view widget
// - Camera image processing to find movement
// - Selection to toggle image view: real, processed or movement image
