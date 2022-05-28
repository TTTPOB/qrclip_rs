
use clipboard_win::{formats, get_clipboard};
use image;
use rqrr;

pub fn get_clipboard_image() -> image::DynamicImage {
    let img_bytes = get_clipboard(formats::Bitmap).unwrap();
    image::load_from_memory(&img_bytes).unwrap()
}

pub fn image_to_string_vec(img: image::DynamicImage) -> String {
    let img = img.to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let qr_scan_result = grids[0].decode();
    match qr_scan_result {
        Ok(qr_scan_result) => {let (_meta, string) = qr_scan_result; string},
        Err(e) => format!("{:?}", e),
    }
}
