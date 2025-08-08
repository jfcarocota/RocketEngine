use std::collections::HashMap;
use crate::components::*;
use rapier2d::prelude::*;
use nalgebra::Vector2;

/// World contains all components in HashMaps + Rapier2D Physics World
pub struct World {
    pub next_entity_id: Entity,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub sprites: HashMap<Entity, Sprite>,
    pub texture_sprites: HashMap<Entity, TextureSprite>,
    pub sprite_atlas: Option<SpriteAtlas>,
    
    // Rapier2D Physics World
    pub physics_world: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub event_handler: (),
    
    // Map Entity IDs to Rapier RigidBodyHandle
    pub entity_to_body: HashMap<Entity, RigidBodyHandle>,
    pub body_to_entity: HashMap<RigidBodyHandle, Entity>,
}

impl World {
    /// Create a new world
    pub fn new() -> Self {
        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = 1.0 / 60.0; // 60 FPS
        
        // Configure CCD parameters for better fast-object collision handling
        integration_parameters.max_ccd_substeps = 4; // More substeps for better CCD
        integration_parameters.min_ccd_dt = 1.0 / 240.0; // Smaller minimum CCD timestep for accuracy

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

    /// Set the sprite atlas
    pub fn set_sprite_atlas(&mut self, atlas: SpriteAtlas) {
        self.sprite_atlas = Some(atlas);
    }

    /// Create a new entity
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        entity
    }

    /// Add position component to an entity
    pub fn add_position(&mut self, entity: Entity, position: Position) {
        self.positions.insert(entity, position);
    }

