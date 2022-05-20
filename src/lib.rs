use image::{GenericImage, GenericImageView};
use std::f32::consts::{PI, TAU};

use glam::Vec3;
use image::DynamicImage;

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

// TODO Is there something like this in the image crate?
fn new_dyn_image(like_img: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    match like_img {
        DynamicImage::ImageLuma8(_) => DynamicImage::new_luma8(w, h),
        DynamicImage::ImageLumaA8(_) => DynamicImage::new_luma_a8(w, h),
        DynamicImage::ImageRgb8(_) => DynamicImage::new_rgb8(w, h),
        DynamicImage::ImageRgba8(_) => DynamicImage::new_rgba8(w, h),
        DynamicImage::ImageLuma16(_) => DynamicImage::new_luma16(w, h),
        DynamicImage::ImageLumaA16(_) => DynamicImage::new_luma_a16(w, h),
        DynamicImage::ImageRgb16(_) => DynamicImage::new_rgb16(w, h),
        DynamicImage::ImageRgba16(_) => DynamicImage::new_rgba16(w, h),
        DynamicImage::ImageRgb32F(_) => DynamicImage::new_rgb32f(w, h),
        DynamicImage::ImageRgba32F(_) => DynamicImage::new_rgba32f(w, h),
        _ => unreachable!(),
    }
}

pub fn nearest_px(img: &DynamicImage, x: f32, y: f32) -> <DynamicImage as GenericImageView>::Pixel {
    img.get_pixel(
        (x as u32).rem_euclid(img.width()),
        (y as u32).rem_euclid(img.height()),
    )
}

pub fn convert(in_img: &DynamicImage, face_size: u32) -> DynamicImage {
    let mut output_image = new_dyn_image(in_img, face_size, face_size * 6);

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
                let sample_x = in_img.width() as f32 * lon / PI / 2.0;
                let sample_y = in_img.height() as f32 * lat / PI;
                // TODO Currently just nearest neighbor sampling
                let pix = nearest_px(in_img, sample_x, sample_y);
                output_image.put_pixel(x, y + side as u32 * face_size, pix);
            }
        }
    }
    output_image
}

#[cfg(test)]
mod tests {

    use super::*;
    use image::io::Reader as ImageReader;

    #[test]
    fn test_conv() {
        let img = ImageReader::open("autumn_park_1k.jpg")
            .unwrap()
            .decode()
            .unwrap();
        convert(&img, img.width() / 4)
            .save("autumn_park_1k_out.jpg")
            .unwrap();
    }
}
