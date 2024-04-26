use softbuffer::Surface;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::grid::{self, Grid};

fn get_buf_pixel(x: u32, y: u32, width: u32, height: u32, grid: &Grid) -> u32 {
    let xf = x as f64 / width as f64;
    let yf = y as f64 / height as f64;
    let xb = (xf * grid::WIDTH as f64) as usize;
    let yb = (yf * grid::HEIGHT as f64) as usize;
    let v = grid.get_buf()[yb * grid::WIDTH + xb];
    v as u32 * (u32::MAX / 256)
}

fn handle_redraw_request(
    window: &Rc<Window>,
    surface: &mut Surface<Rc<Window>, Rc<Window>>,
    grid: &mut Grid,
    i: u32,
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
    for index in 0..(width * height) {
        let y = index / width;
        let x = index % width;
        buffer[index as usize] = get_buf_pixel(x, y, width, height, grid);
    }
    buffer.present().unwrap();
    // Every 0.1 seconds
    // sleep(Duration::new(0, 100000000));
    grid.next(i);
    window.request_redraw();
}

pub fn main(grid: &mut Grid) {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    let mut i = 0;
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    handle_redraw_request(&window, &mut surface, grid, i);
                    i = i + 1;
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
