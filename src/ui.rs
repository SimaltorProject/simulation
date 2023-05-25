use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand::{thread_rng, Rng, SeedableRng};

use crate::resources::WorldRes;

#[derive(Resource, Clone, PartialEq)]
pub struct UiState {
	global_seed_str: String,
	invalid_seed: bool,
	sun_mass_min: f64,
	sun_mass_max: f64,
}

impl Default for UiState {
	fn default() -> Self {
		let mut rng = thread_rng();
		Self {
			global_seed_str: rng.gen::<u64>().to_string(),
			invalid_seed: false,
			sun_mass_min: 0.5,
			sun_mass_max: 1.4,
		}
	}
}

pub fn ui(
	mut ui_state: ResMut<UiState>,
	mut ui_previus_state: Local<UiState>,
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
	let mut rng = thread_rng();
	//let range = 0.5..=1.4; // habitable

	//let range = 0.2..=10.0; // sensible

	//let range = 0.179..=31.0; // computable - i am not respoinse for this

	egui::SidePanel::right("side_panel")
		.exact_width(window.resolution.width() * 0.25)
		.resizable(false)
		.show(ctx, |ui| {
			ui.heading("Settings");
			ui.add_space(10.0);
			ui.heading("Star");
			ui.horizontal(|ui| {
				if ui_state.invalid_seed {
					ui.style_mut().visuals.extreme_bg_color = egui::Color32::DARK_RED;
				}
				ui.text_edit_singleline(&mut ui_state.global_seed_str);

				if ui.button("R").clicked() {
					let seed: u64 = rng.gen();
					ui_state.global_seed_str = seed.to_string();
				}
			});

			ui.label("Star mass (solar masses)");
			ui_state.sun_mass_max = ui_state.sun_mass_max.max(ui_previus_state.sun_mass_min);
			ui_state.sun_mass_min = ui_state.sun_mass_min.min(ui_previus_state.sun_mass_max);
			ui.horizontal(|ui| {
				ui.add(egui::Slider::new(&mut ui_state.sun_mass_min, 0.179..=31.0).text("min"));
				if ui.button("R").clicked() {
					ui_state.sun_mass_min = UiState::default().sun_mass_min;
				}
			});
			ui.horizontal(|ui| {
				ui.add(egui::Slider::new(&mut ui_state.sun_mass_max, 0.179..=31.0).text("max"));
				if ui.button("R").clicked() {
					ui_state.sun_mass_max = UiState::default().sun_mass_max;
				}
			});
			ui.label(format!("Mass {:.2}", world.sun_mass));
			ui.label(format!(
				"Temperature {:.0}K",
				crate::units::calculations::stars::temperature(world.sun_mass)
			))
		});
	if *ui_previus_state != *ui_state {
		generate(ui_state.as_mut(), world.as_mut());
	}
	*ui_previus_state = ui_state.clone();
}

pub fn generate(ui_state: &mut UiState, world: &mut WorldRes) {
	println!("A");
	let Ok(seed) = ui_state.global_seed_str.parse() else {
		ui_state.invalid_seed = true;
		return;
	};
	ui_state.invalid_seed = false;
	world.seed_global = seed;
	let mut rng = rand::rngs::StdRng::seed_from_u64(world.seed_global);
	if ui_state.sun_mass_max >= ui_state.sun_mass_min {
		world.sun_mass = rng.gen_range(ui_state.sun_mass_min..=ui_state.sun_mass_max); // TODO star distribution
	}
}
