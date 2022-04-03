//! Widgets for the UI.
#![allow(missing_docs)] // derive(Lense) has no docs..

mod webcam;

use druid::{
	widget::{Align, Flex, Label},
	Data, Env, Lens, Widget,
};

/// Root UI widget state.
#[derive(Debug, Clone, Default, Data, Lens)]
pub struct RootUIState {
	// TODO
}

/// Build the root UI widget.
pub fn root_widget() -> impl Widget<RootUIState> {
	let label = Label::new(|_state: &RootUIState, _env: &Env| "Hello world!".to_owned());

	let layout = Flex::row().with_child(label);

	Align::centered(layout)
}
