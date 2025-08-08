// Systems module exports
pub mod input;
pub mod physics;
pub mod render;
pub mod scheduler;
pub mod query;
pub mod movement;

// Re-export all systems
pub use input::InputSystem;
pub use physics::PhysicsSystem;
pub use render::RenderSystem;
pub use movement::{MovementSystem, QueryDemoSystem};
pub use scheduler::{System, Scheduler, QuerySystem, QuerySystemAdapter};
pub use query::*;
