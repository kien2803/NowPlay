#!/usr/bin/env bash
# Removes the user-local shell package override, restoring the stock
# lockscreen (and stock shell package resolution) entirely.
set -euo pipefail
rm -rf "$HOME/.local/share/plasma/shells/org.kde.plasma.desktop"
echo "removed override — stock lockscreen restored"
