use serde::{Deserialize, Serialize};
use rapier2d::prelude::RigidBodyType;
use crate::components::{Position, Velocity, TextureSprite};
use crate::world::World;
use std::collections::HashMap;

/// Serializable entity data for scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub name: Option<String>,
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
    pub texture_sprite: Option<TextureSprite>,
    pub physics_body: Option<PhysicsBodyData>,
}

/// Serializable physics body configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBodyData {
    pub size: f32,
    pub body_type: PhysicsBodyType,
}

/// Serializable version of RigidBodyType for RON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicsBodyType {
    Dynamic,
    Fixed,
    KinematicPositionBased,
    KinematicVelocityBased,
}

impl From<PhysicsBodyType> for RigidBodyType {
    fn from(body_type: PhysicsBodyType) -> Self {
        match body_type {
            PhysicsBodyType::Dynamic => RigidBodyType::Dynamic,
            PhysicsBodyType::Fixed => RigidBodyType::Fixed,
            PhysicsBodyType::KinematicPositionBased => RigidBodyType::KinematicPositionBased,
            PhysicsBodyType::KinematicVelocityBased => RigidBodyType::KinematicVelocityBased,
        }
    }
}

/// Scene data structure containing all entities
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub description: Option<String>,
    pub entities: Vec<EntityData>,
}

/// Scene loader for spawning entities from scene files
pub struct SceneLoader;

impl SceneLoader {
    /// Load a scene from a RON file
    pub fn load_from_ron(file_path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file_path)?;
        let scene: Scene = ron::from_str(&contents)?;
        Ok(scene)
    }

    /// Load a scene from a JSON file
    pub fn load_from_json(file_path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file_path)?;
        let scene: Scene = serde_json::from_str(&contents)?;
        Ok(scene)
    }

    /// Save a scene to a RON file
    pub fn save_to_ron(scene: &Scene, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = ron::ser::to_string_pretty(scene, ron::ser::PrettyConfig::default())?;
        std::fs::write(file_path, contents)?;
        Ok(())
    }

    /// Save a scene to a JSON file
    pub fn save_to_json(scene: &Scene, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string_pretty(scene)?;
        std::fs::write(file_path, contents)?;
        Ok(())
    }

    /// Spawn all entities from a scene into the world
    pub fn spawn_scene(scene: &Scene, world: &mut World) -> HashMap<String, crate::components::Entity> {
        let mut entity_map = HashMap::new();
        
        println!("Loading scene: {}", scene.name);
        if let Some(description) = &scene.description {
            println!("Description: {}", description);
        }

        for (index, entity_data) in scene.entities.iter().enumerate() {
            let entity = world.create_entity();
            
            // Add position component if specified
            if let Some(position) = entity_data.position {
                world.add_position(entity, position);
            }

            // Add velocity component if specified
            if let Some(velocity) = entity_data.velocity {
                world.add_velocity(entity, velocity);
            }

            // Add texture sprite component if specified
            if let Some(ref texture_sprite) = entity_data.texture_sprite {
                world.add_texture_sprite(entity, texture_sprite.clone());
            }

            // Add physics body if specified
            if let Some(ref physics_data) = entity_data.physics_body {
                let position = entity_data.position.unwrap_or(Position::new(0.0, 0.0));
                world.add_physics_body(
                    entity,
                    position,
                    physics_data.size,
                    physics_data.body_type.clone().into(),
                );
            }

            // Map entity by name or index
            let entity_name = entity_data.name
                .clone()
                .unwrap_or_else(|| format!("entity_{}", index));
            entity_map.insert(entity_name.clone(), entity);

            println!("Spawned entity '{}' (ID: {})", entity_name, entity);
        }

        println!("Scene loaded successfully! Spawned {} entities", scene.entities.len());
        entity_map
    }

    /// Create a scene from current world state (for saving/debugging)
    pub fn create_scene_from_world(_world: &World, name: String, description: Option<String>) -> Scene {
        let entities = Vec::new();

        // Note: This is a simplified version that would need to iterate through all entities
        // In a real implementation, you'd want to iterate through all entities in the world
        // For now, this serves as a template for the scene structure

        Scene {
            name,
            description,
            entities,
        }
    }
}

/// Make Position serializable
impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Position::new(x, y))
    }
}

/// Make Velocity serializable
impl Serialize for Velocity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Velocity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Velocity::new(x, y))
    }
}

/// Make TextureSprite serializable
impl Serialize for TextureSprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TextureSprite", 2)?;
        state.serialize_field("atlas_name", &self.atlas_name)?;
        state.serialize_field("scale", &self.scale)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TextureSprite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            AtlasName,
            Scale,
        }

        struct TextureSpriteVisitor;

        impl<'de> serde::de::Visitor<'de> for TextureSpriteVisitor {
            type Value = TextureSprite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct TextureSprite")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TextureSprite, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut atlas_name = None;
                let mut scale = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::AtlasName => {
                            if atlas_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("atlas_name"));
                            }
                            atlas_name = Some(map.next_value()?);
                        }
                        Field::Scale => {
                            if scale.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale = Some(map.next_value()?);
                        }
                    }
                }
                let atlas_name = atlas_name.ok_or_else(|| serde::de::Error::missing_field("atlas_name"))?;
                let scale = scale.unwrap_or(1.0);
                Ok(TextureSprite::new(atlas_name, scale))
            }
        }

        const FIELDS: &'static [&'static str] = &["atlas_name", "scale"];
        deserializer.deserialize_struct("TextureSprite", FIELDS, TextureSpriteVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_serialization() {
        let scene = Scene {
            name: "Test Scene".to_string(),
            description: Some("A test scene for unit testing".to_string()),
            entities: vec![
                EntityData {
                    name: Some("player".to_string()),
                    position: Some(Position::new(100.0, 100.0)),
                    velocity: Some(Velocity::new(0.0, 0.0)),
                    texture_sprite: Some(TextureSprite::with_scale("player", 2.0)),
                    physics_body: Some(PhysicsBodyData {
                        size: 32.0,
                        body_type: PhysicsBodyType::Dynamic,
                    }),
                },
                EntityData {
                    name: Some("enemy".to_string()),
                    position: Some(Position::new(300.0, 200.0)),
                    velocity: Some(Velocity::new(20.0, 15.0)),
                    texture_sprite: Some(TextureSprite::with_name("enemy1")),
                    physics_body: Some(PhysicsBodyData {
                        size: 24.0,
                        body_type: PhysicsBodyType::Dynamic,
                    }),
                },
            ],
        };

        // Test RON serialization
        let ron_string = ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::default()).unwrap();
        println!("Serialized scene:\n{}", ron_string);

        // Test deserialization
        let deserialized: Scene = ron::from_str(&ron_string).unwrap();
        assert_eq!(deserialized.name, scene.name);
        assert_eq!(deserialized.entities.len(), scene.entities.len());
    }
}
