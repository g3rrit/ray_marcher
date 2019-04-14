use std::thread;
use std::sync::{Arc, Mutex};
use crate::canvas::*;
use crate::scene::*;

enum RenderState {
    Done,
    Running,
}

pub struct Engine {
    canvas: Arc<Canvas>,
    rs: Arc<Mutex<RenderState>>,
    sm: Arc<Mutex<SceneManager>>,
}

impl Engine {
    pub fn new(c: RefImageBuffer) -> Self {
        Self {
            canvas: Arc::new(Canvas::new(c)),
            rs: Arc::new(Mutex::new(RenderState::Done)),
            sm: Arc::new(Mutex::new(SceneManager::new())),
        }
    }
    
    pub fn update(&self) {
        let ars = Arc::clone(&self.rs);
        let asm = Arc::clone(&self.sm);
        let mut rs = self.rs.lock().unwrap();
        if let RenderState::Done = *rs {
            *rs = RenderState::Running;
            let arc = Arc::clone(&self.canvas);
            thread::spawn(move || Self::update_screen(arc, asm, ars));
        }
        self.sm.lock().unwrap().update();
    }
    
    fn update_screen(canvas: Arc<Canvas>, sm: Arc<Mutex<SceneManager>>, rs: Arc<Mutex<RenderState>>) {
        
        // test
        let p = Pixel::new(255, 0,0, 255);
        for x in 0..100 {
            for y in 0..100 {
                canvas.put_pixel(x,y, p);
            }
        }
        
        // fill canvas
        let (cw, ch) = canvas.dim();
        for x in 0..cw {
            for y in 0..ch {
                canvas.put_pixel(x, y, sm.lock().unwrap().get_pixel(x as f64/(cw as f64), y as f64/(ch as f64)));
            }
        }
        

        // -- update renderstate
        let mut rs = rs.lock().unwrap();
        *rs = RenderState::Done;
    }
}
