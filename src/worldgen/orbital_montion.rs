use std::{f64::consts::PI, ops::AddAssign};

use bevy::{
	math::{DQuat, DVec3},
	prelude::*,
};
use big_space::FloatingOriginSettings;

use crate::{components::Orbiting, types::GalacticGrid};

pub(crate) fn update_pos(
	origin: Res<FloatingOriginSettings>,
	//world: Res<WorldRes>,
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

			let mean_anomaly = orbiting.timer.percent() as f64 * 2.0 * PI + orbiting.phase_angle;

			let eccentric_anomaly = solve_kepler(mean_anomaly, orbiting.eccentricity);
			let x = semi_major_axis * (eccentric_anomaly.cos() - orbiting.eccentricity);
			let z = semi_minor_axis * eccentric_anomaly.sin();

			let inclined_plane = DQuat::from_rotation_x(orbiting.inclanation);
			let parametric_angle = DQuat::from_rotation_y(orbiting.argument_of_semi_major_axis);

			let pos: DVec3 = parametric_angle * inclined_plane * DVec3::new(x, 0.0, z);

			let (cell, translation) = origin.translation_to_grid::<i64>(pos);
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
		let orbiting = unsafe { orbiting_querry.get_unchecked(pair.0) };
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
	let accuracy = 1.0e-12;
	let max_iterations = 100; // average 5-6
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
