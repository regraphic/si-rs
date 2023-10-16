use si_img::{SiFont, SiImage};
use tokio; // Make sure to enable the "async" feature
use image;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let sirin = SiFont::from_network_async(
            "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
        )
        .await
        .expect("Could not load font");
        let arial = SiFont::from_network_async(
            "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/arial.ttf",
        )
        .await
        .expect("Could not load font");
        let mut img = SiImage::from_network_async(
            "https://res.cloudinary.com/zype/image/upload/w_1200,h_650/CodeWithR/Template.png"
        )
        .await;
        let mut img = img
            .render_text("Hello, World!", 64.0, 480.00, 254.00, None, sirin)
            .render_text("Finally, it works!", 48.0, 480.00, 320.0, None, arial);
        let mut file = File::create("output_async.png").expect("Could not create bytes");
        image::load_from_memory(&img.to_bytes())
            .expect("Could not load image")
            .write_to(&mut file, image::ImageFormat::Png);
        println!("Created!");
    Ok(())
}