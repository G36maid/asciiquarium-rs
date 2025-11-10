# Fixes Needed for Random Objects

This document lists the specific code changes required to match the original Perl implementation exactly.

## 1. Big Fish - Multiple Issues

### Issue 1a: Wrong Depth Value
**File:** `src/entities/big_fish.rs`
**Line:** ~95 (in `new_variant` method)

**Current:**
```rust
position: Position::new(x as f32, y as f32, FISH_START),
```

**Should be:**
```rust
position: Position::new(x as f32, y as f32, crate::depth::SHARK),
```

**Reason:** Original Perl uses `$depth{'shark'}` (value 2), not fish depth (3).

---

### Issue 1b: Wrong X Position Calculation
**File:** `src/entities/big_fish.rs`
**Lines:** ~70-75 (in `new_variant` method)

**Current:**
```rust
let sprite_width = sprite.get_bounding_box().0 as i32;

let x = match direction {
    Direction::Right => 1 - sprite_width, // Start fully off-left
    Direction::Left => screen_bounds.width as i32 - 2, // Start mostly visible at right
};
```

**Should be:**
```rust
// Match original Perl: x = -34 (variant1) or -33 (variant2) for right-moving
// x = width - 1 for left-moving
let x = match direction {
    Direction::Right => {
        match variant {
            BigFishVariant::Variant1 => -34,
            BigFishVariant::Variant2 => -33,
        }
    }
    Direction::Left => screen_bounds.width as i32 - 1,
};
```

**Reason:** Original Perl has hardcoded values, not computed from sprite width.

---

### Issue 1c: Y Position Should Vary by Variant
**File:** `src/entities/big_fish.rs`
**Lines:** ~78-80 (in `new_variant` method)

**Current:**
```rust
let max_height = 9;
let min_height = screen_bounds.height.saturating_sub(15).max(max_height + 1);
let y = rng.gen_range(max_height..min_height) as i32;
```

**Should be:**
```rust
// Original: rand(height - 15) + 9 for variant1, rand(height - 14) + 9 for variant2
let max_height = 9;
let height_offset = match variant {
    BigFishVariant::Variant1 => 15, // Original: height - 15
    BigFishVariant::Variant2 => 14, // Original: height - 14
};
let min_height = screen_bounds.height.saturating_sub(height_offset).max(max_height + 1);
let y = rng.gen_range(max_height..min_height) as i32;
```

**Reason:** The two variants have different sprite heights and use different height constraints.

---

### Issue 1d: Variant Selection Logic (Minor)
**File:** `src/entities/big_fish.rs`
**Lines:** ~48-53 (in `new` method)

**Current:**
```rust
if rng.gen_range(0..3) > 0 {
    BigFishVariant::Variant2
} else {
    BigFishVariant::Variant1
}
```

**Should be:**
```rust
// Original Perl: int(rand(3)) > 1
// This gives: 0 -> variant1, 1 -> variant1, 2 -> variant2
if rng.gen_range(0..3) > 1 {
    BigFishVariant::Variant2
} else {
    BigFishVariant::Variant1
}
```

**Reason:** Match exact Perl logic. Current gives 2/3 vs 1/3 (functionally correct), but comparison should be `> 1` not `> 0`.

---

## 2. Sea Monster - Missing Classic Mode Support

### Issue 2a: Only Modern Sprites Implemented
**File:** `src/entities/sea_monster.rs`
**Current:** Only has 2-frame "new monster" sprites

**Needed:** Implement 4-frame "old monster" sprites from original Perl `add_old_monster` function.

---

### Issue 2b: No Classic Mode Check
**File:** `src/entities/sea_monster.rs`
**Line:** ~26 (in `new` method signature)

**Current:**
```rust
pub fn new(id: EntityId, screen_bounds: Rect) -> Self {
```

**Should be:**
```rust
pub fn new(id: EntityId, screen_bounds: Rect, classic_mode: bool) -> Self {
```

And then use `classic_mode` to select between old/new sprites:
```rust
let sprites = if classic_mode {
    Self::create_old_monster_sprites(&direction)
} else {
    Self::create_new_monster_sprites(&direction)
};
```

---

### Issue 2c: Update Spawner
**File:** `src/spawning.rs`
**Line:** ~80 (in `add_sea_monster` function)

**Current:**
```rust
let monster = SeaMonster::new(monster_id, screen_bounds);
```

**Should be:**
```rust
let classic_mode = entity_manager.classic_mode();
let monster = SeaMonster::new(monster_id, screen_bounds, classic_mode);
```

---

### Issue 2d: Add Old Monster Sprites
**File:** `src/entities/sea_monster.rs`

**Add new function:**
```rust
fn create_old_monster_sprites(direction: &Direction) -> Vec<Sprite> {
    // Implement 4-frame animation from Perl's add_old_monster
    // See asciiquarium.pl lines 1075-1168
    // This is a large sprite with 4 animation frames
}
```

---

## 3. Update Tests

After fixes, update these test files:
- `src/entities/big_fish.rs` - Update tests to verify correct depth (2), positions, and variant heights
- `src/entities/sea_monster.rs` - Add tests for classic mode
- Update test count in `TODO.md` if adding new tests

---

## Priority Order

1. **HIGH**: Big Fish depth (Issue 1a) - Wrong depth affects rendering order
2. **HIGH**: Big Fish positions (Issue 1b) - Affects spawn locations
3. **MEDIUM**: Sea Monster classic mode (Issues 2a-2d) - Feature completeness
4. **LOW**: Big Fish Y position (Issue 1c) - Minor visual difference
5. **LOW**: Big Fish variant selection (Issue 1d) - Already functionally correct

---

## Verification Checklist

After applying fixes:

- [ ] Big Fish spawns at depth 2 (shark depth), not 3
- [ ] Big Fish variant 1 spawns at x=-34 (right) or width-1 (left)
- [ ] Big Fish variant 2 spawns at x=-33 (right) or width-1 (left)
- [ ] Big Fish variant 1 uses height-15 for Y range
- [ ] Big Fish variant 2 uses height-14 for Y range
- [ ] Sea Monster uses old sprites when classic_mode=true
- [ ] Sea Monster uses new sprites when classic_mode=false
- [ ] All tests pass: `cargo test --all-features --all-targets`
- [ ] Clippy is happy: `cargo clippy`
- [ ] Visual verification with `cargo run` (ask user to test)