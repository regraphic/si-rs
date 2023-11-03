use crate::image::*;

/// Represents a Preset
pub struct SiPreset {
    pub cb: Box<dyn Fn(&mut SiImage, std::collections::HashMap<String, Box<dyn std::any::Any>>) -> SiImage>,
}

impl SiPreset {
    pub fn new(cb: Box<dyn Fn(&mut SiImage, std::collections::HashMap<String, Box<dyn std::any::Any>>) -> SiImage>) -> Box<SiPreset> {
        Box::new(SiPreset { cb: Box::new(cb) })
    }
}