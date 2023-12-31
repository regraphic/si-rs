use crate::TextOptions;
use ab_glyph::{Font, FontVec, OutlinedGlyph};
use reqwest;
use wasm_bindgen::prelude::*;

/// Represents a font used for text rendering.
#[wasm_bindgen]
pub struct SiFont {
    pub(crate) font: FontVec,
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
        let font = FontVec::try_from_vec(vec).unwrap();
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
        let font = FontVec::try_from_vec(font_data).unwrap();
        Ok(SiFont { font })
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
        let font = FontVec::try_from_vec(font_data).unwrap();
        SiFont { font }
    }

    /// Placeholder method for when blocking feature is not enabled.
    #[cfg(not(feature = "blocking"))]
    pub fn from_network(url: &str) {
        panic!("blocking feature not enabled")
    }

    pub(crate) fn layout(
        &self,
        text: &str,
        scale: f32,
        position: Position,
        options: &TextOptions,
    ) -> Vec<OutlinedGlyph> {
        let mut res: Vec<OutlinedGlyph> = Vec::new();
        let mut tmp_x: f32 = position.0;
        for char in text.chars() {
            if char.is_whitespace() {
                tmp_x += options.space_width;
            }
            if let Some(glyph) = self.font.outline_glyph(
                self.font
                    .glyph_id(char)
                    .with_scale_and_position(scale, ab_glyph::point(tmp_x, position.1)),
            ) {
                let bb = glyph.px_bounds();
                res.push(glyph);
                tmp_x += bb.width() + options.letter_spacing;
                // tmp_y += bb.height();
            }
        }
        res
    }
}

pub type Position = (f32, f32);
