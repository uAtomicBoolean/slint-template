# slint-template
A Slint template for Rust that comes with translations, bases for winresources and github actions to bundle the project into installers for linux, macos arm and windows.  

## Clean after using to create a repo
- Update all occurences of `slint-template` to your project's name in the following files :
  - `Cargo.toml`
  - `ui/app-window.slint`
  - all translations filenames
  - `.github/workflows/build.yaml`
- Update the translations metadata.

## Translations
[*Please follow the official documentation.*](https://docs.slint.dev/latest/docs/slint/guide/development/translations/)  
Translations are stored in the `lang/` folder.  

## Winresources
The metadata are configured in the `Cargo.toml` file.  
Resources such as the project's icon are stored in the `res/` folder.  

## Sources folder
The `src/` folder contains three elements:
- `main.rs`: your project's entrypoint.
- `app/`: a folder that contains all the application's logic that is not directly related to the UI such as installing a dependency.  
- `ui/`: a folder that contains the bindings to the UI such as callbacks, creating the windows, etc.

## UI folder
The UI folder contains some subfolders to organize the project (nothing in set in stone) :
- `assets/` : the fonts, images and other assets of the UI.
- `app-window.slint` : the main UI file loaded by Rust.

## Github action
All 4 github actions will run when a new tag (ex: v1.0.0) is pushed.  
They will then create a release with standalone executables and installers for linux, windows and macos intel/arm.  

### Changelogs
Changelogs files are stored in the `changelogs` folder. Each changelog must use the following naming convention: `version_number-changelog.txt`. 
Exemple: `v1.0.0-changelog.txt`  