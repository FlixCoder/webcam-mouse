//! Widgets for the UI.
// derive(Lense) has no docs, widgets no Debug impl..
#![allow(missing_docs, missing_debug_implementations)]

mod webcam;

use druid::{
	widget::{Align, Flex},
	Data, Lens, Widget, WidgetExt,
};

use crate::camera::SELECTOR_CAMERA;

/// Root UI widget state.
#[derive(Debug, Clone, Default, Data, Lens)]
pub struct RootUIState {
	cam_view: webcam::CameraViewState,
}

/// Build the root UI widget.
pub fn root_widget() -> impl Widget<RootUIState> {
	let cam = webcam::CameraView::new(SELECTOR_CAMERA).lens(RootUIState::cam_view);

	let layout = Flex::row().with_child(cam);

	Align::centered(layout)
}
