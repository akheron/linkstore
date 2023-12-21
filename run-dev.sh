#!/bin/sh

# Modify to your liking
export DATABASE_URL=postgresql://linkstore:linkstore@localhost:5432/linkstore
export DATABASE_POOL_SIZE=5
export BIND=127.0.0.1:8000
export COOKIE_SECRET=thuNgaemohm9av9Eexae1xi7gei3vom4uh9yihie6chaShieloor6fohNgaiwees
export AUTH_USERNAME=linkstore
export AUTH_PASSWORD=dev
export ASSET_PATH=assets
export ENV=local

cargo watch -x run
