use si_img::{SiImage, SiFont};
use std::fs::File;

fn main() {
    let font = SiFont::new("https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf", None);
    let mut img = SiImage::new("https://res.cloudinary.com/zype/image/upload/w_1200,h_650/CodeWithR/Template.png", font.clone(), None);
    let img = img.text("Hello, World!", 64.0, 480.00, 254.00, None).font(font).text("Finally, it freaking works!", 48.0, 480.00, 320.0, Option::Some("#00ffff".to_string()));
    let mut file = File::create("output.png").expect("Could not create bytes");
    let _ = image::load_from_memory(&img.as_bytes()).expect("Could not load image").write_to(&mut file, image::ImageFormat::Png);
    println!("Created!")
}