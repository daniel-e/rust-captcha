extern crate libc;

use self::libc::{c_char, c_double, size_t};

// TODO libraries should be configures in build.rs
#[link(name = "image", kind = "static")]
#[link(name = "MagickWand-6.Q16")]
#[link(name = "MagickCore-6.Q16")]
extern {
    fn draw_on_buf(
        buf: *mut c_char,
        x: size_t,
        y: size_t,
        width: size_t,
        height: size_t,
        angle: c_double,
        font_size: size_t,
        fgcolor: *const c_char,
        font: *const c_char,
        txt: *const c_char
    );
}

#[cfg(test)]
mod tests {
    extern crate libc;
    use self::libc::{c_char, c_double, size_t};
    use std::iter::repeat;
    use super::draw_on_buf;

    #[test]
    fn test_draw_on_buf() {
        let mut v: Vec<u8> = repeat(255).take(200 * 200 * 3).collect();

        let mut color = "green".to_string();
        color.push('\0');
        let mut font = "Verdana-Bold-Italic".to_string();
        font.push('\0');
        let mut text = "P".to_string();
        text.push('\0');

        unsafe {
            draw_on_buf(
                v.as_mut_ptr() as *mut c_char,
                50    as size_t,
                100   as size_t,
                200   as size_t,
                200   as size_t,
                -10.0 as c_double,
                72    as size_t,
                color.as_ptr() as *const c_char,
                font.as_ptr() as *const c_char,
                text.as_ptr() as *const c_char
            )
        }
    }
}
