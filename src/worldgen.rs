use std::{f64::consts::PI, ops::Mul};

use crate::{
	cd::Cd,
	components::{self, AstronomicalObjectType, Mass, Orbiting},
	materials,
	resources::{self, Orbit, WorldRes},
	types::GalacticGrid,
};
use bevy::{math::DVec3, prelude::*, sprite::MaterialMesh2dBundle};
use big_space::{FloatingOrigin, FloatingOriginSettings};

use orbital_montion::update_pos;

pub(crate) mod orbital_montion;
pub(crate) mod planet;
pub(crate) mod stars;

pub(crate) struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(resources::WorldRes {
			sun_mass: Cd::new(1.0),
			orbits: vec![Orbit {
				argument_of_semi_major_axis: Cd::new(PI * 0.0),
				phase_angle: Cd::new(PI),
				eccentricity: Cd::new(0.4),
				semi_major_axis: Cd::new(149_600_000_000.0),
				orbiting_object: resources::OrbitingObject::Planet {
					radius: Cd::new(6_371_000.0 * 500.0),
				},
				inclanation: Cd::new(0.02 * PI),
			}],
			..default()
		});
		app.add_systems(Startup, gen);
		app.add_systems(Update, (update, update_pos, draw_orbit_and_object));
	}
}

// TODO split gen & render
fn gen(
	world: Res<WorldRes>,
	origin: Res<FloatingOriginSettings>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut shader_materials: ResMut<Assets<materials::Sun>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let transform = Transform::default();

	let entity = commands
		.spawn((components::CommonCenterOfMass {}, GalacticGrid::ZERO, transform))
		.id();

	let sun_entity = commands
		.spawn(stars::gen(
			*world.sun_mass,
			(origin.as_ref(), DVec3::new(0.0, 0.0, 0.0)),
			meshes.as_mut(),
			shader_materials.as_mut(),
			(&GalacticGrid::ZERO, &transform, entity),
		))
		.id();

	world.orbits.iter().for_each(|orbit| {
		let object = &orbit.orbiting_object;
		match object {
			resources::OrbitingObject::Planet { radius } => commands.spawn(planet::gen(
				Orbiting {
					center_of_mass: sun_entity,
					phase_angle: *orbit.phase_angle,
					eccentricity: *orbit.eccentricity,
					semi_major_axis: *orbit.semi_major_axis,
					inclanation: *orbit.inclanation,
					argument_of_semi_major_axis: *orbit.argument_of_semi_major_axis,
					timer: Timer::from_seconds(500.0, TimerMode::Repeating),
				},
				**radius,
				(origin.as_ref(), DVec3::new(*orbit.semi_major_axis, 0.0, 0.0)),
				meshes.as_mut(),
				materials.as_mut(),
				(&GalacticGrid::ZERO, &transform),
			)),
		};
	});
}

fn update(
	mut world: ResMut<WorldRes>,
	mut objects: Query<(
		&AstronomicalObjectType,
		&mut Mass,
		&Handle<Mesh>,
		&Handle<materials::Sun>,
	)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut shader_materials: ResMut<Assets<materials::Sun>>,
) {
	if world.sun_mass.changed() {
		stars::update(
			objects.get_single_mut().expect("give me break"),
			(*world.sun_mass, 0.0),
			meshes.as_mut(),
			shader_materials.as_mut(),
		)
	}
	world.sun_mass.reset();
}

#[allow(clippy::type_complexity)]
pub(crate) fn draw_orbit_and_object(
	windows: Query<&Window>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	orbiting_querry: Query<(
		Entity,
		&Transform,
		&GlobalTransform,
		&GalacticGrid,
		Option<&Orbiting>,
		&AstronomicalObjectType,
	)>,
	main_cameras: Query<(&Camera, &GlobalTransform), With<FloatingOrigin>>,
) {
	let window = windows.single();
	let main_camera = main_cameras
		.get_single()
		.expect("There should be onnly one");
	let mut orbiting_orbited_pairs: Vec<(Entity, Entity)> = vec![];
	let overlay_size = Vec2::new(window.width().mul(0.75), window.height());

	orbiting_querry.for_each(|orbiting_object| {
		let (entity, _, global_tranform, _, orbiting_option, _) = orbiting_object;
		if let Some(orbiting) = orbiting_option {
			orbiting_orbited_pairs.push((entity, orbiting.center_of_mass));
		}
		if let Some(pos) = main_camera
			.0
			.world_to_ndc(main_camera.1, global_tranform.translation())
		{
			let radius = 2.;
			commands.spawn(MaterialMesh2dBundle {
				mesh: meshes.add(shape::Circle::new(radius).into()).into(),
				material: materials.add(ColorMaterial::from(Color::PURPLE)),
				transform: Transform::from_translation(Vec3::new(
					overlay_size.x.mul(pos.x * 0.5),
					overlay_size.y.mul(pos.y * 0.5),
					0.,
				)),
				..default()
			});
		}
	});

	orbiting_orbited_pairs.iter().for_each(|pair| {
		let orbited = orbiting_querry.get(pair.1);
		if pair.0 == pair.1 {
			return;
		}
		// if above ensures that unsafe do not violates memory safety
		let orbiting = unsafe { orbiting_querry.get_unchecked(pair.0) };
		match (orbiting, orbited) {
			(Ok(_orbiting), Ok(_orbited)) => {
				//todo draw
			}
			_ => todo!(),
		}
	});

	//println!("mean_anomaly: {mean_anomaly}  eccentric_anomaly: {eccentric_anomaly} ")
}
