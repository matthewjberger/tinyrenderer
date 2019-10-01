use image::{ImageBuffer, Pixel, Rgb};

fn main() {
    let (width, height) = (100, 100);
    let red = Rgb::from_channels(255, 0, 0, 255);
    let mut img = ImageBuffer::new(width, height);
    img.put_pixel(52, 41, red);
    image::imageops::flip_vertical(&img);
    img.save("output.png").unwrap();
}
