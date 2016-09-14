#!/bin/bash -e

DIR=$(dirname $0)

cd ${DIR}
# DEBUG="y"

rm -f core
ulimit -c unlimited
if test -z "$DEBUG"; then
    cargo build --release

    set -x
    LD_LIBRARY_PATH=./target/release/deps/ ./target/release/test-tag  &
    set +x
else
    cargo build

    set -x
    LD_LIBRARY_PATH=./target/debug/deps/ ./target/debug/test-tag  &
    set +x
fi

CHILD_PID=$!

sleep 1 
kill -6  ${CHILD_PID}
wait

## wait until core file has been written
sleep 1

if ! cat core | strings |grep BUILD_; then
    echo "Failed: releasetags with prefix'BUILD_' not present in core file"
    exit 1
else
    echo "Success: releasetags found in file 'core'"
    exit 0
fi
