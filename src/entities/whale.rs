use crate::entity::{DeathCallback, Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

pub struct Whale {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    sprite: Sprite,
    animation_frame: usize,
    last_frame_time: Instant,
    created_at: Instant,
    alive: bool,
}

impl Whale {
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
                (-18.0, 1.0)
            }
            Direction::Left => {
                // Start off-screen right, move left
                (screen_bounds.width as f32 + 2.0, -1.0)
            }
        };

        let y = 0.0; // Surface level
        let depth = 5; // water_gap2 depth

        let position = Position::new(x, y, depth);
        let velocity = Velocity::new(dx, 0.0);

        // Create initial sprite (whale without spout)
        let sprite = Self::create_whale_sprite(&direction, false, 0);

        Self {
            id,
            position,
            velocity,
            direction,
            sprite,
            animation_frame: 0,
            last_frame_time: Instant::now(),
            created_at: Instant::now(),
            alive: true,
        }
    }

    fn create_whale_sprite(direction: &Direction, has_spout: bool, spout_frame: usize) -> Sprite {
        let whale_ascii = match direction {
            Direction::Right => {
                "\n\n\n        .-----:\n      .'       `.\n,????/       (o) \\\n\\`._/          ,__)"
            }
            Direction::Left => {
                "\n\n\n    :-----.\n  .'       `.\n / (o)       \\????,\n(__,          \\_.'/'"
            }
        };

        let whale_mask = match direction {
            Direction::Right => {
                "\n\n\n             C C\n           CCCCCCC\n           C  C  C\n        BBBBBBB\n      BB       BB\nB    B       BWB B\nBBBBB          BBBB"
            }
            Direction::Left => {
                "\n\n\n   C C\n CCCCCCC\n C  C  C\n    BBBBBBB\n  BB       BB\n B BWB       B    B\nBBBB          BBBBB"
            }
        };

        if !has_spout {
            return Sprite::from_ascii_art(whale_ascii, Some(whale_mask));
        }

        // Create whale with water spout
        let spout_ascii = Self::get_water_spout_frame(spout_frame);
        let spout_alignment = match direction {
            Direction::Right => 11,
            Direction::Left => 1,
        };

        // Align the spout above the whale
        let aligned_spout = spout_ascii
            .lines()
            .map(|line| format!("{}{}", " ".repeat(spout_alignment), line))
            .collect::<Vec<_>>()
            .join("\n");

        let combined_ascii = format!(
            "{}{}",
            aligned_spout,
            whale_ascii
                .trim_start_matches('\n')
                .trim_start_matches('\n')
                .trim_start_matches('\n')
        );

        // Create spout color mask (all 'C' for cyan water)
        let spout_color_mask = spout_ascii
            .lines()
            .map(|line| {
                let colored_line = line
                    .chars()
                    .map(|c| if c == ' ' { ' ' } else { 'C' })
                    .collect::<String>();
                format!("{}{}", " ".repeat(spout_alignment), colored_line)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let combined_mask = format!(
            "{}{}",
            spout_color_mask,
            whale_mask
                .trim_start_matches('\n')
                .trim_start_matches('\n')
                .trim_start_matches('\n')
        );

        Sprite::from_ascii_art(&combined_ascii, Some(&combined_mask))
    }

    fn get_water_spout_frame(frame: usize) -> &'static str {
        match frame {
            0 => "\n\n\n   :",
            1 => "\n\n   :\n   :",
            2 => "\n  . .\n  -:-\n   :",
            3 => "\n  . .\n .-:-.\n   :",
            4 => "\n  . .\n'.-:-.`\n'  :  '",
            5 => "\n\n .- -.\n;  :  ;",
            6 => "\n\n\n;     ;",
            _ => "",
        }
    }

    fn update_animation(&mut self) {
        // Update animation frame every 500ms
        if self.last_frame_time.elapsed().as_millis() > 500 {
            self.animation_frame = (self.animation_frame + 1) % 12; // 5 frames without spout + 7 frames with spout
            self.last_frame_time = Instant::now();

            // Update sprite based on animation frame
            if self.animation_frame < 5 {
                // Whale without spout
                self.sprite = Self::create_whale_sprite(&self.direction, false, 0);
            } else {
                // Whale with spout
                let spout_frame = self.animation_frame - 5;
                self.sprite = Self::create_whale_sprite(&self.direction, true, spout_frame);
            }
        }
    }

    fn check_offscreen_death(&mut self, screen_bounds: Rect) {
        let is_off_screen = match self.direction {
            Direction::Right => self.position.x > screen_bounds.width as f32 + 20.0,
            Direction::Left => self.position.x < -20.0,
        };

        if is_off_screen {
            self.alive = false;
        }
    }
}

