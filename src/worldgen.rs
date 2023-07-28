use std::f64::consts::PI;

use crate::{
	cd::Cd,
	components::{self, AstronomicalObjectType, Mass, Orbiting},
	materials,
	resources::{self, Orbit, WorldRes},
	types::GalacticGrid,
};
use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;

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
				..default()
			}],
			..default()
		});
		app.add_systems(Startup, gen);
		app.add_systems(Update, (update, update_pos));
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

pub(crate) fn draw_orbit(orbiting_querry: Query<(Entity, &Transform, &GalacticGrid, Option<&Orbiting>)>) {
	let mut orbiting_orbited_pairs: Vec<(Entity, Entity)> = vec![];
	orbiting_querry.for_each(|orbiting_object| {
		let (entity, transform, object_cell, orbiting_option) = orbiting_object;
		if let Some(orbiting) = orbiting_option {
			orbiting_orbited_pairs.push((entity, orbiting.center_of_mass));
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
			(Ok(orbiting), Ok(orbited)) => {
				//todo draw
			}
			_ => todo!(),
		}
	});

	//println!("mean_anomaly: {mean_anomaly}  eccentric_anomaly: {eccentric_anomaly} ")
}
