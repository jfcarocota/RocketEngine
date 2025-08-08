// Component module exports
pub mod position;
pub mod velocity;
pub mod sprite;
pub mod texture_sprite;
pub mod texture;
pub mod atlas;

// Re-export all components for easy access
pub use position::Position;
pub use velocity::Velocity;
pub use sprite::Sprite;
pub use texture_sprite::TextureSprite;
pub use texture::{Texture, AtlasSprite, SpriteAtlas};
pub use atlas::AssetsLoader;

// Entity type definition
pub type Entity = u32;
