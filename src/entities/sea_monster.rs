use crate::entity::{DeathCallback, Direction, Entity, EntityId, Position, Sprite, Velocity};
use rand::Rng;
use ratatui::layout::Rect;
use std::time::{Duration, Instant};

pub struct SeaMonster {
    id: EntityId,
    position: Position,
    velocity: Velocity,
    direction: Direction,
    animation_frame: usize,
    last_frame_time: Instant,
    sprites: Vec<Sprite>,
    #[allow(dead_code)]
    created_at: Instant,
    alive: bool,
}

impl SeaMonster {
    pub fn new(id: EntityId, screen_bounds: Rect, classic_mode: bool) -> Self {
        let mut rng = rand::thread_rng();

        // Random direction
        let direction = if rng.gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };

        // Starting position and velocity
        // Match original Perl asymmetric spawn behavior
        let (x, dx) = match direction {
            Direction::Right => {
                // Start off-screen left, move right
                // Original: x = -54 (new monster) or -64 (old monster)
                if classic_mode {
                    (-64.0, 2.0)
                } else {
                    (-54.0, 2.0)
                }
            }
            Direction::Left => {
                // Start near right edge, move left
                // Original: x = width - 2
                (screen_bounds.width as f32 - 2.0, -2.0)
            }
        };

        let y = 2.0; // Slightly below surface
        let depth = 5; // water_gap2 depth

        let position = Position::new(x, y, depth);
        let velocity = Velocity::new(dx, 0.0);

        // Create animation sprites based on mode
        let sprites = if classic_mode {
            Self::create_old_monster_sprites(&direction)
        } else {
            Self::create_new_monster_sprites(&direction)
        };

