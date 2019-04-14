use crate::geometry::*;
use crate::canvas::*;
use crate::object::*;

const DELTA_DIST: f64 = 0.02;
const MAX_DIST: f64   = 1000.0;
const BLACK_DIST: f64 = 100.0;

struct Camera {
    pos: Point,
    dir: Vector,
    up: Vector,
    width: f64,
    height: f64,
}

impl Camera {
    fn new(pos: Point, dir: Vector, up: Vector, width: f64, height: f64) -> Self {
        Self {
            pos, dir, up, width, height
        }
    }
    
    fn left(&self) -> Vector {
        Vector::cross(self.up, self.dir)
    }
    
    // x and y between 0 .. 1
    fn get_point(&self, x: f64, y: f64) -> Point {
        let relx = x - 0.5;
        let rely = y - 0.5;
        self.pos + (self.dir + 
               (Vector::size(self.up, self.height * rely)) + 
               (Vector::size(self.left(), self.width * relx)))
    }
        
    fn from(&self, p: Point, s: f64) -> Point {
        p + Vector::size(p - self.pos, s)
    }
    
    fn distance(&self, p: &Point) -> f64 {
        (*p - self.pos).norm()
    }
}

pub trait Draw {
    fn distance(&self, p: &Point) -> f64;
}

pub type ObjectManager = Vec<Box<Draw>>;

impl Draw for ObjectManager {
    fn distance(&self, p: &Point) -> f64 {
        let mut dist: f64 = std::f64::MAX;
        let mut temp: f64 = 0.0;
        for obj in self.iter() {
            temp = obj.distance(p);
            if temp < dist { dist = temp; }
        }
        dist
    }
}


pub struct SceneManager {
    om: ObjectManager,
    cam: Camera,
}

unsafe impl Send for SceneManager {}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            om: vec![Box::new(Sphere::new(Point::new(0.0, 10.0, 0.0), 10.0)) ],
            cam: Camera::new(Point::new(0.0,0.0,0.0), 
                             Vector::new(0.0, 1.0, 0.0), 
                             Vector::new(0.0, 0.0, 1.0), 
                             10.0 , 10.0),
        }
    }
    
    pub fn update(&self) {
    }
    
    pub fn get_pixel(&self, x: f64, y: f64) -> Pixel {
        let point = self.get_point(x, y);
        let dist = self.cam.distance(&point);
        let scale: f64 = 20.0;
        let mut dt = (-1.0/(((dist + 200.0) * scale) * ((dist + 200.0) * scale))) + 1.0;
        dt = if dt <= 0.0 { 0.0 } else { dt };
        Pixel::new((255.0 * dt) as u8, (255.0 * dt) as u8, (255.0 * dt) as u8, 255)
    }
    
    fn get_point(&self, x: f64, y: f64) -> Point {
        let mut point: Point = self.cam.get_point(x, y);
        let mut dist: f64 = 1.0;
        while dist >= DELTA_DIST && dist <= MAX_DIST {
            dist = self.om.distance(&point);
            point = self.cam.from(point, dist);
        }
        point
    }
}
