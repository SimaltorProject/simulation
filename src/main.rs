extern crate colortemp;
use bevy::{
	core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
	prelude::*,
	render::camera::Viewport,
};

mod components;
mod worldgen;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(worldgen::WorldGenPlugin)
		.insert_resource(ClearColor(Color::BLACK))
		.add_startup_system(spawn_camera)
		.run();
}

pub fn spawn_camera(windows: Query<&Window>, mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::width(Val::Percent(25.0)),
				justify_content: JustifyContent::SpaceBetween,
				..default()
			},
			..default()
		})
		.with_children(|parent| {
			// left vertical fill (content)
			parent
				.spawn(NodeBundle {
					style: Style {
						size: Size::width(Val::Percent(100.0)),
						..default()
					},
					background_color: Color::rgb(0.15, 0.15, 0.15).into(),
					..default()
				})
				.with_children(|parent| {
					// text
					parent.spawn((
						TextBundle::from_section(
							"Text Example",
							TextStyle {
								font: asset_server.load("fonts/FiraSans-Bold.ttf"),
								font_size: 30.0,
								color: Color::WHITE,
							},
						)
						.with_style(Style {
							margin: UiRect::all(Val::Px(5.0)),
							..default()
						}),
						// Because this is a distinct label widget and
						// not button/list item text, this is necessary
						// for accessibility to treat the text accordingly.
						Label,
					));
				});
		});

	let window = windows.get_single().unwrap();

	commands.spawn((
		Camera3dBundle {
			camera: Camera {
				hdr: true,
				viewport: Some(Viewport {
					physical_position: UVec2::new(window.resolution.physical_width() / 4, 0),
					physical_size: UVec2::new(
						(window.resolution.physical_width() * 3) / 4,
						window.resolution.physical_height(),
					),
					..default()
				}),
				..default()
			},
			tonemapping: Tonemapping::TonyMcMapface,
			transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(1., 0., 0.), Vec3::Y),
			..default()
		},
		BloomSettings::default(),
	));
}
