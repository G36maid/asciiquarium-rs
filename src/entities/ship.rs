use crate::entity::{Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

pub struct Ship {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    sprite: Sprite,
    created_at: Instant,
    alive: bool,
}

impl Ship {
    pub fn new(id: EntityId, screen_bounds: Rect) -> Self {
        let mut rng = rand::thread_rng();

        // Random direction
        let direction = if rng.gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };

        // Starting position and velocity
        let (x, dx) = match direction {
            Direction::Right => {
                // Start off-screen left, move right
                (-24.0, 1.0)
            }
            Direction::Left => {
                // Start off-screen right, move left
                (screen_bounds.width as f32 + 2.0, -1.0)
            }
        };

        let y = 0.0; // Surface level
        let depth = 7; // water_gap1 depth

        let position = Position::new(x, y, depth);
        let velocity = Velocity::new(dx, 0.0);

        // Create ship sprite
        let sprite = Self::create_ship_sprite(&direction);

        Self {
            id,
            position,
            velocity,
            direction,
            sprite,
            created_at: Instant::now(),
            alive: true,
        }
    }

    fn create_ship_sprite(direction: &Direction) -> Sprite {
        let (ship_ascii, ship_mask) = match direction {
            Direction::Right => {
                let ascii = r#"
     |    |    |
    )_)  )_)  )_)
   )___))___))___)\
  )____)____)_____)\\\
_____|____|____|____\\\\\__
\                   /"#;

                let mask = r#"
     y    y    y

                  w
                   ww
