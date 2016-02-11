use std::iter::repeat;
use image::{draw_char, save_img};

enum Location {
    South,
    East,
    West,
    North,
}

pub struct Image {
    buf: Vec<u8>,
    height: usize,
    width : usize,
}

#[derive(PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn white() -> Pixel {
        Pixel { r: 255, g: 255, b: 255 }
    }

    pub fn black() -> Pixel {
        Pixel { r: 0, g: 0, b: 0 }
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image::fill(width, height, 0)
    }

    pub fn fill(width: usize, height: usize, c: u8) -> Image {
        Image {
            height: height,
            width: width,
            buf: repeat(c).take(width * height * 3).collect::<_>(),
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

    pub fn pixel(&self, x: usize, y: usize) -> Option<Pixel> {

        if x >= self.width || y >= self.height {
            return None;
        }

        let off = y * self.width * 3 + x * 3;
        Some(Pixel {
            r: self.buf[off + 0],
            g: self.buf[off + 1],
            b: self.buf[off + 2],
        })
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) -> Option<()> {

        if x >= self.width || y >= self.height {
            return None;
        }

        let off = y * self.width * 3 + x * 3;
        self.buf[off + 0] = p.r;
        self.buf[off + 1] = p.g;
        self.buf[off + 2] = p.b;
        Some(())
    }

    pub fn save(&self, filename: &str) -> Result<(), &'static str> {
        save_img(&self.buf, self.width, self.height, filename)
    }

/*
    pub fn border_size(&self, location: Location) -> usize {

        match location {
            Location::East =>
        }
    }
*/
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
    use super::{Image, Pixel};

    #[test]
    fn test_image_new() {
        let i = Image::new(0, 0);
        assert!(i.height == 0 && i.width == 0 && i.buf.len() == 0);
        let j = Image::new(100, 10);
        assert!(j.height == 10 && j.width == 100 && j.buf.len() == 3000);
    }

    #[test]
    fn test_save_img() {
        let mut i = Image::fill(250, 250, 255);
        for x in 0..250 {
            i.set_pixel(x, 1, Pixel::black());
        }
        i.save("/tmp/b.jpg").unwrap();
    }

    #[test]
    fn test_pixel() {
        assert!(Image::new(100, 10).pixel(0, 0).unwrap() == Pixel::black());
        assert!(Image::fill(100, 10, 255).pixel(0, 0).unwrap() == Pixel::white());
    }
}
