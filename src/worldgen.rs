use crate::{
	components::{AstronomicalObjectType, Mass},
	materials,
	types::GalacticGrid,
	units,
};
use bevy::{math::DVec3, prelude::*};
use big_space::{FloatingOrigin, FloatingOriginSettings};
use rand::{Rng, SeedableRng};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(add_sun);
	}
}

// TODO split gen & render
fn add_sun(
	origin: Res<FloatingOriginSettings>,
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

	let radius = radius_in_sun_radius * units::SUN_RADIUS;

	let sun_material = shader_material.add(materials::Sun {});

	let (cell, translation) = origin.translation_to_grid::<i64>(DVec3::new(5.0 * radius, 0.0, 0.0));

	commands.spawn((
		MaterialMeshBundle {
			mesh: meshes.add(
				shape::Icosphere {
					radius: radius as f32,
					subdivisions: 20,
				}
				.try_into()
				.expect("YYY how?...."),
			),
			material: sun_material,
			transform: Transform::from_translation(translation),
			..default()
		},
		Mass(mass_in_sun_masses),
		AstronomicalObjectType::Star,
		cell,
	));
	println!("OK")
}
