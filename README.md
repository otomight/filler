# Quick start

1. Download the binaries [here](https://assets.01-edu.org/filler/filler.zip).
2. Move the robots and the game engine at the root of the project.
3. Give execution rights to the binaries
```sh
chmod +x linux_game_engine linux_robots/*
```
4. Run
```sh
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator
```
You should see the score of the two players, with terminator as winner.
5. Try your solution
```sh
cargo build --release # don't forget to compile your solution
./linux_game_engine -f maps/map01 -p1 target/release/filler -p2 linux_robots/bender
```

# Setup docker

- To build the image `docker build -t filler .`
- To run the container `docker run --rm -v "$(pwd)"/target:/filler/target -it filler`. This instruction will open a terminal in the container, the directory `target` will be mounted in the container as well. Use `:Z` at the end of the volume path to give rights on Fedora.
