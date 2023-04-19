use bevy::prelude::*;

#[derive(Component)]
pub enum AstronomicalObjectType {
	Star,
	Planet,
}

#[derive(Component)]
pub struct Mass(pub f64);
