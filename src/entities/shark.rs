use crate::entity::{Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

/// A shark entity that hunts fish across the screen
#[derive(Debug, Clone)]
pub struct Shark {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    right_sprite: Sprite,
    left_sprite: Sprite,
    alive: bool,
    _created_at: Instant,
    teeth_id: Option<EntityId>, // ID of associated teeth entity
}

impl Shark {
    /// Create a new shark with random direction and position
    pub fn new_random(id: EntityId, screen_bounds: Rect) -> Self {
        let mut rng = rand::thread_rng();
        let direction = if rng.gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };

        let (right_sprite, left_sprite) = Self::create_shark_sprites();

        // Position based on direction (spawn off-screen)
        let (x, velocity) = match direction {
            Direction::Right => {
                let x = -53.0; // Spawn off left edge
                let velocity = Velocity::new(2.0, 0.0); // Move right
                (x, velocity)
            }
            Direction::Left => {
                let x = (screen_bounds.width + 2) as f32; // Spawn off right edge
                let velocity = Velocity::new(-2.0, 0.0); // Move left
                (x, velocity)
            }
        };

        // Random Y position (original: rand(height - (10 + 9)) + 9)
        let y = rng.gen_range(9..(screen_bounds.height.saturating_sub(19)).max(10)) as f32;

        let position = Position::new(x, y, crate::depth::depth::SHARK);

        Self {
            id,
            position,
            velocity,
            direction,
            right_sprite,
            left_sprite,
            alive: true,
            _created_at: Instant::now(),
            teeth_id: None,
        }
    }

    /// Create a new shark with specific parameters
    pub fn new(id: EntityId, position: Position, velocity: Velocity, direction: Direction) -> Self {
        let (right_sprite, left_sprite) = Self::create_shark_sprites();

        Self {
            id,
            position,
            velocity,
            direction,
            right_sprite,
            left_sprite,
            alive: true,
            _created_at: Instant::now(),
            teeth_id: None,
        }
    }

    /// Create the shark sprites (right and left facing)
    fn create_shark_sprites() -> (Sprite, Sprite) {
        // Right-facing shark (original direction 0)
        let right_art = r#"
                              __
                             ( `\
  ,??????????????????????????)   `\
;' `.????????????????????????(     `\__
 ;   `.?????????????__..---''          `~~~~-._
  `.   `.____...--''                       (b  `--._
    >                     _.-'      .((      ._     )
  .`.-`--...__         .-'     -.___.....-(|/|/|/|/'
 ;.'?????????`. ...----`.___.',,,_______......---'
 '???????????'-'"#;

        let right_mask = r#"





                                           cR

                                          cWWWWWWWW


"#;

        // Left-facing shark (original direction 1)
        let left_art = r#"
                     __
                    /' )
                  /'   (??????????????????????????,
              __/'     )????????????????????????.' `;
      _.-~~~~'          ``---..__?????????????.'   ;
 _.--'  b)                       ``--...____.'   .'
(     _.      )).      `-._                     <
 `\|\|\|\|)-.....___.-     `-.         __...--'-.'.
   `---......_______,,,`.___.'----... .'?????????`.;
                                     `-`???????????`"#;

        let left_mask = r#"





        Rc

  WWWWWWWWc


"#;

        let right_sprite = Sprite::from_ascii_art(right_art, Some(right_mask));
        let left_sprite = Sprite::from_ascii_art(left_art, Some(left_mask));

        (right_sprite, left_sprite)
    }

    /// Get the teeth position for this shark
    pub fn get_teeth_position(&self) -> Position {
        let teeth_offset = match self.direction {
            Direction::Right => (44.0, 7.0), // Original: teeth_x = -9, shark_x = -53, so offset = 44
            Direction::Left => (-44.0, 7.0), // Original: teeth_x = x + 9, so offset = 9
        };

        Position::new(
            self.position.x + teeth_offset.0,
            self.position.y + teeth_offset.1,
            self.position.depth + 1, // Teeth are slightly in front
        )
    }

    /// Set the associated teeth entity ID
    pub fn set_teeth_id(&mut self, teeth_id: EntityId) {
        self.teeth_id = Some(teeth_id);
    }

    /// Get the associated teeth entity ID
    pub fn get_teeth_id(&self) -> Option<EntityId> {
        self.teeth_id
    }

    /// Get shark dimensions for collision calculations
    pub fn get_dimensions() -> (u16, u16) {
        (53, 11) // Shark is roughly 53 wide, 11 tall
    }

    /// Check if shark has moved off screen
    fn is_off_screen(&self, screen_bounds: Rect) -> bool {
        match self.direction {
            Direction::Right => self.position.x > (screen_bounds.width + 10) as f32,
            Direction::Left => self.position.x < -60.0,
        }
    }
}

impl Entity for Shark {
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
        match self.direction {
            Direction::Right => &self.right_sprite,
            Direction::Left => &self.left_sprite,
        }
    }

    fn update(&mut self, delta_time: Duration, screen_bounds: Rect) {
        if !self.alive {
            return;
        }

        // Update position based on velocity
        let dt_secs = delta_time.as_secs_f32();
        self.position.x += self.velocity.dx * dt_secs * 60.0; // Scale for 60 FPS equivalent
        self.position.y += self.velocity.dy * dt_secs * 60.0;

        // Check if shark has moved off screen
        if self.is_off_screen(screen_bounds) {
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
        "shark"
    }
}

/// Shark teeth entity for collision detection
#[derive(Debug, Clone)]
pub struct SharkTeeth {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    sprite: Sprite,
    alive: bool,
    _created_at: Instant,
    shark_id: EntityId, // ID of associated shark
}

impl SharkTeeth {
    /// Create new shark teeth
    pub fn new(id: EntityId, position: Position, velocity: Velocity, shark_id: EntityId) -> Self {
        let sprite = Sprite::from_ascii_art("*", Some("R")); // Red asterisk

        Self {
            id,
            position,
            velocity,
            sprite,
            alive: true,
            _created_at: Instant::now(),
            shark_id,
        }
    }

    /// Get the associated shark ID
    pub fn get_shark_id(&self) -> EntityId {
        self.shark_id
    }

    /// Check if teeth have moved off screen
    fn is_off_screen(&self, screen_bounds: Rect) -> bool {
        self.position.x < -10.0 || self.position.x > (screen_bounds.width + 10) as f32
    }
}

impl Entity for SharkTeeth {
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
        &self.sprite
    }

    fn update(&mut self, delta_time: Duration, screen_bounds: Rect) {
        if !self.alive {
            return;
        }

        // Update position based on velocity
        let dt_secs = delta_time.as_secs_f32();
        self.position.x += self.velocity.dx * dt_secs * 60.0; // Scale for 60 FPS equivalent
        self.position.y += self.velocity.dy * dt_secs * 60.0;

        // Check if teeth have moved off screen
        if self.is_off_screen(screen_bounds) {
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
        "shark_teeth"
    }
}

/// Shark manager handles shark spawning and behavior
pub struct SharkManager {
    last_spawn: Instant,
    spawn_interval: Duration,
}

impl SharkManager {
    pub fn new() -> Self {
        Self {
            last_spawn: Instant::now(),
            spawn_interval: Duration::from_secs(30), // Spawn shark every 30 seconds
        }
    }

    /// Check if we should spawn a new shark
    pub fn should_spawn_shark(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_spawn) > self.spawn_interval {
            self.last_spawn = now;
            true
        } else {
            false
        }
    }

    /// Reset spawn timer (for redraw)
    pub fn reset(&mut self) {
        self.last_spawn = Instant::now();
    }
}

impl Default for SharkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shark_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let shark = Shark::new_random(1, screen_bounds);

        assert!(shark.is_alive());
        assert_eq!(shark.entity_type(), "shark");
        assert_eq!(shark.depth(), crate::depth::depth::SHARK);
    }

    #[test]
    fn test_shark_sprites() {
        let shark = Shark::new_random(1, Rect::new(0, 0, 80, 24));
        let right_sprite = &shark.right_sprite;
        let left_sprite = &shark.left_sprite;

        assert!(!right_sprite.lines.is_empty());
        assert!(!left_sprite.lines.is_empty());
        assert_ne!(right_sprite.lines, left_sprite.lines);
    }

    #[test]
    fn test_shark_teeth_position() {
        let position = Position::new(10.0, 10.0, crate::depth::depth::SHARK);
        let velocity = Velocity::new(2.0, 0.0);
        let shark = Shark::new(1, position, velocity, Direction::Right);

        let teeth_pos = shark.get_teeth_position();
        assert_eq!(teeth_pos.x, 54.0); // 10 + 44
        assert_eq!(teeth_pos.y, 17.0); // 10 + 7
        assert_eq!(teeth_pos.depth, crate::depth::depth::SHARK + 1);
    }

    #[test]
    fn test_shark_teeth_creation() {
        let position = Position::new(10.0, 10.0, crate::depth::depth::SHARK);
        let velocity = Velocity::new(2.0, 0.0);
        let teeth = SharkTeeth::new(2, position, velocity, 1);

        assert!(teeth.is_alive());
        assert_eq!(teeth.entity_type(), "shark_teeth");
        assert_eq!(teeth.get_shark_id(), 1);
    }

    #[test]
    fn test_shark_manager() {
        let mut manager = SharkManager::new();

        // Should not spawn immediately after creation (needs time to pass)
        assert!(!manager.should_spawn_shark());

        // Simulate time passing by resetting to past time
        manager.last_spawn = Instant::now() - Duration::from_secs(31);
        assert!(manager.should_spawn_shark());

        // Should not spawn again immediately
        assert!(!manager.should_spawn_shark());
    }

    #[test]
    fn test_shark_movement() {
        let position = Position::new(10.0, 10.0, crate::depth::depth::SHARK);
        let velocity = Velocity::new(2.0, 0.0);
        let mut shark = Shark::new(1, position, velocity, Direction::Right);

        let screen_bounds = Rect::new(0, 0, 80, 24);
        shark.update(Duration::from_millis(16), screen_bounds); // ~60 FPS

        // Shark should have moved right
        assert!(shark.position().x > 10.0);
    }
}
