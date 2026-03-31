### ip7z
Safe bindings for 7zip.


### Linking
Static or dynamic linking is supported via their feature flags.  
In the case of static linking (the default), 7zip will be built from source, which requires a C++ compiler and make to be installed (or nmake on windows).  

For dynamic linking, on linux or macos install 7zip via your package manager:  
- `sudo apt install p7zip-full`
- `sudo dnf install 7zip`
- `sudo pacman -S 7zip`
- `brew install sevenzip`

Or on windows use the 7zip installer from https://www.7-zip.org/download.html .