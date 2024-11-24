use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{AsBindGroup, Sampler, ShaderRef},
        texture::DefaultImageSampler,
    },
    sprite::{Material2d, MaterialMesh2dBundle},
    utils::RandomState,
};

use crate::BGColorIndex;

#[derive(Event)]
pub struct SpawnNebulaeEvent;

const BACKGROUND_COLORS: [LinearRgba; 5] = [
    LinearRgba::BLUE,
    LinearRgba::BLACK,
    LinearRgba::GREEN,
    LinearRgba::WHITE,
    LinearRgba::RED,
];

#[derive(Component)]
pub struct Nebulae;

pub fn setup(mut event_writer: EventWriter<SpawnNebulaeEvent>) {
    event_writer.send(SpawnNebulaeEvent);
}

pub fn spawn_nebulae(
    mut trigger: EventReader<SpawnNebulaeEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NebulaeMaterial>>,
    mut color_index: ResMut<BGColorIndex>,
    current_nebulae: Query<Entity, With<Nebulae>>,
    asset_server: Res<AssetServer>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = current_nebulae.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    commands.spawn((
        Nebulae,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            //transform: Transform::default().with_scale(Vec3::splat(TEXTURE_SIZE)),
            material: materials.add(NebulaeMaterial {
                size: 5.0,
                octaves: 3,
                seed: rand::random::<f32>() % 10.,
                pixels: 2000.0,
                background_color: Vec4::new(0.0901961, 0.0901961, 0.0666667, 1.),
                uv_correct: Vec2::new(1.0, 1.0),
                color_texture: Some(asset_server.load("background.png")),
                should_tile: 0,
                reduce_background: 0,
            }),
            ..default()
        },
    ));

    color_index.0 = (color_index.0 + 1) % 5;
}

const COLORSCHEME: [Vec4; 8] = [
    Vec4::new(0.12549, 0.133333, 0.0823529, 1.),
    Vec4::new(0.227451, 0.156863, 0.00784314, 1.),
    Vec4::new(0.588235, 0.235294, 0.235294, 1.),
    Vec4::new(0.792157, 0.352941, 0.180392, 1.),
    Vec4::new(1., 0.470588, 0.192157, 1.),
    Vec4::new(0.952941, 0.6, 0.286275, 1.),
    Vec4::new(0.921569, 0.760784, 0.458824, 1.),
    Vec4::new(0.87451, 0.843137, 0.521569, 1.),
];

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
    #[uniform(8)]
    should_tile: i32,
    #[uniform(9)]
    reduce_background: i32,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for NebulaeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/output/nebulae.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/output/nebulae.vert".into()
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