impl Entity for Whale {
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

        // Update animation
        self.update_animation();

        // Update position based on velocity
        self.position.x += self.velocity.dx * delta_time.as_secs_f32() * 60.0; // Scale for 60 FPS

        // Check if whale should die (off-screen)
        self.check_offscreen_death(screen_bounds);
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "whale"
    }

    fn death_callback(&self) -> Option<DeathCallback> {
        Some(crate::spawning::random_object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whale_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let whale = Whale::new(1, screen_bounds);

        assert!(whale.is_alive());
        assert_eq!(whale.entity_type(), "whale");
        assert_eq!(whale.depth(), 5); // water_gap2 depth
    }

    #[test]
    fn test_whale_direction_and_position() {
        let screen_bounds = Rect::new(0, 0, 80, 24);

        // Test multiple whales to check randomization
        for _ in 0..10 {
            let whale = Whale::new(1, screen_bounds);

            match whale.direction {
                Direction::Right => {
                    assert_eq!(whale.position().x, -18.0);
                    assert_eq!(whale.velocity().dx, 1.0);
                }
                Direction::Left => {
                    assert_eq!(whale.position().x, 82.0); // screen_width + 2
                    assert_eq!(whale.velocity().dx, -1.0);
                }
            }
            assert_eq!(whale.position().y, 0.0); // Surface level
        }
    }

    #[test]
    fn test_whale_sprite_creation() {
        let right_sprite = Whale::create_whale_sprite(&Direction::Right, false, 0);
        let left_sprite = Whale::create_whale_sprite(&Direction::Left, false, 0);

        assert!(!right_sprite.lines.is_empty());
        assert!(!left_sprite.lines.is_empty());
        assert_ne!(right_sprite.lines, left_sprite.lines);

        // Check that sprites contain whale features
        let right_text = right_sprite.lines.join("\n");
        let left_text = left_sprite.lines.join("\n");

        assert!(right_text.contains(".-----:"));
        assert!(left_text.contains(":-----."));
    }

    #[test]
    fn test_whale_spout_animation() {
        let sprite_without_spout = Whale::create_whale_sprite(&Direction::Right, false, 0);
        let sprite_with_spout = Whale::create_whale_sprite(&Direction::Right, true, 0);

        // Check that spout sprite contains spout character
        let spout_text = sprite_with_spout.lines.join("\n");
        assert!(spout_text.contains(":"));

        // Check that sprites are different
        assert_ne!(sprite_without_spout.lines, sprite_with_spout.lines);

        // Sprite with spout should contain water spout elements
        assert!(spout_text.contains(":"));

        // Both should contain whale body
        let whale_text = sprite_without_spout.lines.join("\n");
        assert!(whale_text.contains(".-----:"));
        assert!(spout_text.contains(".-----:"));
    }

    #[test]
    fn test_whale_water_spout_frames() {
        // Test all spout frames
        for frame in 0..7 {
            let spout = Whale::get_water_spout_frame(frame);
            assert!(!spout.is_empty());
        }

        // Test invalid frame
        let invalid_spout = Whale::get_water_spout_frame(10);
        assert_eq!(invalid_spout, "");
    }

    #[test]
    fn test_whale_animation_update() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut whale = Whale::new(1, screen_bounds);

        let initial_frame = whale.animation_frame;

        // Animation should not update immediately
        whale.update_animation();
        assert_eq!(whale.animation_frame, initial_frame);

        // Simulate time passing
        whale.last_frame_time = Instant::now() - Duration::from_millis(600);
        whale.update_animation();

        // Frame should have advanced
        assert_ne!(whale.animation_frame, initial_frame);
    }

    #[test]
    fn test_whale_movement() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut whale = Whale::new(1, screen_bounds);

        let initial_x = whale.position().x;
        whale.update(Duration::from_millis(16), screen_bounds); // ~60 FPS

        // Whale should have moved
        assert_ne!(whale.position().x, initial_x);
    }

    #[test]
    fn test_whale_offscreen_death() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut whale = Whale::new(1, screen_bounds);

        // Move whale far off screen
        match whale.direction {
            Direction::Right => whale.position.x = 200.0,
            Direction::Left => whale.position.x = -100.0,
        }

        whale.update(Duration::from_millis(16), screen_bounds);
        assert!(!whale.is_alive());
    }
}
