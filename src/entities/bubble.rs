use crate::entity::{Animation, Entity, EntityId, Position, Sprite, Velocity};
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

/// A bubble entity that rises from fish to the water surface
#[derive(Debug)]
pub struct Bubble {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    animation: Animation,
    alive: bool,
    created_at: Instant,
}

impl Bubble {
    /// Create a new bubble at the specified position
    pub fn new(id: EntityId, position: Position) -> Self {
        // Create the 5-frame bubble animation: '.', 'o', 'O', 'O', 'O'
        // Use cyan color mask for all frames
        let frames = vec![
            Sprite::from_ascii_art(".", Some("C")),
            Sprite::from_ascii_art("o", Some("C")),
            Sprite::from_ascii_art("O", Some("C")),
            Sprite::from_ascii_art("O", Some("C")),
            Sprite::from_ascii_art("O", Some("C")),
        ];

        let animation = Animation::new(
            frames,
            Duration::from_millis(200), // Each frame lasts 200ms
            false,                      // Don't loop - bubble grows then stays at max size
        );

        // Bubbles rise with slight random variation
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let horizontal_drift = rng.gen_range(-0.1..0.1);
        let velocity = Velocity::new(horizontal_drift, -1.0); // Rise upward with slight horizontal drift

        Self {
            id,
            position,
            velocity,
            animation,
            alive: true,
            created_at: Instant::now(),
        }
    }

    /// Create a bubble from a fish position with direction awareness
    pub fn from_fish_position(
        id: EntityId,
        fish_pos: Position,
        fish_sprite_width: u16,
        fish_moving_right: bool,
    ) -> Self {
        // Position bubble at fish's mouth/front based on direction
        let bubble_x = if fish_moving_right {
            fish_pos.x + fish_sprite_width as f32 // Right side for right-moving fish
        } else {
            fish_pos.x // Left side for left-moving fish
        };

        let bubble_y = fish_pos.y + 1.0; // Slightly below fish center

        // Bubble appears one depth layer above the fish
        let bubble_depth = fish_pos.depth.saturating_sub(1);

        let bubble_position = Position::new(bubble_x, bubble_y, bubble_depth);
        Self::new(id, bubble_position)
    }

    /// Check if bubble has reached water surface and should pop
    fn check_surface_collision(&mut self) {
        // Water surface is around Y=5-9 based on original code
        let water_surface_y = 9.0;
        if self.position.y <= water_surface_y {
            self.alive = false;
        }
    }

    /// Check if bubble is too old and should disappear
    fn check_age_limit(&mut self) {
        // Bubbles live for maximum 30 seconds (very generous)
        if self.created_at.elapsed() > Duration::from_secs(30) {
            self.alive = false;
        }
    }
}

impl Entity for Bubble {
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
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    fn depth(&self) -> u8 {
        self.position.depth
    }

    fn get_current_sprite(&self) -> &Sprite {
        self.animation.get_current_sprite()
    }

    fn update(&mut self, delta_time: Duration, screen_bounds: Rect) {
        if !self.alive {
            return;
        }

        // Update animation
        self.animation.update();

        // Update position based on velocity
        let speed_multiplier = 60.0; // Scale for 60 FPS
        self.position.x += self.velocity.dx * delta_time.as_secs_f32() * speed_multiplier;
        self.position.y += self.velocity.dy * delta_time.as_secs_f32() * speed_multiplier;

        // Add slight buoyancy effect - bubbles accelerate upward slightly
        self.velocity.dy -= 0.01; // Small upward acceleration

        // Limit maximum rise speed
        self.velocity.dy = self.velocity.dy.max(-2.0);

        // Check if bubble should die
        self.check_surface_collision();
        self.check_age_limit();

        // Check if bubble is off-screen horizontally
        let pos = self.position.to_screen_coords();
        if pos.0 > screen_bounds.width + 5 || (pos.0 as i32) < -5 {
            self.alive = false;
        }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "bubble"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::depth;

    #[test]
    fn test_bubble_creation() {
        let position = Position::new(10.0, 15.0, depth::random_fish_depth());
        let bubble = Bubble::new(1, position);

        assert!(bubble.is_alive());
        assert_eq!(bubble.entity_type(), "bubble");
        assert_eq!(bubble.position().x, 10.0);
        assert_eq!(bubble.position().y, 15.0);
        assert!(bubble.velocity().dy < 0.0); // Should be moving upward
    }

    #[test]
    fn test_bubble_from_fish() {
        use crate::depth;
        let fish_pos = Position::new(20.0, 12.0, depth::random_fish_depth());
        let bubble = Bubble::from_fish_position(1, fish_pos, 6, true); // Fish moving right

        // Bubble should be positioned at fish's mouth area
        assert_eq!(bubble.position().x, 26.0); // 20 + 6 (right side)
        assert_eq!(bubble.position().y, 13.0); // 12 + 1 (slightly below)
        assert_eq!(bubble.depth(), fish_pos.depth - 1); // One layer above fish
    }

    #[test]
    fn test_bubble_animation() {
        use crate::depth;
        let position = Position::new(10.0, 15.0, depth::random_fish_depth());
        let bubble = Bubble::new(1, position);

        let initial_sprite = bubble.get_current_sprite();
        assert_eq!(initial_sprite.lines[0], "."); // Should start with small bubble
    }

    #[test]
    fn test_bubble_surface_collision() {
        use crate::depth;
        let position = Position::new(10.0, 8.0, depth::random_fish_depth()); // Near surface
        let mut bubble = Bubble::new(1, position);

        bubble.update(Duration::from_millis(100), Rect::new(0, 0, 80, 24));

        // Bubble should die when it reaches the surface
        if bubble.position().y <= 9.0 {
            assert!(!bubble.is_alive());
        }
    }

    #[test]
    fn test_bubble_movement() {
        use crate::depth;
        let position = Position::new(10.0, 15.0, depth::random_fish_depth());
        let mut bubble = Bubble::new(1, position);

        let initial_y = bubble.position().y;
        bubble.update(Duration::from_millis(16), Rect::new(0, 0, 80, 24)); // ~60 FPS

        // Bubble should move upward
        assert!(bubble.position().y < initial_y);
    }
}
