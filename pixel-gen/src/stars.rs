use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::{options::Options, ScreenSize};

#[derive(Event)]
pub struct SpawnBigStarEvent;

#[derive(Component)]
pub struct Star;

pub fn spawn_star(
    mut events: EventReader<SpawnBigStarEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<BigStarMaterial>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<Entity, With<Star>>,
    assets: Res<AssetServer>,
    screen_size: Res<ScreenSize>,
    options: Res<Options>,
) {
    if events.is_empty() {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }

    let mut rng = rand::thread_rng();

    for _ in events.read() {
        let star = assets.load("stars-special.png");
        let color_gradiant = images.add(options.colorscheme.gradient_image_with_bg().0);
        let position = screen_size.random_postion(1.5);
        let index = rng.gen_range(0..6);

        let mesh = MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::splat(24. * 2.)))
                .into(),
            material: mats.add(BigStarMaterial::new(star, color_gradiant, position, index)),
            ..default()
        };

        commands.spawn((mesh, Star));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BigStarMaterial {
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[uniform(3)]
    position: Vec3,

    #[texture(4)]
    #[sampler(5)]
    image: Option<Handle<Image>>,
    #[uniform(6)]
    star_type: i32,
}

impl BigStarMaterial {
    fn new(
        star: Handle<Image>,
        color_gradiant: Handle<Image>,
        position: Vec3,
        star_type: i32,
    ) -> Self {
        BigStarMaterial {
            color_texture: Some(color_gradiant),
            position,
            image: Some(star),
            star_type,
        }
    }
}

impl Material2d for BigStarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/output/big_star.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/output/big_star.vert".into()
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
