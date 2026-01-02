use std::fmt::Debug;
use std::io::Write;
use std::time;
use std::time::Duration;
use std::thread;
use std::io;
use clearscreen::clear;
use clap::{Parser, Subcommand};

pub mod automaton;
use crate::automaton::CellularAutomaton;

pub mod webping;
use crate::webping::{save_webp_anim};

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

#[derive(Subcommand, Debug)]
enum Commands {
    Animated {
		#[arg(short='d', long="delay", default_value_t = 300)]
		delay:u32
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

fn main() {
	let arguments = Args::parse();
	println!("{:?}", arguments);
	let width = arguments.width;
	let height = arguments.height;
	let steps = arguments.steps;
	let alive_prob = arguments.alive_prob;
	match arguments.command {
		Commands::Animated {delay     } =>
			animated(width, height, steps, alive_prob, delay),
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

fn result(width:usize, height:usize, steps:u64, alive_prob:f64, output:String) {
	let mut sample_automaton = CellularAutomaton::new(width, height);
	sample_automaton
		.randomize_prob(alive_prob / 100.0)
		.steps(steps)
		// .print()
	;
	let mut dot = String::new();
	sample_automaton.cells.iter().enumerate().for_each(
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

fn animated(width:usize, height:usize, steps:u64, alive_prob:f64, delay:u32) {
	let mut sample_automaton = CellularAutomaton::new(width, height);
	clear().expect("failed to clear screen");
	println!("Starting...");
	println!("Press enter to start...");
	println!("{}", alive_prob / 100.0);
	sample_automaton
		.randomize_prob(alive_prob / 100.0)
		.print();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Read error");
	thread::sleep(Duration::from_millis(2000));
	for _ in 1..steps{
		sample_automaton.step();
		clear().expect("failed to clear screen");
		sample_automaton.print();
		thread::sleep(Duration::from_millis(300 as u64));
	}
}

fn webp(width:usize, height:usize, steps:u64, alive_prob:f64, output:String) {
	let mut sample_automaton = CellularAutomaton::new(width, height);
	println!("{}", alive_prob / 100.0);
	sample_automaton.randomize_prob(alive_prob / 100.0);
	let now = time::Instant::now();
	let mut frames = Vec::new();
	for _ in 1..steps{
		frames.push(sample_automaton.step().cells.clone());
	}
	println!("Generated in {}ms", now.elapsed().as_millis());
	let now_image = time::Instant::now();
	save_webp_anim(
		&frames, width as u32, height as u32,
		output.as_str(), 100)
		.expect("failed to save anim");
	println!("Saved to {} in {}ms", output, now_image.elapsed().as_millis());
}
