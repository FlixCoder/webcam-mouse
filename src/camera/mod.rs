//! Webcam handler. Uses separate threads to retrieve images from the camera,
//! analyze them and send them to the UI view.

pub mod analysis;

use std::{
	sync::mpsc,
	thread::{self, JoinHandle},
	time::Instant,
};

use color_eyre::Result;
use druid::{ExtEventSink, Selector, SingleUse, Target};
use image::RgbImage;
use nokhwa::Camera;

/// Selector name for unprocessed camera frames.
pub const S_CAMERA_FRAME: &str = "CameraFrame";
/// Selector name for processed camera frames.
pub const S_PROCESSED_FRAME: &str = "CameraProcessedFrame";
/// Selector name for difference camera frames.
pub const S_DIFFERENCE_FRAME: &str = "CameraDifferenceFrame";
/// Selector name for detected point.
pub const S_CAMERA_POINT: &str = "CameraDetectedPoint";
/// Selector name for camera FPS.
pub const S_CAMERA_FPS: &str = "CameraFPS";

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
	pub fn spawn(self) -> (JoinHandle<()>, JoinHandle<Result<()>>) {
		let (frame_sender, frame_receiver) = mpsc::sync_channel(2);

		let mut pick_receiver = self.pick_receiver;
		let frame_receiver_handle = thread::spawn(move || {
			let mut cam_index = 0;
			while let Err(err) =
				Self::run_frame_receiver(&mut pick_receiver, cam_index, frame_sender.clone())
			{
				eprintln!("Error running camera handler: {err}");
				match pick_receiver.recv() {
					Ok(index) => cam_index = index,
					Err(mpsc::RecvError) => break,
				}
			}
		});

		let mut event_sender = self.event_sender;
		let frame_processor_handle =
			thread::spawn(move || Self::run_frame_processor(frame_receiver, &mut event_sender));

		(frame_receiver_handle, frame_processor_handle)
	}

	/// Run the frame receiver for this camera handler.
	fn run_frame_receiver(
		pick_receiver: &mut PickReceiver,
		start_index: usize,
		frame_sender: mpsc::SyncSender<RgbImage>,
	) -> Result<()> {
		let mut camera = Camera::new(start_index, None)?;
		camera.open_stream()?;

		loop {
			// Retrieve camera frame and send it to the processor
			let current_frame = camera.frame()?;
			frame_sender.send(current_frame)?;

			// Check if there is a signal to switch to another camera.
			match pick_receiver.try_recv() {
				Ok(index) => {
					camera.stop_stream()?;
					camera.set_index(index)?;
					// Does `camera.set_index` keep the camera format?
					// camera = Camera::new(index, None)?;
					camera.open_stream()?;
				}
				Err(mpsc::TryRecvError::Disconnected) => break,
				_ => {}
			}
		}
		Ok(())
	}

	/// Run image processor with the given event sender
	fn run_frame_processor(
		frame_receiver: mpsc::Receiver<RgbImage>,
		event_sender: &mut ExtEventSink,
	) -> Result<()> {
		let mut previous_frame: Option<RgbImage> = None;
		let mut timer = Instant::now();
		while let Ok(mut current_frame) = frame_receiver.recv() {
			// Process the frame to reduce noise and such.
			analysis::flip_in_place(&mut current_frame);
			let processed_frame = analysis::process_frame(&current_frame);

			// Send original and processed image.
			event_sender.submit_command(
				Selector::new(S_CAMERA_FRAME),
				SingleUse::new(current_frame),
				Target::Auto,
			)?;
			event_sender.submit_command(
				Selector::new(S_PROCESSED_FRAME),
				SingleUse::new(processed_frame.clone()),
				Target::Auto,
			)?;

			// Compare to previous frame, send diff image and send position.
			if let Some(mut previous) = previous_frame {
				if previous.dimensions() == processed_frame.dimensions() {
					analysis::frame_difference(&mut previous, &processed_frame);
					let point = analysis::find_right_movement(&previous);

					event_sender.submit_command(
						Selector::new(S_DIFFERENCE_FRAME),
						SingleUse::new(previous),
						Target::Auto,
					)?;
					if let Some(detected_point) = point {
						event_sender.submit_command(
							Selector::new(S_CAMERA_POINT),
							(detected_point.x, detected_point.y),
							Target::Auto,
						)?;
					}
				}
			}
			previous_frame = Some(processed_frame);

			// Send FPS
			let elapsed = timer.elapsed().as_secs_f32();
			timer = Instant::now();
			let frame_rate = 1.0 / elapsed;
			event_sender.submit_command(Selector::new(S_CAMERA_FPS), frame_rate, Target::Auto)?;
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
