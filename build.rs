fn main() {
    std::process::Command::new("ship")
        .spawn()
        .expect("Failed to acquire dependencies")
        .wait()
        .expect("Could not determine if depencencies were acquired");
    sixtyfps_build::compile("ui/main.60").unwrap();
}
