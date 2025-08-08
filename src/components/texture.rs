use std::collections::HashMap;

/// Texture data containing RGBA pixels
#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>, // RGBA pixels
}

impl Texture {
    /// Create a new texture
    pub fn new(width: usize, height: usize, data: Vec<u32>) -> Self {
        Self { width, height, data }
    }

    /// Get a pixel at the specified coordinates
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0 // Transparent/black for out of bounds
        }
    }

    /// Set a pixel at the specified coordinates
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    /// Get texture dimensions as tuple
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Get total pixel count
    pub fn pixel_count(&self) -> usize {
        self.width * self.height
    }
}

/// Atlas sprite definition containing position and size within an atlas
#[derive(Debug, Clone)]
pub struct AtlasSprite {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl AtlasSprite {
    /// Create a new atlas sprite
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }

    /// Get sprite bounds as tuple (x, y, width, height)
    pub fn bounds(&self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.width, self.height)
    }

    /// Check if a point is within this sprite's bounds
    pub fn contains_point(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

/// Sprite atlas containing multiple sprites in a single texture
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub texture: Texture,
    pub sprites: HashMap<String, AtlasSprite>,
}

impl SpriteAtlas {
    /// Create a new sprite atlas
    pub fn new(texture: Texture) -> Self {
        Self {
            texture,
            sprites: HashMap::new(),
        }
    }

    /// Add a sprite to the atlas
    pub fn add_sprite(&mut self, name: String, x: usize, y: usize, width: usize, height: usize) {
        self.sprites.insert(name, AtlasSprite::new(x, y, width, height));
    }

    /// Get a sprite by name
    pub fn get_sprite(&self, name: &str) -> Option<&AtlasSprite> {
        self.sprites.get(name)
    }

    /// List all sprite names
    pub fn sprite_names(&self) -> Vec<&String> {
        self.sprites.keys().collect()
    }

    /// Get the number of sprites in the atlas
    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }
}
