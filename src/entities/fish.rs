use crate::depth;
use crate::entity::{Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::{layout::Rect, style::Color};
use std::time::{Duration, Instant};

/// Fish species with their ASCII art and colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishSpecies {
    Small1,
    Small2,
    Medium1,
    Medium2,
    Large1,
    Large2,
    // Add more species as we implement them
}

impl FishSpecies {
    /// Get all available fish species
    pub fn all_species() -> &'static [FishSpecies] {
        &[
            FishSpecies::Small1,
            FishSpecies::Small2,
            FishSpecies::Medium1,
            FishSpecies::Medium2,
            FishSpecies::Large1,
            FishSpecies::Large2,
        ]
    }

    /// Get a random fish species
    pub fn random() -> Self {
        let species = Self::all_species();
        let mut rng = rand::thread_rng();
        species[rng.gen_range(0..species.len())]
    }

    /// Get the sprites for this fish species (right-facing, left-facing)
    pub fn get_sprites(&self) -> (Sprite, Sprite) {
        match self {
            FishSpecies::Small1 => {
                let right_art = r#"   \
  / \
>=_('>
  \_/
   /"#;
                let right_mask = r#"   1
  1 1
663745
  111
   3"#;

                let left_art = r#"  /
 / \
<')_=<
 \_/
  \"#;
                let left_mask = r#"  2
 111
547366
 111
  3"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::Small2 => {
                let right_art = r#"     ,
     \}\
\  .'  `\
\}\<   ( 6>
/  `,  .'
     \}/
     '"#;
                let right_mask = r#"     2
     22
6  11  11
661   7 45
6  11  11
     33
     3"#;

                let left_art = r#"    ,
   /\{
 /'  `.  /
<6 )   >\{\{
 `.  ,'  \
   \{
    `"#;
                let left_mask = r#"    2
   22
 11  11  6
54 7   166
 11  11  6
   33
    3"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::Medium1 => {
                let right_art = r#"            \\'`.
             )  \
(`.??????_.-`' ' '`-.
 \ `.??.`        (o) \_
  >  ><     (((       (
 / .`??`._      /_|  /'
(.`???????`-. _  _.-`
            /__/'"#;
                let right_mask = r#"            1111
             1  1
111      11111 1 1111
 1 11  11        141 11
  1  11     777       5
 1 11  111      333  11
111       111 1  1111
            11111"#;

                let left_art = r#"       .'`/
      /  (
  .-'` ` `'-._??????.')
_/ (o)        '.??.' /
)       )))     ><  <
`\  |_\      _.'??'. \
  '-._  _ .-'???????'.)
      `\__\"#;
                let left_mask = r#"       1111
      1  1
  1111 1 11111      111
11 141        11  11 1
5       777     11  1
11  333      111  11 1
  1111  1 111       111
      11111"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::Medium2 => {
                let right_art = r#"       ,--,_
__    _\.---'-.
\ '.-"     // o\
/_.'-._    \\  /
       `"--(/"`"#;
                let right_mask = r#"       22222
66    121111211
6 6111     77 41
6661111    77  1
       11113311"#;

                let left_art = r#"    _,--,
 .-'---./_    __
/o \\     "-.' /
\  //    _.-'._\
 `"\)--"`"#;
                let left_mask = r#"    22222
 112111121    66
14 77     1116 6
1  77    1111666
 11331111"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::Large1 => {
                // Placeholder for big fish - will implement later
                let right_art = "><(((((*>";
                let left_art = "<*))))))><";
                (
                    Sprite::from_ascii_art(right_art, None),
                    Sprite::from_ascii_art(left_art, None),
                )
            }
            FishSpecies::Large2 => {
                // Placeholder for big fish - will implement later
                let right_art = "><(((((o>";
                let left_art = "<o))))))><";
                (
                    Sprite::from_ascii_art(right_art, None),
                    Sprite::from_ascii_art(left_art, None),
                )
            }
        }
    }

    /// Get the base color for this fish species
    pub fn get_base_color(&self) -> Color {
        match self {
            FishSpecies::Small1 => Color::Yellow,
            FishSpecies::Small2 => Color::Cyan,
            FishSpecies::Medium1 => Color::Green,
            FishSpecies::Medium2 => Color::Magenta,
            FishSpecies::Large1 => Color::Blue,
            FishSpecies::Large2 => Color::Red,
        }
    }
}

/// A fish entity that swims across the screen
#[derive(Debug)]
pub struct Fish {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    species: FishSpecies,
    right_sprite: Sprite,
    left_sprite: Sprite,
    base_color: Color,
    alive: bool,
    bubble_timer: f32,
    age: Duration,
    created_at: Instant,
}

impl Fish {
    /// Create a new fish with random properties
    pub fn new_random(id: EntityId, screen_bounds: Rect) -> Self {
        let mut rng = rand::thread_rng();

        let species = FishSpecies::random();
        let (right_sprite, left_sprite) = species.get_sprites();
        let base_color = species.get_base_color();

        // Alternate direction based on fish ID (like original)
        let direction = if id % 2 == 0 {
            Direction::Right
        } else {
            Direction::Left
        };

        // Position based on direction (start off-screen)
        let sprite_bounds = match direction {
            Direction::Right => right_sprite.get_bounding_box(),
            Direction::Left => left_sprite.get_bounding_box(),
        };

        let (x, dx) = match direction {
            Direction::Right => {
                // Start completely off-screen to the left, move right
                let x = -(sprite_bounds.0 as f32 + 5.0);
                let speed = rng.gen_range(0.5..2.0);
                (x, speed)
            }
            Direction::Left => {
                // Start completely off-screen to the right, move left
                let x = screen_bounds.width as f32 + sprite_bounds.0 as f32 + 5.0;
                let speed = rng.gen_range(0.5..2.0);
                (x, -speed)
            }
        };

        // Random Y position in underwater area (below water surface)
        let water_surface_y = 9; // Based on original code
        let min_y = screen_bounds.height.saturating_sub(sprite_bounds.1);
        let y = rng.gen_range(water_surface_y..min_y.max(water_surface_y + 1)) as f32;

        // Random depth in fish layer
        let depth = depth::random_fish_depth();

        // Fish only move horizontally (no vertical movement in original)
        let dy = 0.0;

        Self {
            id,
            position: Position::new(x, y, depth),
            velocity: Velocity::new(dx, dy),
            direction,
            species,
            right_sprite,
            left_sprite,
            base_color,
            alive: true,
            bubble_timer: rng.gen_range(2.0..8.0), // Seconds until next bubble
            age: Duration::ZERO,
            created_at: Instant::now(),
        }
    }

    /// Create a fish with specific properties
    pub fn new(
        id: EntityId,
        position: Position,
        velocity: Velocity,
        direction: Direction,
        species: FishSpecies,
    ) -> Self {
        let (right_sprite, left_sprite) = species.get_sprites();
        let base_color = species.get_base_color();
        let mut rng = rand::thread_rng();

        Self {
            id,
            position,
            velocity,
            direction,
            species,
            right_sprite,
            left_sprite,
            base_color,
            alive: true,
            bubble_timer: rng.gen_range(2.0..8.0),
            age: Duration::ZERO,
            created_at: Instant::now(),
        }
    }

    /// Get the current direction the fish is facing
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Get the fish species
    pub fn species(&self) -> FishSpecies {
        self.species
    }

    /// Get the base color
    pub fn base_color(&self) -> Color {
        self.base_color
    }

    /// Check if fish should emit a bubble
    pub fn should_emit_bubble(&mut self, delta_time: Duration) -> bool {
        self.bubble_timer -= delta_time.as_secs_f32();
        if self.bubble_timer <= 0.0 {
            // Reset timer for next bubble
            let mut rng = rand::thread_rng();
            self.bubble_timer = rng.gen_range(3.0..10.0);
            true
        } else {
            false
        }
    }

    /// Get the position where a bubble should be emitted from this fish
    pub fn get_bubble_position(&self) -> Position {
        let sprite = self.get_current_sprite();
        let (width, height) = sprite.get_bounding_box();

        // Position bubble at fish's mouth/front
        let bubble_x = match self.direction {
            Direction::Right => self.position.x + width as f32, // Right side of fish
            Direction::Left => self.position.x,                 // Left side of fish
        };

        let bubble_y = self.position.y + (height as f32 / 2.0); // Middle of fish vertically

        // Bubble appears one depth layer above the fish (lower depth number = more foreground)
        let bubble_depth = self.position.depth.saturating_sub(1);

        Position::new(bubble_x, bubble_y, bubble_depth)
    }

    /// Update fish direction based on velocity
    fn update_direction(&mut self) {
        if self.velocity.dx > 0.0 {
            self.direction = Direction::Right;
        } else if self.velocity.dx < 0.0 {
            self.direction = Direction::Left;
        }
    }

    /// Check if fish is off-screen and should die
    fn check_offscreen_death(&mut self, screen_bounds: Rect) {
        let sprite_bounds = self.get_current_sprite().get_bounding_box();
        let pos_x = self.position.x;
        let pos_y = self.position.y;

        // Check if completely off screen with generous margins
        let off_left = pos_x < -(sprite_bounds.0 as f32 + 10.0);
        let off_right = pos_x > screen_bounds.width as f32 + sprite_bounds.0 as f32 + 10.0;
        let off_top = pos_y < -(sprite_bounds.1 as f32 + 5.0);
        let off_bottom = pos_y > screen_bounds.height as f32 + sprite_bounds.1 as f32 + 5.0;

        if off_left || off_right || off_top || off_bottom {
            self.alive = false;
        }
    }
}

impl Entity for Fish {
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
        self.update_direction();
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

        // Update age
        self.age = self.created_at.elapsed();

        // Update position based on velocity (fish only move horizontally)
        self.position.x += self.velocity.dx * delta_time.as_secs_f32() * 60.0; // Scale for 60 FPS
        // Fish don't move vertically in the original implementation

        // Check if fish should die (off-screen)
        self.check_offscreen_death(screen_bounds);
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "fish"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::depth;

    #[test]
    fn test_fish_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let fish = Fish::new_random(1, screen_bounds);

        assert!(fish.is_alive());
        assert_eq!(fish.entity_type(), "fish");
        assert!(depth::is_fish_depth(fish.depth()));
    }

    #[test]
    fn test_fish_species() {
        let species = FishSpecies::Small1;
        let (right, left) = species.get_sprites();

        assert!(!right.lines.is_empty());
        assert!(!left.lines.is_empty());
        assert_ne!(right.lines, left.lines); // Should be different sprites
    }

    #[test]
    fn test_fish_movement() {
        let mut fish = Fish::new(
            1,
            Position::new(10.0, 10.0, depth::depth::FISH_START),
            Velocity::new(1.0, 0.0),
            Direction::Right,
            FishSpecies::Small1,
        );

        let initial_x = fish.position().x;
        fish.update(Duration::from_millis(16), Rect::new(0, 0, 80, 24)); // ~60 FPS

        assert!(fish.position().x > initial_x); // Should move right
    }
}
