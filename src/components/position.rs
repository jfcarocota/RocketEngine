/// Position component for entities in 2D space
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    /// Create a new position
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Get the position as a tuple
    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Calculate distance to another position
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
