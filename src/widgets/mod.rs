//! Widgets for the UI.
// derive(Lense) has no docs, widgets no Debug impl..
#![allow(missing_docs, missing_debug_implementations)]

mod cam_picker;
mod webcam;

use druid::{
	widget::{Align, Flex},
	Data, Lens, UnitPoint, Widget, WidgetExt,
};

use self::cam_picker::PickerSender;
use crate::camera::{S_CAMERA_FRAME, S_CAMERA_POINT};

/// Root UI widget state.
#[derive(Debug, Clone, Default, Data, Lens)]
pub struct RootUIState {
	cam_index: usize,
	cam_view: webcam::CameraViewState,
}

/// Build the root UI widget.
pub fn root_widget(cam_pick_sender: PickerSender) -> impl Widget<RootUIState> {
	let cam_view =
		webcam::CameraView::new(S_CAMERA_FRAME, S_CAMERA_POINT).lens(RootUIState::cam_view);

	let cam_dropdown = cam_picker::widget(cam_pick_sender).lens(RootUIState::cam_index);
	let controls = Flex::column()
		.with_child(cam_dropdown)
		.padding((10.0, 10.0))
		.align_vertical(UnitPoint::TOP);

	let layout = Flex::row().with_child(cam_view).with_child(controls).must_fill_main_axis(true);

	Align::centered(layout)
}
