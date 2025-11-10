# Fish Species Implementation

This document explains the fish species implementation in asciiquarium-rs and how it matches the original Perl code.

## Overview

The original asciiquarium (version 1.1) introduced a distinction between "new" fish (added in v1.1) and "old" fish (classic species from earlier versions). The selection logic gives a 25% chance for new fish and 75% chance for old fish, with a `-c` (classic) flag to disable new fish entirely.

## Fish Categories

### New Fish (4 species)

These fish were added in asciiquarium 1.1 and can be disabled with the `-c` classic mode flag:

1. **NewSmall1** - Small angled fish with fins and underscore body
   ```
      \
     / \
   >=_('>
     \_/
      /
   ```

2. **NewSmall2** - Fancy double-bracket fish with decorative marks
   ```
        ,
        \}\
   \  .'  `\
   \}\<   ( 6>
   /  `,  .'
        \}/
        '
   ```

3. **NewMedium1** - Large fancy fish with question marks (representing scales)
   ```
               \\'`.
                )  \
   (`.??????_.-`' ' '`-.
    \ `.??.`        (o) \_
     >  ><     (((       (
    / .`??`._      /_|  /'
   (.`???????`-. _  _.-`
               /__/'
   ```

4. **NewMedium2** - Bulgy fish with comma decorations
   ```
          ,--,_
   __    _\.---'-.
   \ '.-"     // o\
   /_.'-._    \\  /
          `"--(/"`
   ```

### Old Fish (8 species)

These are the classic fish from the original asciiquarium:

1. **OldFancy** - Fancy fish with scales and apostrophes
   ```
          \
        ...\..,
   \  /'       \
    >=     (  ' >
   /  \      / /
       `"'"'/'
   ```

2. **OldSimple** - Simple angled fish with straight body
   ```
       \
   \ /--\
   >=  (o>
   / \__/
       /
   ```

3. **OldWavy** - Dotted wavy fish with colons (representing scales)
   ```
          \:.
   \;,   ,;\\\\\,,
     \\\\\;;:::::::o
     ///;;::::::::<
    /;` ``/////``
   ```

4. **OldTiny** - Tiny simple fish
   ```
     __
   ><_'>
      '
   ```

5. **OldCommaLarge** - Small comma fish with quotes
   ```
      ..\,
   >='   ('>
     '''/''
   ```

6. **OldAngledFin** - Small angled fish (same art as NewSmall1, appears in both arrays)
   ```
      \
     / \
   >=_('>
     \_/
      /
   ```

7. **OldCommaSmall** - Even smaller comma fish
   ```
     ,\
   >=('>
     '/
   ```

8. **OldRounded** - Rounded small fish with diagonal body
   ```
     __
   \/ o\
   /\__/
   ```

## Selection Logic

The original Perl code uses this selection logic:

```perl
if ($new_fish) {
    if (int(rand(12)) > 8) {
        add_new_fish(@parm);
    } else {
        add_old_fish(@parm);
    }
}
```

This means:
- `int(rand(12))` generates integers 0-11
- Values 9, 10, 11 (3 out of 12 = 25%) trigger new fish
- Values 0-8 (9 out of 12 = 75%) trigger old fish

Our Rust implementation matches this exactly:

```rust
pub fn random(classic_mode: bool) -> Self {
    let mut rng = rand::thread_rng();
    
    if classic_mode {
        // Classic mode: only old fish
        let old = Self::old_species();
        old[rng.gen_range(0..old.len())]
    } else {
        // Modern mode: 25% new, 75% old
        if rng.gen_range(0..12) > 8 {
            // New fish (9, 10, 11 = 3 out of 12 = 25%)
            let new = Self::new_species();
            new[rng.gen_range(0..new.len())]
        } else {
            // Old fish (0-8 = 9 out of 12 = 75%)
            let old = Self::old_species();
            old[rng.gen_range(0..old.len())]
        }
    }
}
```

## Color Masks

Each fish has a color mask that defines which parts get which colors:

- `1` = body color (randomly selected from color palette)
- `2` = dorsal fin color
- `3` = flippers/tail color
- `4` = eye color (white)
- `5` = mouth color
- `6` = tailfin color
- `7` = gills color

The color palette includes: cyan, red, yellow, blue, green, magenta (both normal and bright variants).

## Classic Mode

The `classic_mode` flag (corresponding to the `-c` command-line option in the original) disables new fish entirely:

- When `classic_mode = true`: Only old fish (8 species) are spawned
- When `classic_mode = false`: 25% new fish (4 species), 75% old fish (8 species)

This is implemented at multiple levels:
- `FishSpecies::random(classic_mode)` - Selection logic
- `EntityManager::classic_mode()` - Stores the flag
- `App::classic_mode` - Application-level setting

## Implementation Details

### Fish Enum

```rust
pub enum FishSpecies {
    // NEW FISH (4 species)
    NewSmall1,
    NewSmall2,
    NewMedium1,
    NewMedium2,
    
    // OLD FISH (8 species)
    OldFancy,
    OldSimple,
    OldWavy,
    OldTiny,
    OldCommaLarge,
    OldAngledFin,
    OldCommaSmall,
    OldRounded,
}
```

### Fish Category

```rust
pub enum FishCategory {
    New,
    Old,
}
```

Each species knows its category via `species.category()`.

## Testing

The implementation includes comprehensive tests:

1. **Species Count Test**: Verifies 4 new + 8 old = 12 total species
2. **Category Test**: Ensures each species has correct category
3. **Classic Mode Test**: Confirms only old fish spawn in classic mode
4. **Distribution Test**: Validates 25%/75% ratio over 1000 samples
5. **Sprite Test**: Verifies all 12 species have valid left/right sprites

## Verification Against Original

All fish sprites have been verified character-by-character against the original `asciiquarium.pl` code:
- Character positions match exactly
- Color masks match exactly
- Direction sprites (left/right) match exactly
- Special characters (backslashes, quotes, etc.) are properly escaped

## Notes

- The fish `OldAngledFin` uses the same ASCII art as `NewSmall1`. This matches the original Perl code where this fish appears in both the new and old fish arrays.
- Each fish pair (right-facing and left-facing) is carefully crafted to mirror each other while maintaining the visual appearance of the same fish facing different directions.
- Color randomization ensures variety - even the same species will appear in different colors each time it spawns.

## Future Work

- Command-line argument parsing to enable `-c` classic mode from command line
- Additional fish species can be added by extending the enum and implementing `get_sprites()`
- Big fish implementation (currently uses placeholder)