use crossbeam::scope;
use rand::{rngs::ThreadRng, Rng};
use std::{sync::Arc, thread, time};

pub struct Config {
    width: usize,
    height: usize,
    size: usize,
    n_threads: usize,
    ribbon_len: usize,
    // rng: ThreadRng,
}

impl Config {
    fn new(width: usize, height: usize) -> Config {
        let size = width * height;
        let n_threads = 1;
        let ribbon_len = size / n_threads;
        // let rng = rand::thread_rng();
        Config {
            width,
            height,
            size,
            n_threads,
            ribbon_len,
            // rng,
        }
    }

    fn get_dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    // Computes the x and y coordinates from the flat index i
    fn index_to_coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    // Computes the flat index i from the x and y coordinates
    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn next_pixel(
    cfg: &Config,
    source: &[u8],
    target: &mut [u8],
    ribbon_i: usize,
    real_i: usize,
) {
    let (x, y) = cfg.index_to_coords(ribbon_i);
    let (_, real_y) = cfg.index_to_coords(real_i);
    let mut moved = false;
    if real_y < cfg.height - 1 && source[ribbon_i] != 0 {
        let below = ribbon_i + cfg.width;
        if source[below] == 0 {
            target[below - cfg.width] = source[ribbon_i];
            moved = true;
        }
        // let lateral_modifier = cfg.rng.gen_range(0..2) as isize * 2 - 1;
        let lateral_modifier = 1;
        if x as isize + lateral_modifier >= 0
            && x as isize + lateral_modifier < cfg.width as isize
            && !moved
        {
            let below_lateral = cfg.coords_to_index(
                ((x as isize) + lateral_modifier) as usize,
                y + 1,
            );
            if source[below_lateral] == 0 {
                target[below_lateral - cfg.width] = source[ribbon_i];
                moved = true;
            }
        }
        if x as isize - lateral_modifier >= 0
            && x as isize - lateral_modifier < cfg.width as isize
            && !moved
        {
            let below_lateral = cfg.coords_to_index(
                ((x as isize) - lateral_modifier) as usize,
                y + 1,
            );
            if source[below_lateral] == 0 {
                target[below_lateral - cfg.width] = source[ribbon_i];
                moved = true;
            }
        }
    }
    if ribbon_i >= cfg.width {
        if moved {
            target[ribbon_i - cfg.width] = 0;
        } else if !moved && source[ribbon_i] != 0 {
            target[ribbon_i - cfg.width] = source[ribbon_i];
        }
    }
}

pub struct DoubleBuffer {
    buf_a: Vec<u8>,
    buf_b: Vec<u8>,
    count: usize,
}

impl DoubleBuffer {
    pub fn new(cfg: &Config) -> DoubleBuffer {
        // Add a row of padding at the bottom
        DoubleBuffer {
            buf_a: vec![0_u8; cfg.size + cfg.width],
            buf_b: vec![0_u8; cfg.size + cfg.width],
            count: 0,
        }
    }

    pub fn get_front(&self) -> &Vec<u8> {
        if self.count % 2 == 0 {
            &self.buf_a
        } else {
            &self.buf_b
        }
    }

    fn get_front_mut(&mut self) -> &mut Vec<u8> {
        if self.count % 2 == 0 {
            &mut self.buf_a
        } else {
            &mut self.buf_b
        }
    }

    fn get_pair(&mut self) -> (&Vec<u8>, &mut Vec<u8>) {
        if self.count % 2 == 0 {
            (&self.buf_a, &mut self.buf_b)
        } else {
            (&self.buf_b, &mut self.buf_a)
        }
    }

    fn switch_buffers(&mut self) {
        self.count += 1
    }
}

pub struct Grid {
    cfg: Arc<Config>,
    buf: DoubleBuffer,
}

fn generate_target_ribbons(
    target: &mut Vec<u8>,
    width: usize,
    ribbon_len: usize,
) -> Vec<&mut [u8]> {
    let (_, target_without_first_row) = target.split_at_mut(width);
    target_without_first_row.chunks_mut(ribbon_len).collect()
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cfg = Arc::new(Config::new(width, height));
        let buf = DoubleBuffer::new(&cfg);
        Grid { cfg, buf }
    }

    pub fn get_front(&self) -> &Vec<u8> {
        self.buf.get_front()
    }

    pub fn get_dims(&self) -> (usize, usize) {
        self.cfg.get_dims()
    }

    pub fn spawn(&mut self, frame: u32) {
        let source = self.buf.get_front_mut();
        if frame % 4 == 0 {
            for i in 0..(self.cfg.width / 4) {
                // let i = self.cfg.rng.gen_range(0..self.cfg.width);
                let j = i * 4;
                if source[j] == 0 {
                    source[j] = ((frame / 5) % 254 + 1) as u8;
                }
            }
        }
    }

    pub fn next(&mut self, frame: u32) {
        self.spawn(frame);

        let ribbon_len = self.cfg.ribbon_len;
        let (source, target) = self.buf.get_pair();
        target.fill(0);
        let source_ribbons: Vec<&[u8]> =
            source.chunks_exact(ribbon_len).collect();
        let mut target_ribbons =
            generate_target_ribbons(target, self.cfg.width, ribbon_len);

        scope(|s| {
            for (i, (source, target)) in source_ribbons
                .iter()
                .zip(target_ribbons.iter_mut())
                .enumerate()
            {
                let cfg = Arc::clone(&self.cfg);
                s.spawn(move |_| {
                    for j in 0..ribbon_len {
                        let real_index = i * ribbon_len + j;
                        next_pixel(&cfg, source, target, j, real_index)
                    }
                });
            }
        })
        .unwrap();

        println!("Frame {frame}");
        // thread::sleep(time::Duration::from_millis(200));

        self.buf.switch_buffers();
    }

    // pub fn next(&mut self, frame: u32) {
    //     self.spawn(frame);
    //     let (source, target) = self.buf.get_pair();
    //     target.fill(0);
    //     for i in 0..self.cfg.size {
    //         next_pixel(&mut self.cfg, source, target, i)
    //     }
    //     self.buf.switch_buffers();
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn internal() {
//         let g = Grid::new(4, 8);
//         let v = g.cfg.split(g.buf.get_front(), 0, 8, 4);
//         assert_eq!(v.len(), 4);
//     }
// }
