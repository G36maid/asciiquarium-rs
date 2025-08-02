# Asciiquarium-rs TODO

This document outlines the implementation roadmap for asciiquarium-rs, a Rust port of the classic ASCII aquarium animation.

## Phase 1: Core Framework ✅ (In Progress)

### Basic Infrastructure
- [x] Set up Rust project with Ratatui
- [x] Basic app structure with event handling
- [ ] Entity system foundation
  - [ ] Define Entity trait
  - [ ] Position and movement components
  - [ ] Animation frame system
  - [ ] Entity manager/registry
- [ ] Rendering pipeline
  - [ ] Depth-based rendering
  - [ ] Color support
  - [ ] ASCII art rendering utilities
- [ ] Input handling
  - [ ] Keyboard controls (q, r, p)
  - [ ] Signal handling for graceful exit
- [ ] Screen management
  - [ ] Dynamic resize handling
  - [ ] Coordinate system setup

## Phase 2: Environment System

### Water Surface
- [ ] Implement 4-layer water surface animation
  - [ ] Tileable water segments
  - [ ] Wave animation timing
  - [ ] Cyan coloring
- [ ] Screen-width tiling system
- [ ] Water surface collision detection

### Background Elements
- [ ] Castle implementation
  - [ ] Multi-line ASCII art rendering
  - [ ] Color mask application (red/yellow highlights)
  - [ ] Positioning (bottom-right)
- [ ] Depth layer management
- [ ] Static entity system

### Seaweed System
- [ ] Seaweed entity implementation
  - [ ] Random height generation (3-7 chars)
  - [ ] 2-frame swaying animation
  - [ ] Green coloring
- [ ] Lifecycle management (8-12 minute lifespan)
- [ ] Random positioning across screen bottom
- [ ] Animation timing (0.25-0.30 fps)

## Phase 3: Fish System

### Basic Fish
- [ ] Fish entity structure
  - [ ] Position, velocity, direction
  - [ ] Species variants
  - [ ] Age tracking
- [ ] Fish ASCII art assets
  - [ ] Classic fish designs (old generation)
  - [ ] New fish designs (enhanced generation)
  - [ ] Left/right directional sprites
  - [ ] Color mask definitions

### Fish Behavior
- [ ] Movement system
  - [ ] Horizontal swimming
  - [ ] Slight vertical drift
  - [ ] Speed variations by species
- [ ] Population management
  - [ ] Screen-size based spawning (area/350)
  - [ ] Continuous respawning
  - [ ] Off-screen death detection
- [ ] Fish AI
  - [ ] Random direction changes
  - [ ] Boundary avoidance
  - [ ] Depth layer distribution

### Bubble System
- [ ] Bubble entity implementation
  - [ ] 5-frame animation (., o, O, O, O)
  - [ ] Vertical movement (rising)
  - [ ] Cyan coloring
- [ ] Bubble generation
  - [ ] Random emission from fish
  - [ ] Position calculation based on fish direction
  - [ ] Timer-based spawning
- [ ] Bubble physics
  - [ ] Surface collision (popping)
  - [ ] Buoyancy simulation

## Phase 4: Predators and Large Entities

### Shark System
- [ ] Shark entity implementation
  - [ ] Large ASCII art (left/right variants)
  - [ ] White/cyan coloring
  - [ ] Horizontal movement
- [ ] Teeth system
  - [ ] Separate collision entity
  - [ ] Red coloring
  - [ ] Position synchronization with shark
- [ ] Fish predation
  - [ ] Collision detection with fish
  - [ ] Fish death animation/removal
  - [ ] Shark feeding behavior

### Whale System
- [ ] Whale entity implementation
  - [ ] Large ASCII art with direction variants
  - [ ] Blue/cyan coloring
  - [ ] Horizontal movement
- [ ] Water spout animation
  - [ ] 7-frame spout sequence
  - [ ] Positioning above whale
  - [ ] Timing and triggering
- [ ] Surface interaction effects

### Ships
- [ ] Ship entity implementation
  - [ ] Surface-level positioning
  - [ ] Detailed ASCII art
  - [ ] Horizontal movement across water
- [ ] Wake effects (optional enhancement)

### Sea Monsters
- [ ] Monster entity variants
  - [ ] Two different monster designs
  - [ ] Large tentacle-like appendages
  - [ ] Dark/menacing coloring
- [ ] Monster behavior
  - [ ] Horizontal movement
  - [ ] Intimidating presence
  - [ ] Interaction with other entities

### Big Fish
- [ ] Big fish variants
  - [ ] Two species implementations
  - [ ] Larger detailed ASCII art
  - [ ] Enhanced movement patterns
- [ ] Behavioral differences from regular fish

## Phase 5: Advanced Features

### Collision System
- [ ] Bounding box collision detection
- [ ] Entity type-based collision rules
- [ ] Collision callbacks and responses
- [ ] Performance optimization for many entities

### Random Object System
- [ ] Random spawning manager
  - [ ] Probability-based entity selection
  - [ ] Timing controls
  - [ ] Population limits
- [ ] Entity recycling system
- [ ] Memory management

### Animation Framework
- [ ] Frame-based animation system
- [ ] Configurable timing
- [ ] Animation state management
- [ ] Smooth interpolation

## Phase 6: Polish and Optimization

### Performance
- [ ] Entity pooling
- [ ] Efficient collision detection
- [ ] Memory usage optimization
- [ ] Frame rate stabilization
- [ ] Large screen handling

### User Experience
- [ ] Classic mode implementation (-c flag)
- [ ] Smooth pause/unpause
- [ ] Graceful redraw functionality
- [ ] Error handling and recovery

### Visual Polish
- [ ] Color fallback for limited terminals
- [ ] Smooth entity transitions
- [ ] Visual effects refinement
- [ ] ASCII art alignment perfection

## Phase 7: Testing and Documentation

### Testing
- [ ] Unit tests for entity system
- [ ] Integration tests for major features
- [ ] Performance benchmarks
- [ ] Terminal compatibility testing
- [ ] Memory leak detection

### Documentation
- [ ] Code documentation
- [ ] User manual
- [ ] Installation instructions
- [ ] Troubleshooting guide

### Packaging
- [ ] Cargo.toml metadata completion
- [ ] Release preparation
- [ ] Distribution testing
- [ ] Cross-platform verification

## Technical Debt and Refactoring

### Code Quality
- [ ] Comprehensive error handling
- [ ] Logging system implementation
- [ ] Configuration management
- [ ] Code review and cleanup

### Architecture
- [ ] Component system refinement
- [ ] Plugin architecture (for extensibility)
- [ ] Save/load state functionality
- [ ] Configurable entity behaviors

## Future Enhancements (Optional)

### Extended Features
- [ ] Sound effects (terminal bell usage)
- [ ] Additional fish species
- [ ] Interactive elements (feeding fish)
- [ ] Day/night cycle
- [ ] Weather effects

### Modern Touches
- [ ] Unicode fish variants
- [ ] Improved color palette
- [ ] Smooth animations
- [ ] Performance metrics display

## Current Status

**Active Phase**: Phase 1 - Core Framework
**Next Milestone**: Complete entity system foundation
**Estimated Completion**: TBD

---

## Notes

- Keep compatibility with original Perl version behavior
- Maintain ASCII art authenticity from original
- Focus on performance and smooth animation
- Ensure graceful handling of various terminal sizes
- Preserve the meditative, relaxing nature of the original