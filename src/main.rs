#[cfg(feature = "cli")]
use clearscreen::clear;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
#[cfg(feature = "cli")]
use colored::Colorize;

#[cfg(feature = "cli")]
use std::time;
#[cfg(feature = "cli")]
use std::time::Duration;
#[cfg(feature = "cli")]
use std::io;
#[cfg(feature = "cli")]
use std::thread;

pub mod automaton;
use crate::automaton::CellularAutomaton;

pub mod strats;
#[cfg(feature = "cli")]
use crate::strats::*;

#[cfg(feature = "cli")]
pub mod webping;
#[cfg(feature = "cli")]
use crate::webping::{save_webp_anim};


impl CellularAutomaton {
	pub fn print(&self) -> &Self {
		for y in 0..self.y {
			for x in 0..self.x {
				let v = self.cells[x][y];
				// if v == 1 {print!("{}", "#".green());}
				// else      {print!("{}", "X".red()  );}
				#[cfg(feature = "cli")]
				if v == 1 {print!("{}", "■".green());}
				else      {print!("{}", "▢".red()  );}
				#[cfg(not(feature = "cli"))]
				if v == 1 {print!("{}", "■");}
				else      {print!("{}", "▢");}
			}
			print!("\n");
		}
		self
	}
}

// fn prompt<T>(prompt: &str) -> T where T: std::str::FromStr + Debug {
// 	print!("{} ", prompt);
// 	let _ = io::stdout().flush();
// 	match io::stdin().lines()
// 		.next()
// 		.unwrap().unwrap()
// 		.trim()
// 		.parse() {
// 		Ok(v) => v,
// 		Err(_e) => panic!("Failed to parse input")
// 	}
// }

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
struct Args {
	#[command(subcommand)]
    command: Commands,
	#[arg(index=1, default_value_t = 100, global=true)]
	width: usize,
	#[arg(index=2, default_value_t = 100, global=true)]
	height: usize,
	#[arg(short='s', long="steps", default_value_t = 100)]
	steps: u64,
	#[arg(short='p', long="prob", default_value_t = 50.0)]
	alive_prob: f64
}

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug)]
enum Commands {
    Animated {
		#[arg(short='d', long="delay", default_value_t = 300)]
		delay:u32,
		#[arg(short='p', long="preview", default_value_t = false)]
		preview:bool
    },
    Webp {
        #[arg(short='o', long="out", default_value_t = String::from("output.webp"))]
        output: String,
    },
	Res{
        #[arg(short='o', long="out", default_value_t = String::from("output.frame"))]
        output: String,
	}
}

fn main(){
	#[cfg(feature = "cli")]
	cli_main();
	#[cfg(not(feature = "cli"))]
	{
		println!("To use rayon-ca as bin/CLI, you need to add feature CLI.");
		println!("Try cargo install rayon-ca --features cli");
		std::process::exit(1);
	}
}

#[cfg(feature = "cli")]
fn cli_main() {
	let arguments = Args::parse();
	println!("{:?}", arguments);
	let width = arguments.width;
	let height = arguments.height;
	let steps = arguments.steps;
	let alive_prob = arguments.alive_prob;
	match arguments.command {
		Commands::Animated {delay, preview} =>
			animated(width, height, steps, alive_prob, delay, preview),
		Commands::Webp        {output } =>
			webp(width, height, steps, alive_prob, output),
		Commands::Res         {output } =>
			result(width, height, steps, alive_prob, output),

	}
	// match arguments.mode {
	// 	// "animated" => interactive(width, height, steps, alive_prob),
	// 	"gen" => generate_webp(width, height, steps, alive_prob),
	// 	_ => println!("Unknown mode: {}", arguments.mode),
	// }
}

#[cfg(feature = "cli")]
fn result(width:usize, height:usize, steps:u64, alive_prob:f64, output:String) {
	let mut c = CellularAutomaton::new(width, height);
	c.set_processor(conway_next);
	c
		.randomize_prob(alive_prob / 100.0)
		.steps(steps)
		// .print()
	;
	let mut dot = String::new();
	c.cells.iter().enumerate().for_each(
		|(xdx, row)|
		// "({}, {}): {}\n", cell.0.0, cell.0.1, cell.1).as_str()
			row.iter().enumerate().for_each(|(ydx, value)|
				dot.push_str(format!("{} {}: {}\n", xdx, ydx, value).as_str()),
			)
		);
	match std::fs::write(&output, dot){
		Ok(_v) => println!("Saved to {}", &output),
		Err(e) => println!("Failed to save to {}, error: {}", &output, e)
	};
}

#[cfg(feature = "cli")]
fn animated(width:usize, height:usize, steps:u64, alive_prob:f64, delay:u32, preview:bool) {
	let mut c = CellularAutomaton::new(width, height);
	c.set_processor(conway_next);
	clear().expect("failed to clear screen");
	c.randomize_prob(alive_prob / 100.0).print();
	println!("Starting...");
	println!("Press enter to start...");
	println!("{}", alive_prob / 100.0);
	if preview
	{
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_v) => println!("Starting animation..."),
			Err(e) => println!("Failed to read input somehow, error: {}", e),
		};
	}
	for _ in 1..steps{
		c.step();
		clear().expect("failed to clear screen");
		c.print();
		thread::sleep(Duration::from_millis(delay as u64));
	}
}

#[cfg(feature = "cli")]
fn webp(width:usize, height:usize, steps:u64, alive_prob:f64, output:String) {
	let mut c = CellularAutomaton::new_with_processor(width, height, conway_next);
	println!("{}", alive_prob / 100.0);
	c.randomize_prob(alive_prob / 100.0);
	let now = time::Instant::now();
	let mut frames = Vec::new();
	for _ in 1..steps{
		frames.push(c.step().cells.clone());
	}
	println!("Generated in {}ms", now.elapsed().as_millis());
	let now_image = time::Instant::now();
	save_webp_anim(
		&frames, width as u32, height as u32,
		output.as_str(), 100)
		.expect("failed to save anim");
	println!("Saved to {} in {}ms", output, now_image.elapsed().as_millis());
}
