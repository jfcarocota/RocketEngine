use eframe::egui;
use std::collections::HashMap;
use std::time::Instant;
use crate::components::*;
use crate::world::World;
use crate::scene::{Scene, SceneLoader, EntityData, PhysicsBodyData, PhysicsBodyType};
use crate::systems::*;

/// Editor state and application
pub struct EditorApp {
    /// The game world
    world: World,
    /// Editor state
    editor_state: EditorState,
    /// Available assets
    asset_manager: AssetManager,
    /// Entity hierarchy
    hierarchy: EntityHierarchy,
    /// Grid editor
    grid_editor: GridEditor,
    /// Game state (playing/stopped)
    game_state: GameState,
    /// Current scene path
    current_scene_path: Option<String>,
    /// Systems scheduler for game simulation
    scheduler: Scheduler,
    /// Last frame time for delta time calculation
    last_time: Instant,
}

/// Editor state management
#[derive(Default)]
pub struct EditorState {
    /// Currently selected entity
    selected_entity: Option<Entity>,
    /// Whether the properties panel is open
    show_properties: bool,
    /// Whether the asset panel is open
    show_assets: bool,
    /// Whether the hierarchy panel is open
    show_hierarchy: bool,
    /// Grid settings
    grid_settings: GridSettings,
    /// Component editor state
    component_editor: ComponentEditorState,
}

/// Component editor state
pub struct ComponentEditorState {
    /// Show add component menu
    show_add_component: bool,
    /// Available component types
    available_components: Vec<ComponentType>,
    /// Component being edited
    editing_component: Option<ComponentType>,
}

/// Available component types that can be added to entities
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Position,
    Velocity,
    TextureSprite,
    PhysicsBody,
}

/// Asset management system
pub struct AssetManager {
    /// Available sprites from the atlas
    available_sprites: Vec<String>,
    /// Drag and drop state
    drag_state: DragState,
}

/// Entity hierarchy management
#[derive(Default)]
pub struct EntityHierarchy {
    /// Parent-child relationships
    parent_child_map: HashMap<Entity, Vec<Entity>>,
    /// Root entities (no parent)
    root_entities: Vec<Entity>,
    /// Expanded state in UI
    expanded_entities: HashMap<Entity, bool>,
}

/// Grid-based level editor
pub struct GridEditor {
    /// Grid size (cells per side)
    grid_size: usize,
    /// Cell size in pixels
    cell_size: f32,
    /// Grid offset for panning
    grid_offset: egui::Vec2,
    /// Entities placed on grid
    grid_entities: HashMap<(i32, i32), Entity>,
    /// Currently dragging entity
    dragging_entity: Option<DraggedEntity>,
}

/// Drag and drop state
#[derive(Default)]
pub struct DragState {
    /// Currently dragging sprite
    dragging_sprite: Option<String>,
    /// Drag start position
    drag_start: Option<egui::Pos2>,
}

/// Dragged entity information
pub struct DraggedEntity {
    pub entity: Entity,
    pub offset: egui::Vec2,
    pub original_grid_pos: (i32, i32),
}

/// Grid display settings
pub struct GridSettings {
    /// Show grid lines
    pub show_grid: bool,
    /// Grid line color
    pub grid_color: egui::Color32,
    /// Snap to grid
    pub snap_to_grid: bool,
    /// Grid size in world units
    pub grid_size: f32,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            show_grid: true,
            grid_color: egui::Color32::from_rgba_unmultiplied(100, 100, 100, 128),
            snap_to_grid: true,
            grid_size: 32.0,
        }
    }
}

/// Game state management
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Stopped,
    Playing,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Stopped
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self {
            available_sprites: vec![
                "player".to_string(),
                "enemy1".to_string(),
                "enemy2".to_string(),
                "powerup".to_string(),
            ],
            drag_state: DragState::default(),
        }
    }
}

impl Default for ComponentEditorState {
    fn default() -> Self {
        Self {
            show_add_component: false,
            available_components: vec![
                ComponentType::Position,
                ComponentType::Velocity,
                ComponentType::TextureSprite,
                ComponentType::PhysicsBody,
            ],
            editing_component: None,
        }
    }
}

impl Default for GridEditor {
    fn default() -> Self {
        Self {
            grid_size: 20,
            cell_size: 40.0,
            grid_offset: egui::Vec2::ZERO,
            grid_entities: HashMap::new(),
            dragging_entity: None,
        }
    }
}

