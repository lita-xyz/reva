#!/usr/bin/env sh

HOSTTRIPLE=x86_64-unknown-linux-gnu

rpcurl=$1

scriptsdir=$(dirname "$0")
topdir=$(dirname "$scriptsdir")

source "$scriptsdir/BLOCKS.sh"

cd $topdir/bin/host

for i in $blocks; do
    cargo +valida run --release --target=$HOSTTRIPLE -- --rpc-url $rpcurl --chain-id 1 --block-number $i;
done
