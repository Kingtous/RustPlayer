#[cfg(target_os = "macos")]
fn emit_minimum_version() {
    println!("set minimum version to 10.15");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15");
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
