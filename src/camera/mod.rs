//! Webcam handler. Uses a separate thread to retrieve images from the camera
//! and send them to the UI view.

use std::thread::{self, JoinHandle};

use color_eyre::Result;
use druid::{ExtEventSink, Selector, SingleUse, Target};
use nokhwa::Camera;

/// Selector name for unprocessed camera frames
pub const SELECTOR_CAMERA: &str = "CameraFrame";

/// Handler to connect to the camera and retrieve images
pub struct CameraConnector {
	event_sender: ExtEventSink,
}

impl CameraConnector {
	/// Create new camera connector with the given information.
	pub fn new(event_sender: ExtEventSink) -> Self {
		Self { event_sender }
	}

	/// Spawn and run the camera handler in a new thread.
	pub fn spawn(self) -> JoinHandle<Result<()>> {
		thread::spawn(move || self.run())
	}

	/// Run this camera handler.
	pub fn run(self) -> Result<()> {
		// TODO: configure camera
		let mut camera = Camera::new(0, None)?;
		camera.open_stream()?;

		loop {
			let frame = camera.frame()?;
			self.event_sender.submit_command(
				Selector::new(SELECTOR_CAMERA),
				SingleUse::new(frame),
				Target::Auto,
			)?;
		}
	}
}

impl std::fmt::Debug for CameraConnector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CameraConnector").field("event_sender", &"<object>").finish()
	}
}
