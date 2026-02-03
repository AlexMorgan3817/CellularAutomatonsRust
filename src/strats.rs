use crate::automaton::CellularAutomaton;

/// Conway's Game of Life
#[inline(always)]
pub fn conway_next(c: &CellularAutomaton, x:usize, y:usize) -> i32{
	let mut living_count = 0;
	let mystate = c.cells[x][y];
	if (x > 0 && y > 0) && (x < c.x - 1 && y < c.y - 1) { // Inner Volume
		for i in -1..=1 {
			for j in -1..=1 {
				if i == 0 && j == 0 { continue; }
				if c.cells[(x as i32 + i) as usize][(y as i32 + j) as usize] == 1 {
					living_count += 1;
				}
			}
		}
	} else { // bounds
		for i in -1..=1{
			for j in -1..=1{
				let nx = x as i32 + i;
				let ny = y as i32 + j;
				if nx < 0 || ny < 0 || nx >= c.x as i32 || ny >= c.y as i32
					{continue;}
				if c.cells[nx as usize][ny as usize] == 1{
					living_count += 1;
				}
			}
		}
	}
	match (mystate, living_count) {
		(1, 2) | (1, 3) | (0, 3) => 1,
		_ => 0,
	}
}

/// Attempt to recreate by memory my LifeCorridors from C# Cellulars. Made just for testing and to be example.
#[inline(always)]
pub fn corridors_next(automaton:&CellularAutomaton, x:usize, y:usize) -> i32{
	let iam = automaton.cells[x][y];
	// let mut living = 0;
	// if x+1 < automaton.x {living += automaton.cells[x+1][y];}
	// if x > 0 {living += automaton.cells[x-1][y];}
	// if y+1 < automaton.y {living += automaton.cells[x][y+1];}
	// if y > 0 {living += automaton.cells[x][y-1];}

	let mut d = 0;
	if x > 0{
		if y > 0{
			d += automaton.cells[x-1][y-1];
		}
		if y < automaton.y - 1{
			d += automaton.cells[x-1][y+1];
		}
	}
	if x < automaton.x - 1{
		if y > 0{
			d += automaton.cells[x+1][y-1];
		}
		if y < automaton.y - 1{
			d += automaton.cells[x+1][y+1];
		}
	}
	// if living == 4{
	// 	return 0
	// }
	match iam{
		0 => match d == 2 {
			true => 1,
			false => 0,
		},
		1 => match d <= 2 {
			true => 1,
			false => 0,
		},
		_ => 0,
	}

}
