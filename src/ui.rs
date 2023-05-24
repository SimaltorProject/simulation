use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::resources::WorldRes;

#[derive(Resource)]
pub struct UiState {
	sun_mass: f32,
}

impl Default for UiState {
	fn default() -> Self {
		Self { sun_mass: 1.0 }
	}
}

pub fn ui(
	mut ui_state: ResMut<UiState>,
	//mut rendered_texture_id: Local<egui::TextureId>,
	mut is_initialized: Local<bool>,
	mut contexts: EguiContexts,
	windows: Query<&Window>,
	mut world: ResMut<WorldRes>,
) {
	let window = windows.get_single().expect("There is no window??");
	//let font_handle = asset_server.load("fonts/Lato-Regular.ttf");
	if !*is_initialized {
		*is_initialized = true;
	}
	let ctx = contexts.ctx_mut();

	egui::SidePanel::right("side_panel")
		.exact_width(window.resolution.width() * 0.25)
		.resizable(false)
		.show(ctx, |ui| {
			ui.heading("Settings");
			ui.horizontal(|ui| {
				ui.add(egui::Slider::new(&mut ui_state.sun_mass, 0.6..=1.4).text("Star mass (solar masses)"));
			});
		});

	world.sun_mass = ui_state.sun_mass; // TODO redesign
}
/*
commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::width(Val::Percent(100.0)),
				justify_content: JustifyContent::End,
				align_items: AlignItems::Center,
				..default()
			},
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::Start,
						align_items: AlignItems::Center,
						padding: UiRect::all(Val::Percent(1.0)),
						margin: UiRect {
							right: Val::Percent(2.5),
							..default()
						},
						size: Size::new(Val::Percent(20.0), Val::Percent(90.0)),
						..default()
					},
					background_color: Color::rgba(0.051, 0.07, 0.09, 0.9).into(),
					..default()
				})
				.with_children(|parent| {
					parent.spawn((
						TextBundle::from_section(
							"Settings",
							TextStyle {
								font: font_handle,
								font_size: 25.,
								color: Color::WHITE,
							},
						)
						.with_style(Style {
							size: Size::height(Val::Px(25.)),
							..default()
						}),
						Label,
					));
					parent
						.spawn((
							NodeBundle {
								style: Style {
									flex_direction: FlexDirection::Column,
									max_size: Size::UNDEFINED,
									align_items: AlignItems::Center,
									..default()
								},
								..default()
							},
							//ScrollingList::default(),
							//AccessibilityNode(NodeBuilder::new(Role::List)),
						))
						.with_children(|parent| {});
				});
		}); */
