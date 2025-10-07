FROM rust:buster

WORKDIR /filler/

COPY ./maps					/filler/maps
COPY ./linux_robots			/filler/linux_robots
COPY ./linux_game_engine	/filler/linux_game_engine

ENTRYPOINT ["/bin/bash"]
