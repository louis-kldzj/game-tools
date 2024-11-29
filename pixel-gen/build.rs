fn main() {
    println!("{:?}", std::env::current_dir().unwrap());

    std::fs::copy(
        "assets/shaders/nebulae.glsl",
        "assets/shaders/output/nebulae.frag",
    )
    .unwrap();
    std::fs::copy(
        "assets/shaders/nebulae.vert.glsl",
        "assets/shaders/output/nebulae.vert",
    )
    .unwrap();
    std::fs::copy(
        "assets/shaders/star_stuff.glsl",
        "assets/shaders/output/star_stuff.frag",
    )
    .unwrap();
    std::fs::copy(
        "assets/shaders/star_stuff.vert.glsl",
        "assets/shaders/output/star_stuff.vert",
    )
    .unwrap();
    std::fs::copy(
        "assets/shaders/planets.glsl",
        "assets/shaders/output/planets.frag",
    )
    .unwrap();
    std::fs::copy(
        "assets/shaders/planets.vert.glsl",
        "assets/shaders/output/planets.vert",
    )
    .unwrap();
}
