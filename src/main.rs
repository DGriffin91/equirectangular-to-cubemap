use std::{env, fs::File, io::BufWriter, path::Path};

use equirectangular_to_cubemap::convert;
use image::{codecs::jpeg::JpegEncoder, io::Reader as ImageReader, DynamicImage, ImageResult};

pub fn save_jpg<P>(output_path: P, img: &DynamicImage, jpg_quality: u8) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    // So we can save jpg with higher quality than default
    let fout = &mut BufWriter::new(File::create(output_path).unwrap());
    let mut j = JpegEncoder::new_with_quality(fout, jpg_quality);
    j.encode_image(img)
}

/// Example usage
fn main() {
    println!("Example usage:");
    println!("cargo run --release autumn_park_1k.exr autumn_park_1k_out.exr");
    let mut args = env::args();
    args.next();
    let input_path = args.next().unwrap();
    let output_path = args.next().unwrap();
    let img = ImageReader::open(&input_path).unwrap().decode().unwrap();
    let img = convert(&img, img.width() / 4);
    let extension = Path::new(&output_path).extension().unwrap();
    if extension == "jpg" || extension == "jpeg" {
        save_jpg(output_path, &img, 95).unwrap();
    } else {
        img.save(output_path).unwrap();
    }
}
