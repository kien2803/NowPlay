#!/usr/bin/env bash
# Installs the NowPlay lockscreen layer as a user-local override of the
# Plasma shell package. No root needed. Safe: the greeter falls back to
# the stock lockscreen if the override is ever incompatible.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="$HOME/.local/share/plasma/shells/org.kde.plasma.desktop"
SRC="/usr/share/plasma/shells/org.kde.plasma.desktop"
UI="$DEST/contents/lockscreen/LockScreenUi.qml"

if [ ! -d "$SRC" ]; then
    echo "error: $SRC not found — this script targets KDE Plasma." >&2
    exit 1
fi

# Copy the stock shell package locally (only done once; an existing
# override is respected so user's other tweaks survive).
if [ ! -d "$DEST" ]; then
    cp -r "$SRC" "$DEST"
    echo "copied shell package -> $DEST"
fi

cp "$HERE/NowPlayMedia.qml" "$DEST/contents/lockscreen/"

# Insert the NowPlay layer right before the clock's DropShadow, i.e.
# above the wallpaper fader and below the password UI.
if ! grep -q NowPlayMedia "$UI"; then
    sed -i '0,/        DropShadow {/s//        Loader {\n            anchors.fill: parent\n            active: config.showMediaControls\n            source: "NowPlayMedia.qml"\n        }\n\n        DropShadow {/' "$UI"
    echo "patched LockScreenUi.qml"
else
    echo "LockScreenUi.qml already patched"
fi

if [ "${1:-}" = "--testing" ]; then
    GREETER="$(command -v kscreenlocker_greet || echo /usr/libexec/kscreenlocker_greet)"
    exec "$GREETER" --testing --shell="$DEST"
fi

echo "done. Lock the screen (Meta+L) to see it. Uninstall: $(dirname "$0")/uninstall.sh"
