use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::colour;
use crate::grid::Grid;

fn get_dims(window: &Rc<Window>) -> (u32, u32) {
    let size = window.inner_size();
    (size.width, size.height)
}

fn render(i: usize, source: &[u8], target: &mut [u8]) {
    let v = source[i];
    let v = match v {
        0 => 0,
        v => colour::hsv_to_rgb(v as f64),
    };
    target.copy_from_slice(&v.to_ne_bytes());
}

fn handle_redraw_request(
    window: &Rc<Window>,
    pixels: &mut Pixels,
    grid: &mut Grid,
    frame_i: u32,
) {
    let (width, height) = get_dims(window);
    pixels.resize_surface(width, height).unwrap();
    let target = pixels.frame_mut();
    let source = grid.get_front();
    for (i, pixel) in target.chunks_exact_mut(4).enumerate() {
        render(i, source, pixel)
    }
    pixels.render().unwrap();
    grid.next(frame_i);
    window.request_redraw();
}

pub fn main(grid: &mut Grid) {
    let (width, height) = grid.get_dims();
    grid.spawn(0);
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
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
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width as u32, height as u32, surface_texture).unwrap()
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
