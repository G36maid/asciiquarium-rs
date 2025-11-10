# Random Objects Verification Report

This document verifies that all random objects (large creatures) match the original Perl implementation.

## Summary

All five random objects have been implemented and checked against the original `asciiquarium.pl`:

1. ✅ **Ship** - Correct
2. ✅ **Whale** - Correct
3. ⚠️ **Sea Monster** - Issues found
4. ⚠️ **Big Fish** - Issues found
5. ⚠️ **Shark** - Issues found

## Detailed Analysis

### 1. Ship (`add_ship`)

**Status:** ✅ **CORRECT**

**Original Perl Behavior:**
- Speed: 1
- Depth: 7 (water_gap1)
- Y position: 0 (surface level)
- X position: -24 (right) or width-2 (left)
- Death callback: `random_object`

**Rust Implementation:**
- ✅ Speed: 1.0
- ✅ Depth: 7
- ✅ Y position: 0.0
- ✅ X position: -24.0 (right) or width-2 (left)
- ✅ Death callback: `random_object`
- ✅ Sprites match original (both directions)

**Location:** `src/entities/ship.rs`

---

### 2. Whale (`add_whale`)

**Status:** ✅ **CORRECT**

**Original Perl Behavior:**
- Speed: 1
- Depth: 5 (water_gap2)
- Y position: 0 (surface level)
- X position: -18 (right) or width-2 (left)
- Death callback: `random_object`
- Animation: 5 frames without spout + 7 frames with spout
- Spout alignment: 11 (right) or 1 (left)

**Rust Implementation:**
- ✅ Speed: 1.0
- ✅ Depth: 5
- ✅ Y position: 0.0
- ✅ X position: -18.0 (right) or width-2 (left)
- ✅ Death callback: `random_object`
- ✅ Animation frames: 12 total (5 + 7)
- ✅ Spout alignment: 11 (right) or 1 (left)
- ✅ Sprites match original

**Location:** `src/entities/whale.rs`

---

### 3. Sea Monster (`add_monster`)

**Status:** ⚠️ **ISSUES FOUND**

**Original Perl Behavior:**
- Speed: 2
- Depth: 5 (water_gap2)
- Y position: 2
- X position: -54 (right) or width-2 (left)
- Death callback: `random_object`
- Classic mode: Uses old monster sprites (4 frames)
- Modern mode: Uses new monster sprites (2 frames)
- Animation speed: 0.25 (callback_args)

**Rust Implementation:**
- ✅ Speed: 2.0
- ✅ Depth: 5
- ✅ Y position: 2.0
- ✅ X position: -54.0 (right) or width-2 (left)
- ✅ Death callback: `random_object`
- ❌ **ISSUE**: Only implements new/modern monster sprites
- ❌ **ISSUE**: Classic mode monster sprites missing
- ✅ Animation frames: 2 (correct for modern)
- ⚠️ Animation timing: 250ms (should verify if this matches 0.25 callback_args)

**Required Changes:**
1. Add old monster sprite variants (4 animation frames)
2. Check `classic_mode` flag in `SeaMonster::new()`
3. Use old sprites when `classic_mode = true`

**Location:** `src/entities/sea_monster.rs`

---

### 4. Big Fish (`add_big_fish`)

**Status:** ⚠️ **ISSUES FOUND**

**Original Perl Behavior:**
- Variant 1: Speed 3, height 14 lines, x=-34 (right) or width-1 (left)
- Variant 2: Speed 2.5, height 13 lines, x=-33 (right) or width-1 (left)
- Depth: 2 (shark depth)
- Y position: random(9 to height-15) for variant1, random(9 to height-14) for variant2
- Death callback: `random_object`
- Classic mode: Always variant 1
- Modern mode: 2/3 variant 2, 1/3 variant 1 (using `int(rand(3)) > 1`)

**Rust Implementation:**
- ✅ Speed: 3.0 (variant1), 2.5 (variant2)
- ❌ **ISSUE**: Depth is FISH_START (3), should be 2 (SHARK depth)
- ⚠️ **ISSUE**: Y position calculation doesn't differentiate between variants
- ❌ **ISSUE**: X position uses `1 - sprite_width` instead of -34/-33
- ❌ **ISSUE**: Left spawn uses `width - 2` instead of `width - 1`
- ✅ Death callback: `random_object`
- ⚠️ Variant selection: Uses `gen_range(0..3) > 0` which gives 2/3 for variant2, should be `> 1`

