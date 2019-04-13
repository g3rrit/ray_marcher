use std::thread;
use std::sync::{Arc, Mutex};
use crate::canvas::*;

enum RenderState {
    Done,
    Running,
}

pub struct Renderer {
    canvas: Arc<Canvas>,
    rs: Arc<Mutex<RenderState>>,
}

impl Renderer {
    pub fn new(c: RefImageBuffer) -> Self {
        Self {
            canvas: Arc::new(Canvas::new(c)),
            rs: Arc::new(Mutex::new(RenderState::Done)),
        }
    }
    
    pub fn update(&self) {
        let ars = Arc::clone(&self.rs);
        let mut rs = self.rs.lock().unwrap();
        if let RenderState::Done = *rs {
            *rs = RenderState::Running;
            let arc = Arc::clone(&self.canvas);
            thread::spawn(move || Self::update_screen(arc, ars));
        }
    }
    
    fn update_screen(canvas: Arc<Canvas>, rs: Arc<Mutex<RenderState>>) {
        
        let p = Pixel::new(255, 0,0, 255);
        for x in 0..100 {
            for y in 0..100 {
                canvas.put_pixel(x,y, p);
            }
        }

        // -- update renderstate
        let mut rs = rs.lock().unwrap();
        *rs = RenderState::Done;
    }
}
