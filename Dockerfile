FROM rust:bookworm

WORKDIR /filler

COPY linux_robots      /filler/linux_robots
COPY maps              /filler/maps
COPY Cargo.lock        /filler/Cargo.lock
COPY Cargo.toml        /filler/Cargo.toml
COPY linux_game_engine /filler/linux_game_engine
COPY test.sh           /filler/test.sh

ENTRYPOINT ["/bin/bash"]
