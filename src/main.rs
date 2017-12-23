extern crate itertools;

mod grid;
mod point;
mod line_gen;
mod room_gen;

use grid::Grid;
use room_gen::RoomGenerator;

fn main() {
    let mut grid = Grid::new_with_fill(30, 30, '#');
    let room_gen = RoomGenerator {
        width: 20,
        height: 15,
        distortion_chance: 0.9,
        vertex_offset_min: -2,
        vertex_offset_max: 2,
        vertex_offset_chance: 0.5,
    };
    generate_room(&mut grid, &room_gen);
    println!("{}", grid);
}

fn generate_room(grid: &mut Grid, room_gen: &RoomGenerator) {
    let start_x = 3;
    let start_y = 3;
    for p in room_gen.generate() {
        grid.set((start_x + p.x) as usize, (start_y + p.y) as usize, '.');
    }
    grid.fill(start_x as usize + 5, start_y as usize + 5, '.');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fill_only_room() {
        let grid = Grid::new(70, 70);
        let size = 10;
        let x = grid.width() / 2;
        let y = grid.height() / 2;
        let room_gen = RoomGenerator {
            width: size,
            height: size,
            distortion_chance: 0.8,
            vertex_offset_max: 2,
            vertex_offset_chance: 0.5,
        };
        let failed: Vec<Grid> = (0..1000)
            .map(|_| grid.clone())
            .map(|mut g| {
                generate_room(&mut g, &room_gen);
                g
            })
            .filter(|g| g.count('#') < g.count('.'))
            .collect();

        for g in &failed {
            println!("{}\n", g);
        }
        assert!(failed.is_empty());
    }
}
