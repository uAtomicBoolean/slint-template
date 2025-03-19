# slint-template
A Slint template for Rust that comes with translations, bases for winresources and a github action to bundle the project into installers.  

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

## UI folder
The UI folder contains some subfolders to organize the project (nothing in set in stone) :
- `assets/` : the fonts, images and other assets of the UI.
- `components/` : the UI components such as a sidebar. Components are built with widgets.
- `pages/` : the different pages of the software.
- `theme/` : the themes used in the software, can be widget's theme, windows themes, global themes, etc.
- `widgets/` : the custom widgets used in the UI such as a button, a link, a select menu, etc.
- `windows/` : other windows that could be opened apart from the main window.
- `app-window.slint` : the main UI file loaded by Rust.

## Github action
- The github action will run when a new tag is pushed to github.  
- This will build the project and bundle it into installers for linux, macos arm and windows using `cargo-packager`.
- A release will be created with the installers, the standalone executable resulting from the build.
- The release will have the project's name and the tag as a name.