use crate::world::World;
use crate::systems::System;

/// Optional system to push ECS velocity into physics bodies each frame
/// so bodies move even without external forces/input.
pub struct VelocitySyncSystem;

impl VelocitySyncSystem {
    pub fn new() -> Self { Self }
}

impl System for VelocitySyncSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        for (&entity, velocity) in &world.velocities {
            if let Some(&body_handle) = world.entity_to_body.get(&entity) {
                if let Some(body) = world.physics_world.get_mut(body_handle) {
                    body.set_linvel(nalgebra::Vector2::new(velocity.x, velocity.y), true);
                }
            }
        }
    }

    fn name(&self) -> &'static str { "VelocitySyncSystem" }
}

impl Default for VelocitySyncSystem { fn default() -> Self { Self::new() } }


