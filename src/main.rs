use image::{ImageBuffer, Rgb};

mod ray;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut image = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = x as f32 / (image_width - 1) as f32;
        let g = y as f32 / (image_height - 1) as f32;
        let b = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    image.save("temp.png").unwrap();
}
