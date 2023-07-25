use bevy::asset::ChangeWatcher;
use bevy::math::DVec3;
use bevy::render::camera::Viewport;
use bevy::window::CursorGrabMode;
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::EguiPlugin;
use big_space::camera::CameraInput;
use big_space::{camera::CameraController, FloatingOrigin, FloatingOriginPlugin};
use core::time::Duration;
use std::ops::Mul;

mod cd;
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
					watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
					..Default::default()
				})
				.build()
				.disable::<TransformPlugin>(),
		)
		.add_plugins(EguiPlugin)
		.add_plugins(FloatingOriginPlugin::<i64>::new(1_000.0, 1.0))
		.add_plugins(big_space::debug::FloatingOriginDebugPlugin::<i64>::default())
		.add_plugins(big_space::camera::CameraControllerPlugin::<i64>::default())
		.add_plugins(MaterialPlugin::<materials::Sun>::default())
		.add_plugins(worldgen::WorldGenPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, grab_mouse)
		.add_systems(Update, camera_resize)
		.add_systems(Update, ui::ui)
		.run();
}

pub(crate) fn setup(
	windows: Query<&Window>,
	mut commands: Commands,
	mut camera_input: ResMut<CameraInput>,
	origin_settings: Res<big_space::FloatingOriginSettings>,
) {
	//let (cell, translation) = origin_settings.translation_to_grid::<i64>(DVec3::new(units::SUN_RADIUS * -5.0, 0.0, 0.9 * units::SUN_RADIUS));
	let (cell, translation) = origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 151_600_000_000.0));
	let window = windows.single();

	commands.spawn((
		Camera3dBundle {
			camera: Camera {
				hdr: true,
				viewport: Some(Viewport {
					physical_size: UVec2::new(
						(window.physical_width() as f64).mul(0.75).ceil() as u32, //#![feature(int_roundings)] div_ceil
						window.physical_height(),
					),
					..default()
				}),
				..default()
			},
			transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		BloomSettings {
			intensity: 0.3, // the default is 0.3
			low_frequency_boost: 0.2,
			..default()
		},
		cell,
		FloatingOrigin,
		CameraController::default()
			.with_max_speed(2.0)
			.with_slowing(true), // Built-in camera controller
	));

	/*commands.spawn((
		Camera3dBundle {
			camera: Camera {
				hdr: true,
				viewport: Some(Viewport {
					physical_size: UVec2::new(
						(window.physical_width() as f64).mul(0.75).ceil() as u32, //#![feature(int_roundings)] div_ceil
						window.physical_height(),
					),
					..default()
				}),
				..default()
			},
			transform: Transform::from_translation(translation).looking_at(Vec3::new(0.01, -1., 0.), Vec3::Y),
			..default()
		},
		BloomSettings {
			intensity: 0.3, // the default is 0.3
			low_frequency_boost: 0.2,
			..default()
		},
		cell,
		FloatingOrigin,
		CameraController::default()
			.with_max_speed(2e9)
			.with_slowing(true), // Built-in camera controller
	));*/
	camera_input.defaults_disabled = true;
}

fn camera_resize(windows: Query<&Window>, mut cameras: Query<&mut Camera>) {
	let window = windows.single();
	let mut camera = cameras.single_mut();
	let size = UVec2::new(
		(window.physical_width() as f64).mul(0.75).ceil() as u32, //#![feature(int_roundings)] div_ceil
		window.physical_height(),
	);
	camera
		.viewport
		.as_mut()
		.map(|v| v.physical_size.apply(&size));
}

fn grab_mouse(
	mut windows: Query<&mut Window>,
	_mouse: Res<Input<MouseButton>>,
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

// TODO camera resizing
