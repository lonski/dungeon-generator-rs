extern crate rand;

use grid::Grid;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod grid;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/dung.txt")
        .unwrap();

    let (start_x, start_y) = (3, 3);
    let base_size = 6;

    for i in 0..20 {
        let size = base_size + i;
        let mut grid = Grid::new_with_fill(size * 2 + 4, size * 2 + 4, '#');
        generate_room(&mut grid, size, 0.2, 2, start_x, start_y, true);
        let s = grid.to_string();
        println!("{}", &s);
        write!(file, "{}", &s).unwrap();
    }
}

fn generate_room(
    grid: &mut Grid,
    size: usize,
    offset_chance: f32,
    max_offset: usize,
    start_x: usize,
    start_y: usize,
    clear_floor: bool,
) {
    let mut x = start_x as i32;
    let mut y = start_y as i32;

    let gen = RoomGenerator::new(size, size, offset_chance, max_offset);

    //for (dx, dy) in make_room_points(size, size, offset_chance, max_offset) {
    for (dx, dy) in gen.make_room_points() {
        x += dx;
        y += dy;
        grid.set(x as usize, y as usize, '.');
    }
    //TODO: find the missing point and fill it to avoid need for calling this fn
    grid.connect(x as usize, y as usize, start_x, start_y, '.');
    if clear_floor {
        grid.fill(start_x + size / 2, start_y + size / 2, '.');
    }
}
type Percent = f32;

struct RoomGenerator {
    offset_chance: f32,
    max_offset: usize,
    width: usize,
    height: usize,
}

impl RoomGenerator {
    fn new(width: usize, height: usize, offset_chance: f32, max_offset: usize) -> Self {
        RoomGenerator {
            width: width,
            height: height,
            offset_chance: offset_chance,
            max_offset: max_offset,
        }
    }

    fn get_corners(&self) -> Vec<(i32, i32)> {
        vec![
            (self.width as i32, 0),
            (self.width as i32, self.height as i32),
            (0, self.height as i32),
            (0, 0),
        ]
    }

    fn random_offset(&self) -> i32 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() <= self.offset_chance {
            if rng.gen() {
                return 1;
            } else {
                return -1;
            }
        }
        0
    }

    fn next_point(&self, axis_diff: i32, current_offset: i32) -> (i32, i32) {
        let mut diff = 0;
        let mut offset = 0;

        if axis_diff != 0 {
            diff = axis_diff / axis_diff.abs();
            //make offset
            let offset_candidate = self.random_offset();
            let new_total_offset = (current_offset + offset_candidate).abs() as usize;
            if new_total_offset <= self.max_offset {
                offset = offset_candidate;
            }
        }

        (diff, offset)
    }

    fn make_room_points(&self) -> Vec<(i32, i32)> {
        let mut steps: Vec<(i32, i32)> = vec![(0, 0)];

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let corners = self.get_corners();

        for original_corner in corners {

            let mut corner_x = original_corner.0;
            let mut corner_y = original_corner.1;

            while x != corner_x || y != corner_y {

                let mut dx = corner_x - x;
                let mut dy = corner_y - y;
                let y_offset = corner_y - original_corner.1;
                let x_offset = corner_x - original_corner.0;


                if dx != 0 {
                    let (new_dx, new_dy) = self.next_point(dx, y_offset);
                    dx = new_dx;
                    dy = new_dy;
                    corner_y += dy;
                } else {
                    let (new_dy, new_dx) = self.next_point(dy, x_offset);
                    dx = new_dx;
                    dy = new_dy;
                    corner_x += dx;
                }

                //avoid diagonal moves
                if dx != 0 && dy != 0 {
                    if rand::thread_rng().gen() {
                        steps.push((dx, 0));
                        steps.push((0, dy));
                    } else {
                        steps.push((0, dy));
                        steps.push((dx, 0));
                    }
                } else {
                    steps.push((dx, dy));
                }

                x += dx;
                y += dy;
            }
        }

        steps
    }
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
        let failed: Vec<Grid> = (0..1000)
            .map(|_| grid.clone())
            .map(|mut g| {
                generate_room(&mut g, size, 0.2, 2, x, y, false);
                g.set(x + size / 2, y + size / 2, '@');
                g
            })
            .map(|g| (g.clone(), g.clone()))
            .map(|mut g| {
                g.1.fill(x + size / 2, y + size / 2, '.');
                g
            })
            .map(|g| (g.0, g.1.count('#') < g.1.count('.')))
            .filter(|g| g.1)
            .map(|g| g.0)
            .map(|mut g| {
                g.set(x, y, '*');
                g
            })
            .collect();

        for g in &failed {
            println!("{}\n", g);
        }
        assert!(failed.is_empty());
    }
}
