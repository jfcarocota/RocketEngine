# RocketEngine 🚀

A modular 2D game engine built with Rust and powered by Rapier2D physics.

## ✨ Features

- **🏗️ ECS Architecture**: Clean Entity-Component-System design
- **⚡ Rapier2D Physics**: Professional-grade physics simulation
- **🎨 Sprite System**: Basic sprites and texture atlas support  
- **📦 Asset Loading**: PNG loading and sprite atlas management
- **🔧 Modular Design**: Well-organized, reusable components

## 🏗️ Architecture

### Components (`src/components/`)
- **`Position`**: 2D position with utility methods
- **`Velocity`**: 2D velocity with magnitude/normalization 
- **`Sprite`**: Basic colored sprites with helper constructors
- **`TextureSprite`**: Atlas-based sprites with scaling
- **`Texture`**: Raw texture data with pixel manipulation
- **`SpriteAtlas`**: Multi-sprite texture atlas management
- **`AssetsLoader`**: PNG loading and sample atlas creation

### Systems (`src/systems/`)
- **`InputSystem`**: Keyboard input handling
- **`PhysicsSystem`**: Rapier2D physics simulation 
- **`RenderSystem`**: Sprite and texture rendering
- **`Scheduler`**: System execution management

### World (`src/world.rs`)
- **ECS Management**: Entity creation and component storage
- **Physics Integration**: Rapier2D world with ECS synchronization
- **Body Mapping**: Entity ↔ RigidBody relationships

## 🚀 Quick Start

### As a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
rocket_engine = { path = "path/to/RocketEngine" }
```

Basic usage:
```rust
use rocket_engine::*;

fn main() {
    // Create world
    let mut world = World::new();
    
    // Create entity
    let player = world.create_entity();
    world.add_position(player, Position::new(100.0, 100.0));
    world.add_velocity(player, Velocity::new(50.0, 0.0));
    
    // Add physics body
    world.add_physics_body(
        player, 
        Position::new(100.0, 100.0), 
        32.0, 
        rapier2d::prelude::RigidBodyType::Dynamic
    );
    
    // Set up systems
    let mut scheduler = Scheduler::new();
    scheduler.add_system(Box::new(PhysicsSystem::new()));
    
    // Game loop
    loop {
        scheduler.update(&mut world, 0.016); // 60 FPS
        // ... rendering
    }
}
```

### Running the Demo

```bash
# Run the included demo
cargo run

# Build the library
cargo build
```

## 🎮 Demo Controls

- **Arrow Keys**: Move the player sprite
- **Escape**: Exit the game

## 🏗️ Project Structure

```
RocketEngine/
├── src/
│   ├── components/          # ECS Components
│   │   ├── mod.rs          # Module exports
│   │   ├── position.rs     # Position component
│   │   ├── velocity.rs     # Velocity component  
│   │   ├── sprite.rs       # Basic sprite component
│   │   ├── texture_sprite.rs # Atlas sprite component
│   │   ├── texture.rs      # Texture & atlas types
│   │   └── atlas.rs        # Asset loading
│   ├── systems/            # ECS Systems
│   │   ├── mod.rs          # Module exports
│   │   ├── scheduler.rs    # System scheduler
│   │   ├── input.rs        # Input handling
│   │   ├── physics.rs      # Physics simulation
│   │   └── render.rs       # Rendering
│   ├── world.rs            # ECS World + Physics
│   ├── lib.rs              # Library interface
│   └── main.rs             # Demo executable
├── assets/sprites/         # Game assets
│   └── atlas.png          # Sprite atlas
├── Cargo.toml             # Dependencies
└── README.md              # This file
```

## 🔧 Dependencies

- **`rapier2d`**: 2D physics simulation
- **`nalgebra`**: Linear algebra for physics
- **`minifb`**: Cross-platform windowing 
- **`image`**: PNG loading and processing

## 🎯 Physics Features

- ✅ **Collision Detection**: AABB and shape-based
- ✅ **Collision Response**: Realistic bouncing and separation
- ✅ **Friction & Restitution**: Configurable material properties
- ✅ **Multiple Body Types**: Static, kinematic, dynamic
- ✅ **Zero Gravity Mode**: Space-like physics (current default)
- 🔄 **Future**: Joints, constraints, forces

## 🛠️ Development

### Adding New Components

1. Create `src/components/my_component.rs`
2. Add to `src/components/mod.rs`
3. Update `World` struct in `src/world.rs`

### Adding New Systems  

1. Create `src/systems/my_system.rs`
2. Implement `System` trait
3. Add to `src/systems/mod.rs`

### Building Assets

The engine includes a sample sprite atlas generator. You can also load your own PNG files:

```rust
// Load custom atlas
let atlas = AssetsLoader::load_png("path/to/your/atlas.png")?;
world.set_sprite_atlas(atlas);
```

## 📝 License

MIT OR Apache-2.0

## 🤝 Contributing

Contributions welcome! The modular architecture makes it easy to extend with new components, systems, and features.

---

**Built with ❤️ and ⚡ Rust**