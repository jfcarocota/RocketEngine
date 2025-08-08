use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::time::Instant;
use image::RgbaImage;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Entity Component System (ECS) Architecture with Scheduler

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

// Sprite Atlas for managing multiple sprites in one texture
#[derive(Debug, Clone)]
struct SpriteAtlas {
    texture: Texture,
    sprites: HashMap<String, AtlasSprite>,
}

#[derive(Debug, Clone)]
struct AtlasSprite {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
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

// Assets loader
struct AssetsLoader;

impl AssetsLoader {
    fn load_png(path: &str) -> Result<Texture, Box<dyn std::error::Error>> {
        println!("Loading texture: {}", path);
        
        let img = image::open(path)?;
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        
        // Convert RGBA8 to u32 format (ARGB)
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for pixel in rgba_img.pixels() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = pixel[3] as u32;
            
            // Convert to ARGB format for minifb
            let argb = (a << 24) | (r << 16) | (g << 8) | b;
            pixels.push(argb);
        }
        
        Ok(Texture::new(width as usize, height as usize, pixels))
    }

    fn create_sample_sprites() -> SpriteAtlas {
        // Create a simple 64x64 atlas with colored squares for testing
        let mut atlas_data = vec![0; 64 * 64];
        
        // Red sprite (0,0) 16x16
        for y in 0..16 {
            for x in 0..16 {
                atlas_data[y * 64 + x] = 0xFFFF0000; // Red
            }
        }
        
        // Green sprite (16,0) 16x16
        for y in 0..16 {
            for x in 16..32 {
                atlas_data[y * 64 + x] = 0xFF00FF00; // Green
            }
        }
        
        // Blue sprite (32,0) 16x16
        for y in 0..16 {
            for x in 32..48 {
                atlas_data[y * 64 + x] = 0xFF0000FF; // Blue
            }
        }
        
        // Yellow sprite (0,16) 16x16
        for y in 16..32 {
            for x in 0..16 {
                atlas_data[y * 64 + x] = 0xFFFFFF00; // Yellow
            }
        }
        
        let texture = Texture::new(64, 64, atlas_data);
        let mut atlas = SpriteAtlas::new(texture);
        
        atlas.add_sprite("player".to_string(), 0, 0, 16, 16);
        atlas.add_sprite("enemy1".to_string(), 16, 0, 16, 16);
        atlas.add_sprite("enemy2".to_string(), 32, 0, 16, 16);
        atlas.add_sprite("powerup".to_string(), 0, 16, 16, 16);
        
        atlas
    }
}

// Component that uses texture atlas sprites
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

#[derive(Debug, Clone, Copy)]
struct Collider {
    width: f32,
    height: f32,
    solid: bool,         // If true, entities bounce off; if false, just trigger events
}

impl Collider {
    fn new(width: f32, height: f32, solid: bool) -> Self {
        Self { width, height, solid }
    }

