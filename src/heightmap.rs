use crate::grid::Grid;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

pub struct TerrainParams {
    pub initial_amplitude: f32,
    pub roughness: f32,
    pub seed: u64,
}

fn diamond_square_step(grid: &mut Grid<f32>, step: usize, amplitude: f32, rng: &mut StdRng) {
    todo!()
}

fn normalize(grid: &mut Grid<f32>) {
    todo!()
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

    *heightmap.get_mut(0, 0) = rng.random_range(-amplitude..amplitude);
    *heightmap.get_mut(side_length - 1, 0) = rng.random_range(-amplitude..amplitude);
    *heightmap.get_mut(0, side_length - 1) = rng.random_range(-amplitude..amplitude);
    *heightmap.get_mut(side_length - 1, side_length - 1) = rng.random_range(-amplitude..amplitude);

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
        let g = heightmap(257, &params(42));
        for y in 0..g.side_length {
            for x in 0..g.side_length {
                let v = *g.get(x, y);
                assert!((0.0..=1.0).contains(&v), "({x},{y}) = {v} out of [0,1]");
            }
        }
    }

    #[test]
    fn normalize_hits_both_bounds() {
        let g = heightmap(65, &params(42));
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for y in 0..g.side_length {
            for x in 0..g.side_length {
                min = min.min(*g.get(x, y));
                max = max.max(*g.get(x, y));
            }
        }
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn deterministic_same_seed() {
        let a = heightmap(129, &params(7));
        let b = heightmap(129, &params(7));
        for y in 0..a.side_length {
            for x in 0..a.side_length {
                assert_eq!(*a.get(x, y), *b.get(x, y));
            }
        }
    }

    #[test]
    fn different_seeds_differ() {
        let a = heightmap(129, &params(1));
        let b = heightmap(129, &params(2));
        let differs =
            (0..a.side_length).any(|y| (0..a.side_length).any(|x| *a.get(x, y) != *b.get(x, y)));
        assert!(differs, "different seeds produced identical maps");
    }

    #[test]
    fn fully_populated() {
        let g = heightmap(129, &params(42));
        let untouched = (0..g.side_length)
            .flat_map(|y| (0..g.side_length).map(move |x| (x, y)))
            .filter(|&(x, y)| *g.get(x, y) == 0.0)
            .count();
        assert!(untouched <= 1, "{untouched} cells still zero");
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
