//! Root crate library to use for execution and easier testing.
#![deny(unused_extern_crates)]
#![warn(
	trivial_casts,
	trivial_numeric_casts,
	missing_debug_implementations,
	unused_qualifications,
	missing_docs,
	dead_code,
	clippy::unwrap_used,
	clippy::expect_used
)]

mod camera;
#[cfg(feature = "wasm")]
pub mod wasm;
mod widgets;

use std::sync::mpsc;

use color_eyre::Result;
use druid::{AppLauncher, LocalizedString, WindowDesc};

pub use self::{
	camera::{analysis, CameraConnector},
	widgets::{root_widget, RootUIState},
};

/// Main function of this app. Must be in `lib.rs` to be used in WASM.
#[allow(clippy::expect_used)] // OK here..
pub fn run() -> Result<()> {
	color_eyre::install()?;

	let (pick_sender, pick_receiver) = mpsc::channel();

	let window = WindowDesc::new(root_widget(pick_sender))
		.title(LocalizedString::new("Window-Title").with_placeholder("Webcam Mouse"))
		.window_size((1100.0, 550.0));
	let launcher = AppLauncher::with_window(window);
	let event_sender = launcher.get_external_handle();

	let camera_handler = CameraConnector::new(event_sender, pick_receiver);
	let cam_handles = camera_handler.spawn();

	launcher.log_to_console().launch(RootUIState::default()).expect("launching window");

	cam_handles.0.join().expect("joining camera frame receiver");
	cam_handles.1.join().expect("joining camera frame processor")?;
	Ok(())
}
