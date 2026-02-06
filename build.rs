fn main() {
    // Ask pkg-config for raylib and emit the right cargo:rustc-link-* lines.
    pkg_config::Config::new()
        .atleast_version("5.0")
        .probe("raylib")
        .expect("raylib not found via pkg-config!");
}
