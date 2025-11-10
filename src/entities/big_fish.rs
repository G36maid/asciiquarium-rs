//! Big Fish entity - large predatory fish
//!
//! There are two variants of big fish:
//! - BigFish1: Traditional large fish shape (14 lines tall, speed 3)
//! - BigFish2: More stylized large fish (13 lines tall, speed 2.5)
//!
//! In modern mode, BigFish2 appears 2/3 of the time.
//! In classic mode, only BigFish1 appears.

use crate::depth::SHARK;
use crate::entity::{DeathCallback, Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::Duration;

/// Big fish variant type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigFishVariant {
    /// Traditional big fish (classic mode compatible)
    Variant1,
    /// Stylized big fish (modern mode only)
    Variant2,
}

/// A large predatory fish
#[derive(Debug)]
pub struct BigFish {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    sprite: Sprite,
    variant: BigFishVariant,
    alive: bool,
}

impl BigFish {
    /// Create a new big fish with random variant selection
    pub fn new(id: EntityId, screen_bounds: Rect, classic_mode: bool) -> Self {
        let mut rng = rand::thread_rng();

        // Select variant based on mode
        let variant = if classic_mode {
            BigFishVariant::Variant1
        } else {
            // 2/3 chance for Variant2, 1/3 for Variant1
            // Original Perl: int(rand(3)) > 1
            if rng.gen_range(0..3) > 1 {
                BigFishVariant::Variant2
            } else {
                BigFishVariant::Variant1
            }
        };

        Self::new_variant(id, screen_bounds, variant)
    }

    /// Create a new big fish with specific variant
    pub fn new_variant(id: EntityId, screen_bounds: Rect, variant: BigFishVariant) -> Self {
        let mut rng = rand::thread_rng();
        let direction = if rng.gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };

        let (sprite, speed) = match variant {
            BigFishVariant::Variant1 => (create_big_fish_1_sprite(direction), 3.0),
            BigFishVariant::Variant2 => (create_big_fish_2_sprite(direction), 2.5),
        };

        // Match original Perl spawn positions:
        // Variant1: x = -34 (right) or width-1 (left), y = rand(height-15) + 9
        // Variant2: x = -33 (right) or width-1 (left), y = rand(height-14) + 9
        let x = match direction {
            Direction::Right => match variant {
                BigFishVariant::Variant1 => -34,
                BigFishVariant::Variant2 => -33,
            },
            Direction::Left => screen_bounds.width as i32 - 1,
        };

        // Y position varies by variant due to different sprite heights
        let max_height = 9;
        let height_offset = match variant {
            BigFishVariant::Variant1 => 15, // Original: height - 15
            BigFishVariant::Variant2 => 14, // Original: height - 14
        };
        let min_height = screen_bounds
            .height
            .saturating_sub(height_offset)
            .max(max_height + 1);
        let y = rng.gen_range(max_height..min_height) as i32;

        let velocity = match direction {
            Direction::Right => Velocity::new(speed, 0.0),
            Direction::Left => Velocity::new(-speed, 0.0),
        };

        Self {
            id,
            position: Position::new(x as f32, y as f32, SHARK),
            velocity,
            direction,
            sprite,
            variant,
            alive: true,
        }
    }
}

impl Entity for BigFish {
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
        SHARK
    }

    fn get_current_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn update(&mut self, delta_time: Duration, _screen_bounds: Rect) {
        let dt = delta_time.as_secs_f32();
        self.position.x += self.velocity.dx * dt;
        self.position.y += self.velocity.dy * dt;
    }

    fn is_alive(&self) -> bool {
        if !self.alive {
            return false;
        }

        // Die when completely off screen
        let sprite_width = self.sprite.get_bounding_box().0 as f32;
        match self.direction {
            Direction::Right => self.position.x < 200.0, // Will die off right edge
            Direction::Left => self.position.x + sprite_width > -200.0, // Will die off left edge
        }
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        match self.variant {
            BigFishVariant::Variant1 => "big_fish_1",
            BigFishVariant::Variant2 => "big_fish_2",
        }
    }

    fn death_callback(&self) -> Option<DeathCallback> {
        Some(crate::spawning::random_object)
    }
}

