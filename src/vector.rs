use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Add for Vec3
{
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3
    {
        Vec3
        {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3
{
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3
    {
        Vec3
        {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3
    {
        Vec3
        {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

//DOT PRODUCT
impl Mul for Vec3
{
    type Output = f64;
    fn mul(self, rhs: Vec3) -> f64
    {
        let mut dot = 0.0;
        dot += self.x * rhs.x;
        dot += self.y * rhs.y;
        dot += self.z * rhs.z;
        return dot;
    }
}

impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Self
    {
        Self { x: x, y: y, z: z}
    }
}

