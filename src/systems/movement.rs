use crate::world::World;
use crate::systems::{QuerySystem, Query};
use crate::systems::query::MovementQuery;
use crate::query;

/// Movement System - updates positions based on velocities using queries
pub struct MovementSystem;

impl MovementSystem {
    /// Create a new movement system
    pub fn new() -> Self {
        Self
    }
}

impl QuerySystem for MovementSystem {
    fn update_with_queries(&mut self, world: &mut World, dt: f32) {
        // Example 1: Query for entities with both Position and Velocity
        // This demonstrates safe, borrow-checked access to multiple components
        let mut updates = Vec::new();
        
        // First pass: collect position updates (immutable borrow)
        for (entity, position, velocity) in MovementQuery::query(world) {
            let new_x = position.x + velocity.x * dt;
            let new_y = position.y + velocity.y * dt;
            updates.push((entity, new_x, new_y));
        }
        
        // Second pass: apply updates (mutable borrow)
        for (entity, new_x, new_y) in updates {
            if let Some(position) = world.positions.get_mut(&entity) {
                position.x = new_x;
                position.y = new_y;
            }
        }
        
        // Example 2: Using the query macro for cleaner syntax
        // Count how many entities have positions
        let position_count = query!(world, Position).count();
        if position_count > 0 {
            // println!("Updated {} entities with positions", position_count);
        }
    }
    
    fn name(&self) -> &'static str {
        "MovementSystem"
    }
}

impl Default for MovementSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Example usage and demonstration system
pub struct QueryDemoSystem;

impl QueryDemoSystem {
    pub fn new() -> Self {
        Self
    }
}

impl QuerySystem for QueryDemoSystem {
    fn update_with_queries(&mut self, world: &mut World, _dt: f32) {
        // Demonstrate different query patterns
        
        // 1. Query single component type
        let position_count = query!(world, Position).count();
        let _velocity_count = query!(world, Velocity).count();
        
        // 2. Query multiple components
        let _movement_entities: Vec<_> = query!(world, Position, Velocity)
            .map(|(entity, _pos, _vel)| entity)
            .collect();
        
        // 3. Query with filtering (entities that have all three components)
        let _renderable_count = query!(world, Position, Velocity, Sprite).count();
        
        // Print statistics (commented out to avoid spam)
        if position_count > 0 {
            // println!("Query Demo - Positions: {}, Velocities: {}, Movement: {}, Renderable: {}", 
            //          position_count, velocity_count, movement_entities.len(), renderable_count);
        }
    }
    
    fn name(&self) -> &'static str {
        "QueryDemoSystem"
    }
}

impl Default for QueryDemoSystem {
    fn default() -> Self {
        Self::new()
    }
}
