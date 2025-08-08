/// Basic sprite component with color and size
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub color: u32,    // RGB color
    pub size: usize,   // Size in pixels
}

impl Sprite {
    /// Create a new sprite
    pub fn new(color: u32, size: usize) -> Self {
        Self { color, size }
    }

    /// Create a red sprite
    pub fn red(size: usize) -> Self {
        Self::new(0xFFFF0000, size)
    }

    /// Create a green sprite
    pub fn green(size: usize) -> Self {
        Self::new(0xFF00FF00, size)
    }

    /// Create a blue sprite
    pub fn blue(size: usize) -> Self {
        Self::new(0xFF0000FF, size)
    }

    /// Create a white sprite
    pub fn white(size: usize) -> Self {
        Self::new(0xFFFFFFFF, size)
    }

    /// Create a yellow sprite
    pub fn yellow(size: usize) -> Self {
        Self::new(0xFFFFFF00, size)
    }

    /// Get ARGB components
    pub fn get_argb(&self) -> (u8, u8, u8, u8) {
        let a = ((self.color >> 24) & 0xFF) as u8;
        let r = ((self.color >> 16) & 0xFF) as u8;
        let g = ((self.color >> 8) & 0xFF) as u8;
        let b = (self.color & 0xFF) as u8;
        (a, r, g, b)
    }
}
