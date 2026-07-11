mod grid;
mod heightmap;

use grid::Grid;
use heightmap::{TerrainParams, heightmap};

const RAMP: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn to_symbols(grid: &Grid<f32>) -> String {
    let mut out = String::with_capacity(grid.side_length * (grid.side_length * 2 + 1));
    for y in 0..grid.side_length {
        for x in 0..grid.side_length {
            let v = grid.get(x, y).clamp(0.0, 1.0);
            let idx = ((v * (RAMP.len() - 1) as f32).round() as usize).min(RAMP.len() - 1);
            out.push(RAMP[idx]);
            out.push(RAMP[idx]);
        }
        out.push('\n');
    }
    out
}

fn main() {
    let params = TerrainParams {
        initial_amplitude: 1.0,
        roughness: 0.6,
        seed: 42,
    };

    let terrain = heightmap(65, &params);
    print!("{}", to_symbols(&terrain));
}
