use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct WorldRes {
	pub seed_global: u64,
	pub sun_mass: f64,
}
