use png;

// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


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

	pub fn color_render_test(&mut self)
	{
		for i in 0..self.r_size.0
		{
			for j in 0..self.r_size.1
			{
				self.buffer[i][j].0 = i as u8 % 255;
				self.buffer[i][j].1 = j as u8 % 255;
				self.buffer[i][j].2 = 128;
			}
		}
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