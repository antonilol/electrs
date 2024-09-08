#!/bin/bash

out_dir=blocks

mkdir "$out_dir"

for height in {180001..182000}; do
	hash=$(bitcoin-cli getblockhash $height)
	bitcoin-cli getblock $hash 0 | xxd -r -p > "$out_dir"/blk-$height.dat
	echo -ne "\r$height"
done
