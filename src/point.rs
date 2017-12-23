use std::ops::Sub;

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy) as f32).sqrt()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_distance() {
        assert_eq!(Point::new(1, 1).distance(&Point::new(1, 2)), 1.0);
        assert_eq!(Point::new(0, 1).distance(&Point::new(-1, 1)), 1.0);
        assert_eq!(Point::new(1, 1).distance(&Point::new(2, 2)), 1.4142135);
        assert_eq!(Point::new(0, 0).distance(&Point::new(2, 3)), 3.6055512);
    }
}
