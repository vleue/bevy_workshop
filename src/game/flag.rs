use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

pub fn flag_plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<FlagMaterial>::default());
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlagMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub atlas: Handle<Image>,
    #[uniform(2)]
    pub index: Vec4,
    #[uniform(3)]
    pub distance: Vec4,
}

impl Material2d for FlagMaterial {
    fn fragment_shader() -> ShaderRef {
        "flag_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
