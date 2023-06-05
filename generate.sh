#!/bin/bash

mv README.md README.tmp.md

SRC_URL='https://raw.githubusercontent.com/traPtitech/traQ/master/docs/v3-api.yaml'
java -jar openapi-generator-cli.jar generate \
    -i "${SRC_URL}" \
    -g rust

cat README.md | sed -e 's/docs\///' > docs/README.md
rm README.md
mv README.tmp.md README.md

FILES='.openapi-generator/FILES'
cat "$FILES" | sed -e 's/README/docs\/README/' > "$FILES.tmp"
rm "$FILES"
mv "$FILES.tmp" "$FILES"

cargo fmt
