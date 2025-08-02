//! Entity implementations for the asciiquarium
//!
//! This module contains all the different types of entities that can appear
//! in the aquarium, including fish, bubbles, seaweed, and other creatures.

pub mod bubble;
pub mod castle;
pub mod fish;
pub mod seaweed;
pub mod shark;
pub mod water_surface;

pub use bubble::Bubble;
pub use castle::{Castle, CastleManager};
pub use fish::{Fish, FishSpecies};
pub use seaweed::{Seaweed, SeaweedManager};
pub use shark::{Shark, SharkManager, SharkTeeth};
pub use water_surface::{WaterSurface, WaterSurfaceManager};
