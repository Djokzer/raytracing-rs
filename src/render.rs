use png;

// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::vec;
use crate::vector::Vec3;

pub struct Render
{
	pub path	:	String,
	pub r_size	:	(usize, usize),
	pub buffer	:	Vec<Vec<(u8, u8, u8, u8)>>,	//RGBA
}

impl Render
{
	pub fn new(path: String, img_size: (usize, usize)) -> Self
	{
		Self {path: path, r_size: img_size, buffer: vec![vec![(0, 0, 0, 255); img_size.0]; img_size.1]}
	}

	#[allow(dead_code)]
	pub fn color_render_test(&mut self)
	{
		for i in 0..self.r_size.0
		{
			for j in 0..self.r_size.1
			{
				let coord: Vec<f64> = vec![(j as f64) / (self.r_size.0 as f64), (i as f64) / (self.r_size.1 as f64)];

				self.buffer[i][j].0 = (coord[0] * 255.0) as u8;
				self.buffer[i][j].1 = (coord[1] * 255.0) as u8;
				self.buffer[i][j].2 = 128;
			}
		}
	}

	pub fn render(&mut self)
	{
		for y in 0..self.r_size.1
		{
			for x in 0..self.r_size.0
			{
				let mut coord: Vec<f64> = vec![(x as f64) / (self.r_size.0 as f64), (y as f64) / (self.r_size.1 as f64)];
				coord[0] = coord[0] * 2.0 - 1.0;	//0 -> -1
				coord[1] = coord[1] * 2.0 - 1.0;	//0 -> -1
				self.buffer[x][y] = self.per_pixel(coord);
			}
		}
	}

	pub fn per_pixel(&mut self, coord: Vec<f64>) -> (u8, u8, u8, u8)
	{
		//	Sphere quadratic formula
		// 	(b.x^2 + b.y^2 + b.z^2) * t^2 + 2*(a.x * b.x + a.y * b.y + a.z * b.z) * t + (a.x^2 + a.y^2 + a.z^2 - r^2) = 0
		//	a = Ray Origin
		//	b = Ray Direction
		//	r = Sphere radius
		//	t = Hit Distance = Our Variable
		let ray_origin : Vec3 = Vec3::new(0.0, 0.0, 2.0);
		let ray_direction : Vec3 = Vec3::new(coord[0], coord[1], -1.0);
		let radius = 1.0;

		//Viete formula
		let a :f64 = ray_direction * ray_direction;
		let b :f64 = 2.0 * (ray_origin * ray_direction);
		let c :f64 = (ray_origin * ray_origin) - radius*radius;
		
		//	dt = b^2 - 4 * a * c
		let dt = b*b - 4.0 * a * c;	//delta
		
		if dt < 0.0	//IF RAY DOES NOT TOUCH THE SPHERE
		{
			return Vec3::new(0.0, 0.0, 0.0).to_rgba();
		}

		// s0, s1 = (-b +- dt.sqrt()) / (2.0 * a)
		let solution: f64 = (-b - dt.sqrt()) / (2.0 * a);	//MINUS IS THE CLOSEST SOLUTION

		let mut hit_point : Vec3 = ray_origin + ray_direction * solution;
		let normalized_hp : Vec3 = hit_point.normalize();
		let light_direction : Vec3 = Vec3::new(1.0, -1.0, -1.0).normalize();

		let lighting : f64 = (normalized_hp * (-light_direction)).max(0.0);

		let color: (u8, u8, u8, u8) = (70, 180, 160, 255);

		let mut vec_color : Vec3 = Vec3::rgb_to_vec3(color);
		vec_color = vec_color * lighting;

		return vec_color.to_rgba();
	}

	pub fn buffer_to_1d(&mut self) -> Vec<u8>	
	{
		let mut d_array: Vec<u8> = vec![];
		for i in 0..self.r_size.0
		{
			for j in 0..self.r_size.1
			{
				d_array.push(self.buffer[i][j].0);	//R	
				d_array.push(self.buffer[i][j].1);	//G
				d_array.push(self.buffer[i][j].2);	//B
				d_array.push(self.buffer[i][j].3);	//A
			}
		}
		return d_array;
	}

	pub fn export_png(&mut self)
	{
		let path = Path::new(&self.path);
		let file = File::create(path).unwrap();
		let ref mut w = BufWriter::new(file);

		let mut encoder = png::Encoder::new(w, self.r_size.0 as u32, self.r_size.1 as u32); // Width and height
		encoder.set_color(png::ColorType::Rgba);
		encoder.set_depth(png::BitDepth::Eight);
		encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
		encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
		let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
			(0.31270, 0.32900),
			(0.64000, 0.33000),
			(0.30000, 0.60000),
			(0.15000, 0.06000)
		);
		encoder.set_source_chromaticities(source_chromaticities);
		let mut writer = encoder.write_header().unwrap();
		
		let data = self.buffer_to_1d();
		writer.write_image_data(&data).unwrap(); // Save
	}
}