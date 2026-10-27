#!/usr/bin/env bash
# Installs the NowPlay lockscreen layer as a user-local Plasma/Shell
# package registered via kpackagetool6. No root needed. Safe: the
# greeter falls back to the stock lockscreen if the override is ever
# incompatible.
#
# NOTE: simply copying files into ~/.local/share/plasma/shells does
# NOT work — the package must be installed through kpackagetool6 so
# KPackage registers it, otherwise the real lockscreen keeps loading
# the system package.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="$HOME/.local/share/plasma/shells/org.kde.plasma.desktop"
SRC="/usr/share/plasma/shells/org.kde.plasma.desktop"
UI="$DEST/contents/lockscreen/LockScreenUi.qml"

if [ ! -d "$SRC" ]; then
    echo "error: $SRC not found — this script targets KDE Plasma." >&2
    exit 1
fi

if [ ! -d "$DEST" ]; then
    kpackagetool6 --type Plasma/Shell --install "$SRC"
    echo "installed user shell package -> $DEST"
else
    echo "user shell package already present, patching in place"
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

# Register NowPlay settings in config.xml + config.qml so they can be
# tweaked from System Settings -> Screen Locking -> Appearance.
python3 - "$DEST" <<'PYEOF'
import sys
d = sys.argv[1] + "/contents/lockscreen"
x = d + "/config.xml"
s = open(x).read()
if "nowplayBlurRadius" not in s:
    block = '''    <entry name="nowplayBlurRadius" type="Int"><default>80</default></entry>
    <entry name="nowplayArtSize" type="Int"><default>14</default></entry>
    <entry name="nowplayShowVisualizer" type="Bool"><default>true</default></entry>
    <entry name="nowplayShowLyrics" type="Bool"><default>true</default></entry>
'''
    open(x, "w").write(s.replace("  </group>", block + "  </group>", 1))

q = d + "/config.qml"
s = open(q).read()
if "cfg_nowplayShowLyrics" not in s:
    s = s.replace(
        "property bool cfg_showMediaControlsDefault: false",
        "property bool cfg_showMediaControlsDefault: false\n"
        "    property int cfg_nowplayBlurRadius: 80\n"
        "    property int cfg_nowplayArtSize: 14\n"
        "    property bool cfg_nowplayShowVisualizer: true\n"
        "    property bool cfg_nowplayShowLyrics: true")
    ui = '''
    QQC2.CheckBox {
        Kirigami.FormData.label: "NowPlay:"
        text: "Visualizer"
        checked: configForm.cfg_nowplayShowVisualizer
        onToggled: configForm.cfg_nowplayShowVisualizer = checked
    }
    QQC2.CheckBox {
        text: "Synced lyrics"
        checked: configForm.cfg_nowplayShowLyrics
        onToggled: configForm.cfg_nowplayShowLyrics = checked
    }
    QQC2.SpinBox {
        Kirigami.FormData.label: "Blur radius:"
        from: 0; to: 200
        value: configForm.cfg_nowplayBlurRadius
        onValueModified: configForm.cfg_nowplayBlurRadius = value
    }
    QQC2.SpinBox {
        Kirigami.FormData.label: "Art size (grid units):"
        from: 6; to: 30
        value: configForm.cfg_nowplayArtSize
        onValueModified: configForm.cfg_nowplayArtSize = value
    }
'''
    s = s.rstrip()
    assert s.endswith("}")
    open(q, "w").write(s[:-1] + ui + "}\n")
print("config patched")
PYEOF

if [ "${1:-}" = "--testing" ]; then
    GREETER="$(command -v kscreenlocker_greet || echo /usr/libexec/kscreenlocker_greet)"
    exec "$GREETER" --testing --shell="$DEST"
fi

echo "done. Lock the screen (Meta+L) to see it. Uninstall: $(dirname "$0")/uninstall.sh"
