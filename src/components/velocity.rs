/// Velocity component for entities in 2D space
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Create a new velocity
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a zero velocity
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Get the velocity as a tuple
    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Calculate the magnitude (speed) of the velocity
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalize the velocity to unit length
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Self::zero()
        }
    }
}
