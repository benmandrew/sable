mod draw;
mod grid;

fn main() {
    let mut s = grid::Grid::new();
    draw::main(&mut s);
}
