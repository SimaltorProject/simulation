use bevy::window::CursorGrabMode;
use bevy::{
	core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
	prelude::*,
};
use big_space::camera::CameraInput;
//use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use big_space::{camera::CameraController, FloatingOrigin, FloatingOriginPlugin};
use types::GalacticGrid;

mod components;
mod materials;
mod types;
mod units;
mod worldgen;

fn main() {
	App::new()
		.add_plugins(
			DefaultPlugins
				.set(AssetPlugin {
					watch_for_changes: true,
					..Default::default()
				})
				.build()
				.disable::<TransformPlugin>(),
		)
		.add_plugin(FloatingOriginPlugin::<i64>::new(1_000.0, 1.0))
		.add_plugin(big_space::debug::FloatingOriginDebugPlugin::<i64>::default())
		.add_plugin(big_space::camera::CameraControllerPlugin::<i64>::default())
		/* .add_plugin(NoCameraPlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.00015, // default: 0.00012
			speed: 2.5e4,         // default: 12.0
		})*/
		.add_plugin(MaterialPlugin::<materials::Sun>::default())
		.add_plugin(worldgen::WorldGenPlugin)
		.add_system(grab_mouse)
		.insert_resource(ClearColor(Color::BLACK))
		.add_startup_system(spawn_camera)
		.run();
}

pub fn spawn_camera(mut commands: Commands, mut camera_input: ResMut<CameraInput>) {
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
		//FlyCam,
		GalacticGrid::new(0, 0, 0),
		FloatingOrigin,
		CameraController::default().with_max_speed(10e35), // Built-in camera controller
	));
	camera_input.defaults_disabled = true;
}

fn grab_mouse(
	mut windows: Query<&mut Window>,
	mouse: Res<Input<MouseButton>>,
	key: Res<Input<KeyCode>>,
	mut camera_input: ResMut<CameraInput>,
) {
	let mut window = windows.single_mut();

	if mouse.just_pressed(MouseButton::Left) {
		camera_input.defaults_disabled = false;
		window.cursor.visible = false;
		window.cursor.grab_mode = CursorGrabMode::Locked;
	}

	if key.just_pressed(KeyCode::Escape) {
		camera_input.defaults_disabled = true;
		window.cursor.visible = true;
		window.cursor.grab_mode = CursorGrabMode::None;
	}
}
