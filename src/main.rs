use std::{env, fs::File, io::BufWriter, path::Path};

use equirectangular_to_cubemap::convert;
use image::{codecs::jpeg::JpegEncoder, io::Reader as ImageReader};

/// Example usage
fn main() {
    let mut args = env::args();
    args.next();
    let input_path = args.next().unwrap();
    let output_path = args.next().unwrap();
    let img = ImageReader::open(&input_path).unwrap().decode().unwrap();
    let extension = Path::new(&output_path).extension().unwrap();
    let converted = convert(&img, img.width() / 4);
    if extension == "jpg" || extension == "jpeg" {
        // Save jpg with higher quality than default
        let fout = &mut BufWriter::new(File::create(output_path).unwrap());
        let mut j = JpegEncoder::new_with_quality(fout, 95);
        j.encode_image(&converted).unwrap();
    } else {
        converted.save(output_path).unwrap();
    }
}
