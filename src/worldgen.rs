use crate::{
	components::{self, AstronomicalObjectType, Mass},
	materials,
	resources::{self, WorldRes},
	types::GalacticGrid,
	units,
};
use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;
use rand::{Rng, SeedableRng};

pub mod stars;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(resources::WorldRes {
			sun_mass: 1.0,
			..default()
		});
		app.add_startup_system(gen);
		app.add_system(update);
	}
}

// TODO split gen & render
fn gen(
	origin: Res<FloatingOriginSettings>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut shader_materials: ResMut<Assets<materials::Sun>>,
) {
	let mass_stellar: f64 = 1.0;
	let transform = Transform::default();

	let entity = commands
		.spawn((components::CommonCenterOfMass {}, GalacticGrid::ZERO, transform))
		.id();

	commands.spawn(stars::gen(
		mass_stellar,
		(origin.as_ref(), DVec3::new(0.0, 0.0, 0.0)),
		meshes.as_mut(),
		shader_materials.as_mut(),
		(&GalacticGrid::ZERO, &transform, entity),
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
	stars::update(
		objects.get_single_mut().expect("give me break"),
		(world.sun_mass as f64, 0.0),
		meshes.as_mut(),
		shader_materials.as_mut(),
	)
}
