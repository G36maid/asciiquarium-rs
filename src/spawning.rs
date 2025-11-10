//! Simple spawning functions to replace complex manager system
//!
//! This module contains simple functions for spawning entities, matching the
//! original Perl implementation's approach of using function references instead
//! of complex manager classes.

use crate::entities::*;
use crate::entity::{Entity, EntityManager};
use rand::Rng;
use ratatui::layout::Rect;

/// Add a fish (death callback for fish)
pub fn add_fish(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    let fish_id = entity_manager.get_next_id();
    let classic_mode = entity_manager.classic_mode();
    let fish = Fish::new_random(fish_id, screen_bounds, classic_mode);
    entity_manager.add_entity(Box::new(fish));
}

/// Add seaweed (death callback for seaweed)
pub fn add_seaweed(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    let seaweed_id = entity_manager.get_next_id();
    let seaweed = Seaweed::new_random(seaweed_id, screen_bounds);
    entity_manager.add_entity(Box::new(seaweed));
}

/// Random object spawner - spawns one random large creature (original behavior)
pub fn random_object(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    // Only spawn if no large creature exists (original constraint)
    if entity_manager.has_large_creature() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Original random objects array
    let spawners: &[fn(&mut EntityManager, Rect)] = &[
        add_ship,
        add_whale,
        add_sea_monster,
        add_big_fish,
        add_shark,
    ];

    // Random selection like original: int(rand(scalar(@random_objects)))
    let index = rng.gen_range(0..spawners.len());
    spawners[index](entity_manager, screen_bounds);
}

/// Add a ship (large creature)
pub fn add_ship(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return; // Only one large creature at a time
    }

    let ship_id = entity_manager.get_next_id();
    let ship = Ship::new(ship_id, screen_bounds);
    entity_manager.set_large_creature(ship_id);
    entity_manager.add_entity(Box::new(ship));
}

/// Add a whale (large creature)
pub fn add_whale(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return; // Only one large creature at a time
    }

    let whale_id = entity_manager.get_next_id();
    let whale = Whale::new(whale_id, screen_bounds);
    entity_manager.set_large_creature(whale_id);
    entity_manager.add_entity(Box::new(whale));
}

/// Add a sea monster (large creature)
pub fn add_sea_monster(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return; // Only one large creature at a time
    }

    let monster_id = entity_manager.get_next_id();
    let classic_mode = entity_manager.classic_mode();
    let monster = SeaMonster::new(monster_id, screen_bounds, classic_mode);
    entity_manager.set_large_creature(monster_id);
    entity_manager.add_entity(Box::new(monster));
}

/// Add a shark (large creature) - special case with teeth cleanup
pub fn add_shark(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return; // Only one large creature at a time
    }

    let shark_id = entity_manager.get_next_id();
    let teeth_id = entity_manager.get_next_id();

    // Create shark
    let mut shark = Shark::new_random(shark_id, screen_bounds);

    // Create teeth at shark's teeth position
    let teeth_position = shark.get_teeth_position();
    let teeth_velocity = Entity::velocity(&shark);
    let teeth = SharkTeeth::new(teeth_id, teeth_position, teeth_velocity, shark_id);

    // Associate shark with teeth
    shark.set_teeth_id(teeth_id);

    entity_manager.set_large_creature(shark_id);
    entity_manager.add_entity(Box::new(shark));
    entity_manager.add_entity(Box::new(teeth));
}

/// Add a big fish (large creature)
pub fn add_big_fish(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    if entity_manager.has_large_creature() {
        return; // Only one large creature at a time
    }

    let fish_id = entity_manager.get_next_id();
    let classic_mode = entity_manager.classic_mode();
    let big_fish = BigFish::new(fish_id, screen_bounds, classic_mode);
    entity_manager.set_large_creature(fish_id);
    entity_manager.add_entity(Box::new(big_fish));
}

/// Shark death callback - cleans up teeth and spawns new random object
pub fn shark_death(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    // Remove any remaining shark teeth
    let teeth_ids: Vec<_> = entity_manager
        .get_entities_by_type("shark_teeth")
        .iter()
        .map(|e| e.id())
        .collect();

    for teeth_id in teeth_ids {
        entity_manager.remove_entity(teeth_id);
    }

    // Spawn new random large creature
    random_object(entity_manager, screen_bounds);
}

/// Initialize all fish population based on screen size (original formula)
pub fn add_all_fish(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    // Original formula: (height - 9) * width / 350
    let screen_size =
        (screen_bounds.height.saturating_sub(9)) as usize * screen_bounds.width as usize;
    let fish_count = screen_size / 350;

    for _ in 0..fish_count {
        add_fish(entity_manager, screen_bounds);
    }
}

/// Initialize all seaweed population based on screen width (original formula)
pub fn add_all_seaweed(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    // Original formula: width / 15
    let seaweed_count = (screen_bounds.width as usize / 15).max(1);

    for _ in 0..seaweed_count {
        add_seaweed(entity_manager, screen_bounds);
    }
}

/// Initialize water surface
pub fn add_environment(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    let start_id = entity_manager.get_next_id();

    // Create 4 water surface layers
    for layer_index in 0..4 {
        let layer = WaterSurface::new(
            start_id + layer_index as u64,
            layer_index,
            screen_bounds.width,
        );
        entity_manager.add_entity(Box::new(layer));
    }
}

/// Initialize castle
pub fn add_castle(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    let castle_id = entity_manager.get_next_id();
    let castle = Castle::new(castle_id, screen_bounds);
    entity_manager.add_entity(Box::new(castle));
}

/// Complete initialization sequence (matching original Perl main loop)
pub fn initialize_aquarium(entity_manager: &mut EntityManager, screen_bounds: Rect) {
    // Match original initialization order:
    // add_environment($anim);
    // add_castle($anim);
    // add_all_seaweed($anim);
    // add_all_fish($anim);
    // random_object(undef, $anim);

    add_environment(entity_manager, screen_bounds);
    add_castle(entity_manager, screen_bounds);
    add_all_seaweed(entity_manager, screen_bounds);
    add_all_fish(entity_manager, screen_bounds);
    random_object(entity_manager, screen_bounds);
}
