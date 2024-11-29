use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::{options::Options, ScreenSize};

#[derive(Event)]
pub struct SpawnNebulaeEvent;

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
    options: Res<Options>,
    current_nebulae: Query<Entity, With<Nebulae>>,
    mut asset_server: ResMut<Assets<Image>>,
    screen_size: Res<ScreenSize>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = current_nebulae.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    if !options.nebulae {
        return;
    }

    commands.spawn((
        Nebulae,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::splat(screen_size.0.y)))
                .into(),
            material: materials.add(NebulaeMaterial::new(
                &options,
                &mut asset_server,
                screen_size.x_offset(),
            )),
            ..default()
        },
    ));
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
    #[uniform(8)]
    should_tile: i32,
    #[uniform(9)]
    reduce_background: i32,
    #[uniform(10)]
    x_offset: Vec3,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl NebulaeMaterial {
    fn new(options: &Options, asset_server: &mut Assets<Image>, x_offset: f32) -> Self {
        let (image, bg) = options.colorscheme.gradient_image_with_bg();

        NebulaeMaterial {
            size: 5.0,
            octaves: 3,
            seed: rand::random::<f32>() % 10.,
            pixels: options.pixels,
            background_color: bg.to_srgba().to_vec4(),
            uv_correct: Vec2::new(0.9, 1.6),
            color_texture: Some(asset_server.add(image)),
            should_tile: options.tile as i32,
            reduce_background: options.darken as i32,
            x_offset: Vec3::new(x_offset, 0., 0.),
        }
    }
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
