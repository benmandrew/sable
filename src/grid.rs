use crossbeam::scope;
use rand::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use std::sync::Arc;
use std::vec;

static N_THREADS: usize = 4;

pub struct Config {
    width: usize,
    height: usize,
    size: usize,
    ribbon_len: usize,
}

impl Config {
    fn new(width: usize, height: usize, n_threads: Option<usize>) -> Config {
        let n_threads = n_threads.unwrap_or(N_THREADS);
        let rem = height % (n_threads * 2);
        let mut height = height;
        if rem != 0 {
            height -= rem;
            println!("Warning: grid height must be divisible by N_THREADS * 2. Reducing {} to {}", height + rem, height);
        }
        let size = width * height;
        let ribbon_len = size / n_threads;
        Config {
            width,
            height,
            size,
            ribbon_len,
        }
    }

    fn get_dims(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    // Computes the y coordinate from the flat index i
    fn index_to_y(&self, i: usize) -> usize {
        i / self.width
    }

    // Computes the x and y coordinates from the flat index i
    fn index_to_coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }
}

fn move_lateral(
    cfg: &Config,
    x: usize,
    offset: isize,
    source: &[u8],
    target: &mut [u8],
    ribbon_i: usize,
    real_i: usize,
) -> bool {
    let mut moved = false;
    if x as isize + offset >= 0 && x as isize + offset < cfg.width as isize {
        let below_lateral_real =
            ((real_i + cfg.width) as isize + offset) as usize;
        if source[below_lateral_real] == 0 {
            let below_lateral =
                ((ribbon_i + cfg.width) as isize + offset) as usize;
            target[below_lateral] = source[real_i];
            moved = true;
        }
    }
    moved
}

