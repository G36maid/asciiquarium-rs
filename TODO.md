# Asciiquarium-rs TODO

This document outlines the implementation roadmap for asciiquarium-rs, a Rust port of the classic ASCII aquarium animation.

## Phase 1: Core Framework ✅ (COMPLETED)

### Basic Infrastructure
- [x] Set up Rust project with Ratatui
- [x] Basic app structure with event handling
- [x] Entity system foundation
  - [x] Define Entity trait
  - [x] Position and movement components
  - [x] Animation frame system
  - [x] Entity manager/registry
- [x] Rendering pipeline
  - [x] Depth-based rendering
  - [x] Color support (basic implementation)
  - [x] ASCII art rendering utilities
  - [x] Transparency system with character-level transparency
- [x] Input handling
  - [x] Keyboard controls (q, r, p)
  - [x] Signal handling for graceful exit
- [x] Screen management
  - [x] Dynamic resize handling
  - [x] Coordinate system setup

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

## Phase 3: Fish System ✅ (PARTIALLY COMPLETED)

### Basic Fish
- [x] Fish entity structure
  - [x] Position, velocity, direction
  - [x] Species variants (4 basic species implemented)
  - [x] Age tracking
- [x] Fish ASCII art assets
  - [x] Classic fish designs (2 species from original)
  - [ ] New fish designs (enhanced generation) - **TODO**: Add remaining species
  - [x] Left/right directional sprites (FIXED: proper direction mapping)
  - [x] Color mask definitions - **ISSUE**: Colors don't match original exactly

### Fish Behavior
- [x] Movement system
  - [x] Horizontal swimming
  - [x] Slight vertical drift
  - [x] Speed variations by species
- [x] Population management
  - [x] Screen-size based spawning (area/350)
  - [x] Continuous respawning
  - [x] Off-screen death detection
- [x] Fish AI
  - [x] Random direction changes (basic implementation)
  - [x] Boundary avoidance (off-screen death)
  - [x] Depth layer distribution

### Bubble System ✅ (COMPLETED)
- [x] Bubble entity implementation
  - [x] 5-frame animation (., o, O, O, O)
  - [x] Vertical movement (rising)
  - [x] Cyan coloring
- [x] Bubble generation
  - [x] Random emission from fish
  - [x] Position calculation based on fish direction
  - [x] Timer-based spawning
- [x] Bubble physics
  - [x] Surface collision (popping)
  - [x] Buoyancy simulation with upward acceleration

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

**Active Phase**: Phase 2 - Environment System (Water surface animation next)
**Next Milestone**: Implement animated water surface and seaweed
**Recent Completion**: Bubble system with fish emission and surface collision

### Recent Fixes
- ✅ Fixed fish direction mapping (sprites now face correct direction)
- ✅ Fixed fish positioning (alternating left/right starts)
- ✅ Fixed entity ID assignment for proper direction alternation
- ✅ Implemented basic transparency system
- ✅ Added depth-based rendering
- ✅ Implemented bubble system with 5-frame animation
- ✅ Added fish-to-bubble spawning with direction awareness
- ✅ Bubble physics with surface collision detection

### Known Issues to Address
- 🔧 **Color Issue**: Fish colors don't exactly match original code masks
- 🔧 **Missing Species**: Need to add more fish species from original
- 🔧 **Performance**: May need optimization for larger screens

---

## Notes

- Keep compatibility with original Perl version behavior
- Maintain ASCII art authenticity from original
- Focus on performance and smooth animation
- Ensure graceful handling of various terminal sizes
- Preserve the meditative, relaxing nature of the original

### Priority Next Steps
1. **Water Surface Animation** - Implement animated tileable water waves at top
2. **Fix Color Mapping** - Make fish colors match original color mask system
3. **Add Seaweed System** - Swaying seaweed plants at bottom
4. **Add More Fish Species** - Port remaining fish designs from original
5. **Castle Background** - Add the castle decoration at bottom-right