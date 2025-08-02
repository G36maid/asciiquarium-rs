# Asciiquarium-rs TODO

This document outlines the development roadmap for asciiquarium-rs, a faithful Rust port of the classic ASCII aquarium animation.

## 🎯 Current Status

**Version**: Development (Hybrid Approach - Original Behavior + Modern Improvements)
**Priority**: Authentic behavior with enhanced population management

### 🎯 **Hybrid Strategy: Best of Both Worlds**

**Original Behavior (Core Authenticity):**
- Large Creatures: Exactly one at a time via death callback chain
- Fish/Seaweed: Death callback respawning maintains populations
- Main Loop: Animation-focused with minimal spawning logic

**Our Improvements (Keep):**
- Dynamic screen size adaptation and population scaling
- Robust entity management system
- Modern error handling and performance
- Real-time population adjustments for screen changes

**Target Architecture:**
- Death callback system for authenticity
- Population managers for screen size adaptation
- Single large creature constraint
- Enhanced stability and user experience

## 🚨 Phase 1: Behavioral Authenticity with Modern Improvements

### Death Callback System (Core Authenticity)

#### Core Architecture Additions
- [ ] **Add death callback trait to Entity system**
- [ ] **Implement RandomObjectManager with single creature constraint**
- [ ] **Add death callback support alongside existing managers**
- [ ] **Hybrid spawning: Death callbacks + screen size adaptation**

#### Authentic Death Callback Chain
- [ ] **Large creature death → spawn single random new large creature**
- [ ] **Fish death → trigger population check and potential spawn**
- [ ] **Seaweed death → trigger population check and potential spawn**
- [ ] **Shark death → cleanup teeth + spawn random large creature**

#### Population Management (Enhanced Original)
```rust
// Base formulas (original)
fish_base = (height - 9) * width / 350
seaweed_base = width / 15

// Our enhancement: Dynamic scaling
fish_target = max(fish_base, min_fish_count)
seaweed_target = max(seaweed_base, min_seaweed_count)

// Random objects: exactly one (original)
random_objects = [ship, whale, monster, big_fish, shark]
```

### Hybrid Runtime Management
- [ ] **Enhanced initialization phase:**
  - [ ] Spawn water surface (dynamic sizing)
  - [ ] Spawn castle (screen-adaptive positioning)
  - [ ] Spawn calculated fish population (enhanced formula)
  - [ ] Spawn calculated seaweed population (enhanced formula)
  - [ ] Spawn exactly ONE random large creature (original behavior)
- [ ] **Optimized runtime loop:**
  - [ ] Keep population maintenance (our improvement)
  - [ ] Add death callback triggers (original behavior)
  - [ ] Maintain single large creature constraint (original behavior)
  - [ ] Keep screen size adaptation (our improvement)

### Entity Death Callbacks
- [ ] **Implement fish death callback** (spawn replacement fish)
- [ ] **Implement seaweed death callback** (spawn replacement seaweed)
- [ ] **Implement large creature death callback** (spawn random large creature)
- [ ] **Implement shark special death callback** (cleanup teeth + spawn random)

## 🎯 Phase 2: Missing Original Features

### Big Fish Species (Part of Random Objects)
- [ ] **add_big_fish_1() implementation**
  - [ ] Large detailed ASCII art from original
  - [ ] Yellow coloring (default_color => 'YELLOW')
  - [ ] Shark depth positioning
- [ ] **add_big_fish_2() implementation**
  - [ ] Alternative big fish design
  - [ ] Same coloring and depth as big_fish_1
- [ ] **Random selection between big fish variants**
- [ ] **Integration into death callback chain**

### Bubble System
- [ ] **Fish bubble generation**
  - [ ] Random chance for fish to emit bubbles
  - [ ] Bubble spawning at fish mouth position
- [ ] **Bubble physics**
  - [ ] 5-frame animation: `.`, `o`, `O`, `O`, `O`
  - [ ] Vertical rise to surface
  - [ ] Pop when reaching water surface
- [ ] **Bubble death callback** (bubbles just die, no replacement)

### Classic Mode (-c flag)
- [ ] **Command line argument parsing**
- [ ] **Disable new fish species in classic mode**
- [ ] **Disable new monsters in classic mode**
- [ ] **Use only original fish variants**

## 🧹 Phase 3: Refactor for Hybrid System

