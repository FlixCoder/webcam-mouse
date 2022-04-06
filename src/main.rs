use std::sync::mpsc;

use color_eyre::Result;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use webcam_mouse::{root_widget, CameraConnector, RootUIState};

// TODO:
// - Camera image processing to find movement
// - Selection to toggle image view: real, processed or movement image

fn main() -> Result<()> {
	color_eyre::install()?;

	let (pick_sender, pick_receiver) = mpsc::channel();

	let window = WindowDesc::new(root_widget(pick_sender))
		.title(LocalizedString::new("Window-Title").with_placeholder("Webcam Mouse"))
		.window_size((1100.0, 600.0));
	let launcher = AppLauncher::with_window(window);
	let event_sender = launcher.get_external_handle();

	let camera_handler = CameraConnector::new(event_sender, pick_receiver);
	let cam_handle = camera_handler.spawn();

	launcher.log_to_console().launch(RootUIState::default()).expect("running app");

	cam_handle.join().expect("joining camera handler");
	Ok(())
}
