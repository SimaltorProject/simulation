use std::{
	f64::consts::PI,
	ops::{Add, AddAssign},
};

use crate::{
	cd::Cd,
	components::{self, AstronomicalObjectType, Mass, Orbiting},
	materials,
	resources::{self, Orbit, WorldRes},
	types::GalacticGrid,
	units,
};
use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;

pub(crate) mod planet;
pub(crate) mod stars;

pub(crate) struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(resources::WorldRes {
			sun_mass: Cd::new(1.0),
			orbits: vec![Orbit {
				starting_angle: Cd::new(0.0),
				eccentricity: Cd::new(0.9),
				semi_major_axis: Cd::new(149_600_000_000.0),
				orbiting_object: resources::OrbitingObject::Planet {
					radius: Cd::new(6_371_000.0 * 500.0),
				},
				..default()
			}],
			..default()
		});
		app.add_startup_system(gen);
		app.add_system(update);
		app.add_system(update_pos);
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
					starting_angle: *orbit.starting_angle,
					eccentricity: *orbit.eccentricity,
					semi_major_axis: *orbit.semi_major_axis,
					_inclanation: *orbit._inclanation,
					_argument_of_semi_major_axis: *orbit._argument_of_semi_major_axis,
					timer: Timer::from_seconds(4.7, TimerMode::Repeating),
				},
				**radius,
				(origin.as_ref(), DVec3::new(*orbit.semi_major_axis, 0.0, 0.0)),
				meshes.as_mut(),
				materials.as_mut(),
				(&GalacticGrid::ZERO, &transform, entity),
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

fn update_pos(
	origin: Res<FloatingOriginSettings>,
	mut world: ResMut<WorldRes>,
	mut orbiting_querry: Query<(Entity, &mut Transform, &mut GalacticGrid, Option<&mut Orbiting>)>,
	time: Res<Time>,
) {
	let mut translations_to_do: Vec<(Entity, Entity)> = vec![];
	orbiting_querry.for_each_mut(|orbiting_object| {
		let (entity, mut transform, mut object_cell, mut orbiting_option) = orbiting_object;
		if let Some(orbiting) = orbiting_option.as_mut() {
			orbiting.timer.tick(time.delta());

			let semi_major_axis = orbiting.semi_major_axis;
			let semi_minor_axis = semi_major_axis * (1.0 - orbiting.eccentricity.powf(2.0)).powf(0.5);

			let mean_anomaly = orbiting.timer.percent() as f64 * 2.0 * PI;

			let eccentric_anomaly = solve_kepler(mean_anomaly, orbiting.eccentricity);
			let x = semi_major_axis * (eccentric_anomaly.cos() - orbiting.eccentricity);
			let z = semi_minor_axis * eccentric_anomaly.sin();

			let (cell, translation) = origin.translation_to_grid::<i64>(DVec3::new(x, 0.0, z));
			transform.translation = translation;
			object_cell.apply(&cell);
			translations_to_do.push((entity, orbiting.center_of_mass));
		}
	});

	translations_to_do.iter().for_each(|pair| {
		let orbited = orbiting_querry.get(pair.1);
		if pair.0 == pair.1 {
			return;
		}
		// if above ensures that unsafe do not violates memory safety
		let mut orbiting = unsafe { orbiting_querry.get_unchecked(pair.0) };
		match (orbiting, orbited) {
			(Ok(mut orbiting), Ok(orbited)) => {
				orbiting.2.add_assign(*orbited.2);
				orbiting.1.translation.add_assign(orbited.1.translation);
			}
			_ => todo!(),
		}
	});

	//println!("mean_anomaly: {mean_anomaly}  eccentric_anomaly: {eccentric_anomaly} ")
}

fn solve_kepler(mean_anomaly: f64, eccentricity: f64) -> f64 {
	let accuracy = 0.0000001;
	let max_iterations = 100;
	let mut eccentric_anomaly = if eccentricity < 0.8 { mean_anomaly } else { PI };

	for _ in 0..max_iterations {
		let next_value = eccentric_anomaly
			- (kepler_equation(mean_anomaly, eccentric_anomaly, eccentricity)
				/ kepler_equation_differentiated(eccentric_anomaly, eccentricity));

		let diff = (eccentric_anomaly - next_value).abs();
		eccentric_anomaly = next_value;
		if diff < accuracy {
			break;
		}
	}

	eccentric_anomaly
}

fn kepler_equation(mean_anomaly: f64, eccentric_anomaly: f64, eccentricity: f64) -> f64 {
	// orginal version
	// mean_anomaly = eccentric_anomaly - (eccentricity * eccentric_anomaly.sin())
	// we want to compute eccentric_anomaly so for corrent eccentric_anomaly function returns 0
	eccentric_anomaly - (eccentricity * eccentric_anomaly.sin()) - mean_anomaly
}

// dervitive / pochodna
fn kepler_equation_differentiated(eccentric_anomaly: f64, eccentricity: f64) -> f64 {
	1.0 - (eccentricity * eccentric_anomaly.cos())
}
