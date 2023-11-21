use si_img::{anymap, preset};

use std::{collections::HashMap, fs, io::Write};

use si_img::{SiFont, SiImage, SiPreset, TextOptions};

fn main() {
    // Create it with macro
    preset! {
        my_preset(img, font: SiFont, title: String, tagline: String) {
            println!("{}", title);
            img
                .render_text(title, 64.0, 480.0, 254.0, None, &font, &TextOptions::default())
                .render_text(tagline, 48.0, 480.0, 320.0, Some(String::from("#FFFFFF")), &font, &TextOptions::default())
        }
    };

    // Use it
    // Create the image
    let mut img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic");
    // Create the font
    let font = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
    );
    img.load_preset(
        my_preset,
        anymap! {
            font: font,
            title: "Hello, World!".to_string(),
            tagline: "Cool!".to_string()
        },
    );
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        // either use the ? operator or unwrap since it returns a Result
        .open("out.png")
        .unwrap();
    let _ = file.write_all(&img.to_bytes()).unwrap();
}
