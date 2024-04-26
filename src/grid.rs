pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 20;
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

fn next(source: &mut [u8; SIZE], target: &mut [u8; SIZE]) {
    clear(target);
    for i in (0..SIZE).rev() {
        let (x, y) = to_coords(i);
        let below_i = to_index(x, y + 1);
        if y < HEIGHT - 10 && source[below_i] == 0 {
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
}

impl Grid {
    pub fn new() -> Grid {
        let front = [0_u8; SIZE];
        let back = [0_u8; SIZE];
        let mut v = Grid {
            which: Buffer::Front,
            front,
            back,
        };
        v.which = Buffer::Front;
        for i in 0..SIZE {
            v.front[i] = 0;
            v.back[i] = 0;
            if i < WIDTH {
                v.front[i] = 1;
            }
        }
        v
    }

    pub fn next(&mut self) {
        match self.which {
            Buffer::Front => {
                next(&mut self.front, &mut self.back);
                self.which = Buffer::Back
            }
            Buffer::Back => {
                next(&mut self.back, &mut self.front);
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