        Self {
            id,
            position,
            velocity,
            direction,
            animation_frame: 0,
            last_frame_time: Instant::now(),
            sprites,
            created_at: Instant::now(),
            alive: true,
        }
    }

    fn create_new_monster_sprites(direction: &Direction) -> Vec<Sprite> {
        match direction {
            Direction::Right => {
                vec![
                    // Frame 0
                    Sprite::from_ascii_art(
                        "\n         _???_?????????????????????_???_???????_a_a\n       _{.`=`.}_??????_???_??????_{.`=`.}_????{/ ''\\_\n _????{.'  _  '.}????{.`'`.}????{.'  _  '.}??{|  ._oo)\n{ \\??{/  .'?'.  \\}??{/ .-. \\}??{/  .'?'.  \\}?{/  |",
                        Some("\n                                                W W\n\n\n"),
                    ),
                    // Frame 1
                    Sprite::from_ascii_art(
                        "\n                      _???_????????????????????_a_a\n  _??????_???_??????_{.`=`.}_??????_???_??????{/ ''\\_\n { \\????{.`'`.}????{.'  _  '.}????{.`'`.}????{|  ._oo)\n  \\ \\??{/ .-. \\}??{/  .'?'.  \\}??{/ .-. \\}???{/  |",
                        Some("\n                                                W W\n\n\n"),
                    ),
                ]
            }
            Direction::Left => {
                vec![
                    // Frame 0
                    Sprite::from_ascii_art(
                        "\n   a_a_???????_???_?????????????????????_???_\n _/'' \\}????_{.`=`.}_??????_???_??????_{.`=`.}_\n(oo_.  |}??{.'  _  '.}????{.`'`.}????{.'  _  '.}????_\n    |  \\}?{/  .'?'.  \\}??{/ .-. \\}??{/  .'?'.  \\}??/ }",
                        Some("\n   W W\n\n\n"),
                    ),
                    // Frame 1
                    Sprite::from_ascii_art(
                        "\n   a_a_????????????????????_   _\n _/'' \\}??????_???_??????_{.`=`.}_??????_???_??????_\n(oo_.  |}????{.`'`.}????{.'  _  '.}????{.`'`.}????/ }\n    |  \\}???{/ .-. \\}??{/  .'?'.  \\}??{/ .-. \\}??/ /",
                        Some("\n   W W\n\n\n"),
                    ),
                ]
            }
        }
    }

    fn create_old_monster_sprites(direction: &Direction) -> Vec<Sprite> {
        // Original Perl old/classic monster (4 animation frames)
        match direction {
            Direction::Right => {
                vec![
                    // Frame 0
                    Sprite::from_ascii_art(
                        "\n                                                          ____\n            __??????????????????????????????????????????/   o  \\\n          /    \\????????_?????????????????????_???????/     ____ >\n  _??????|  __  |?????/   \\????????_????????/   \\????|     |\n | \\?????|  ||  |????|     |?????/   \\?????|     |???|     |",
                        Some("\n\n                                                            W\n\n\n"),
                    ),
                    // Frame 1
                    Sprite::from_ascii_art(
                        "\n                                                          ____\n                                             __?????????/   o  \\\n             _?????????????????????_???????/    \\?????/     ____ >\n   _???????/   \\????????_????????/   \\????|  __  |???|     |\n  | \\?????|     |?????/   \\?????|     |???|  ||  |???|     |",
                        Some("\n\n                                                            W\n\n\n"),
                    ),
                    // Frame 2
                    Sprite::from_ascii_art(
                        "\n                                                          ____\n                                  __????????????????????/   o  \\\n _??????????????????????_???????/    \\????????_???????/     ____ >\n| \\??????????_????????/   \\????|  __  |?????/   \\????|     |\n \\ \\???????/   \\?????|     |???|  ||  |????|     |???|     |",
                        Some("\n\n                                                            W\n\n\n"),
                    ),
                    // Frame 3
                    Sprite::from_ascii_art(
                        "\n                                                          ____\n                       __???????????????????????????????/   o  \\\n  _??????????_???????/    \\????????_??????????????????/     ____ >\n | \\???????/   \\????|  __  |?????/   \\????????_??????|     |\n  \\ \\?????|     |???|  ||  |????|     |?????/   \\????|     |",
                        Some("\n\n                                                            W\n\n\n"),
                    ),
                ]
            }
            Direction::Left => {
                vec![
                    // Frame 0
                    Sprite::from_ascii_art(
                        "\n    ____\n  /  o   \\??????????????????????????????????????????__\n< ____     \\???????_?????????????????????_????????/    \\\n      |     |????/   \\????????_????????/   \\?????|  __  |??????_\n      |     |???|     |?????/   \\?????|     |????|  ||  |?????/ |",
                        Some("\n\n     W\n\n\n"),
                    ),
                    // Frame 1
                    Sprite::from_ascii_art(
                        "\n    ____\n  /  o   \\?????????__\n< ____     \\?????/    \\???????_?????????????????????_\n      |     |???|  __  |????/   \\????????_????????/   \\???????_\n      |     |???|  ||  |???|     |?????/   \\?????|     |?????/ |",
                        Some("\n\n     W\n\n\n"),
                    ),
                    // Frame 2
                    Sprite::from_ascii_art(
                        "\n    ____\n  /  o   \\????????????????????__\n< ____     \\???????_????????/    \\???????_??????????????????????_\n      |     |????/   \\?????|  __  |????/   \\????????_??????????/ |\n      |     |???|     |????|  ||  |???|     |?????/   \\???????/ /",
                        Some("\n\n     W\n\n\n"),
                    ),
                    // Frame 3
                    Sprite::from_ascii_art(
                        "\n    ____\n  /  o   \\???????????????????????????????__\n< ____     \\??????????????????_????????/    \\???????_??????????_\n      |     |??????_????????/   \\?????|  __  |????/   \\???????/ |\n      |     |????/   \\?????|     |????|  ||  |???|     |?????/ /",
                        Some("\n\n     W\n\n\n"),
                    ),
                ]
            }
        }
    }

    fn update_animation(&mut self) {
        // Update animation frame every 250ms for tentacle movement
        if self.last_frame_time.elapsed().as_millis() > 250 {
            self.animation_frame = (self.animation_frame + 1) % self.sprites.len();
            self.last_frame_time = Instant::now();
        }
    }

    fn check_offscreen_death(&mut self, screen_bounds: Rect) {
        let is_off_screen = match self.direction {
            Direction::Right => self.position.x > screen_bounds.width as f32 + 60.0,
            Direction::Left => self.position.x < -60.0,
        };

        if is_off_screen {
            self.alive = false;
        }
    }
}

