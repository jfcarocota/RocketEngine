# RocketEngine 🚀

A modular 2D game engine built with Rust and powered by Rapier2D physics.

## ✨ Features

- **🏗️ ECS Architecture**: Clean Entity-Component-System design
- **⚡ Rapier2D Physics**: Professional-grade physics simulation
- **🎨 Sprite System**: Basic sprites and texture atlas support  
- **📦 Asset Loading**: PNG loading and sprite atlas management
- **🔧 Modular Design**: Well-organized, reusable components
- **🎨 Visual Editor**: GUI-based level editor with drag & drop
- **📋 Scene System**: RON/JSON scene loading and saving

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

### Running the Applications

```bash
# Run the visual editor (recommended)
cargo run --bin editor

# Run the traditional demo
cargo run --bin rocket_engine

# Build everything
cargo build
```

## 🎮 Controls

### Traditional Demo
- **Arrow Keys**: Move the player sprite
- **Escape**: Exit the game

### Visual Editor
- **Drag & Drop**: Drag assets from panel to grid
- **Click**: Select entities
- **Play Button**: Start/stop simulation
- **File Menu**: Save/load scenes
- **View Menu**: Toggle panels and grid

See [EDITOR_README.md](EDITOR_README.md) for complete editor documentation.

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
│   ├── bin/                # Binary executables
│   │   └── editor.rs       # Visual editor
│   ├── world.rs            # ECS World + Physics
│   ├── scene.rs            # Scene loading/saving
│   ├── editor.rs           # Editor implementation
│   ├── lib.rs              # Library interface
│   └── main.rs             # Traditional demo
├── scenes/                 # Scene files
│   ├── README.md          # Scene documentation
│   ├── example_scene.ron  # Example RON scene
│   └── example_scene.json # Example JSON scene
├── assets/sprites/         # Game assets
│   └── atlas.png          # Sprite atlas
├── Cargo.toml             # Dependencies
├── README.md              # This file
├── EDITOR_README.md       # Editor documentation
└── PR_SCENE_SYSTEM.md     # Scene system PR description
```

## 🔧 Dependencies

### Core Engine
- **`rapier2d`**: 2D physics simulation
- **`nalgebra`**: Linear algebra for physics
- **`minifb`**: Cross-platform windowing 
- **`image`**: PNG loading and processing

### Scene System
- **`serde`**: Serialization framework
- **`serde_json`**: JSON support
- **`ron`**: Rust Object Notation

### Visual Editor
- **`egui`**: Immediate mode GUI
- **`eframe`**: Application framework
- **`rfd`**: File dialogs
- **`uuid`**: Unique identifiers
- **`env_logger`**: Logging

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