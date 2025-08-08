//! # RocketEngine
//! 
//! A modular 2D game engine built with Rust and powered by Rapier2D physics.
//! 
//! ## Features
//! 
//! - **ECS Architecture**: Entity-Component-System for clean, modular game logic
//! - **Rapier2D Physics**: Professional-grade physics simulation with collision detection
//! - **Sprite System**: Support for both basic sprites and texture atlases
//! - **Asset Loading**: PNG loading and sprite atlas management
//! - **Modular Design**: Separated components, systems, and world management
//! 
//! ## Basic Usage
//! 
//! ```rust
//! use rocket_engine::*;
//! 
//! // Create a world
//! let mut world = World::new();
//! 
//! // Create an entity with components
//! let entity = world.create_entity();
//! world.add_position(entity, Position::new(100.0, 100.0));
//! world.add_velocity(entity, Velocity::new(50.0, 0.0));
//! 
//! // Set up systems
//! let mut scheduler = Scheduler::new();
//! scheduler.add_system(Box::new(PhysicsSystem::new()));
//! 
//! // Game loop
//! // scheduler.update(&mut world, dt);
//! ```

// Re-export all public modules
pub mod components;
pub mod systems;
pub mod world;
pub mod scene;
pub mod editor;

// Re-export commonly used types for convenience
pub use components::*;
pub use systems::*;
pub use world::World;
pub use scene::*;
pub use editor::*;

// Constants
pub const DEFAULT_WIDTH: usize = 800;
pub const DEFAULT_HEIGHT: usize = 600;
