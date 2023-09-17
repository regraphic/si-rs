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
            "https://res.cloudinary.com/zype/image/upload/w_1200,h_650/CodeWithR/Template.png",
            sirin,
        )
        .await;
        let mut img = img
            .text("Hello, World!", 64.0, 480.00, 254.00, None)
            .set_font(arial)
            .text("Finally, it freaking works!", 48.0, 480.00, 320.0, None);
        // let mut new_img = SiImage{font: arial, image: img};
        // // let mut file = File::create("output_1.png").expect("Could not create bytes");
        // // file.write_all(&new_img.render_text("Tagline!!!", 48.0, 480.0, 320.0).as_bytes());
        // let binding = new_img.render_text("Finally!!! It freaking worked!", 48.0, 480.0, 320.0);
        // let b = binding.clone();
        // // println!("{}", b.len())
        let mut file = File::create("output_2.png").expect("Could not create bytes");
        // b.write_to(&mut file, image::ImageFormat::Png);
        // img.set_font(arial);
        // let a = img.render_text("Finally!!! It freaking worked!", 48.0, 480.0, 320.0);
        image::load_from_memory(&img.to_bytes())
            .expect("Could not load image")
            .write_to(&mut file, image::ImageFormat::Png);
        println!("Created!");
    Ok(())
}