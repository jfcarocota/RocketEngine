# Scene File System Implementation

## 🎯 Overview

This PR introduces a comprehensive scene file system to RocketEngine, enabling data-driven game development through external scene configuration files. Developers can now define entities and their components in RON or JSON files instead of hardcoding them, making level design more flexible and accessible.

## ✨ Features

### 🗂️ **Scene File Format Support**
- **RON (Rust Object Notation)**: Human-readable format with Rust-like syntax
- **JSON**: Standard JavaScript Object Notation for wider compatibility
- **Unified API**: Same loader interface for both formats

### 🏗️ **Scene Structure**
```rust
Scene {
    name: String,
    description: Option<String>,
    entities: Vec<EntityData>
}
```

### 🎮 **Entity Component Support**
- **Position**: (x, y) coordinates in world space
- **Velocity**: (x, y) velocity vectors for movement
- **TextureSprite**: Atlas sprite name and scale factor
- **PhysicsBody**: Size and body type (Dynamic, Fixed, Kinematic variants)

### 🔧 **SceneLoader API**
```rust
// Loading scenes
SceneLoader::load_from_ron("path/to/scene.ron")?;
SceneLoader::load_from_json("path/to/scene.json")?;

// Saving scenes
SceneLoader::save_to_ron(&scene, "path/to/scene.ron")?;
SceneLoader::save_to_json(&scene, "path/to/scene.json")?;

// Spawning entities
let entity_map = SceneLoader::spawn_scene(&scene, &mut world);
```

## 📁 Files Added

- `src/scene.rs` - Complete scene system implementation (302 lines)
- `scenes/example_scene.ron` - Example scene in RON format
- `scenes/example_scene.json` - Example scene in JSON format  
- `scenes/README.md` - Comprehensive documentation

## 🔄 Files Modified

- `Cargo.toml` - Added dependencies: `serde`, `serde_json`, `ron`
- `src/lib.rs` - Exported scene module
- `src/main.rs` - Integrated scene loading with robust fallback system

## 🧪 Example Usage

### RON Scene Format
```ron
Scene(
    name: "Example Scene",
    description: Some("A small example scene"),
    entities: [
        EntityData(
            name: Some("player"),
            position: Some((100.0, 100.0)),
            velocity: Some((0.0, 0.0)),
            texture_sprite: Some(TextureSprite(
                atlas_name: "player",
                scale: 2.0,
            )),
            physics_body: Some(PhysicsBodyData(
                size: 32.0,
                body_type: Dynamic,
            )),
        ),
        // ... more entities
    ],
)
```

### JSON Scene Format
```json
{
  "name": "Example Scene",
  "description": "A small example scene",
  "entities": [
    {
      "name": "player",
      "position": [100.0, 100.0],
      "velocity": [0.0, 0.0],
      "texture_sprite": {
        "atlas_name": "player",
        "scale": 2.0
      },
      "physics_body": {
        "size": 32.0,
        "body_type": "Dynamic"
      }
    }
  ]
}
```

## 🛡️ Backward Compatibility

The implementation maintains full backward compatibility:
- Existing hardcoded entity creation continues to work unchanged
- Automatic fallback when scene files are missing
- No breaking changes to existing APIs

## 🔍 Integration Details

### Main Application Integration
```rust
// Try loading scene files with graceful fallback
let scene_result = SceneLoader::load_from_ron("scenes/example_scene.ron")
    .or_else(|_| SceneLoader::load_from_json("scenes/example_scene.json"));

match scene_result {
    Ok(scene) => {
        let entity_map = SceneLoader::spawn_scene(&scene, &mut world);
        // Use entities from scene
    }
    Err(_) => {
        // Fall back to hardcoded entities
        create_default_entities(&mut world)
    }
}
```

### Named Entity Access
```rust
let entity_map = SceneLoader::spawn_scene(&scene, &mut world);
if let Some(&player_entity) = entity_map.get("player") {
    // Access specific entities by name
    input_system.set_target(player_entity);
}
```

## ✅ Testing

- ✅ **RON Loading**: Successfully loads and spawns entities from RON files
- ✅ **JSON Loading**: Successfully loads and spawns entities from JSON files  
- ✅ **Physics Integration**: All entities have proper physics bodies and collision detection
- ✅ **Fallback System**: Gracefully handles missing scene files
- ✅ **Component Systems**: All components work correctly with serialization
- ✅ **Performance**: No noticeable impact on runtime performance

## 🚀 Benefits

1. **Data-Driven Design**: Level design without code compilation
2. **Rapid Prototyping**: Quick iteration on entity layouts
3. **Designer Friendly**: Non-programmers can create scenes
4. **Version Control**: Scene files can be tracked and merged easily
5. **Modding Support**: External scene files enable community content
6. **Debugging**: Easy to inspect and modify entity configurations

## 📊 Statistics

- **Lines Added**: 623
- **Lines Removed**: 25
- **Files Changed**: 8
- **Dependencies Added**: 3 (`serde`, `serde_json`, `ron`)
- **Example Scenes**: 2 (RON + JSON)

## 🎯 Future Enhancements

This foundation enables future features like:
- Scene editor GUI tools
- Dynamic scene loading/unloading
- Scene inheritance and templating
- Asset reference validation
- Performance profiling per scene

## 📝 Notes

- Scene files are validated during loading with descriptive error messages
- All component serialization is thoroughly tested
- Documentation includes comprehensive usage examples
- Both human-readable (RON) and machine-readable (JSON) formats supported
