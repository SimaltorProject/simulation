use bevy::{prelude::Resource, time::Timer};

use crate::cd::Cd;

#[derive(Resource, Default)]
pub(crate) struct WorldRes {
	pub(crate) seed_global: Cd<u64>,
	pub(crate) sun_mass: Cd<f64>,
	pub(crate) orbits_seed: Cd<u64>,
	pub(crate) orbits: Vec<Orbit>,
}

#[derive(Resource, Default)]
pub(crate) struct Orbit {
	pub starting_angle: Cd<f64>,
	pub eccentricity: Cd<f64>,
	pub semi_major_axis: Cd<f64>,
	pub _inclanation: Cd<f64>,
	pub _argument_of_semi_major_axis: Cd<f64>,
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
