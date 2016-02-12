extern crate libc;
extern crate lodepng;

use std::ffi::CString;
use self::libc::{c_char, c_double, size_t, c_int};

pub fn draw_char(
    buf: &mut Vec<u8>,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    angle: f64,
    font_size: usize,
    fgcolor: &str,
    fontname: &str,
    c: char) -> Result<(), &'static str> {

    CString::new(fgcolor)
        .map_err(|_| "Invalid foreground color.")
        .and_then(|color| {
            CString::new(fontname)
                .map_err(|_| "Invalid fontname.")
                .and_then(|font| {
                    let mut cs = String::new();
                    cs.push(c);
                    CString::new(cs)
                        .map_err(|_| "Invalid character.")
                        .and_then(|text| {
                            unsafe {
                                draw_on_buf(
                                    buf.as_mut_ptr() as *mut c_char,
                                    x      as c_int,
                                    y      as c_int,
                                    width  as size_t,
                                    height as size_t,
                                    angle  as c_double,
                                    font_size      as size_t,
                                    color.as_ptr() as *const c_char,
                                    font.as_ptr()  as *const c_char,
                                    text.as_ptr()  as *const c_char
                                )
                            }
                            Ok(())
                        })
                })
        })
}

pub fn save_img(buf: &Vec<u8>,
    width: usize, height: usize, filename: &str) -> Result<(), &'static str> {

    match CString::new(filename) {
        Err(_) => Err("Could not create filename."),
        Ok(s)  => {
            unsafe {
                let r = save_buf(
                    buf.as_ptr() as *mut c_char,
                    width as size_t,
                    height as size_t,
                    s.as_ptr() as *const c_char
                );
                match r {
                    0 => Ok(()),
                    _ => Err("Could not save image.")
                }
            }
        }
    }
}

pub fn init_img() {
    unsafe {
        init_image();
    }
}

pub fn done_img() {
    unsafe {
        done_image();
    }
}

// ----------------------------------------------------------------------------

// TODO libraries should be configures in build.rs
#[link(name = "image", kind = "static")]
#[link(name = "MagickWand-6.Q16")]
#[link(name = "MagickCore-6.Q16")]
extern {
    fn draw_on_buf(
        buf: *mut c_char,
        x: c_int,
        y: c_int,
        width: size_t,
        height: size_t,
        angle: c_double,
        font_size: size_t,
        fgcolor: *const c_char,
        font: *const c_char,
        txt: *const c_char
    );

    fn save_buf(
        buf: *mut c_char,
        width: size_t,
        height: size_t,
        filename: *const c_char
    ) -> c_int;

    fn init_image();
    fn done_image();
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use std::fs;
    use super::{init_img, done_img, draw_char, save_img};

    #[test]
    fn test_draw_on_buf() {
        let mut v: Vec<u8> = repeat(255).take(200 * 200 * 3).collect();
        let _ = fs::remove_file("/tmp/a.jpg");

        init_img();
        draw_char(
            &mut v, 50, 100, 200, 200, -10.0, 72,
            "green",
            "Verdana-Bold-Italic",
            'Q'
        ).unwrap();
        save_img(&mut v, 200, 200, "/tmp/a.png").unwrap();
        done_img();
    }
}
