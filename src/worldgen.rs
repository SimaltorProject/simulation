use crate::{
	components::{AstronomicalObjectType, Mass},
	materials,
};
use bevy::prelude::*;
use rand::{Rng, SeedableRng};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(add_sun);
	}
}

// TODO split gen & render
fn add_sun(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut shader_material: ResMut<Assets<materials::Sun>>,
) {
	let mut rng = rand::rngs::StdRng::seed_from_u64(0);
	let mass_in_sun_masses: f64 = rng.gen_range(0.6..1.4);
	let radius_in_sun_radius = mass_in_sun_masses.powf(0.8); // TODO Aproximation

	println!(
		"mass: {mass_in_sun_masses}   R: {} {}",
		radius_in_sun_radius, radius_in_sun_radius as f32
	);

	let radius_scaled = radius_in_sun_radius * 1e5;

	let sun_material = shader_material.add(materials::Sun {});

	commands.spawn((
		MaterialMeshBundle {
			mesh: meshes.add(
				shape::Icosphere {
					radius: radius_scaled as f32,
					subdivisions: 20,
				}
				.try_into()
				.expect("YYY how?...."),
			),
			material: sun_material,
			transform: Transform::from_xyz(5. * radius_scaled as f32, 0.0, 0.0),
			..default()
		},
		Mass(mass_in_sun_masses),
		AstronomicalObjectType::Star,
	));
	println!("OK")
}
