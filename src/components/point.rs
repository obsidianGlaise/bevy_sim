use std::fmt;
use bevy::{math::vec3, prelude::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Point(f64,f64,f64);

impl Point {
    pub fn new() -> Point { Point(0.0,0.0,0.0) }

    pub fn from(x: f64, y: f64, z: f64) -> Point {
        return Point(x,y,z);
    }
    
    pub fn add(points: Vec<Point>) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for i in points {
            x += i.0;
            y += i.1;
            z += i.2;
        }
        return Point(x,y,z);
    }

    pub fn s_times(s: f64, v: Point) -> Point { Point(s*v.0,s*v.1,s*v.2) }
    
    pub fn s_div(s: f64, v: Point) -> Point { Point(v.0/s,v.1/s,v.2/s) }

    pub fn unit(self) -> Point {
        if self.mag() == 0.0 { return Point(0.0,0.0,0.0); }
        Point(self.0/self.mag(), self.1/self.mag(), self.2/self.mag())
    }

    pub fn neg(self) -> Point { Point(-self.0,-self.1,-self.2) }

    pub fn mag(self) -> f64 {
        return f64::powf(f64::powf(self.0,2.0)+f64::powf(self.1,2.0)+f64::powf(self.2,2.0),0.5)
    }

    pub fn align(self, v: Point) -> Point {
        Self::s_times(self.mag(), v.unit())
    }

    pub fn x(self) -> f64 { self.0 }
    pub fn y(self) -> f64 { self.1 }
    pub fn z(self) -> f64 { self.2 }

    pub fn to_vec(self) -> Vec3 {
        vec3(self.0 as f32,self.1 as f32,self.2 as f32)
    }

    pub fn dist(a: Point, b: Point) -> f64 {
        let x = f64::powf(b.x()-a.x(),2.0);
        let y = f64::powf(b.y()-a.y(),2.0);
        let z = f64::powf(b.z()-a.z(),2.0);
        return f64::powf(x+y+z, 0.5);
    }

}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2},{:.2},{:.2})", self.0,self.1,self.2)
    }
}