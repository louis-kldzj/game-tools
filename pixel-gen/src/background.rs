use crate::*;

#[derive(Event)]
pub struct SpawnBackgroundEvent;

pub fn setup(mut writer: EventWriter<SpawnBackgroundEvent>) {
    writer.send(SpawnBackgroundEvent);
}

pub fn spawn(
    mut reader: EventReader<SpawnBackgroundEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Query<&Window>,
    mut options: ResMut<config::Options>,
) {
    let Some(_) = reader.read().next() else {
        return;
    };
    reader.clear();

    let Ok(window) = window.get_single() else {
        return;
    };

    let size = window.size();
    options.screen_size.set(size);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
        transform: Transform::from_xyz(0., 0.0, -1.0),
        material: materials.add(ColorMaterial::from_color(
            utils::colors::hex_to_color(options.colorscheme.colors().first().unwrap())
                .with_alpha(if options.transparency { 0.0 } else { 1.0 }),
        )),
        ..default()
    });
}
