# Mods
Mods require a parent directory and a structure like this:
```
mods/
    modname/
        assets/
            textures/
                ...
            images/
                ...
            sounds/
                ...
            ...
        data/
            example.toml
            ...
        mod.toml
```
mod.toml is a necessary file detailing name, version, required version, and a checksum value. The checksum value is used to verify the mod's integrity. The logic directory is where the mod's logic is stored. The assets directory is where the mod's assets are stored. The assets directory is optional. The logic directory is optional. The mod.toml file is required.