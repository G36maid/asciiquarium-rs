use ratatui::{buffer::Buffer, layout::Rect, style::Color};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Unique identifier for entities
pub type EntityId = u64;

/// Characters that are considered transparent and won't be rendered
pub const TRANSPARENCY_CHARS: &[char] = &[' ', '?', '·', '\0'];

/// Represents a sprite with ASCII art and optional color mask
#[derive(Debug, Clone)]
pub struct Sprite {
    pub lines: Vec<String>,
    pub color_mask: Option<Vec<String>>,
    pub transparent_chars: HashSet<char>,
}

impl Sprite {
    /// Create a new sprite from ASCII art and optional color mask
    pub fn from_ascii_art(art: &str, mask: Option<&str>) -> Self {
        let lines: Vec<String> = art.lines().map(|s| s.to_string()).collect();
        let color_mask = mask.map(|m| m.lines().map(|s| s.to_string()).collect());

        // Use the global transparency characters
        let transparent_chars = TRANSPARENCY_CHARS.iter().cloned().collect();

        Self {
            lines,
            color_mask,
            transparent_chars,
        }
    }

    /// Get the bounding box (width, height) of the sprite
    pub fn get_bounding_box(&self) -> (u16, u16) {
        let height = self.lines.len() as u16;
        let width = self
            .lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u16;
        (width, height)
    }

    /// Check if a character at given position is transparent
    pub fn is_transparent_at(&self, col: usize, row: usize) -> bool {
        if row >= self.lines.len() {
            return true;
        }

        let line = &self.lines[row];
        let chars: Vec<char> = line.chars().collect();

        if col >= chars.len() {
            return true;
        }

        self.transparent_chars.contains(&chars[col])
    }

    /// Get the character at given position, or space if out of bounds
    pub fn get_char_at(&self, col: usize, row: usize) -> char {
        if row >= self.lines.len() {
            return ' ';
        }

        let line = &self.lines[row];
        let chars: Vec<char> = line.chars().collect();

        if col >= chars.len() {
            return ' ';
        }

        chars[col]
    }

    /// Get the color for a character based on color mask
    pub fn get_color_at(&self, col: usize, row: usize) -> Option<Color> {
        let color_mask = self.color_mask.as_ref()?;

        if row >= color_mask.len() {
            return None;
        }

        let mask_line = &color_mask[row];
        let mask_chars: Vec<char> = mask_line.chars().collect();

        if col >= mask_chars.len() {
            return None;
        }

        // Convert color mask character to color
        match mask_chars[col] {
            'R' | 'r' => Some(Color::Red),
            'G' | 'g' => Some(Color::Green),
            'B' | 'b' => Some(Color::Blue),
            'Y' | 'y' => Some(Color::Yellow),
            'M' | 'm' => Some(Color::Magenta),
            'C' | 'c' => Some(Color::Cyan),
            'W' | 'w' => Some(Color::White),
            '1' => Some(Color::Red),
            '2' => Some(Color::Green),
            '3' => Some(Color::Yellow),
            '4' => Some(Color::Blue),
            '5' => Some(Color::Magenta),
            '6' => Some(Color::Cyan),
            '7' => Some(Color::White),
            _ => None,
        }
    }

    /// Get all non-transparent character positions relative to sprite origin
    pub fn get_non_transparent_positions(&self) -> HashSet<(u16, u16)> {
        let mut positions = HashSet::new();

        for (row, line) in self.lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if !self.transparent_chars.contains(&ch) {
                    positions.insert((col as u16, row as u16));
                }
            }
        }

        positions
    }
}

/// Direction an entity is facing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// Position in 3D space (x, y, depth)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub depth: u8,
}

impl Position {
    pub fn new(x: f32, y: f32, depth: u8) -> Self {
        Self { x, y, depth }
    }

    pub fn to_screen_coords(&self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}

/// Velocity for entity movement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    pub fn zero() -> Self {
        Self { dx: 0.0, dy: 0.0 }
    }
}

/// Animation state for entities with multiple frames
#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec<Sprite>,
    pub current_frame: usize,
    pub frame_duration: Duration,
    pub last_frame_time: Instant,
    pub looping: bool,
}

impl Animation {
    pub fn new(frames: Vec<Sprite>, frame_duration: Duration, looping: bool) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_duration,
            last_frame_time: Instant::now(),
            looping,
        }
    }

    pub fn update(&mut self) {
        if self.frames.len() <= 1 {
            return;
        }

        if self.last_frame_time.elapsed() >= self.frame_duration {
            self.advance_frame();
            self.last_frame_time = Instant::now();
        }
    }

    fn advance_frame(&mut self) {
        if self.current_frame + 1 >= self.frames.len() {
            if self.looping {
                self.current_frame = 0;
            }
        } else {
            self.current_frame += 1;
        }
    }

    pub fn get_current_sprite(&self) -> &Sprite {
        &self.frames[self.current_frame]
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = Instant::now();
    }
}

