use std::iter::FromIterator;
use std::fmt;
use std::cmp;

#[derive(Clone)]
pub struct Grid {
    tiles: Vec<char>,
    width: usize,
}

impl Grid {
    pub fn new_with_fill(width: usize, height: usize, fill: char) -> Self {
        Grid {
            tiles: (0..width * height).map(|_| fill).collect(),
            width: width,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        Grid::new_with_fill(width, height, '#')
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        match self.cord_to_pos(x, y) {
            Some(pos) => Some(self.tiles[pos]),
            None => None,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, new_tile: char) -> bool {
        match self.cord_to_pos(x, y) {
            Some(pos) => {
                self.tiles[pos] = new_tile;
                true
            }
            None => false,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    pub fn connect(
        &mut self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        fill: char,
    ) {
        let mut x = start_x;
        let mut y = start_y;
        while x != end_x || y != end_y {
            let dx = end_x as i32 - x as i32;
            if dx != 0 {
                x = (x as i32 + dx / dx.abs()) as usize;
            }
            let dy = end_y as i32 - y as i32;
            if dy != 0 {
                y = (y as i32 + dy / dy.abs()) as usize;
            }
            self.set(x, y, fill);
        }
    }

    pub fn fill(&mut self, start_x: usize, start_y: usize, fill: char) {
        let mut frontier = vec![(start_x, start_y)];
        while !frontier.is_empty() {
            let (cx, cy) = frontier.pop().unwrap();
            self.set(cx, cy, fill);
            for (nx, ny) in self.neighbours(cx, cy) {
                if let Some(c) = self.get(nx, ny) {
                    if c != fill {
                        frontier.push((nx, ny));
                    }
                }
            }
        }
    }

    pub fn count(&self, tile: char) -> usize {
        self.tiles.iter().filter(|&c| *c == tile).count()
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![
            (x, cmp::max(0, y as i32 - 1) as usize),
            (x + 1, y),
            (x, y + 1),
            (cmp::max(0, x as i32 - 1) as usize, y),

            (x + 1, y + 1),
            (x + 1, cmp::max(0, y as i32 - 1) as usize),
            (
                cmp::max(0, x as i32 - 1) as usize,
                cmp::max(0, y as i32 - 1) as usize
            ),
            (cmp::max(0, x as i32 - 1) as usize, y + 1),
        ]
    }

    fn cord_to_pos(&self, x: usize, y: usize) -> Option<usize> {
        let pos = y * self.width + x;
        if pos < self.tiles.len() {
            Some(pos)
        } else {
            None
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str: String = self.tiles
            .chunks(self.width)
            .map(|c| String::from_iter(c))
            .fold(String::new(), |acc, row| format!("{}{}\n", acc, row));
        write!(f, "{}", as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_grid() {
        assert_eq!(Grid::new(2, 3).to_string(), "##\n##\n##\n");
    }

    #[test]
    fn should_change_tile() {
        let mut grid = Grid::new(2, 3);
        grid.set(1, 2, 'x');
        assert_eq!(grid.to_string(), "##\n##\n#x\n");
        assert_eq!(grid.get(1, 2).unwrap(), 'x');
    }

    #[test]
    fn should_return_true_only_if_successfully_set_tile() {
        let mut grid = Grid::new(1, 1);
        assert!(!grid.set(1, 0, '@'));
        assert_eq!(grid.to_string(), "#\n");
        assert!(grid.set(0, 0, '@'));
        assert_eq!(grid.to_string(), "@\n");
    }

    #[test]
    fn should_return_correct_dimmensions() {
        let grid = Grid::new(5, 8);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 8);
    }
}
