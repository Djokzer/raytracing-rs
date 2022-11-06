use std::{vec};

pub struct Vector
{
    pub size : u32,
    pub data : Vec<f64>
}

impl Vector
{
    pub fn empty(size : u32) -> Self
    {
        Self { size: size, data: vec![0.0; size as usize]}
    }

    pub fn new(size: u32, vec: Vec<f64>) -> Self
    {
        Self { size: size, data: vec}
    }

    pub fn add(vec1 : &Vector, vec2 : &Vector) -> Vector
    {
        let mut vec3 = Vector::empty(vec1.size);

        for i in 0..vec1.size as usize
        {
            vec3.data[i] = vec1.data[i] + vec2.data[i];
        }

        return vec3;
    }

    pub fn sub(vec1 : &Vector, vec2 : &Vector) -> Vector
    {
        let mut vec3 = Vector::empty(vec1.size);

        for i in 0..vec1.size as usize
        {
            vec3.data[i] = vec1.data[i] - vec2.data[i];
        }

        return vec3;
    }
    
    pub fn mul(vec1 : &Vector, scalar : f64) -> Vector
    {
        let mut vec3 = Vector::empty(vec1.size);

        for i in 0..vec1.size as usize
        {
            vec3.data[i] = vec1.data[i] * scalar;
        }

        return vec3;
    }

    pub fn dot(vec1 : &Vector, vec2 : &Vector) -> f64
    {
        let mut dot = 0.0;

        for i in 0..vec1.size as usize
        {
            dot += vec1.data[i] * vec2.data[i];
        }

        return dot;
    }
    
    //TO DO ALL VECTORS OPERATIONS
}