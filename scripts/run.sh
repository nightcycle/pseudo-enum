#!/usr/bin/env bash
set -e

cargo run -- export --rojo "test/prod-3/default.project.json" --ignore "test/prod-3/ignore.txt" --universe "$TESTING_BLOX_UNIVERSE_ID" --place "$TB_PROD_3_PLACE_ID" --key "$INSTANCE_READ_API_KEY"
