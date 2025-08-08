/// Texture sprite component that references sprites from an atlas
#[derive(Debug, Clone)]
pub struct TextureSprite {
    pub atlas_name: String,   // Which sprite in the atlas to use
    pub scale: f32,          // Scale factor (1.0 = original size)
}

impl TextureSprite {
    /// Create a new texture sprite
    pub fn new(atlas_name: String, scale: f32) -> Self {
        Self { atlas_name, scale }
    }

    /// Create a texture sprite with default scale (1.0)
    pub fn with_name(atlas_name: &str) -> Self {
        Self::new(atlas_name.to_string(), 1.0)
    }

    /// Create a texture sprite with custom scale
    pub fn with_scale(atlas_name: &str, scale: f32) -> Self {
        Self::new(atlas_name.to_string(), scale)
    }

    /// Get the scaled dimensions if we know the original size
    pub fn get_scaled_size(&self, original_width: usize, original_height: usize) -> (usize, usize) {
        (
            (original_width as f32 * self.scale) as usize,
            (original_height as f32 * self.scale) as usize,
        )
    }
}
