use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::time::Instant;


// Rapier2D imports
use rapier2d::prelude::*;
use nalgebra::Vector2;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Entity Component System (ECS) Architecture with Rapier2D

// Entity is just a unique identifier
type Entity = u32;

// Components
#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy)]
struct Sprite {
    color: u32,    // RGB color
    size: usize,   // Size in pixels
}

impl Sprite {
    fn new(color: u32, size: usize) -> Self {
        Self { color, size }
    }
}

#[derive(Debug, Clone)]
struct TextureSprite {
    atlas_name: String,   // Which sprite in the atlas to use
    scale: f32,          // Scale factor (1.0 = original size)
}

impl TextureSprite {
    fn new(atlas_name: String, scale: f32) -> Self {
        Self { atlas_name, scale }
    }
}

// World contains all components in HashMaps + Rapier2D Physics World
struct World {
    next_entity_id: Entity,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    sprites: HashMap<Entity, Sprite>,
    texture_sprites: HashMap<Entity, TextureSprite>,
    sprite_atlas: Option<SpriteAtlas>,
    
    // Rapier2D Physics World
    physics_world: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    
    // Map Entity IDs to Rapier RigidBodyHandle
    entity_to_body: HashMap<Entity, RigidBodyHandle>,
    body_to_entity: HashMap<RigidBodyHandle, Entity>,
}

impl World {
    fn new() -> Self {
        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = 1.0 / 60.0; // 60 FPS

        Self {
            next_entity_id: 0,
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            texture_sprites: HashMap::new(),
            sprite_atlas: None,
            
            physics_world: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            
            entity_to_body: HashMap::new(),
            body_to_entity: HashMap::new(),
        }
    }

    fn set_sprite_atlas(&mut self, atlas: SpriteAtlas) {
        self.sprite_atlas = Some(atlas);
    }

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        entity
    }

    fn add_position(&mut self, entity: Entity, position: Position) {
        self.positions.insert(entity, position);
    }

    fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }

    fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprites.insert(entity, sprite);
    }

    fn add_texture_sprite(&mut self, entity: Entity, texture_sprite: TextureSprite) {
        self.texture_sprites.insert(entity, texture_sprite);
    }

    // Create a physics body for this entity
    fn add_physics_body(&mut self, entity: Entity, position: Position, size: f32, body_type: RigidBodyType) {
        // Create Rapier rigid body
        let rigid_body = RigidBodyBuilder::new(body_type)
            .translation(Vector2::new(position.x, position.y))
            .build();

        let body_handle = self.physics_world.insert(rigid_body);

        // Create collider (using a square shape)
        let collider = ColliderBuilder::cuboid(size / 2.0, size / 2.0)
            .restitution(0.8) // Bounciness
            .friction(0.3)    // Friction
            .build();

        self.collider_set.insert_with_parent(collider, body_handle, &mut self.physics_world);

        // Map entity to body handle
        self.entity_to_body.insert(entity, body_handle);
        self.body_to_entity.insert(body_handle, entity);
    }

    fn get_position(&self, entity: Entity) -> Option<&Position> {
        self.positions.get(&entity)
    }

    fn get_position_mut(&mut self, entity: Entity) -> Option<&mut Position> {
        self.positions.get_mut(&entity)
    }

    fn get_velocity(&self, entity: Entity) -> Option<&Velocity> {
        self.velocities.get(&entity)
    }

    fn get_velocity_mut(&mut self, entity: Entity) -> Option<&mut Velocity> {
        self.velocities.get_mut(&entity)
    }

    fn get_sprite(&self, entity: Entity) -> Option<&Sprite> {
        self.sprites.get(&entity)
    }

    fn get_texture_sprite(&self, entity: Entity) -> Option<&TextureSprite> {
        self.texture_sprites.get(&entity)
    }

    // Set velocity of a physics body
    fn set_physics_velocity(&mut self, entity: Entity, velocity: Vector2<f32>) {
        if let Some(&body_handle) = self.entity_to_body.get(&entity) {
            if let Some(body) = self.physics_world.get_mut(body_handle) {
                body.set_linvel(velocity, true);
            }
        }
    }

    // Sync positions from physics world to ECS
    fn sync_positions_from_physics(&mut self) {
        for (&entity, &body_handle) in &self.entity_to_body {
            if let Some(body) = self.physics_world.get(body_handle) {
                let translation = body.translation();
                if let Some(position) = self.positions.get_mut(&entity) {
                    position.x = translation.x;
                    position.y = translation.y;
                }
                
                let velocity = body.linvel();
                if let Some(vel) = self.velocities.get_mut(&entity) {
                    vel.x = velocity.x;
                    vel.y = velocity.y;
                }
            }
        }
    }

    // Step the physics simulation
    fn step_physics(&mut self) {
        self.physics_pipeline.step(
            &Vector2::new(0.0, 0.0), // No gravity - space-like physics
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.physics_world,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &self.physics_hooks,
            &self.event_handler,
        );

        // Sync physics positions back to ECS
        self.sync_positions_from_physics();
    }
}

