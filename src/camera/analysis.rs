//! Video / Image analysis.

use image::{Pixel, RgbImage};
use imageproc::point::Point;

/// Type of frame image.
type Image = RgbImage; // ImageBuffer<Rgb<u8>, Vec<u8>>

/// Process image frame to reduce noise and such for optimal comparison to
/// previous image. Makes motion detection reliable.
pub fn process_frame(image: Image) -> Image {
	// TODO: do things actually
	image
}

/// Compute the difference image, which is just the pixels that have changed
/// compared to the previous version.
pub fn frame_difference(mut previous: Image, current: &Image) -> Image {
	for (prev_pix, cur_pix) in previous.pixels_mut().zip(current.pixels()) {
		*prev_pix = if *prev_pix == *cur_pix { *Pixel::from_slice(&[0; 3]) } else { *cur_pix };
	}
	previous
}

/// Find the rightmost pixel that is not black and return its position.
pub fn find_right_movement(diff_img: &Image) -> Option<Point<u32>> {
	let width = diff_img.width();
	let rotated = image::imageops::rotate270(diff_img);
	for (_, mut column) in rotated.enumerate_rows() {
		if let Some((y, x_rev, _)) = column.find(|(_, _, pix)| pix.0 == [0; 3]) {
			return Some(Point::new(width - x_rev - 1, y));
		}
	}
	None
}
