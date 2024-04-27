use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::colour;
use crate::grid::{self, Grid};

fn handle_redraw_request(window: &Rc<Window>, pixels: &mut Pixels, grid: &mut Grid, frame_i: u32) {
    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };
    pixels.resize_surface(width, height).unwrap();
    let frame = pixels.frame_mut();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let v = grid.get_buf()[i];
        let v = if v == 0 {
            0
        } else {
            colour::hsv_to_rgb(v as f64)
        };
        pixel.copy_from_slice(&v.to_ne_bytes());
    }
    pixels.render().unwrap();
    grid.next(frame_i);
    window.request_redraw();
}

pub fn main(grid: &mut Grid) {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(grid::WIDTH as f64, grid::HEIGHT as f64);
        Rc::new(
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap(),
        )
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(grid::WIDTH as u32, grid::HEIGHT as u32, surface_texture).unwrap()
    };

    let mut frame = 0;
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);
            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    handle_redraw_request(&window, &mut pixels, grid, frame);
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
