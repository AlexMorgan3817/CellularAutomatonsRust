use std::{mem::swap};
use rand::{random_range, random_bool};
use colored::Colorize;
use rayon::prelude::*;

use crate::strats::conway_next;

pub struct CellularAutomaton {
	// cells: LinkedList<LinkedList<i32>>,
	// cells: &[&[i32; Y]; X],
	// cells: Vec<Vec<i32>>,/
	pub cells: Vec<Vec<i32>>,
	next_cells: Vec<Vec<i32>>,
	pub x:usize,
	pub y:usize,
	// pub cell_processor: Arc<dyn Fn(&Self, usize, usize) -> i32 + Sync + Send + 'static>,
	pub cell_processor: fn(&Self, usize, usize) -> i32,
}

impl CellularAutomaton
{
	pub fn set_processor(&mut self, processor: fn(&Self, usize, usize) -> i32) -> &mut Self
	{
		self.cell_processor = processor;
		self
	}
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

	pub fn set_xy(&mut self, x:usize, y:usize, state: i32) -> &mut Self {
		self.x = x;
		self.y = y;
		self.cells = Vec::new();
		for _ in 0..x {
			let mut row = Vec::new();
			for _ in 0..y {
				row.push(state);
			}
			self.cells.push(row);
		}
		self
	}

	#[inline]
	pub fn next(&self, x:usize, y:usize) -> i32
	{
		(&self.cell_processor)(self, x, y)
	}

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
	pub fn steps(&mut self, steps:u64) -> &mut Self {
		for _ in 0..steps {
			self.step();
		}
		self
	}

	pub fn print(&self) -> &Self {
		for y in 0..self.y {
			for x in 0..self.x {
				let v = self.cells[x][y];
				// if v == 1 {print!("{}", "#".green());}
				// else      {print!("{}", "X".red()  );}
				if v == 1 {print!("{}", "■".green());}
				else      {print!("{}", "▢".red()  );}
			}
			print!("\n");
		}
		self
	}
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
}

#[cfg(test)]
mod tests {
	use std::time;
    use colored::{ColoredString, Colorize};

    use crate::automaton::{CellularAutomaton};

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
			println!("{}: {} > {}.", "БЕНЧМАРК НЕ ПРОЙДЕН".red(),elapsed, threshold);
			assert!(false);
		}
		// println!("Done in {}ms ({}ms)", elapsed, threshold);
		elapsed
	}

	fn testing(threshold: u128, steps_count: u32, x: usize, y: usize, tests_count:u32){
		println!("{} {} {} {}", steps_count, threshold, x, y);
		for i in 0..tests_count{
			let result:u128 = bechmark(steps_count, threshold, x, y);
			let status:ColoredString;
			if result < threshold{
				status = "OK".green();
			} else {
				status = "FAIL".red();
			}
			println!("Test {}: {:.3}s ({:.3}s): {}",
				i,
				result as f64 / 1000.0,
				threshold as f64 / 1000.0,
				status);
		}
	}

	#[test]
	fn test11_100(){
		// let mut c = CellularAutomaton::new(x, y);
		testing(200, 100, 100, 100, 1);
	}
	#[test]
	fn test12_100_200x200(){
		testing(400, 100, 200, 200, 1);
	}
	#[test]
	fn test13_100_300x300(){
		testing(1500, 100, 300, 300, 1);
	}
	#[test]
	fn test14_100_400x400(){
		testing(2000, 100, 400, 400, 1);
	}
	#[test]
	fn test15_100_1000x1000(){
		testing(20000, 100, 1000, 1000, 1);
	}
	#[test]
	fn test16_1000_1000x1000(){
		testing(25000, 1000, 1000, 1000, 1);
	}
	#[test]
	fn test21_1000(){
		testing(1500, 1000, 100, 100, 1);
	}
	#[test]
	fn test22_1000_200x200(){
		testing(3500, 1000, 200, 200, 1);
	}
	#[test]
	fn test31_50_100x100(){
		testing(100, 50, 100, 100, 1);
	}
	#[test]
	fn test32_50_200x200(){
		testing(300, 50, 200, 200, 1);
	}
	#[test]
	fn test33_50_1000x1000(){
		testing(15000, 50, 1000, 1000, 1);
	}
}
