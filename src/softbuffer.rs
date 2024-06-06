use softbuffer::Surface;
use std::num::NonZeroU32;
use std::rc::Rc;
// use std::thread::sleep;
// use std::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::colour;
use crate::grid::Grid;

fn get_buf_pixel(
    x: u32,
    y: u32,
    screen_width: u32,
    screen_height: u32,
    buf: &[u8],
    (buf_width, buf_height): (usize, usize),
) -> u32 {
    let xf = x as f64 / screen_width as f64;
    let yf = y as f64 / screen_height as f64;
    let xb = (xf * buf_width as f64) as usize;
    let yb = (yf * buf_height as f64) as usize;
    let v = buf[yb * buf_width + xb];
    if v == 0 {
        0
    } else {
        colour::hsv_to_rgb(v as f64)
    }
}

fn handle_redraw_request(
    window: &Rc<Window>,
    surface: &mut Surface<Rc<Window>, Rc<Window>>,
    grid: &mut Grid,
    frame: u32,
) {
    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };
    surface
        .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();
    let mut buffer = surface.buffer_mut().unwrap();
    for i in 0..(width * height) {
        let y = i / width;
        let x = i % width;
        buffer[i as usize] = get_buf_pixel(
            x,
            y,
            width,
            height,
            grid.get_front(),
            grid.get_dims(),
        );
    }
    buffer.present().unwrap();
    grid.spawn(frame);
    grid.next();
    window.request_redraw();
}

pub fn main(grid: &mut Grid) {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface =
        softbuffer::Surface::new(&context, window.clone()).unwrap();
    let mut frame = 0;
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    handle_redraw_request(&window, &mut surface, grid, frame);
                    frame += 1;
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => elwt.exit(),
                _ => {}
            }
        })
        .unwrap();
}
