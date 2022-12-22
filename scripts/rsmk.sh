#!/bin/bash

PKG="$1"
DIRNAME="$(basename "$PWD")"
FILENAME="../${DIRNAME}.rs"

if [[ -z "$PKG" ]]; then
  echo "Please specify the pkg name you want to create"
  exit 1
fi

mkdir "${PKG}"
touch "${PKG}.rs"

if [[ "$DIRNAME" == "src" ]]; then
FILENAME="lib.rs"
fi

cat <<EOF >> "$FILENAME"
mod $1;
EOF
