use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::menu::Options;

#[derive(Event)]
pub struct SpawnStarStuffEvent;

#[derive(Component)]
pub struct StarStuff;

pub fn setup(mut trigger: EventWriter<SpawnStarStuffEvent>) {
    trigger.send(SpawnStarStuffEvent);
}

pub fn spawn_star_stuff(
    mut trigger: EventReader<SpawnStarStuffEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarStuffMaterial>>,
    options: Res<Options>,
    current_nebulae: Query<Entity, With<StarStuff>>,
    mut asset_server: ResMut<Assets<Image>>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = current_nebulae.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    if !options.dust {
        return;
    }

    commands.spawn((
        StarStuff,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(StarStuffMaterial::new(&options, &mut asset_server)),
            ..default()
        },
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarStuffMaterial {
    #[uniform(0)]
    size: f32,
    #[uniform(3)]
    octaves: i32,
    #[uniform(4)]
    seed: f32,
    #[uniform(5)]
    pixels: f32,
    #[uniform(6)]
    uv_correct: Vec2,
    #[uniform(7)]
    should_tile: i32,
    #[uniform(8)]
    reduce_background: i32,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl StarStuffMaterial {
    fn new(options: &Options, asset_server: &mut Assets<Image>) -> Self {
        StarStuffMaterial {
            size: 5.0,
            octaves: 3,
            seed: rand::random::<f32>() % 10.,
            pixels: options.pixels,
            uv_correct: Vec2::new(1.0, 1.0),
            color_texture: Some(asset_server.add(options.colorscheme.gradient_image_with_bg().0)),
            should_tile: options.tile as i32,
            reduce_background: options.darken as i32,
        }
    }
}

impl Material2d for StarStuffMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/output/star_stuff.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/output/star_stuff.vert".into()
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
