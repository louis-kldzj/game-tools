use bevy::{
    ecs::system::SystemParam,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use rand::Rng;

use crate::*;

#[derive(Event)]
pub struct SpawnBigStarEvent;

#[derive(Component)]
pub struct Star;

#[derive(SystemParam)]
pub struct StarSpawner<'w, 's> {
    events: EventReader<'w, 's, SpawnBigStarEvent>,
    commands: Commands<'w, 's>,
    meshes: ResMut<'w, Assets<Mesh>>,
    mats: ResMut<'w, Assets<BigStarMaterial>>,
    images: ResMut<'w, Assets<Image>>,
    query: Query<'w, 's, Entity, With<Star>>,
    assets: Res<'w, AssetServer>,
    options: Res<'w, config::Options>,
}

pub fn spawn_star(mut ss: StarSpawner) {
    if ss.events.is_empty() {
        return;
    }
    for entity in ss.query.iter() {
        ss.commands.entity(entity).despawn_recursive()
    }

    let mut rng = rand::thread_rng();

    for _ in ss.events.read() {
        let star = ss.assets.load("stars-special.png");
        let color_gradiant = ss
            .images
            .add(ss.options.colorscheme.gradient_image_with_bg().0);
        let position = ss.options.screen_size.random_postion(1.5);
        let index = rng.gen_range(0..6);

        let mesh = MaterialMesh2dBundle {
            mesh: ss
                .meshes
                .add(Rectangle::from_size(Vec2::splat(24. * 2.)))
                .into(),
            material: ss
                .mats
                .add(BigStarMaterial::new(star, color_gradiant, position, index)),
            ..default()
        };

        ss.commands.spawn((mesh, Star));
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
