use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use rand::Rng;

use crate::{
    shaders::{AnimatedMaterial2D, AnimatedMaterialConfig, DefaultAnimationConfig},
    *,
};

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
    options: Res<config::Options>,
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

    let mat = StarStuffMaterial::new(
        &options,
        &mut asset_server,
        options.screen_size.x_offset(),
        &options.screen_size,
    );

    let mut animation_config = StarStuffConfig::new();

    animation_config.start(mat.get(), mat.get() + 1.);

    commands.spawn((
        StarStuff,
        animation_config,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(
                    options.screen_size.width(),
                    options.screen_size.height(),
                )))
                .into(),
            material: materials.add(mat),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct StarStuffConfig {
    default: DefaultAnimationConfig,
}

impl StarStuffConfig {
    pub fn new() -> Self {
        StarStuffConfig {
            default: DefaultAnimationConfig::default(),
        }
    }
}

impl AnimatedMaterialConfig for StarStuffConfig {
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
    #[uniform(9)]
    position: Vec3,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl AnimatedMaterial2D for StarStuffMaterial {
    fn get(&self) -> f32 {
        self.size
    }

    fn update(&mut self, new_value: f32) {
        self.size = new_value
    }
}

impl StarStuffMaterial {
    fn new(
        options: &config::Options,
        asset_server: &mut Assets<Image>,
        x_offset: f32,
        screen_size: &config::ScreenSize,
    ) -> Self {
        let mut rng = rand::thread_rng();

        StarStuffMaterial {
            size: screen_size.width() / options.pixels,
            octaves: rng.gen_range(3..5),
            seed: rng.gen_range(1.0..50.0),
            pixels: options.pixels,
            uv_correct: options.screen_size.aspect(),
            color_texture: Some(asset_server.add(options.colorscheme.gradient_image_with_bg().0)),
            should_tile: options.tile as i32,
            reduce_background: options.darken as i32,
            position: Vec3::new(x_offset, 0., 0.),
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
