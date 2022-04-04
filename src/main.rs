use color_eyre::Result;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use webcam_mouse::{root_widget, CameraConnector, RootUIState};

// TODO:
// - Camera picker combo box
// - Camera image processing to find movement
// - Selection to toggle image view: real, processed or movement image

fn main() -> Result<()> {
	color_eyre::install()?;

	let window = WindowDesc::new(root_widget)
		.title(LocalizedString::new("Webcam Mouse"))
		.window_size((853.0, 480.0));
	let launcher = AppLauncher::with_window(window);
	let event_sender = launcher.get_external_handle();

	let camera_handler = CameraConnector::new(event_sender);
	let _jh = camera_handler.spawn();

	launcher.use_simple_logger().launch(RootUIState::default()).expect("running app");

	Ok(())
}
