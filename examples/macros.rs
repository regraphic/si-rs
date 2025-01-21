use si_img::{anymap, preset, render};

use std::{collections::HashMap, fs, io::Write};

use si_img::{SiFont, SiImage, SiPreset, TextOptions};

fn main() {
    // Create it with macro
    preset! {
        my_preset(img, font: SiFont, title: String, tagline: String) {
            println!("{}", title);
            let mut new = img;
            render!(new: title; 480.0, 254.0; "font" &font, "scale" 64.0, "opts" &TextOptions::default(), "color" None);
            render!(new: tagline; 480.0, 320.0; "font" &font, "scale" 48.0, "opts" &TextOptions::default(), "color" Some(String::from("#FFFFFF")));
            new
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
    file.write_all(&img.to_bytes()).unwrap();
}
