use crate::depth;
use crate::entity::{DeathCallback, Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::{layout::Rect, style::Color};
use std::time::{Duration, Instant};

/// Fish species category (new vs old from original Perl)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishCategory {
    New,
    Old,
}

/// Fish species with their ASCII art and colors
/// Matches all 12 species from original asciiquarium.pl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishSpecies {
    // NEW FISH (4 species) - Added in asciiquarium 1.1
    NewSmall1,  // Small angled fish with fins
    NewSmall2,  // Fancy double-bracket fish
    NewMedium1, // Large fancy fish with question marks
    NewMedium2, // Bulgy fish with comma decorations

    // OLD FISH (8 species) - Original classic fish
    OldFancy,      // Fancy fish with scales and apostrophes
    OldSimple,     // Simple angled fish with straight body (>=  (o>)
    OldWavy,       // Dotted wavy fish (:::::::)
    OldTiny,       // Tiny simple fish (><_'>)
    OldCommaLarge, // Small comma fish with quotes (..\,)
    OldAngledFin,  // Small angled fish with fins (same art as NewSmall1)
    OldCommaSmall, // Even smaller comma fish (,\)
    OldRounded,    // Rounded small fish with diagonal body (\/ o\)
}

impl FishSpecies {
    /// Get category of this fish species
    pub fn category(&self) -> FishCategory {
        match self {
            FishSpecies::NewSmall1
            | FishSpecies::NewSmall2
            | FishSpecies::NewMedium1
            | FishSpecies::NewMedium2 => FishCategory::New,

            FishSpecies::OldFancy
            | FishSpecies::OldSimple
            | FishSpecies::OldWavy
            | FishSpecies::OldTiny
            | FishSpecies::OldCommaLarge
            | FishSpecies::OldAngledFin
            | FishSpecies::OldCommaSmall
            | FishSpecies::OldRounded => FishCategory::Old,
        }
    }

