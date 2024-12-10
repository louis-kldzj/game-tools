use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use rand::Rng;
use shaders::{AnimatedMaterial2D, AnimatedMaterialConfig, DefaultAnimationConfig};

use crate::*;

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
    for _ in 0..rand::thread_rng().gen_range(1..5) {
        event_writer.send(SpawnPlanetsEvent);
    }
}

pub fn spawn_planets(
    mut trigger: EventReader<SpawnPlanetsEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetsMaterial>>,
    options: Res<config::Options>,
    current_planet: Query<Entity, With<Planets>>,
    mut images: ResMut<Assets<Image>>,
    screen_size: Res<config::ScreenSize>,
) {
    if trigger.is_empty() {
        return;
    }

    for entity in current_planet.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if !options.planets {
        return;
    }

    for _ in trigger.read() {
        let mat = PlanetsMaterial::new(&options, &mut images, &screen_size);

        let mut config = PlanetsConfig::new();

        config.start(mat.get(), mat.get() + 10.);

        commands.spawn((
            Planets { scale: 1. },
            config,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Circle::new(rand::thread_rng().gen_range(40.0..70.0)))
                    .into(),
                material: materials.add(mat),
                transform: Transform::from_translation(Vec3::ZERO.with_z(1.0)),
                ..default()
            },
        ));
    }
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

impl AnimatedMaterial2D for PlanetsMaterial {
    fn get(&self) -> f32 {
        self.size
    }

    fn update(&mut self, new_value: f32) {
        self.size = new_value
    }
}

impl PlanetsMaterial {
    fn new(
        options: &config::Options,
        asset_server: &mut Assets<Image>,
        screen_size: &config::ScreenSize,
    ) -> Self {
        PlanetsMaterial {
            size: 5.365,
            octaves: 3,
            seed: rand::thread_rng().gen_range(1.0..10.0),
            pixels: 100.0,
            light_origin: Vec2::new(rand::random(), rand::random()),
            color_texture: Some(asset_server.add(options.colorscheme.gradient_image_with_bg().0)),
            position: screen_size.random_postion(2.0),
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

#[derive(Component)]
pub struct PlanetsConfig {
    default: DefaultAnimationConfig,
}

impl PlanetsConfig {
    fn new() -> Self {
        PlanetsConfig {
            default: DefaultAnimationConfig::default(),
        }
    }
}

impl AnimatedMaterialConfig for PlanetsConfig {
    fn start(&mut self, start: f32, target: f32) {
        self.default.start(start, target);
    }

    fn progress(&self) -> f32 {
        self.default.progress()
    }

    fn update_progress(&mut self, new_progress: f32) {
        self.default.update_progress(new_progress);
    }

    fn target(&self) -> f32 {
        self.default.target()
    }

    fn change_direction(&mut self) {
        self.default.change_direction()
    }

    fn cycle(&self) -> bool {
        self.default.cycle()
    }

    fn speed(&self) -> f32 {
        self.default.speed()
    }
}
