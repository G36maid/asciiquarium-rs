# Big Fish Implementation

This document describes the implementation of the Big Fish entity, completing one of the core missing features from the original Perl asciiquarium.

## Overview

The Big Fish is a large predatory fish that appears as one of the random large creatures in the aquarium. Unlike the placeholder implementation that used regular fish, this is now a proper entity with two distinct variants matching the original Perl implementation.

## Variants

### Variant 1 (Traditional)

- **Sprite Size**: 14 lines tall, ~34 characters wide
- **Speed**: 3.0 units/second
- **Appearance**: Traditional large fish shape with prominent eye and fins
- **Availability**: Both classic mode and modern mode
- **Original Code**: `add_big_fish_1()` from Perl

**ASCII Art Sample (Right-facing)**:
```
 ______
`""-.  `````-----.....__
     `.  .      .       `-.
       :     .     .       `.
 ,     :   .    .          _ :
: `.   :                  (@) `._
 `. `..'     .     =`-.       .__)
   ;     .        =  ~  :     .-"
 .' .'`.   .    .  =.-'  `._ .'
: .'   :               .   .'
 '   .'  .    .     .   .-'
   .'____....----''.'=.'
   ""             .'.'
               ''"'`
```

### Variant 2 (Stylized)

- **Sprite Size**: 13 lines tall, ~33 characters wide
- **Speed**: 2.5 units/second
- **Appearance**: More stylized with angular features and decorative elements
- **Availability**: Modern mode only
- **Original Code**: `add_big_fish_2()` from Perl

**ASCII Art Sample (Right-facing)**:
```
                _ _ _
             .='\\ \\ \\`"=,
           .'\\ \\ \\ \\ \\ \\ \\
\\'=._     / \\ \\ \\_\\_\\_\\_\\_\\
\\'=._'.  /\\ \\,-"`- _ - _ - '-.
  \\`=._\\|'.\\/- _ - _ - _ - _- \\
  ;"= ._\\=./_ -_ -_ \{`"=_    @ \\
   ;="_-_=- _ -  _ - \{"=_"-     \\
   ;_=_--_.,          \{_.='   .-/
  ;.="` / ';\\        _.     _.-`
  /_.='/ \\/ /;._ _ _\{.-;`/"`
/._=_.'   '/ / / / /\{.= /
/.='       `'./_/_.=`\{_/
```

## Mode-Based Selection

### Classic Mode (`classic_mode = true`)
- Only spawns **Variant 1**
- Matches original asciiquarium behavior before newer fish were added
- Ensures compatibility with `-c` flag when implemented

### Modern Mode (`classic_mode = false`)
- **2/3 chance** (66.67%) for Variant 2
- **1/3 chance** (33.33%) for Variant 1
- Matches original Perl logic: `int(rand(3)) > 1`

## Technical Implementation

### File Structure
```
src/entities/big_fish.rs    # Complete implementation
```

### Key Features

1. **Asymmetric Spawning**
   - Right-moving: spawns fully off-screen left at `x = 1 - sprite_width`
   - Left-moving: spawns mostly visible at right edge at `x = screen_width - 2`
   - Matches behavior of other large creatures

2. **Random Vertical Positioning**
   - Spawns between `y = 9` and `y = height - 15`
   - Stays in the water (below surface, above bottom)

3. **Color Mask System**
   - Uses numbered masks (1, 2) for random color assignment
   - `W` for white eye
   - Matches original Perl color randomization with `rand_color()`

4. **Large Creature Management**
   - Tracked by `EntityManager` as a large creature
   - Only one large creature can exist at a time
   - Death callback triggers `random_object()` to spawn replacement

5. **Proper Entity Trait Implementation**
   - Implements all required `Entity` trait methods
   - `entity_type()` returns "big_fish_1" or "big_fish_2"
   - Death when completely off-screen
   - Uses `FISH_START` depth for proper layering

## Spawning Integration

The Big Fish is integrated into the `random_object()` spawner in `src/spawning.rs`:

```rust
let spawners: &[fn(&mut EntityManager, Rect)] = &[
    add_ship,
    add_whale,
    add_sea_monster,
    add_big_fish,  // ← Now properly implemented
    add_shark,
];
```

## Testing

Comprehensive test suite with 7 tests:

1. **test_big_fish_creation** - Basic entity creation
2. **test_big_fish_variants** - Verify both variant types and sizes
3. **test_big_fish_classic_mode** - Ensures only Variant 1 in classic mode
4. **test_big_fish_spawn_positions** - Asymmetric spawning verification
5. **test_big_fish_movement** - Movement based on direction
6. **test_big_fish_death_callback** - Death callback exists
7. **test_big_fish_speeds** - Correct speeds for each variant

All tests pass: ✅

## Comparison with Original Perl

| Feature | Perl Implementation | Rust Implementation | Match |
|---------|-------------------|---------------------|-------|
| Two variants | ✅ `big_fish_1`, `big_fish_2` | ✅ `Variant1`, `Variant2` | ✅ |
| Classic mode | ✅ Only variant 1 | ✅ Only `Variant1` | ✅ |
| Selection ratio | ✅ 1/3 vs 2/3 | ✅ 1/3 vs 2/3 | ✅ |
| Sprite size | ✅ 14 and 13 lines | ✅ 14 and 13 lines | ✅ |
| Speed | ✅ 3.0 and 2.5 | ✅ 3.0 and 2.5 | ✅ |
| ASCII art | ✅ Original sprites | ✅ Exact copy | ✅ |
| Color masks | ✅ Random colors | ✅ Random colors | ✅ |
| Large creature | ✅ Yes | ✅ Yes | ✅ |
| Death callback | ✅ `random_object` | ✅ `random_object` | ✅ |

## Impact

- **Removes placeholder**: No longer uses regular fish as substitute
- **Completes large creature set**: All 5 large creatures now properly implemented
- **Improves fidelity**: Matches original Perl behavior exactly
- **Maintains compatibility**: Classic mode ready for CLI flag implementation
- **Test coverage**: +7 tests (59 → 66 total)
- **Code quality**: 0 clippy warnings

## Future Work

The Big Fish implementation is complete and ready. Next steps in the project:

1. Implement CLI argument parsing for `-c` (classic mode) flag
2. Wire up `classic_mode` to be controlled by command-line argument
3. Add `-h` (help) and `-v` (version) flags

## References

- Original Perl code: `sub add_big_fish`, `sub add_big_fish_1`, `sub add_big_fish_2`
- Sprite source: `asciiquarium.pl` lines 1260-1360 (approximate)
- Implementation: `src/entities/big_fish.rs`
- Tests: 7 unit tests in same file
- Integration: `src/spawning.rs::add_big_fish()`
