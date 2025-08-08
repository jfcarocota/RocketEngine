# Scene System

The RocketEngine scene system allows you to define entities and their components in external data files, making it easy to create and modify game levels without recompiling code.

## Supported Formats

- **RON (Rust Object Notation)**: Human-readable format similar to Rust syntax
- **JSON**: Standard JavaScript Object Notation

## Scene Structure

A scene consists of:
- `name`: Scene identifier
- `description`: Optional scene description
- `entities`: Array of entity definitions

Each entity can have:
- `name`: Optional entity identifier
- `position`: [x, y] coordinates
- `velocity`: [x, y] velocity vector
- `texture_sprite`: Sprite configuration with atlas name and scale
- `physics_body`: Physics body configuration with size and body type

## Physics Body Types

- `Dynamic`: Affected by forces and collisions
- `Fixed`: Static, immovable objects
- `KinematicPositionBased`: Moved by setting position directly
- `KinematicVelocityBased`: Moved by setting velocity directly

## Usage

Load a scene in your application:

```rust
use rocket_engine::*;

// Load from RON
let scene = SceneLoader::load_from_ron("scenes/example_scene.ron")?;

// Load from JSON
let scene = SceneLoader::load_from_json("scenes/example_scene.json")?;

// Spawn entities into the world
let entity_map = SceneLoader::spawn_scene(&scene, &mut world);

// Access specific entities by name
if let Some(&player_entity) = entity_map.get("player") {
    // Use the player entity...
}
```

## Example Files

- `example_scene.ron`: Sample scene in RON format
- `example_scene.json`: Same scene in JSON format

Both files define a small game scene with a player, enemies, a powerup, and a static wall.
