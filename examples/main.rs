use si_img::{SiImage, SiFont};
use std::fs::File;

fn main() {
    let font = SiFont::from_network("https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf");
    let mut img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic", font.clone());
    let img = img.text("Hello, World!", 64.0, 480.00, 254.00, Some("#00ffff".to_owned()));
    let mut file = File::create("output.png").expect("Could not create bytes");
    let _ = image::load_from_memory(&img.to_bytes()).expect("Could not load image").write_to(&mut file, image::ImageFormat::Png);
    println!("Created!")
}