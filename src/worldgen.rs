use crate::{
	components::{AstronomicalObjectType, Mass},
	materials,
	resources::{self, WorldRes},
	units,
};
use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;
use rand::{thread_rng, Rng, SeedableRng};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(resources::WorldRes { sun_mass: 1.0 });
		app.add_startup_system(add_sun);
		app.add_system(update);
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

	//println!(
	//	"mass: {mass_in_sun_masses}   R: {} {}",
	//	radius_in_sun_radius, radius_in_sun_radius as f32
	//);

	let radius = radius_in_sun_radius * units::SUN_RADIUS;

	let sun_material = shader_material.add(materials::Sun {
		color: Color::YELLOW,
		luminosity: (mass_in_sun_masses.powf(4.0) * units::LUMINOSITY_MULTIPLAYER) as f32,
	});

	let (cell, translation) = origin.translation_to_grid::<i64>(DVec3::new(units::SUN_RADIUS * 5.0, 0.0, 0.0));

	commands.spawn((
		MaterialMeshBundle {
			mesh: meshes.add(
				shape::Icosphere {
					radius: radius as f32,
					subdivisions: 40,
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

fn update(
	world: Res<WorldRes>,
	mut objects: Query<(
		&AstronomicalObjectType,
		&mut Mass,
		&Handle<Mesh>,
		&Handle<materials::Sun>,
	)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut shader_materials: ResMut<Assets<materials::Sun>>,
) {
	let (_, mut mass, mesh_handle, material_handle) = objects.get_single_mut().expect("give me break");
	mass.0 = world.sun_mass as f64;
	let radius_in_sun_radius = mass.0.powf(0.8); // TODO Aproximation and extract function funtions
	let radius = radius_in_sun_radius * units::SUN_RADIUS;
	let mesh = meshes.get_mut(mesh_handle).expect("yyy");
	mesh.clone_from(
		&shape::Icosphere {
			radius: radius as f32,
			subdivisions: 40,
		}
		.try_into()
		.expect("YYY how?...."),
	);
	let material = shader_materials
		.get_mut(material_handle)
		.expect("yyy say what #55");
	material.luminosity = (mass.0.powf(4.0) * units::LUMINOSITY_MULTIPLAYER) as f32
}
