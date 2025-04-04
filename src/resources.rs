use bevy::prelude::Resource;

use crate::cd::Cd;

#[derive(Resource, Default)]
pub(crate) struct WorldRes {
	pub seed_global: Cd<u64>,
	pub sun_mass: Cd<f64>,
	pub _orbits_seed: Cd<u64>,
	pub orbits: Vec<Orbit>,
}

#[derive(Resource, Default)]
pub(crate) struct Orbit {
	pub eccentricity: Cd<f64>,
	pub phase_angle: Cd<f64>,
	pub semi_major_axis: Cd<f64>,
	pub inclanation: Cd<f64>,
	pub argument_of_semi_major_axis: Cd<f64>,
	pub orbiting_object: OrbitingObject,
}

#[derive(Resource)]
pub(crate) enum OrbitingObject {
	Planet { radius: Cd<f64> },
}

impl Default for OrbitingObject {
	fn default() -> Self {
		OrbitingObject::Planet { radius: Cd::new(0.0) }
	}
}
