const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SIZE: usize = WIDTH * HEIGHT;

// Computes the x and y coordinates from the flat index i
fn to_coords(i: usize) -> (usize, usize) {
    (i % WIDTH, i / WIDTH)
}

// Computes the flat index i from the x and y coordinates
fn to_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn next(source: &mut [u8; SIZE], target: &mut [u8; SIZE]) {
    for i in (0..SIZE).rev() {
        let (x, y) = to_coords(i);
        if y < HEIGHT - 1 {
            let below_i = to_index(x, y + 1);
            if source[below_i] == 0 {
                target[below_i] = source[i];
                target[i] = 0;
            }
        }
    }
}

enum Buffer {
    Front,
    Back,
}

struct Grid {
    which: Buffer,
    front: [u8; SIZE],
    back: [u8; SIZE],
}

impl Grid {
    fn new() -> Grid {
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
        }
        v
    }

    fn next(&mut self) {
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
}

fn main() {
    let mut s = Grid::new();
    loop {
        s.next()
    }
}
