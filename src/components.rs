use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub(crate) enum AstronomicalObjectType {
	Star(Entity),
	Planet,
}

#[derive(Component)]
pub(crate) struct Mass(pub(crate) f64);

#[derive(Component)]
pub(crate) struct CommonCenterOfMass {}

#[derive(Component)]
pub(crate) struct Orbiting {
	pub center_of_mass: Entity,
	pub starting_angle: f64,
	pub eccentricity: f64,
	pub semi_major_axis: f64,
	pub _inclanation: f64,
	pub _argument_of_semi_major_axis: f64,
	pub timer: Timer,
}
