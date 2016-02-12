extern crate rand;
extern crate lodepng;

use std::mem;
use self::rand::os::OsRng;
use self::rand::Rng;

use self::lodepng::{ColorType, encode_memory, RGB};

use std::iter::repeat;
use image::{draw_char, save_img};

pub struct CharConfig {
    min_angle: f64,
    max_angle: f64,
    min_size: usize,
    max_size: usize,
    colors: Vec<String>,
    fonts: Vec<String>,
}

pub fn captcha_png(chars: &str, cc: &CharConfig) -> Result<Vec<u8>, &'static str> {

    image(chars.to_string(), cc).and_then(|mut x| x.as_png())
}

// ----------------------------------------------------------------------------

struct Image {
    buf: Vec<u8>,
    pub height: usize,
    pub width : usize,
}

fn image(chars: String, cc: &CharConfig) -> Result<Image, &'static str> {

    let default_color = "black".to_string();
    let default_font = "Verdana-Bold-Italic".to_string();

    // TODO check min_angle vs max_angle + min_size vs max_size

    OsRng::new()
        .map_err(|_| "Could not create random number generator.")
        .and_then(|mut rng| {
            do_image(chars.chars().map(|c| {
                let angle: f64 = rng.gen_range(cc.min_angle, cc.max_angle);
                let fontsize: usize = rng.gen_range(cc.min_size, cc.max_size);
                let color = rng.choose(&cc.colors[..]).unwrap_or(&default_color);
                let font = rng.choose(&cc.fonts[..]).unwrap_or(&default_font);

                let i = Image::from_char(c, angle, fontsize, &color, &font);
                CharProperties {
                    width: i.width,
                    height: i.height,
                    borders: i.borders(Pixel::white()),
                    fontsize:  fontsize,
                    angle: angle,
                    color: color.clone(),
                    font: font.clone(),
                    c: c,
                }
            }).collect::<Vec<_>>())
        })
}

fn do_image(properties: Vec<CharProperties>) -> Result<Image, &'static str> {

    let width = properties.iter().map(|x| {
        x.width - x.borders.east - x.borders.west
    }).fold(0, |s, x| s + x);

    let maxheight = properties.iter().map(|x| {
        x.height - x.borders.north - x.borders.south
    }).max().unwrap();

    let mut i = Image::fill(width, maxheight, 255);
    let mut px: i32 = 0;

    for p in properties {
        let d = 150 as i32 - p.borders.east as i32;
        let py = 150 as i32 - p.borders.north as i32;
        // TODO check return value
        draw_char(
            &mut i.buf, (px + d), py,
            i.width, i.height,
            p.angle, p.fontsize, &p.color, &p.font, p.c
        );
        px += p.width as i32 - p.borders.east as i32 - p.borders.west as i32;
    }
    Ok(i)
}

struct Borders {
    north: usize,
    west: usize,
    south: usize,
    east: usize
}

#[derive(PartialEq)]
struct Pixel {
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

    pub fn from_char(c: char, angle: f64, fontsize: usize,
        color: &str, font: &str, ) -> Image {

        let mut i = Image::fill(300, 300, 255);
        // TODO check return value
        draw_char(
            &mut i.buf, 150, 150,
            i.width, i.height,
            angle, fontsize, color, font, c
        );
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

    pub fn borders(&self, p: Pixel) -> Borders {

        let mut b = Borders {
            north: 0, south: 0, east: 0, west: 0
        };

        b.north = (0..self.height).take_while(|&y| {
            (0..self.width).all(|x| self.pixel(x, y).unwrap() == p)
        }).count();

        b.south = (0..self.height).take_while(|&y| {
            (0..self.width).all(|x| self.pixel(x, self.height - 1 - y).unwrap() == p)
        }).count();

        b.east = (0..self.width).take_while(|&x| {
            (0..self.height).all(|y| self.pixel(x, y).unwrap() == p)
        }).count();

        b.west = (0..self.width).take_while(|&x| {
            (0..self.height).all(|y| self.pixel(self.width - 1 - x, y).unwrap() == p)
        }).count();

        b
    }

    pub fn as_png(&mut self) -> Result<Vec<u8>, &'static str> {

        unsafe {
            let mut buf = self.buf.iter().cloned().collect::<Vec<u8>>(); //self.buf.clone();

            let l = buf.len();
            let p = buf.as_mut_ptr(); // buf is still the owner of the pointer

            mem::forget(buf); // take ownership of buf; do not call destructor

            let q: *mut RGB<u8> = mem::transmute(p);
            let i: Vec<RGB<u8>> = Vec::from_raw_parts(q, l / 3, l / 3);
            let d = encode_memory::<RGB<u8>>
                (&i, self.width, self.height, ColorType::LCT_RGB, 8);

            match d {
                Ok(cvec) => {
                    let l = cvec.len();
                    //is this a mem leak?
                    //let v: Vec<u8> = Vec::from_raw_parts(cvec.into_inner(), l, l);
                    let v: Vec<u8> = Vec::from_raw_parts(mem::transmute(cvec.get(0)), l, l);
                    let k: Vec<u8> = v.iter().cloned().collect();
                    mem::forget(v);
                    Ok(k)
                },
                Err(e) => Err(e.as_str())
            }
        }
    }
}

struct CharProperties {
    width: usize,
    height: usize,
    borders: Borders,
    fontsize: usize,
    angle: f64,
    color: String,
    font: String,
    c: char,
}


// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{Image, Pixel, image, CharConfig, captcha_png};
    use image::{init_img, done_img};

    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_image_new() {
        init_img();
        let i = Image::new(0, 0);
        assert!(i.height == 0 && i.width == 0 && i.buf.len() == 0);
        let j = Image::new(100, 10);
        assert!(j.height == 10 && j.width == 100 && j.buf.len() == 3000);
        done_img();
    }

    #[test]
    fn test_save_img() {
        init_img();
        let mut i = Image::fill(250, 250, 255);
        for x in 0..250 {
            i.set_pixel(x, 1, Pixel::black());
        }
        i.save("/tmp/b.jpg").unwrap();
        done_img();
    }

    #[test]
    fn test_pixel() {
        init_img();
        assert!(Image::new(100, 10).pixel(0, 0).unwrap() == Pixel::black());
        assert!(Image::fill(100, 10, 255).pixel(0, 0).unwrap() == Pixel::white());
        done_img();
    }

    #[test]
    fn test_borders() {
        init_img();
        let mut i = Image::fill(200, 250, 255);
        i.set_pixel(3, 10, Pixel::black());
        let b = i.borders(Pixel::white());
        assert_eq!(b.north, 10);
        assert_eq!(b.south, 239);
        assert_eq!(b.east, 3);
        assert_eq!(b.west, 196);
        done_img();
    }

    #[test]
    fn test_image() {
        let c = CharConfig {
            min_angle: -15.0,
            max_angle: 15.0,
            min_size: 52,
            max_size: 72,
            colors: vec!["black".to_string()],
            fonts: vec!["Verdana-Bold-Italic".to_string()]
        };

        init_img();
        let mut i = image("abcdefgh".to_string(), &c).unwrap();
        i.save("/tmp/c.jpg").unwrap();
        done_img();

        let d = i.as_png().unwrap();
        let mut f = File::create("/tmp/p.png").unwrap();
        f.write(&d).unwrap();

        let j = captcha_png("abcdefgh", &c).unwrap();
        let mut g = File::create("/tmp/q.png").unwrap();
        g.write(&j).unwrap();
    }
}
