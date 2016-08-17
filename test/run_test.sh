#!/bin/bash -e

rm -f core
ulimit -c unlimited
cargo build --release

./target/release/testct  & 
sleep 1 
kill -6  $!
wait

## wait until core file has been written
sleep 1

if ! cat core | strings |grep BUILD_; then
   echo "Failed: releasetags with prefix'BUILD_' not present in core file"
   exit 1
fi
