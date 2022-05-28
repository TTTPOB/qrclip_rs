use qrclip_rs::*;
use clipboard_win::{set_clipboard, formats};

fn main() {
    let img = get_clipboard_image();
    let code_content = image_to_string_vec(img);
    set_clipboard(formats::Unicode, code_content).unwrap();
}
