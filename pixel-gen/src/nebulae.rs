use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use crate::{options::Options, ScreenSize};

#[derive(Event)]
pub struct SpawnNebulaeEvent;

#[derive(Component)]
pub struct Nebulae;

pub fn setup(mut event_writer: EventWriter<SpawnNebulaeEvent>) {
    event_writer.send(SpawnNebulaeEvent);
}

const ANIMATION_FACTOR: f32 = 0.001;
const ANIMATION_RANGE: f32 = 10.;

pub fn animate_shader(
    time: Res<Time>,
    options: Res<Options>,
    mut timer: ResMut<crate::Timer>,
    mut material: Query<&mut Handle<NebulaeMaterial>, With<Nebulae>>,
    mut neb: ResMut<Assets<NebulaeMaterial>>,
) {
    if !options.animate {
        return;
    }

    timer.0 += time.delta_seconds();

    let Ok(mesh) = material.get_single_mut() else {
        return;
    };

    let mat = neb.get_mut(mesh.into_inner().id()).unwrap();

    if timer.0 >= timer.1 {
        timer.0 = 0.;
        timer.2 = mat.size
            + if timer.3 {
                ANIMATION_RANGE
            } else {
                -ANIMATION_RANGE
            };
        timer.3 = !timer.3;
    }

    mat.size = mat
        .size
        .lerp(timer.2, time.delta_seconds() * ANIMATION_FACTOR);
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
                .add(Rectangle::from_size(Vec2::splat(screen_size.space.height)))
                .into(),
            material: materials.add(NebulaeMaterial::new(
                &options,
                &mut asset_server,
                screen_size.x_offset(),
                &screen_size,
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
    #[uniform(11)]
    time: f32,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl NebulaeMaterial {
    fn new(
        options: &Options,
        asset_server: &mut Assets<Image>,
        x_offset: f32,
        screen_size: &ScreenSize,
    ) -> Self {
        let (image, bg) = options.colorscheme.gradient_image_with_bg();
        let mut rng = rand::thread_rng();

        NebulaeMaterial {
            size: (screen_size.space.height / options.pixels),
            octaves: rng.gen_range(3..5),
            seed: rng.gen_range(1.0..50.0),
            pixels: options.pixels,
            background_color: bg.to_srgba().to_vec4(),
            uv_correct: Vec2::new(1., 1.),
            color_texture: Some(asset_server.add(image)),
            should_tile: options.tile as i32,
            time: 0.,
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