    /// Get all new fish species
    pub fn new_species() -> &'static [FishSpecies] {
        &[
            FishSpecies::NewSmall1,
            FishSpecies::NewSmall2,
            FishSpecies::NewMedium1,
            FishSpecies::NewMedium2,
        ]
    }

    /// Get all old fish species
    pub fn old_species() -> &'static [FishSpecies] {
        &[
            FishSpecies::OldFancy,
            FishSpecies::OldSimple,
            FishSpecies::OldWavy,
            FishSpecies::OldTiny,
            FishSpecies::OldCommaLarge,
            FishSpecies::OldAngledFin,
            FishSpecies::OldCommaSmall,
            FishSpecies::OldRounded,
        ]
    }

    /// Get a random fish species following original logic:
    /// - 25% chance for new fish (int(rand(12)) > 8, meaning 9,10,11 out of 0-11)
    /// - 75% chance for old fish
    /// - classic_mode flag disables new fish
    pub fn random(classic_mode: bool) -> Self {
        let mut rng = rand::thread_rng();

        if classic_mode {
            // Classic mode: only old fish
            let old = Self::old_species();
            old[rng.gen_range(0..old.len())]
        } else {
            // Modern mode: 25% new, 75% old (matching original int(rand(12)) > 8)
            if rng.gen_range(0..12) > 8 {
                // New fish (9, 10, 11 = 3 out of 12 = 25%)
                let new = Self::new_species();
                new[rng.gen_range(0..new.len())]
            } else {
                // Old fish (0-8 = 9 out of 12 = 75%)
                let old = Self::old_species();
                old[rng.gen_range(0..old.len())]
            }
        }
    }

    /// Get the sprites for this fish species (right-facing, left-facing)
    pub fn get_sprites(&self) -> (Sprite, Sprite) {
        match self {
            // NEW FISH
            FishSpecies::NewSmall1 => {
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
            FishSpecies::NewSmall2 => {
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
            FishSpecies::NewMedium1 => {
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
            FishSpecies::NewMedium2 => {
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

            // OLD FISH
            FishSpecies::OldFancy => {
                let right_art = r#"       \
     ...\..,
\  /'       \
 >=     (  ' >
/  \      / /
    `"'"'/'"#;
                let right_mask = r#"       2
     1112111
6  11       1
 66     7  4 5
6  1      3 1
    11111311"#;

                let left_art = r#"      /
  ,../...
 /       '\  /
< '  )     =<
 \ \      /  \
  `'\'"'"'"#;
                let left_mask = r#"      2
  1112111
 1       11  6
5 4  7     66
 1 3      1  6
  11311111"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldSimple => {
                let right_art = r#"    \
\ /--\
>=  (o>
/ \__/
    /"#;
                let right_mask = r#"    2
6 1111
66  745
6 1111
    3"#;

                let left_art = r#"  /
 /--\ /
<o)  =<
 \__/ \
  \"#;
                let left_mask = r#"  2
 1111 6
547  66
 1111 6
  3"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldWavy => {
                let right_art = r#"       \:.
\;,   ,;\\\\\,,
  \\\\\;;:::::::o
  ///;;::::::::<
 /;` ``/////``"#;
                let right_mask = r#"       222
666   1122211
  6661111111114
  66611111111115
 666 113333311"#;

                let left_art = r#"      .:/
   ,,///;,   ,;/
 o:::::::;;///
>::::::::;;\\\\\
  ''\\\\\\\\\'' ';\\"#;
                let left_mask = r#"      222
   1122211   666
 4111111111666
51111111111666
  113333311 666"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldTiny => {
                let right_art = r#"  __
><_'>
   '"#;
                let right_mask = r#"  11
61145
   3"#;

                let left_art = r#" __
<'_><
 `"#;
                let left_mask = r#" 11
54116
 3"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldCommaLarge => {
                let right_art = r#"   ..\,
>='   ('>
  '''/'"#;
                let right_mask = r#"   1121
661   745
  111311"#;

                let left_art = r#"  ,/..
<')   `=<
 ``\```"#;
                let left_mask = r#"  1211
547   166
 113111"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldAngledFin => {
                // Same art as NewSmall1 (appears in both new and old arrays in original)
                let right_art = r#"   \
  / \
>=_('>
  \_/
   /"#;
                let right_mask = r#"   2
  1 1
661745
  111
   3"#;

                let left_art = r#"  /
 / \
<')_=<
 \_/
  \"#;
                let left_mask = r#"  2
 1 1
547166
 111
  3"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldCommaSmall => {
                let right_art = r#"  ,\
>=('>
  '/"#;
                let right_mask = r#"  12
66745
  13"#;

                let left_art = r#" /,
<')=<
 \`"#;
                let left_mask = r#" 21
54766
 31"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
            FishSpecies::OldRounded => {
                let right_art = r#"  __
\/ o\
/\__/"#;
                let right_mask = r#"  11
61 41
61111"#;

                let left_art = r#" __
/o \/
\__/\"#;
                let left_mask = r#" 11
14 16
11116"#;

                (
                    Sprite::from_ascii_art_with_random_colors(right_art, Some(right_mask)),
                    Sprite::from_ascii_art_with_random_colors(left_art, Some(left_mask)),
                )
            }
        }
    }

    /// Get the base color for this fish species
    pub fn get_base_color(&self) -> Color {
        // Colors are randomized in the original, we just provide a base
        match self {
            FishSpecies::NewSmall1 => Color::Yellow,
            FishSpecies::NewSmall2 => Color::Cyan,
            FishSpecies::NewMedium1 => Color::Green,
            FishSpecies::NewMedium2 => Color::Magenta,
            FishSpecies::OldFancy => Color::Blue,
            FishSpecies::OldSimple => Color::Red,
            FishSpecies::OldWavy => Color::Green,
            FishSpecies::OldTiny => Color::Yellow,
            FishSpecies::OldCommaLarge => Color::Cyan,
            FishSpecies::OldAngledFin => Color::Magenta,
            FishSpecies::OldCommaSmall => Color::Blue,
            FishSpecies::OldRounded => Color::Red,
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
    /// classic_mode: if true, only spawn old fish (matches -c flag)
    pub fn new_random(id: EntityId, screen_bounds: Rect, classic_mode: bool) -> Self {
        let mut rng = rand::thread_rng();

        let species = FishSpecies::random(classic_mode);
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
                // Start off-screen to the left, move right
                // Original Perl: X = 1 - WIDTH (fish starts fully off left edge)
                let x = 1.0 - sprite_bounds.0 as f32;
                let speed = rng.gen_range(0.5..2.0);
                (x, speed)
            }
            Direction::Left => {
                // Start near right edge, move left
                // Original Perl: X = width - 2 (fish starts mostly visible)
                let x = screen_bounds.width as f32 - 2.0;
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

        // Match Term::Animation die_offscreen behavior
        // Die when fish is completely off screen
        let off_left = (pos_x + sprite_bounds.0 as f32) < 0.0;
        let off_right = pos_x > (screen_bounds.width as f32);
        let off_top = (pos_y + sprite_bounds.1 as f32) < 0.0;
        let off_bottom = pos_y > (screen_bounds.height as f32);

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

    fn death_callback(&self) -> Option<DeathCallback> {
        Some(crate::spawning::add_fish)
    }

    fn should_spawn_bubble(&mut self, delta_time: Duration) -> Option<Position> {
        if !self.alive {
            return None;
        }

        if self.should_emit_bubble(delta_time) {
            Some(self.get_bubble_position())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::depth;

    #[test]
    fn test_fish_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let fish = Fish::new_random(1, screen_bounds, false);

        assert!(fish.is_alive());
        assert_eq!(fish.entity_type(), "fish");
        assert!(depth::is_fish_depth(fish.depth()));
    }

    #[test]
    fn test_fish_species_count() {
        assert_eq!(FishSpecies::new_species().len(), 4);
        assert_eq!(FishSpecies::old_species().len(), 8);
    }

    #[test]
    fn test_fish_category() {
        assert_eq!(FishSpecies::NewSmall1.category(), FishCategory::New);
        assert_eq!(FishSpecies::OldFancy.category(), FishCategory::Old);
    }

    #[test]
    fn test_classic_mode_only_spawns_old_fish() {
        let screen_bounds = Rect::new(0, 0, 80, 24);

        // Test multiple fish to ensure all are old
        for i in 0..20 {
            let fish = Fish::new_random(i, screen_bounds, true);
            assert_eq!(
                fish.species().category(),
                FishCategory::Old,
                "Classic mode should only spawn old fish"
            );
        }
    }

    #[test]
    fn test_fish_species_sprites() {
        // Test that all species have valid sprites
        let all_species = [
            FishSpecies::NewSmall1,
            FishSpecies::NewSmall2,
            FishSpecies::NewMedium1,
            FishSpecies::NewMedium2,
            FishSpecies::OldFancy,
            FishSpecies::OldSimple,
            FishSpecies::OldWavy,
            FishSpecies::OldTiny,
            FishSpecies::OldCommaLarge,
            FishSpecies::OldAngledFin,
            FishSpecies::OldCommaSmall,
            FishSpecies::OldRounded,
        ];

        for species in all_species {
            let (right, left) = species.get_sprites();
            assert!(
                !right.lines.is_empty(),
                "Species {:?} has empty right sprite",
                species
            );
            assert!(
                !left.lines.is_empty(),
                "Species {:?} has empty left sprite",
                species
            );
            assert_ne!(
                right.lines, left.lines,
                "Species {:?} has identical sprites",
                species
            );
        }
    }

    #[test]
    fn test_fish_movement() {
        let mut fish = Fish::new(
            1,
            Position::new(10.0, 10.0, depth::FISH_START),
            Velocity::new(1.0, 0.0),
            Direction::Right,
            FishSpecies::NewSmall1,
        );

        let initial_x = fish.position().x;
        fish.update(Duration::from_millis(16), Rect::new(0, 0, 80, 24)); // ~60 FPS

        assert!(fish.position().x > initial_x); // Should move right
    }

    #[test]
    fn test_fish_selection_distribution() {
        // Test that fish selection follows approximately 25%/75% distribution
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let sample_size = 1000;
        let mut new_count = 0;
        let mut old_count = 0;

        for i in 0..sample_size {
            let fish = Fish::new_random(i, screen_bounds, false);
            match fish.species().category() {
                FishCategory::New => new_count += 1,
                FishCategory::Old => old_count += 1,
            }
        }

        let new_percentage = (new_count as f32 / sample_size as f32) * 100.0;
        let old_percentage = (old_count as f32 / sample_size as f32) * 100.0;

        // Allow 10% margin of error (15%-35% for new, 65%-85% for old)
        assert!(
            new_percentage >= 15.0 && new_percentage <= 35.0,
            "New fish percentage {} should be around 25%",
            new_percentage
        );
        assert!(
            old_percentage >= 65.0 && old_percentage <= 85.0,
            "Old fish percentage {} should be around 75%",
            old_percentage
        );
    }
}
