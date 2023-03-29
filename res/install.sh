#!/usr/bin/env bash


mkdir -p "$HOME"/.local/bin/ && cp ./qtm2 "$HOME"/.local/bin/qtm2
mkdir -p "$HOME"/.local/share/applications/ && cp ./qtm2.desktop "$HOME"/.local/share/applications/qtm2.desktop
mkdir -p "$HOME"/.local/share/icons/ && cp ./qtm2.svg "$HOME"/.local/share/icons/qtm2.svg