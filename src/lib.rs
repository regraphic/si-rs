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
    /// Constructor for SiFont, asynchronously loading font data from a URL or using provided bytes.
    ///
    /// # Arguments
    ///
    /// * `src` - The URL of the font file.
    /// * `src_bytes` - Optional bytes of the font file.
    ///
    /// # Returns
    ///
    /// A `SiFont` instance containing the loaded font.
    #[wasm_bindgen(constructor)]
    #[cfg(feature = "async")]
    pub async fn new(src: &str, src_bytes: Option<Vec<u8>>) -> Result<SiFont, JsValue> {
        // Load font data from either URL or provided bytes.
        let font_data = match src_bytes {
            Some(bytes) => bytes.to_vec(),
            None => reqwest::get(src)
                .await
                .expect("Could not fetch font")
                .bytes()
                .await
                .expect("Could not extract font")
                .to_vec(),
        };
        let font = Font::try_from_vec(font_data);
        Ok(SiFont { font })
    }

    /// Constructor for SiFont, synchronously loading font data from a URL or using provided bytes.
    ///
    /// # Arguments
    ///
    /// * `src` - The URL of the font file.
    /// * `src_bytes` - Optional bytes of the font file.
    ///
    /// # Returns
    ///
    /// A `SiFont` instance containing the loaded font.
    #[cfg(feature = "blocking")]
    pub fn new(src: &str, src_bytes: Option<Vec<u8>>) -> SiFont {
        // Load font data from either URL or provided bytes.
        let font_data = match src_bytes {
            Some(bytes) => bytes.to_vec(),
            None => reqwest::blocking::get(src)
                .expect("Could not fetch font")
                .bytes()
                .expect("Could not extract font")
                .to_vec(),
        };
        let font = Font::try_from_vec(font_data.to_vec());
        SiFont { font }
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
    /// Constructor for SiImage, asynchronously loading an image from a URL or using provided bytes.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL of the image file.
    /// * `font` - A `SiFont` instance for text rendering.
    /// * `image_bytes` - Optional bytes of the image file.
    ///
    /// # Returns
    ///
    /// A `SiImage` instance containing the loaded image and font.
    #[wasm_bindgen(constructor)]
    #[cfg(feature = "async")]
    pub async fn new(image_url: &str, font: SiFont, image_bytes: Option<Vec<u8>>) -> SiImage {
        // Load image data from either URL or provided bytes.
        let image_data = match image_bytes {
            Some(bytes) => bytes.to_vec(),
            None => reqwest::get(image_url)
                .await
                .expect("Could not fetch image")
                .bytes()
                .await
                .expect("Could not extract image")
                .to_vec(),
        };
        let image = image::load_from_memory(&image_data).expect("Could not decode image");
        let d = image.clone().dimensions();
        Self {
            font,
            image,
            height: d.1,
            width: d.0,
        }
    }

    /// Constructor for SiImage, synchronously loading an image from a URL or using provided bytes.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL of the image file.
    /// * `font` - A `SiFont` instance for text rendering.
    /// * `image_bytes` - Optional bytes of the image file.
    ///
    /// # Returns
    ///
    /// A `SiImage` instance containing the loaded image and font.
    #[cfg(feature = "blocking")]
    pub fn new(image_url: &str, font: SiFont, image_bytes: Option<Vec<u8>>) -> SiImage {
        // Load image data from either URL or provided bytes.
        let image_data = match image_bytes {
            Some(bytes) => bytes.to_vec(),
            None => reqwest::blocking::get(image_url)
                .expect("Could not fetch image")
                .bytes()
                .expect("Could not extract image")
                .to_vec(),
        };
        let image = image::load_from_memory(&image_data).expect("Could not decode image");
        let d = image.clone().dimensions();
        Self {
            font,
            image,
            height: d.1,
            width: d.0,
        }
    }

    /// Render text onto the image.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to render.
    /// * `text_scale` - The scale factor for the text.
    /// * `pos_x` - The X-coordinate for text placement.
    /// * `pos_y` - The Y-coordinate for text placement.
    /// * `color` - Optional text color in hexadecimal format (e.g., "#RRGGBB").
    ///
    /// # Returns
    ///
    /// A new `SiImage` instance with the rendered text.
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

        for glyph in font.layout(text, scale, offset) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let parsed_color = match color.clone() {
                        Some(c) => hex_to_rgb(&c).unwrap_or(Rgb([0, 0, 0])),
                        None => Rgb([0, 0, 0]),
                    };
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

        self.image = image.clone();

        SiImage {
            font: self.font.clone(),
            image,
            height: self.height,
            width: self.width,
        }
    }

    /// Get the image data as bytes in PNG format.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the image data.
    #[wasm_bindgen(getter)]
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut v = std::io::Cursor::new(Vec::new());
        self.image
            .write_to(&mut v, image::ImageFormat::Png)
            .expect("Could not write bytes");
        v.into_inner()
    }

    /// Set the font for text rendering.
    ///
    /// # Arguments
    ///
    /// * `font` - A `SiFont` instance for text rendering.
    ///
    /// # Returns
    ///
    /// A new `SiImage` instance with the updated font.
    #[wasm_bindgen(setter, js_name = "font")]
    pub fn font(&mut self, font: SiFont) -> SiImage {
        self.font = font;
        SiImage {
            font: self.font.clone(),
            image: self.image.clone(),
            height: self.height,
            width: self.width,
        }
    }

    /// Get the height of the image.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the width of the image.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
}

/// Converts a hexadecimal color string (e.g., "#RRGGBB") to an `Rgb<u8>` color.
///
/// # Arguments
///
/// * `hex` - The hexadecimal color string.
///
/// # Returns
///
/// An `Option<Rgb<u8>>` representing the RGB color.
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