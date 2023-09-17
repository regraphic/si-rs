use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Rgba};
use reqwest;
use rusttype::{point, Font, Scale};
use wasm_bindgen::prelude::*;

/// Represents a font used for text rendering.
#[wasm_bindgen]
#[derive(Clone)]
pub struct SiFont {
    font: Option<Font<'static>>,
}

#[wasm_bindgen]
impl SiFont {
    /// Creates a new SiFont from a vector of font data.
    #[wasm_bindgen(constructor)]
    pub fn new(vec: Vec<u8>) -> Self {
        Self::from_vec(vec)
    }

    /// Creates a new SiFont from a vector of font data.
    #[wasm_bindgen]
    pub fn from_vec(vec: Vec<u8>) -> SiFont {
        let font = Font::try_from_vec(vec);
        SiFont { font }
    }

    /// Creates a new SiFont from font data fetched from a network URL asynchronously.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL from which to fetch the font data.
    #[wasm_bindgen]
    #[cfg(feature = "async")]
    pub async fn from_network_async(url: &str) -> Result<SiFont, JsValue> {
        let font_data: Vec<u8> = reqwest::get(url)
            .await
            .expect("Could not fetch font")
            .bytes()
            .await
            .expect("Could not extract font")
            .into();
        let font = Font::try_from_vec(font_data);
        return Ok(SiFont { font });
    }

    /// Placeholder method for when async feature is not enabled.
    #[cfg(not(feature = "async"))]
    pub fn from_network_async(_url: &str) {
        panic!("async feature not enabled")
    }

    /// Creates a new SiFont from font data fetched from a network URL synchronously.
    ///
    /// # Arguments
    ///
    /// * `src` - The URL from which to fetch the font data.
    #[cfg(feature = "blocking")]
    pub fn from_network(src: &str) -> SiFont {
        // Load font data from either URL or provided bytes.
        let font_data: Vec<u8> = reqwest::blocking::get(src)
            .expect("Could not fetch font")
            .bytes()
            .expect("Could not extract font")
            .into();
        let font = Font::try_from_vec(font_data);
        SiFont { font }
    }

    /// Placeholder method for when blocking feature is not enabled.
    #[cfg(not(feature = "blocking"))]
    pub fn from_network(url: &str) {
        panic!("blocking feature not enabled")
    }
}

