use si_img::{
    SiImage,
    SiFont
};

use std::{
    fs,
    io::Write
};

fn main() {
    // Create the image
    let mut img = SiImage::from_network("https://res.cloudinary.com/zype/image/upload/regraphic");
    // Create the font
    let font = SiFont::from_network("https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf");
    // Render some text
    // img.render_text("Hello, World!", 64.0, 480.0, 254.0, None, &font);
    // Render something else
    // img.render_text("Hello, World!", 48.0, 480.0, 320.0, None, &font);
    // Or do chained
    img = img
        .clone()
        .render_text("Hello, World!", 64.0, 480.0, 254.0, None, &font)
        .render_text("Hello, Another!", 48.0, 480.0, 320.0, None, &font);
    // Write it
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true) // To write
        .open("out.png").unwrap();
    let _ = file.write_all(&img.to_bytes()).unwrap();
}