// Texture and SpriteAtlas system (same as before)
#[derive(Debug, Clone)]
struct Texture {
    width: usize,
    height: usize,
    data: Vec<u32>, // RGBA pixels
}

impl Texture {
    fn new(width: usize, height: usize, data: Vec<u32>) -> Self {
        Self { width, height, data }
    }

    fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0 // Transparent/black for out of bounds
        }
    }
}

#[derive(Debug, Clone)]
struct AtlasSprite {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct SpriteAtlas {
    texture: Texture,
    sprites: HashMap<String, AtlasSprite>,
}

impl SpriteAtlas {
    fn new(texture: Texture) -> Self {
        Self {
            texture,
            sprites: HashMap::new(),
        }
    }

    fn add_sprite(&mut self, name: String, x: usize, y: usize, width: usize, height: usize) {
        self.sprites.insert(name, AtlasSprite { x, y, width, height });
    }

    fn get_sprite(&self, name: &str) -> Option<&AtlasSprite> {
        self.sprites.get(name)
    }
}

struct AssetsLoader;

impl AssetsLoader {
    fn load_png(path: &str) -> Result<Texture, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        
        // Convert RGBA to ARGB format for minifb
        let mut data = Vec::with_capacity((width * height) as usize);
        for pixel in rgba_img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;
            
            // Convert to ARGB format (minifb expects this format)
            let argb = (a << 24) | (r << 16) | (g << 8) | b;
            data.push(argb);
        }
        
        Ok(Texture::new(width as usize, height as usize, data))
    }
    
    fn create_sample_atlas() -> SpriteAtlas {
        // Create a simple 128x128 texture with colored squares
        let width = 128;
        let height = 128;
        let mut data = vec![0xFF000000; width * height]; // Black background with full alpha
        
        // Red square (player) at (0,0) 32x32
        for y in 0..32 {
            for x in 0..32 {
                data[y * width + x] = 0xFFFF0000; // Red
            }
        }
        
        // Green square (enemy1) at (32,0) 32x32
        for y in 0..32 {
            for x in 32..64 {
                data[y * width + x] = 0xFF00FF00; // Green
            }
        }
        
        // Blue square (enemy2) at (64,0) 32x32
        for y in 0..32 {
            for x in 64..96 {
                data[y * width + x] = 0xFF0000FF; // Blue
            }
        }
        
        // Yellow square (powerup) at (96,0) 32x32
        for y in 0..32 {
            for x in 96..128 {
                data[y * width + x] = 0xFFFFFF00; // Yellow
            }
        }
        
        let texture = Texture::new(width, height, data);
        let mut atlas = SpriteAtlas::new(texture);
        
        atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
        atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
        atlas.add_sprite("enemy2".to_string(), 64, 0, 32, 32);
        atlas.add_sprite("powerup".to_string(), 96, 0, 32, 32);
        
        atlas
    }
}

