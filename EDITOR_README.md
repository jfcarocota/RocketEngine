# RocketEngine Editor

A comprehensive GUI-based level editor for RocketEngine, built with egui for immediate mode UI.

## 🎯 Features

### 🖼️ **Visual Level Editor**
- **Grid-based editing**: Snap-to-grid entity placement system
- **Drag & Drop**: Drag assets from the asset panel to the grid
- **Visual feedback**: Real-time entity preview and selection
- **Grid visualization**: Toggle grid lines and adjust grid size

### 📦 **Asset Management**
- **Asset Panel**: Browse available sprites and assets
- **Drag & Drop Support**: Drag assets directly to the scene
- **Preview System**: Visual representation of available assets
- **Sprite Atlas Integration**: Direct integration with RocketEngine's sprite system

### 🌳 **Entity Hierarchy**
- **Tree View**: Hierarchical organization of scene entities
- **Entity Selection**: Click to select and inspect entities
- **Parent-Child Relationships**: Support for entity hierarchies (future feature)
- **Entity Management**: Create, delete, and organize entities

### ⚙️ **Properties Panel**
- **Component Editing**: Real-time editing of entity components
- **Position Controls**: Adjust entity positions with drag values
- **Velocity Settings**: Configure entity movement and physics
- **Sprite Configuration**: Change sprite assignments and scaling

### 🎮 **Game Controls**
- **Play/Pause/Stop**: Control game simulation state
- **Real-time Physics**: Watch physics simulation in the editor
- **State Management**: Seamless transitions between editing and playing
- **Scene Validation**: Ensure scene integrity before playing

### 💾 **Scene Management**
- **File Operations**: New, Open, Save, Save As functionality
- **Format Support**: Both RON and JSON scene formats
- **Auto-integration**: Direct connection to RocketEngine's scene system
- **Backup & Recovery**: Scene file management and version control

## 🏗️ Architecture

### Editor Components

```rust
EditorApp
├── World                    // Game world instance
├── EditorState             // Editor-specific state
├── AssetManager            // Asset browsing and drag & drop
├── EntityHierarchy         // Entity organization and tree view
├── GridEditor              // Grid-based level editing
└── GameState               // Play/pause/stop controls
```

### Key Systems

1. **Grid System**: Snap-to-grid entity placement with configurable grid size
2. **Drag & Drop**: Asset-to-scene and entity-to-entity manipulation
3. **State Management**: Seamless switching between editing and play modes
4. **Scene Integration**: Direct save/load from RocketEngine scene files
5. **Component Editor**: Real-time component property modification

## 🚀 Getting Started

### Running the Editor

```bash
# Build and run the editor
cargo run --bin editor

# Or run the traditional game
cargo run --bin rocket_engine
```

### Editor Interface

The editor provides a multi-panel interface:

- **Left Panel**: Asset Manager with draggable sprites
- **Center Panel**: Grid-based level editor with entity placement
- **Right Panel**: Entity hierarchy and organization
- **Top Bar**: Menu with file operations and view options
- **Bottom Bar**: Game controls and grid settings
- **Floating Window**: Properties panel for selected entities

### Basic Workflow

1. **Start the Editor**: Run `cargo run --bin editor`
2. **Create New Scene**: File → New Scene
3. **Add Entities**: Drag assets from the left panel to the grid
4. **Edit Properties**: Select entities and modify in the Properties panel
5. **Test Scene**: Click Play to run the simulation
6. **Save Scene**: File → Save Scene As... (RON or JSON format)
7. **Load Scene**: File → Open Scene...

## 🎨 User Interface

### Grid Editor

- **Grid Visualization**: Toggle with View → Show Grid
- **Snap to Grid**: Enable/disable with View → Snap to Grid
- **Grid Size**: Adjust in the bottom toolbar
- **Entity Placement**: Drag assets from Asset Panel to grid cells
- **Entity Selection**: Click entities to select and edit

### Asset Panel

- **Available Assets**: Shows all sprites from the loaded atlas
- **Drag & Drop**: Click and drag to place in scene
- **Asset Preview**: Visual representation of each sprite
- **Asset Organization**: Grouped by sprite atlas

### Hierarchy Panel

- **Entity Tree**: Hierarchical view of all scene entities
- **Selection**: Click to select entities
- **Entity Creation**: "Create Empty Entity" button
- **Entity Organization**: Visual representation of parent-child relationships

### Properties Panel

- **Position**: X/Y coordinates with drag controls
- **Velocity**: Movement vector configuration
- **Sprite**: Atlas name and scale settings
- **Physics**: Body type and size configuration
- **Actions**: Delete entity button

### Toolbar & Menus

**File Menu**:
- New Scene: Clear current scene
- Open Scene: Load from RON/JSON
- Save Scene: Save to current file
- Save Scene As: Save with file picker

