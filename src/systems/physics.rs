use crate::world::World;
use crate::systems::System;

/// Physics System - manages Rapier2D physics simulation
pub struct PhysicsSystem;

impl PhysicsSystem {
    /// Create a new physics system
    pub fn new() -> Self {
        Self
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        // Step the Rapier2D physics world
        world.step_physics();
    }

    fn name(&self) -> &'static str {
        "PhysicsSystem"
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}
