use std::mem::{ swap, size_of };
use std::cmp::min;
use std::iter::{ FlatMap, Map };
use std::ops::Range;
use rnd::{ Random };

const SIZE: i32 = 25000;

const NEIGHBOURS: [(i32, i32); 4] = [
	          (0, -1),
	(-1,  0),          (1,  0),
	          (0,  1)
];

const BIG_DIST: u16 = 0xAFFF;

pub struct Simulation {
	pub tick: u64,
	pub cur_frame: Frame,
	pub prv_frame: Frame
}

#[derive(Default)]
pub struct Frame {
	pub globals: GlobalValues,
	pub grid: Vec<Cell>
}

#[derive(Default)]
pub struct GlobalValues {
	pub total_work: u64,
	pub total_workers: u64
}

#[derive(Default)]
pub struct Cell {
	pub building: Building
}

#[derive(Clone, Copy)]
pub enum Building {
	Residential { 
		population: u8 
	},
	Office {
		work: u8 
	},
	Road {
		dist_work: u16 
	},
	None
}

impl Default for Building {
	fn default() -> Building {
		Building::None
	}
}

impl Simulation {
	pub fn new() -> Simulation {

		println!("{:?} bytes per cell", size_of::<Cell>());

		let mut sim = Simulation { 
			tick: 0,
			cur_frame: Frame { grid: (0..SIZE*SIZE).map(|_| Cell::default()).collect(), ..Default::default() },
			prv_frame: Frame { grid: (0..SIZE*SIZE).map(|_| Cell::default()).collect(), ..Default::default() },
		};

		sim.cur_frame.grid[SIZE as usize * 5 + 7].building = Building::Residential { population: 5 };
		sim.cur_frame.grid[SIZE as usize * 3 + 5].building = Building::Office { work: 15 };

		sim.cur_frame.grid[SIZE as usize * 3 + 4].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 4 + 4].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 5 + 4].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 6 + 4].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 6 + 5].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 6 + 6].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 6 + 7].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 6 + 8].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 7 + 8].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 8 + 8].building = Building::Road { dist_work: BIG_DIST };
		sim.cur_frame.grid[SIZE as usize * 9 + 8].building = Building::Road { dist_work: BIG_DIST };

		sim
	}

	pub fn tick(&mut self) {

		swap(&mut self.cur_frame, &mut self.prv_frame);

		let old_grid = &self.prv_frame.grid;
		let cur_grid = &mut self.cur_frame.grid;
		let mut _globals = GlobalValues::default();

		// Simulation:
		// - propagate distances
		// - aggregate global values
		run(cur_grid, |c, (x, y)| {
			let mut globals = &mut _globals;

			c.building = get_cell(&old_grid, (x, y)).unwrap().building;

			match c.building {
				Building::Road { dist_work: mut d } => {

					// Propagate distances
					let dist = NEIGHBOURS.iter()
						.map(|&(x_a, y_a)| match get_cell(&old_grid, (x+x_a, y+y_a)) { 
							Some(c) => match c.building {
								Building::Road { dist_work: dist, .. } => dist,
								Building::Office { .. } => 0,
								_ => BIG_DIST
							},
							None => BIG_DIST
						})
						.min()
						.unwrap_or(BIG_DIST);

					// Distance is the lowest neighbour + 1
					d = min(dist + 1, BIG_DIST);

				},
				Building::Residential { population } => {
					globals.total_workers += population as u64;
				},
				Building::Office { work } => {
					globals.total_work += work as u64;
				},
				_ => ()
			}
		});

		self.cur_frame.globals = _globals;

		self.tick += 1;
	}


}


// A vector of the coordinates within a grid
type GridIter = Box<Iterator<Item=(i32, i32)>>;
pub fn grid_coords() -> GridIter {
	Box::new((0..SIZE).flat_map(|y| (0..SIZE).map(move |x| (x, y))))
}

// Run a closure over each cell in a grid
fn run<F>(grid: &mut Vec<Cell>, mut f: F) where F: FnMut(&mut Cell, (i32, i32)) -> () {
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