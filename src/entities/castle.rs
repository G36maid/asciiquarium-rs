use crate::entity::{Entity, EntityId, Position, Sprite, Velocity};
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

/// A castle entity that serves as background decoration
#[derive(Debug, Clone)]
pub struct Castle {
    id: EntityId,
    position: Position,
    sprite: Sprite,
    alive: bool,
    _created_at: Instant,
}

impl Castle {
    /// Create a new castle positioned at bottom-right of screen
    pub fn new(id: EntityId, screen_bounds: Rect) -> Self {
        let castle_sprite = Self::create_castle_sprite();

        // Position at bottom-right (original: width-32, height-13)
        let x = screen_bounds.width.saturating_sub(32) as f32;
        let y = screen_bounds.height.saturating_sub(13) as f32;

        let position = Position::new(x, y, crate::depth::CASTLE);

        Self {
            id,
            position,
            sprite: castle_sprite,
            alive: true,
            _created_at: Instant::now(),
        }
    }

    /// Create a new castle with specific position
    pub fn new_at_position(id: EntityId, x: f32, y: f32) -> Self {
        let castle_sprite = Self::create_castle_sprite();
        let position = Position::new(x, y, crate::depth::CASTLE);

        Self {
            id,
            position,
            sprite: castle_sprite,
            alive: true,
            _created_at: Instant::now(),
        }
    }

    /// Create the castle sprite with ASCII art and color mask
    fn create_castle_sprite() -> Sprite {
        // Castle ASCII art from original Perl implementation
        let castle_image = r#"               T~~
               |
              /^\
             /   \
 _   _   _  /     \  _   _   _
[ ]_[ ]_[ ]/ _   _ \[ ]_[ ]_[ ]
|_=__-_ =_|_[ ]_[ ]_|_=-___-__|
 | _- =  | =_ = _    |= _=   |
 | =_    |= - ___    | =_ =  |
 |=  []- |-  /| |\   |=_ =[] |
 |- =_   | =| | | |  |- = -  |
 |_______|__|_|_|_|__|_______|"#;

        // Color mask: R=red, y=yellow, space=default (black)
        let castle_mask = r#"                RR

              yyy
             y   y
            y     y
           y       y



              yyy
             yy yy
            y y y y
            yyyyyyy"#;

        Sprite::from_ascii_art(castle_image, Some(castle_mask))
    }

    /// Get the castle width (for positioning calculations)
    pub fn width() -> u16 {
        32 // Castle is 32 characters wide
    }

    /// Get the castle height (for positioning calculations)
    pub fn height() -> u16 {
        13 // Castle is 13 lines tall
    }

    /// Check if castle should be repositioned due to screen resize
    pub fn should_reposition(&self, screen_bounds: Rect) -> bool {
        let expected_x = screen_bounds.width.saturating_sub(32) as f32;
        let expected_y = screen_bounds.height.saturating_sub(13) as f32;

        // Reposition if current position doesn't match expected bottom-right position
        (self.position.x - expected_x).abs() > 0.1 || (self.position.y - expected_y).abs() > 0.1
    }

    /// Update castle position for new screen size
    pub fn reposition_for_screen(&mut self, screen_bounds: Rect) {
        let x = screen_bounds.width.saturating_sub(32) as f32;
        let y = screen_bounds.height.saturating_sub(13) as f32;
        self.position.x = x;
        self.position.y = y;
    }
}

impl Entity for Castle {
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
        Velocity::zero() // Castle doesn't move
    }

    fn set_velocity(&mut self, _velocity: Velocity) {
        // Castle ignores velocity changes
    }

    fn depth(&self) -> u8 {
        self.position.depth
    }

    fn get_current_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn update(&mut self, _delta_time: Duration, _screen_bounds: Rect) {
        // Castle is static and doesn't need updates
        // (Screen repositioning is handled externally by the app)
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "castle"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castle_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let castle = Castle::new(1, screen_bounds);

        assert!(castle.is_alive());
        assert_eq!(castle.entity_type(), "castle");
        assert_eq!(castle.depth(), crate::depth::CASTLE);

        // Should be positioned at bottom-right
        assert_eq!(castle.position().x, 48.0); // 80 - 32 = 48
        assert_eq!(castle.position().y, 11.0); // 24 - 13 = 11
    }

    #[test]
    fn test_castle_sprite() {
        let castle = Castle::new(1, Rect::new(0, 0, 80, 24));
        let sprite = castle.get_current_sprite();

        assert!(!sprite.lines.is_empty());
        assert_eq!(sprite.lines.len(), 12); // Castle should have 12 lines

        // Check that first line contains the castle top
        assert!(sprite.lines[0].contains("T~~"));

        // Check that last line contains the castle base
        assert!(sprite.lines.last().unwrap().contains("_______"));
    }

    #[test]
    fn test_castle_dimensions() {
        assert_eq!(Castle::width(), 32);
        assert_eq!(Castle::height(), 13);
    }

    #[test]
    fn test_castle_repositioning() {
        let mut castle = Castle::new(1, Rect::new(0, 0, 80, 24));

        // Should need repositioning for different screen size
        assert!(castle.should_reposition(Rect::new(0, 0, 100, 30)));

        // Reposition castle
        castle.reposition_for_screen(Rect::new(0, 0, 100, 30));
        assert_eq!(castle.position().x, 68.0); // 100 - 32 = 68
        assert_eq!(castle.position().y, 17.0); // 30 - 13 = 17
    }
}
