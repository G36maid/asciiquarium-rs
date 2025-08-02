use crate::depth;
use crate::entity::{Animation, Entity, EntityId, Position, Sprite, Velocity};
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

/// Water surface entity that creates animated wave patterns
#[derive(Debug, Clone)]
pub struct WaterSurface {
    id: EntityId,
    position: Position,
    layer_index: u8, // 0-3 for the 4 water layers
    animation: Animation,
    alive: bool,
    created_at: Instant,
}

impl WaterSurface {
    /// Create a new water surface layer
    pub fn new(id: EntityId, layer_index: u8, screen_width: u16) -> Self {
        let (sprites, depth) = Self::create_water_layer_sprites(layer_index, screen_width);

        let animation = Animation::new(
            sprites,
            Duration::from_millis(800), // Slow wave animation
            true,                       // Loop forever
        );

        // Position at the top of screen for water surface
        let y = 5.0 + layer_index as f32; // Start at Y=5, each layer below the previous
        let position = Position::new(0.0, y, depth);

        Self {
            id,
            position,
            layer_index,
            animation,
            alive: true,
            created_at: Instant::now(),
        }
    }

    /// Create sprites for a specific water layer with proper tiling
    fn create_water_layer_sprites(layer_index: u8, screen_width: u16) -> (Vec<Sprite>, u8) {
        // Original water surface patterns from asciiquarium.pl
        let water_segments = [
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~", // Layer 0
            "^^^^ ^^^  ^^^   ^^^    ^^^^      ", // Layer 1
            "^^^^      ^^^^     ^^^    ^^     ", // Layer 2
            "^^      ^^^^      ^^^    ^^^^^^  ", // Layer 3
        ];

        let segment = water_segments[layer_index as usize % 4];
        let segment_length = segment.len();

        // Calculate how many times to repeat the segment to fill screen width
        let repeat_count = (screen_width as usize / segment_length) + 2; // +2 for smooth scrolling

        // Create multiple animation frames for wave movement effect
        let mut frames = Vec::new();

        // Create 8 frames by shifting the pattern slightly
        for frame in 0..8 {
            let mut tiled_segment;

            // Shift the pattern by moving the start position
            let shift = frame * 2; // Shift by 2 characters per frame
            let extended_segment = segment.repeat(repeat_count + 1);

            // Extract the shifted portion
            let start_pos = shift % segment_length;
            let end_pos = start_pos + (screen_width as usize) + 10; // Extra for edge cases

            if end_pos <= extended_segment.len() {
                tiled_segment = extended_segment[start_pos..end_pos].to_string();
            } else {
                tiled_segment = extended_segment[start_pos..].to_string();
                let remaining = end_pos - extended_segment.len();
                tiled_segment.push_str(&extended_segment[..remaining]);
            }

            // Trim to exact screen width
            tiled_segment.truncate(screen_width as usize);

            // Create sprite with cyan color mask
            let color_mask = "C".repeat(tiled_segment.len());
            frames.push(Sprite::from_ascii_art(&tiled_segment, Some(&color_mask)));
        }

        // Get appropriate depth for this layer
        let depth = match layer_index {
            0 => depth::water_line_depth(0),
            1 => depth::water_line_depth(1),
            2 => depth::water_line_depth(2),
            3 => depth::water_line_depth(3),
            _ => depth::water_line_depth(0),
        };

        (frames, depth)
    }

    /// Update the water surface to resize for new screen width
    pub fn resize(&mut self, new_screen_width: u16) {
        let (new_sprites, _) = Self::create_water_layer_sprites(self.layer_index, new_screen_width);
        self.animation = Animation::new(new_sprites, Duration::from_millis(800), true);
    }

    /// Get the layer index for this water surface
    pub fn layer_index(&self) -> u8 {
        self.layer_index
    }
}

impl Entity for WaterSurface {
    fn id(&self) -> EntityId {
        self.id
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn velocity(&self) -> Velocity {
        Velocity::zero() // Water surface doesn't move
    }

    fn set_velocity(&mut self, _velocity: Velocity) {
        // Water surface ignores velocity changes
    }

    fn depth(&self) -> u8 {
        self.position.depth
    }

    fn get_current_sprite(&self) -> &Sprite {
        self.animation.get_current_sprite()
    }

    fn update(&mut self, _delta_time: Duration, _screen_bounds: Rect) {
        if !self.alive {
            return;
        }

        // Update animation for wave movement
        self.animation.update();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "water_surface"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_surface_creation() {
        let water = WaterSurface::new(1, 0, 80);

        assert!(water.is_alive());
        assert_eq!(water.entity_type(), "water_surface");
        assert_eq!(water.layer_index(), 0);
        assert_eq!(water.position().y, 5.0); // Layer 0 at Y=5
    }

    #[test]
    fn test_water_surface_layers() {
        let water0 = WaterSurface::new(1, 0, 80);
        let water1 = WaterSurface::new(2, 1, 80);
        let water2 = WaterSurface::new(3, 2, 80);
        let water3 = WaterSurface::new(4, 3, 80);

        // Each layer should be at a different Y position
        assert_eq!(water0.position().y, 5.0);
        assert_eq!(water1.position().y, 6.0);
        assert_eq!(water2.position().y, 7.0);
        assert_eq!(water3.position().y, 8.0);

        // Each layer should have different depth
        assert_ne!(water0.depth(), water1.depth());
        assert_ne!(water1.depth(), water2.depth());
        assert_ne!(water2.depth(), water3.depth());
    }

    #[test]
    fn test_sprite_tiling() {
        let (sprites, _) = WaterSurface::create_water_layer_sprites(0, 80);

        assert!(!sprites.is_empty());

        let first_sprite = &sprites[0];
        assert!(!first_sprite.lines.is_empty());

        // Should create a line roughly 80 characters wide
        let line_length = first_sprite.lines[0].len();
        assert!(line_length >= 80);
        assert!(line_length <= 90); // Some tolerance for tiling
    }
}