// System trait for the scheduler
trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &'static str;
}

// Input System - handles keyboard input
struct InputSystem {
    player_entity: Entity,
}

impl InputSystem {
    fn new(player_entity: Entity) -> Self {
        Self { player_entity }
    }

    fn set_input(&mut self, _window: &Window) {
        // Input is now handled in the main loop directly
        // This method is kept for interface compatibility
    }
}

impl System for InputSystem {
    fn update(&mut self, _world: &mut World, _dt: f32) {
        // Input is handled separately since it needs window reference
    }

    fn name(&self) -> &'static str {
        "InputSystem"
    }
}

// Physics System - now just calls Rapier2D step
struct PhysicsSystem;

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        // Step the Rapier2D physics world
        world.step_physics();
    }

    fn name(&self) -> &'static str {
        "PhysicsSystem"
    }
}

// Render System - handles drawing
struct RenderSystem;

impl System for RenderSystem {
    fn update(&mut self, _world: &mut World, _dt: f32) {
        // Rendering is handled in the main loop
    }

    fn name(&self) -> &'static str {
        "RenderSystem"
    }
}

impl RenderSystem {
    fn render_frame(buffer: &mut Vec<u32>, world: &World) {
        // Clear the screen with black
        for pixel in buffer.iter_mut() {
            *pixel = 0xFF000000; // Black with full alpha
        }

        // Render all entities with positions
        for (entity, position) in &world.positions {
            // Check for texture sprite first, then regular sprite
            if let Some(texture_sprite) = world.get_texture_sprite(*entity) {
                if let Some(atlas) = &world.sprite_atlas {
                    Self::draw_texture_sprite(buffer, position.x, position.y, texture_sprite, atlas);
                }
            } else if let Some(sprite) = world.get_sprite(*entity) {
                Self::draw_sprite(buffer, position.x, position.y, sprite);
            } else {
                // Fallback: draw a default red square if no sprite
                Self::draw_default_square(buffer, position.x, position.y);
            }
        }
    }

    fn draw_sprite(buffer: &mut Vec<u32>, x: f32, y: f32, sprite: &Sprite) {
        let sprite_x = x as usize;
        let sprite_y = y as usize;

        for y in sprite_y..sprite_y + sprite.size {
            for x in sprite_x..sprite_x + sprite.size {
                if x < WIDTH && y < HEIGHT {
                    let index = y * WIDTH + x;
                    buffer[index] = sprite.color | 0xFF000000; // Ensure alpha is set
                }
            }
        }
    }

    fn draw_texture_sprite(buffer: &mut Vec<u32>, x: f32, y: f32, texture_sprite: &TextureSprite, atlas: &SpriteAtlas) {
        if let Some(atlas_sprite) = atlas.get_sprite(&texture_sprite.atlas_name) {
            let dest_x = x as i32;
            let dest_y = y as i32;
            let scaled_width = (atlas_sprite.width as f32 * texture_sprite.scale) as i32;
            let scaled_height = (atlas_sprite.height as f32 * texture_sprite.scale) as i32;
            
            for dy in 0..scaled_height {
                for dx in 0..scaled_width {
                    let screen_x = dest_x + dx;
                    let screen_y = dest_y + dy;
                    
                    if screen_x >= 0 && screen_x < WIDTH as i32 && screen_y >= 0 && screen_y < HEIGHT as i32 {
                        // Map screen coordinates back to atlas coordinates
                        let atlas_x = atlas_sprite.x + (dx as f32 / texture_sprite.scale) as usize;
                        let atlas_y = atlas_sprite.y + (dy as f32 / texture_sprite.scale) as usize;
                        
                        let pixel = atlas.texture.get_pixel(atlas_x, atlas_y);
                        
                        // Only draw non-transparent pixels
                        if (pixel >> 24) & 0xFF > 0 {
                            let index = screen_y as usize * WIDTH + screen_x as usize;
                            buffer[index] = pixel;
                        }
                    }
                }
            }
        }
    }

