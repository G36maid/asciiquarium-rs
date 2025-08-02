# Asciiquarium-rs Specification

## Overview

Asciiquarium-rs is a faithful Rust implementation of the classic ASCII art aquarium animation. This specification is based on detailed analysis of the original Perl asciiquarium v1.1 by Kirk Baucom and defines the requirements for an exact behavioral match using Ratatui.

## Critical Behavioral Analysis

### Death Callback System (Core Architecture)

The original asciiquarium uses a **death callback chain system** rather than independent timers:

1. **Initialization Phase**: 
   - All environment elements are created once
   - Fish population calculated and spawned based on screen size
   - Seaweed population calculated and spawned based on screen width  
   - **Exactly one** random large creature is spawned initially
   
2. **Runtime Phase**:
   - Animation loop only animates existing entities
   - **No active spawning** during runtime
   - All new entities are spawned via death callbacks

3. **Death Callback Chain**:
   ```
   Large Creature Dies → random_object() → New Random Large Creature
   Fish Dies → add_fish() → New Fish (maintains population)
   Seaweed Dies → add_seaweed() → New Seaweed (maintains population)
   Shark Dies → shark_death() → Clean up teeth → random_object()
   ```

**Key Insight**: There is always exactly one large creature on screen at any time, never multiple simultaneously.

### Original Spawning Formulas

#### Fish Population
```perl
my $screen_size = ($anim->height() - 9) * $anim->width();
my $fish_count = int($screen_size / 350);
```
- Subtracts 9 lines for water surface area
- One fish per 350 screen characters
- All fish spawned at initialization
- Population maintained via death callbacks

#### Seaweed Population  
```perl
my $seaweed_count = int($anim->width() / 15);
```
- One seaweed plant per 15 columns of screen width
- All seaweed spawned at initialization
- Lives 8-12 minutes then respawns via death callback

#### Random Objects (Large Creatures)
```perl
my @random_objects = (
    \&add_ship,
    \&add_whale, 
    \&add_monster,
    \&add_big_fish,
    \&add_shark,
);
```
- Random selection: `int(rand(scalar(@random_objects)))`
- **One at a time**: Single large creature chain
- Death triggers next random selection

## Entity System Architecture

### Depth Layers (Z-order)
```
Depth   Purpose                 Original Perl Constant
0-1     GUI elements           guiText=0, gui=1
2       Sharks                 shark=2  
2-9     Water surface layers   water_line3=2, water_gap3=3, 
                               water_line2=4, water_gap2=5,
                               water_line1=6, water_gap1=7, 
                               water_line0=8, water_gap0=9
3-20    Fish layers           fish_start=3, fish_end=20
21      Seaweed               seaweed=21
22      Castle                castle=22
```

### Entity Lifecycle

#### Fish
- **Spawn**: At random screen edge based on direction
- **Movement**: Horizontal only (`callback_args => [ $speed, 0, 0 ]`)
- **Death**: Off-screen detection (`die_offscreen => 1`)
- **Respawn**: Via death callback (`death_cb => \&add_fish`)
- **Collision**: Physical entities with collision handlers

#### Large Creatures
- **Spawn**: Random selection from object array
- **Movement**: Horizontal movement patterns
- **Death**: Off-screen detection
- **Respawn**: Via death callback to `random_object`
- **Positioning**:
  - Ships: Surface level (`water_gap1` depth)
  - Whales: Surface level (`water_gap2` depth)  
  - Monsters: Slightly submerged (`water_gap2` depth, y=2)
  - Big Fish: Shark depth (`shark` depth)

#### Seaweed
- **Spawn**: Random x position, bottom anchored
- **Lifespan**: 8-12 minutes (`die_time => time() + int(rand(4*60)) + (8*60)`)
- **Animation**: 2-frame sway, speed 0.25-0.30 fps
- **Respawn**: Via death callback (`death_cb => \&add_seaweed`)

## Animation System

### Frame Rates and Timing
- **Main Loop**: Controlled by `halfdelay(1)` (getch timeout)
- **Entity Animation**: Individual callback timing
- **Seaweed Sway**: 0.25-0.30 fps
- **Whale Spouts**: 7-frame sequence with timing
- **Fish Movement**: Smooth horizontal scrolling

