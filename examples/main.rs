use si_img::{SiFont, SiImage, SiPreset};
use std::{fs::File, any::TypeId};

fn main() {
    let font = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
    );
    let preset = SiPreset::new(Box::new(|img: &mut SiImage, values: std::collections::HashMap<String, Box<dyn std::any::Any>>| {
        println!("Dimensions: {}, {}", img.clone().width(), img.clone().height());
        let font = match values.get("font") {
            Some(font) => {
                if font.type_id() == TypeId::of::<SiFont>() {
                    font.downcast_ref::<SiFont>().unwrap().clone()
                } else {
                    SiFont::from_network(
                        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
                    )
                }
            }
            None => SiFont::from_network(
                "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
            )
        };
        let new_img = img.clone().render_text(
            "Hello from Preset!",
            32.0,
            480.00,
            480.00,
            Some("#00ffff".to_string()),
            font,
        );
        new_img
    }));
    let img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic");

    let mut binding = img.render_text(
        "Hello, World!",
        64.0,
        480.00,
        254.00,
        Some("#00ffff".to_string()),
        font.clone(),
    );
    let font = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/arial.ttf",
    );
    let _font: Box<dyn std::any::Any> = Box::new(font.clone());
    let img = binding.load_preset(preset, std::collections::HashMap::from([("font".to_string(), _font)]));
    let mut file = File::create("output.png").expect("Could not create bytes");
    let _ = image::load_from_memory(&img.clone().to_bytes())
        .expect("Could not load image")
        .write_to(&mut file, image::ImageFormat::Png);
    println!("Created!")
}
