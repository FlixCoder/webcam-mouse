//! Webcam view widget.

use druid::{
	piet::{ImageFormat, InterpolationMode},
	widget::{FillStrat, Image},
	BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx,
	PaintCtx, Selector, SingleUse, UpdateCtx, Widget,
};
use image::{ColorType, ImageBuffer, Rgb};

/// `CameraView` state
#[derive(Debug, Clone, Default, Data)]
pub struct CameraViewState {
	/// Width and height of image.
	image_dimensions: (u32, u32),
}

/// `CameraView` widget
pub struct CameraView {
	image: Image,
	listen_selector: Selector<SingleUse<Frame>>,
}

impl CameraView {
	/// Create new camera view
	pub fn new(listen_selector: &'static str) -> Self {
		let image_buf = ImageBuf::empty();
		let image = Image::new(image_buf)
			.fill_mode(FillStrat::Contain)
			.interpolation_mode(InterpolationMode::Bilinear);

		Self { image, listen_selector: Selector::new(listen_selector) }
	}
}

impl Widget<CameraViewState> for CameraView {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CameraViewState, env: &Env) {
		if let Event::Command(command) = event {
			if let Some(frame) = command.get(self.listen_selector).and_then(SingleUse::take) {
				let (width, height) = frame.dimensions();
				data.image_dimensions = (width, height);
				self.image.set_image_data(frame_to_image(frame));
				ctx.request_paint();
			}
		}

		self.image.event(ctx, event, data, env)
	}

	fn lifecycle(
		&mut self,
		ctx: &mut LifeCycleCtx,
		event: &LifeCycle,
		data: &CameraViewState,
		env: &Env,
	) {
		self.image.lifecycle(ctx, event, data, env)
	}

	fn update(
		&mut self,
		ctx: &mut UpdateCtx,
		old_data: &CameraViewState,
		data: &CameraViewState,
		env: &Env,
	) {
		if old_data.image_dimensions != data.image_dimensions {
			ctx.request_layout();
		}

		self.image.update(ctx, old_data, data, env)
	}

	fn layout(
		&mut self,
		ctx: &mut LayoutCtx,
		bc: &BoxConstraints,
		data: &CameraViewState,
		env: &Env,
	) -> druid::Size {
		self.image.layout(ctx, bc, data, env)
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &CameraViewState, env: &Env) {
		self.image.paint(ctx, data, env)
	}
}

/// Type of camera frame image.
type Frame = ImageBuffer<Rgb<u8>, Vec<u8>>;

/// Convert a frame to a [`Image`] widget compatible type.
fn frame_to_image(frame: Frame) -> ImageBuf {
	let raw = frame.into_flat_samples();
	if raw.color_hint != Some(ColorType::Rgb8) || raw.layout.channels != 3 {
		panic!("Color format did not fit!");
	}

	let width = raw.layout.width as usize;
	let height = raw.layout.height as usize;

	ImageBuf::from_raw(raw.samples, ImageFormat::Rgb, width, height)
}