    fn draw_default_square(buffer: &mut Vec<u32>, x: f32, y: f32) {
        let sprite_x = x as usize;
        let sprite_y = y as usize;
        let default_size = 20;
        let default_color = 0xFFFF0000; // Red with full alpha

        for y in sprite_y..sprite_y + default_size {
            for x in sprite_x..sprite_x + default_size {
                if x < WIDTH && y < HEIGHT {
                    let index = y * WIDTH + x;
                    buffer[index] = default_color;
                }
            }
        }
    }
}

// ECS Scheduler for managing update systems
struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    fn update(&mut self, world: &mut World, dt: f32) {
        for system in &mut self.systems {
            system.update(world, dt);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "RocketEngine - Powered by Rapier2D",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut world = World::new();

    // Load or create sprite atlas
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

    // Create the player entity with texture sprite and physics body
    let player = world.create_entity();
    world.add_position(player, Position { x: 100.0, y: 100.0 });
    world.add_velocity(player, Velocity { x: 0.0, y: 0.0 });
    world.add_texture_sprite(player, TextureSprite::new("player".to_string(), 2.0));
    world.add_physics_body(player, Position { x: 100.0, y: 100.0 }, 32.0, RigidBodyType::Dynamic);

    // Create additional entities with physics bodies
    let enemy1 = world.create_entity();
    world.add_position(enemy1, Position { x: 300.0, y: 200.0 });
    world.add_velocity(enemy1, Velocity { x: 20.0, y: 15.0 });
    world.add_texture_sprite(enemy1, TextureSprite::new("enemy1".to_string(), 1.5));
    world.add_physics_body(enemy1, Position { x: 300.0, y: 200.0 }, 24.0, RigidBodyType::Dynamic);

    let enemy2 = world.create_entity();
    world.add_position(enemy2, Position { x: 500.0, y: 400.0 });
    world.add_velocity(enemy2, Velocity { x: -30.0, y: -20.0 });
    world.add_texture_sprite(enemy2, TextureSprite::new("enemy2".to_string(), 1.0));
    world.add_physics_body(enemy2, Position { x: 500.0, y: 400.0 }, 16.0, RigidBodyType::Dynamic);

    let powerup = world.create_entity();
    world.add_position(powerup, Position { x: 400.0, y: 300.0 });
    world.add_velocity(powerup, Velocity { x: 10.0, y: -10.0 });
    world.add_texture_sprite(powerup, TextureSprite::new("powerup".to_string(), 1.0));
    world.add_physics_body(powerup, Position { x: 400.0, y: 300.0 }, 16.0, RigidBodyType::Dynamic);

    // Setup scheduler with update systems
    let mut scheduler = Scheduler::new();
    let mut input_system = InputSystem::new(player);
    scheduler.add_system(Box::new(PhysicsSystem));

    // Timing for delta time calculation
    let mut last_time = Instant::now();

    println!("RocketEngine started!");
    println!("Use arrow keys to move the player sprite.");
    println!("Watch entities interact with realistic physics!");
    println!("Powered by Rapier2D for professional collision detection!");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Handle input for the player (apply forces to physics body)
        let mut velocity = Vector2::zeros();
        let speed = 150.0;
        
        if window.is_key_down(Key::Left) {
            velocity.x = -speed;
        }
        if window.is_key_down(Key::Right) {
            velocity.x = speed;
        }
        if window.is_key_down(Key::Up) {
            velocity.y = -speed;
        }
        if window.is_key_down(Key::Down) {
            velocity.y = speed;
        }
        
        world.set_physics_velocity(player, velocity);

        // Update all systems via scheduler
        input_system.update(&mut world, dt);
        scheduler.update(&mut world, dt);

        // Render
        RenderSystem::render_frame(&mut buffer, &world);

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    println!("Game ended. Thanks for playing!");
}
