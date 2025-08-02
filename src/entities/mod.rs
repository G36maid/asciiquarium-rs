//! Entity implementations for the asciiquarium
//!
//! This module contains all the different types of entities that can appear
//! in the aquarium, including fish, bubbles, seaweed, and other creatures.

pub mod bubble;
pub mod castle;
pub mod fish;
pub mod sea_monster;
pub mod seaweed;
pub mod shark;
pub mod ship;
pub mod water_surface;
pub mod whale;

pub use bubble::Bubble;
pub use castle::Castle;
pub use fish::{Fish, FishSpecies};
pub use sea_monster::SeaMonster;
pub use seaweed::Seaweed;
pub use shark::{Shark, SharkTeeth};
pub use ship::Ship;
pub use water_surface::WaterSurface;
pub use whale::Whale;
