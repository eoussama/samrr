#!/bin/bash

source .env

tag=$GLOBAL_VERSION
project="samrr"
username="eoussama"
image="$username/$project:$tag"

docker build -f ./docker/Dockerfile -t $image .
docker run -it --rm \
  -v "$(pwd)":/samrr \
  $image