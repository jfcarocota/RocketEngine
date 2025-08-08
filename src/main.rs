use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::time::Instant;

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

// World contains all components in HashMaps
struct World {
    next_entity_id: Entity,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    sprites: HashMap<Entity, Sprite>,
}

impl World {
    fn new() -> Self {
        Self {
            next_entity_id: 0,
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
        }
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
                let sprite_size = world.get_sprite(entity)
                    .map(|s| s.size)
                    .unwrap_or(20); // Default size if no sprite

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

        // Render all entities with positions and sprites
        for (entity, position) in &world.positions {
            if let Some(sprite) = world.get_sprite(*entity) {
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

    fn draw_default_square(buffer: &mut Vec<u32>, x: f32, y: f32) {
        let sprite_x = x as usize;
        let sprite_y = y as usize;
        let default_size = 20;
        let default_color = 0xFF0000; // Red

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

    // Create the player entity with sprite
    let player = world.create_entity();
    world.add_position(player, Position { x: 100.0, y: 100.0 });
    world.add_velocity(player, Velocity { x: 0.0, y: 0.0 });
    world.add_sprite(player, Sprite::new(0xFF0000, 25)); // Red, 25x25 pixels

    // Create additional entities for demonstration
    let enemy1 = world.create_entity();
    world.add_position(enemy1, Position { x: 300.0, y: 200.0 });
    world.add_velocity(enemy1, Velocity { x: 50.0, y: 30.0 }); // Slow movement
    world.add_sprite(enemy1, Sprite::new(0x00FF00, 15)); // Green, 15x15 pixels

    let enemy2 = world.create_entity();
    world.add_position(enemy2, Position { x: 500.0, y: 400.0 });
    world.add_velocity(enemy2, Velocity { x: -75.0, y: -45.0 }); // Different movement
    world.add_sprite(enemy2, Sprite::new(0x0000FF, 30)); // Blue, 30x30 pixels

    // Setup scheduler with update systems
    let mut scheduler = Scheduler::new();
    let mut input_system = InputSystem::new(player);
    scheduler.add_system(Box::new(PhysicsSystem));

    // Timing for delta time calculation
    let mut last_time = Instant::now();

    println!("Game started! Use arrow keys to move the red square.");
    println!("Watch the green and blue squares move automatically!");

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

