#[cfg(target_os = "macos")]
fn emit_minimum_version() {
    println!("set minimum version to 12.0");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.0");
}

fn configure() {
    #[cfg(target_os = "macos")]
    {
        emit_minimum_version();
    }
}

fn main() {
    println!("hello from RustPlayer!");
    configure();
}
