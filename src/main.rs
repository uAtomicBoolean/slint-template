// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{Model, ModelRc, VecModel};
use std::error::Error;
use std::rc::Rc;

use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let backend = ui.global::<Backend>();
    let file_paths: Rc<VecModel<SharedString>> = Rc::new(backend.get_file_paths().iter().collect());
    backend.set_file_paths(ModelRc::new(file_paths.clone()));

    backend.on_add_file_path(move || {
        for k in 1..1000 {
            file_paths.push(SharedString::from(format!("new path {k}")));
        }
    });

    ui.run()?;

    Ok(())
}