    /// Add velocity component to an entity
    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }

    /// Add sprite component to an entity
    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprites.insert(entity, sprite);
    }

    /// Add texture sprite component to an entity
    pub fn add_texture_sprite(&mut self, entity: Entity, texture_sprite: TextureSprite) {
        self.texture_sprites.insert(entity, texture_sprite);
    }

    /// Create a physics body for this entity
    pub fn add_physics_body(&mut self, entity: Entity, position: Position, size: f32, body_type: RigidBodyType) {
        // Get initial velocity from ECS if it exists
        let initial_velocity = self.velocities.get(&entity)
            .map(|v| Vector2::new(v.x, v.y))
            .unwrap_or_else(Vector2::zeros);

        // Create Rapier rigid body with better collision settings and CCD
        let rigid_body = RigidBodyBuilder::new(body_type)
            .translation(Vector2::new(position.x, position.y))
            .linvel(initial_velocity)
            .linear_damping(0.1)  // Slight damping to prevent infinite bouncing
            .angular_damping(0.1) // Prevent excessive spinning
            .can_sleep(false)     // Keep bodies active for better collision response
            .ccd_enabled(true)    // Enable Continuous Collision Detection
            .build();

        let body_handle = self.physics_world.insert(rigid_body);

        // Calculate appropriate CCD thickness based on object size and potential velocity
        // Note: In Rapier2D, CCD thickness is automatically calculated based on collider shape
        // The thickness calculation here serves as documentation for the expected scale
        let _ccd_thickness = (size / 4.0).max(2.0); // At least 2.0, scaled with object size
        
        // Create collider with better collision response settings
        // Note: CCD is enabled at the rigid body level, not collider level in Rapier2D
        let collider = ColliderBuilder::cuboid(size / 2.0, size / 2.0)
            .restitution(0.9)     // High bounciness for visible collisions
            .friction(0.2)        // Low friction for smooth movement
            .density(1.0)         // Consistent mass
            .restitution_combine_rule(CoefficientCombineRule::Max) // Use max restitution
            .friction_combine_rule(CoefficientCombineRule::Average) // Average friction
            .build();

        self.collider_set.insert_with_parent(collider, body_handle, &mut self.physics_world);

        // Map entity to body handle
        self.entity_to_body.insert(entity, body_handle);
        self.body_to_entity.insert(body_handle, entity);
    }

    /// Get position component
    pub fn get_position(&self, entity: Entity) -> Option<&Position> {
        self.positions.get(&entity)
    }

    /// Get mutable position component
    pub fn get_position_mut(&mut self, entity: Entity) -> Option<&mut Position> {
        self.positions.get_mut(&entity)
    }

    /// Get velocity component
    pub fn get_velocity(&self, entity: Entity) -> Option<&Velocity> {
        self.velocities.get(&entity)
    }

    /// Get mutable velocity component
    pub fn get_velocity_mut(&mut self, entity: Entity) -> Option<&mut Velocity> {
        self.velocities.get_mut(&entity)
    }

    /// Get sprite component
    pub fn get_sprite(&self, entity: Entity) -> Option<&Sprite> {
        self.sprites.get(&entity)
    }

    /// Get texture sprite component
    pub fn get_texture_sprite(&self, entity: Entity) -> Option<&TextureSprite> {
        self.texture_sprites.get(&entity)
    }

    /// Set velocity of a physics body
    pub fn set_physics_velocity(&mut self, entity: Entity, velocity: Vector2<f32>) {
        if let Some(&body_handle) = self.entity_to_body.get(&entity) {
            if let Some(body) = self.physics_world.get_mut(body_handle) {
                body.set_linvel(velocity, true);
            }
        }
    }

    /// Sync positions from physics world to ECS
    pub fn sync_positions_from_physics(&mut self) {
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

    /// Step the physics simulation
    pub fn step_physics(&mut self) {
        // Step the physics world with better collision handling
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

        // Check for collisions and log them
        self.check_collisions();

        // Sync physics positions back to ECS
        self.sync_positions_from_physics();
        
        // Apply boundary constraints to keep entities on screen
        self.apply_boundary_constraints();
    }

    /// Check for active collisions and log them
    fn check_collisions(&self) {
        // Check contact pairs for active collisions
        for contact_pair in self.narrow_phase.contact_pairs() {
            if contact_pair.has_any_active_contact {
                // Get the colliders involved
                let collider1 = contact_pair.collider1;
                let collider2 = contact_pair.collider2;
                
                // Find the entities associated with these colliders
                if let (Some(body1), Some(body2)) = (
                    self.collider_set.get(collider1).and_then(|c| c.parent()),
                    self.collider_set.get(collider2).and_then(|c| c.parent())
                ) {
                    if let (Some(&entity1), Some(&entity2)) = (
                        self.body_to_entity.get(&body1),
                        self.body_to_entity.get(&body2)
                    ) {
                        // Get positions for logging
                        if let (Some(pos1), Some(pos2)) = (
                            self.positions.get(&entity1),
                            self.positions.get(&entity2)
                        ) {
                            println!("Collision! Entity {} at ({:.1}, {:.1}) <-> Entity {} at ({:.1}, {:.1})",
                                entity1, pos1.x, pos1.y, entity2, pos2.x, pos2.y);
                        }
                    }
                }
            }
        }
    }

    /// Keep entities within screen bounds
    fn apply_boundary_constraints(&mut self) {
        const SCREEN_WIDTH: f32 = 800.0;
        const SCREEN_HEIGHT: f32 = 600.0;
        const MARGIN: f32 = 16.0; // Half of typical sprite size

        for (&_entity, &body_handle) in &self.entity_to_body {
            if let Some(body) = self.physics_world.get_mut(body_handle) {
                let mut translation = *body.translation();
                let mut velocity = *body.linvel();
                let mut changed = false;

                // Left boundary
                if translation.x < MARGIN {
                    translation.x = MARGIN;
                    velocity.x = velocity.x.abs(); // Bounce right
                    changed = true;
                }
                // Right boundary  
                else if translation.x > SCREEN_WIDTH - MARGIN {
                    translation.x = SCREEN_WIDTH - MARGIN;
                    velocity.x = -velocity.x.abs(); // Bounce left
                    changed = true;
                }

                // Top boundary
                if translation.y < MARGIN {
                    translation.y = MARGIN;
                    velocity.y = velocity.y.abs(); // Bounce down
                    changed = true;
                }
                // Bottom boundary
                else if translation.y > SCREEN_HEIGHT - MARGIN {
                    translation.y = SCREEN_HEIGHT - MARGIN;
                    velocity.y = -velocity.y.abs(); // Bounce up
                    changed = true;
                }

                if changed {
                    body.set_translation(translation, true);
                    body.set_linvel(velocity, true);
                }
            }
        }
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.next_entity_id as usize
    }

    /// Remove an entity and all its components
    pub fn remove_entity(&mut self, entity: Entity) {
        // Remove physics body if exists
        if let Some(&body_handle) = self.entity_to_body.get(&entity) {
            self.physics_world.remove(
                body_handle,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true
            );
            self.entity_to_body.remove(&entity);
            self.body_to_entity.remove(&body_handle);
        }

        // Remove components
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.sprites.remove(&entity);
        self.texture_sprites.remove(&entity);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
