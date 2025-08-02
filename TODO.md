# Asciiquarium-rs TODO

This document outlines the development roadmap for asciiquarium-rs, a faithful Rust port of the classic ASCII aquarium animation.

## 🎯 Current Status

**Version**: Development - Phase 1 Complete (Death Callbacks + Simplified Architecture)
**Priority**: Phase 2 - Complete missing features (Big Fish, Bubbles, Classic Mode)

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

## ✅ Phase 1: COMPLETED - Behavioral Authenticity with Modern Improvements

### Death Callback System (Core Authenticity) - COMPLETED
- ✅ **Added death callback trait to Entity system**
- ✅ **Implemented single large creature constraint in EntityManager**
- ✅ **Replaced complex managers with simple spawning functions**
- ✅ **Death callbacks + screen size adaptation working**

#### Authentic Death Callback Chain - COMPLETED
- ✅ **Large creature death → spawn single random new large creature**
- ✅ **Fish death → spawn replacement fish (maintains population)**
- ✅ **Seaweed death → spawn replacement seaweed (maintains population)**
- ✅ **Shark death → cleanup teeth + spawn random large creature**

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

### Entity Death Callbacks - COMPLETED
- ✅ **Fish death callback implemented** (spawns replacement fish)
- ✅ **Seaweed death callback implemented** (spawns replacement seaweed)  
- ✅ **Large creature death callbacks implemented** (spawn random large creature)
- ✅ **Shark special death callback implemented** (cleanup teeth + spawn random)

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

## ✅ Phase 1: COMPLETED - Code Simplification Tasks  

### Manager System Simplification - COMPLETED
- ✅ **Replaced WhaleManager with simple add_whale() function**
- ✅ **Replaced ShipManager with simple add_ship() function**
- ✅ **Replaced SeaMonsterManager with simple add_sea_monster() function**
- ✅ **Replaced SharkManager with simple add_shark() function**
- ✅ **Removed SeaweedManager entirely (death callbacks handle population)**
- ✅ **Removed CastleManager, WaterSurfaceManager (simple initialization)**

### App Structure Simplification - COMPLETED
- ✅ **Simplified tick() method** (only updates entities, death callbacks handle spawning)
- ✅ **Using simple spawning functions** (matching original Perl functions)
- ✅ **Death callbacks implemented as simple function calls**
- ✅ **Removed complex manager orchestration**
- ✅ **Reduced App struct from 15 fields to 5 fields**

### Code Reduction Achievement
- ✅ **Reduced from ~6,000 lines to ~4,000 lines** (33% reduction)
- ✅ **Removed 7 complex manager classes**
- ✅ **Simplified architecture to match original Perl design**

### Entity System (Keeping Current Structure)
- ⚠️ **Entity trait kept current** (works well, not over-complex)
- ⚠️ **Position, Velocity structs kept** (provide good type safety)
- ⚠️ **Essential fields kept** (created_at fields marked for potential removal)

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

### 🔧 **Needs Modification (Simplify and Fix)**
- **Complex manager system** → Replace with simple functions like original
- **Over-engineered entity trait** → Simplify to 4-5 essential methods
- **Multiple simultaneous large creatures** → Add single creature constraint
- **Timer-based spawning** → Replace with death callback system
- **Over-complex data structures** → Flatten to essential fields only
- **Excessive error handling** → Simplify to direct calls

### 🎯 **Features to Add**
- **Death callback system** (core authenticity)
- **Single large creature constraint** (original behavior)
- **Big fish species** (complete random object system)
- **Bubble system** (fish bubble generation and physics)
- **Classic mode** (-c command line flag)

## 📋 Implementation Priority

### ✅ Priority 1: COMPLETED - Code Simplification & Behavioral Authenticity
1. ✅ **Removed complex manager system** → Simple functions like original
2. ✅ **Death callback system implemented** → Simple function calls like original  
3. ✅ **Single large creature constraint** → One whale/ship/monster at a time
4. ✅ **Original population formulas** → (height-9)*width/350 fish, width/15 seaweed
5. ✅ **Authentic spawn patterns** → Init once + death callbacks only
6. ✅ **Code reduction achieved** → 6,000 → 4,000 lines (more reduction possible)

### 🎯 Priority 2: Complete Missing Features (Current Focus)

### Priority 3: Complete Feature Set
1. **Big fish species** → Add to simplified random object system
2. **Bubble system** → Simple fish bubble physics
3. **Classic mode** → -c flag support

### Priority 4: Final Polish
1. **Performance optimization** (should be better after simplification)
2. **Documentation** (update for simplified architecture)
3. **Testing** (focused on behavior, not implementation details)
1. **Performance optimization** (after simplification)
2. **Documentation updates** (reflect simplified architecture)
3. **Final testing and validation**

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
- ✅ **Simplification completed**: Removed complex managers and over-engineering
- ✅ **Original-inspired architecture**: Simple functions + death callbacks like Perl version
- ✅ **Genuine improvements kept**: Screen adaptation, entity management  
- ✅ **Behavior validated**: Single large creature + death callbacks working

### Simplification Metrics - PHASE 1 RESULTS
- **Previous**: ~6,000 lines across 16 files  
- **Current**: ~4,000 lines (33% reduction achieved)
- **Potential Target**: ~3,000 lines (further reduction possible)
- ✅ **Removed**: 7 complex manager classes, complex spawning logic
- ✅ **Kept**: Entity files, death callbacks, genuine improvements, core functionality