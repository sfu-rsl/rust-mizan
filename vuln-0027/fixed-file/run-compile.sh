#!/usr/bin/env bash

# output minimize-fmt-cargo to stdout to indicate that cargo lints will be output to stdout
# this is the default
# echo "minimize-fmt-cargo"

# The script needs to emit the output here for lints
cargo build --message-format=json
if [ $? != 0 ];
then
    # echo "No reproduction"
    exit 1
else
    exit 0
fi
