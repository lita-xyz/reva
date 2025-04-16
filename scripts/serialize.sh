#!/usr/bin/bash

rpcurl=$1

scriptsdir=$(dirname $(realpath "$0"))
topdir=$(dirname "$scriptsdir")

source "$scriptsdir/BLOCKS.sh"

cd $topdir/bin/host

for i in "${blocks[@]}"; do
    echo "Serializing block: $i"
    cargo run --release -- --rpc-url $rpcurl --chain-id 1 --block-number $i;
done
