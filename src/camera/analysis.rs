//! Video / Image analysis.

use std::ops::BitAnd;

use image::RgbImage;
use imageproc::{filter, point::Point};
use rayon::prelude::*;

/// Mirror / flip image horizontally in place
pub fn flip_in_place(image: &mut RgbImage) {
	let layout = image.as_flat_samples().layout;
	let width = layout.width;
	let pix_size = layout.width_stride;
	image.par_chunks_mut(layout.height_stride).for_each(|row| {
		let mid = (width >> 1) as usize * pix_size;
		let (left, right) = row.split_at_mut(mid);
		for (l, r) in left.chunks_exact_mut(pix_size).zip(right.rchunks_exact_mut(pix_size)) {
			std::mem::swap(&mut l[0], &mut r[0]);
			std::mem::swap(&mut l[1], &mut r[1]);
			std::mem::swap(&mut l[2], &mut r[2]);
		}
	});
}

/// Process image frame to reduce noise and such for optimal comparison to
/// previous image. Makes motion detection reliable.
pub fn process_frame(image: &RgbImage) -> RgbImage {
	let image = filter::median_filter(image, 2, 2); // 5x5
	filter::gaussian_blur_f32(&image, 1.0)
}

/// Compute the difference image, which is just the absolute difference in the
/// pixel values compared to the previous version.
pub fn frame_difference(previous: &mut RgbImage, current: &RgbImage) {
	let stride = previous.as_flat_samples().layout.height_stride;
	previous.par_chunks_mut(stride).zip(current.par_chunks(stride)).for_each(|(prev, cur)| {
		for (prev, cur) in prev.iter_mut().zip(cur) {
			*prev = prev.abs_diff(*cur).bitand(0b11100000);
		}
	})
}

/// Find the rightmost pixel that is not black and return its position.
pub fn find_right_movement(diff_img: &RgbImage) -> Option<Point<u32>> {
	let layout = diff_img.as_flat_samples().layout;
	let pix_size = layout.channel_stride * layout.channels as usize;
	diff_img
		.par_chunks_exact(layout.height_stride)
		.enumerate()
		.filter_map(|(y, row)| {
			row.chunks_exact(pix_size)
				.enumerate()
				.rfind(|(_, pix)| **pix != [0x00, 0x00, 0x00])
				.map(|(x, _)| (x, y))
		})
		.max_by_key(|(x, _y)| *x)
		.map(|(x, y)| Point::new(x as u32, y as u32))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn flip_image() {
		let mut image1 =
			RgbImage::from_fn(256, 1, |x, _y| *image::Pixel::from_slice(&[x as u8; 3]));
		let mut image2 = image1.clone();
		flip_in_place(&mut image1);
		image::imageops::flip_horizontal_in_place(&mut image2);
		assert_eq!(image1, image2);
	}
}
