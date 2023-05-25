use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use bevy::{
	a11y::{
		accesskit::{NodeBuilder, Role},
		AccessibilityNode,
	},
	core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
	prelude::*,
};
use bevy_egui::EguiPlugin;
use big_space::camera::CameraInput;
use big_space::{camera::CameraController, FloatingOrigin, FloatingOriginPlugin};
use types::GalacticGrid;

mod components;
mod materials;
mod resources;
mod types;
mod ui;
mod units;
mod worldgen;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::BLACK))
		.insert_resource(Msaa::Sample4)
		.init_resource::<ui::UiState>()
		.add_plugins(
			DefaultPlugins
				.set(AssetPlugin {
					watch_for_changes: true,
					..Default::default()
				})
				.build()
				.disable::<TransformPlugin>(),
		)
		.add_plugin(EguiPlugin)
		.add_plugin(FloatingOriginPlugin::<i64>::new(1_000.0, 1.0))
		.add_plugin(big_space::debug::FloatingOriginDebugPlugin::<i64>::default())
		.add_plugin(big_space::camera::CameraControllerPlugin::<i64>::default())
		.add_plugin(MaterialPlugin::<materials::Sun>::default())
		.add_plugin(worldgen::WorldGenPlugin)
		.add_system(grab_mouse)
		.add_startup_system(setup)
		.add_system(ui::ui)
		.run();
}

pub fn setup(mut commands: Commands, mut camera_input: ResMut<CameraInput>) {
	commands.spawn((
		Camera3dBundle {
			camera: Camera { hdr: true, ..default() },
			tonemapping: Tonemapping::TonyMcMapface,
			transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(1., 0., 0.), Vec3::Y),
			..default()
		},
		BloomSettings {
			intensity: 0.3, // the default is 0.3
			low_frequency_boost: 0.2,
			..default()
		},
		GalacticGrid::new(0, 0, 0),
		FloatingOrigin,
		CameraController::default()
			.with_max_speed(2e9)
			.with_slowing(false), // Built-in camera controller
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

	/*if mouse.just_pressed(MouseButton::Left) {
		camera_input.defaults_disabled = false;
		window.cursor.visible = false;
		wwindow.cursor.grab_mode = CursorGrabMode::Locked;
	}*/

	if key.just_pressed(KeyCode::Escape) {
		camera_input.defaults_disabled = !camera_input.defaults_disabled;
		window.cursor.visible = !window.cursor.visible;
		window.cursor.grab_mode = if window.cursor.grab_mode == CursorGrabMode::None {
			CursorGrabMode::Locked
		} else {
			CursorGrabMode::None
		};
	}
}
