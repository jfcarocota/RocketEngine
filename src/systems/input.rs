use crate::components::Entity;
use crate::world::World;
use crate::systems::System;
use minifb::{Key, Window};

/// Input System - handles keyboard input
pub struct InputSystem {
    pub player_entity: Entity,
}

impl InputSystem {
    /// Create a new input system
    pub fn new(player_entity: Entity) -> Self {
        Self { player_entity }
    }

    /// Set input state (kept for compatibility)
    pub fn set_input(&mut self, _window: &Window) {
        // Input is now handled in the main loop directly
        // This method is kept for interface compatibility
    }

    /// Get the player entity
    pub fn get_player_entity(&self) -> Entity {
        self.player_entity
    }

    /// Set a new player entity
    pub fn set_player_entity(&mut self, entity: Entity) {
        self.player_entity = entity;
    }
}

impl System for InputSystem {
    fn update(&mut self, _world: &mut World, _dt: f32) {
        // Input is handled separately since it needs window reference
        // The main loop handles input directly to avoid borrowing issues
    }

    fn name(&self) -> &'static str {
        "InputSystem"
    }
}
