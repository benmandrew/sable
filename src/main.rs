mod colour;
mod grid;
mod pixels;
mod softbuffer;

fn main() {
    let mut s = grid::Grid::new();
    pixels::main(&mut s);
}
