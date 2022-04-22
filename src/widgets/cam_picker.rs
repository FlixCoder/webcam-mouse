//! Combobox for picking the camera.

use std::sync::mpsc;

use druid::{widget::Controller, Widget, WidgetExt};
use druid_widget_nursery::DropdownSelect;
use nokhwa::{query_devices, CaptureAPIBackend};

/// Channel sender for camera picker update
pub(crate) type PickerSender = mpsc::Sender<usize>;

/// Create the widget for the camera picker
pub fn widget(update_sender: PickerSender) -> impl Widget<usize> {
	let mut cameras: Vec<_> = query_devices(CaptureAPIBackend::Auto).unwrap_or_default();
	cameras.sort_by_key(|info| info.index());
	let dropdown_cams: Vec<_> = cameras
		.into_iter()
		.map(|info| (format!("{}: {}", info.index(), info.human_name()), info.index()))
		.collect();

	DropdownSelect::new(dropdown_cams).controller(SelectionController::new(update_sender))
}

/// Controller for changing camera when the selection is changed
struct SelectionController {
	sender: PickerSender,
}

impl SelectionController {
	/// Create new SelectionController
	pub fn new(sender: PickerSender) -> Self {
		Self { sender }
	}
}

impl<W: Widget<usize>> Controller<usize, W> for SelectionController {
	fn event(
		&mut self,
		child: &mut W,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut usize,
		env: &druid::Env,
	) {
		child.event(ctx, event, data, env)
	}

	fn lifecycle(
		&mut self,
		child: &mut W,
		ctx: &mut druid::LifeCycleCtx,
		event: &druid::LifeCycle,
		data: &usize,
		env: &druid::Env,
	) {
		child.lifecycle(ctx, event, data, env)
	}

	fn update(
		&mut self,
		child: &mut W,
		ctx: &mut druid::UpdateCtx,
		old_data: &usize,
		data: &usize,
		env: &druid::Env,
	) {
		if *old_data != *data {
			let res = self.sender.send(*data);
			if res.is_err() {
				eprintln!("Error sending picked camera!");
			}
		}

		child.update(ctx, old_data, data, env)
	}
}
