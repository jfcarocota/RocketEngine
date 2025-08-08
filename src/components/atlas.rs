use super::texture::{Texture, SpriteAtlas};

/// Assets loader for managing textures and sprite atlases
pub struct AssetsLoader;

impl AssetsLoader {
    /// Load a PNG file and convert it to a Texture
    pub fn load_png(path: &str) -> Result<Texture, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        
        // Convert RGBA to ARGB format for minifb
        let mut data = Vec::with_capacity((width * height) as usize);
        for pixel in rgba_img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;
            
            // Convert to ARGB format (minifb expects this format)
            let argb = (a << 24) | (r << 16) | (g << 8) | b;
            data.push(argb);
        }
        
        Ok(Texture::new(width as usize, height as usize, data))
    }
    
    /// Create a sample sprite atlas for testing/demo purposes
    pub fn create_sample_atlas() -> SpriteAtlas {
        // Create a simple 128x128 texture with colored squares
        let width = 128;
        let height = 128;
        let mut data = vec![0xFF000000; width * height]; // Black background with full alpha
        
        // Red square (player) at (0,0) 32x32
        for y in 0..32 {
            for x in 0..32 {
                data[y * width + x] = 0xFFFF0000; // Red
            }
        }
        
        // Green square (enemy1) at (32,0) 32x32
        for y in 0..32 {
            for x in 32..64 {
                data[y * width + x] = 0xFF00FF00; // Green
            }
        }
        
        // Blue square (enemy2) at (64,0) 32x32
        for y in 0..32 {
            for x in 64..96 {
                data[y * width + x] = 0xFF0000FF; // Blue
            }
        }
        
        // Yellow square (powerup) at (96,0) 32x32
        for y in 0..32 {
            for x in 96..128 {
                data[y * width + x] = 0xFFFFFF00; // Yellow
            }
        }
        
        let texture = Texture::new(width, height, data);
        let mut atlas = SpriteAtlas::new(texture);
        
        atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
        atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
        atlas.add_sprite("enemy2".to_string(), 64, 0, 32, 32);
        atlas.add_sprite("powerup".to_string(), 96, 0, 32, 32);
        
        atlas
    }

    /// Load an atlas from a PNG file with predefined sprite layout
    pub fn load_atlas(path: &str) -> Result<SpriteAtlas, Box<dyn std::error::Error>> {
        let texture = Self::load_png(path)?;
        let mut atlas = SpriteAtlas::new(texture);
        
        // Add default sprite layout (assumes 32x32 sprites in a grid)
        atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
        atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
        atlas.add_sprite("enemy2".to_string(), 64, 0, 32, 32);
        atlas.add_sprite("powerup".to_string(), 96, 0, 32, 32);
        
        Ok(atlas)
    }
}
