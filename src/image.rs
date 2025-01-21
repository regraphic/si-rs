use ab_glyph::{Font, ScaleFont};
use image::{
    imageops::{overlay, resize},
    DynamicImage, GenericImage, GenericImageView, Rgb, Rgba,
};
use wasm_bindgen::prelude::*;

use crate::font::*;
use crate::preset::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct TextOptions {
    pub letter_spacing: f32,
    pub space_width: f32,
}

#[wasm_bindgen]
impl TextOptions {
    #[wasm_bindgen]
    pub fn default() -> Self {
        TextOptions {
            letter_spacing: 2.0,
            space_width: 10.0,
        }
    }
}

/// Represents an image with text rendering capabilities.
#[wasm_bindgen]
#[derive(Clone)]
pub struct SiImage {
    image: DynamicImage,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl SiImage {
    /// Creates a new SiImage from a vector of image data.
    ///
    /// # Arguments
    ///
    /// * `src` - The vector of image data.
    #[wasm_bindgen(constructor)]
    pub fn new(src: Vec<u8>) -> Self {
        Self::from_vec(src)
    }

    /// Creates a new SiImage from a vector of image data.
    #[wasm_bindgen]
    pub fn from_vec(vec: Vec<u8>) -> SiImage {
        let image = image::load_from_memory(&vec).expect("Could not decode image");
        let (width, height) = image.dimensions();
        SiImage {
            image,
            height,
            width,
        }
    }

    /// Creates a new SiImage from image data fetched from a network URL asynchronously.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL from which to fetch the image data.
    #[wasm_bindgen]
    #[cfg(feature = "async")]
    pub async fn from_network_async(image_url: &str) -> SiImage {
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
            image,
            height,
            width,
        }
    }

    /// Placeholder method for when async feature is not enabled.
    #[cfg(not(feature = "async"))]
    pub fn from_network_async(_image_url: &str) {
        panic!("async feature not enabled")
    }

    /// Creates a new SiImage from image data fetched from a network URL synchronously.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL from which to fetch the image data.
    #[cfg(feature = "blocking")]
    pub fn from_network(image_url: &str) -> SiImage {
        // Load image data from either URL or provided bytes.
        let image_data: Vec<u8> = reqwest::blocking::get(image_url)
            .expect("Could not fetch image")
            .bytes()
            .expect("Could not extract image")
            .into();

        let image = image::load_from_memory(&image_data).expect("Could not decode image");
        let (width, height) = image.dimensions();

        Self {
            image,
            height,
            width,
        }
    }

    /// Placeholder method for when blocking feature is not enabled.
    #[cfg(not(feature = "blocking"))]
    pub fn from_network(image_url: &str) {
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
    /// * `using_font` - The SiFont used for text rendering on the image.
    ///
    /// # Returns
    ///
    /// A mutable instance of the main image, with the text rendered on it.
    #[wasm_bindgen(js_name = "text")]
    pub fn render_text(
        self,
        text: &str,
        text_scale: f32,
        pos_x: f32,
        pos_y: f32,
        color: Option<String>,
        using_font: &SiFont,
        options: &TextOptions,
    ) -> SiImage {
        let mut image = self.image.clone(); // Clone the image explicitly for the method
        let font = &using_font.font.as_scaled(text_scale);
        let ascent = font.ascent();

        let parsed_color = match color.as_ref() {
            Some(c) => hex_to_rgb(c).unwrap_or_else(|| {
                // Log an error, if necessary
                eprintln!("Invalid color hex: {}", c);
                Rgb([0, 0, 0])
            }),
            None => Rgb([0, 0, 0]),
        };

        for glyph in &using_font.layout(text, text_scale, (pos_x, pos_y + ascent), options) {
            let bb = glyph.px_bounds();
            glyph.draw(|_x, _y, v| {
                let x = _x + bb.min.x as u32;
                let y = _y + bb.min.y as u32;
                if x < image.width() && y < image.height() {
                    let pixel = image.get_pixel(x, y);
                    let new_pixel = blend_pixel(&pixel, parsed_color, v);
                    image.put_pixel(x, y, new_pixel);
                }
            });
        }

        SiImage { image, height: self.height, width: self.width }
    }

    /// Renders some image into the image
    ///
    /// # Arguments
    ///
    /// * `image` - The SiImage to render.
    /// * `pos_x` - The X-coordinate position for rendering.
    /// * `pos_y` - The Y-coordinate position for rendering.
    ///
    /// # Returns
    ///
    /// A mutable instance of the main image, with overlay of the provided one
    #[wasm_bindgen(js_name = "image")]
    pub fn render_image(mut self, image: &SiImage, pos_x: i64, pos_y: i64) -> SiImage {
        overlay(&mut self.image, &image.image, pos_x, pos_y);
        self
    }

    /// Gets the image data as bytes in PNG format.
    ///
    /// # Returns
    ///
    /// The image data as bytes in PNG format
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = std::io::Cursor::new(Vec::new());
        self.clone()
            .image
            .write_to(&mut v, image::ImageFormat::Png)
            .expect("Could not write bytes");
        v.into_inner()
    }

    /// Gets the height of the image.
    ///
    /// # Returns
    ///
    /// The height of the image
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.clone().height
    }

    /// Gets the width of the image.
    ///
    /// # Returns
    ///
    /// The width of the image
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.clone().width
    }

    /// Resizes the image
    ///
    /// # Arguments
    ///
    /// * `width` - The new width of the image
    /// * `height` - The new height of the image
    ///
    /// # Returns
    ///
    /// A mutable instance of the main image, with the resized image
    #[wasm_bindgen]
    pub fn resize(mut self, width: u32, height: u32) -> SiImage {
        let new_image = DynamicImage::ImageRgba8(resize(
            &self.image,
            width,
            height,
            image::imageops::FilterType::Triangle,
        ));
        let _ = std::mem::replace(&mut self.image, new_image);
        let _ = std::mem::replace(&mut self.width, width);
        let _ = std::mem::replace(&mut self.height, height);
        self
    }
}

impl SiImage {
    /// Load a preset.
    /// **NOTE**: It doesn't work in WASM. Only for direct usage.
    pub fn load_preset(
        &mut self,
        preset: Box<SiPreset>,
        values: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    ) -> &mut SiImage {
        let res = (preset.cb)(self, values);
        let _ = std::mem::replace(self, res);
        self
    }
}

/// Helper function to blend colors
fn blend_pixel(base_pixel: &Rgba<u8>, color: Rgb<u8>, alpha: f32) -> Rgba<u8> {
    Rgba([
        blend_channel(base_pixel[0], color[0], alpha),
        blend_channel(base_pixel[1], color[1], alpha),
        blend_channel(base_pixel[2], color[2], alpha),
        255,
    ])
}

fn blend_channel(base: u8, overlay: u8, alpha: f32) -> u8 {
    ((overlay as f32 * alpha) + (base as f32 * (1.0 - alpha))) as u8
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
        Some(Rgb([255, 255, 255]))
    }
}