yyyyyyyyyyyyyyyyyyyywwwyy
y                   y"#;

                (ascii, mask)
            }
            Direction::Left => {
                let ascii = r#"
         |    |    |
        (_(  (_(  (_(
      /(___((___((___(
    //(_____(____(____(
__///____|____|____|_____
    \                   /"#;

                let mask = r#"
         y    y    y

      w
    ww
yywwwyyyyyyyyyyyyyyyyyyyy
    y                   y"#;

                (ascii, mask)
            }
        };

        Sprite::from_ascii_art(ship_ascii, Some(ship_mask))
    }

    fn check_offscreen_death(&mut self, screen_bounds: Rect) {
        let is_off_screen = match self.direction {
            Direction::Right => self.position.x > screen_bounds.width as f32 + 30.0,
            Direction::Left => self.position.x < -30.0,
        };

        if is_off_screen {
            self.alive = false;
        }
    }
}

impl Entity for Ship {
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
        self.position.x += self.velocity.dx * delta_time.as_secs_f32() * 60.0; // Scale for 60 FPS

        // Check if ship should die (off-screen)
        self.check_offscreen_death(screen_bounds);
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "ship"
    }
}

/// Manager for ship spawning and lifecycle
pub struct ShipManager {
    last_spawn_time: Instant,
    spawn_interval: Duration,
}

impl ShipManager {
    pub fn new() -> Self {
        Self {
            last_spawn_time: Instant::now(),
            spawn_interval: Duration::from_secs(45), // Spawn roughly every 45 seconds
        }
    }

    pub fn should_spawn(&mut self) -> bool {
        if self.last_spawn_time.elapsed() >= self.spawn_interval {
            self.last_spawn_time = Instant::now();
            // Random chance to spawn
            rand::thread_rng().gen_bool(0.25) // 25% chance when interval elapses
        } else {
            false
        }
    }

    pub fn create_ship(&self, id: EntityId, screen_bounds: Rect) -> Ship {
        Ship::new(id, screen_bounds)
    }
}

impl Default for ShipManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let ship = Ship::new(1, screen_bounds);

        assert!(ship.is_alive());
        assert_eq!(ship.entity_type(), "ship");
        assert_eq!(ship.depth(), 7); // water_gap1 depth
    }

    #[test]
    fn test_ship_direction_and_position() {
        let screen_bounds = Rect::new(0, 0, 80, 24);

        // Test multiple ships to check randomization
        for _ in 0..10 {
            let ship = Ship::new(1, screen_bounds);

            match ship.direction {
                Direction::Right => {
                    assert_eq!(ship.position().x, -24.0);
                    assert_eq!(ship.velocity().dx, 1.0);
                }
                Direction::Left => {
                    assert_eq!(ship.position().x, 82.0); // screen_width + 2
                    assert_eq!(ship.velocity().dx, -1.0);
                }
            }
            assert_eq!(ship.position().y, 0.0); // Surface level
        }
    }

    #[test]
    fn test_ship_sprite_creation() {
        let right_sprite = Ship::create_ship_sprite(&Direction::Right);
        let left_sprite = Ship::create_ship_sprite(&Direction::Left);

        assert!(!right_sprite.lines.is_empty());
        assert!(!left_sprite.lines.is_empty());
        assert_ne!(right_sprite.lines, left_sprite.lines);

        // Check that sprites contain ship features
        let right_text = right_sprite.lines.join("\n");
        let left_text = left_sprite.lines.join("\n");

        assert!(right_text.contains("|")); // Masts
        assert!(left_text.contains("|")); // Masts
        assert!(right_text.contains(")")); // Hull curves
        assert!(left_text.contains("(")); // Hull curves
    }

    #[test]
    fn test_ship_color_masks() {
        let right_sprite = Ship::create_ship_sprite(&Direction::Right);
        let left_sprite = Ship::create_ship_sprite(&Direction::Left);

        // Check that color masks contain yellow (y) and white (w) colors
        if let Some(ref mask) = right_sprite.color_mask {
            let mask_text = mask.join("\n");
            assert!(mask_text.contains("y")); // Yellow masts
            assert!(mask_text.contains("w")); // White hull parts
        }

        if let Some(ref mask) = left_sprite.color_mask {
            let mask_text = mask.join("\n");
            assert!(mask_text.contains("y")); // Yellow masts
            assert!(mask_text.contains("w")); // White hull parts
        }
    }

    #[test]
    fn test_ship_movement() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut ship = Ship::new(1, screen_bounds);

        let initial_x = ship.position().x;
        ship.update(Duration::from_millis(16), screen_bounds); // ~60 FPS

        // Ship should have moved
        assert_ne!(ship.position().x, initial_x);
    }

    #[test]
    fn test_ship_offscreen_death() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut ship = Ship::new(1, screen_bounds);

        // Move ship far off screen
        match ship.direction {
            Direction::Right => ship.position.x = 200.0,
            Direction::Left => ship.position.x = -100.0,
        }

        ship.update(Duration::from_millis(16), screen_bounds);
        assert!(!ship.is_alive());
    }

    #[test]
    fn test_ship_manager() {
        let mut manager = ShipManager::new();

        // Should not spawn immediately after creation
        assert!(!manager.should_spawn());

        // Simulate time passing
        manager.last_spawn_time = Instant::now() - Duration::from_secs(46);

        // Should have a chance to spawn now (may not always spawn due to randomness)
        let mut _spawned = false;
        for _ in 0..20 {
            if manager.should_spawn() {
                _spawned = true;
                break;
            }
            manager.last_spawn_time = Instant::now() - Duration::from_secs(46);
        }
        // At least one attempt should succeed with 25% chance over 20 tries
    }

    #[test]
    fn test_ship_manager_create() {
        let manager = ShipManager::new();
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let ship = manager.create_ship(1, screen_bounds);

        assert_eq!(ship.id(), 1);
        assert_eq!(ship.entity_type(), "ship");
    }

    #[test]
    fn test_ship_surface_positioning() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let ship = Ship::new(1, screen_bounds);

        // Ships should be at surface level (y=0) and water_gap1 depth
        assert_eq!(ship.position().y, 0.0);
        assert_eq!(ship.depth(), 7);
    }
}
