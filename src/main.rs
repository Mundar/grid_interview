#[derive(Clone, Debug)]
struct Grid<T: Clone> {
    dimentions: (isize, isize),
    contents: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(height: isize, width: isize, value: T) -> Result<Self, ()> {
        if (0 >= height) || (0 >= width) { return Err(()); }
        let vector_size = (height*width) as usize;
	let mut contents: Vec<T> = Vec::with_capacity(vector_size);
	contents.resize(vector_size, value);
        Ok(Self {
	    dimentions: (height, width),
	    contents,
	})
    }

    pub fn height(&self) -> isize {
        self.dimentions.0
    }

    pub fn width(&self) -> isize {
        self.dimentions.1
    }

    fn index(&self, height: isize, width: isize) -> Option<usize> {
        if (0 <= height) && (height < self.dimentions.0) && (0 <= width) && (width < self.dimentions.1) {
	    Some(((height * self.dimentions.1) + width) as usize)
	}
	else {
	    None
	}
    }

    pub fn get(&self, height: isize, width: isize) -> Option<&T> {
        match self.index(height, width) {
	    Some(i) => Some(&self.contents[i]),
	    None => None,
	}
    }

    pub fn get_mut(&mut self, height: isize, width: isize) -> Option<&mut T> {
        match self.index(height, width) {
	    Some(i) => Some(&mut self.contents[i]),
	    None => None,
	}
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter::new(self)
    }
}

#[derive(Debug)]
struct GridIter<'grid, T: Clone> {
    grid: &'grid Grid<T>,
    height: isize,
    width: isize,
}

impl<'grid, T: Clone> GridIter<'grid, T> {
    fn new(grid: &'grid Grid<T>) -> Self {
        Self {
	    grid,
	    height: 0,
	    width: 0,
	}
    }
}

impl<'grid, T: Clone> Iterator for GridIter<'grid, T> {
    type Item = (isize, isize, &'grid T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.grid.get(self.height, self.width) {
	    Some(item) => {
	        let result = Some((self.height, self.width, item));
		self.width += 1;
		if self.width == self.grid.dimentions.1 {
		    self.height += 1;
		    self.width = 0;
		}
		result
	    },
	    None => None,
	}
    }
}

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

fn print_grid(grid: &Grid<Location>) {
    for row in 0..grid.height() {
       for col in 0..grid.width() {
           if let Some(loc) = grid.get(row, col) {
               match loc.tipe {
	           LocationType::Empty => { print!("- "); }
	           LocationType::Tree => { print!("T "); }
	           LocationType::House => { print!("H "); }
	           LocationType::Well => { print!("O "); }
	       }
	   }
	   else {
	       print!("? ");
	   }
       }
       println!("");
    }
}

macro_rules! generate_path {
    ($grid: ident, $other: ident, $check: ident, $height: expr, $width: expr) => (
	if let Some(loc) = $grid.get_mut($height, $width) {
	    if loc.next_to(&$other) {
		$check.push(($height, $width));
	    }
	}
    )
}

fn find_best_location(houses: Vec<(isize, isize)>, trees: Vec<(isize, isize)>, grid_size: (isize, isize)) -> (isize, isize) {
    let mut grid = Grid::new(grid_size.0, grid_size.1, Location::new(houses.len())).unwrap();
    let mut check: Vec<(isize, isize)> = Vec::new();
    let mut index = 0;
    for (i, j) in houses {
        if let Some(loc) = grid.get_mut(i, j) {
            check.push((i, j));
	    loc.is_house(index);
	    index += 1;
	}
    }
    for (i, j) in trees {
        if let Some(loc) = grid.get_mut(i, j) {
	    loc.is_tree();
	}
    }
    println!("Starting Grid:");
    print_grid(&grid);
    while !check.is_empty() {
        let old_check = check.clone();
	check.clear();
        for (i, j) in old_check {
	    let other = grid.get(i, j).unwrap().clone();
	    generate_path!(grid, other, check, i-1, j);
	    generate_path!(grid, other, check, i, j-1);
	    generate_path!(grid, other, check, i+1, j);
	    generate_path!(grid, other, check, i, j+1);
	}
    }
    let mut min_distance = usize::MAX;
    let mut min_location = (0, 0);
    for (x, y, loc) in grid.iter() {
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
    }
    grid.get_mut(min_location.0, min_location.1).unwrap().is_well();
    println!("Final Grid:");
    print_grid(&grid);
    min_location
}

fn main() {
    println!("best location = {:?}", find_best_location(vec![(1, 1), (2, 3), (3, 2)], vec![(1,2), (2, 2), (3, 1)], (5, 5)));
}

#[cfg(test)]
mod tests {
    use crate::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
	fn grid_creation_tests(good_height in 1_isize..256, good_width in 1_isize..256,
	                       bad_height in isize::MIN..=0, bad_width in isize::MIN..=0,
			       value in 0_usize..=usize::MAX)
	{
	    prop_assert!(Grid::new(good_height, good_width, value).is_ok());
	    prop_assert!(Grid::new(bad_height, good_width, value).is_err());
	    prop_assert!(Grid::new(good_height, bad_width, value).is_err());
	    prop_assert!(Grid::new(bad_height, bad_width, value).is_err());
	}
    }

    proptest! {
        #[test]
	fn grid_get_tests(grid_height in 1_isize..256, grid_width in 1_isize..256,
	                  random_height in 0_isize..isize::MAX, random_width in 0_isize..isize::MAX,
			  start_value in 0_usize..=usize::MAX, new_value in 0_usize..=usize::MAX)
	{
	    let access_height = random_height % grid_height;
	    let access_width = random_width % grid_width;
	    let mut grid = Grid::new(grid_height, grid_width, start_value).unwrap();
	    prop_assert_eq!(grid.get(-1, -1), None);
	    prop_assert_eq!(grid.get(-1, 0), None);
	    prop_assert_eq!(grid.get(0, -1), None);
	    prop_assert_eq!(grid.get(0, 0), Some(&start_value));
	    prop_assert_eq!(grid.get(access_height, access_width), Some(&start_value));
	    prop_assert_eq!(grid.get(grid_height-1, grid_width-1), Some(&start_value));
	    prop_assert_eq!(grid.get(grid_height, grid_width-1), None);
	    prop_assert_eq!(grid.get(grid_height-1, grid_width), None);
	    prop_assert_eq!(grid.get(grid_height, grid_width), None);
	    *grid.get_mut(access_height, access_width).unwrap() = new_value;
	    prop_assert_eq!(grid.get(access_height, access_width), Some(&new_value));
	    let grid_ref: &mut usize = grid.get_mut(access_height, access_width).unwrap();
	    *grid_ref = start_value;
	    prop_assert_eq!(grid.get(access_height, access_width), Some(&start_value));
	}
    }

    proptest! {
        #[test]
	fn grid_iterator_test(grid_height in 1_isize..256, grid_width in 1_isize..256) {
	    let mut grid = Grid::new(grid_height, grid_width, (0_isize, 0_isize)).unwrap();
	    for height in 0..grid_height {
	        for width in 0..grid_width {
		    *grid.get_mut(height, width).unwrap() = (height, width);
		}
	    }
	    for (height, width, coordinates) in grid.iter() {
	        prop_assert_eq!(&(height, width), coordinates);
	    }
	}
    }
}
