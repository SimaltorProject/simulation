use crate::{
	components::{AstronomicalObjectType, Mass},
	materials,
	resources::{self, WorldRes},
	units,
};
use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;
use rand::{thread_rng, Rng, SeedableRng};

pub mod stars;

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
	mut shader_materials: ResMut<Assets<materials::Sun>>,
) {
	let mut rng = rand::rngs::StdRng::seed_from_u64(0);
	let mass_stellar: f64 = rng.gen_range(0.6..1.4);

	commands.spawn(stars::gen(
		mass_stellar,
		(
			origin.as_ref(),
			DVec3::new(units::SUN_RADIUS * 5.0, 0.0, -0.9 * units::SUN_RADIUS),
		),
		meshes.as_mut(),
		shader_materials.as_mut(),
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
