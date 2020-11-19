#[derive(Clone, Copy, Debug, PartialEq)]
enum LocationType {
    Empty,
    House,
    Tree,
    Well,
}

#[derive(Clone, Debug)]
struct Location {
   tipe: LocationType,
   distances: Vec<Option<usize>>,
}

impl Location {
    pub fn new(houses: usize) -> Self {
	let mut distances: Vec<Option<usize>> = Vec::with_capacity(houses);
	distances.resize(houses, None);
	Self {
            tipe: LocationType::Empty,
	    distances,
	}
    }

    pub fn is_house(&mut self, index: usize) {
       self.tipe = LocationType::House;
       self.distances[index] = Some(0);
    }

    pub fn is_tree(&mut self) {
       self.tipe = LocationType::Tree;
    }

    pub fn is_well(&mut self) {
       self.tipe = LocationType::Well;
    }

    pub fn next_to(&mut self, other: &Location) -> bool {
        let mut changed = false;
	if self.tipe == LocationType::Empty {
            for i in 0..self.distances.len() {
	        if let Some(d) = other.distances[i] {
		    match self.distances[i] {
	                None => {
		            self.distances[i] = Some(d+1);
			    changed = true;
		        },
		        Some(_) => {
		        },
		    }
	        }
	    }
	}
	changed
    }
    pub fn total_distance(&self) -> Result<usize, ()> {
        let mut total = 0;
        for i in 0..self.distances.len() {
	    match self.distances[i] {
	        Some(add) => { total += add; },
		None => { return Err(()); },
	    }
	}
	Ok(total)
    }
}

fn print_grid(grid: &Vec<Vec<Location>>) {
    for row in grid.iter() {
       for loc in row.iter() {
           match loc.tipe {
	       LocationType::Empty => { print!("- "); }
	       LocationType::Tree => { print!("T "); }
	       LocationType::House => { print!("H "); }
	       LocationType::Well => { print!("O "); }
	   }
       }
       println!("");
    }
}

fn find_best_location(houses: Vec<(usize, usize)>, trees: Vec<(usize, usize)>, grid_size: (usize, usize)) -> (usize, usize) {
    let mut grid: Vec<Vec<Location>> = Vec::with_capacity(grid_size.0);
    grid.resize_with(grid_size.0, || {
        let mut neu = Vec::with_capacity(grid_size.1);
	neu.resize_with(grid_size.1, || {Location::new(houses.len())});
	neu
    });
    let mut check: Vec<(usize, usize)> = Vec::new();
    let mut index = 0;
    for (i, j) in houses {
        check.push((i, j));
	grid[i][j].is_house(index);
	index += 1;
    }
    for (i, j) in trees {
	grid[i][j].is_tree();
    }
    println!("Starting Grid:");
    print_grid(&grid);
    while !check.is_empty() {
        let old_check = check.clone();
	check.clear();
        for (i, j) in old_check {
	    let other = grid[i][j].clone();
	    if 0 != i {
	        let x = i-1;
	        if grid[x][j].next_to(&other) {
		    check.push((x, j));
		}
	    }
	    if 0 != j {
	        let y = j-1;
	        if grid[i][y].next_to(&other) {
		    check.push((i, y));
		}
	    }
	    if grid_size.0-1 != i {
	        let x = i+1;
	        if grid[x][j].next_to(&other) {
		    check.push((x, j));
		}
	    }
	    if grid_size.1-1 != j {
	        let y = j+1;
	        if grid[i][y].next_to(&other) {
		    check.push((i, y));
		}
	    }
	}
    }
    let mut min_distance = usize::MAX;
    let mut min_location = (0, 0);
    let mut x = 0;
    for row in grid.iter() {
        let mut y = 0;
        for loc in row.iter() {
	    match loc.tipe {
	        LocationType::Empty => {
		    match loc.total_distance() {
		        Ok(total) => {
		            if total < min_distance {
		                // println!("({}, {}) = {:?}", x, y, loc);
		                min_distance = total;
			        min_location = (x, y);
		            }
		        },
			Err(()) => {
		            // println!("({}, {}) = Unreachable ({:?})", x, y, loc);
			},
		    }
		},
		_ => {},
	    }
	    y += 1;
	}
	x += 1;
    }
    grid[min_location.0][min_location.1].is_well();
    println!("Final Grid:");
    print_grid(&grid);
    min_location
}

fn main() {
    println!("best location = {:?}", find_best_location(vec![(1, 1), (2, 3), (3, 2)], vec![(1,2), (2, 2), (3, 1)], (5, 5)));
}
