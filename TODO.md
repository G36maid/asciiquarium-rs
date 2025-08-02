# Asciiquarium-rs TODO

This document outlines the development roadmap for asciiquarium-rs, a faithful Rust port of the classic ASCII aquarium animation.

## 🎯 Current Status

**Version**: Development (Phase 4 - Large Creatures)
**Core Features**: ✅ Complete and working
**Environment**: ✅ Complete (water, seaweed, castle)
**Fish System**: ⚠️ Behavior partially implemented (missing death callbacks)
**Predators**: ⚠️ Sharks implemented (missing death callback system)
**Large Creatures**: ⚠️ Entities implemented (missing death callback system)

### ✅ Working Features
- **Core Framework**: Entity system, rendering, input handling, screen management
- **Environment**: 4-layer animated water surface, swaying seaweed, castle decoration
- **Fish System**: 6 species with authentic movement, randomized colors, bubble generation
- **Shark System**: Predatory sharks with collision detection and fish hunting
- **Large Creatures**: Whales with water spouts, ships with detailed hulls, animated sea monsters
- **Color System**: Authentic Perl-style color randomization
- **Screen Adaptation**: Dynamic resizing and entity scaling

## 🚧 Active Development

### Phase 4: Critical Behavioral Fixes (Death Callback System)

#### ❌ **URGENT: Core Architecture Mismatch**

Our current implementation uses **timer-based spawning** but the original uses a **death callback chain system**. This is a fundamental architectural difference that must be fixed for authentic behavior.

**Current Problems:**
- Multiple large creatures spawn simultaneously (should be exactly one)
- Independent spawn timers (should be death-callback triggered)
- Continuous runtime spawning (should be initialization + callbacks only)
- Population recalculation (should be death-callback maintenance)

#### 🎯 **Required Death Callback System**

**Original System:**
```
Large Creature Dies → random_object() → New Random Large Creature
Fish Dies → add_fish() → New Fish (maintains population)
Seaweed Dies → add_seaweed() → New Seaweed (maintains population)
Shark Dies → shark_death() → Clean up teeth → random_object()
```

**Priority Tasks:**
- [ ] **Replace timer-based managers with death callbacks**
- [ ] **Implement single large creature constraint** 
- [ ] **Add entity death callback trait**
- [ ] **Fix fish population to use death callbacks**
- [ ] **Fix seaweed 8-12 minute lifespan with death callbacks**
- [ ] **Implement shark special death handler (cleanup teeth)**

#### ✅ Completed: Entity Implementations
- [x] Whale system (entities, sprites, animation)
- [x] Ship system (entities, surface positioning) 
- [x] Sea monster system (tentacle animation)
- [x] Shark system (teeth collision entities)
- [x] Fish system (movement, colors, sprites)
- [x] Seaweed system (sway animation)

#### ❌ **Architecture Fixes Needed**
- [ ] **Death Callback Trait Implementation**
- [ ] **Random Object Manager** (single creature constraint)
- [ ] **Population Maintenance via Callbacks** 
- [ ] **Initialization vs Runtime Separation**
- [ ] **Original Spawn Formulas** (screen_size/350 fish, width/15 seaweed)

## 🎯 Next Up (After Death Callback Fix)

### Phase 5: Big Fish Species
- [ ] **Big Fish Integration into Random Object System**
  - [ ] add_big_fish_1() implementation
  - [ ] add_big_fish_2() implementation  
  - [ ] Random selection between variants
  - [ ] Integration into death callback chain
- [ ] **Original ASCII Art Porting**
  - [ ] Large detailed fish sprites from Perl
  - [ ] Proper color masks and randomization
  - [ ] Yellow coloring (default_color => 'YELLOW')

### Phase 6: Advanced Features  
- [ ] **Bubble Physics System**
  - [ ] Fish bubble generation
  - [ ] 5-frame bubble animation (., o, O, O, O)
  - [ ] Surface collision and popping
- [ ] **Classic Mode Implementation** 
  - [ ] `-c` flag support
  - [ ] Disable new fish and monsters
  - [ ] Original fish species only

## 🛠️ Technical Debt (Post Death Callback Fix)

### Architectural Cleanup
- [ ] **Remove timer-based spawn managers** (after death callbacks work)
- [ ] **Simplify entity managers** (no population tracking needed)
- [ ] **Remove continuous spawning logic** from main loop
- [ ] **Entity lifecycle management** via callbacks only

### Performance & Polish  
- [ ] Memory usage optimization (death callback efficiency)
- [ ] Frame rate stabilization
- [ ] Large screen performance optimization

## 🧹 Code Quality

### Immediate Cleanup
- [ ] **Death callback system tests**
- [ ] **Single large creature constraint tests**
- [ ] **Population maintenance tests**
- [ ] Remove unused `created_at` fields in entities

### Integration Testing
- [ ] **Original behavior validation** 
- [ ] **Screen size population formula verification**
- [ ] **Death callback chain testing**
- [ ] Cross-platform compatibility

## 🔴 Critical Issues

### Behavioral Authenticity (HIGH PRIORITY)
- 🚨 **Death Callback System**: Complete architecture redesign needed
- 🚨 **Multiple Large Creatures**: Should be exactly one at a time
- 🚨 **Timer-based Spawning**: Should be death-callback triggered only
- 🚨 **Population Management**: Should maintain via death callbacks, not recalculation

### Original Behavior Compliance
- 🔧 **Fish Movement**: ✅ Fixed (horizontal only)
- 🔧 **Color System**: ✅ Fixed (authentic randomization)
- 🔧 **Spawn Formulas**: ❌ Need original calculations (screen_size/350, width/15)
- 🔧 **Seaweed Lifespan**: ❌ Need 8-12 minute lifecycle with death callbacks

### Implementation Gaps
- 🔧 **Big Fish Species**: Not yet implemented in random object system
- 🔧 **Shark Death Handler**: Missing teeth cleanup + random object spawn
- 🔧 **Bubble System**: Not yet implemented

## 📋 Implementation Strategy

### Death Callback Implementation Plan
1. **Add DeathCallback trait** to entity system
2. **Create RandomObjectManager** with single creature constraint  
3. **Replace all spawn timers** with death callback registration
4. **Implement original population formulas** for initialization
5. **Add entity lifecycle management** via callback chain

### Architecture Principles  
- **Death Callback Chain**: Core system for entity spawning
- **Initialization vs Runtime**: Clear separation of phases
- **Single Large Creature**: Constraint enforcement
- **Original Formulas**: Exact population calculations
- **Behavioral Authenticity**: Indistinguishable from original

### Success Metrics
- **Single Large Creature**: Never more than one simultaneously
- **Population Stability**: Fish/seaweed counts maintained via callbacks  
- **Authentic Timing**: Match original spawn/death patterns
- **Visual Fidelity**: Identical appearance to Perl version
