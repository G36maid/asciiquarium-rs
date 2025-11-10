# Specification: asciiquarium-rs

A faithful Rust port of the classic ASCII aquarium animation using ratatui for terminal rendering.

## Overview

This is a complete reimplementation of Kirk Baucom's `asciiquarium` Perl script in Rust, maintaining authentic behavior while embracing modern Rust practices and the ratatui TUI framework.

## Architecture

### Core Design Philosophy

1. **Faithful to Original**: Matches the original Perl implementation's behavior, including:
   - Entity spawning mechanics
   - Death callback system for population management
   - Single large creature constraint
   - Screen size-based entity counts
   - Animation timing and movement

2. **Clean Rust Implementation**:
   - Simple functions instead of over-engineered managers
   - Entity trait for polymorphic behavior
   - Death callbacks using function pointers
   - Proper depth-based Z-ordering

3. **Ratatui Integration**:
   - Custom rendering using ratatui's Buffer API
   - Terminal event handling via crossterm
   - Efficient screen updates

### Module Structure

```
src/
├── main.rs                 # Entry point, terminal initialization
├── app.rs                  # Main application loop and state
├── entity.rs               # Entity trait, EntityManager, Sprite system
├── depth.rs                # Z-ordering depth constants
├── spawning.rs             # Entity spawning functions (death callbacks)
├── event.rs                # Event handling (keyboard, timer)
├── ui.rs                   # Rendering widget for ratatui
└── entities/
    ├── mod.rs              # Entity module exports
    ├── fish.rs             # Fish with multiple species
    ├── bubble.rs           # Air bubbles from fish
    ├── seaweed.rs          # Swaying seaweed
    ├── castle.rs           # Static castle decoration
    ├── water_surface.rs    # 4-layer animated water surface
    ├── whale.rs            # Whale with water spout
    ├── ship.rs             # Sailing ship
    ├── sea_monster.rs      # Sea monster
    └── shark.rs            # Shark with teeth
```

## Entity System

### Entity Trait

All entities implement the `Entity` trait:

```rust
pub trait Entity: Send + Sync {
    fn id(&self) -> EntityId;
    fn entity_type(&self) -> &str;
    fn position(&self) -> Position;
    fn velocity(&self) -> Velocity;
    fn depth(&self) -> u8;
    fn sprite(&self) -> &Sprite;
    fn direction(&self) -> Direction;
    
    fn update(&mut self, delta_time: Duration, screen_bounds: Rect);
    fn is_alive(&self) -> bool;
    fn death_callback(&self) -> Option<DeathCallback>;
    
    // Optional methods with default implementations
    fn should_generate_bubbles(&self) -> bool { false }
    fn bubble_generation_rate(&self) -> f32 { 0.0 }
    fn is_large_creature(&self) -> bool { false }
}
```

### Entity Manager

`EntityManager` manages all entities in the aquarium:

- **Entity Storage**: HashMap for O(1) lookup by ID
- **Depth Sorting**: Entities sorted by depth for proper rendering
- **Death Callbacks**: Automatically executes callbacks when entities die
- **Large Creature Tracking**: Enforces single large creature constraint
- **ID Generation**: Monotonic counter for unique entity IDs

### Sprite System

`Sprite` represents visual appearance:

- **ASCII Art**: Multi-line string representation
- **Color Masks**: Character-based color mapping
  - `1-7`: Random colors (fish body parts)
  - `R/r`: Red
  - `G/g`: Green
  - `Y/y`: Yellow
  - `B/b`: Blue
  - `M/m`: Magenta
  - `C/c`: Cyan
  - `W/w`: White
- **Transparency**: Configurable transparent characters (space, `?`, `·`)
- **Color Randomization**: Random color selection for numbered masks

## Depth System (Z-Ordering)

Based on original Perl depth system where **higher numbers render first** (background) and **lower numbers render last** (foreground):

```
Layer           Depth   Description
─────────────────────────────────────────────
GUI_TEXT        0       Future GUI elements
GUI             1       Future GUI elements
SHARK           2       Sharks (foreground)
WATER_LINE3     2       Top water line (same layer as shark)
WATER_GAP3      3       Water gap
FISH_START      3       Front fish layer
...             ...     Multiple fish layers
FISH_END        20      Back fish layer
SEAWEED         21      Seaweed plants
CASTLE          22      Castle (background)
```

### Water Surface Layering

The water surface uses 4 interleaved layers for wave animation:

- Layer 0: `WATER_LINE0` (depth 8), `WATER_GAP0` (depth 9)
- Layer 1: `WATER_LINE1` (depth 6), `WATER_GAP1` (depth 7)
- Layer 2: `WATER_LINE2` (depth 4), `WATER_GAP2` (depth 5)
- Layer 3: `WATER_LINE3` (depth 2), `WATER_GAP3` (depth 3)

## Entity Types

### Fish

**Species**: 6 types (Small1, Small2, Medium1, Medium2, Large1, Large2)

**Behavior**:
- Horizontal movement (left or right)
- Random depth in fish layer range (3-20)
- Randomized colors based on color mask
- Bubble generation (15% chance per update cycle)
- Die when off-screen
- Death callback: spawns new fish

**Spawning**: Population based on screen size: `(height - 9) * width / 350`

### Bubbles

**Behavior**:
- Rise vertically (negative Y velocity)
- 5-frame animation: `.` → `o` → `O` → `O` → `O`
- Cyan color
- Pop when reaching water surface
- Die when off-screen

**Spawning**: Generated by fish randomly

### Seaweed

**Behavior**:
- 2-frame sway animation: `(` and `)` sides alternate
- Random height (3-7 segments)
- Anchored to bottom of screen
- Random X position
- Lives 8-12 minutes then dies
- Death callback: spawns new seaweed

**Spawning**: Count based on screen width: `width / 15`

### Castle

**Behavior**:
- Static decoration
- Multi-colored using color mask (red flags, yellow walls)
- Positioned at bottom-right of screen
- No movement or death

**Spawning**: Single instance, created at initialization

### Water Surface

**Behavior**:
- 4 layers with different wave patterns
- Each layer has 2 sub-layers (line + gap)
- Tiled horizontally to fill screen width
- Position fixed at Y=5 (top of water)
- Cyan color

**Spawning**: 4 layers created at initialization

### Whale

**Behavior**:
- Large creature (only one at a time)
- Horizontal movement across screen
- Water spout animation (5 frames, alternates with body)
- Die when completely off-screen
- Death callback: spawns new random large creature

**Spawning**: Via `random_object()` when no large creature exists

### Ship

**Behavior**:
- Large creature (only one at a time)
- Horizontal movement across screen (surface level)
- Multi-colored using color mask
- Die when completely off-screen
- Death callback: spawns new random large creature

**Spawning**: Via `random_object()` when no large creature exists

### Sea Monster

**Behavior**:
- Large creature (only one at a time)
- Horizontal movement across screen
- Die when completely off-screen
- Death callback: spawns new random large creature

**Spawning**: Via `random_object()` when no large creature exists

### Shark

**Behavior**:
- Large creature (only one at a time)
- Horizontal movement across screen
- Separate teeth entity that follows shark
- Teeth use same velocity as shark body
- Both die together when off-screen
- Special death callback: removes teeth, spawns new random large creature

**Spawning**: Via `random_object()` when no large creature exists

### Big Fish

**Behavior**:
- Large creature (only one at a time)
- Two variants:
  - **Variant 1** (traditional): 14 lines tall, speed 3.0, classic mode compatible
  - **Variant 2** (stylized): 13 lines tall, speed 2.5, modern mode only
- Horizontal movement across screen
- In classic mode: only Variant 1 spawns
- In modern mode: 2/3 chance for Variant 2, 1/3 chance for Variant 1
- Die when completely off-screen
- Death callback: spawns new random large creature

**Spawning**: Via `random_object()` when no large creature exists

## Spawning System

### Death Callbacks

When an entity dies, its death callback is executed to maintain population:

```rust
pub type DeathCallback = fn(&mut EntityManager, Rect);
```

**Examples**:
- Fish death → `add_fish()` spawns a new fish
- Seaweed death → `add_seaweed()` spawns new seaweed
- Whale death → `random_object()` spawns new random large creature
- Shark death → `shark_death()` cleans up teeth, then spawns new random large creature

### Population Management

**Fish**: Constant population based on screen size
**Seaweed**: Constant population based on screen width
**Large Creatures**: Exactly one at a time (whale/ship/monster/shark)
**Bubbles**: Dynamic, generated by fish
**Castle/Water**: Static, never die

### Random Object Spawner

`random_object()` selects one large creature randomly:
- Ship
- Whale
- Sea Monster
- Big Fish (two variants: traditional and stylized)
- Shark

Only spawns if no large creature currently exists.

## Initialization Sequence

Matches original Perl initialization order:

1. `add_environment()` - Create 4-layer water surface
2. `add_castle()` - Create castle
3. `add_all_seaweed()` - Populate seaweed based on width
4. `add_all_fish()` - Populate fish based on screen size
5. `random_object()` - Spawn initial large creature