fn next_pixel(
    cfg: &Config,
    source: &[u8],
    target: &mut [u8],
    ribbon_i: usize,
    real_i: usize,
) {
    let mut moved = false;
    let (x, y) = cfg.index_to_coords(ribbon_i);
    let real_y = cfg.index_to_y(real_i);
    let within_full = real_y < cfg.height - 1;
    let within_half_ribbon = y < cfg.ribbon_len / 2;
    if within_full && within_half_ribbon && source[real_i] != 0 {
        if source[real_i + cfg.width] == 0 {
            target[ribbon_i + cfg.width] = source[real_i];
            moved = true;
        }
        if !moved {
            moved = move_lateral(cfg, x, 1, source, target, ribbon_i, real_i)
        }
        if !moved {
            moved = move_lateral(cfg, x, -1, source, target, ribbon_i, real_i)
        }
    }
    if moved {
        target[ribbon_i] = 0;
    } else if !moved && source[real_i] != 0 {
        target[ribbon_i] = source[real_i];
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

    fn get_back_mut(&mut self) -> &mut Vec<u8> {
        if self.count % 2 == 0 {
            &mut self.buf_b
        } else {
            &mut self.buf_a
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

    fn empty_back(&mut self) {
        self.get_back_mut().fill(0)
    }
}

pub struct Grid {
    cfg: Arc<Config>,
    buf: DoubleBuffer,
    rng: ChaCha8Rng,
    convert_colour: fn(f64) -> u32,
}

fn count_leading_whitespace(s: &str) -> usize {
    s.chars().take_while(|c| c.is_whitespace()).count()
}

fn skip_n_chars(s: &str, n: usize) -> String {
    s.chars().skip(n).collect::<String>()
}

impl std::fmt::Display for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, _) = self.get_dims();
        let rows = self
            .get_front()
            .chunks(w)
            .map(|row| {
                row.iter()
                    .map(|c| format!("{: >4}", c))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>();
        let rows_leading_whitespace = rows
            .iter()
            .map(|s| count_leading_whitespace(s))
            .min()
            .unwrap_or(0);
        let stripped_rows = rows
            .iter()
            .map(|s| skip_n_chars(s, rows_leading_whitespace))
            .collect::<Vec<_>>();
        write!(fmt, "{}", stripped_rows.join("\n"))
    }
}

fn generate_target_ribbons(
    target: &mut [u8],
    start: usize,
    ribbon_len: usize,
) -> Vec<&mut [u8]> {
    let (_, target_shifted) = target.split_at_mut(start);
    target_shifted.chunks_mut(ribbon_len).collect()
}

impl Grid {
    pub fn new(
        width: usize,
        height: usize,
        n_threads: Option<usize>,
        seed: u64,
        convert_colour: fn(f64) -> u32,
    ) -> Grid {
        let cfg = Arc::new(Config::new(width, height, n_threads));
        let buf = DoubleBuffer::new(&cfg);
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        Grid {
            cfg,
            buf,
            rng,
            convert_colour,
        }
    }

    pub fn convert_colour(&self, v: f64) -> u32 {
        (self.convert_colour)(v)
    }

    pub fn get_front(&self) -> &Vec<u8> {
        self.buf.get_front()
    }

    pub fn get_dims(&self) -> (usize, usize) {
        self.cfg.get_dims()
    }

    pub fn spawn(&mut self, frame: u32) {
        let source = self.buf.get_front_mut();
        for _ in 0..(self.cfg.width / 20 + 1) {
            let i = self.rng.next_u32() as usize % self.cfg.width;
            if source[i] == 0 {
                source[i] = ((frame / 5) % 254 + 1) as u8;
            }
        }
    }

    fn propagate_half(&mut self, offset: usize) {
        let (source, target) = self.buf.get_pair();
        let mut target_ribbons =
            generate_target_ribbons(target, offset, self.cfg.ribbon_len);
        let ribbon_len = self.cfg.ribbon_len;
        scope(|s| {
            for (i, target) in target_ribbons.iter_mut().enumerate().rev() {
                let cfg = Arc::clone(&self.cfg);
                s.spawn(move |_| {
                    for j in (0..(ribbon_len / 2)).rev() {
                        let real_index = i * ribbon_len + j + offset;
                        next_pixel(&cfg, source, target, j, real_index)
                    }
                });
            }
        })
        .unwrap();
    }

    fn propagate(&mut self) {
        self.buf.empty_back();
        self.propagate_half(self.cfg.ribbon_len / 2);
        self.propagate_half(0);
    }

    pub fn next(&mut self) {
        self.propagate();
        self.buf.switch_buffers();
    }

    #[cfg(test)]
    fn set_px(&mut self, x: usize, y: usize, v: u8) {
        let (w, _) = self.get_dims();
        let buf = self.buf.get_front_mut();
        buf[y * w + x] = v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    const DUMMY_CONVERT_COLOUR: fn(f64) -> u32 = |v: f64| v as u32;

    #[test]
    fn stays_on_ground() {
        let mut g = Grid::new(3, 2, Some(1), 0, DUMMY_CONVERT_COLOUR);
        g.set_px(1, 1, 2);
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    2    0
        "#);
        g.next();
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    2    0
        "#);
    }

    #[test]
    fn falls_straight() {
        let mut g = Grid::new(3, 2, Some(1), 0, DUMMY_CONVERT_COLOUR);
        g.set_px(1, 0, 2);
        assert_snapshot!(g.to_string(), @r#"
            0    2    0
            0    0    0
        "#);
        g.next();
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    2    0
        "#);
    }

    #[test]
    fn falls_laterally() {
        let mut g = Grid::new(3, 2, Some(1), 0, DUMMY_CONVERT_COLOUR);
        g.set_px(0, 1, 255);
        g.set_px(0, 0, 255);
        assert_snapshot!(g.to_string(), @r#"
            255    0    0
            255    0    0
        "#);
        g.next();
        assert_snapshot!(g.to_string(), @r#"
              0    0    0
            255  255    0
        "#);
    }

    #[test]
    fn falls_laterally_both() {
        let mut g = Grid::new(3, 6, Some(1), 0, DUMMY_CONVERT_COLOUR);
        g.set_px(1, 4, 3);
        g.set_px(1, 2, 3);
        g.set_px(1, 0, 3);
        assert_snapshot!(g.to_string(), @r#"
            0    3    0
            0    0    0
            0    3    0
            0    0    0
            0    3    0
            0    0    0
        "#);
        g.next();
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    3    0
            0    0    0
            0    3    0
            0    0    0
            0    3    0
        "#);
    }

    #[test]
    fn falls_laterally_both_multithreaded() {
        let mut g = Grid::new(3, 6, Some(3), 0, DUMMY_CONVERT_COLOUR);
        g.set_px(1, 4, 3);
        g.set_px(1, 2, 3);
        g.set_px(1, 0, 3);
        assert_snapshot!(g.to_string(), @r#"
            0    3    0
            0    0    0
            0    3    0
            0    0    0
            0    3    0
            0    0    0
        "#);
        g.next();
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    3    0
            0    0    0
            0    3    0
            0    0    0
            0    3    0
        "#);
        g.next();
        g.next();
        g.next();
        g.next();
        assert_snapshot!(g.to_string(), @r#"
            0    0    0
            0    0    0
            0    0    0
            0    0    0
            0    0    0
            3    3    3
        "#);
    }
}
