use bevy::prelude::*;
use rand::Rng;

use crate::ScreenSize;

#[derive(Event)]
pub struct SpawnBigStarEvent;

#[derive(Component)]
pub struct Star;

pub fn spawn_star(
    mut events: EventReader<SpawnBigStarEvent>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<Star>>,
    assets: Res<AssetServer>,
    screen_size: Res<ScreenSize>,
) {
    if events.is_empty() {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }

    let mut rng = rand::thread_rng();

    for _ in events.read() {
        let layout = layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(24),
            6,
            1,
            None,
            None,
        ));

        let texture = assets.load("stars-special.png");

        let position = screen_size.random_postion(1.5);
        let sprite = SpriteBundle {
            texture,
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(rng.gen_range(1.0..2.0))),
            ..default()
        };

        let atlas = TextureAtlas {
            layout,
            index: rng.gen_range(0..6),
        };

        commands.spawn((sprite, atlas, Star));
    }
}

pub struct BigStarMaterial {}

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
