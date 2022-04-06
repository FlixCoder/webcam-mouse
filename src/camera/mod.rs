//! Webcam handler. Uses a separate thread to retrieve images from the camera
//! analyze them and send them to the UI view.

mod analysis;

use std::{
	sync::mpsc,
	thread::{self, JoinHandle},
};

use color_eyre::Result;
use druid::{ExtEventSink, Selector, SingleUse, Target};
use nokhwa::Camera;

/// Selector name for unprocessed camera frames
pub const SELECTOR_CAMERA: &str = "CameraFrame";

/// Camera picker index receiver
pub type PickReceiver = mpsc::Receiver<usize>;

/// Handler to connect to the camera and retrieve images
pub struct CameraConnector {
	event_sender: ExtEventSink,
	pick_receiver: PickReceiver,
}

impl CameraConnector {
	/// Create new camera connector with the given information.
	pub fn new(event_sender: ExtEventSink, cam_pick_receiver: PickReceiver) -> Self {
		Self { event_sender, pick_receiver: cam_pick_receiver }
	}

	/// Spawn and run the camera handler in a new thread.
	pub fn spawn(self) -> JoinHandle<()> {
		thread::spawn(move || self.run().expect("running camera handler"))
	}

	/// Run this camera handler.
	pub fn run(self) -> Result<()> {
		let mut camera = Camera::new(0, None)?;
		camera.open_stream()?;

		loop {
			let frame = camera.frame()?;
			self.event_sender.submit_command(
				Selector::new(SELECTOR_CAMERA),
				SingleUse::new(frame),
				Target::Auto,
			)?;

			match self.pick_receiver.try_recv() {
				Ok(index) => {
					camera.stop_stream()?;
					camera.set_index(index)?;
					camera.open_stream()?;
				}
				Err(mpsc::TryRecvError::Disconnected) => break,
				_ => {}
			}
		}
		Ok(())
	}
}

impl std::fmt::Debug for CameraConnector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CameraConnector").field("event_sender", &"<object>").finish()
	}
}
