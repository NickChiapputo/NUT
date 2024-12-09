# Truck Sim
The library in this directory is a plugin for American Truck Simulator and European Truck Simulator 2.

## Build
Run `make nut.so` to build a linux shared object.

Run `make nut.dll` to cross-compile a windows DLL. I haven't tested this in conjunction with anything else in this repository, so don't consider this a complete feature (not that anything else is).

## Deploy
Either symlink or copy the dynamic library created in the build step to your Steam ATS/ETS2 bin directory: `<steam_root>/steamapps/common/Euro Truck Simulator 2/bin/<os>/plugins/`.
    - `steam_root` is your Steam install (typically `~/.local/share/Steam/` but can be other places if you have secondary drives for such things)
    - `os` is either `linux_x64` or `win_x64` (unless you're crazy and are on an x86 -- I don't know if this works!)

## Webserver
A basic Python webserver is found in the `python` subdirectory. This will display speed (kph) and rpm from the plugin. It requires a glue dynamic library to make use of the shared memory in the `nut.so` library. This is because Python will free that shared memory when it stops running. This is dumb and it would be nice to not have to restart the simulator if the script is restarted. So we use a second library (built with just `make` in the `python` directory) and then linked with the Python script to access the shared memory.

## C
The `c` directory just serves as an reference of how to access the shared memory resources from C. It currently plays no part in this project.