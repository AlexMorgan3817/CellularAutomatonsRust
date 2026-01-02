use image;
use image::ImageBuffer;
use image::Rgb;
use std::collections::HashMap;
use webp_animation::prelude::*;

pub fn save_webp(cells: &HashMap<(i32, i32), i32>, width: u32, height: u32, path: &str) {
	// let mut imgbuf = ImageBuffer::new(width, height);
	let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
	for ((x, y), value) in cells.iter() {
		if *value == 1 {
			let px = imgbuf.get_pixel_mut(*x as u32, *y as u32);
			*px = Rgb([255, 255, 255]);
		} else {
			let px = imgbuf.get_pixel_mut(*x as u32, *y as u32);
			*px = Rgb([0, 0, 0]);
		}
	}

	match imgbuf.save_with_format(path, image::ImageFormat::WebP)
	{
		Ok(_v) => println!("Saved to {}", path),
		Err(e) => println!("Failed to save to {}, error: {}", path, e)
	};
}
pub fn save_webp_anim(
	frames: &[HashMap<(u32, u32), i32>],
	width: u32,
	height: u32,
	path: &str,
	frame_ms: i32,
) -> Result<(), Box<dyn std::error::Error>> {
	let dimensions = (width, height);
	let mut encoder = Encoder::new(dimensions)?;

	for (i, cells) in frames.iter().enumerate() {
		let mut rgba = vec![0u8; (width * height * 4) as usize];

		for (&(x, y), &value) in cells.iter() {
			let idx = ((y as u32 * width + x as u32) * 4) as usize;
			if value == 1 {
				rgba[idx + 0] = 255;
				rgba[idx + 1] = 255;
				rgba[idx + 2] = 255;
				rgba[idx + 3] = 255;
			} else {
				rgba[idx + 0] = 0;
				rgba[idx + 1] = 0;
				rgba[idx + 2] = 0;
				rgba[idx + 3] = 255;
			}
		}

		let ts = i as i32 * frame_ms;
		encoder.add_frame(&rgba, ts)?;
	}

	let final_ts = frames.len() as i32 * frame_ms;
	let webp_data = encoder.finalize(final_ts)?;
	std::fs::write(path, webp_data)?;

	Ok(())
}
