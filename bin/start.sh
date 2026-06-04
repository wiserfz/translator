#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Translator
# @raycast.mode fullOutput

# Optional parameters:
# @raycast.icon 󰊿
# @raycast.argument1 { "type": "text", "placeholder": "Placeholder" }

# Documentation:
# @raycast.description This script provides quick access to Google's Translate service.

export PATH="$HOME/.local/bin:$PATH"

tror -p "127.0.0.1:10800" "$1"
