## SDL2 Setup
We use environment variables to tell rust where the sdl2 libraries are. We use this to implement consistent setup and both macOS and Windows. Below are how to install the sdl2 libraries and set up the required environment variables. The varibles we need are: `SDL2_LIB_PATH`, `SDL2_IMAGE_LIB_PATH` and `SDL2_TTF_LIB_PATH`.

### SDL2 for macOS
This guide walks you through installing SDL2 and its extensions (SDL2_image, SDL2_ttf) via Homebrew, and configuring your environment so your Rust project can build and link properly.

#### Step 1: Install SDL2 Libraries
Open a terminal and run:
```bash
brew install sdl2 sdl2_image sdl2_ttf
```

You can verify the installation with:
```bash
brew --prefix sdl2
brew --prefix sdl2_image
brew --prefix sdl2_ttf
```

You should see:
```bash
/opt/homebrew/opt/sdl2
/opt/homebrew/opt/sdl2_image
/opt/homebrew/opt/sdl2_ttf
```

#### Step 2: Set Environment Variables
These variables tell Rust where to find the native libraries for linking.

Temporary (for current terminal session):
```bash
export SDL2_LIB_PATH=/opt/homebrew/opt/sdl2/lib
export SDL2_IMAGE_LIB_PATH=/opt/homebrew/opt/sdl2_image/lib
export SDL2_TTF_LIB_PATH=/opt/homebrew/opt/sdl2_ttf/lib
```

Permanent (for all sessions):

Add the following lines to your shell config file (e.g. ~/.zshrc or ~/.bashrc):
```bash
# SDL2 library paths
export SDL2_LIB_PATH=/opt/homebrew/opt/sdl2/lib
export SDL2_IMAGE_LIB_PATH=/opt/homebrew/opt/sdl2_image/lib
export SDL2_TTF_LIB_PATH=/opt/homebrew/opt/sdl2_ttf/lib
```

Then apply changes:
```bash
source ~/.zshrc   # or source ~/.bashrc
```

### SDL2 for Windows
#### Step 1: Install SDL2 Libraries
Install vcpkg and set it up (or other ways you prefer):
```powershell
vcpkg install sdl2 sdl2-image sdl2-ttf
```

#### Step 2: Set Environment Variables
Set environment variables (PowerShell):
```powershell
$env:SDL2_LIB_PATH="C:\path\to\vcpkg\installed\x64-windows\lib"
$env:SDL2_IMAGE_LIB_PATH="C:\path\to\vcpkg\installed\x64-windows\lib"
$env:SDL2_TTF_LIB_PATH="C:\path\to\vcpkg\installed\x64-windows\lib"
```
Make sure to replace `C:\path\to\vcpkg` with your actual vcpkg installation path.
