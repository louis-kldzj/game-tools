use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};

pub fn spawn_nebulae(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NebulaeMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(NebulaeMaterial {
            size: 5.0,
            octaves: 3,
            seed: 4.507,
            pixels: 500.0,
            background_color: Vec4::new(0.0901, 0.0901, 0.0666, 1.0),
            uv_correct: Vec2::new(1.0, 1.0),
            color_texture: Some(asset_server.load("background.png")),
        }),
        ..default()
    });
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NebulaeMaterial {
    #[uniform(0)]
    size: f32,
    #[uniform(3)]
    octaves: i32,
    #[uniform(4)]
    seed: f32,
    #[uniform(5)]
    pixels: f32,
    #[uniform(6)]
    background_color: Vec4,
    #[uniform(7)]
    uv_correct: Vec2,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for NebulaeMaterial {
    fn fragment_shader() -> ShaderRef {
        "nebulae.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "nebulae.vert".into()
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
