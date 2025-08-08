use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

// Rapier2D imports
use nalgebra::Vector2;

// Use RocketEngine as a library
use rocket_engine::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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
    world.add_position(player, Position::new(100.0, 100.0));
    world.add_velocity(player, Velocity::zero());
    world.add_texture_sprite(player, TextureSprite::with_scale("player", 2.0));
    world.add_physics_body(player, Position::new(100.0, 100.0), 32.0, rapier2d::prelude::RigidBodyType::Dynamic);

    // Create additional entities with physics bodies
    let enemy1 = world.create_entity();
    world.add_position(enemy1, Position::new(300.0, 200.0));
    world.add_velocity(enemy1, Velocity::new(20.0, 15.0));
    world.add_texture_sprite(enemy1, TextureSprite::with_scale("enemy1", 1.5));
    world.add_physics_body(enemy1, Position::new(300.0, 200.0), 24.0, rapier2d::prelude::RigidBodyType::Dynamic);

    let enemy2 = world.create_entity();
    world.add_position(enemy2, Position::new(500.0, 400.0));
    world.add_velocity(enemy2, Velocity::new(-30.0, -20.0));
    world.add_texture_sprite(enemy2, TextureSprite::with_scale("enemy2", 1.0));
    world.add_physics_body(enemy2, Position::new(500.0, 400.0), 16.0, rapier2d::prelude::RigidBodyType::Dynamic);

    let powerup = world.create_entity();
    world.add_position(powerup, Position::new(400.0, 300.0));
    world.add_velocity(powerup, Velocity::new(10.0, -10.0));
    world.add_texture_sprite(powerup, TextureSprite::with_name("powerup"));
    world.add_physics_body(powerup, Position::new(400.0, 300.0), 16.0, rapier2d::prelude::RigidBodyType::Dynamic);

    // Setup scheduler with update systems
    let mut scheduler = Scheduler::new();
    let mut input_system = InputSystem::new(player);
    
    // Add traditional systems
    scheduler.add_system(Box::new(PhysicsSystem::new()));
    
    // Add query-based systems for demonstration
    scheduler.add_query_system(MovementSystem::new());
    scheduler.add_query_system(QueryDemoSystem::new());

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