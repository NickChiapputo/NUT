# Run shm-bridge for ACC
 WINEFSYNC=1 WINEPREFIX=/ssd-backup/SteamLibrary/steamapps/compatdata/805550/pfx "/ssd-backup/SteamLibrary/steamapps/common/Proton 10.0/files/bin/wine" /path/to/NUT/NUT/simracing/shm-bridge/target/x86_64-pc-windows-gnu/release/shm-bridge.exe

# Run shm-bridge for AMS2
 WINEFSYNC=1 WINEPREFIX=/ssd-backup/SteamLibrary/steamapps/compatdata/1066890/pfx "/home/nick/.local/share/Steam/steamapps/common/Proton 9.0 (Beta)/files/bin/wine" /path/to/NUT/simracing/shm-bridge/target/x86_64-pc-windows-gnu/release/shm-bridge.exe

 ## From simhshmbridge
 ./assets/pcars2shm

 %command% & sleep 5 && "/home/nick/.local/share/Steam/steamapps/common/Proton 9.0 (Beta)/proton" run ~/Documents/projects/simshmbridge/assets/pcars2bridge.exe

# Launch Options for AMS2
 %command% & /path/to/NUT/simracing/moza/run.sh & sleep 5 && "/home/nick/.local/share/Steam/steamapps/common/Proton 9.0 (Beta)/proton" run /path/to/NUT/simracing/shm-bridge/target/x86_64-pc-windows-gnu/release/shm-bridge.exe

