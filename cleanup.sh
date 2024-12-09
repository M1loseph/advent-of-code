#!/bin/bash
set -e


cleanup_rust() {
    for directory in $(find . -path '*/rust/target')
    do
        rm -r "$directory"
    done
}

cleanup_rust
