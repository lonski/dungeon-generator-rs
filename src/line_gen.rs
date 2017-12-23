extern crate rand;

use std::cmp;
use itertools::Itertools;
use point::Point;
use self::rand::Rng;

pub struct DistortedLineGnerator {
    distortion_chance: f32,
}

impl DistortedLineGnerator {
    pub fn new(distortion_chance: f32) -> Self {
        DistortedLineGnerator { distortion_chance }
    }

    pub fn generate(&self, start: &Point, end: &Point) -> Vec<Point> {
        let points = Self::stright_line(start, end, true);
        let points = self.distort_points(points);
        let points = Self::connect_points(points);
        Self::dediagonalize(points)
    }

    fn dediagonalize(points: Vec<Point>) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        points
            .into_iter()
            .tuple_windows::<(Point, Point)>()
            .map(|t| {
                let dx = t.1.x - t.0.x;
                let dy = t.1.y - t.0.y;
                let mut pts = Vec::new();
                if dx != 0 && dy != 0 {
                    pts.push(if rng.gen() {
                        Point::new(t.1.x, t.1.y - dy)
                    } else {
                        Point::new(t.1.x - dx, t.1.y)
                    });
                }
                pts.push(t.1);
                pts
            })
            .fold(Vec::new(), |mut acc, p| {
                acc.extend(p);
                acc
            })
    }

    fn distort_points(&self, points: Vec<Point>) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        let start = points[0].clone();
        let end = points[points.len() - 1].clone();
        points
            .into_iter()
            .map(|p| if p != end && p != start &&
                rng.gen::<f32>() <= self.distortion_chance
            {
                let offset = if rng.gen() { 1 } else { -1 };
                if rng.gen() {
                    Point::new(p.x, p.y + offset)
                } else {
                    Point::new(p.x + offset, p.y)
                }
            } else {
                p
            })
            .collect()
    }

    fn connect_points(points: Vec<Point>) -> Vec<Point> {
        let mut points = points
            .into_iter()
            .tuple_windows::<(Point, Point)>()
            .map(|p| (p.0.distance(&p.1), p))
            .map(|(distance, p)| if distance > 1.5 {
                Self::stright_line(&p.0, &p.1, true)
            } else {
                vec![p.0, p.1]
            })
            .fold(Vec::new(), |mut acc, points| {
                acc.extend(points);
                acc
            });
        points.dedup();
        points
    }

    fn stright_line(start: &Point, end: &Point, allow_diagonal: bool) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let n = cmp::max((start.x - end.x).abs(), (start.y - end.y).abs());
        for i in 0..n + 1 {
            let t = i as f32 / n as f32;
            let x = (start.x as f32 + (end.x as f32 - start.x as f32) * t).round() as i32;
            let y = (start.y as f32 + (end.y as f32 - start.y as f32) * t).round() as i32;
            if allow_diagonal || x.abs() != y.abs() {
                points.push(Point::new(x, y));
            } else {
                points.push(Point::new(x, 0));
                points.push(Point::new(0, y));
            }
        }
        points
    }
}
