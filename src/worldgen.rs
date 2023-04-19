use crate::components::{AstronomicalObjectType, Mass};
use bevy::prelude::*;
use rand::{Rng, SeedableRng};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(add_sun);
	}
}

fn add_sun(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let mut rng = rand::rngs::StdRng::seed_from_u64(0);
	let massInSunMasses: f64 = rng.gen_range(0.6..1.4);
	let radiusInSunRadius = massInSunMasses.powf(0.8); // TODO Aproximation

	println!(
		"mass: {massInSunMasses}   R: {} {}",
		radiusInSunRadius, radiusInSunRadius as f32
	);

	let material_emissive1 = materials.add(StandardMaterial {
		emissive: Color::rgb_linear(10., 0., 0.),
		..default()
	});

	commands.spawn((
		PbrBundle {
			mesh: meshes.add(
				shape::Icosphere {
					radius: radiusInSunRadius as f32,
					subdivisions: 10,
				}
				.try_into()
				.expect("YYY how?...."),
			),
			material: material_emissive1,
			transform: Transform::from_xyz(10. * radiusInSunRadius as f32, 0.0, 0.0),
			..default()
		},
		Mass(massInSunMasses),
		AstronomicalObjectType::Star,
	));
	println!("OK")
}
