#!/bin/bash

for PROJECT in versioneer versioneer-cli
do
    pushd $PROJECT && cargo publish && popd
done
