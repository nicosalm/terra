use std::fmt;

pub struct Grid<T> {
    pub side_length: usize,
    data: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(side_length: usize) -> Self {
        Self {
            side_length,
            data: vec![T::default(); side_length * side_length],
        }
    }
}
impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.side_length);
        assert!(y < self.side_length);
        &self.data[y * self.side_length + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < self.side_length);
        assert!(y < self.side_length);
        &mut self.data[y * self.side_length + x]
    }

    pub fn neighbor(&self, x: usize, y: usize, dx: i32, dy: i32) -> Option<&T> {
        let nx = x as i32 + dx;
        let ny: i32 = y as i32 + dy;
        if nx < 0 || ny < 0 {
            return None;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= self.side_length || ny >= self.side_length {
            return None;
        }
        Some(&self.data[ny * self.side_length + nx])
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.side_length {
            for x in 0..self.side_length {
                write!(f, "{:?} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn round_trip_read_write() {
        let mut grid: Grid<i32> = Grid::new(10);
        assert_eq!(grid.side_length, 10);
        assert_eq!(*grid.get(1, 1), 0);
        *grid.get_mut(0, 0) = 42;
        assert_eq!(*grid.get(0, 0), 42);
    }

    #[test]
    fn distinguishes_x_from_y() {
        let mut grid: Grid<i32> = Grid::new(10);
        *grid.get_mut(3, 7) = 99;
        assert_eq!(*grid.get(3, 7), 99);
        assert_eq!(*grid.get(7, 3), 0);
    }

    #[test]
    fn neighbor_low_edges() {
        let mut grid: Grid<i32> = Grid::new(10);
        *grid.get_mut(1, 0) = 7;
        assert_eq!(grid.neighbor(0, 0, -1, 0), None);
        assert_eq!(grid.neighbor(0, 0, 0, -1), None);
        assert_eq!(grid.neighbor(0, 0, 1, 0), Some(&7));
    }

    #[test]
    fn neighbor_vertical_axis() {
        let mut grid: Grid<i32> = Grid::new(10);
        *grid.get_mut(0, 1) = 5;
        assert_eq!(grid.neighbor(0, 0, 0, 1), Some(&5));
    }

    #[test]
    fn neighbor_high_edges() {
        let grid: Grid<i32> = Grid::new(10);
        assert_eq!(grid.neighbor(9, 9, 1, 0), None);
        assert_eq!(grid.neighbor(9, 9, 0, 1), None);
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_panics() {
        let grid: Grid<i32> = Grid::new(10);
        grid.get(10, 0);
    }
}
