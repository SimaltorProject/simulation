use bevy::{math::DVec3, prelude::*};
use big_space::FloatingOriginSettings;

use crate::{
	components::{self, Orbiting},
	types,
};

pub(crate) fn gen(
	orbiting: Orbiting,
	radius_planet: f64,
	(origin_settings, pos): (&FloatingOriginSettings, DVec3),
	meshes: &mut Assets<Mesh>,
	material: &mut Assets<StandardMaterial>,
	(mass_center_cell, mass_center_transform): (&types::GalacticGrid, &Transform),
) -> (
	MaterialMeshBundle<StandardMaterial>,
	components::Mass,
	components::AstronomicalObjectType,
	types::GalacticGrid,
	Orbiting,
) {
	let (cell, translation) = origin_settings.translation_to_grid::<i64>(pos);
	(
		MaterialMeshBundle {
			mesh: meshes.add(
				shape::Icosphere {
					radius: radius_planet as f32,
					subdivisions: 40,
				}
				.try_into()
				.expect("YYY how?...."),
			),
			material: material.add(StandardMaterial {
				base_color: Color::BLUE,
				emissive: Color::BLUE,
				..default()
			}),
			transform: Transform::from_translation(translation).with_translation(mass_center_transform.translation),
			..default()
		},
		components::Mass(0.0),
		components::AstronomicalObjectType::Planet,
		cell + *mass_center_cell,
		orbiting,
	)
}
