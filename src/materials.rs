use bevy::{prelude::Material, reflect::TypeUuid, render::render_resource::AsBindGroup};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "7426e12f-7bdd-4beb-8b46-dc5c2d8a7a3d"]
pub struct Sun {}

impl Material for Sun {
	fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
		"shaders/sun.wgsl".into()
	}
}
