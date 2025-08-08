use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SQUARE_SIZE: usize = 20;

// Entity Component System (ECS) Architecture

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

// World contains all components in HashMaps
struct World {
    next_entity_id: Entity,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
}

impl World {
    fn new() -> Self {
        Self {
            next_entity_id: 0,
            positions: HashMap::new(),
            velocities: HashMap::new(),
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
}

// Systems
struct InputSystem;

impl InputSystem {
    fn update(world: &mut World, window: &Window, player_entity: Entity) {
        if let Some(velocity) = world.get_velocity_mut(player_entity) {
            let speed = 200.0; // pixels per second
            
            velocity.x = 0.0;
            velocity.y = 0.0;

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
        }
    }
}

struct PhysicsSystem;

impl PhysicsSystem {
    fn update(world: &mut World, dt: f32) {
        // Get all entities that have both position and velocity
        let entities_with_velocity: Vec<Entity> = world.velocities.keys().cloned().collect();
        
        for entity in entities_with_velocity {
            // First get the velocity value (copy it since it's Copy)
            if let Some(velocity) = world.get_velocity(entity).copied() {
                // Now we can get a mutable reference to position
                if let Some(position) = world.get_position_mut(entity) {
                    // Update position based on velocity and delta time
                    position.x += velocity.x * dt;
                    position.y += velocity.y * dt;

                    // Keep within bounds
                    position.x = position.x.max(0.0).min((WIDTH - SQUARE_SIZE) as f32);
                    position.y = position.y.max(0.0).min((HEIGHT - SQUARE_SIZE) as f32);

                    // Print position updates
                    if velocity.x != 0.0 || velocity.y != 0.0 {
                        println!("Entity {}: Position: {:?}", entity, position);
                    }
                }
            }
        }
    }
}

struct RenderSystem;

impl RenderSystem {
    fn update(world: &World, buffer: &mut Vec<u32>) {
        // Clear the buffer (black background)
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }

        // Render all entities with positions
        for (_entity, position) in &world.positions {
            Self::draw_square(buffer, position.x, position.y);
        }
    }

    fn draw_square(buffer: &mut Vec<u32>, x: f32, y: f32) {
        let square_x = x as usize;
        let square_y = y as usize;

        for y in square_y..square_y + SQUARE_SIZE {
            for x in square_x..square_x + SQUARE_SIZE {
                if x < WIDTH && y < HEIGHT {
                    let index = y * WIDTH + x;
                    buffer[index] = 0xFF0000; // Red color (RGB: FF0000)
                }
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Mini Engine with Rust - ECS Architecture",
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

    // Create the player entity
    let player = world.create_entity();
    world.add_position(player, Position { x: 100.0, y: 100.0 });
    world.add_velocity(player, Velocity { x: 0.0, y: 0.0 });

    // Timing for delta time calculation
    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Run systems
        InputSystem::update(&mut world, &window, player);
        PhysicsSystem::update(&mut world, dt);
        RenderSystem::update(&world, &mut buffer);

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    println!("Closing...");
}