use std::{mem::swap};
use rand::{random_range, random_bool};
use rayon::prelude::*;

use crate::strats::{conway_next};

pub struct CellularAutomaton {
	pub x:usize,
	pub y:usize,
	/// Field of simulation with self.x and self.y dimensions.
	pub cells: Vec<Vec<i32>>,
	/// Temporary field for next generation, swaps with cells after generating all self.next() in self.step()
	next_cells: Vec<Vec<i32>>,
	// pub cell_processor: Arc<dyn Fn(&Self, usize, usize) -> i32 + Sync + Send + 'static>,
	pub cell_processor: fn(&Self, usize, usize) -> i32,
}

/// Construction
impl CellularAutomaton
{
	/// Constructor with the given dimensions and default (Conway's game of life) processor.
	/// ---
	pub fn new(x:usize, y:usize) -> CellularAutomaton{
		let mut this = CellularAutomaton{
			cells: vec![vec![0; y]; x],
			next_cells: vec![vec![0; y]; x],
			x:x, y:y,
			cell_processor:conway_next
		};
		this.set_xy(x, y, 0);
		this
	}

	/// Constructor with the given dimensions and proccesing lambda.
	/// ---
	pub fn new_with_processor(
			x:usize,
			y:usize,
			processor: fn(c:&CellularAutomaton, x:usize, y:usize) -> i32
	) -> CellularAutomaton {
		let mut this = CellularAutomaton::new(x, y);
		this.set_processor(processor);
		this
	}
	/// Sets the cell processor lambda for the automaton.
	/// ---
	///
	/// **processor**: fn(*C*:&CellularAutomaton, *X*:usize, *Y*:usize) -> NextState:i32
	///
	/// ---
	/// Allows to use multiple strategies for one field,
	/// by swapping cell_processor instead of cloning or moving matrices between automatons.
	pub fn set_processor(&mut self, processor: fn(c:&CellularAutomaton, x:usize, y:usize) -> i32) -> &mut Self
	{
		self.cell_processor = processor;
		self
	}
}
/// Core processing
impl CellularAutomaton
{
	/// Processes the next state of one cell by coordinates via lambda processor self.cell_processor.
	#[inline]
	pub fn next(&self, x:usize, y:usize) -> i32
	{
		(&self.cell_processor)(self, x, y)
	}
	/// Processes field with rayon borrowing unmutable self to self.cell_processor.
	pub fn step(&mut self) -> &mut Self {
		self.next_cells = (0..self.x)
			.into_par_iter()
			.map(|i| {
				(0..self.y)
					.map(|j| self.next(i, j))
					.collect()
			})
			.collect();
		swap(&mut self.cells, &mut self.next_cells);
		self
	}
	/// Method to process multiple steps of the automaton as one call.
	///
	/// Will be more useful in FFI. But still solves convenience of using.
	pub fn steps(&mut self, steps:u64) -> &mut Self {
		for _ in 0..steps {
			self.step();
		}
		self
	}
}
/// Field fillings
impl CellularAutomaton
{
	/// TODO: Add randomize strat
	pub fn randomize(&mut self) -> &mut Self {
		for x in 0..self.x
		{
			for y in 0..self.y
			{
				self.cells[x][y] = random_range(0..2);
			}
		}
		self
	}
	pub fn randomize_prob(&mut self, alive_probability:f64) -> &mut Self {
		for x in 0..self.x
		{
			for y in 0..self.y
			{
				let p = random_bool(alive_probability);
				self.cells[x][y] = p as i32;
			}
		}
		self
	}
	/// Sets the size of the automaton and initializes all cells with the given state.
	/// ---
	pub fn set_xy(&mut self, x:usize, y:usize, init_state: i32) -> &mut Self {
		self.x = x;
		self.y = y;
		self.cells = Vec::new();
		for _ in 0..x {
			let mut row = Vec::new();
			for _ in 0..y {
				row.push(init_state);
			}
			self.cells.push(row);
		}
		self
	}
	// /// Sets the size of the automaton and initializes all cells with the given matrix.
	// /// ---
	// /// Allows to use multiple strategies for one field.
	// /// But better swap cell_processor instead, it will be more perfomant than cloning matrices between automatons.
	// pub fn set_matrix(&mut self, matrix: &Vec<Vec<i32>>) -> &mut Self {
	// 	self.x = matrix.len();
	// 	self.y = matrix[0].len();
	// 	self.cells = matrix.clone();
	// 	self
	// }
}

#[cfg(test)]
mod tests {
	use std::time;
	#[cfg(feature = "cli")]
	use colored::{Colorize, ColoredString};
	use crate::automaton::CellularAutomaton;

