use std::vec;

pub struct Vector
{
    size : u32,
    data : Vec<f64>
}

impl Vector
{
    pub fn new(size: u32) -> Self
    {
        Self { size: size, data: vec![0.0; size as usize]}
    }

    //TO DO ALL VECTORS OPERATIONS
}