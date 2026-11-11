#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ "$(id -u)" -eq 0 ]; then
    BIN_DIR="/usr/local/bin"
    APP_DIR="/usr/local/share/applications"
    ICON_DIR="/usr/local/share/icons/hicolor/256x256/apps"
elif command -v sudo >/dev/null 2>&1; then
    echo "Cài đặt LMD vào /usr/local/bin (yêu cầu quyền sudo)..."
    exec sudo bash "$0" "$@"
else
    BIN_DIR="$HOME/.local/bin"
    APP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
fi

mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

BIN_SRC="$SCRIPT_DIR/lmd"
[ ! -f "$BIN_SRC" ] && BIN_SRC="$SCRIPT_DIR/tezadis"
DESKTOP_SRC="$SCRIPT_DIR/lmd.desktop"
[ ! -f "$DESKTOP_SRC" ] && DESKTOP_SRC="$SCRIPT_DIR/tezadis.desktop"
ICON_SRC="$SCRIPT_DIR/lmd.png"
[ ! -f "$ICON_SRC" ] && ICON_SRC="$SCRIPT_DIR/tezadis.png"

install -m 755 "$BIN_SRC" "$BIN_DIR/lmd"
install -m 644 "$DESKTOP_SRC" "$APP_DIR/lmd.desktop"
install -m 644 "$ICON_SRC" "$ICON_DIR/lmd.png"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true
fi

echo "Đã cài đặt thành công LMD tại $BIN_DIR/lmd"
echo "Chạy ứng dụng bằng lệnh: lmd (hoặc mở từ menu ứng dụng)."