impl EditorApp {
    /// Create a new editor application
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Load sprite atlas
        let atlas = match AssetsLoader::load_png("assets/sprites/atlas.png") {
            Ok(texture) => {
                println!("Successfully loaded PNG atlas!");
                let mut atlas = SpriteAtlas::new(texture);
                atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
                atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
                atlas.add_sprite("enemy2".to_string(), 64, 0, 32, 32);
                atlas.add_sprite("powerup".to_string(), 96, 0, 32, 32);
                atlas
            }
            Err(_) => {
                println!("Could not load PNG atlas, using sample sprites");
                AssetsLoader::create_sample_atlas()
            }
        };
        
        world.set_sprite_atlas(atlas);

        // Setup systems scheduler for game simulation
        let mut scheduler = Scheduler::new();
        scheduler.add_system(Box::new(VelocitySyncSystem::new()));
        scheduler.add_system(Box::new(PhysicsSystem::new()));
        scheduler.add_query_system(MovementSystem::new());

        Self {
            world,
            editor_state: EditorState::default(),
            asset_manager: AssetManager::default(),
            hierarchy: EntityHierarchy::default(),
            grid_editor: GridEditor::default(),
            game_state: GameState::Stopped,
            current_scene_path: None,
            scheduler,
            last_time: Instant::now(),
        }
    }

    /// Create a new entity at the specified grid position
    pub fn create_entity_at_grid(&mut self, grid_x: i32, grid_y: i32, sprite_name: &str) -> Entity {
        let entity = self.world.create_entity();
        
        // Convert grid position to world position
        let world_x = grid_x as f32 * self.editor_state.grid_settings.grid_size;
        let world_y = grid_y as f32 * self.editor_state.grid_settings.grid_size;
        
        // Add components
        self.world.add_position(entity, Position::new(world_x, world_y));
        self.world.add_velocity(entity, Velocity::new(
            // Add small velocity for interesting physics
            ((entity % 3) as f32 - 1.0) * 10.0,
            ((entity % 5) as f32 - 2.0) * 10.0
        ));
        self.world.add_texture_sprite(entity, TextureSprite::with_name(sprite_name));
        
        // Add physics body for most entities
        self.world.add_physics_body(
            entity,
            Position::new(world_x, world_y),
            self.editor_state.grid_settings.grid_size / 2.0,
            rapier2d::prelude::RigidBodyType::Dynamic,
        );

        // Add to grid
        self.grid_editor.grid_entities.insert((grid_x, grid_y), entity);
        
        // Add to hierarchy as root entity
        self.hierarchy.root_entities.push(entity);

        println!("Created entity {} at grid ({}, {}) with world pos ({:.1}, {:.1})", 
                entity, grid_x, grid_y, world_x, world_y);

        entity
    }

    /// Load a scene into the editor
    pub fn load_scene(&mut self, scene_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Clear current state
        self.clear_scene();

        // Load scene
        let scene = if scene_path.ends_with(".ron") {
            SceneLoader::load_from_ron(scene_path)?
        } else {
            SceneLoader::load_from_json(scene_path)?
        };

        // Spawn entities
        let entity_map = SceneLoader::spawn_scene(&scene, &mut self.world);

        // Rebuild grid and hierarchy from spawned entities
        self.rebuild_editor_state(&entity_map);
        
        self.current_scene_path = Some(scene_path.to_string());
        Ok(())
    }

    /// Save the current scene
    pub fn save_scene(&self, scene_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let scene = self.create_scene_from_editor();
        
        if scene_path.ends_with(".ron") {
            SceneLoader::save_to_ron(&scene, scene_path)?;
        } else {
            SceneLoader::save_to_json(&scene, scene_path)?;
        }
        
        Ok(())
    }

    /// Clear the current scene
    fn clear_scene(&mut self) {
        // Remove all entities except keep world structure
        self.grid_editor.grid_entities.clear();
        self.hierarchy.root_entities.clear();
        self.hierarchy.parent_child_map.clear();
        self.editor_state.selected_entity = None;
        
        // Create a new world to clear everything
        let mut new_world = World::new();
        if let Some(atlas) = self.world.sprite_atlas.take() {
            new_world.set_sprite_atlas(atlas);
        }
        self.world = new_world;
    }

    /// Rebuild editor state from entity map
    fn rebuild_editor_state(&mut self, entity_map: &HashMap<String, Entity>) {
        // For now, place entities in a simple grid layout
        // In a full implementation, you'd read grid positions from scene metadata
        let mut grid_x = 0;
        let mut grid_y = 0;
        
        for (_name, &entity) in entity_map {
            // Add to grid at current position
            self.grid_editor.grid_entities.insert((grid_x, grid_y), entity);
            
            // Add to hierarchy
            self.hierarchy.root_entities.push(entity);
            
            // Move to next grid position
            grid_x += 1;
            if grid_x >= 10 {
                grid_x = 0;
                grid_y += 1;
            }
        }
    }

    /// Create scene from current editor state
    fn create_scene_from_editor(&self) -> Scene {
        let mut entities = Vec::new();

        // Convert grid entities to scene entities
        for ((grid_x, grid_y), &entity) in &self.grid_editor.grid_entities {
            if let Some(entity_data) = self.create_entity_data(entity, Some((*grid_x, *grid_y))) {
                entities.push(entity_data);
            }
        }

        Scene {
            name: "Editor Scene".to_string(),
            description: Some("Scene created from editor".to_string()),
            entities,
        }
    }

    /// Create entity data from world entity
    fn create_entity_data(&self, entity: Entity, grid_pos: Option<(i32, i32)>) -> Option<EntityData> {
        let position = self.world.get_position(entity).copied();
        let velocity = self.world.get_velocity(entity).copied();
        let texture_sprite = self.world.get_texture_sprite(entity).cloned();

        // Create physics body data if entity has physics
        let physics_body = if self.world.entity_to_body.contains_key(&entity) {
            Some(PhysicsBodyData {
                size: self.editor_state.grid_settings.grid_size / 2.0,
                body_type: PhysicsBodyType::Dynamic,
            })
        } else {
            None
        };

        // Generate entity name from grid position or entity ID
        let name = if let Some((x, y)) = grid_pos {
            Some(format!("entity_{}_{}", x, y))
        } else {
            Some(format!("entity_{}", entity))
        };

        Some(EntityData {
            name,
            position,
            velocity,
            texture_sprite,
            physics_body,
        })
    }

    /// Get world position from grid coordinates
    fn grid_to_world(&self, grid_x: i32, grid_y: i32) -> (f32, f32) {
        (
            grid_x as f32 * self.editor_state.grid_settings.grid_size,
            grid_y as f32 * self.editor_state.grid_settings.grid_size,
        )
    }

    /// Get grid coordinates from world position
    fn world_to_grid(&self, world_x: f32, world_y: f32) -> (i32, i32) {
        (
            (world_x / self.editor_state.grid_settings.grid_size).round() as i32,
            (world_y / self.editor_state.grid_settings.grid_size).round() as i32,
        )
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update game simulation if playing
        if self.game_state == GameState::Playing {
            // Calculate delta time
            let current_time = Instant::now();
            let dt = current_time.duration_since(self.last_time).as_secs_f32();
            self.last_time = current_time;
            
            // Run systems scheduler (includes physics, movement, etc.)
            self.scheduler.update(&mut self.world, dt);
            
            // Request repaint for smooth animation
            ctx.request_repaint();
        } else {
            // Reset timer when not playing to avoid large dt when resuming
            self.last_time = Instant::now();
        }

        // Main menu bar
        self.draw_menu_bar(ctx);

        // Main layout with panels
        egui::SidePanel::left("asset_panel")
            .default_width(200.0)
            .show_animated(ctx, self.editor_state.show_assets, |ui| {
                self.draw_asset_panel(ui);
            });

        egui::SidePanel::right("hierarchy_panel")
            .default_width(200.0)
            .show_animated(ctx, self.editor_state.show_hierarchy, |ui| {
                self.draw_hierarchy_panel(ui);
            });

        egui::TopBottomPanel::bottom("toolbar")
            .default_height(60.0)
            .show(ctx, |ui| {
                self.draw_toolbar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_grid_editor(ui);
        });

        // Properties panel (floating window)
        if self.editor_state.show_properties {
            self.draw_properties_window(ctx);
        }
    }
}

