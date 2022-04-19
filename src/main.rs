use std::sync::mpsc;

use color_eyre::Result;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use webcam_mouse::{root_widget, CameraConnector, RootUIState};

fn main() -> Result<()> {
	color_eyre::install()?;

	let (pick_sender, pick_receiver) = mpsc::channel();

	let window = WindowDesc::new(root_widget(pick_sender))
		.title(LocalizedString::new("Window-Title").with_placeholder("Webcam Mouse"))
		.window_size((1100.0, 550.0));
	let launcher = AppLauncher::with_window(window);
	let event_sender = launcher.get_external_handle();

	let camera_handler = CameraConnector::new(event_sender, pick_receiver);
	let cam_handles = camera_handler.spawn();

	launcher.log_to_console().launch(RootUIState::default()).expect("running app");

	cam_handles.0.join().expect("joining camera frame receiver");
	cam_handles.1.join().expect("joining camera frame processor");
	Ok(())
}