/// Represents an image with text rendering capabilities.
#[wasm_bindgen]
#[derive(Clone)]
pub struct SiImage {
    font: SiFont,
    image: DynamicImage,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl SiImage {
    /// Creates a new SiImage from a vector of image data and a SiFont.
    ///
    /// # Arguments
    ///
    /// * `src` - The vector of image data.
    /// * `font` - The SiFont used for text rendering on the image.
    #[wasm_bindgen(constructor)]
    pub fn new(src: Vec<u8>, font: SiFont) -> Self {
        Self::from_vec(src, font)
    }

    /// Creates a new SiImage from a vector of image data.
    #[wasm_bindgen]
    pub fn from_vec(vec: Vec<u8>, font: SiFont) -> SiImage {
        let image = image::load_from_memory(&vec).expect("Could not decode image");
        let (width, height) = image.dimensions();
        SiImage { image, height, width, font }
    }

    /// Creates a new SiImage from image data fetched from a network URL asynchronously.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL from which to fetch the image data.
    /// * `font` - The SiFont used for text rendering on the image.
    #[wasm_bindgen]
    #[cfg(feature = "async")]
    pub async fn from_network_async(image_url: &str, font: SiFont) -> SiImage {
        let image_data: Vec<u8> = reqwest::get(image_url)
                .await
                .expect("Could not fetch image")
                .bytes()
                .await
                .expect("Could not extract image")
                .into();
        let image = image::load_from_memory(&image_data).expect("Could not decode image");
        let (width, height) = image.dimensions();
        Self {
            font,
            image,
            height,
            width,
        }
    }

    /// Placeholder method for when async feature is not enabled.
    #[cfg(not(feature = "async"))]
    pub fn from_network_async(_image_url: &str, _font: SiFont) {
        panic!("async feature not enabled")
    }

    /// Creates a new SiImage from image data fetched from a network URL synchronously.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL from which to fetch the image data.
    /// * `font` - The SiFont used for text rendering on the image.
    #[cfg(feature = "blocking")]
    pub fn from_network(image_url: &str, font: SiFont) -> SiImage {
        // Load image data from either URL or provided bytes.
        let image_data: Vec<u8> = reqwest::blocking::get(image_url)
            .expect("Could not fetch image")
            .bytes()
            .expect("Could not extract image")
            .into();

        let image = image::load_from_memory(&image_data).expect("Could not decode image");
        let (width, height) = image.dimensions();

        Self {
            font,
            image,
            height,
            width,
        }
    }

    /// Placeholder method for when blocking feature is not enabled.
    #[cfg(not(feature = "blocking"))]
    pub fn from_network(image_url: &str, font: SiFont) {
        panic!("blocking feature not enabled")
    }

    /// Renders text onto the image.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to render on the image.
    /// * `text_scale` - The scale of the rendered text.
    /// * `pos_x` - The X-coordinate position for rendering.
    /// * `pos_y` - The Y-coordinate position for rendering.
    /// * `color` - The color of the rendered text in hexadecimal format (e.g., "#RRGGBB").
    #[wasm_bindgen]
    pub fn text(
        &mut self,
        text: &str,
        text_scale: f32,
        pos_x: f32,
        pos_y: f32,
        color: Option<String>,
    ) -> SiImage {
        let mut image = self.image.clone();
        let font = self
            .font
            .font
            .as_ref()
            .ok_or("Error loading font")
            .expect("Could not decode/load font");
        let scale = Scale::uniform(text_scale);
        let v_metrics = font.v_metrics(scale);
        let offset = point(pos_x, pos_y + v_metrics.ascent);

        let parsed_color = match color.clone() {
            Some(c) => hex_to_rgb(&c).unwrap_or(Rgb([0, 0, 0])),
            None => Rgb([0, 0, 0]),
        };

        for glyph in font.layout(text, scale, offset) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let new_pixel = Rgba([
                        (((parsed_color[0] as f32 * (v)) as f32) + (pixel[0] as f32 * (1.0 - v)))
                            as u8,
                        ((parsed_color[1] as f32 * (v)) as f32 + (pixel[1] as f32 * (1.0 - v)))
                            as u8,
                        ((parsed_color[2] as f32 * (v)) as f32 + (pixel[2] as f32 * (1.0 - v)))
                            as u8,
                        (pixel[3] as f32 * (v)) as u8,
                    ]);
                    image.put_pixel(x as u32, y as u32, new_pixel);
                });
            }
        }

        self.image = image;

        self.clone()
    }

    /// Gets the image data as bytes in PNG format.
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = std::io::Cursor::new(Vec::new());
        self.image
            .write_to(&mut v, image::ImageFormat::Png)
            .expect("Could not write bytes");
        v.into_inner()
    }

    /// Sets the font for text rendering on the image.
    ///
    /// # Arguments
    ///
    /// * `font` - The SiFont to set as the font.
    #[wasm_bindgen(setter)]
    pub fn set_font(&mut self, font: SiFont) -> SiImage {
        self.font = font;
        self.clone()
    }

    /// Gets the height of the image.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Gets the width of the image.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
}

/// Converts a hexadecimal color code to an RGB color.
///
/// # Arguments
///
/// * `hex` - The hexadecimal color code (e.g., "#RRGGBB").
///
/// # Returns
///
/// An `Option` containing the RGB color as `Rgb<u8>`, or `None` if the conversion fails.
pub fn hex_to_rgb(hex: &str) -> Option<Rgb<u8>> {
    let hex = hex.trim_start_matches('#'); // Remove "#" if present
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Rgb([r, g, b]))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        Some(Rgb([r, g, b]))
    } else {
        Some(Rgb([0, 0, 0]))
    }
}