## Movement and Animation

### Update Cycle

1. Calculate delta time since last update
2. For each entity:
   - Update position based on velocity and delta time
   - Update animation frame if applicable
   - Check if still alive (on-screen, lifetime, etc.)
   - Generate bubbles if applicable
3. Remove dead entities and execute death callbacks
4. Sort entities by depth for rendering

### Velocity System

Position update: `position += velocity * delta_time`

Typical velocities:
- Fish: 5-15 units/second horizontal
- Bubbles: -10 units/second vertical (upward)
- Large creatures: 3-8 units/second horizontal
- Seaweed: 0 (stationary, animation only)

### Animation Timing

- **Seaweed**: ~0.25-0.3 seconds per frame
- **Whale Spout**: ~0.2 seconds per frame
- **Bubbles**: Frame based on Y position

## Rendering

### Ratatui Widget

`App` implements `Widget` trait for ratatui:

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer);
}
```

### Rendering Algorithm

1. Sort entities by depth (high to low, background to foreground)
2. For each entity:
   - Get sprite and position
   - For each character in sprite:
     - Skip if transparent
     - Check if within screen bounds
     - Get color from color mask
     - Write character to buffer at position

### Color Application

Colors applied per-character based on mask:
- Numbered colors (1-7): Random selection at creation
- Letter colors: Fixed colors (R=red, G=green, etc.)
- No mask: Use entity's default color

## Event Handling

### Keyboard Controls

- `q` / `Esc`: Quit application
- `Ctrl+C`: Quit application
- `p` / `P`: Toggle pause/unpause
- `r` / `R`: Redraw (recreate all entities)

### Event System

- **Tick Events**: 60 FPS target (16.67ms interval)
- **Crossterm Events**: Keyboard input
- **App Events**: Internal application events (quit)

### Pause Behavior

When paused:
- Entity updates are skipped
- Rendering continues
- User input still processed

## Screen Adaptation

### Dynamic Sizing

All entities adapt to terminal size:
- Fish spawn within visible bounds
- Seaweed count scales with width
- Fish count scales with screen area
- Water surface tiles to screen width
- Castle positions relative to screen size

### Redraw on Resize

Original Perl approach: Manual redraw with 'r' key
Rust implementation: Same approach (no automatic resize handling)

## Performance Considerations

### Efficient Updates

- Only update entities when not paused
- Delta-time based movement for smooth animation
- HashMap for O(1) entity lookup

### Memory Management

- Entities stored as `Box<dyn Entity>` for polymorphism
- Dead entities removed immediately
- No memory leaks from death callbacks

### Rendering Optimization

- Depth sorting done once per frame
- Transparency checks avoid overdraw
- Bounded character iteration (skip off-screen)

## Original Perl Fidelity

### Matching Behaviors

✅ Death callback system for population management
✅ Single large creature constraint
✅ Screen size-based entity counts
✅ Random color selection for fish
✅ Seaweed lifetime (8-12 minutes)
✅ Bubble generation from fish
✅ Water surface animation
✅ Depth-based Z-ordering
✅ Entity death when off-screen
✅ Pause/Resume/Redraw controls

### Implementation Differences

- **Language**: Rust instead of Perl
- **TUI Framework**: Ratatui instead of Term::Animation/Curses
- **Entity System**: Trait-based OOP instead of Perl blessed hashes
- **Type Safety**: Compile-time guarantees vs runtime checks

### Future Enhancements

Potential additions not in original:
- Automatic screen resize handling
- Configuration file for colors/counts
- Additional fish species
- Sound effects (terminal bell)
- Statistics display (entity counts)

## Testing

### Test Coverage

54 unit tests covering:
- Entity creation and initialization
- Movement and velocity
- Sprite rendering and color masks
- Death conditions
- Animation frames
- Depth functions
- Spawning logic

### Test Organization

Tests located in same files as implementation using `#[cfg(test)]`:
- `entities/fish.rs`: Fish behavior tests
- `entities/seaweed.rs`: Seaweed tests
- `entities/whale.rs`: Whale animation tests
- `depth.rs`: Depth layer tests
- etc.

## Dependencies

- **ratatui** (0.29.0): Terminal UI framework
- **crossterm** (0.28.1): Cross-platform terminal control
- **color-eyre** (0.6.3): Enhanced error reporting
- **rand** (0.8.5): Random number generation

## License

MIT License (matches original GPL for this port)

Original asciiquarium by Kirk Baucom licensed under GPL v2.