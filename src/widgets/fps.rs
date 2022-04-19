//! FPS view.

use druid::{
	widget::{Controller, Flex, Label},
	Env, Event, Selector, Widget, WidgetExt,
};

use crate::camera::S_CAMERA_FPS;

/// Data type of FPS messages and therefore also the widget
pub type Fps = (u32, f32);

/// Create the widget for the FPS view.
pub fn widget() -> impl Widget<Fps> {
	Flex::column()
		.with_child(Label::new(|fps: &Fps, _env: &Env| format!("Camera FPS: {}", fps.0)))
		.with_child(Label::new(|fps: &Fps, _env: &Env| format!("Processing FPS: {:.1}", fps.1)))
		.controller(FpsController)
}

/// Controller for receiving camera FPS.
struct FpsController;

impl<W: Widget<Fps>> Controller<Fps, W> for FpsController {
	fn event(
		&mut self,
		child: &mut W,
		ctx: &mut druid::EventCtx,
		event: &Event,
		data: &mut Fps,
		env: &Env,
	) {
		if let Event::Command(command) = event {
			if let Some(fps) = command.get(Selector::<Fps>::new(S_CAMERA_FPS)) {
				*data = *fps;
			}
		}

		child.event(ctx, event, data, env)
	}

	fn lifecycle(
		&mut self,
		child: &mut W,
		ctx: &mut druid::LifeCycleCtx,
		event: &druid::LifeCycle,
		data: &Fps,
		env: &Env,
	) {
		child.lifecycle(ctx, event, data, env)
	}

	fn update(
		&mut self,
		child: &mut W,
		ctx: &mut druid::UpdateCtx,
		old_data: &Fps,
		data: &Fps,
		env: &Env,
	) {
		child.update(ctx, old_data, data, env)
	}
}
