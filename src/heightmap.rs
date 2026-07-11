use crate::grid::Grid;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub struct TerrainParams {
    pub initial_amplitude: f32,
    pub roughness: f32,
    pub seed: u64,
}

const CARDINAL_OFFSETS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn diamond_square_step(
    heightmap: &mut Grid<f32>,
    step_size: usize,
    amplitude: f32,
    rng: &mut StdRng,
) {
    let side = heightmap.side_length;
    let half = step_size / 2;

    // DIAMOND
    for y in (0..side - 1).step_by(step_size) {
        for x in (0..side - 1).step_by(step_size) {
            let avg = (*heightmap.get(x, y)
                + *heightmap.get(x + step_size, y)
                + *heightmap.get(x, y + step_size)
                + *heightmap.get(x + step_size, y + step_size))
                / 4.0;

            *heightmap.get_mut(x + half, y + half) = avg + rng.random_range(-amplitude..=amplitude);
        }
    }

    // SQUARE
    for y in (0..side).step_by(half) {
        let x_start = if (y / half) % 2 == 0 { half } else { 0 };

        for x in (x_start..side).step_by(step_size) {
            let mut sum = 0.0;
            let mut count = 0;

            for (x_offset, y_offset) in CARDINAL_OFFSETS {
                let x_neighbor = x as isize + x_offset * half as isize;
                let y_neighbor = y as isize + y_offset * half as isize;

                if x_neighbor >= 0
                    && x_neighbor < side as isize
                    && y_neighbor >= 0
                    && y_neighbor < side as isize
                {
                    sum += *heightmap.get(x_neighbor as usize, y_neighbor as usize);
                    count += 1;
                }
            }

            *heightmap.get_mut(x, y) =
                sum / count as f32 + rng.random_range(-amplitude..=amplitude);
        }
    }
}

fn normalize(grid: &mut Grid<f32>) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    for y in 0..grid.side_length {
        for x in 0..grid.side_length {
            let value = *grid.get(x, y);
            min = min.min(value);
            max = max.max(value);
        }
    }

    let range = max - min;

    if range == 0.0 {
        return;
    }

    for y in 0..grid.side_length {
        for x in 0..grid.side_length {
            let value = *grid.get(x, y);
            *grid.get_mut(x, y) = (value - min) / range;
        }
    }
}

pub fn heightmap(side_length: usize, params: &TerrainParams) -> Grid<f32> {
    assert!(
        side_length >= 3 && (side_length - 1).is_power_of_two(),
        "diamond-square needs 2^n+1, got {}!",
        side_length
    );
    let mut heightmap: Grid<f32> = Grid::new(side_length);
    let mut rng = StdRng::seed_from_u64(params.seed);
    let mut step_size = side_length - 1;
    let mut amplitude = params.initial_amplitude;

    *heightmap.get_mut(0, 0) = rng.random_range(-amplitude..=amplitude);
    *heightmap.get_mut(side_length - 1, 0) = rng.random_range(-amplitude..=amplitude);
    *heightmap.get_mut(0, side_length - 1) = rng.random_range(-amplitude..=amplitude);
    *heightmap.get_mut(side_length - 1, side_length - 1) = rng.random_range(-amplitude..=amplitude);

    while step_size > 1 {
        diamond_square_step(&mut heightmap, step_size, amplitude, &mut rng);
        step_size /= 2;
        amplitude *= params.roughness;
    }

    normalize(&mut heightmap);
    heightmap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(seed: u64) -> TerrainParams {
        TerrainParams {
            initial_amplitude: 1.0,
            roughness: 0.6,
            seed,
        }
    }

    #[test]
    fn normalized_within_bounds() {
        let terrain = heightmap(257, &params(42));
        for y in 0..terrain.side_length {
            for x in 0..terrain.side_length {
                let v = *terrain.get(x, y);
                assert!((0.0..=1.0).contains(&v), "({x},{y}) = {v} out of [0,1]");
            }
        }
    }

    #[test]
    fn normalize_hits_both_bounds() {
        let terrain = heightmap(65, &params(42));
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for y in 0..terrain.side_length {
            for x in 0..terrain.side_length {
                min = min.min(*terrain.get(x, y));
                max = max.max(*terrain.get(x, y));
            }
        }
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn deterministic_same_seed() {
        let terrain_a = heightmap(129, &params(7));
        let terrain_b = heightmap(129, &params(7));
        for y in 0..terrain_a.side_length {
            for x in 0..terrain_a.side_length {
                assert_eq!(*terrain_a.get(x, y), *terrain_b.get(x, y));
            }
        }
    }

    #[test]
    fn different_seeds_differ() {
        let terrain_a = heightmap(129, &params(1));
        let terrain_b = heightmap(129, &params(2));
        let differs = (0..terrain_a.side_length).any(|y| {
            (0..terrain_a.side_length).any(|x| *terrain_a.get(x, y) != *terrain_b.get(x, y))
        });
        assert!(differs, "different seeds produced identical maps");
    }

    #[test]
    fn fully_populated() {
        let terrain = heightmap(129, &params(42));

        let invalid = (0..terrain.side_length)
            .flat_map(|y| (0..terrain.side_length).map(move |x| (x, y)))
            .filter(|&(x, y)| !terrain.get(x, y).is_finite())
            .count();

        assert_eq!(invalid, 0);
    }

    #[test]
    #[should_panic]
    fn rejects_non_power_of_two() {
        heightmap(100, &params(42));
    }

    #[test]
    fn smallest_valid_grid() {
        let _ = heightmap(3, &params(42));
    }
}
