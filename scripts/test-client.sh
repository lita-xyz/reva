#!/usr/bin/env sh

scriptsdir=$(dirname "$0")
topdir=$(dirname "$scriptsdir")

source "$scriptsdir/BLOCKS.sh"

cd $topdir/bin/client-eth

failed=0
for i in ${!blocks[@]}; do
    cargo +valida run --release -- < ../host/input/1/${blocks[i]}.bin | grep -q "${hashes[i]}";
    if [ $? -ne 0 ]; then
        echo "block ${blocks[i]} failed"
        failed=1
    fi
done

if [ $failed -eq 0 ]; then
    echo "All blocks succeeded"
else
    echo "One or more blocks failed"
fi
exit $failed
