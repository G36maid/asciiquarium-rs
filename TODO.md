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

## Phase 2: Environment System ✅ (PARTIALLY COMPLETED)

### Water Surface ✅ (COMPLETED)
- [x] Implement 4-layer water surface animation
  - [x] Tileable water segments
  - [x] Wave animation timing (8-frame smooth scrolling)
  - [x] Cyan coloring
- [x] Screen-width tiling system
- [x] Water surface collision detection (bubbles pop at surface)
- [x] Dynamic screen resize handling
- [x] Proper depth layering with fish swimming behind/in front

### Background Elements ✅ (COMPLETED)
- [x] Castle implementation
  - [x] Multi-line ASCII art rendering
  - [x] Color mask application (red/yellow highlights)
  - [x] Positioning (bottom-right)
- [x] Depth layer management
- [x] Static entity system
- [x] CastleManager for lifecycle management
- [x] Screen resize handling with repositioning
- [x] Integration with main app loop

### Seaweed System ✅ (COMPLETED)
- [x] Seaweed entity implementation
  - [x] Random height generation (3-7 chars)
  - [x] 2-frame swaying animation
  - [x] Green coloring
- [x] Lifecycle management (8-12 minute lifespan)
- [x] Random positioning across screen bottom
- [x] Animation timing (0.25-0.30 fps)
- [x] SeaweedManager for population control
- [x] Screen-size based target count (width/15)
- [x] Integration with main app loop

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

**Active Phase**: Phase 3 - Fish System Enhancement & Phase 4 - Large Entities
**Next Milestone**: Fix color mapping and add more fish species, then implement sharks and whales
**Recent Completion**: Animated 4-layer water surface system

### Testing Status
- ✅ **Bubbles**: Now visible as bright cyan characters rising from fish
- ✅ **Fish Movement**: Fish spawn off-screen and swim across properly
- ✅ **Fish Lifecycle**: Fish disappear when leaving screen boundaries
- ✅ **Screen Adaptation**: Works correctly on different terminal sizes

### Recent Fixes
- ✅ Fixed fish direction mapping (sprites now face correct direction)
- ✅ Fixed fish positioning (alternating left/right starts)
- ✅ Fixed entity ID assignment for proper direction alternation
- ✅ Implemented basic transparency system
- ✅ Added depth-based rendering
- ✅ Implemented bubble system with 5-frame animation
- ✅ Added fish-to-bubble spawning with direction awareness
- ✅ Bubble physics with surface collision detection
- ✅ **FIXED: Background transparency** - Removed solid blue background hiding bubbles
- ✅ **FIXED: Fish lifecycle** - Fish now spawn completely off-screen and die when leaving
- ✅ **FIXED: Screen bounds** - App now uses actual terminal size instead of hardcoded 80x24
- ✅ **Animated Water Surface** - 4-layer wave animation with proper depth sorting and screen adaptation
- ✅ **Seaweed System** - Implemented swaying seaweed with 2-frame animation, population management, and proper lifecycle
- ✅ **Castle Background** - Static castle decoration with red/yellow color highlights, proper positioning, and screen resize handling

### Known Issues to Address
- 🔧 **Color Issue**: Fish colors don't exactly match original code masks
- 🔧 **Missing Species**: Need to add more fish species from original
- 🔧 **Performance**: May need optimization for larger screens
- 🔧 **Debug Output**: Remove debug print statements from fish off-screen detection

---

## Notes

- Keep compatibility with original Perl version behavior
- Maintain ASCII art authenticity from original
- Focus on performance and smooth animation
- Ensure graceful handling of various terminal sizes
- Preserve the meditative, relaxing nature of the original

### Priority Next Steps
1. **Fix Color Mapping** - Make fish colors match original color mask system exactly
2. **Add More Fish Species** - Port remaining fish designs from original
3. **Large Creatures** - Begin Phase 4 with sharks and whales
4. **Performance Optimization** - Optimize for larger screens and many entities
5. **Advanced Features** - Collision system and random object spawning