    fn get_aabb(&self, position: &Position) -> AABB {
        AABB {
            min_x: position.x,
            min_y: position.y,
            max_x: position.x + self.width,
            max_y: position.y + self.height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AABB {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl AABB {
    fn intersects(&self, other: &AABB) -> bool {
        self.min_x < other.max_x &&
        self.max_x > other.min_x &&
        self.min_y < other.max_y &&
        self.max_y > other.min_y
    }

    fn get_overlap(&self, other: &AABB) -> Option<(f32, f32)> {
        if !self.intersects(other) {
            return None;
        }

        let overlap_x = (self.max_x.min(other.max_x) - self.min_x.max(other.min_x)).abs();
        let overlap_y = (self.max_y.min(other.max_y) - self.min_y.max(other.min_y)).abs();
        
        Some((overlap_x, overlap_y))
    }
}

// World contains all components in HashMaps
struct World {
    next_entity_id: Entity,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    sprites: HashMap<Entity, Sprite>,
    texture_sprites: HashMap<Entity, TextureSprite>,
    colliders: HashMap<Entity, Collider>,
    sprite_atlas: Option<SpriteAtlas>,
}

impl World {
    fn new() -> Self {
        Self {
            next_entity_id: 0,
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            texture_sprites: HashMap::new(),
            colliders: HashMap::new(),
            sprite_atlas: None,
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

    fn add_collider(&mut self, entity: Entity, collider: Collider) {
        self.colliders.insert(entity, collider);
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

    fn get_collider(&self, entity: Entity) -> Option<&Collider> {
        self.colliders.get(&entity)
    }
}

// System trait for the scheduler
trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &'static str;
}

// Update Systems (scheduled)
struct InputSystem {
    player_entity: Entity,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl InputSystem {
    fn new(player_entity: Entity) -> Self {
        Self { 
            player_entity,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    fn set_input(&mut self, window: &Window) {
        self.up_pressed = window.is_key_down(Key::Up);
        self.down_pressed = window.is_key_down(Key::Down);
        self.left_pressed = window.is_key_down(Key::Left);
        self.right_pressed = window.is_key_down(Key::Right);
    }
}

impl System for InputSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        if let Some(velocity) = world.get_velocity_mut(self.player_entity) {
            let speed = 200.0; // pixels per second
            
            velocity.x = 0.0;
            velocity.y = 0.0;

            if self.left_pressed {
                velocity.x = -speed;
            }
            if self.right_pressed {
                velocity.x = speed;
            }
            if self.up_pressed {
                velocity.y = -speed;
            }
            if self.down_pressed {
                velocity.y = speed;
            }
        }
    }

    fn name(&self) -> &'static str {
        "InputSystem"
    }
}

struct PhysicsSystem;

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Get all entities that have both position and velocity
        let entities_with_velocity: Vec<Entity> = world.velocities.keys().cloned().collect();
        
        for entity in entities_with_velocity {
            // First get the velocity and sprite size (copy them since they're Copy)
            if let Some(velocity) = world.get_velocity(entity).copied() {
                // Get size from either regular sprite or texture sprite
                let sprite_size = if let Some(sprite) = world.get_sprite(entity) {
                    sprite.size
                } else if let Some(texture_sprite) = world.get_texture_sprite(entity) {
                    if let Some(atlas) = &world.sprite_atlas {
                        if let Some(atlas_sprite) = atlas.get_sprite(&texture_sprite.atlas_name) {
                            ((atlas_sprite.width as f32 * texture_sprite.scale) as usize).max(1)
                        } else {
                            20 // Default if sprite not found in atlas
                        }
                    } else {
                        20 // Default if no atlas
                    }
                } else {
                    20 // Default size if no sprite component
                };

                // Now we can get a mutable reference to position
                if let Some(position) = world.get_position_mut(entity) {
                    // Update position based on velocity and delta time
                    position.x += velocity.x * dt;
                    position.y += velocity.y * dt;

                    // Keep within bounds based on sprite size
                    position.x = position.x.max(0.0).min((WIDTH - sprite_size) as f32);
                    position.y = position.y.max(0.0).min((HEIGHT - sprite_size) as f32);

                    // Print position updates
                    if velocity.x != 0.0 || velocity.y != 0.0 {
                        println!("Entity {}: Position: {:?}", entity, position);
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "PhysicsSystem"
    }
}

// Collision System - handles entity collisions
struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        // Get all entities with both position and collider
        let entities_with_colliders: Vec<Entity> = world.colliders.keys().cloned().collect();
        
        // Check all pairs of colliding entities
        for i in 0..entities_with_colliders.len() {
            for j in (i + 1)..entities_with_colliders.len() {
                let entity_a = entities_with_colliders[i];
                let entity_b = entities_with_colliders[j];
                
                // Get positions and colliders for both entities
                if let (Some(pos_a), Some(collider_a), Some(pos_b), Some(collider_b)) = (
                    world.get_position(entity_a),
                    world.get_collider(entity_a),
                    world.get_position(entity_b),
                    world.get_collider(entity_b),
                ) {
                    let aabb_a = collider_a.get_aabb(pos_a);
                    let aabb_b = collider_b.get_aabb(pos_b);
                    
                    // Check for collision
                    if aabb_a.intersects(&aabb_b) {
                        self.handle_collision(world, entity_a, entity_b, &aabb_a, &aabb_b);
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "CollisionSystem"
    }
}

impl CollisionSystem {
    fn handle_collision(&self, world: &mut World, entity_a: Entity, entity_b: Entity, aabb_a: &AABB, aabb_b: &AABB) {
        // Get collision overlap
        if let Some((overlap_x, overlap_y)) = aabb_a.get_overlap(aabb_b) {
            // Determine collision direction (smallest overlap axis)
            let horizontal_collision = overlap_x < overlap_y;
            
            // Get collider info to determine if collision should be solid
            let collider_a = world.get_collider(entity_a);
            let collider_b = world.get_collider(entity_b);
            
            let solid_collision = collider_a.map(|c| c.solid).unwrap_or(false) ||
                                 collider_b.map(|c| c.solid).unwrap_or(false);
            
            if solid_collision {
                self.separate_entities(world, entity_a, entity_b, horizontal_collision, overlap_x, overlap_y);
            }
            
            // Print collision info
            println!("Collision! Entity {} <-> Entity {} (overlap: {:.1}x{:.1})", 
                    entity_a, entity_b, overlap_x, overlap_y);
        }
    }
    
    fn separate_entities(&self, world: &mut World, entity_a: Entity, entity_b: Entity, 
                        horizontal_collision: bool, overlap_x: f32, overlap_y: f32) {
        // Get velocities to determine which entity should move
        let vel_a = world.get_velocity(entity_a).copied();
        let vel_b = world.get_velocity(entity_b).copied();
        
        // Separate entities by moving them apart
        let separation = if horizontal_collision { overlap_x } else { overlap_y };
        let half_sep = separation / 2.0;
        
        // Move entities apart based on their velocities
        // We need to handle the borrowing carefully to avoid double mutable borrow
        let pos_a = world.positions.get(&entity_a).copied();
        let pos_b = world.positions.get(&entity_b).copied();
        
        if let (Some(mut pos_a), Some(mut pos_b)) = (pos_a, pos_b) {
            if horizontal_collision {
                // Horizontal separation
                if let Some(vel_a) = vel_a {
                    if vel_a.x > 0.0 {
                        pos_a.x -= half_sep; // Entity A was moving right, push it left
                    } else if vel_a.x < 0.0 {
                        pos_a.x += half_sep; // Entity A was moving left, push it right
                    }
                }
                if let Some(vel_b) = vel_b {
                    if vel_b.x > 0.0 {
                        pos_b.x -= half_sep; // Entity B was moving right, push it left
                    } else if vel_b.x < 0.0 {
                        pos_b.x += half_sep; // Entity B was moving left, push it right
                    }
                }
            } else {
                // Vertical separation
                if let Some(vel_a) = vel_a {
                    if vel_a.y > 0.0 {
                        pos_a.y -= half_sep; // Entity A was moving down, push it up
                    } else if vel_a.y < 0.0 {
                        pos_a.y += half_sep; // Entity A was moving up, push it down
                    }
                }
                if let Some(vel_b) = vel_b {
                    if vel_b.y > 0.0 {
                        pos_b.y -= half_sep; // Entity B was moving down, push it up
                    } else if vel_b.y < 0.0 {
                        pos_b.y += half_sep; // Entity B was moving up, push it down
                    }
                }
            }
            
            // Update positions back to the world
            world.positions.insert(entity_a, pos_a);
            world.positions.insert(entity_b, pos_b);
        }
        
        // Bounce velocities for solid collisions
        if horizontal_collision {
            if let Some(vel_a) = world.velocities.get_mut(&entity_a) {
                vel_a.x *= -0.8; // Reverse and dampen X velocity
            }
            if let Some(vel_b) = world.velocities.get_mut(&entity_b) {
                vel_b.x *= -0.8; // Reverse and dampen X velocity
            }
        } else {
            if let Some(vel_a) = world.velocities.get_mut(&entity_a) {
                vel_a.y *= -0.8; // Reverse and dampen Y velocity
            }
            if let Some(vel_b) = world.velocities.get_mut(&entity_b) {
                vel_b.y *= -0.8; // Reverse and dampen Y velocity
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
        println!("Added system: {}", system.name());
        self.systems.push(system);
    }

    fn update(&mut self, world: &mut World, dt: f32) {
        for system in &mut self.systems {
            system.update(world, dt);
        }
    }
}

// Render System (called separately on redraw)
struct RenderSystem;

impl RenderSystem {
    fn render(world: &World, buffer: &mut Vec<u32>) {
        // Clear the buffer (black background)
        for pixel in buffer.iter_mut() {
            *pixel = 0;
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
                    buffer[index] = sprite.color;
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

                    if screen_x >= 0 && screen_y >= 0 && 
                       (screen_x as usize) < WIDTH && (screen_y as usize) < HEIGHT {
                        
                        // Map back to source texture coordinates
                        let src_x = atlas_sprite.x + (dx as f32 / texture_sprite.scale) as usize;
                        let src_y = atlas_sprite.y + (dy as f32 / texture_sprite.scale) as usize;
                        
                        let pixel = atlas.texture.get_pixel(src_x, src_y);
                        
                        // Only draw non-transparent pixels
                        if (pixel >> 24) > 0 {
                            let index = (screen_y as usize) * WIDTH + (screen_x as usize);
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
        let default_color = 0xFFFF0000; // Red

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

fn main() {
    let mut window = Window::new(
        "Mini Engine with Rust - ECS + Scheduler",
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

    // Load sprite atlas (try to load from file, fall back to sample sprites)
    let atlas = match AssetsLoader::load_png("assets/sprites/atlas.png") {
        Ok(texture) => {
            println!("Loaded atlas from file!");
            let mut atlas = SpriteAtlas::new(texture);
            // You would define your sprite positions here for a real atlas
            atlas.add_sprite("player".to_string(), 0, 0, 32, 32);
            atlas.add_sprite("enemy1".to_string(), 32, 0, 32, 32);
            atlas.add_sprite("enemy2".to_string(), 64, 0, 32, 32);
            atlas
        },
        Err(_) => {
            println!("Could not load atlas file, using sample sprites");
            AssetsLoader::create_sample_sprites()
        }
    };
    
    world.set_sprite_atlas(atlas);

    // Create the player entity with texture sprite and collider
    let player = world.create_entity();
    world.add_position(player, Position { x: 100.0, y: 100.0 });
    world.add_velocity(player, Velocity { x: 0.0, y: 0.0 });
    world.add_texture_sprite(player, TextureSprite::new("player".to_string(), 2.0)); // 2x scale
    world.add_collider(player, Collider::new(32.0, 32.0, true)); // Solid collider

    // Create additional entities for demonstration with texture sprites and colliders
    let enemy1 = world.create_entity();
    world.add_position(enemy1, Position { x: 300.0, y: 200.0 });
    world.add_velocity(enemy1, Velocity { x: 50.0, y: 30.0 }); // Slow movement
    world.add_texture_sprite(enemy1, TextureSprite::new("enemy1".to_string(), 1.5)); // 1.5x scale
    world.add_collider(enemy1, Collider::new(24.0, 24.0, true)); // Solid collider

    let enemy2 = world.create_entity();
    world.add_position(enemy2, Position { x: 500.0, y: 400.0 });
    world.add_velocity(enemy2, Velocity { x: -75.0, y: -45.0 }); // Different movement
    world.add_texture_sprite(enemy2, TextureSprite::new("enemy2".to_string(), 1.0)); // Normal scale
    world.add_collider(enemy2, Collider::new(16.0, 16.0, true)); // Solid collider

    // Create a powerup entity with texture sprite and non-solid collider
    let powerup = world.create_entity();
    world.add_position(powerup, Position { x: 400.0, y: 300.0 });
    world.add_velocity(powerup, Velocity { x: 25.0, y: -25.0 }); // Slow diagonal movement
    world.add_texture_sprite(powerup, TextureSprite::new("powerup".to_string(), 1.0)); // Normal scale
    world.add_collider(powerup, Collider::new(16.0, 16.0, false)); // Non-solid trigger collider

    // Setup scheduler with update systems
    let mut scheduler = Scheduler::new();
    let mut input_system = InputSystem::new(player);
    scheduler.add_system(Box::new(PhysicsSystem));
    scheduler.add_system(Box::new(CollisionSystem));

    // Timing for delta time calculation
    let mut last_time = Instant::now();

    println!("Game started with Texture Assets + AABB Collision System!");
    println!("Use arrow keys to move the player sprite.");
    println!("Watch entities bounce off each other when they collide!");
    println!("Player (red): solid collider, Enemies (green/blue): solid colliders");
    println!("Powerup (yellow): trigger collider (non-solid)");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Handle input for the input system (special case since it needs window reference)
        input_system.set_input(&window);
        input_system.update(&mut world, dt);

        // Update all systems via scheduler
        scheduler.update(&mut world, dt);

        // Render system (called separately on redraw)
        RenderSystem::render(&world, &mut buffer);

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    println!("Closing...");
}

