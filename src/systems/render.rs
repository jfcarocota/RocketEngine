use crate::components::{Sprite, TextureSprite};
use crate::components::texture::SpriteAtlas;
use crate::world::World;
use crate::systems::System;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Render System - handles drawing sprites and textures
pub struct RenderSystem;

impl RenderSystem {
    /// Create a new render system
    pub fn new() -> Self {
        Self
    }

    /// Render a frame to the buffer
    pub fn render_frame(buffer: &mut Vec<u32>, world: &World) {
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

    /// Draw a basic sprite
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

    /// Draw a texture sprite from an atlas
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

    /// Draw a default square for entities without sprites
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

impl System for RenderSystem {
    fn update(&mut self, _world: &mut World, _dt: f32) {
        // Rendering is handled in the main loop
        // This is kept for interface compatibility
    }

    fn name(&self) -> &'static str {
        "RenderSystem"
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new()
    }
}
