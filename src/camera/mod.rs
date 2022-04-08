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

/// Selector name for unprocessed camera frames.
pub const S_CAMERA_FRAME: &str = "CameraFrame";
/// Selector name for processed camera frames.
pub const S_PROCESSED_FRAME: &str = "CameraProcessedFrame";
/// Selector name for difference camera frames.
pub const S_DIFFERENCE_FRAME: &str = "CameraDifferenceFrame";
/// Selector name for detected point.
pub const S_CAMERA_POINT: &str = "CameraDetectedPoint";

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

		let mut previous_frame = None;
		loop {
			// Retrieve camera frame and send it.
			let current_frame = camera.frame()?;
			self.event_sender.submit_command(
				Selector::new(S_CAMERA_FRAME),
				SingleUse::new(current_frame.clone()),
				Target::Auto,
			)?;

			// Process frame to reduce noise and such and send processed image.
			let processed_frame = analysis::process_frame(current_frame);
			self.event_sender.submit_command(
				Selector::new(S_PROCESSED_FRAME),
				SingleUse::new(processed_frame.clone()),
				Target::Auto,
			)?;

			// Compare to previous frame, send diff image and send position.
			if let Some(previous) = previous_frame {
				let difference_frame = analysis::frame_difference(previous, &processed_frame);
				let point = analysis::find_right_movement(&difference_frame);
				self.event_sender.submit_command(
					Selector::new(S_DIFFERENCE_FRAME),
					SingleUse::new(difference_frame),
					Target::Auto,
				)?;

				if let Some(detected_point) = point {
					self.event_sender.submit_command(
						Selector::new(S_CAMERA_POINT),
						(detected_point.x, detected_point.y),
						Target::Auto,
					)?;
				}
			}
			previous_frame = Some(processed_frame);

			// Check if there is a signal to switch to another camera.
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
		f.debug_struct("CameraConnector")
			.field("event_sender", &"<object>")
			.field("pick_receiver", &"<object>")
			.finish()
	}
}
