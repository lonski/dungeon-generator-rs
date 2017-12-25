extern crate rand;

use self::rand::Rng;
use line_gen::DistortedLineGnerator;
use point::Point;
use itertools::Itertools;
use grid::Grid;

pub struct RoomGenerator {
    pub width: usize,
    pub height: usize,
    pub distortion_chance: f32,
    pub vertex_offset_min: i32,
    pub vertex_offset_max: i32,
    pub vertex_offset_chance: f32,
}

pub struct Room {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

impl Room {
    pub fn new(points: Vec<Point>) -> Self {
        let x_points = points.iter().map(|p| p.x).sorted();
        let y_points = points.iter().map(|p| p.y).sorted();
        Room {
            points: points,
            width: (x_points[x_points.len() - 1] - x_points[0]) as usize,
            height: (y_points[y_points.len() - 1] - y_points[0]) as usize,
        }
    }

    pub fn draw(&self, grid: &mut Grid, pos: &Point) {
        self.points.iter().for_each(|p| {
            grid.set((pos.x + p.x) as usize, (pos.y + p.y) as usize, '.');
        });
        grid.fill(
            pos.x as usize + self.width / 2,
            pos.y as usize + self.height / 2,
            '.',
        );
    }
}

impl RoomGenerator {
    pub fn generate(&self) -> Room {
        let line_gen = DistortedLineGnerator::new(self.distortion_chance);
        let vertices: Vec<Point> = vec![
            Point::new(0, 0),
            Point::new(self.width as i32, 0),
            Point::new(self.width as i32, self.height as i32),
            Point::new(0, self.height as i32),
        ];
        let vertices = self.apply_vertex_offset(vertices);
        let mut points = vec![vertices[0].clone()];
        points.extend(line_gen.generate(&vertices[0], &vertices[1]));
        points.extend(line_gen.generate(&vertices[1], &vertices[2]));
        points.extend(line_gen.generate(&vertices[2], &vertices[3]));
        points.extend(line_gen.generate(&vertices[3], &vertices[0]));
        Room::new(points)
    }

    fn apply_vertex_offset(&self, vertices: Vec<Point>) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        vertices
            .into_iter()
            .map(|mut v| {
                if rng.gen::<f32>() < self.vertex_offset_chance {
                    let offset: i32 = rng.gen_range(self.vertex_offset_min, self.vertex_offset_max);
                    if rng.gen() {
                        v.x += offset;
                    } else {
                        v.y += offset;
                    }
                }
                v
            })
            .collect()
    }
}