**Required Changes:**
1. Change depth from `FISH_START` (3) to `2` (shark depth)
2. Fix X position: Use -34 (variant1) or -33 (variant2) for right-moving
3. Fix left spawn: Use `width - 1` instead of `width - 2`
4. Fix Y position: Use height-15 for variant1, height-14 for variant2
5. Fix variant selection: Use `gen_range(0..3) > 1` instead of `> 0` (currently correct logic but wrong comparison)

**Location:** `src/entities/big_fish.rs`

---

### 5. Shark (`add_shark`)

**Status:** ⚠️ **ISSUES FOUND**

**Original Perl Behavior:**
- Speed: 2
- Depth: 2 (shark depth)
- Y position: random(9 to height-19) → `int(rand(height - (10 + 9))) + 9`
- X position: -53 (right) or width-2 (left)
- Death callback: `shark_death` (special - removes teeth then calls `random_object`)
- Teeth: Separate invisible entity for collision detection
- Teeth position: shark_x + 44 (right) or shark_x + 9 (left), shark_y + 7
- Teeth depth: shark_depth + 1 (for fish collision: depth range fish_end - fish_start)

**Rust Implementation:**
- ✅ Speed: 2.0
- ✅ Depth: 2 (uses `crate::depth::SHARK`)
- ✅ Y position: `gen_range(9..height-19)`
- ✅ X position: -53.0 (right) or width-2 (left)
- ✅ Death callback: `shark_death`
- ✅ Teeth implementation exists
- ✅ Teeth position offsets: (44, 7) right, (9, 7) left
- ✅ Teeth depth: shark_depth + 1

**Note:** The Y position formula matches: Perl uses `int(rand($anim->height() - (10 + 9))) + 9` which is equivalent to `rand(height - 19) + 9`, same as Rust `gen_range(9..height-19)`.

**Location:** `src/entities/shark.rs`

---

## Spawning System Verification

### Original Perl `random_object` Array

```perl
sub init_random_objects {
    return (
        \&add_ship,
        \&add_whale,
        \&add_monster,
        \&add_big_fish,
        \&add_shark,
    );
}

sub random_object {
    my ($dead_object, $anim) = @_;
    my $sub = int(rand(scalar(@random_objects)));
    $random_objects[$sub]->($dead_object, $anim);
}
```

### Rust Implementation

✅ Correct implementation in `src/spawning.rs`:

```rust
pub fn random_object(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return;
    }

    let mut rng = rand::thread_rng();
    let spawners: &[fn(&mut EntityManager, Rect)] = &[
        add_ship,
        add_whale,
        add_sea_monster,
        add_big_fish,
        add_shark,
    ];

    let index = rng.gen_range(0..spawners.len());
    spawners[index](entity_manager, screen_bounds);
}
```

---

## Critical Issues Summary

### High Priority (Correctness)

1. **Big Fish Depth** - Wrong depth value (3 instead of 2)
2. **Big Fish Spawn Position** - Wrong X coordinates
3. **Sea Monster Classic Mode** - Missing old/classic monster sprites

### Medium Priority (Precision)

4. **Big Fish Y Position** - Should differentiate between variant heights
5. **Big Fish Variant Selection** - Logic is functionally correct but comparison differs from original

### Low Priority (Already Correct but Worth Noting)

- All creatures use asymmetric spawning correctly
- All death callbacks are correct
- All speeds match original values
- Shark implementation is complete and correct

---

## Recommended Action Items

1. Fix Big Fish depth from 3 to 2
2. Fix Big Fish spawn X positions (-34/-33 for right, width-1 for left)
3. Fix Big Fish Y position to use variant-specific height constraints
4. Implement classic monster sprites for Sea Monster
5. Add classic_mode parameter to SeaMonster::new()
6. Update tests to verify all fixes

---

## Testing Checklist

- [ ] Ship spawns correctly at surface
- [ ] Whale animates with water spout
- [ ] Sea Monster uses correct sprites in classic vs modern mode
- [ ] Big Fish spawns at correct depth (2, not 3)
- [ ] Big Fish spawns at correct X positions
- [ ] Big Fish variant heights match original
- [ ] Shark teeth collision works correctly
- [ ] All creatures die offscreen and spawn replacements
- [ ] Only one large creature exists at a time
- [ ] Random selection gives equal probability to all 5 types