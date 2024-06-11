use crate::colour::hsv_to_rgb;
use crate::grid::Grid;
use bmp_rust::bmp::BMP;
use std::time::Instant;

fn write_to_bmp(grid: &Grid) {
    let (w, h) = grid.get_dims();
    let mut bmp = BMP::new(w as i32, h as u32, None);
    let dib_header = bmp.get_dib_header().unwrap();
    let header = bmp.get_header();
    let buf = grid.get_front();
    for y in 0..h {
        for x in 0..w {
            let v = buf[y * w + x];
            let colour = if v == 0 { 0 } else { hsv_to_rgb(v as f64) };
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
    bmp.save_to_new("out.bmp").unwrap()
}

pub fn main(grid: &mut Grid) {
    let now = Instant::now();
    for i in 0..10000 {
        grid.spawn(i);
        grid.next()
    }
    let elapsed_time = now.elapsed();
    println!("{} ms", elapsed_time.as_millis());
    // print!("{}", grid.to_string());
    write_to_bmp(grid)
}
