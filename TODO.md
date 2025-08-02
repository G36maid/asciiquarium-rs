# Asciiquarium-rs TODO

This document outlines the development roadmap for asciiquarium-rs, a faithful Rust port of the classic ASCII aquarium animation.

## 🎯 Current Status

**Version**: Development (Phase 4 - Large Creatures)
**Core Features**: ✅ Complete and working
**Environment**: ✅ Complete (water, seaweed, castle)
**Fish System**: ✅ Enhanced with authentic behavior
**Predators**: ✅ Sharks implemented

### ✅ Working Features
- **Core Framework**: Entity system, rendering, input handling, screen management
- **Environment**: 4-layer animated water surface, swaying seaweed, castle decoration
- **Fish System**: 6 species with authentic movement, randomized colors, bubble generation
- **Shark System**: Predatory sharks with collision detection and fish hunting
- **Color System**: Authentic Perl-style color randomization
- **Screen Adaptation**: Dynamic resizing and entity scaling

## 🚧 Active Development

### Phase 4: Large Creatures (In Progress)

#### Next Priority: Whale System
- [ ] Whale entity implementation
  - [ ] Large ASCII art (left/right variants)
  - [ ] Blue/cyan coloring
  - [ ] Horizontal movement
- [ ] Water spout animation
  - [ ] 7-frame spout sequence
  - [ ] Positioning above whale
  - [ ] Timing and triggering
- [ ] Whale manager and spawning

#### Ship System
- [ ] Ship entity implementation
  - [ ] Surface-level positioning
  - [ ] Detailed ASCII art
  - [ ] Horizontal movement across water
- [ ] Integration with random object spawning

#### Sea Monsters
- [ ] Monster entity variants
  - [ ] Two different monster designs
  - [ ] Large tentacle-like appendages
  - [ ] Dark/menacing coloring
- [ ] Monster behavior and movement

## 🎯 Next Up (Phase 5)

### Random Object System
- [ ] Random spawning manager
  - [ ] Probability-based entity selection
  - [ ] Timing controls (every 30-60 seconds)
  - [ ] Population limits
- [ ] Object lifecycle management
- [ ] Enhanced collision system

### Big Fish Species
- [ ] Complete Large1 and Large2 fish implementations
  - [ ] Port original big fish ASCII art from Perl
  - [ ] Proper color masks and randomization
  - [ ] Enhanced movement patterns
- [ ] Old fish variants from original

## 🛠️ Technical Improvements

### Performance & Polish
- [ ] Performance optimization for larger screens
- [ ] Remove debug output from fish off-screen detection
- [ ] Memory usage optimization
- [ ] Frame rate stabilization

### Features
- [ ] Classic mode implementation (-c flag for original fish only)
- [ ] Enhanced error handling and recovery
- [ ] Configuration system

## 🧹 Technical Debt

### Code Quality
- [ ] Comprehensive error handling
- [ ] Remove unused `created_at` fields in water_surface.rs
- [ ] Code review and cleanup
- [ ] Documentation improvements

### Testing
- [ ] Integration tests for collision system
- [ ] Performance benchmarks
- [ ] Cross-platform testing
- [ ] Memory leak detection

## 🎨 Known Issues

### Minor Issues
- 🔧 **Seaweed Spawning**: Frequency differs from original (spawns every 5s vs original timing)
- 🔧 **Performance**: May need optimization for very large screens
- 🔧 **Placeholder Art**: Large1/Large2 fish still use simple placeholder sprites

### Future Enhancements
- [ ] Sound effects (terminal bell usage)
- [ ] Unicode fish variants (optional)
- [ ] Interactive elements
- [ ] Save/load state functionality

## 📋 Implementation Notes

### Architecture Decisions
- **Entity-Component System**: Flexible, performant architecture
- **Depth-based Rendering**: Proper Z-ordering with 22 depth layers
- **Character-level Transparency**: Faithful to original rendering
- **Modular Design**: Easy to add new creatures and features

### Compatibility Goals
- **Visual Authenticity**: Indistinguishable from original when possible
- **Behavior Matching**: Movement, timing, and interactions match original
- **Performance**: Smooth animation on typical terminal sizes
- **Extensibility**: Easy to add new features while maintaining compatibility