/// Core entity trait that all aquarium entities must implement
pub trait Entity {
    fn id(&self) -> EntityId;
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn velocity(&self) -> Velocity;
    fn set_velocity(&mut self, velocity: Velocity);
    fn depth(&self) -> u8;
    fn get_current_sprite(&self) -> &Sprite;
    fn update(&mut self, delta_time: Duration, screen_bounds: Rect);
    fn is_alive(&self) -> bool;
    fn kill(&mut self);
    fn entity_type(&self) -> &'static str;

    /// Check if this entity collides with another at given positions
    fn collides_with(&self, other: &dyn Entity) -> bool {
        let self_pos = self.position().to_screen_coords();
        let other_pos = other.position().to_screen_coords();

        let self_sprite = self.get_current_sprite();
        let other_sprite = other.get_current_sprite();

        let self_bounds = self_sprite.get_non_transparent_positions();
        let other_bounds = other_sprite.get_non_transparent_positions();

        // Check if any non-transparent pixels overlap
        for &(sx, sy) in &self_bounds {
            let world_x = self_pos.0 + sx;
            let world_y = self_pos.1 + sy;

            for &(ox, oy) in &other_bounds {
                let other_world_x = other_pos.0 + ox;
                let other_world_y = other_pos.1 + oy;

                if world_x == other_world_x && world_y == other_world_y {
                    return true;
                }
            }
        }

        false
    }

    /// Render the entity to the buffer with transparency
    fn render(&self, buffer: &mut Buffer, screen_bounds: Rect) {
        let position = self.position().to_screen_coords();
        let sprite = self.get_current_sprite();

        for (row_idx, line) in sprite.lines.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                let x = position.0.saturating_add(col_idx as u16);
                let y = position.1.saturating_add(row_idx as u16);

                // Check bounds
                if x >= screen_bounds.width || y >= screen_bounds.height {
                    continue;
                }

                // Skip transparent characters
                if sprite.is_transparent_at(col_idx, row_idx) {
                    continue;
                }

                // Get the cell and update it
                if x < buffer.area.width && y < buffer.area.height {
                    let cell = buffer.cell_mut((x, y)).unwrap();
                    cell.set_char(ch);

                    // Apply color from mask if available
                    if let Some(color) = sprite.get_color_at(col_idx, row_idx) {
                        cell.set_fg(color);
                    }
                }
            }
        }
    }
}

/// Entity manager handles all entities and rendering
pub struct EntityManager {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    depth_layers: HashMap<u8, Vec<EntityId>>,
    next_id: EntityId,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            depth_layers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_next_id(&self) -> EntityId {
        self.next_id
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let depth = entity.depth();

        // Update entity ID (this requires entities to implement a set_id method)
        // For now, we'll assume the entity constructor sets the ID

        self.depth_layers
            .entry(depth)
            .or_insert_with(Vec::new)
            .push(id);

        self.entities.insert(id, entity);
        id
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        if let Some(entity) = self.entities.remove(&id) {
            let depth = entity.depth();
            if let Some(layer) = self.depth_layers.get_mut(&depth) {
                layer.retain(|&entity_id| entity_id != id);
                if layer.is_empty() {
                    self.depth_layers.remove(&depth);
                }
            }
        }
    }

    pub fn update_all(&mut self, delta_time: Duration, screen_bounds: Rect) {
        let mut dead_entities = Vec::new();

        for (id, entity) in &mut self.entities {
            entity.update(delta_time, screen_bounds);
            if !entity.is_alive() {
                dead_entities.push(*id);
            }
        }

        // Remove dead entities
        for id in dead_entities {
            self.remove_entity(id);
        }
    }

    pub fn render_all(&self, buffer: &mut Buffer, screen_bounds: Rect) {
        // Get all depth layers and sort them (render back to front)
        let mut depths: Vec<u8> = self.depth_layers.keys().cloned().collect();
        depths.sort_by(|a, b| b.cmp(a)); // Reverse order: higher depth first (background)

        for depth in depths {
            if let Some(entity_ids) = self.depth_layers.get(&depth) {
                for &entity_id in entity_ids {
                    if let Some(entity) = self.entities.get(&entity_id) {
                        entity.render(buffer, screen_bounds);
                    }
                }
            }
        }
    }

    pub fn get_entities_by_type(&self, entity_type: &str) -> Vec<&dyn Entity> {
        self.entities
            .values()
            .filter(|entity| entity.entity_type() == entity_type)
            .map(|boxed| boxed.as_ref())
            .collect()
    }

    pub fn check_collisions(&self) -> Vec<(EntityId, EntityId)> {
        let mut collisions = Vec::new();
        let entity_ids: Vec<EntityId> = self.entities.keys().cloned().collect();

        for i in 0..entity_ids.len() {
            for j in (i + 1)..entity_ids.len() {
                let id1 = entity_ids[i];
                let id2 = entity_ids[j];

                if let (Some(entity1), Some(entity2)) =
                    (self.entities.get(&id1), self.entities.get(&id2))
                {
                    if entity1.collides_with(entity2.as_ref()) {
                        collisions.push((id1, id2));
                    }
                }
            }
        }

        collisions
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
