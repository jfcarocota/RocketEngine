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
