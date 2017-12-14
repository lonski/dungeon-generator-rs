use grid::Grid;

mod grid;

fn main() {
    let mut grid = Grid::new(80, 40);
    println!("Grid: \n{}", grid);
}
