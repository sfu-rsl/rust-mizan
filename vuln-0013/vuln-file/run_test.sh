#!/usr/bin/env bash

# output minimize-fmt-cargo to stdout to indicate that cargo lints will be output to stdout
# this is the default
# echo "minimize-fmt-cargo"

TEST_NAME=preserve_vec_file

# The script needs to emit the output here for lints
cargo test --message-format=json --test $TEST_NAME --no-run
if [ $? != 0 ];
then
    # echo "No reproduction"
    exit 1
fi

CARGO_TEST_OUTPUT=$(cargo test --message-format=json --test $TEST_NAME)
CARGO_ERR_CODE=$?
if [ $CARGO_ERR_CODE = 0 ];
then
    exit 0
else
    exit 1
fi
