#[cfg(target_os = "windows")]
fn build_rc_file() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("res/icon.ico");
    res.compile()
        .expect("Error while compiling rc file informations.");
}

fn main() {
    #[cfg(target_os = "windows")]
    build_rc_file();

    let config = slint_build::CompilerConfiguration::default().with_bundled_translations("lang/");
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
