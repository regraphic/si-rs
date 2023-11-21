use si_img::SiFont;

#[allow(unused)]
fn main() {
    // From URL (synchronous)
    // For Async example, see examples/async/font.rs
    let font_from_url: SiFont = SiFont::from_network(
        "https://github.com/Zype-Z/ShareImage.js/raw/main/assets/fonts/sirin-stencil.ttf",
    );
    // From Vec
    let font_from_url: SiFont = SiFont::from_vec(Vec::new());
}
