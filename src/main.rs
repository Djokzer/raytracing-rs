mod render;

fn main() 
{
    let mut r = render::Render::new("render/test.png".to_string(), (1024, 1024));
    r.export_png();
}
