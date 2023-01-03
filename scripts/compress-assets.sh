#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "tin-$RELEASE_VERSION-$TARGET.zip" tin-"$RELEASE_VERSION"/
else
  tar -czvf tin-"$RELEASE_VERSION"-"$TARGET".tar.gz tin-"$RELEASE_VERSION"/
  shasum -a 512 tin-"$RELEASE_VERSION"-"$TARGET".tar.gz >tin-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
