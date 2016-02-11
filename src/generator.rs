use std::iter::repeat;
use image::draw_char;

pub struct Image {
    buf: Vec<u8>,
    height: usize,
    width : usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            height: height,
            width: width,
            buf: repeat(0).take(width * height * 3).collect::<_>(),
        }
    }

    pub fn from_char(c: char) -> Image {
        let mut i = Image::new(300, 300);
        draw_char(
            &mut i.buf,
            150,
            150,
            i.width, i.height,
            -10.0,
            72,
            "black",
            "Verdana-Bold-Italic",
            c);
        i
    }

    //pub fn as_png() -> Vec<u8> {

    //}
}


pub fn image(chars: String) -> Image {
    chars.chars().fold(Image::new(0, 0), |a, b| {
        Image::from_char(b)
        // TODO
    })
}

#[cfg(test)]
mod tests {
    use super::{Image, image};

    #[test]
    fn test_image_new() {
        let i = Image::new(0, 0);
        assert!(i.height == 0 && i.width == 0 && i.buf.len() == 0);
        let j = Image::new(100, 10);
        assert!(j.height == 10 && j.width == 100 && j.buf.len() == 3000);
    }
}
