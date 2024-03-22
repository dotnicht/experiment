#!/bin/bash

tag="dotnicht/experiment"
DOCKER_BUILDKIT=1 docker build . -t $tag
docker push $tag
