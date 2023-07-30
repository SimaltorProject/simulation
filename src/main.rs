use bevy::asset::ChangeWatcher;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::math::DVec3;
use bevy::render::camera::Viewport;
use bevy::window::{CursorGrabMode, WindowResized};
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::EguiPlugin;
use big_space::camera::CameraInput;
use big_space::{camera::CameraController, FloatingOrigin};
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
		.add_plugins((
			DefaultPlugins
				.set(AssetPlugin {
					watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
					..Default::default()
				})
				.build()
				.disable::<TransformPlugin>(),
			EguiPlugin,
			big_space::FloatingOriginPlugin::<i64>::new(1_000.0, 1.0),
			big_space::debug::FloatingOriginDebugPlugin::<i64>::default(),
			big_space::camera::CameraControllerPlugin::<i64>::default(),
			MaterialPlugin::<materials::Sun>::default(),
			worldgen::WorldGenPlugin,
		))
		.add_systems(Startup, setup)
		.add_systems(Update, (grab_mouse, camera_resize, ui::ui))
		.run();
}

pub(crate) fn setup(
	windows: Query<&Window>,
	mut commands: Commands,
	mut camera_input: ResMut<CameraInput>,
	origin_settings: Res<big_space::FloatingOriginSettings>,
) {
	//let (cell, translation) = origin_settings.translation_to_grid::<i64>(DVec3::new(units::SUN_RADIUS * -5.0, 0.0, 0.9 * units::SUN_RADIUS));
	let (cell, translation) = origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 200_000_000_000.0));
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
	println!(
		"{:?} {:?}",
		(window.physical_width() as f32).mul(0.5),
		(window.physical_height() as f32).mul(-0.5)
	);
	commands.spawn((Camera2dBundle {
		camera_2d: Camera2d {
			clear_color: ClearColorConfig::None,
		},
		camera: Camera {
			hdr: true,
			order: 1,
			viewport: Some(Viewport {
				physical_size: UVec2::new(
					(window.physical_width() as f64).mul(0.75).ceil() as u32, //#![feature(int_roundings)] div_ceil
					window.physical_height(),
				),
				..default()
			}),
			..default()
		},
		transform: Transform::from_translation(Vec3::from_array([0.0, 0.0, 1.0])),
		..default()
	},));

	camera_input.defaults_disabled = true;
}

fn camera_resize(
	mut resize_events: EventReader<WindowResized>,
	windows: Query<&Window>,
	mut cameras: Query<&mut Camera>,
) {
	for resize_event in resize_events.iter() {
		let window = windows.get(resize_event.window).unwrap();
		cameras.for_each_mut(|mut camera| {
			let size = UVec2::new(
				(window.physical_width() as f64).mul(0.75).ceil() as u32, //#![feature(int_roundings)] div_ceil
				window.physical_height(),
			);
			if let Some(v) = camera.viewport.as_mut() {
				v.physical_size.apply(&size)
			}
		});
	}
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
