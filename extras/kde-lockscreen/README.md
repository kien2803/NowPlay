# NowPlay — KDE Plasma lockscreen layer

Adds a full-screen media widget to the Plasma 6 lockscreen: blurred
album-art backdrop, large rounded album art, track info and transport
controls. Uses the same MPRIS data source as the stock `MediaControls`
(`org.kde.plasma.private.mpris`), so it works with every MPRIS player.

## Install

```bash
./install.sh --testing   # preview in a window first (recommended)
./install.sh             # install for real
./uninstall.sh           # restore stock lockscreen
```

No root required. The script copies the stock shell package to
`~/.local/share/plasma/shells/org.kde.plasma.desktop/` and patches
`LockScreenUi.qml` to load `NowPlayMedia.qml` below the password UI.

If the greeter ever finds the override incompatible (e.g. after a
major Plasma upgrade), it falls back to the default lockscreen — you
cannot be locked out.

## Tuning

Edit `NowPlayMedia.qml` after install (or here and re-run `install.sh`):

| What | Key | Default |
|---|---|---|
| Backdrop blur strength | `FastBlur.radius` | `80` |
| Backdrop dim | overlay `Rectangle.opacity` | `0.55` |
| Art size | `gridUnit * 14` | ~252px |
| Panel position | `bottomMargin` | `gridUnit * 6` |
