use std::time;
use std::time::Duration;
use std::thread;

pub mod automaton;
use crate::automaton::CellularAutomaton;


fn main() {
	println!("Hello, world!");
	let mut c = CellularAutomaton::new(100, 100);
	/*let mut cells = c.cells;
	for i in 0..c.x {
		for j in 0..c.y {
			cells.insert((i,j), 1);
		}
	}
	c.cells = cells;*/
	c.randomize();
	c.print();
	println!("Starting...");
	// thread::sleep(Duration::from_millis(2000));
	println!("Started.");
	let prev = time::Instant::now();
	for _ in 1..100{
		c.step();
		clearscreen::clear().expect("failed to clear screen");
		c.print();
		thread::sleep(Duration::from_millis(100));
	}
	let elapsed = prev.elapsed().as_millis();
	c.print();
	println!("Done in {}", elapsed);
}
