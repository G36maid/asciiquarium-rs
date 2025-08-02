use crate::entity::{Animation, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

/// A seaweed entity that sways at the bottom of the aquarium
#[derive(Debug, Clone)]
pub struct Seaweed {
    id: EntityId,
    position: Position,
    animation: Animation,
    alive: bool,
    _created_at: Instant,
    die_time: Instant,
    height: u8,
}

impl Seaweed {
    /// Create a new seaweed with random height and position
    pub fn new_random(id: EntityId, screen_bounds: Rect) -> Self {
        let mut rng = rand::thread_rng();

        // Random height between 3-7 characters (original: rand(4) + 3)
        let height = rng.gen_range(3..=6) as u8;

        // Random X position (original: rand(width-2) + 1)
        let x = rng.gen_range(1..(screen_bounds.width.saturating_sub(1)).max(2)) as f32;

        // Y position at bottom minus height (original: height() - height)
        let y = (screen_bounds.height.saturating_sub(height as u16)) as f32;

        Self::new(id, x, y, height)
    }

    /// Create a new seaweed with specific parameters
    pub fn new(id: EntityId, x: f32, y: f32, height: u8) -> Self {
        let (left_sprite, right_sprite) = Self::create_seaweed_sprites(height);

        // Create 2-frame animation for swaying effect
        let frames = vec![left_sprite, right_sprite];

        // Random animation speed (original: rand(.05) + .25 = 0.25 to 0.30)
        let mut rng = rand::thread_rng();
        let anim_speed_secs = rng.gen_range(0.25..0.30);
        let frame_duration = Duration::from_secs_f32(1.0 / anim_speed_secs);

        let animation = Animation::new(frames, frame_duration, true);

        let position = Position::new(x, y, crate::depth::depth::SEAWEED);

        // Seaweed lives for 8-12 minutes (original: rand(4*60) + (8*60))
        let lifetime_secs = rng.gen_range(8 * 60..12 * 60);
        let die_time = Instant::now() + Duration::from_secs(lifetime_secs);

        Self {
            id,
            position,
            animation,
            alive: true,
            _created_at: Instant::now(),
            die_time,
            height,
        }
    }

    /// Create the two seaweed sprites (left and right sway)
    fn create_seaweed_sprites(height: u8) -> (Sprite, Sprite) {
        let mut left_image = String::new();
        let mut right_image = String::new();

        // Build seaweed pattern based on original algorithm
        for i in 1..=height {
            let left_side = (i % 2) == 1; // odd rows go left
            let _right_side = !left_side; // even rows go right

            if left_side {
                left_image.push('(');
                left_image.push('\n');
                right_image.push(' ');
                right_image.push(')');
                right_image.push('\n');
            } else {
                left_image.push(' ');
                left_image.push(')');
                left_image.push('\n');
                right_image.push('(');
                right_image.push('\n');
            }
        }

        // Remove trailing newline
        if left_image.ends_with('\n') {
            left_image.pop();
        }
        if right_image.ends_with('\n') {
            right_image.pop();
        }

        // Create green color masks
        let left_mask = "G".repeat(left_image.lines().map(|line| line.len()).max().unwrap_or(1));
        let right_mask = "G".repeat(
            right_image
                .lines()
                .map(|line| line.len())
                .max()
                .unwrap_or(1),
        );

        let left_sprite = Sprite::from_ascii_art(&left_image, Some(&left_mask));
        let right_sprite = Sprite::from_ascii_art(&right_image, Some(&right_mask));

        (left_sprite, right_sprite)
    }

    /// Get the seaweed height
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Check if seaweed should die due to age
    fn check_age_death(&mut self) {
        if Instant::now() >= self.die_time {
            self.alive = false;
        }
    }
}

impl Entity for Seaweed {
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
        Velocity::zero() // Seaweed doesn't move
    }

    fn set_velocity(&mut self, _velocity: Velocity) {
        // Seaweed ignores velocity changes
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

        // Update animation for swaying effect
        self.animation.update();

        // Check if seaweed should die from old age
        self.check_age_death();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "seaweed"
    }
}

/// Seaweed manager handles seaweed population
pub struct SeaweedManager {
    last_spawn: Instant,
    target_count: usize,
}

impl SeaweedManager {
    pub fn new() -> Self {
        Self {
            last_spawn: Instant::now(),
            target_count: 0,
        }
    }

    /// Calculate target seaweed count based on screen width
    /// Original: int(width / 15)
    pub fn update_target_count(&mut self, screen_bounds: Rect) {
        self.target_count = (screen_bounds.width as usize / 15).max(1);
    }

    /// Check if we should spawn new seaweed
    pub fn should_spawn_seaweed(&mut self, current_count: usize) -> bool {
        let now = Instant::now();

        // Only spawn if below target and enough time has passed
        if current_count < self.target_count
            && now.duration_since(self.last_spawn) > Duration::from_secs(5)
        {
            self.last_spawn = now;
            true
        } else {
            false
        }
    }

    /// Get target seaweed count
    pub fn target_count(&self) -> usize {
        self.target_count
    }
}

impl Default for SeaweedManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seaweed_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let seaweed = Seaweed::new_random(1, screen_bounds);

        assert!(seaweed.is_alive());
        assert_eq!(seaweed.entity_type(), "seaweed");
        assert!(seaweed.height() >= 3 && seaweed.height() <= 6);
        assert_eq!(seaweed.depth(), crate::depth::depth::SEAWEED);
    }

    #[test]
    fn test_seaweed_sprites() {
        let (left, right) = Seaweed::create_seaweed_sprites(3);

        assert!(!left.lines.is_empty());
        assert!(!right.lines.is_empty());
        assert_eq!(left.lines.len(), 3);
        assert_eq!(right.lines.len(), 3);

        // Should have different patterns for left and right
        assert_ne!(left.lines, right.lines);
    }

    #[test]
    fn test_seaweed_manager() {
        let mut manager = SeaweedManager::new();
        let screen_bounds = Rect::new(0, 0, 80, 24);

        manager.update_target_count(screen_bounds);
        assert_eq!(manager.target_count(), 5); // 80/15 = 5.33 -> 5

        // Set last_spawn to past time to allow spawning
        manager.last_spawn = Instant::now() - Duration::from_secs(6);

        // Should want to spawn when count is below target and enough time has passed
        assert!(manager.should_spawn_seaweed(0));

        // Should not spawn immediately again (time constraint)
        assert!(!manager.should_spawn_seaweed(0));
    }

    #[test]
    fn test_seaweed_pattern() {
        let (left, right) = Seaweed::create_seaweed_sprites(4);

        // Check the pattern matches original algorithm
        // Row 1 (i=1, odd): left gets '(', right gets ' )'
        // Row 2 (i=2, even): left gets ' )', right gets '('
        // Row 3 (i=3, odd): left gets '(', right gets ' )'
        // Row 4 (i=4, even): left gets ' )', right gets '('

        assert_eq!(left.lines[0], "(");
        assert_eq!(left.lines[1], " )");
        assert_eq!(left.lines[2], "(");
        assert_eq!(left.lines[3], " )");

        assert_eq!(right.lines[0], " )");
        assert_eq!(right.lines[1], "(");
        assert_eq!(right.lines[2], " )");
        assert_eq!(right.lines[3], "(");
    }
}