// UI drawing methods will be implemented in separate impl blocks
impl EditorApp {
    fn draw_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Scene").clicked() {
                        self.clear_scene();
                        ui.close_menu();
                    }
                    
                    if ui.button("Open Scene...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                    
                    if ui.button("Save Scene").clicked() {
                        if let Some(ref path) = self.current_scene_path.clone() {
                            if let Err(e) = self.save_scene(path) {
                                eprintln!("Failed to save scene: {}", e);
                            }
                        }
                        ui.close_menu();
                    }
                    
                    if ui.button("Save Scene As...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.editor_state.show_assets, "Asset Panel");
                    ui.checkbox(&mut self.editor_state.show_hierarchy, "Hierarchy Panel");
                    ui.checkbox(&mut self.editor_state.show_properties, "Properties Panel");
                    ui.separator();
                    ui.checkbox(&mut self.editor_state.grid_settings.show_grid, "Show Grid");
                    ui.checkbox(&mut self.editor_state.grid_settings.snap_to_grid, "Snap to Grid");
                });

                ui.menu_button("Game", |ui| {
                    let play_text = match self.game_state {
                        GameState::Stopped => "▶ Play",
                        GameState::Playing => "⏸ Pause",
                        GameState::Paused => "▶ Resume",
                    };
                    
                    if ui.button(play_text).clicked() {
                        self.toggle_play_state();
                        ui.close_menu();
                    }
                    
                    if ui.button("⏹ Stop").clicked() {
                        self.game_state = GameState::Stopped;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn draw_asset_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Assets");
        ui.separator();

        for sprite_name in &self.asset_manager.available_sprites.clone() {
            let button = ui.button(&format!("📦 {}", sprite_name));
            
            if button.hovered() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            }

            // Handle drag start
            if button.drag_started() {
                self.asset_manager.drag_state.dragging_sprite = Some(sprite_name.clone());
                self.asset_manager.drag_state.drag_start = Some(button.rect.center());
            }
        }

        // Clear drag state if not dragging
        if !ui.input(|i| i.pointer.any_pressed()) {
            self.asset_manager.drag_state.dragging_sprite = None;
            self.asset_manager.drag_state.drag_start = None;
        }
    }

    fn draw_hierarchy_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hierarchy");
        ui.separator();

        if ui.button("+ Create Empty Entity").clicked() {
            let entity = self.world.create_entity();
            self.hierarchy.root_entities.push(entity);
        }

        ui.separator();

        // Draw entity tree
        let root_entities = self.hierarchy.root_entities.clone();
        for entity in root_entities {
            self.draw_entity_tree_node(ui, entity, 0);
        }
    }

    fn draw_entity_tree_node(&mut self, ui: &mut egui::Ui, entity: Entity, depth: usize) {
        let indent = "  ".repeat(depth);
        
        // Build entity display name with components
        let mut entity_name = format!("{}Entity {}", indent, entity);
        let mut components_info = Vec::new();
        
        if self.world.positions.contains_key(&entity) {
            components_info.push("📍");
        }
        if self.world.velocities.contains_key(&entity) {
            components_info.push("🏃");
        }
        if self.world.texture_sprites.contains_key(&entity) {
            components_info.push("🖼️");
        }
        if self.world.entity_to_body.contains_key(&entity) {
            components_info.push("⚡");
        }
        
        if !components_info.is_empty() {
            entity_name = format!("{} [{}]", entity_name, components_info.join(" "));
        }
        
        let selected = self.editor_state.selected_entity == Some(entity);
        let response = ui.selectable_label(selected, entity_name);
        
        if response.clicked() {
            self.editor_state.selected_entity = Some(entity);
            // Auto-open properties panel when selecting entity
            self.editor_state.show_properties = true;
        }

        // Draw children if expanded
        let children = self.hierarchy.parent_child_map.get(&entity).cloned();
        if let Some(children) = children {
            for child in children {
                self.draw_entity_tree_node(ui, child, depth + 1);
            }
        }
    }

    fn draw_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Play/Stop controls with status indicators
            let (play_button_text, play_button_color) = match self.game_state {
                GameState::Stopped => ("▶ Play", egui::Color32::GREEN),
                GameState::Playing => ("⏸ Pause", egui::Color32::YELLOW),
                GameState::Paused => ("▶ Resume", egui::Color32::BLUE),
            };

            let play_button = egui::Button::new(play_button_text)
                .fill(play_button_color);
            
            if ui.add(play_button).clicked() {
                self.toggle_play_state();
            }

            let stop_button = egui::Button::new("⏹ Stop")
                .fill(egui::Color32::RED);
            
            if ui.add(stop_button).clicked() {
                self.game_state = GameState::Stopped;
            }

            ui.separator();

            // Game state indicator
            let state_text = match self.game_state {
                GameState::Stopped => "🔴 EDITING",
                GameState::Playing => "🟢 PLAYING",
                GameState::Paused => "🟡 PAUSED",
            };
            ui.label(state_text);

            ui.separator();

            // Grid settings
            ui.label("Grid Size:");
            ui.add(egui::DragValue::new(&mut self.editor_state.grid_settings.grid_size)
                .range(16.0..=128.0));

            ui.label("Cell Size:");
            ui.add(egui::DragValue::new(&mut self.grid_editor.cell_size)
                .range(20.0..=100.0));

            ui.separator();

            // Entity count
            let entity_count = self.hierarchy.root_entities.len();
            ui.label(&format!("Entities: {}", entity_count));

            ui.separator();

            // Current scene info
            let scene_text = if let Some(ref path) = self.current_scene_path {
                format!("📄 {}", path.split('/').last().unwrap_or(path))
            } else {
                "📄 Untitled Scene".to_string()
            };
            ui.label(scene_text);
        });
    }

    fn draw_grid_editor(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
        let rect = response.rect;

        // Draw grid background
        if self.editor_state.grid_settings.show_grid {
            self.draw_grid_lines(&painter, rect);
        }

        // Handle asset drag and drop
        let dragging_sprite = self.asset_manager.drag_state.dragging_sprite.clone();
        if let Some(sprite_name) = dragging_sprite {
            if response.hovered() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                
                if ui.input(|i| i.pointer.any_released()) {
                    // Drop asset at current position
                    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                        let grid_pos = self.screen_to_grid(pos, rect);
                        
                        // Only create if grid cell is empty
                        if !self.grid_editor.grid_entities.contains_key(&grid_pos) {
                            self.create_entity_at_grid(grid_pos.0, grid_pos.1, &sprite_name);
                        }
                    }
                    
                    // Clear drag state
                    self.asset_manager.drag_state.dragging_sprite = None;
                }
            }
        }

        // Draw entities on grid
        self.draw_grid_entities(&painter, rect);

        // Handle entity selection and dragging
        self.handle_entity_interaction(&response, rect);
    }

    fn draw_properties_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Properties")
            .default_width(350.0)
            .show(ctx, |ui| {
                if let Some(entity) = self.editor_state.selected_entity {
                    ui.heading(&format!("Entity {}", entity));
                    ui.separator();

                    // Component sections
                    self.draw_position_component(ui, entity);
                    self.draw_velocity_component(ui, entity);
                    self.draw_texture_sprite_component(ui, entity);
                    self.draw_physics_body_component(ui, entity);

                    ui.separator();
                    
                    // Add Component button
                    if ui.button("+ Add Component").clicked() {
                        self.editor_state.component_editor.show_add_component = true;
                    }

                    // Add component popup
                    if self.editor_state.component_editor.show_add_component {
                        self.draw_add_component_popup(ui, entity);
                    }

                    ui.separator();
                    if ui.button("🗑 Delete Entity").clicked() {
                        self.delete_entity(entity);
                        self.editor_state.selected_entity = None;
                    }
                } else {
                    ui.label("No entity selected");
                    ui.separator();
                    ui.label("💡 Tip: Click an entity in the hierarchy or grid to select it");
                }
            });
    }

    fn draw_position_component(&mut self, ui: &mut egui::Ui, entity: Entity) {
        ui.collapsing("📍 Position", |ui| {
            if let Some(position) = self.world.get_position_mut(entity) {
                let mut pos_x = position.x;
                let mut pos_y = position.y;
                let mut changed = false;
                
                ui.horizontal(|ui| {
                    ui.label("X:");
                    if ui.add(egui::DragValue::new(&mut pos_x).speed(1.0)).changed() {
                        changed = true;
                    }
                    ui.label("Y:");
                    if ui.add(egui::DragValue::new(&mut pos_y).speed(1.0)).changed() {
                        changed = true;
                    }
                });
                
                if changed {
                    position.x = pos_x;
                    position.y = pos_y;
                    
                    // Update physics body position if it exists
                    if let Some(&body_handle) = self.world.entity_to_body.get(&entity) {
                        if let Some(body) = self.world.physics_world.get_mut(body_handle) {
                            body.set_translation(nalgebra::Vector2::new(pos_x, pos_y), true);
                        }
                    }
                }
            } else {
                ui.horizontal(|ui| {
                    if ui.button("+ Add Position").clicked() {
                        self.world.add_position(entity, Position::new(0.0, 0.0));
                    }
                });
            }
        });
    }

    fn draw_velocity_component(&mut self, ui: &mut egui::Ui, entity: Entity) {
        ui.collapsing("🏃 Velocity", |ui| {
            if let Some(velocity) = self.world.get_velocity_mut(entity) {
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut velocity.x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut velocity.y).speed(0.1));
                });
                
                ui.horizontal(|ui| {
                    ui.label(&format!("Speed: {:.2}", velocity.magnitude()));
                    if ui.button("Reset").clicked() {
                        velocity.x = 0.0;
                        velocity.y = 0.0;
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    if ui.button("+ Add Velocity").clicked() {
                        self.world.add_velocity(entity, Velocity::zero());
                    }
                });
            }
        });
    }

    fn draw_texture_sprite_component(&mut self, ui: &mut egui::Ui, entity: Entity) {
        ui.collapsing("🖼️ Texture Sprite", |ui| {
            if let Some(texture_sprite) = self.world.texture_sprites.get(&entity).cloned() {
                ui.horizontal(|ui| {
                    ui.label("Atlas:");
                    ui.label(&texture_sprite.atlas_name);
                });
                
                let mut scale = texture_sprite.scale;
                ui.horizontal(|ui| {
                    ui.label("Scale:");
                    if ui.add(egui::DragValue::new(&mut scale).range(0.1..=5.0).speed(0.1)).changed() {
                        let new_sprite = TextureSprite::with_scale(&texture_sprite.atlas_name, scale);
                        self.world.texture_sprites.insert(entity, new_sprite);
                    }
                });

                // Sprite selection
                ui.horizontal(|ui| {
                    ui.label("Sprite:");
                    egui::ComboBox::from_id_salt(format!("sprite_selector_{}", entity))
                        .selected_text(&texture_sprite.atlas_name)
                        .show_ui(ui, |ui| {
                            for sprite_name in &self.asset_manager.available_sprites {
                                if ui.selectable_value(&mut texture_sprite.atlas_name.clone(), sprite_name.clone(), sprite_name).clicked() {
                                    let new_sprite = TextureSprite::with_scale(sprite_name, scale);
                                    self.world.texture_sprites.insert(entity, new_sprite);
                                }
                            }
                        });
                });
            } else {
                ui.horizontal(|ui| {
                    if ui.button("+ Add Texture Sprite").clicked() {
                        self.world.add_texture_sprite(entity, TextureSprite::with_name("player"));
                    }
                });
            }
        });
    }

    fn draw_physics_body_component(&mut self, ui: &mut egui::Ui, entity: Entity) {
        ui.collapsing("⚡ Physics Body", |ui| {
            if self.world.entity_to_body.contains_key(&entity) {
                ui.label("✅ Physics body active");
                
                // Show physics info
                if let Some(&body_handle) = self.world.entity_to_body.get(&entity) {
                    if let Some(body) = self.world.physics_world.get(body_handle) {
                        let pos = body.translation();
                        let vel = body.linvel();
                        ui.label(&format!("Physics Pos: ({:.1}, {:.1})", pos.x, pos.y));
                        ui.label(&format!("Physics Vel: ({:.1}, {:.1})", vel.x, vel.y));
                        
                        ui.horizontal(|ui| {
                            if ui.button("Reset Physics").clicked() {
                                if let Some(body) = self.world.physics_world.get_mut(body_handle) {
                                    body.set_linvel(nalgebra::Vector2::zeros(), true);
                                    body.set_angvel(0.0, true);
                                }
                            }
                        });
                    }
                }
                
                ui.horizontal(|ui| {
                    if ui.button("🗑 Remove Physics Body").clicked() {
                        self.remove_physics_body(entity);
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    if ui.button("+ Add Physics Body").clicked() {
                        let position = self.world.get_position(entity)
                            .copied()
                            .unwrap_or(Position::new(0.0, 0.0));
                        self.world.add_physics_body(
                            entity,
                            position,
                            32.0,
                            rapier2d::prelude::RigidBodyType::Dynamic,
                        );
                    }
                });
            }
        });
    }

    fn draw_add_component_popup(&mut self, ui: &mut egui::Ui, entity: Entity) {
        ui.separator();
        ui.label("Add Component:");
        
        let available_components = self.editor_state.component_editor.available_components.clone();
        for component_type in available_components {
            let can_add = match component_type {
                ComponentType::Position => !self.world.positions.contains_key(&entity),
                ComponentType::Velocity => !self.world.velocities.contains_key(&entity),
                ComponentType::TextureSprite => !self.world.texture_sprites.contains_key(&entity),
                ComponentType::PhysicsBody => !self.world.entity_to_body.contains_key(&entity),
            };

            if can_add {
                let button_text = match component_type {
                    ComponentType::Position => "📍 Position",
                    ComponentType::Velocity => "🏃 Velocity", 
                    ComponentType::TextureSprite => "🖼️ Texture Sprite",
                    ComponentType::PhysicsBody => "⚡ Physics Body",
                };

                if ui.button(button_text).clicked() {
                    self.add_component_to_entity(entity, component_type);
                    self.editor_state.component_editor.show_add_component = false;
                }
            }
        }

        if ui.button("Cancel").clicked() {
            self.editor_state.component_editor.show_add_component = false;
        }
    }

    fn add_component_to_entity(&mut self, entity: Entity, component_type: ComponentType) {
        match component_type {
            ComponentType::Position => {
                self.world.add_position(entity, Position::new(0.0, 0.0));
            }
            ComponentType::Velocity => {
                self.world.add_velocity(entity, Velocity::zero());
            }
            ComponentType::TextureSprite => {
                self.world.add_texture_sprite(entity, TextureSprite::with_name("player"));
            }
            ComponentType::PhysicsBody => {
                let position = self.world.get_position(entity)
                    .copied()
                    .unwrap_or(Position::new(0.0, 0.0));
                self.world.add_physics_body(
                    entity,
                    position,
                    32.0,
                    rapier2d::prelude::RigidBodyType::Dynamic,
                );
            }
        }
    }

    fn remove_physics_body(&mut self, entity: Entity) {
        if let Some(&body_handle) = self.world.entity_to_body.get(&entity) {
            self.world.physics_world.remove(
                body_handle,
                &mut self.world.island_manager,
                &mut self.world.collider_set,
                &mut self.world.impulse_joint_set,
                &mut self.world.multibody_joint_set,
                true
            );
            self.world.entity_to_body.remove(&entity);
            self.world.body_to_entity.remove(&body_handle);
        }
    }

    fn draw_grid_lines(&self, painter: &egui::Painter, rect: egui::Rect) {
        let grid_size = self.grid_editor.cell_size;
        let stroke = egui::Stroke::new(1.0, self.editor_state.grid_settings.grid_color);

        // Vertical lines
        let mut x = rect.min.x + (grid_size - (rect.min.x % grid_size));
        while x < rect.max.x {
            painter.line_segment([egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)], stroke);
            x += grid_size;
        }

        // Horizontal lines
        let mut y = rect.min.y + (grid_size - (rect.min.y % grid_size));
        while y < rect.max.y {
            painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)], stroke);
            y += grid_size;
        }
    }

    fn draw_grid_entities(&self, painter: &egui::Painter, rect: egui::Rect) {
        // When playing, render based on live world positions
        if self.game_state == GameState::Playing {
            for (&entity, position) in &self.world.positions {
                let screen_pos = self.world_to_screen(position.x, position.y, rect);
                let size = self.grid_editor.cell_size * 0.8;
                let entity_rect = egui::Rect::from_center_size(screen_pos, egui::vec2(size, size));

                let color = if self.editor_state.selected_entity == Some(entity) {
                    egui::Color32::YELLOW
                } else if self.world.texture_sprites.contains_key(&entity) {
                    egui::Color32::from_rgb(100, 150, 255)
                } else if self.world.entity_to_body.contains_key(&entity) {
                    egui::Color32::from_rgb(255, 150, 100)
                } else {
                    egui::Color32::from_rgb(150, 255, 150)
                };

                painter.rect_filled(entity_rect, 4.0, color);
                if self.editor_state.selected_entity == Some(entity) {
                    painter.rect_stroke(entity_rect, 4.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
                }

                let display_text = if let Some(texture_sprite) = self.world.texture_sprites.get(&entity) {
                    texture_sprite.atlas_name.chars().take(8).collect::<String>()
                } else {
                    format!("{}", entity)
                };
                painter.text(
                    entity_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    display_text,
                    egui::FontId::monospace(10.0),
                    egui::Color32::WHITE,
                );
            }
            return;
        }

        // Editing mode: render based on grid placement
        for ((grid_x, grid_y), &entity) in &self.grid_editor.grid_entities {
            let screen_pos = self.grid_to_screen(*grid_x, *grid_y, rect);
            let cell_size = self.grid_editor.cell_size;

            let entity_rect = egui::Rect::from_center_size(
                screen_pos,
                egui::vec2(cell_size * 0.8, cell_size * 0.8),
            );

            let color = if self.editor_state.selected_entity == Some(entity) {
                egui::Color32::YELLOW
            } else if self.world.texture_sprites.contains_key(&entity) {
                egui::Color32::from_rgb(100, 150, 255)
            } else if self.world.entity_to_body.contains_key(&entity) {
                egui::Color32::from_rgb(255, 150, 100)
            } else if self.world.positions.contains_key(&entity) {
                egui::Color32::from_rgb(150, 255, 150)
            } else {
                egui::Color32::GRAY
            };

            painter.rect_filled(entity_rect, 4.0, color);
            if self.editor_state.selected_entity == Some(entity) {
                painter.rect_stroke(entity_rect, 4.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
            }

            let display_text = if let Some(texture_sprite) = self.world.texture_sprites.get(&entity) {
                texture_sprite.atlas_name.chars().take(8).collect::<String>()
            } else {
                format!("{}", entity)
            };
            painter.text(
                entity_rect.center(),
                egui::Align2::CENTER_CENTER,
                display_text,
                egui::FontId::monospace(10.0),
                egui::Color32::WHITE,
            );
        }
    }

    fn handle_entity_interaction(&mut self, response: &egui::Response, rect: egui::Rect) {
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let grid_pos = self.screen_to_grid(pos, rect);
                
                if let Some(&entity) = self.grid_editor.grid_entities.get(&grid_pos) {
                    self.editor_state.selected_entity = Some(entity);
                } else {
                    self.editor_state.selected_entity = None;
                }
            }
        }
    }

    fn screen_to_grid(&self, screen_pos: egui::Pos2, rect: egui::Rect) -> (i32, i32) {
        let relative_pos = screen_pos - rect.min;
        let grid_x = (relative_pos.x / self.grid_editor.cell_size).floor() as i32;
        let grid_y = (relative_pos.y / self.grid_editor.cell_size).floor() as i32;
        (grid_x, grid_y)
    }

    fn grid_to_screen(&self, grid_x: i32, grid_y: i32, rect: egui::Rect) -> egui::Pos2 {
        let x = rect.min.x + (grid_x as f32 + 0.5) * self.grid_editor.cell_size;
        let y = rect.min.y + (grid_y as f32 + 0.5) * self.grid_editor.cell_size;
        egui::pos2(x, y)
    }

    fn world_to_screen(&self, world_x: f32, world_y: f32, rect: egui::Rect) -> egui::Pos2 {
        // Convert world coordinates to grid coordinates first
        let gx = world_x / self.editor_state.grid_settings.grid_size;
        let gy = world_y / self.editor_state.grid_settings.grid_size;
        let x = rect.min.x + (gx + 0.5) * self.grid_editor.cell_size;
        let y = rect.min.y + (gy + 0.5) * self.grid_editor.cell_size;
        egui::pos2(x, y)
    }

    fn toggle_play_state(&mut self) {
        self.game_state = match self.game_state {
            GameState::Stopped => GameState::Playing,
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        };
    }

    fn delete_entity(&mut self, entity: Entity) {
        // Remove from grid
        self.grid_editor.grid_entities.retain(|_, &mut e| e != entity);
        
        // Remove from hierarchy
        self.hierarchy.root_entities.retain(|&e| e != entity);
        self.hierarchy.parent_child_map.remove(&entity);
        
        // Remove from world
        self.world.remove_entity(entity);
    }
}