impl Entity for SeaMonster {
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
        &self.sprites[self.animation_frame]
    }

    fn update(&mut self, delta_time: Duration, screen_bounds: Rect) {
        if !self.alive {
            return;
        }

        // Update animation
        self.update_animation();

        // Update position based on velocity
        self.position.x += self.velocity.dx * delta_time.as_secs_f32() * 60.0; // Scale for 60 FPS

        // Check if monster should die (off-screen)
        self.check_offscreen_death(screen_bounds);
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn entity_type(&self) -> &'static str {
        "sea_monster"
    }

    fn death_callback(&self) -> Option<DeathCallback> {
        Some(crate::spawning::random_object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sea_monster_creation() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let monster = SeaMonster::new(1, screen_bounds, false);

        assert!(monster.is_alive());
        assert_eq!(monster.entity_type(), "sea_monster");
        assert_eq!(monster.depth(), 5); // water_gap2 depth
    }

    #[test]
    fn test_sea_monster_direction_and_position() {
        let screen_bounds = Rect::new(0, 0, 80, 24);

        // Test multiple monsters to check randomization (modern mode)
        for _ in 0..10 {
            let monster = SeaMonster::new(1, screen_bounds, false);

            match monster.direction {
                Direction::Right => {
                    assert_eq!(monster.position().x, -54.0);
                    assert_eq!(monster.velocity().dx, 2.0);
                }
                Direction::Left => {
                    assert_eq!(monster.position().x, 78.0); // screen_width - 2
                    assert_eq!(monster.velocity().dx, -2.0);
                }
            }
            assert_eq!(monster.position().y, 2.0); // Slightly below surface
        }
    }

    #[test]
    fn test_sea_monster_sprite_creation() {
        let right_sprites = SeaMonster::create_new_monster_sprites(&Direction::Right);
        let left_sprites = SeaMonster::create_new_monster_sprites(&Direction::Left);

        assert_eq!(right_sprites.len(), 2); // Two animation frames
        assert_eq!(left_sprites.len(), 2); // Two animation frames

        // Check that sprites are not empty
        for sprite in &right_sprites {
            assert!(!sprite.lines.is_empty());
        }
        for sprite in &left_sprites {
            assert!(!sprite.lines.is_empty());
        }

        // Check that left and right sprites are different
        assert_ne!(right_sprites[0].lines, left_sprites[0].lines);
    }

    #[test]
    fn test_sea_monster_tentacle_features() {
        let right_sprites = SeaMonster::create_new_monster_sprites(&Direction::Right);
        let left_sprites = SeaMonster::create_new_monster_sprites(&Direction::Left);

        // Check that sprites contain monster features
        let right_text = right_sprites[0].lines.join("\n");
        let left_text = left_sprites[0].lines.join("\n");

        // Should contain tentacle-like characters
        assert!(right_text.contains("?"));
        assert!(left_text.contains("?"));

        // Should contain eyes (a_a)
        assert!(right_text.contains("a_a"));
        assert!(left_text.contains("a_a"));
    }

    #[test]
    fn test_sea_monster_animation_frames() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let monster_new = SeaMonster::new(1, screen_bounds, false);

        // New monster should have 2 animation frames
        assert_eq!(monster_new.sprites.len(), 2);
        assert_ne!(monster_new.sprites[0].lines, monster_new.sprites[1].lines);

        let monster_old = SeaMonster::new(2, screen_bounds, true);

        // Old monster should have 4 animation frames
        assert_eq!(monster_old.sprites.len(), 4);
    }

    #[test]
    fn test_sea_monster_animation_update() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut monster = SeaMonster::new(1, screen_bounds, false);

        let initial_frame = monster.animation_frame;

        // Animation should not update immediately
        monster.update_animation();
        assert_eq!(monster.animation_frame, initial_frame);

        // Simulate time passing
        monster.last_frame_time = Instant::now() - Duration::from_millis(300);
        monster.update_animation();

        // Frame should have advanced
        assert_ne!(monster.animation_frame, initial_frame);
    }

    #[test]
    fn test_sea_monster_movement() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut monster = SeaMonster::new(1, screen_bounds, false);

        let initial_x = monster.position().x;
        monster.update(Duration::from_millis(16), screen_bounds); // ~60 FPS

        // Monster should have moved (faster than whales/ships)
        assert_ne!(monster.position().x, initial_x);
    }

    #[test]
    fn test_sea_monster_speed() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let monster = SeaMonster::new(1, screen_bounds, false);

        // Sea monsters should move faster than whales (speed 2)
        assert_eq!(monster.velocity().dx.abs(), 2.0);
    }

    #[test]
    fn test_sea_monster_offscreen_death() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let mut monster = SeaMonster::new(1, screen_bounds, false);

        // Move monster far off screen
        match monster.direction {
            Direction::Right => monster.position.x = 200.0,
            Direction::Left => monster.position.x = -100.0,
        }

        monster.update(Duration::from_millis(16), screen_bounds);
        assert!(!monster.is_alive());
    }

    #[test]
    fn test_sea_monster_positioning() {
        let screen_bounds = Rect::new(0, 0, 80, 24);
        let monster = SeaMonster::new(1, screen_bounds, false);

        // Monsters should be slightly below surface (y=2) and water_gap2 depth
        assert_eq!(monster.position().y, 2.0);
        assert_eq!(monster.depth(), 5);
    }

    #[test]
    fn test_sea_monster_classic_mode() {
        let screen_bounds = Rect::new(0, 0, 80, 24);

        // Modern mode should use new sprites (2 frames)
        let monster_modern = SeaMonster::new(1, screen_bounds, false);
        assert_eq!(monster_modern.sprites.len(), 2);

        // Classic mode should use old sprites (4 frames)
        let monster_classic = SeaMonster::new(2, screen_bounds, true);
        assert_eq!(monster_classic.sprites.len(), 4);

        // Classic mode should have different spawn position for right-moving
        for _ in 0..10 {
            let monster = SeaMonster::new(3, screen_bounds, true);
            if monster.direction == Direction::Right {
                assert_eq!(monster.position.x, -64.0); // Old monster spawns at -64
            }
        }

        // Modern mode spawns at -54 for right-moving
        for _ in 0..10 {
            let monster = SeaMonster::new(4, screen_bounds, false);
            if monster.direction == Direction::Right {
                assert_eq!(monster.position.x, -54.0); // New monster spawns at -54
            }
        }
    }

    #[test]
    fn test_sea_monster_old_sprite_features() {
        let right_sprites = SeaMonster::create_old_monster_sprites(&Direction::Right);
        let left_sprites = SeaMonster::create_old_monster_sprites(&Direction::Left);

        // Old monster should have 4 frames
        assert_eq!(right_sprites.len(), 4);
        assert_eq!(left_sprites.len(), 4);

        // Check that sprites contain monster features
        for sprite in &right_sprites {
            let text = sprite.lines.join("\n");
            assert!(text.contains("?"));
            assert!(text.contains("o")); // Eye
        }

        for sprite in &left_sprites {
            let text = sprite.lines.join("\n");
            assert!(text.contains("?"));
            assert!(text.contains("o")); // Eye
        }
    }
}
