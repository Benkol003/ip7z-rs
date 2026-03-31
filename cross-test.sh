#!/bin/bash
set -e
targets=(\
  "aarch64-unknown-linux-gnu" \
  "arm-unknown-linux-gnueabi" \
  "armv5te-unknown-linux-gnueabi" \
  "armv7-unknown-linux-gnueabi" \
  "i686-unknown-linux-gnu" \
  "x86_64-unknown-linux-gnu"

  #"x86_64-unknown-freebsd"
  #mingw
  #"i686-pc-windows-gnu" \
  #"x86_64-pc-windows-gnu" \
)

for target in "${targets[@]}"; do
  echo testing target "$target"
  cross test --target $target -p ip7z
done 
