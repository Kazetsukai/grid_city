use std::mem::{ swap, size_of };
use std::cmp::min;
use std::iter::{ FlatMap, Map };
use std::ops::Range;

const SIZE: i32 = 5000;
const NEIGHBOURS: [(i32, i32); 4] = [
	          (0, -1),
	(-1,  0),          (1,  0),
	          (0,  1)
];

const BIG_DIST: u16 = 0xAFFF;

pub struct Simulation {
	pub tick: u64,
	pub grid: Vec<Cell>,
	pub grid_old: Vec<Cell>
}

#[derive(Default)]
pub struct Cell {
	pub nav: Navigation,
	pub building: Option<Building>
}

#[derive(Clone, Copy, Default)]
pub struct Navigation {
	pub distWork: u16
}

#[derive(Clone, Copy)]
pub enum Building {
	Residential,
	Office,
	Road
}

impl Simulation {
	pub fn new() -> Simulation {

		println!("{:?}", size_of::<Cell>());

		let mut sim = Simulation { 
			tick: 0,
			grid: (0..SIZE*SIZE).map(|_| Cell { nav: Navigation { distWork: BIG_DIST, ..Default::default() }, ..Default::default() }).collect(),
			grid_old: (0..SIZE*SIZE).map(|_| Cell { nav: Navigation { distWork: BIG_DIST, ..Default::default() }, ..Default::default() }).collect()
		};

		sim.grid[SIZE as usize * 5 + 7].building = Some(Building::Residential);
		sim.grid[SIZE as usize * 3 + 5].building = Some(Building::Office);

		sim.grid[SIZE as usize * 3 + 4].building = Some(Building::Road);
		sim.grid[SIZE as usize * 4 + 4].building = Some(Building::Road);
		sim.grid[SIZE as usize * 5 + 4].building = Some(Building::Road);
		sim.grid[SIZE as usize * 6 + 4].building = Some(Building::Road);
		sim.grid[SIZE as usize * 6 + 5].building = Some(Building::Road);
		sim.grid[SIZE as usize * 6 + 6].building = Some(Building::Road);
		sim.grid[SIZE as usize * 6 + 7].building = Some(Building::Road);
		sim.grid[SIZE as usize * 6 + 8].building = Some(Building::Road);
		sim.grid[SIZE as usize * 7 + 8].building = Some(Building::Road);
		sim.grid[SIZE as usize * 8 + 8].building = Some(Building::Road);
		sim.grid[SIZE as usize * 9 + 8].building = Some(Building::Road);

		sim
	}

	pub fn tick(&mut self) {
		println!("Ticking {}...", self.tick);

		swap(&mut self.grid, &mut self.grid_old);

		let old_grid = &self.grid_old;

		run(&mut self.grid, |c, (x, y)| {
			c.building = get_cell(&old_grid, (x, y)).unwrap().building;

			// mark offices as 0 and update distances
			match c.building {
				Some(Building::Office) => { c.nav.distWork = 0; },
				Some(Building::Road) => {
					let dist = NEIGHBOURS.iter()
						.map(|&(x_a, y_a)| match get_cell(&old_grid, (x+x_a, y+y_a)) { Some(c) => c.nav.distWork, None => BIG_DIST })
						.min()
						.unwrap_or(BIG_DIST) + 1;

					c.nav.distWork = min(dist, BIG_DIST);
				},
				_ => ()
			}
		});


		println!("Ticked {}", self.tick);
		self.tick += 1;
	}


}


// A vector of the coordinates within a grid
type GridIter = Box<Iterator<Item=(i32, i32)>>;
pub fn grid_coords() -> GridIter {
	Box::new((0..SIZE).flat_map(|y| (0..SIZE).map(move |x| (x, y))))
}

// Run a closure over each cell in a grid
fn run<F>(grid: &mut Vec<Cell>, f: F) where F: Fn(&mut Cell, (i32, i32)) -> () {
    for (ref mut cell, coord) in grid.iter_mut().zip(grid_coords()) {
    	f(cell, coord);
    }
}

// Return a cell at a coordinate from a grid
fn get_cell(grid: &Vec<Cell>, coord: (i32, i32)) -> Option<&Cell> {
	match coord {
		(x, y) if x >= 0 && x < SIZE && y >= 0 && y < SIZE => Some(&grid[(SIZE * y + x) as usize]),
		_ => None
	}
}