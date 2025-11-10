# Development Guide

This document provides technical details for developers working on asciiquarium-rs.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Fish Species System](#fish-species-system)
- [Fish Spawning Behavior](#fish-spawning-behavior)
- [Code Quality Standards](#code-quality-standards)
- [Testing Guidelines](#testing-guidelines)
- [Recent Improvements](#recent-improvements)

## Architecture Overview

See [SPEC.md](SPEC.md) for complete architecture documentation.

**Key Components:**
- **Entity System**: Trait-based polymorphism for all aquarium entities
- **Death Callbacks**: Automatic population management and respawning
- **Depth Layers**: Z-ordering system (higher depth = background, lower = foreground)
- **Event System**: Non-blocking input handling with dedicated thread

## Fish Species System

### Overview

The fish system implements all 12 fish species from the original Perl code with proper categorization and selection logic.

**Total Species: 12**
- 4 "new" fish (added in v1.1)
- 8 "old" fish (classic species)

### Selection Logic

Fish selection follows the original Perl algorithm:

```rust
if rng.gen_range(0..12) > 8 {
    // New fish (9, 10, 11 = 3 out of 12 = 25%)
    spawn_new_fish()
} else {
    // Old fish (0-8 = 9 out of 12 = 75%)
    spawn_old_fish()
}
```

This gives a **25% chance for new fish, 75% for old fish**.

### Classic Mode

When `classic_mode = true` (triggered by `-c` flag):
- Only old fish spawn (no new fish)
- Matches original asciiquarium behavior

**Implementation:**
```rust
let fish = Fish::new_random(id, screen_bounds, classic_mode);
```

### Species List

**New Fish (4 species):**
- `NewSmall1` - Small angled fish with fins
- `NewSmall2` - Fancy double-bracket fish  
- `NewMedium1` - Large fancy fish with question marks
- `NewMedium2` - Bulgy fish with comma decorations

**Old Fish (8 species):**
- `OldFancy` - Fancy fish with scales
- `OldSimple` - Simple angled fish (>=  (o>)
- `OldWavy` - Dotted wavy fish (:::::::)
- `OldTiny` - Tiny simple fish (><_'>)
- `OldCommaLarge` - Comma fish with quotes
- `OldAngledFin` - Small angled fish (same art as NewSmall1)
- `OldCommaSmall` - Smaller comma fish
- `OldRounded` - Rounded fish (\/ o\)

See [FISH_SPECIES.md](FISH_SPECIES.md) for complete details and ASCII art.

### Color Masks

Each fish has a color mask defining which parts get which colors:

- `1` = body color
- `2` = dorsal fin
- `3` = flippers/tail
- `4` = eye (white)
- `5` = mouth
- `6` = tailfin
- `7` = gills

Colors are randomly selected from the palette: cyan, red, yellow, blue, green, magenta (normal and bright variants).

## Fish Spawning Behavior

### Asymmetric Spawning

Fish spawning is **intentionally asymmetric** to match the original:

**Right-moving fish (→):**
- Start at `X = 1 - sprite_width` (completely off-screen left)
- Swim onto screen from left edge
- Die when `X > screen_width` (off right edge)

**Left-moving fish (←):**
- Start at `X = screen_width - 2` (mostly visible at right edge)
- Already visible when spawned
- Die when `X + sprite_width < 0` (off left edge)

### Visual Example

```
Right-moving: [OFF SCREEN] →→→ [APPEARS] →→→ [VISIBLE] →→→ [DISAPPEARS]
              X = -5              X = 0           X = 40         X = 81

Left-moving:  [APPEARS] ←←← [VISIBLE] ←←← [DISAPPEARS] ←←← [OFF SCREEN]
              X = 78         X = 40         X = 0             X = -5
```

### Why Asymmetric?

This matches the original Perl code exactly:

```perl
if($fish_num % 2) {
    $fish_object->{'X'} = $anim->width()-2;  # Left-moving
} else {
    $fish_object->{'X'} = 1 - $fish_object->{'WIDTH'};  # Right-moving
}
```

## Code Quality Standards

### Zero Clippy Warnings

The codebase maintains **zero clippy warnings**. All code must pass:

```bash
cargo clippy --all-targets --all-features
```

**Common patterns to follow:**
- Use struct initialization instead of field assignment after `Default::default()`
- Use `if let` for single pattern matching instead of `match`
- Use `RangeInclusive::contains()` instead of manual `>=` and `<=`
- Use `or_default()` instead of `or_insert_with(Vec::new)`
- Implement `Default` for types with parameter-less `new()`

### Code Formatting

All code must be formatted with rustfmt:

```bash
cargo fmt
```

### Module Structure

- No nested modules with same name as parent (e.g., no `depth::depth`)
- Use `pub use` to re-export commonly used types
- Group related entities in the `entities` module

## Testing Guidelines

### Test Coverage

Current: **59 tests, all passing**

Every entity should have tests for:
- Creation and initialization
- Movement and updates
- Sprite generation
- Death conditions
- Special behaviors (animation, spawning, etc.)

### Running Tests

```bash
# All tests
cargo test --all-features --all-targets

# Single test
cargo test test_fish_creation

# With output
cargo test -- --nocapture
```

### Test Categories

1. **Unit Tests**: In each module (entity behavior)
2. **Integration Tests**: Full system behavior (TODO)
3. **Distribution Tests**: Statistical validation (e.g., fish selection ratio)

### Writing Tests

```rust
#[test]
fn test_entity_behavior() {
    let screen_bounds = Rect::new(0, 0, 80, 24);
    let entity = MyEntity::new_random(1, screen_bounds);
    
    assert!(entity.is_alive());
    assert_eq!(entity.entity_type(), "my_entity");
    // ... more assertions
}
```

## Recent Improvements

### Issues Fixed (All 5 Critical Issues Complete)

1. ✅ **Water Surface**: Now static (no animation)
2. ✅ **Fish Spawn/Despawn**: Fully swim on/off screen
3. ✅ **Fish Species**: All 12 species with 25%/75% selection
4. ✅ **Bubbles**: Generated by fish and rise to surface
5. ✅ **Screen Resize**: Auto-reinitialize on terminal size change

### Code Quality

- ✅ **Zero clippy warnings** (was 6, now 0)
- ✅ **All tests passing** (59 tests)
- ✅ **Clean module structure** (removed depth::depth inception)
- ✅ **Idiomatic Rust** patterns throughout

### Test Coverage Improvements

- Added distribution test for fish selection (validates 25%/75% ratio)
- Added classic mode test (ensures only old fish in classic mode)
- Added sprite validation for all 12 fish species
- Added category assignment tests

## Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run (don't use for testing - visual only)
cargo run --release

# Run tests
cargo test --all-features --all-targets

# Check for issues
cargo clippy

# Format code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

## Next Steps

See [TODO.md](TODO.md) for complete task list.

**High Priority:**
1. Refactor code to reduce duplication
2. Implement proper big fish sprites (currently placeholder)
3. Add command-line argument parsing (`-c`, `-h`, `-v`)
4. Write integration tests

**Medium Priority:**
- Additional fish species
- More decorations (rocks, coral, etc.)
- Configuration file support
- Performance profiling

## Debugging Tips

### Entity Not Appearing

1. Check depth value (higher = background)
2. Verify position is on-screen
3. Check sprite is not all transparent chars
4. Ensure entity is added to EntityManager

### Fish Popping In/Out

- Verify spawn positions match asymmetric behavior
- Check death conditions (should be fully off-screen)
- Test with different screen sizes

### Colors Wrong

- Verify color mask matches sprite dimensions
- Check random color generation
- Ensure color codes (1-7) are correct

### Performance Issues

- Profile with `cargo flamegraph`
- Check entity count with different screen sizes
- Monitor update loop timing

## References

- [SPEC.md](SPEC.md) - Complete architecture specification
- [FISH_SPECIES.md](FISH_SPECIES.md) - All fish species with ASCII art
- [TODO.md](TODO.md) - Task tracking and priorities
- [CHANGELOG.md](CHANGELOG.md) - Version history and changes
- [AGENTS.md](AGENTS.md) - Agent coding guidelines
- Original Perl: `asciiquarium.pl`

## Contributing

When contributing:

1. Follow the coding standards (clippy, rustfmt)
2. Add tests for new features
3. Update documentation (SPEC.md, TODO.md, CHANGELOG.md)
4. Verify original Perl behavior when implementing features
5. Keep commits focused and well-described

## License

GPL v2 - See [LICENSE](LICENSE) file