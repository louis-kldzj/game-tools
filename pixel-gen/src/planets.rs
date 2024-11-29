use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::{menu::Options, ScreenSize};

#[derive(Event)]
pub struct SpawnPlanetsEvent;

#[derive(Component)]
pub struct Planets {
    scale: f32,
}

pub fn update_scale(
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Planets>,
) {
    let delta = if kb_input.pressed(KeyCode::ArrowLeft) {
        -1.0
    } else if kb_input.pressed(KeyCode::ArrowRight) {
        1.0
    } else {
        return;
    };

    let Ok(mut planets) = query.get_single_mut() else {
        return;
    };

    planets.scale += delta * time.delta_seconds();
}

pub fn lerp_scale(time: Res<Time>, mut query: Query<(&mut Transform, &Planets)>) {
    let Ok((mut transform, planets)) = query.get_single_mut() else {
        return;
    };

    const SPEED: f32 = 1.;

    transform.scale = transform
        .scale
        .lerp(Vec3::splat(planets.scale), time.delta_seconds() * SPEED)
}

pub fn setup(mut event_writer: EventWriter<SpawnPlanetsEvent>) {
    event_writer.send(SpawnPlanetsEvent);
}

pub fn spawn_planets(
    mut trigger: EventReader<SpawnPlanetsEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetsMaterial>>,
    options: Res<Options>,
    current_planet: Query<Entity, With<Planets>>,
    mut images: ResMut<Assets<Image>>,
    screen_size: Res<ScreenSize>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = current_planet.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    if !options.planets {
        return;
    }

    commands.spawn((
        Planets { scale: 1. },
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.)).into(),
            material: materials.add(PlanetsMaterial::new(&options, &mut images, &screen_size)),
            transform: Transform::from_translation(Vec3::ZERO.with_z(1.0)),
            ..default()
        },
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetsMaterial {
    #[uniform(0)]
    size: f32,
    #[uniform(1)]
    octaves: i32,
    #[uniform(2)]
    seed: f32,
    #[uniform(3)]
    pixels: f32,
    #[uniform(4)]
    light_origin: Vec2,

    #[texture(5)]
    #[sampler(6)]
    color_texture: Option<Handle<Image>>,
    #[uniform(7)]
    position: Vec3,
}

impl PlanetsMaterial {
    fn new(options: &Options, asset_server: &mut Assets<Image>, screen_size: &ScreenSize) -> Self {
        let half_square = screen_size.0.y / 2.;
        let x_offset = screen_size.x_offset();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range((x_offset - half_square)..(x_offset + half_square));
        let y = rng.gen_range(-half_square..half_square);
        PlanetsMaterial {
            size: 1.,
            octaves: 3,
            seed: rng.gen_range(1.0..10.0),
            pixels: 100.0,
            light_origin: Vec2::new(rand::random(), rand::random()),
            color_texture: Some(asset_server.add(options.colorscheme.gradient_image_with_bg().0)),
            position: Vec3::new(x, y, 2.0),
        }
    }
}

impl Material2d for PlanetsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/output/planets.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/output/planets.vert".into()
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
