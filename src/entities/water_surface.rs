use crate::depth;
use crate::entity::{Entity, EntityId, Position, Sprite, Velocity};
use ratatui::layout::Rect;
use std::time::Duration;

/// Water surface entity - static, no animation (matches original Perl behavior)
#[derive(Debug, Clone)]
pub struct WaterSurface {
    id: EntityId,
    position: Position,
    layer_index: u8, // 0-3 for the 4 water layers
    sprite: Sprite,
    alive: bool,
}

impl WaterSurface {
    /// Create a new water surface layer
    pub fn new(id: EntityId, layer_index: u8, screen_width: u16) -> Self {
        let (sprite, depth) = Self::create_water_layer_sprite(layer_index, screen_width);

        // Position at the top of screen for water surface
        let y = 5.0 + layer_index as f32; // Start at Y=5, each layer below the previous
        let position = Position::new(0.0, y, depth);

        Self {
            id,
            position,
            layer_index,
            sprite,
            alive: true,
        }
    }

    /// Create a static sprite for a specific water layer with proper tiling
    fn create_water_layer_sprite(layer_index: u8, screen_width: u16) -> (Sprite, u8) {
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
        // Original Perl: $segment_repeat = int($anim->width()/$segment_size) + 1;
        let repeat_count = (screen_width as usize / segment_length) + 1;

        // Tile the segment to fill the screen width
        let tiled_segment = segment.repeat(repeat_count);

        // Create sprite with cyan color mask
        let color_mask = "C".repeat(tiled_segment.len());
        let sprite = Sprite::from_ascii_art(&tiled_segment, Some(&color_mask));

        // Get appropriate depth for this layer
        let depth = match layer_index {
            0 => depth::water_line_depth(0),
            1 => depth::water_line_depth(1),
            2 => depth::water_line_depth(2),
            3 => depth::water_line_depth(3),
            _ => depth::water_line_depth(0),
        };

        (sprite, depth)
    }

    /// Update the water surface to resize for new screen width
    pub fn resize(&mut self, new_screen_width: u16) {
        let (new_sprite, _) = Self::create_water_layer_sprite(self.layer_index, new_screen_width);
        self.sprite = new_sprite;
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
        &self.sprite
    }

    fn update(&mut self, _delta_time: Duration, _screen_bounds: Rect) {
        // Water surface is completely static - no updates needed
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
        let (sprite, _) = WaterSurface::create_water_layer_sprite(0, 80);

        assert!(!sprite.lines.is_empty());

        // Should create a line at least 80 characters wide
        let line_length = sprite.lines[0].len();
        assert!(line_length >= 80);
    }

    #[test]
    fn test_water_is_static() {
        let mut water = WaterSurface::new(1, 0, 80);
        let original_sprite_lines = water.sprite.lines.clone();

        // Update multiple times
        water.update(Duration::from_secs(1), Rect::new(0, 0, 80, 24));
        water.update(Duration::from_secs(1), Rect::new(0, 0, 80, 24));
        water.update(Duration::from_secs(1), Rect::new(0, 0, 80, 24));

        // Sprite should not change - water is static
        assert_eq!(water.sprite.lines, original_sprite_lines);
    }
}
