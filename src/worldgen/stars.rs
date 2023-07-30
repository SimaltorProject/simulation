use bevy::{
	math::DVec3,
	prelude::{default, shape, Assets, Color, Entity, Handle, MaterialMeshBundle, Mesh, Mut, Transform},
};
use big_space::FloatingOriginSettings;

use crate::{components, materials, types, units};

pub(crate) fn gen(
	mass_stellar: f64,
	(origin_settings, pos): (&FloatingOriginSettings, DVec3),
	meshes: &mut Assets<Mesh>,
	shader_materials: &mut Assets<materials::Sun>,
	(mass_center_cell, mass_center_transform, mass_center_entity): (&types::GalacticGrid, &Transform, Entity),
) -> (
	MaterialMeshBundle<materials::Sun>,
	components::Mass,
	components::AstronomicalObjectType,
	types::GalacticGrid,
) {
	let radius_stellar = units::calculations::stars::radius(mass_stellar); // TODO Aproximation
	let temperature = units::calculations::stars::temperature(mass_stellar);

	//println!(
	//	"mass: {mass_in_sun_masses}   R: {} {}",
	//	radius_in_sun_radius, radius_in_sun_radius as f32
	//);

	let radius = radius_stellar * units::SUN_RADIUS;

	let sun_material = shader_materials.add(materials::Sun {
		color: Color::from(units::calculations::stars::color(temperature)),
		luminosity: (units::calculations::stars::luminosity(mass_stellar) * units::LUMINOSITY_MULTIPLAYER) as f32,
	});

	let (cell, translation) = origin_settings.translation_to_grid::<i64>(pos);

	(
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
			transform: Transform::from_translation(translation).with_translation(mass_center_transform.translation),
			..default()
		},
		components::Mass(mass_stellar),
		components::AstronomicalObjectType::Star(mass_center_entity),
		cell + *mass_center_cell,
	)
}

pub(crate) fn update(
	(_, mut mass, mesh_handle, material_handle): (
		&components::AstronomicalObjectType,
		Mut<'_, components::Mass>,
		&Handle<Mesh>,
		&Handle<materials::Sun>,
	),
	(new_mass, _new_age): (f64, f64),
	meshes: &mut Assets<Mesh>,
	shader_materials: &mut Assets<materials::Sun>,
) {
	mass.0 = new_mass;
	let temperature = units::calculations::stars::temperature(mass.0);
	let luminosity = units::calculations::stars::luminosity(mass.0);
	let radius_in_sun_radius = units::calculations::stars::radius(mass.0); // TODO Aproximation and extract function funtions
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
	material.color = Color::from(units::calculations::stars::color(temperature));
	material.luminosity = (luminosity * units::LUMINOSITY_MULTIPLAYER) as f32
}
