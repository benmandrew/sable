use crate::grid::Grid;
use bmp_rust::bmp::BMP;

fn write_to_bmp(grid: &Grid, filename: &str) {
    let (w, h) = grid.get_dims();
    let mut bmp = BMP::new(h as i32, w as u32, None);
    let dib_header = bmp.get_dib_header().unwrap();
    let header = bmp.get_header();
    let buf = grid.get_front();
    for y in 0..h {
        for x in 0..w {
            let v = buf[y * w + x];
            let colour = if v == 0 {
                0
            } else {
                grid.convert_colour(v as f64)
            };
            bmp.change_color_of_pixel_efficient(
                x as u16,
                y as u16,
                colour.to_ne_bytes(),
                &dib_header,
                &header,
            )
            .unwrap()
        }
    }
    bmp.save_to_new(filename).unwrap()
}

pub fn main_terminal(grid: &mut Grid, n_iterations: usize) {
    for i in 0..n_iterations {
        grid.spawn(i as u32);
        grid.next()
    }
    println!("{}", grid)
}

pub fn main_bmp(grid: &mut Grid, n_iterations: usize, filename: &str) {
    for i in 0..n_iterations {
        grid.spawn(i as u32);
        grid.next()
    }
    write_to_bmp(grid, filename)
}
