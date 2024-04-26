use rand::{rngs::ThreadRng, Rng};

pub const WIDTH: usize = 75;
pub const HEIGHT: usize = 75;
pub const SIZE: usize = WIDTH * HEIGHT;

// Computes the x and y coordinates from the flat index i
fn to_coords(i: usize) -> (usize, usize) {
    (i % WIDTH, i / WIDTH)
}

// Computes the flat index i from the x and y coordinates
fn to_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn clear(buf: &mut [u8; SIZE]) {
    for i in 0..SIZE {
        buf[i] = 0;
    }
}

fn next(source: &mut [u8; SIZE], target: &mut [u8; SIZE], rng: &mut ThreadRng, frame: u32) {
    clear(target);

    for _ in 0..2 {
        let i = rng.gen_range(0..WIDTH);
        if source[i] == 0 {
            source[i] = ((frame / 7) % 254 + 1) as u8;
        }
    }

    for i in (0..SIZE).rev() {
        let (x, y) = to_coords(i);
        let below_i = to_index(x, y + 1);
        if y < HEIGHT - 1 && source[below_i] == 0 {
            target[below_i] = source[i];
            target[i] = 0;
        } else {
            target[i] = source[i];
        }
    }
}

enum Buffer {
    Front,
    Back,
}

pub struct Grid {
    which: Buffer,
    front: [u8; SIZE],
    back: [u8; SIZE],
    rng: ThreadRng,
}

impl Grid {
    pub fn new() -> Grid {
        let front = [0_u8; SIZE];
        let back = [0_u8; SIZE];
        let rng = rand::thread_rng();
        let mut v = Grid {
            which: Buffer::Front,
            front,
            back,
            rng,
        };
        v.which = Buffer::Front;
        for i in 0..SIZE {
            v.front[i] = 0;
            v.back[i] = 0;
        }
        v
    }

    pub fn next(&mut self, frame: u32) {
        match self.which {
            Buffer::Front => {
                next(&mut self.front, &mut self.back, &mut self.rng, frame);
                self.which = Buffer::Back
            }
            Buffer::Back => {
                next(&mut self.back, &mut self.front, &mut self.rng, frame);
                self.which = Buffer::Front
            }
        }
    }

    pub fn get_buf(&self) -> &[u8; SIZE] {
        match self.which {
            Buffer::Front => &self.front,
            Buffer::Back => &self.back,
        }
    }
}
