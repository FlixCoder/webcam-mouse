//! Image analysis benchmarks.

use std::{collections::VecDeque, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{Pixel, RgbImage};
use imageproc::{filter, point::Point};
use rayon::prelude::*;
use webcam_mouse::analysis;

criterion_main!(benches);
criterion_group! {
	name = benches;
	config = Criterion::default().measurement_time(Duration::from_secs(15));
	targets = mirror_benches, image_processing_benches, image_diff_benches, find_rightmost_benches
}

/// Benchmark mirroring images
pub fn mirror_benches(crit: &mut Criterion) {
	let mut group = crit.benchmark_group("Mirror image");

	let image = RgbImage::from_fn(800, 600, |x, y| {
		if [(400, 300), (300, 300), (300, 200)].contains(&(x, y)) {
			*Pixel::from_slice(&[0xFF, 0xFF, 0xFF])
		} else {
			*Pixel::from_slice(&[0x00, 0x00, 0x00])
		}
	});

	group.bench_function("analysis::original", |b| {
		b.iter(|| {
			let mut image = image.clone();
			analysis::flip_in_place(black_box(&mut image));
		})
	});

	group.bench_function("image::flip_in_place", |b| {
		b.iter(|| {
			let mut image = image.clone();
			image::imageops::flip_horizontal_in_place(black_box(&mut image));
		})
	});

	group.bench_function("try", |b| {
		b.iter(|| {
			let mut image = image.clone();
			image_flip_try(black_box(&mut image));
		})
	});

	group.finish();
}

fn image_flip_try(image: &mut RgbImage) {
	let layout = image.as_flat_samples().layout;
	let width = layout.width;
	let pix_size = layout.width_stride;
	image.chunks_mut(layout.height_stride).for_each(|row| {
		let mid = (width >> 1) as usize * pix_size;
		let (left, right) = row.split_at_mut(mid);
		for (l, r) in left.chunks_exact_mut(pix_size).zip(right.rchunks_exact_mut(pix_size)) {
			std::mem::swap(&mut l[0], &mut r[0]);
			std::mem::swap(&mut l[1], &mut r[1]);
			std::mem::swap(&mut l[2], &mut r[2]);
		}
	});
}

/// Benchmark image processing for preparing stable difference images.
pub fn image_processing_benches(crit: &mut Criterion) {
	let mut group = crit.benchmark_group("Processing image");

	let image = RgbImage::from_fn(800, 600, |x, y| {
		if [(400, 300), (300, 300), (300, 200)].contains(&(x, y)) {
			*Pixel::from_slice(&[0xFF, 0xFF, 0xFF])
		} else {
			*Pixel::from_slice(&[0x00, 0x00, 0x00])
		}
	});

	group.bench_function("analysis::original", |b| {
		b.iter(|| analysis::process_frame(black_box(&image)))
	});

	group.bench_function("imageproc", |b| b.iter(|| process_frame_imageproc(black_box(&image))));

	group.bench_function("try", |b| b.iter(|| process_frame_try(black_box(&image))));

	group.finish();
}

fn process_frame_imageproc(image: &RgbImage) -> RgbImage {
	let image = filter::median_filter(image, 2, 2); // 5x5
	let kernel = [
		1.0 / 16.0,
		2.0 / 16.0,
		1.0 / 16.0,
		2.0 / 16.0,
		4.0 / 16.0,
		2.0 / 16.0,
		1.0 / 16.0,
		2.0 / 16.0,
		1.0 / 16.0,
	];
	filter::filter3x3(&image, &kernel)
}

fn process_frame_try(image: &RgbImage) -> RgbImage {
	let radius = 2;
	let kernel_width = radius * 2 + 1;
	let layout = image.as_flat_samples().layout;

	let radius = radius as isize;
	let mut out = vec![0; image.as_raw().len()];
	out.par_chunks_exact_mut(layout.height_stride).zip(0..layout.height).for_each(|(line, y)| {
		// Initialize histogram buffers
		let mut hists =
			vec![VecDeque::with_capacity(kernel_width * kernel_width); layout.channels.into()];
		for filter_x in -radius..=radius {
			for filter_y in -radius..=radius {
				for (c_val, hist) in get_pixel_clamped(image, filter_x, y as isize + filter_y)
					.channels()
					.iter()
					.zip(hists.iter_mut())
				{
					hist.push_back(*c_val);
				}
			}
		}

		let mut i = 0;
		for val in median_pixel(&hists) {
			line[i] = val;
			i += 1;
		}

		for x in 1..layout.width {
			for filter_y in -radius..=radius {
				for (c_val, hist) in
					get_pixel_clamped(image, x as isize + radius, y as isize + filter_y)
						.channels()
						.iter()
						.zip(hists.iter_mut())
				{
					hist.pop_front();
					hist.push_back(*c_val);
				}
			}

			for val in median_pixel(&hists) {
				line[i] = val;
				i += 1;
			}
		}
	});

	RgbImage::from_raw(layout.width, layout.height, out).unwrap()
}

/// Compute median of each queue buffer and return raw pixel values of the
/// combined results.
#[inline]
fn median_pixel(bufs: &[VecDeque<u8>]) -> impl Iterator<Item = u8> + '_ {
	bufs.iter().map(|buf| {
		let mut hist = [0_u8; 256];
		for val in buf {
			hist[*val as usize] += 1;
		}

		let mid = (buf.len() / 2) as u8;
		let mut found = 0;
		for i in 0..=0xFF {
			found += hist[i as usize];
			if found > mid {
				return i;
			}
		}

		panic!("Should've returned by now!");
	})
}

