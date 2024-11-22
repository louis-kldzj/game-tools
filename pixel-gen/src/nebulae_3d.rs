use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle},
};

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<NebulaeMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct NebulaeMaterial {
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

impl Material for NebulaeMaterial {
    fn fragment_shader() -> ShaderRef {
        "nebulae.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "nebulae.vert".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NebulaeMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
