use std::fmt::Debug;
use std::io::Write;
use std::time;
use std::time::Duration;
use std::thread;
use std::io;
use image;
use image::ImageBuffer;
use image::Rgb;
use std::collections::HashMap;
use std::env;
use webp_animation::prelude::*;
use clap::Parser;

pub mod automaton;
use crate::automaton::CellularAutomaton;

fn prompt<T>(prompt: &str) -> T where T: std::str::FromStr + Debug {
	print!("{} ", prompt);
	let _ = io::stdout().flush();
	match io::stdin().lines()
		.next()
		.unwrap().unwrap()
		.trim()
		.parse() {
		Ok(v) => v,
		Err(_e) => panic!("Failed to parse input.")
	}
}

fn save_webp(cells: &HashMap<(i32, i32), i32>, width: u32, height: u32, path: &str) {
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
fn save_webp_anim(
	frames: &[HashMap<(i32, i32), i32>],
	width: u32,
	height: u32,
	path: &str,
	frame_ms: i32, // длительность кадра
) -> Result<(), Box<dyn std::error::Error>> {
	let dimensions = (width, height);
	let mut encoder = Encoder::new(dimensions)?; // по умолчанию ColorMode::Rgba[web:78]

	for (i, cells) in frames.iter().enumerate() {
		let mut rgba = vec![0u8; (width * height * 4) as usize];

		for (&(x, y), &value) in cells.iter() {
			if x < 0 || y < 0 || x as u32 >= width || y as u32 >= height {
				continue;
			}
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
				rgba[idx + 3] = 255; // непрозрачный фон
			}
		}

		let ts = i as i32 * frame_ms;
		encoder.add_frame(&rgba, ts)?; // RGBA подряд[web:78]
	}

	let final_ts = frames.len() as i32 * frame_ms;
	let webp_data = encoder.finalize(final_ts)?;
	std::fs::write(path, webp_data)?;

	Ok(())
}

#[derive(Parser, Debug)]
struct Args {
	#[arg(short='w', long="width", default_value_t = 100)]
	width: i32,
	#[arg(short='y', long="height", default_value_t = 100)]
	height: i32,
	#[arg(short='s', long="steps", default_value_t = 100)]
	steps: i64,
	#[arg(short='p', long="prob", default_value_t = 0.3)]
	alive_prob: f64
}

fn main() {
	let arguments = Args::parse();
	println!("{:?}", arguments);
	let width = arguments.width;
	let height = arguments.height;
	let steps = arguments.steps;
	let alive_prob = arguments.alive_prob;
	// let args: Vec<String> = env::args().collect();
	// let mut width     :i32 = 0;
	// let mut height    :i32 = 0;
	// let mut steps     :i64 = 0;
	// let mut alive_prob:f64 = -1.0;
	// for arg in args.iter() {
	// 	println!("{}", arg);
	// 	if arg.parse::<f64>().is_err(){continue;}
	// 	if width == 0                 {width      = arg.parse().unwrap();}
	// 	else if height == 0           {height     = arg.parse().unwrap();}
	// 	else if steps == 0            {steps      = arg.parse().unwrap();}
	// 	else if alive_prob == -1.0    {alive_prob = arg.parse().unwrap();}
	// }
	// if width  == 0 {width  = prompt("Provide width: ")}
	// if height == 0 {height = prompt("Provide height: ")}
	// if steps  == 0 {
	// 	steps  = prompt("Provide steps of automaton (0 <): ");
	// 	if steps < 1 {
	// 		panic!("Steps must be more than 0.");
	// 	}
	// 	// let step_delay:i32 = read("Provide time delay between steps: ");
	// }
	// if alive_prob == -1.0 {alive_prob = prompt("Provide alive probability (0-100): ")}

	let mut sample_automaton = CellularAutomaton::new(width, height);
	// clearscreen::clear().expect("failed to clear screen");
	println!("Starting...");
	println!("Press enter to start...");
	println!("{}", alive_prob / 100.0);
	sample_automaton.randomize_prob(alive_prob / 100.0);
	// sample_automaton.print();
	// let mut input = String::new();
	// io::stdin().read_line(&mut input).expect("Read error");
	// thread::sleep(Duration::from_millis(2000));
	let mut frames:Vec<HashMap<(i32, i32), i32>> = Vec::new();
	for _ in 1..steps{
		// sample_automaton.step();
		frames.push(sample_automaton.step().cells.clone());
		// save_webp(&(sample_automaton.cells.clone()), width as u32, height as u32, &format!("result/{frame_idx}.webp"));
		// clearscreen::clear().expect("failed to clear screen");
		// sample_automaton.print();
		// thread::sleep(Duration::from_millis(300 as u64));
	}
	// sample_automaton.print();
	// for frame_idx in 0..frames.len() {
	// 	save_webp(&frames[frame_idx], width as u32, height as u32, &format!("result/{frame_idx}.webp"));
	// }
	save_webp_anim(
		&frames, width as u32, height as u32,
		"result/anim.webp", 100)
		.expect("failed to save anim");
	print!("Saved to {}", "result/anim.webp");
}
