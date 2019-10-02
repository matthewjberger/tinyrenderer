use image::{ImageBuffer, Pixel, Rgb};
use std::{mem, path::Path};
use tobj;

type RendererImage = ImageBuffer<Rgb<u8>, Vec<u8>>;
type RendererColor = Rgb<u8>;

fn main() {
    let model = tobj::load_obj(&Path::new("african_head.obj"));
    let (width, height) = (100, 100);
    let white = Rgb::from_channels(255, 255, 255, 255);
    let red = Rgb::from_channels(255, 0, 0, 255);
    let mut img = ImageBuffer::new(width, height);
    line(13, 20, 80, 40, &mut img, white);
    line(20, 13, 40, 80, &mut img, red);
    line(80, 40, 13, 20, &mut img, red);
    img = image::imageops::flip_vertical(&img);
    img.save("output.png").unwrap();
}

fn line(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut RendererImage, color: RendererColor) {
    let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);
    let steep = if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        true
    } else {
        false
    };

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = (2.0 * dy as f32).abs();
    let mut error = 0.0;
    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx as f32 {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx as f32 * 2.0;
        }
    }
}
