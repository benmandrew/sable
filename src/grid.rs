use rand::{rngs::ThreadRng, Rng};

pub const WIDTH: usize = 50;
pub const HEIGHT: usize = 50;
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

fn spawn(frame: u32, rng: &mut ThreadRng, source: &mut [u8; SIZE]) {
    if frame % 2 == 0 {
        for _ in 0..4 {
            let i = rng.gen_range(0..WIDTH);
            if source[i + WIDTH] == 0 {
                source[i + WIDTH] = ((frame / 2) % 254 + 1) as u8;
            }
        }
    }
}

fn next(rng: &mut ThreadRng, source: &[u8; SIZE], target: &mut [u8; SIZE]) {
    clear(target);
    for i in (0..SIZE).rev() {
        let (x, y) = to_coords(i);
        let mut moved = false;
        if y < HEIGHT - 1 && source[i] != 0 {
            let below = to_index(x, y + 1);
            if source[below] == 0 {
                target[below] = source[i];
                moved = true;
            }
            let lateral_modifier = rng.gen_range(0..2) as isize * 2 - 1;
            // let lateral_modifier = 1;
            if x as isize + lateral_modifier >= 0
                && x as isize + lateral_modifier < WIDTH as isize
                && !moved
            {
                let below_lateral = to_index(((x as isize) + lateral_modifier) as usize, y + 1);
                if source[below_lateral] == 0 {
                    target[below_lateral] = source[i];
                    moved = true;
                }
            }
            if x as isize - lateral_modifier >= 0
                && x as isize - lateral_modifier < WIDTH as isize
                && !moved
            {
                let below_lateral = to_index(((x as isize) - lateral_modifier) as usize, y + 1);
                if source[below_lateral] == 0 {
                    target[below_lateral] = source[i];
                    moved = true;
                }
            }
        }
        if !moved {
            target[i] = source[i];
        } else {
            target[i] = 0;
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
                spawn(frame, &mut self.rng, &mut self.front);
                next(&mut self.rng, &self.front, &mut self.back);
                self.which = Buffer::Back
            }
            Buffer::Back => {
                spawn(frame, &mut self.rng, &mut self.back);
                next(&mut self.rng, &self.back, &mut self.front);
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
