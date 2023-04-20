use bevy::{
	core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
	prelude::*,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

mod components;
mod materials;
mod worldgen;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			watch_for_changes: true,
			..Default::default()
		}))
		.add_plugin(NoCameraPlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.00015, // default: 0.00012
			speed: 2.5e4,         // default: 12.0
		})
		.add_plugin(MaterialPlugin::<materials::Sun>::default())
		.add_plugin(worldgen::WorldGenPlugin)
		.insert_resource(ClearColor(Color::BLACK))
		.add_startup_system(spawn_camera)
		.run();
}

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			camera: Camera { hdr: true, ..default() },
			tonemapping: Tonemapping::TonyMcMapface,
			transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(1., 0., 0.), Vec3::Y),
			..default()
		},
		BloomSettings {
			intensity: 0.3, // the default is 0.3
			low_frequency_boost: 0.45,
			..default()
		},
		FlyCam,
	));
}
