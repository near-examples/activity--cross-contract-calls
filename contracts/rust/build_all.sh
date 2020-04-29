#!/bin/bash
set -e

TARGET=../../../out/

for d in */ ; do
    echo $d
    echo "Building $d"
    pushd $d
    ./build.sh $TARGET
    popd
done

