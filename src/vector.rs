use std::ops::{Add, Sub, Mul, Neg};

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

impl Neg for Vec3
{
    type Output = Vec3;
    fn neg(self) -> Vec3
    {
        Vec3
        {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
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

    pub fn rgb_to_vec3(rgba: (u8, u8, u8, u8)) -> Self
    {
        Self 
        { 
            x: rgba.0 as f64 / 255.0, 
            y: rgba.1 as f64 / 255.0, 
            z: rgba.2 as f64 / 255.0,
        }
    }

    pub fn normalize(&mut self) -> Vec3
    {
        let length : f64 = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 
        { 
            x: self.x / length, 
            y: self.y / length, 
            z: self.z / length, 
        }
    }

    pub fn max(&mut self, max: f64)
    {
        if self.x > max
        {
            self.x = max
        }
        if self.y > max
        {
            self.y = max
        }
        if self.x > max
        {
            self.y = max
        }
    }

    pub fn min(&mut self, min: f64)
    {
        if self.x < min
        {
            self.x = min
        }
        if self.y < min
        {
            self.y = min
        }
        if self.x < min
        {
            self.y = min
        }
    }

    pub fn clamp(&mut self, min: f64, max: f64)
    {
        self.max(max);
        self.min(min);
    }

    pub fn to_rgba(&mut self) -> (u8, u8, u8, u8)
    {
        self.clamp(0.0, 1.0);
        return ((self.x * 255.0) as u8, (self.y * 255.0) as u8, (self.z * 255.0) as u8, 255);
    }


}