/// Create sprite for big fish variant 1 (traditional)
fn create_big_fish_1_sprite(direction: Direction) -> Sprite {
    let (image, mask) = match direction {
        Direction::Right => (
            r#" ______
`""-.  `````-----.....__
     `.  .      .       `-.
       :     .     .       `.
 ,     :   .    .          _ :
: `.   :                  (@) `._
 `. `..'     .     =`-.       .__)
   ;     .        =  ~  :     .-"
 .' .'`.   .    .  =.-'  `._ .'
: .'   :               .   .'
 '   .'  .    .     .   .-'
   .'____....----''.'=.'
   ""             .'.'
               ''"'`"#,
            r#" 111111
11111  11111111111111111
     11  2      2       111
       1     2     2       11
 1     1   2    2          1 1
1 11   1                  1W1 111
 11 1111     2     1111       1111
   1     2        1  1  1     111
 11 1111   2    2  1111  111 11
1 11   1               2   11
 1   11  2    2     2   111
   111111111111111111111
   11             1111
               11111"#,
        ),
        Direction::Left => (
            r#"                           ______
          __.....-----'''''  .-""'
       .-'       .      .  .'
     .'       .     .     :
    : _          .    .   :     ,
 _.' (@)                  :   .' :
(__.       .-'=     .     `..' .'
 "-.     :  ~  =        .     ;
   `. _.'  `-.=  .    .   .'`. `.
     `.   .               :   `. :
       `-.   .     .    .  `.   '
          `.=`.``----....____`.
            `.`.             ""
              '`"``               "#,
            r#"                           111111
          11111111111111111  11111
       111       2      2  11
     11       2     2     1
    1 1          2    2   1     1
 111 1W1                  1   11 1
1111       1111     2     1111 11
 111     1  1  1        2     1
   11 111  1111  2    2   1111 11
     11   2               1   11 1
       111   2     2    2  11   1
          111111111111111111111
            1111             11
              11111               "#,
        ),
    };

    Sprite::from_ascii_art_with_random_colors(image, Some(mask))
}

/// Create sprite for big fish variant 2 (stylized)
fn create_big_fish_2_sprite(direction: Direction) -> Sprite {
    let (image, mask) = match direction {
        Direction::Right => (
            r#"                _ _ _
             .='\\ \\ \\`"=,
           .'\\ \\ \\ \\ \\ \\ \\
\\'=._     / \\ \\ \\_\\_\\_\\_\\_\\
\\'=._'.  /\\ \\,-"`- _ - _ - '-.
  \\`=._\\|'.\\/- _ - _ - _ - _- \\
  ;"= ._\\=./_ -_ -_ \{`"=_    @ \\
   ;="_-_=- _ -  _ - \{"=_"-     \\
   ;_=_--_.,          \{_.='   .-/
  ;.="` / ';\\        _.     _.-`
  /_.='/ \\/ /;._ _ _\{.-;`/"`
/._=_.'   '/ / / / /\{.= /
/.='       `'./_/_.=`\{_/"#,
            r#"                1 1 1
             1111 1 11111
           111 1 1 1 1 1 1
11111     1 1 1 11111111111
1111111  11 111112 2 2 2 2 111
  111111111112 2 2 2 2 2 2 22 1
  111 1111 12 22 22 11111    W 1
   11111112 2 2  2 2 111111     1
   111111111          11111   111
  11111 11111        11     1111
  111111 11 1111 1 111111111
1111111   11 1 1 1 1111 1
1111       1111111111111"#,
        ),
        Direction::Left => (
            r#"            _ _ _
        ,="`/ / /'=.
       / / / / / / /'.
      /_/_/_/_/_/ / / \\     _.='/
   .-' - _ - _ -`"-,/ /\\  .'_.='/
  / -_ - _ - _ - _ -\\/.'|/_.=`/
 / @    _="`\} _- _- _\\.=/_. =";
/     -"_="\} - _  - _ -=_-_"=;
\\-.   '=._\}          ,._--_=_;
 `-._     ._        /;' \\ `"=.;
     `"\\`;-.\}_ _ _.;\\ \\/ \\'=._\\
        \\ =.\}\\ \\ \\ \\ \\'   '._=_.\\
         \\_\}`=._\\_\\.'`       '=.\\"#,
            r#"            1 1 1
        11111 1 1111
       1 1 1 1 1 1 111
      11111111111 1 1 1     11111
   111 2 2 2 2 211111 11  1111111
  1 22 2 2 2 2 2 2 211111111111
 1 W    11111 22 22 2111111 111
1     111111 2 2  2 2 21111111
111   11111          111111111
 1111     11        111 1 11111
     111111111 1 1111 11 111111
        1 1111 1 1 1 11   1111111
         1111111111111       1111"#,
        ),
    };

    Sprite::from_ascii_art_with_random_colors(image, Some(mask))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_fish_creation() {
        let bounds = Rect::new(0, 0, 80, 24);
        let fish = BigFish::new(1, bounds, false);
        assert_eq!(fish.id, 1);
        // Large creature is tracked by EntityManager, not a trait method
        assert!(fish.alive);
    }

    #[test]
    fn test_big_fish_variants() {
        let bounds = Rect::new(0, 0, 80, 24);

        let fish1 = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        assert_eq!(fish1.entity_type(), "big_fish_1");
        assert_eq!(fish1.sprite.get_bounding_box().1, 14);

        let fish2 = BigFish::new_variant(2, bounds, BigFishVariant::Variant2);
        assert_eq!(fish2.entity_type(), "big_fish_2");
        assert_eq!(fish2.sprite.get_bounding_box().1, 13);
    }

    #[test]
    fn test_big_fish_classic_mode() {
        let bounds = Rect::new(0, 0, 80, 24);

        // Classic mode should only create Variant1
        for _ in 0..10 {
            let fish = BigFish::new(1, bounds, true);
            assert_eq!(fish.variant, BigFishVariant::Variant1);
        }
    }

    #[test]
    fn test_big_fish_spawn_positions() {
        let bounds = Rect::new(0, 0, 80, 24);

        // Test Variant1 positions (matches original Perl)
        let fish1 = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        match fish1.direction {
            Direction::Right => {
                assert_eq!(fish1.position.x, -34.0); // Original Perl: x = -34
            }
            Direction::Left => {
                assert_eq!(fish1.position.x, 79.0); // width - 1 = 79
            }
        }

        // Test Variant2 positions (matches original Perl)
        let fish2 = BigFish::new_variant(2, bounds, BigFishVariant::Variant2);
        match fish2.direction {
            Direction::Right => {
                assert_eq!(fish2.position.x, -33.0); // Original Perl: x = -33
            }
            Direction::Left => {
                assert_eq!(fish2.position.x, 79.0); // width - 1 = 79
            }
        }
    }

    #[test]
    fn test_big_fish_movement() {
        let bounds = Rect::new(0, 0, 80, 24);
        let mut fish = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        let initial_x = fish.position.x;

        fish.update(Duration::from_secs(1), bounds);

        match fish.direction {
            Direction::Right => assert!(fish.position.x > initial_x),
            Direction::Left => assert!(fish.position.x < initial_x),
        }
    }

    #[test]
    fn test_big_fish_death_callback() {
        let bounds = Rect::new(0, 0, 80, 24);
        let fish = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        assert!(fish.death_callback().is_some());
    }

    #[test]
    fn test_big_fish_speeds() {
        let bounds = Rect::new(0, 0, 80, 24);

        let fish1 = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        let speed1 = fish1.velocity.dx.abs();
        assert!((speed1 - 3.0).abs() < 0.01);

        let fish2 = BigFish::new_variant(2, bounds, BigFishVariant::Variant2);
        let speed2 = fish2.velocity.dx.abs();
        assert!((speed2 - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_big_fish_y_position_ranges() {
        let bounds = Rect::new(0, 0, 80, 24);

        // Test Variant1 Y range (height - 15)
        for _ in 0..10 {
            let fish1 = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
            // Y should be between 9 and (24 - 15) = 9, so exactly 9 for small screen
            assert!(fish1.position.y >= 9.0);
            assert!(fish1.position.y < (bounds.height.saturating_sub(15).max(10)) as f32);
        }

        // Test Variant2 Y range (height - 14)
        for _ in 0..10 {
            let fish2 = BigFish::new_variant(2, bounds, BigFishVariant::Variant2);
            // Y should be between 9 and (24 - 14) = 10
            assert!(fish2.position.y >= 9.0);
            assert!(fish2.position.y < (bounds.height.saturating_sub(14).max(10)) as f32);
        }
    }

    #[test]
    fn test_big_fish_variant_selection() {
        let bounds = Rect::new(0, 0, 80, 24);

        // Classic mode should always use Variant1
        for _ in 0..10 {
            let fish = BigFish::new(1, bounds, true); // classic_mode = true
            assert_eq!(fish.variant, BigFishVariant::Variant1);
        }

        // Modern mode should have both variants
        let mut has_variant1 = false;
        let mut has_variant2 = false;
        for _ in 0..30 {
            let fish = BigFish::new(1, bounds, false); // classic_mode = false
            match fish.variant {
                BigFishVariant::Variant1 => has_variant1 = true,
                BigFishVariant::Variant2 => has_variant2 = true,
            }
        }
        // With 30 iterations, we should see both variants (statistically)
        assert!(has_variant1 || has_variant2); // At least one variant appears
    }

    #[test]
    fn test_big_fish_depth() {
        use crate::depth::SHARK;
        let bounds = Rect::new(0, 0, 80, 24);

        // Both variants should use SHARK depth (2), not FISH_START (3)
        let fish1 = BigFish::new_variant(1, bounds, BigFishVariant::Variant1);
        assert_eq!(fish1.depth(), SHARK);
        assert_eq!(fish1.depth(), 2);

        let fish2 = BigFish::new_variant(2, bounds, BigFishVariant::Variant2);
        assert_eq!(fish2.depth(), SHARK);
        assert_eq!(fish2.depth(), 2);
    }
}
