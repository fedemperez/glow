# Glow: A Minecraft server

## About:

Glow is a creative mode server for Minecraft, compatible with the 1.16.5 Java version. It's a learning project and not intended for real world usage.

## Features:
* Block breaking and placement
* Random ticks, block updates
* Ability to see other players
* Superflat world generation
* Saving of player and world data
* Compatibility with vanilla savefiles (make a copy, read section *Loading existing worlds*)

## Compiling:
* Download the [rust toolchain](https://www.rust-lang.org/learn/get-started)
* Open a terminal in the project's folder (the one containing Cargo.toml)
* Run the following command:
```bash
cargo build --release
```
* You'll find an executable inside target/release, along other files such as glow.d and .cargo-lock. You only need the executable

## Running:
* Place the executable in a separate folder, as it will create some files inside it
* Open up a terminal in that folder and type `./glow`
* To save all files and stop press `Ctrl + C`

## Loading existing worlds:
* [Locate your .minecraft folder](https://minecraft.fandom.com/wiki/.minecraft)
* Go to saves/<world_name>/region
* Copy every file there to the world/region folder created by Glow. If there were files there before, move or delete them
* Make sure you've copied your files. Once they're opened by Glow they will not be readable to Minecraft again
* You'll likely spawn inside a block, break your way to the top

## Configuration:
You can change some setting in the `config.yml` file generated when running the server. These changes will be applied after a restart
