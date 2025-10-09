#!/usr/bin/env bash
set -e

PLAYER_PATH="target/release/filler"

# List of (map, opponent) pairs
declare -a TEST_PAIRS=(
	"maps/map00 linux_robots/wall_e"
	"maps/map01 linux_robots/h2_d2"
	"maps/map02 linux_robots/bender"
)

echo "=== Starting automated filler tests ==="
echo

for PAIR in "${TEST_PAIRS[@]}"; do
	read -r MAP OPPONENT <<< "$PAIR"
	echo "--- Testing on $MAP vs $OPPONENT ---"

	for i in {1..5}; do
		if (( i % 2 == 1 )); then
			CMD="./linux_game_engine -f $MAP -p1 $PLAYER_PATH -p2 $OPPONENT"
		else
			CMD="./linux_game_engine -f $MAP -p1 $OPPONENT -p2 $PLAYER_PATH"
		fi

		echo "[$i] Running: $CMD"
		$CMD | tail -n 3
		echo
	done
	echo
done
