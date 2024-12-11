use rand::Rng;

use crate::*;

pub fn controls(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut refresh_event: EventReader<RefreshAllEvent>,
    mut spawn_nebulae: EventWriter<nebulae::SpawnNebulaeEvent>,
    mut spawn_star_stuff: EventWriter<star_stuff::SpawnStarStuffEvent>,
    mut spawn_planets: EventWriter<planets::SpawnPlanetsEvent>,
    mut spawn_bg: EventWriter<background::SpawnBackgroundEvent>,
    mut spawn_big_star: EventWriter<stars::SpawnBigStarEvent>,
) {
    if !kb_input.just_released(KeyCode::Space) {
        let Some(_) = refresh_event.read().next() else {
            return;
        };
        refresh_event.clear();
    }

    spawn_nebulae.send(nebulae::SpawnNebulaeEvent);
    spawn_star_stuff.send(star_stuff::SpawnStarStuffEvent);
    for _ in 0..=rand::thread_rng().gen_range(1..4) {
        spawn_planets.send(planets::SpawnPlanetsEvent);
    }
    spawn_bg.send(background::SpawnBackgroundEvent);
    for _ in 0..=rand::thread_rng().gen_range(10..100) {
        spawn_big_star.send(stars::SpawnBigStarEvent);
    }
}
