//! Widgets for the UI.

mod cam_picker;
mod fps;
mod webcam;

use druid::{
	widget::{Align, Axis, Flex, Tabs, TabsEdge},
	Data, Lens, UnitPoint, Widget, WidgetExt,
};

use self::{cam_picker::PickerSender, fps::Fps};
use crate::camera::{S_CAMERA_FRAME, S_CAMERA_POINT, S_DIFFERENCE_FRAME, S_PROCESSED_FRAME};

/// Root UI widget state.
#[derive(Debug, Clone, Default, Data, Lens)]
pub struct RootUIState {
	/// Index of system camera
	cam_index: usize,
	/// State of camera view
	cam_view: webcam::CameraViewState,
	/// Camera FPS
	cam_fps: Fps,
}

/// Build the root UI widget.
pub fn root_widget(cam_pick_sender: PickerSender) -> impl Widget<RootUIState> {
	let cam_view = Tabs::new()
		.with_axis(Axis::Vertical)
		.with_edge(TabsEdge::Trailing)
		.with_tab(
			"Original",
			webcam::CameraView::new(S_CAMERA_FRAME, S_CAMERA_POINT).lens(RootUIState::cam_view),
		)
		.with_tab(
			"Processed",
			webcam::CameraView::new(S_PROCESSED_FRAME, S_CAMERA_POINT).lens(RootUIState::cam_view),
		)
		.with_tab(
			"Difference",
			webcam::CameraView::new(S_DIFFERENCE_FRAME, S_CAMERA_POINT).lens(RootUIState::cam_view),
		);

	let cam_dropdown =
		cam_picker::widget(cam_pick_sender).lens(RootUIState::cam_index).padding((10.0, 10.0));
	let fps = fps::widget().lens(RootUIState::cam_fps).padding((10.0, 10.0));
	let controls = Flex::column()
		.with_child(cam_dropdown)
		.with_default_spacer()
		.with_child(fps)
		.align_vertical(UnitPoint::TOP);

	let layout =
		Flex::row().with_flex_child(cam_view, 1.0).with_child(controls).must_fill_main_axis(true);

	Align::centered(layout)
}
