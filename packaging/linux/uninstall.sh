#!/usr/bin/env bash
set -euo pipefail

if [ "$(id -u)" -ne 0 ] && [ -f "/usr/local/bin/lmd" ] && command -v sudo >/dev/null 2>&1; then
    exec sudo bash "$0" "$@"
fi

rm -f /usr/local/bin/lmd \
      /usr/local/share/applications/lmd.desktop \
      /usr/local/share/icons/hicolor/256x256/apps/lmd.png \
      "$HOME/.local/bin/lmd" \
      "$HOME/.local/share/applications/lmd.desktop" \
      "$HOME/.local/share/icons/hicolor/256x256/apps/lmd.png"

echo "Đã gỡ cài đặt LMD."