	fn bechmark(steps_count: u32, threshold: u128, x: usize, y: usize) -> u128{
		let mut c:CellularAutomaton = CellularAutomaton::new(x, y);
		c.set_processor(|automaton, x, y|{
			let mut living = 0;
			if x+1 < automaton.x {living += automaton.cells[x+1][y];}
			if x > 0             {living += automaton.cells[x-1][y];}
			if y+1 < automaton.y {living += automaton.cells[x][y+1];}
			if y > 0            {living += automaton.cells[x][y-1];}
			match living > 2 {
				true => 0,
				false => 1,
			}
		});
		c.randomize();
		let prev = time::Instant::now();
		for _ in 0..steps_count{
			c.step();
		}
		let elapsed:u128 = prev.elapsed().as_millis();
		if elapsed > threshold
		{
			let message = "Benchmark failed";
			#[cfg(feature = "cli")]
			let msg = message.red();
			#[cfg(not(feature = "cli"))]
			let msg = message;
			println!("{}: {} > {}.", msg, elapsed, threshold);
			assert!(false);
		}
		// println!("Done in {}ms ({}ms)", elapsed, threshold);
		elapsed
	}

	fn testing(threshold: u128, steps_count: u32, x: usize, y: usize, tests_count:u32) -> u128{
		println!("{} {} {} {}", steps_count, threshold, x, y);
		let mut results = 0;
		let mut mean = 0;
		for i in 0..tests_count{
			let result:u128 = bechmark(steps_count, threshold, x, y);
			mean = (mean * (results) + result) / (results + 1);
			results += 1;
			#[cfg(feature = "cli")]
			let status:ColoredString;
			#[cfg(feature = "cli")]
			{
				if result < threshold{
					status = "OK".green();
				} else {
					status = "FAIL".red();
				}
			}
			#[cfg(not(feature = "cli"))]
			let status:&'static str;
			#[cfg(not(feature = "cli"))]
			{
				if result < threshold{
					status = "OK";
				} else {
					status = "FAIL";
				}
			}
			println!("Test {}: {:.3}s ({:.3}s): {}",
				i + 1,
				result as f64 / 1000.0,
				threshold as f64 / 1000.0,
				status);
		}
		println!("Mean: {:.3}s", mean as f64 / 1000.0);
		mean
	}

	pub const TESTS_AMT:u32 = 3;

	#[test]
	fn test11_100(){
		// let mut c = CellularAutomaton::new(x, y);
		testing(20, 100, 100, 100, TESTS_AMT);
	}
	#[test]
	fn test12_100_200x200(){
		testing(40, 100, 200, 200, TESTS_AMT);
	}
	#[test]
	fn test13_100_300x300(){
		testing(50, 100, 300, 300, TESTS_AMT);
	}
	#[test]
	fn test14_100_400x400(){
		testing(100, 100, 400, 400, TESTS_AMT);
	}
	#[test]
	fn test15_100_1000x1000(){
		testing(500, 100, 1000, 1000, TESTS_AMT);
	}
	#[test]
	fn test16_1000_1000x1000(){
		testing(3000, 1000, 1000, 1000, TESTS_AMT);
	}
	#[test]
	fn test21_1000(){
		testing(200, 1000, 100, 100, TESTS_AMT);
	}
	#[test]
	fn test22_1000_200x200(){
		testing(500, 1000, 200, 200, TESTS_AMT);
	}
	#[test]
	fn test31_50_100x100(){
		testing(10, 50, 100, 100, TESTS_AMT);
	}
	#[test]
	fn test32_50_200x200(){
		testing(20, 50, 200, 200, TESTS_AMT);
	}
	#[test]
	fn test33_50_1000x1000(){
		testing(200, 50, 1000, 1000, TESTS_AMT);
	}
	fn ns2s(n:i128) -> f64{
		(n as f64) / (10 as f64).powf(9.0)
	}
	#[test]
	fn switching_test(){
		println!("Switching Test");
		for _ in 0..TESTS_AMT{
			switching();
		}
	}
	fn switching(){
		// use crate::strats::corridors_next;
		use crate::strats::conway_next;
		let mut c1 = CellularAutomaton::new(100, 100);
		let start_time = time::Instant::now();
		c1.steps(100);
		let elapsed_time = start_time.elapsed().as_nanos();
		println!("Elapsed time for c1: {}ns", elapsed_time);

		let mut c2 = CellularAutomaton::new(100, 100);

		let p1_start = time::Instant::now();
		c2.steps(50);
		let p1_elapsed = p1_start.elapsed().as_nanos();
		println!("Elapsed time for p1: {}ns", p1_elapsed);

		c2.set_processor(conway_next);

		let p2_start = time::Instant::now();
		c2.steps(50);
		let p2_elapsed = p2_start.elapsed().as_nanos();
		// let c2_elapsed = p1_start.elapsed().as_nanos();
		println!("Elapsed time for p2: {}ns", p2_elapsed);

		println!("Difference between ca1 ({}ns) and ca2 ({}ns): {}ns, {}",
			ns2s(elapsed_time as i128),
			ns2s((p2_elapsed + p1_elapsed) as i128),
			ns2s(((p2_elapsed + p1_elapsed) - (elapsed_time)) as i128),
			((p2_elapsed+p1_elapsed) as f64)/(elapsed_time as f64)
		);
	}
}
