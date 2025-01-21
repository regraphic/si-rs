use si_img::{SiFont, SiImage, TextOptions};

use std::{fs, io::Write};

fn main() {
    // Create the image
    let mut img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic");
    // Create the font
    let font = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
    );
    // Render some text
    let text_options = TextOptions::default();
    // img.render_text("Hello, World!", 64.0, 480.0, 254.0, None, &font, &text_options);
    // Render something else
    // img.render_text("Hello, World!", 48.0, 480.0, 320.0, None, &font, &text_options);
    // Or do chained
    img = img
        .clone()
        .render_text(
            "Hello, World!",
            64.0,
            480.0,
            254.0,
            None,
            &font,
            &text_options,
        )
        .render_text(
            "Hello, Another!",
            48.0,
            480.0,
            320.0,
            None,
            &font,
            &text_options,
        );
    // Write it
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true) // To write
        .open("out.png")
        .unwrap();
    file.write_all(&img.to_bytes()).unwrap();
}
