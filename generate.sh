#!/bin/bash

SRC_URL='https://raw.githubusercontent.com/traPtitech/traQ/master/docs/v3-api.yaml'
java -jar openapi-generator-cli.jar generate \
    -i "${SRC_URL}" \
    -g rust

cargo fmt
