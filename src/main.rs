#![allow(clippy::let_and_return)]

use nalgebra;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use std::convert::TryFrom;

mod canvas;
mod utils;

use canvas::Canvas;
use utils::load_obj;

pub type Vector3 = nalgebra::Vector3<f64>;
pub type Vector4 = nalgebra::Vector4<f64>;
pub type Matrix4 = nalgebra::Matrix4<f64>;

pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl<T> From<(T, T, T)> for Triangle
where
    T: Into<Vector3>,
{
    fn from(vertices: (T, T, T)) -> Self {
        Self {
            a: vertices.0.into(),
            b: vertices.1.into(),
            c: vertices.2.into(),
        }
    }
}


pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

const SCALE: f64 = 1.0;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const RATIO: f64 = HEIGHT as f64 / WIDTH as f64;

fn rasterize_triangle(
    triangle: &Triangle,
    canvas: &mut Canvas<'_>,
) {
    let side_1 = triangle.c - triangle.a;
    let side_2 = triangle.b - triangle.a;
    let cross = side_1.cross(&side_2).normalize();
    let camera_v: Vector3 = [0.0, 0.0, -1.0].into();
    let dot = cross.dot(&camera_v);
    let shade = (dot * 255.0).round();
    let shade = if shade < 0.0 {
        return;
    } else {
        shade as u8
    };

    let coeff = |from: &[i32; 2], to: &[i32; 2]| {
        let dx = (to[0] - from[0]) as f64;
        let dy = (to[1] - from[1]) as f64;

        dx / dy
    };

    let size = HEIGHT.min(WIDTH) as f64;
    let width_f = WIDTH as f64;
    let height_f = HEIGHT as f64;

    let a = [
        (0.5 * (triangle.a[0] * size + width_f)).round() as i32,
        (0.5 * (-triangle.a[1] * size + height_f)).round() as i32,
    ];
    let b = [
        (0.5 * (triangle.b[0] * size + width_f)).round() as i32,
        (0.5 * (-triangle.b[1] * size + height_f)).round() as i32,
    ];
    let c = [
        (0.5 * (triangle.c[0] * size + width_f)).round() as i32,
        (0.5 * (-triangle.c[1] * size + height_f)).round() as i32,
    ];
    let mut vertices = [a, b, c];
    vertices.sort_by_key(|vert| vert[1]);
    let [top, middle, bottom] = vertices;

    let dy_top = middle[1] - top[1];
    let dy_bottom = bottom[1] - middle[1];

    let coeff_tb = coeff(&top, &bottom);
    let coeff_tm = coeff(&top, &middle);
    let coeff_mb = coeff(&middle, &bottom);

    for y in 0..dy_top {
        let x_a = (coeff_tb * (y as f64)).round() as i32 + top[0];
        let x_b = (coeff_tm * (y as f64)).round() as i32 + top[0];

        let x_left = x_a.min(x_b);
        let x_right = x_a.max(x_b);

        for x in x_left..=x_right {
            let y = y + top[1];
            canvas.put_pixel(x, y, triangle.a[2], [shade; 3]);
        }
    }

    for y in 0..=dy_bottom {
        let x_a = (coeff_tb * ((y + dy_top) as f64)).round() as i32 + top[0];
        let x_b = (coeff_mb * (y as f64)).round() as i32 + middle[0];

        let x_left = x_a.min(x_b);
        let x_right = x_a.max(x_b);

        for x in x_left..=x_right {
            let y = y + top[1] + dy_top;
            canvas.put_pixel(x, y, triangle.a[2], [shade; 3]);
        }
    }
}

fn main() -> Result<(), Error> {
    let (event_loop, mut input, window, mut pixels) = setup()?;
    let mesh = load_obj("assets/african_head.obj");

    let mut z_buf = vec![f64::NEG_INFINITY; (WIDTH * HEIGHT) as usize];

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // DRAW

            let mut canvas = Canvas::new(pixels.get_frame(), &mut z_buf);
            canvas.clear();

            for triangle in mesh.triangles.iter() {
                rasterize_triangle(triangle, &mut canvas);
            }

            // /DRAW

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}

fn setup() -> Result<(EventLoop<()>, WinitInputHelper, Window, Pixels), Error> {
    let event_loop = EventLoop::new();
    let input = WinitInputHelper::new();

    let window = {
        let size =
            LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);

        WindowBuilder::new()
            .with_title("maasbree")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    Ok((event_loop, input, window, pixels))
}
