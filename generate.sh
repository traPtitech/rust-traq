#!/bin/bash

mv README.md README.tmp.md

SRC_URL='https://raw.githubusercontent.com/traPtitech/traQ/master/docs/v3-api.yaml'
java -jar openapi-generator-cli.jar generate \
    -i "${SRC_URL}" \
    -g rust

# OpenAPI Generator 7.27.0 duplicates the models prefix for enum form parameters.
sed -i 's/models::models::/models::/g' src/apis/oauth2_api.rs

# Generator 7.27 uses Display for these chrono date-time query parameters.
# The API requires RFC 3339 rather than chrono's space-separated Display format.
sed -i '/req_builder.query.*"\(since\|until\|after\|before\)"/s/param_value\.to_string()/param_value.to_rfc3339()/g' src/apis/*.rs

cat README.md | sed -e 's/docs\///' > docs/README.md
rm README.md
mv README.tmp.md README.md

FILES='.openapi-generator/FILES'
cat "$FILES" | sed -e 's/README/docs\/README/' > "$FILES.tmp"
rm "$FILES"
mv "$FILES.tmp" "$FILES"

cargo fmt
