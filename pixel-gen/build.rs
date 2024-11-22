fn main() {
    println!("{:?}", std::env::current_dir().unwrap());

    std::fs::copy("assets/nebulae.glsl", "assets/nebulae.frag").unwrap();
    std::fs::copy("assets/nebulae.vert.glsl", "assets/nebulae.vert").unwrap();
}
