//! FPS view.

use druid::{
	widget::{Controller, Label},
	Env, Event, Selector, Widget, WidgetExt,
};

use crate::camera::S_CAMERA_FPS;

/// Create the widget for the FPS view.
pub fn widget() -> impl Widget<u32> {
	Label::new(|fps: &u32, _env: &Env| format!("FPS: {fps}")).controller(FpsController)
}

/// Controller for receiving camera FPS.
struct FpsController;

impl<W: Widget<u32>> Controller<u32, W> for FpsController {
	fn event(
		&mut self,
		child: &mut W,
		ctx: &mut druid::EventCtx,
		event: &Event,
		data: &mut u32,
		env: &Env,
	) {
		if let Event::Command(command) = event {
			if let Some(fps) = command.get(Selector::<u32>::new(S_CAMERA_FPS)) {
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
		data: &u32,
		env: &Env,
	) {
		child.lifecycle(ctx, event, data, env)
	}

	fn update(
		&mut self,
		child: &mut W,
		ctx: &mut druid::UpdateCtx,
		old_data: &u32,
		data: &u32,
		env: &Env,
	) {
		child.update(ctx, old_data, data, env)
	}
}
