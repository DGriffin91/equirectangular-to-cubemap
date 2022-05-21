use image::{ImageBuffer, Pixel, Rgba};
use std::f32::consts::{PI, TAU};

use glam::Vec3;

fn orientation(x: f32, y: f32, side: u8) -> Vec3 {
    match side {
        0 => Vec3::new(x, -1., -y),
        1 => Vec3::new(-x, 1., -y),
        2 => Vec3::new(-y, -x, 1.),
        3 => Vec3::new(y, -x, -1.),
        4 => Vec3::new(-1., -x, -y),
        _ => Vec3::new(1., x, -y),
    }
}

pub fn nearest_px(img: &ImageBuffer<Rgba<f32>, Vec<f32>>, x: f32, y: f32) -> Rgba<f32> {
    *img.get_pixel(
        (x as u32).rem_euclid(img.width()),
        (y as u32).rem_euclid(img.height()),
    )
}

pub fn linear_px(img: &ImageBuffer<Rgba<f32>, Vec<f32>>, x: f32, y: f32) -> Rgba<f32> {
    let p00 = nearest_px(img, x.floor(), y.floor());
    let p10 = nearest_px(img, x.ceil(), y.floor());
    let p01 = nearest_px(img, x.floor(), y.ceil());
    let p11 = nearest_px(img, x.ceil(), y.ceil());
    let xd = x - x.floor();
    let yd = y - y.floor();

    let p0 = p00.map2(&p10, |a, b| a * (1.0 - xd) + b * xd);
    let p1 = p01.map2(&p11, |a, b| a * (1.0 - xd) + b * xd);

    p0.map2(&p1, |a, b| a * (1.0 - yd) + b * yd)
}

// TODO more samplers
pub enum Sampler {
    Nearest,
    Bilinear,
}

pub fn convert(
    img: &ImageBuffer<Rgba<f32>, Vec<f32>>,
    face_size: u32,
    sampler: Sampler,
) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
    let mut output_image = ImageBuffer::new(face_size, face_size * 6);

    for side in 0..6 {
        for x in 0..face_size {
            for y in 0..face_size {
                let fx = x as f32;
                let fy = y as f32;
                let fface_size = face_size as f32;
                let pos = orientation(
                    (fx / fface_size) * 2.0 - 1.0,
                    (fy / fface_size) * 2.0 - 1.0,
                    side,
                );
                let lon = pos.y.atan2(pos.x).rem_euclid(TAU);
                let lat = (pos.z / pos.length()).acos();
                let sample_x = img.width() as f32 * lon / PI / 2.0;
                let sample_y = img.height() as f32 * lat / PI;
                // TODO Currently just nearest neighbor sampling
                let pix = match sampler {
                    Sampler::Nearest => nearest_px(img, sample_x, sample_y),
                    Sampler::Bilinear => linear_px(img, sample_x, sample_y),
                };
                output_image.put_pixel(x, y + side as u32 * face_size, pix);
            }
        }
    }
    output_image
}
