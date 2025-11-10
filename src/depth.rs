//! Depth layer constants for proper Z-ordering of entities
//!
//! Based on the original asciiquarium depth system where higher numbers
//! are rendered first (background) and lower numbers last (foreground).

// GUI elements (future use)
pub const GUI_TEXT: u8 = 0;
pub const GUI: u8 = 1;

// Foreground elements
pub const SHARK: u8 = 2;

// Fish layers (multiple layers for schooling effect)
pub const FISH_START: u8 = 3;
pub const FISH_END: u8 = 20;

// Environment background elements
pub const SEAWEED: u8 = 21;
pub const CASTLE: u8 = 22;

// Water surface layers (animated waves)
pub const WATER_LINE3: u8 = 2;
pub const WATER_GAP3: u8 = 3;
pub const WATER_LINE2: u8 = 4;
pub const WATER_GAP2: u8 = 5;
pub const WATER_LINE1: u8 = 6;
pub const WATER_GAP1: u8 = 7;
pub const WATER_LINE0: u8 = 8;
pub const WATER_GAP0: u8 = 9;

/// Get a random fish depth between FISH_START and FISH_END
pub fn random_fish_depth() -> u8 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(FISH_START..=FISH_END)
}

/// Check if a depth value is in the fish layer range
pub fn is_fish_depth(depth: u8) -> bool {
    (FISH_START..=FISH_END).contains(&depth)
}

/// Check if a depth value is in the water surface range
pub fn is_water_surface_depth(depth: u8) -> bool {
    matches!(
        depth,
        WATER_LINE0
            | WATER_GAP0
            | WATER_LINE1
            | WATER_GAP1
            | WATER_LINE2
            | WATER_GAP2
            | WATER_LINE3
            | WATER_GAP3
    )
}

/// Get depth for water line segment by index (0-3)
pub fn water_line_depth(index: u8) -> u8 {
    match index {
        0 => WATER_LINE0,
        1 => WATER_LINE1,
        2 => WATER_LINE2,
        3 => WATER_LINE3,
        _ => WATER_LINE0, // Default fallback
    }
}

/// Get depth for water gap segment by index (0-3)
pub fn water_gap_depth(index: u8) -> u8 {
    match index {
        0 => WATER_GAP0,
        1 => WATER_GAP1,
        2 => WATER_GAP2,
        3 => WATER_GAP3,
        _ => WATER_GAP0, // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_ordering() {
        // GUI should be in front of everything
        assert!(GUI_TEXT < GUI);
        assert!(GUI < SHARK);

        // Sharks should be in front of fish
        assert!(SHARK < FISH_START);

        // Fish should be in front of environment
        assert!(FISH_END < SEAWEED);
        assert!(SEAWEED < CASTLE);

        // Water surface should be mixed with other elements
        assert!(WATER_LINE3 == SHARK);
    }

    #[test]
    fn test_fish_depth_functions() {
        assert!(is_fish_depth(FISH_START));
        assert!(is_fish_depth(FISH_END));
        assert!(is_fish_depth(10)); // Middle of fish range
        assert!(!is_fish_depth(SHARK));
        assert!(!is_fish_depth(SEAWEED));
    }

    #[test]
    fn test_water_surface_functions() {
        assert!(is_water_surface_depth(WATER_LINE0));
        assert!(is_water_surface_depth(WATER_GAP1));
        // Note: Water surface depths overlap with other entity depths due to layering design
        assert!(is_water_surface_depth(SHARK)); // Same as WATER_LINE3 (depth 2)
        assert!(is_water_surface_depth(FISH_START)); // Same as WATER_GAP3 (depth 3)
        assert!(!is_water_surface_depth(SEAWEED)); // Depth 21, not in water surface range
        assert!(!is_water_surface_depth(CASTLE)); // Depth 22, not in water surface range
    }

    #[test]
    fn test_water_depth_getters() {
        assert_eq!(water_line_depth(0), WATER_LINE0);
        assert_eq!(water_line_depth(3), WATER_LINE3);
        assert_eq!(water_gap_depth(1), WATER_GAP1);

        // Test fallback
        assert_eq!(water_line_depth(99), WATER_LINE0);
    }
}