### Manager System Enhancement (EVOLVE, Don't Delete)
- [ ] **Enhance WhaleManager for death callback integration**
- [ ] **Enhance ShipManager for death callback integration**
- [ ] **Enhance SeaMonsterManager for death callback integration**
- [ ] **Enhance SharkManager for death callback integration**
- [ ] **Keep SeaweedManager population logic + add death callbacks**

### Spawning Logic Optimization
- [ ] **Add single large creature constraint to spawn methods**
- [ ] **Integrate death callback triggers with existing spawn logic**
- [ ] **Optimize spawn timing (reduce frequency, improve efficiency)**
- [ ] **Keep population management for screen size changes**
- [ ] **Add death callback registration system**

### App Structure Enhancement
- [ ] **Keep existing manager fields (they work well)**
- [ ] **Add RandomObjectManager alongside existing managers**
- [ ] **Enhance initialization method with hybrid approach**
- [ ] **Optimize tick() method for authenticity + modern features**

## 🎯 Phase 4: Behavioral Validation

### Original Behavior Testing
- [ ] **Single large creature constraint testing**
- [ ] **Death callback chain testing**
- [ ] **Population formula validation**
- [ ] **Seaweed 8-12 minute lifecycle testing**
- [ ] **Fish population stability testing**

### Performance Validation
- [ ] **Memory usage should be stable** (no entity accumulation)
- [ ] **Entity count should match original formulas**
- [ ] **Animation smoothness comparable to original**

### Visual Fidelity
- [ ] **Color system matches original**
- [ ] **Movement patterns match original**
- [ ] **Timing and animation speeds match original**
- [ ] **Screen size adaptation matches original**

## 🔴 Current Implementation Status

### ✅ **Correctly Implemented (Keep & Enhance)**
- **Entity ASCII art and sprites** (whale, ship, monster sprites are correct)
- **Fish color randomization** (matches original rand_color system)
- **Fish horizontal movement** (no vertical drift, correct)
- **Water surface animation** (4-layer system correct)
- **Castle positioning and art** (static background correct)
- **Seaweed sway animation** (2-frame correct)
- **Depth layer system** (Z-order correct)
- **Dynamic screen size adaptation** (our improvement - keep!)
- **Population scaling with screen changes** (our improvement - keep!)
- **Robust entity management** (our improvement - keep!)

### 🔧 **Needs Modification (Enhance, Don't Remove)**
- **Multiple simultaneous large creatures** → Add single creature constraint
- **Timer-based spawning** → Add death callback integration  
- **Population management** → Enhance with death callback triggers
- **Manager architecture** → Enhance for hybrid system

### 🎯 **Features to Add**
- **Death callback system** (core authenticity)
- **Single large creature constraint** (original behavior)
- **Big fish species** (complete random object system)
- **Bubble system** (fish bubble generation and physics)
- **Classic mode** (-c command line flag)

## 📋 Implementation Priority

### Priority 1: Behavioral Authenticity
1. **Death callback trait and system** (add to existing architecture)
2. **Single large creature constraint** (enhance RandomObjectManager)
3. **Hybrid population management** (death callbacks + screen adaptation)
4. **Authentic spawn patterns** (one large creature, callback-driven)

### Priority 2: Complete Feature Set
1. **Big fish species implementation** (complete random object system)
2. **Bubble system implementation** (authentic fish bubble physics)
3. **Classic mode support** (-c flag compatibility)

### Priority 3: Polish & Optimization
1. **Performance optimization** (leverage our robust architecture)
2. **Enhanced error handling** (build on our modern foundation)
3. **Documentation and testing** (comprehensive coverage)

## 🎯 Success Criteria

**Primary Goal**: Authentic behavior with modern enhancements
- Core behavior matches original (single large creature, death callbacks)
- Enhanced with dynamic screen adaptation and population scaling
- Same visual appearance, movement, and timing as original
- Better stability and user experience than original

**Secondary Goals**: Best-in-class implementation
- Clean hybrid architecture (authentic + modern)
- Efficient memory usage and performance
- Maintainable, well-tested codebase
- Enhanced features while preserving authenticity

## 📝 Notes

### Architecture Philosophy
- **Authenticity enhanced by modern practices**: Core behavior + improvements
- **Hybrid approach**: Death callbacks for authenticity + managers for robustness
- **Faithful evolution**: Preserve original feel, enhance user experience

### Development Approach
- **Enhance existing architecture**: Build on what works
- **Add authenticity features**: Death callbacks + single creature constraint
- **Validate behavior**: Test against original while preserving improvements
- **Iterative enhancement**: Improve without breaking existing functionality