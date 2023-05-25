use bevy::prelude::*;

#[derive(Component)]
pub enum AstronomicalObjectType {
	Star(Entity),
	Planet,
}

#[derive(Component)]
pub struct Mass(pub f64);

#[derive(Component)]
pub struct CommonCenterOfMass {}
