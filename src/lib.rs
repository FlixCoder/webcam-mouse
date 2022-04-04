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

mod camera;
mod widgets;

pub use self::{
	camera::CameraConnector,
	widgets::{root_widget, RootUIState},
};
