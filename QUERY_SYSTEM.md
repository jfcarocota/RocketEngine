# 🔍 Query System Documentation

## Overview

RocketEngine now includes a powerful **Query System** that provides borrow-checked access to specific components with safe, ergonomic APIs. This system allows you to write systems that operate on entities with specific component combinations while ensuring memory safety through Rust's borrow checker.

## 🚀 Key Features

- ✅ **Safe Component Access**: Borrow-checked access prevents data races
- ✅ **Ergonomic Macros**: Clean, readable query syntax
- ✅ **Multiple Component Queries**: Query entities with specific component combinations
- ✅ **Performance Optimized**: Iterator-based design for efficient iteration
- ✅ **Backward Compatible**: Works alongside existing System trait

## 📚 Core Concepts

### Query Trait

The `Query` trait provides immutable access to components:

```rust
pub trait Query<'world> {
    type Item;
    fn query(world: &'world World) -> Self::Item;
}
```

### QueryMut Trait

The `QueryMut` trait provides mutable access to components:

```rust
pub trait QueryMut<'world> {
    type Item;
    fn query_mut(world: &'world mut World) -> Self::Item;
}
```

### QuerySystem Trait

The `QuerySystem` trait is an alternative to the traditional `System` trait:

```rust
pub trait QuerySystem {
    fn update_with_queries(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &'static str;
}
```

## 🎯 Usage Examples

### 1. Single Component Queries

```rust
// Query all entities with Position component
for (entity, position) in query!(world, Position) {
    println!("Entity {} at ({}, {})", entity, position.x, position.y);
}

// Mutable access to positions
for (entity, position) in query_mut!(world, Position) {
    position.x += 10.0;
}
```

### 2. Multiple Component Queries

```rust
// Query entities with both Position and Velocity
for (entity, position, velocity) in query!(world, Position, Velocity) {
    println!("Entity {} moving at ({}, {})", entity, velocity.x, velocity.y);
}

// Query entities with Position, Velocity, and Sprite
for (entity, position, velocity, sprite) in query!(world, Position, Velocity, Sprite) {
    // Process renderable moving entities
}
```

### 3. Creating Query-Based Systems

```rust
use crate::systems::{QuerySystem, Query};
use crate::query;

pub struct MovementSystem;

impl QuerySystem for MovementSystem {
    fn update_with_queries(&mut self, world: &mut World, dt: f32) {
        // Collect updates first (immutable borrow)
        let mut updates = Vec::new();
        for (entity, position, velocity) in query!(world, Position, Velocity) {
            let new_x = position.x + velocity.x * dt;
            let new_y = position.y + velocity.y * dt;
            updates.push((entity, new_x, new_y));
        }
        
        // Apply updates (mutable borrow)
        for (entity, new_x, new_y) in updates {
            if let Some(position) = world.positions.get_mut(&entity) {
                position.x = new_x;
                position.y = new_y;
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "MovementSystem"
    }
}
```

### 4. Adding Systems to Scheduler

```rust
let mut scheduler = Scheduler::new();

// Traditional systems
scheduler.add_system(Box::new(PhysicsSystem::new()));

// Query-based systems
scheduler.add_query_system(MovementSystem::new());
scheduler.add_query_system(QueryDemoSystem::new());
```

## 🔧 Available Queries

### Single Component Queries
- `query!(world, Position)` - All entities with Position
- `query!(world, Velocity)` - All entities with Velocity  
- `query!(world, Sprite)` - All entities with Sprite
- `query_mut!(world, Position)` - Mutable access to positions

### Multi-Component Queries
- `query!(world, Position, Velocity)` - Entities with both Position and Velocity
- `query!(world, Position, Velocity, Sprite)` - Entities with all three components

## 📊 Query Types

### PositionQuery
```rust
for (entity, position) in PositionQuery::query(world) {
    // Access entity positions
}
```

### VelocityQuery
```rust
for (entity, velocity) in VelocityQuery::query(world) {
    // Access entity velocities
}
```

### MovementQuery
```rust
for (entity, position, velocity) in MovementQuery::query(world) {
    // Access entities that can move
}
```

### RenderableQuery
```rust
for (entity, position, velocity, sprite) in RenderableQuery::query(world) {
    // Access entities that can be rendered and move
}
```

## 🛡️ Safety Features

### Borrow Checking
The query system enforces Rust's borrowing rules:
- Multiple immutable borrows are allowed simultaneously
- Only one mutable borrow is allowed at a time
- No simultaneous mutable and immutable borrows

### Safe Patterns
```rust
// ✅ Safe: Immutable queries don't conflict
let positions = query!(world, Position);
let velocities = query!(world, Velocity);

// ✅ Safe: Sequential mutable access
for (entity, mut pos) in query_mut!(world, Position) {
    pos.x += 1.0;
}

// ❌ Unsafe: Would cause compile error
// let mut positions = query_mut!(world, Position);
// let velocities = query!(world, Velocity); // Borrow conflict!
```

## 🚀 Performance Benefits

1. **Iterator-Based**: Efficient iteration over components
2. **Zero-Cost Abstractions**: Compiles to optimal code
3. **Cache-Friendly**: Sequential access patterns
4. **Minimal Allocations**: Iterator chains avoid temporary collections

## 🔄 Migration Guide

### From Direct World Access
```rust
// Old way
impl System for MySystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (entity, position) in &world.positions {
            // Process...
        }
    }
}

// New way
impl QuerySystem for MySystem {
    fn update_with_queries(&mut self, world: &mut World, dt: f32) {
        for (entity, position) in query!(world, Position) {
            // Process...
        }
    }
}
```

## 🎉 Example: Complete Query-Based System

```rust
use crate::systems::{QuerySystem, Query};
use crate::{query, query_mut};

pub struct HealthSystem {
    damage_per_second: f32,
}

impl HealthSystem {
    pub fn new() -> Self {
        Self { damage_per_second: 10.0 }
    }
}

impl QuerySystem for HealthSystem {
    fn update_with_queries(&mut self, world: &mut World, dt: f32) {
        // Find entities that need health updates
        let mut entities_to_damage = Vec::new();
        
        for (entity, position) in query!(world, Position) {
            // Example: damage entities in a danger zone
            if position.x < 50.0 {
                entities_to_damage.push(entity);
            }
        }
        
        // Apply damage (this would require a Health component)
        for entity in entities_to_damage {
            println!("Entity {} takes damage!", entity);
        }
        
        // Count statistics
        let total_entities = query!(world, Position).count();
        let moving_entities = query!(world, Position, Velocity).count();
        
        println!("Total: {}, Moving: {}", total_entities, moving_entities);
    }
    
    fn name(&self) -> &'static str {
        "HealthSystem"
    }
}
```

## 🛠️ Advanced Usage

### Custom Query Types

You can create your own query types by implementing the `Query` trait:

```rust
pub struct CustomQuery;

impl<'world> Query<'world> for CustomQuery {
    type Item = impl Iterator<Item = (Entity, &'world Position, &'world Sprite)>;
    
    fn query(world: &'world World) -> Self::Item {
        // Custom query logic
        world.positions.iter()
            .filter_map(|(&entity, position)| {
                world.sprites.get(&entity)
                    .map(|sprite| (entity, position, sprite))
            })
    }
}
```

This query system provides a foundation for building more complex ECS patterns while maintaining safety and performance!
