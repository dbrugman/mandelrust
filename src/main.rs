extern crate piston_window;
extern crate image as im;
extern crate num;

use piston_window::*;
use num::complex::Complex;

const MAX_ITER: u32 = 100;
const NORM_SQR_THRESHOLD: f64 = 16.0;
const CANVAS_WIDTH: u32 = 1600;
const CANVAS_HEIGHT: u32 = 900;
const SCENE_CENTER_X: f64 = -0.5;
const SCENE_CENTER_Y: f64 = 0.0;
const SCENE_TO_CANVAS_SCALE: f64 = 400.0;

fn get_color(c: Complex<f64>) -> im::Rgba<u8> {
  let mut z = c;
  let mut i = 0;
  while i < MAX_ITER && z.norm_sqr() < NORM_SQR_THRESHOLD {
      z = z * z + c;
      i = i + 1;
  }
  let lum = if i < MAX_ITER { (i as f64) / (MAX_ITER as f64) * 255.0 } else { 0.0 };
  let r = if lum > 128.0 { (lum - 128.0) * 2.0 } else { 0.0 } as u8;
  let g = r;
  let b = lum as u8;
  im::Rgba([r, g, b, 255])
}

fn canvas_to_scene(xc: u32, yc: u32) -> (f64, f64) {
    let xs = ((xc as f64) - (CANVAS_WIDTH / 2) as f64) / SCENE_TO_CANVAS_SCALE + SCENE_CENTER_X;
    let ys = ((yc as f64) - (CANVAS_HEIGHT /2) as f64) / SCENE_TO_CANVAS_SCALE + SCENE_CENTER_Y;
    (xs, ys)
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Mandelrust", (CANVAS_WIDTH, CANVAS_HEIGHT))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut texture = Texture::from_image(
            &mut window.factory,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    for x in 0..CANVAS_WIDTH {
        for y in 0..CANVAS_HEIGHT {
            let (re, im) = canvas_to_scene(x, y);
            let rgb = get_color(Complex::new(re, im));
            canvas.put_pixel(x, y, rgb);
        }
    };

    while let Some(e) = window.next() {
        if let Event::Render(_) = e {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}
