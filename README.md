# RocketEngine 🚀

A modern, high-performance 2D game engine built in Rust with Entity Component System (ECS) architecture.

## Features

### 🏗️ **Entity Component System (ECS)**
- Clean separation of data (Components) and logic (Systems)
- Type-safe Entity management with unique IDs
- Flexible component composition

### 📅 **System Scheduler**
- Automated system execution with proper ordering
- Delta-time based physics updates
- Separation between update systems and render system

### 🎨 **Advanced Asset Management**
- PNG texture loading with `image` crate
- Sprite atlas system for efficient texture memory usage
- Texture component with scaling support
- Fallback to procedural sprites when assets unavailable

### 🎮 **Game Components**
- **Position**: 2D coordinates (x, y)
- **Velocity**: Movement speed per second
- **Sprite**: Basic colored squares with customizable size
- **TextureSprite**: Atlas-based sprites with scaling

### ⚙️ **Systems**
1. **InputSystem**: Keyboard input handling (Arrow keys)
2. **PhysicsSystem**: Position updates with velocity and boundary collision
3. **RenderSystem**: Efficient sprite rendering with texture support

## Quick Start

```bash
# Clone the repository
git clone https://github.com/jfcarocota/RocketEngine.git
cd RocketEngine

# Run the engine
cargo run
```

## Controls

- **Arrow Keys**: Move the player sprite
- **Escape**: Exit the game

## Asset Loading

The engine attempts to load textures from `assets/sprites/atlas.png`. If the file is not found, it falls back to procedurally generated sample sprites.

### Creating Your Own Atlas

1. Create `assets/sprites/` directory
2. Add your `atlas.png` file (64x64 minimum recommended)
3. Update sprite definitions in `main.rs`:

```rust
atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
// Add more sprites...
```

## Architecture

```
Entity (u32 ID)
├── Components (HashMap storage)
│   ├── Position { x: f32, y: f32 }
│   ├── Velocity { x: f32, y: f32 }
│   ├── Sprite { color: u32, size: usize }
│   └── TextureSprite { atlas_name: String, scale: f32 }
├── Systems (Scheduled execution)
│   ├── InputSystem (handles keyboard input)
│   ├── PhysicsSystem (updates positions, collision)
│   └── RenderSystem (draws sprites to screen)
└── World (manages all entities and components)
```

## Performance Features

- **Frame-rate independent physics** with delta time
- **Efficient sprite atlas** reduces texture memory usage
- **Boundary collision detection** respects individual sprite sizes
- **60 FPS target** with smooth rendering

## Development

### Branches
- `master`: Stable production code
- `dev`: Active development branch

### Building
```bash
cargo build --release
```

### Dependencies
- `minifb`: Cross-platform windowing and pixel buffer
- `image`: PNG loading and image processing

## License

This project is open source and available under the MIT License.

---

Built with ❤️ in Rust
