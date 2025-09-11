# smart_road

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


I am currently working on a smart_road project. where i need to program a intersection crossing program for AVs. 
Refer to here for details: https://github.com/01-edu/public/blob/master/subjects/smart-road/README.md

The intersection is 4 way with 3 lanes (turn left, turn right, go straight).
Each lane is 50 px wide. A road is 300 px wide (50 x 6) to accomodate for two way traffic.
For simplicity, all cars are 80px long.

I have decided to use a time slot reservation for crossing.
Each car has to maintain a safety distance of 50 px plus the dist base on its current speed.
Right turns will never be obstructed as they never have to worry about incoming traffic.
we set a max speed of 7 px/ frame for all cars.
Only straight and turn left need slot reservation. Thus a 4 x 4 zone...
Cars should accelerate to 7px/ frame as they approach the crossing.
After they enter the crossing, we assume they all travel at 7px/ frame for consistent calculation.

The program is written in rust.
I have:
car.rs (car struct and logic)
cars_id.rs (car_id generation. we use the id as a ref for removing a car in the reservation list)
crossing_manager.rs (creating the zones and reservation logic)
itersection.rs (create the roads and spawning of cars)

i will share the details of each file.
For now, just remember the files and context and wait for my prompt (unless there is something critical)

we need to implement:
resevation logic is faulty
need better speed calculation