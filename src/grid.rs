use rand::{rngs::ThreadRng, Rng};
use std::{
    thread::{self, Thread},
    time::Duration,
};

pub struct Config {
    width: usize,
    height: usize,
    size: usize,
    n_threads: usize,
    ribbon_len: usize,
    rng: ThreadRng,
}

impl Config {
    fn new(width: usize, height: usize) -> Config {
        let size = width * height;
        let n_threads = 4;
        let ribbon_len = width * height / (n_threads * 2);
        let rng = rand::thread_rng();
        Config {
            width,
            height,
            size,
            n_threads,
            ribbon_len,
            rng,
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

    /// split array `arr` into `len(arr)/step` chunks, returning the first `slice_len` elements of each chunk
    fn split<'a>(
        &self,
        arr: &'a Vec<u8>,
        start: usize,
        step: usize,
        slice_len: usize,
    ) -> Vec<&'a [u8]> {
        assert!(slice_len <= step);
        assert_eq!(self.size % slice_len, start);
        let mut chunks = Vec::new();
        let mut arr: &[u8] = &arr[start..];
        // let end_i = (self.cfg.size / slice_len) - start;
        for _i in 0..self.n_threads {
            let (fst, snd) = arr.split_at(slice_len);
            chunks.push(fst);
            (_, arr) = snd.split_at(step - slice_len);
        }
        chunks
    }

    fn split_mut<'a>(
        &self,
        arr: &'a mut Vec<u8>,
        start: usize,
        step: usize,
        slice_len: usize,
    ) -> Vec<&'a mut [u8]> {
        assert!(slice_len <= step);
        assert_eq!(self.size % slice_len, start);
        let mut v = Vec::new();
        let mut a: &mut [u8] = &mut arr[start..];
        // let end_i = (self.cfg.size / slice_len) - start;
        for i in 0..self.n_threads {
            let (fst, snd) = a.split_at_mut(i * step + slice_len);
            v.push(fst);
            (_, a) = snd.split_at_mut(step - slice_len)
        }
        assert_eq!(v.len(), self.n_threads);
        v
    }
}

fn next_pixel(
    cfg: &mut Config,
    source: &Vec<u8>,
    target: &mut Vec<u8>,
    index: usize,
) {
    let (x, y) = cfg.index_to_coords(index);
    let mut moved = false;
    if y < cfg.height - 1 && source[index] != 0 {
        let below = index + cfg.width;
        if source[below] == 0 {
            target[below] = source[index];
            moved = true;
        }
        let lateral_modifier = cfg.rng.gen_range(0..2) as isize * 2 - 1;
        // let lateral_modifier = 1;
        if x as isize + lateral_modifier >= 0
            && x as isize + lateral_modifier < cfg.width as isize
            && !moved
        {
            let below_lateral = cfg.coords_to_index(
                ((x as isize) + lateral_modifier) as usize,
                y + 1,
            );
            if source[below_lateral] == 0 {
                target[below_lateral] = source[index];
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
                target[below_lateral] = source[index];
                moved = true;
            }
        }
    }
    if moved {
        target[index] = 0;
    } else if !moved && source[index] != 0 {
        // } else {
        target[index] = source[index];
    }
}

pub struct DoubleBuffer {
    buf_a: Vec<u8>,
    buf_b: Vec<u8>,
    count: usize,
}

impl DoubleBuffer {
    pub fn new(cfg: &Config) -> DoubleBuffer {
        DoubleBuffer {
            buf_a: vec![0_u8; cfg.size],
            buf_b: vec![0_u8; cfg.size],
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
    cfg: Config,
    buf: DoubleBuffer,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cfg = Config::new(width, height);
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
            for _ in 0..(self.cfg.width / 20 + 1) {
                let i = self.cfg.rng.gen_range(0..self.cfg.width);
                if source[i] == 0 {
                    source[i] = ((frame / 5) % 254 + 1) as u8;
                }
            }
        }
    }

    // pub fn next(&mut self, frame: u32) {
    //     self.spawn(frame);
    //     let (source, target) = self.buf.get_pair();

    //     let firsts = self.cfg.split(
    //         source,
    //         0,
    //         self.cfg.ribbon_len * 2,
    //         self.cfg.ribbon_len,
    //     );
    //     let seconds = self.cfg.split_mut(
    //         target,
    //         0,
    //         self.cfg.ribbon_len * 2,
    //         self.cfg.ribbon_len + 1,
    //     );

    //     // let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    //     for i in 0..self.cfg.n_threads {
    //         let start_i = 2 * i * self.cfg.ribbon_len;
    //         let source_ribbon = &source[start_i..start_i + self.cfg.ribbon_len];
    //         let target_ribbon =
    //             if start_i + self.cfg.ribbon_len + self.cfg.width
    //                 <= self.cfg.size
    //             {
    //                 &mut target[start_i
    //                     ..start_i + self.cfg.ribbon_len + self.cfg.width]
    //             } else {
    //                 &mut target[start_i..start_i + self.cfg.ribbon_len]
    //             };
    //         let mut f = || {
    //             next_pixel(
    //                 &self.cfg,
    //                 source_ribbon,
    //                 target_ribbon,
    //                 self.cfg.ribbon_len,
    //                 i,
    //             )
    //         };
    //         f ()
    //         // handles.push(f);
    //         // handles.push(thread::spawn(f));
    //     }

    //     // for mut h in handles {
    //     //     h.join().unwrap();
    //     // }

    //     self.buf.switch_buffers();

    //     // for i in (0..self.cfg.size).rev() {
    //     //     next_pixel(rng, source, target, i)
    //     // }
    // }

    pub fn next(&mut self, frame: u32) {
        self.spawn(frame);
        let (source, target) = self.buf.get_pair();
        target.fill(0);
        for i in 0..self.cfg.size {
            next_pixel(&mut self.cfg, source, target, i)
        }
        self.buf.switch_buffers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let g = Grid::new(4, 8);
        let v = g.cfg.split(g.buf.get_front(), 0, 8, 4);
        assert_eq!(v.len(), 4);
    }
}
