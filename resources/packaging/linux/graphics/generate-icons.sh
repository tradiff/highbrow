#!/bin/bash

for icon in 16x16 22x22 24x24 32x32 36x36 48x48 64x64 72x72 96x96 128x128 192x192 256x256 512x512; do
    magick -background none highbrow.svg -resize ${icon} highbrow-${icon}.png
done
