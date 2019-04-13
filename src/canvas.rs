use im::Rgba;
use std::sync::{Arc, Mutex};

type ImageBuffer = im::ImageBuffer<Rgba<u8>, Vec<u8>>;
pub type RefImageBuffer = Arc<Mutex<ImageBuffer>>;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba {
            data: [ self.r, self.g, self.b, self.a ],
        }
    }
}

pub struct Canvas {
    c: RefImageBuffer,
}

impl Canvas {
    pub fn new(c: RefImageBuffer) -> Self {
        Self {
            c: c,
        }
    }
    
    pub fn dim(&self) -> (u32, u32) {
        self.c.lock().unwrap().dimensions()
    }
    
    pub fn put_pixel(&self, x: u32, y: u32, p: Pixel) {
        self.c.lock().unwrap().put_pixel(x, y, p.to_rgba());
    }
    
}
