use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use rand::Rng;
use shaders::{AnimatedMaterial2D, AnimatedMaterialConfig};

use crate::*;

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
    options: Res<config::Options>,
    current_nebulae: Query<Entity, With<Nebulae>>,
    mut asset_server: ResMut<Assets<Image>>,
    screen_size: Res<config::ScreenSize>,
    mut animation_config: ResMut<NebulaeConfig>,
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

    let mat = NebulaeMaterial::new(
        &options,
        &mut asset_server,
        screen_size.x_offset(),
        &screen_size,
    );

    animation_config.start(mat.get(), mat.get() + 1.);

    commands.spawn((
        Nebulae,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::splat(screen_size.space.height)))
                .into(),
            material: materials.add(mat),
            ..default()
        },
    ));
}

#[derive(Resource)]
pub struct NebulaeConfig {
    default: shaders::DefaultAnimationConfig,
}

impl NebulaeConfig {
    pub fn new() -> Self {
        NebulaeConfig {
            default: shaders::DefaultAnimationConfig::default(),
        }
    }
}

// TODO: This could be a derive macro
impl shaders::AnimatedMaterialConfig for NebulaeConfig {
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
        self.default.change_direction();
    }

    fn cycle(&self) -> bool {
        self.default.cycle()
    }

    fn speed(&self) -> f32 {
        self.default.speed()
    }
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

impl shaders::AnimatedMaterial2D for NebulaeMaterial {
    fn get(&self) -> f32 {
        self.size
    }

    fn update(&mut self, new_value: f32) {
        self.size = new_value
    }
}

impl NebulaeMaterial {
    fn new(
        options: &config::Options,
        asset_server: &mut Assets<Image>,
        x_offset: f32,
        screen_size: &config::ScreenSize,
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