**View Menu**:
- Toggle panels: Asset, Hierarchy, Properties
- Grid options: Show Grid, Snap to Grid

**Game Menu**:
- Play/Pause/Resume: Control simulation
- Stop: Reset to editing mode

## 🔧 Technical Details

### Scene Format Integration

The editor directly integrates with RocketEngine's scene system:

```rust
// Supported scene formats
Scene {
    name: String,
    description: Option<String>,
    entities: Vec<EntityData>
}

EntityData {
    name: Option<String>,
    position: Option<Position>,
    velocity: Option<Velocity>,
    texture_sprite: Option<TextureSprite>,
    physics_body: Option<PhysicsBodyData>,
}
```

### Entity Components

- **Position**: (x, y) world coordinates
- **Velocity**: (x, y) movement vector
- **TextureSprite**: Atlas sprite name and scale
- **PhysicsBody**: Size and body type (Dynamic, Fixed, Kinematic)

### Grid System

- **Grid Cells**: 32x32 pixel cells by default
- **World Coordinates**: Automatic conversion between grid and world space
- **Snap to Grid**: Optional snapping for precise placement
- **Visual Grid**: Overlay grid lines for alignment

### State Management

```rust
GameState {
    Stopped,   // Editing mode - full editor functionality
    Playing,   // Simulation running - limited editing
    Paused,    // Simulation paused - can resume or stop
}
```

## 🎯 Future Enhancements

### Planned Features

1. **Entity Templates**: Reusable entity configurations
2. **Layer System**: Multiple layers for complex scenes
3. **Copy/Paste**: Duplicate entities and configurations
4. **Undo/Redo**: Action history and reversal
5. **Asset Import**: Drag & drop external images
6. **Scene Preview**: Thumbnail previews of scenes
7. **Component Editor**: Advanced component property editing
8. **Script Integration**: Visual scripting or Lua integration
9. **Tilemap Support**: Tile-based level editing
10. **Animation Preview**: Preview sprite animations

### Advanced Tools

- **Entity Grouping**: Group selection and manipulation
- **Prefab System**: Reusable entity groups
- **Asset Pipeline**: Automatic sprite atlas generation
- **Scene Templates**: Pre-configured scene types
- **Entity Search**: Find entities by name or component
- **Property Binding**: Link entity properties
- **Scene Validation**: Check for common issues
- **Performance Tools**: Profiling and optimization

## 💡 Tips & Tricks

### Efficient Workflow

1. **Use Keyboard Shortcuts**: 
   - Ctrl+N: New scene
   - Ctrl+O: Open scene
   - Ctrl+S: Save scene
   - Space: Play/Pause
   - Delete: Remove selected entity

2. **Grid Management**:
   - Adjust grid size for different detail levels
   - Use snap-to-grid for precise alignment
   - Toggle grid visibility when not needed

3. **Entity Organization**:
   - Use descriptive entity names
   - Group related entities in hierarchy
   - Use the properties panel for fine-tuning

4. **Scene Testing**:
   - Test scenes frequently with Play button
   - Use Stop to return to editing mode
   - Check entity physics and interactions

### Best Practices

1. **Scene Structure**: Organize entities logically in hierarchy
2. **Naming Convention**: Use clear, descriptive entity names
3. **Component Setup**: Configure physics bodies appropriately
4. **Performance**: Avoid too many dynamic entities
5. **Testing**: Regularly test scene behavior
6. **Backup**: Save scenes frequently during development

## 🐛 Troubleshooting

### Common Issues

**Editor Won't Start**:
- Check asset files are present
- Verify Cargo.toml dependencies
- Run `cargo clean && cargo build`

**Drag & Drop Not Working**:
- Ensure grid cells are empty
- Check asset panel responsiveness
- Verify sprite atlas is loaded

**Scene Won't Load**:
- Check file format (RON vs JSON)
- Validate scene file syntax
- Ensure sprite references exist

**Physics Issues**:
- Verify physics body configuration
- Check entity positioning
- Ensure proper body types

### Performance Tips

1. **Entity Count**: Limit dynamic entities for better performance
2. **Grid Size**: Use appropriate grid resolution
3. **Asset Loading**: Ensure efficient sprite atlas
4. **Scene Complexity**: Balance detail with performance

## 📖 Additional Resources

- [RocketEngine Documentation](../README.md)
- [Scene System Guide](../scenes/README.md)
- [Component Reference](../src/components/)
- [Examples & Tutorials](../examples/)

The RocketEngine Editor provides a powerful, intuitive interface for creating game levels and scenes. With its grid-based editing, drag & drop functionality, and real-time preview capabilities, it streamlines the game development workflow and makes level design accessible to both programmers and designers.
