#!/bin/sh
if [ $# -eq 0 ]; then
    echo "Error: No arguments provided."
    exit 1  # Exit with an error code
fi

rm -rf tmp-download
mkdir tmp-download || exit 1
cd tmp-download || exit 1
curl -L https://crates.io/api/v1/crates/$1/$2/download -o "$1-$2.crate" 
tar -xvf "$1-$2.crate"
mv $1-$2 ..
cd .. || exit 1
rm -rf tmp-download