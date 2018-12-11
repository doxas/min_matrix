
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 {x: x, y: y}
    }
    pub fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y;
        l.sqrt()
    }
    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(&self, other: &Vec2) -> f64 {
        self.x * other.y - self.y * other.x
    }
    pub fn normalize(&mut self) -> Vec2 {
        let l: f64 = self.length();
        self.x /= l;
        self.y /= l;
        Vec2 {x: self.x, y: self.y}
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}
impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