/// Get pixel by clamped coordinates.
#[inline]
fn get_pixel_clamped(image: &RgbImage, x: isize, y: isize) -> &image::Rgb<u8> {
	let (w, h) = image.dimensions();
	let w = w as isize - 1;
	let h = h as isize - 1;
	image.get_pixel(x.clamp(0, w) as u32, y.clamp(0, h) as u32)
}

/// Benchmark computing difference images.
pub fn image_diff_benches(crit: &mut Criterion) {
	let mut group = crit.benchmark_group("Compute difference image");

	let image1 = RgbImage::from_fn(800, 600, |x, y| {
		if [(400, 300), (300, 300), (300, 200)].contains(&(x, y)) {
			*Pixel::from_slice(&[0xFF, 0xFF, 0xFF])
		} else {
			*Pixel::from_slice(&[0x00, 0x00, 0x00])
		}
	});
	let image2 = RgbImage::from_fn(800, 600, |_x, _y| *Pixel::from_slice(&[0x00, 0x00, 0x00]));

	group.bench_function("analysis::original", |b| {
		b.iter(|| {
			let mut image = image1.clone();
			analysis::frame_difference(black_box(&mut image), black_box(&image2))
		})
	});

	group.bench_function("try", |b| {
		b.iter(|| {
			let mut image = image1.clone();
			frame_difference_try(black_box(&mut image), black_box(&image2))
		})
	});

	group.finish();
}

fn frame_difference_try(previous: &mut RgbImage, current: &RgbImage) {
	for (prev, cur) in previous.iter_mut().zip(current.as_raw()) {
		*prev = match prev.abs_diff(*cur) {
			small if small < 32 => 0,
			big => big,
		};
	}
}

/// Benchmark and compare finding the rightmost pixel that is not black (with
/// movement when applied to diff images).
pub fn find_rightmost_benches(crit: &mut Criterion) {
	let mut group = crit.benchmark_group("Find rightmost pixel");

	let image = RgbImage::from_fn(800, 600, |x, y| {
		if [(400, 300), (300, 300), (300, 200)].contains(&(x, y)) {
			*Pixel::from_slice(&[0xFF, 0xFF, 0xFF])
		} else {
			*Pixel::from_slice(&[0x00, 0x00, 0x00])
		}
	});
	let expected = Some(Point::new(400, 300));

	group.bench_function("analysis::original", |b| {
		b.iter(|| {
			let res = analysis::find_right_movement(black_box(&image));
			assert_eq!(res, expected);
		})
	});

	group.bench_function("try", |b| {
		b.iter(|| {
			let res = find_right_try(black_box(&image));
			assert_eq!(res, expected);
		})
	});

	group.finish();
}

fn find_right_try(diff_img: &RgbImage) -> Option<Point<u32>> {
	let layout = diff_img.as_flat_samples().layout;
	let pix_size = layout.channel_stride * layout.channels as usize;
	let buffer = diff_img.as_raw();
	for x in (0..layout.width).rev() {
		let mut pos = x as usize * pix_size;
		for y in 0..layout.height {
			if buffer[pos..pos + pix_size] != [0x00, 0x00, 0x00] {
				return Some(Point::new(x, y));
			}
			pos += layout.height_stride;
		}
	}
	None
}
