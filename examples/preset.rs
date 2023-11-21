use std::{collections::HashMap, fs, io::Write};

use si_img::{SiFont, SiImage, SiPreset, TextOptions};

fn main() {
    // Create the preset
    let preset = SiPreset::new(Box::new(|img, vals| {
        let new_img = img.clone();
        // img is the full image
        println!("Dimensions: {}x{}", img.width(), img.height());
        println!("Values: {:?}", vals);
        // Get the font
        let font = match vals.get("font") {
            Some(font) => {
                // Do type checking
                if font.type_id() == std::any::TypeId::of::<SiFont>() {
                    // Downcast it
                    font.downcast_ref::<SiFont>().unwrap()
                } else {
                    panic!(
                        "Expected type: {:?}\nFound type: {:?}",
                        std::any::TypeId::of::<SiFont>(),
                        font.type_id()
                    );
                }
            }
            None => panic!("No font provided"),
        };
        // Render something on the image with that font
        // Get the title
        let title = match vals.get("title") {
            Some(title) => {
                // Do type checking
                if title.type_id() == std::any::TypeId::of::<String>() {
                    // Downcast it
                    title.downcast_ref::<String>().unwrap()
                } else {
                    panic!(
                        "Expected type: {:?}\nFound type: {:?}",
                        std::any::TypeId::of::<String>(),
                        title.type_id()
                    );
                }
            }
            None => panic!("No title provided"),
        };
        let text_options = TextOptions::default();
        // Render it
        new_img.render_text(title, 64.0, 480.0, 254.0, None, &font, &text_options)
    }));

    // Use it
    // Create the image
    let mut img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic");
    // Create the font
    let font = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
    );
    let font_val: Box<dyn std::any::Any> = Box::new(font);
    let title_val: Box<dyn std::any::Any> = Box::new("Hello, World!".to_string());
    let values: HashMap<String, Box<dyn std::any::Any>> = HashMap::from([
        ("font".to_string(), font_val),
        ("title".to_string(), title_val),
    ]);
    img.load_preset(preset, values);
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        // either use the ? operator or unwrap since it returns a Result
        .open("out.png")
        .unwrap();
    let _ = file.write_all(&img.to_bytes()).unwrap();
}
