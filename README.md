# slint-template
A simple template for a rust project using the Slint GUI library.  

## Development
### Environment
You must set the following environment variable (at least for windows) to avoid badly rendered fonts (should be fixed in future slint versions) :
```sh
# Windows
$env:SLINT_BACKEND='winit-software';

# Unix
export SLINT_BACKEND='winit-software'
```  

### UI folder structure
**`assets`:** contains UI specific assets such as icons or fonts.  
**`components`:** contains UI components such as a sidebar, a dropzone, etc. Components are built with widgets.  
**`pages`:** contains the differents pages of the software. A page is built with components and widgets.  
**`widgets`:** contains widgets used in the components and pages. An example of a widget would be a button, or a custom input.  

### UI main files
**`app-window.slint`:**  configure the main window of the software and contains its main layout.  
**`theme`:**  contains all the theme configuration for the software such as the colors used.