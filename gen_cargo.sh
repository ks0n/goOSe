#!/bin/sh

set -e

BOARDS_DIR='boards'

usage() {
    echo "Available boards:"
    for board in "$BOARDS_DIR"/*.conf
    do
        board="${board##"$BOARDS_DIR"/}" # Remove prefix
        board="${board%%.conf}" # Remove suffix
        echo -e "\t$board"
    done

    echo "Usage: $0 <board>"
}

if [ $# -ne 1 ]; then
    usage
    exit 1
fi

board="$1"
board_conf="$BOARDS_DIR/$board.conf"
board_linker_script="$BOARDS_DIR/$board.ld"

if [ ! -e "$board_conf" ]; then
    echo "Cannot find board: '$board' (looked for configuration file: $board_conf)"
    exit 1
fi

mkdir -p .cargo
ln -srf "$board_conf" .cargo/config
ln -srf "$board_linker_script" memory.x
