//! Entity implementations for the asciiquarium
//!
//! This module contains all the different types of entities that can appear
//! in the aquarium, including fish, bubbles, seaweed, and other creatures.

pub mod bubble;
pub mod fish;
pub mod seaweed;
pub mod water_surface;

pub use bubble::Bubble;
pub use fish::{Fish, FishSpecies};
pub use seaweed::{Seaweed, SeaweedManager};
pub use water_surface::{WaterSurface, WaterSurfaceManager};