### Color System
- **Fish Colors**: Randomized using `rand_color()` function
- **Color Masks**: Numeric codes (1-9) mapped to random colors per instance
- **Fixed Colors**: 
  - Seaweed: Green
  - Water: Cyan
  - Castle: Red/Yellow highlights
  - Ships: Yellow masts, White hulls

## Movement and Physics

### Fish Movement
```perl
callback_args => [ $speed, 0, 0 ]  # dx, dy, rotation
```
- **Horizontal Only**: No vertical movement
- **Speed**: Random per fish
- **Direction**: Alternates based on fish ID
- **No Schooling AI**: Depth layering creates schooling illusion

### Large Creature Movement
- **Ships**: `callback_args => [ $speed, 0, 0 ]`
- **Whales**: `callback_args => [ $speed, 0, 0, 1 ]` (with animation)
- **Monsters**: `callback_args => [ $speed, 0, 0, .25 ]` (with animation)
- **Speed Variation**: Different speeds per creature type

### Collision System
- **Shark Teeth**: Separate collision entity
- **Fish Collision**: `physical => 1, coll_handler => \&fish_collision`
- **Deadly Entities**: Sharks cause fish death
- **Bubble Physics**: Rise to surface, pop on contact

## User Interface

### Controls
- `q`: Quit application
- `r`: Redraw (restart initialization)
- `p`: Toggle pause
- `Ctrl+C`: Emergency exit

### Command Line Options
- `-c`: Classic mode (disables new fish and monsters)

### Screen Management
- Dynamic terminal size detection
- Full screen utilization
- Water surface adjusted to screen width
- Entity populations scale with screen size

## Implementation Requirements

### Death Callback System
```rust
trait Entity {
    fn on_death(&self, entity_manager: &mut EntityManager);
}

enum DeathCallback {
    SpawnFish,
    SpawnSeaweed, 
    SpawnRandomObject,
    SharkDeath, // Special: cleanup teeth + spawn random
}
```

### Random Object Manager
```rust
struct RandomObjectManager {
    creators: Vec<fn(EntityId, Rect) -> Box<dyn Entity>>,
    current_large_creature: Option<EntityId>,
}
```

### Initialization Sequence
1. Create water surface layers
2. Create castle
3. Calculate and spawn seaweed population
4. Calculate and spawn fish population  
5. Spawn exactly one random large creature
6. Enter animation loop (no active spawning)

## Known Behavioral Differences

### Current Implementation Issues
- ❌ **Multiple Large Creatures**: Our system spawns multiple simultaneously
- ❌ **Independent Timers**: Using timers instead of death callbacks
- ❌ **Continuous Spawning**: Active spawning during runtime vs. callback-only
- ❌ **Population Management**: Re-calculating instead of death-callback maintenance

### Required Fixes
1. **Implement Death Callback System**: Replace timer-based spawning
2. **Single Large Creature**: Ensure only one at a time
3. **Initialization vs. Runtime**: Separate spawn logic phases  
4. **Population Formulas**: Use exact original calculations
5. **Seaweed Lifecycle**: 8-12 minute lifespan with death callbacks

## Original Animation Sequences

### Whale Water Spout (7 frames)
```
Frame 0: "\n\n\n   :"
Frame 1: "\n\n   :\n   :"  
Frame 2: "\n  . .\n  -:-\n   :"
Frame 3: "\n  . .\n .-:-.\n   :"
Frame 4: "\n  . .\n'.-:-.`\n'  :  '"
Frame 5: "\n\n .- -.\n;  :  ;"
Frame 6: "\n\n\n;     ;"
```

### Seaweed Sway (2 frames)
```
Frame 0: "("
Frame 1: " )"
```

### Bubble Rise (5 frames)  
```
Frame 0: "."
Frame 1: "o"
Frame 2: "O" 
Frame 3: "O"
Frame 4: "O"
```

## Performance Requirements

### Entity Limits
- **Fish**: Based on screen size formula
- **Seaweed**: Based on screen width formula
- **Large Creatures**: Exactly 1
- **Bubbles**: Dynamic based on fish activity

### Memory Management
- Entity cleanup via death callbacks
- No entity accumulation over time
- Stable memory usage during long runs

## Compatibility Goals

The Rust implementation must be **behaviorally identical** to the original:
- Same entity counts and population formulas
- Same movement patterns and speeds  
- Same spawning behavior (death callbacks only)
- Same visual appearance and timing
- Same color randomization system

**Success Criteria**: An observer should not be able to distinguish between the original Perl version and the Rust version when running in the same terminal.