use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy,Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }
    
    pub fn point(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
    
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn norm_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn dot(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z + v2.z
    }
    
    pub fn size(v: Vector, s: f64) -> Vector {
        let ns: f64 = s / v.norm();
        v * ns
    }
    
    pub fn cross(v1: Vector, v2: Vector) -> Vector {
        Vector::new(v1.y * v2.z - v1.z * v2.y,
                    v1.z * v2.x - v1.x * v2.z,
                    v1.x * v2.y - v1.y * v2.x)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, self.z)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }
    
    pub fn vec(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }

    pub fn distance(p1: &Point, p2: &Point) -> f64 {
        Vector::new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z).norm()
    }
    
    pub fn distance_square(p1: &Point, p2: &Point) -> f64 {
        Vector::new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z).norm_square()
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
