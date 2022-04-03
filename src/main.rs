use druid::{AppLauncher, LocalizedString, WindowDesc};
use webcam_mouse::{root_widget, RootUIState};

fn main() {
	let window = WindowDesc::new(root_widget)
		.title(LocalizedString::new("Hello World!"))
		.window_size((853.0, 480.0));

	AppLauncher::with_window(window).launch(RootUIState::default()).expect("running app");
}
