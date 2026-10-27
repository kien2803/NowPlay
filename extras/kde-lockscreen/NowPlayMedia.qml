/*
    NowPlay lockscreen media layer v2.
    Full-screen blurred album-art backdrop + large art + controls,
    optional visualizer bars and synced lyric line (lrclib.net).
    Reads MPRIS via Plasma's private Mpris module.

    Customizable via System Settings -> Screen Locking -> Appearance
    (config keys nowplay* in config.xml).
*/

import QtQuick
import QtQuick.Layouts
import Qt5Compat.GraphicalEffects

import org.kde.plasma.components as PlasmaComponents3
import org.kde.plasma.extras as PlasmaExtras
import org.kde.kirigami as Kirigami
import org.kde.plasma.private.mpris as Mpris

Item {
    id: nowplay
    visible: instantiator.count > 0

    // --- synced-lyrics helper (plain JS, works in QML) ---
    function parseLrc(lrc) {
        var out = [];
        var lines = lrc.split("\n");
        var re = /\[(\d+):(\d+(?:\.\d+)?)\]/;
        for (var i = 0; i < lines.length; i++) {
            var m = re.exec(lines[i]);
            if (!m) continue;
            var t = parseInt(m[1]) * 60 + parseFloat(m[2]);
            var text = lines[i].replace(/\[[^\]]*\]/g, "").trim();
            if (text) out.push({t: t, text: text});
        }
        return out;
    }

    function fetchLyrics(title, artist, album, durSec, cb) {
        try {
            var xhr = new XMLHttpRequest();
            var q = "track_name=" + encodeURIComponent(title)
                  + "&artist_name=" + encodeURIComponent(artist)
                  + "&album_name=" + encodeURIComponent(album)
                  + "&duration=" + Math.round(durSec);
            xhr.open("GET", "https://lrclib.net/api/get?" + q, true);
            xhr.onreadystatechange = function() {
                if (xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
                    try { cb(JSON.parse(xhr.responseText)); } catch (e) { cb(null); }
                } else if (xhr.readyState === XMLHttpRequest.DONE) {
                    cb(null);
                }
            };
            xhr.send();
        } catch (e) { cb(null); }
    }

    Repeater {
        id: instantiator
        model: Mpris.MultiplexerModel { }

        Item {
            anchors.fill: parent

            // --- Blurred album-art background ---
            Image {
                id: bgArt
                anchors.fill: parent
                source: model.artUrl
                fillMode: Image.PreserveAspectCrop
                asynchronous: true
                visible: false
            }
            FastBlur {
                anchors.fill: parent
                source: bgArt
                radius: config.nowplayBlurRadius || 80
                visible: bgArt.status === Image.Ready
            }
            Rectangle {
                anchors.fill: parent
                color: "black"
                opacity: bgArt.status === Image.Ready ? 0.55 : 0.25
            }

            // --- Lyrics state ---
            property var lrc: null
            property int lrcIdx: -1
            property string trackKey: ""

            // MPRIS position (seconds). Role may be absent -> guarded.
            property real pos: (typeof model.position === "number") ? model.position : -1
            property bool isPlaying: model.playbackStatus === Mpris.PlaybackStatus.Playing

            onPosChanged: updateLine()
            onLrcChanged: updateLine()

            function updateLine() {
                if (!lrc || pos < 0) return;
                var idx = -1;
                for (var i = 0; i < lrc.length; i++) {
                    if (lrc[i].t <= pos) idx = i; else break;
                }
                lrcIdx = idx;
            }

            Component.onCompleted: {
                var key = (model.track || "") + "|" + (model.artist || "");
                if (key === trackKey || !model.track) return;
                trackKey = key;
                lrc = null;
                if (config.nowplayShowLyrics !== false) {
                    nowplay.fetchLyrics(model.track, model.artist || "", model.album || "",
                        (model.length || 0), function(j) {
                            if (j && j.syncedLyrics) lrc = nowplay.parseLrc(j.syncedLyrics);
                            else if (j && j.plainLyrics)
                                lrc = [{t: 0, text: j.plainLyrics.split("\n")[0]}];
                        });
                }
            }

            // --- Foreground panel ---
            ColumnLayout {
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: parent.bottom
                anchors.bottomMargin: Kirigami.Units.gridUnit * 5
                spacing: Kirigami.Units.largeSpacing
                width: Math.min(parent.width * 0.6, Kirigami.Units.gridUnit * 26)

                Image {
                    source: model.artUrl
                    Layout.preferredWidth: Kirigami.Units.gridUnit * (config.nowplayArtSize || 14)
                    Layout.preferredHeight: Layout.preferredWidth
                    Layout.alignment: Qt.AlignHCenter
                    fillMode: Image.PreserveAspectFit
                    asynchronous: true
                    visible: status === Image.Ready
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 0
                    PlasmaComponents3.Label {
                        Layout.fillWidth: true
                        text: model.track.length > 0 ? model.track : model.identity
                        elide: Text.ElideRight
                        maximumLineCount: 1
                        horizontalAlignment: Text.AlignHCenter
                        font.pointSize: Kirigami.Theme.defaultFont.pointSize + 3
                        font.weight: Font.DemiBold
                        color: "white"
                    }
                    PlasmaExtras.DescriptiveLabel {
                        Layout.fillWidth: true
                        text: model.artist || ""
                        elide: Text.ElideRight
                        maximumLineCount: 1
                        horizontalAlignment: Text.AlignHCenter
                        font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
                        color: Qt.rgba(1, 1, 1, 0.75)
                    }
                }

                // current synced lyric line
                PlasmaComponents3.Label {
                    Layout.fillWidth: true
                    visible: (config.nowplayShowLyrics !== false) && lrcIdx >= 0
                    text: lrcIdx >= 0 ? lrc[lrcIdx].text : ""
                    elide: Text.ElideRight
                    maximumLineCount: 1
                    horizontalAlignment: Text.AlignHCenter
                    font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
                    font.italic: true
                    color: Qt.rgba(1, 1, 1, 0.9)
                }

                RowLayout {
                    Layout.alignment: Qt.AlignHCenter
                    spacing: Kirigami.Units.largeSpacing

                    PlasmaComponents3.ToolButton {
                        Layout.preferredWidth: Kirigami.Units.gridUnit * 2.5
                        Layout.preferredHeight: Layout.preferredWidth
                        visible: model.canGoBack || model.canGoNext
                        enabled: model.canGoPrevious
                        focusPolicy: Qt.TabFocus
                        icon.name: LayoutMirroring.enabled ? "media-skip-forward" : "media-skip-backward"
                        onClicked: model.container.Previous()
                    }
                    PlasmaComponents3.ToolButton {
                        Layout.preferredWidth: Kirigami.Units.gridUnit * 3.5
                        Layout.preferredHeight: Layout.preferredWidth
                        focusPolicy: Qt.TabFocus
                        enabled: model.canControl
                        icon.name: model.playbackStatus === Mpris.PlaybackStatus.Playing
                            ? "media-playback-pause" : "media-playback-start"
                        onClicked: model.container.PlayPause()
                    }
                    PlasmaComponents3.ToolButton {
                        Layout.preferredWidth: Kirigami.Units.gridUnit * 2.5
                        Layout.preferredHeight: Layout.preferredWidth
                        visible: model.canGoBack || model.canGoNext
                        enabled: model.canGoNext
                        focusPolicy: Qt.TabFocus
                        icon.name: LayoutMirroring.enabled ? "media-skip-backward" : "media-skip-forward"
                        onClicked: model.container.Next()
                    }
                }

                // --- visualizer bars (decorative) ---
                Row {
                    Layout.alignment: Qt.AlignHCenter
                    spacing: 3
                    height: Kirigami.Units.gridUnit
                    visible: config.nowplayShowVisualizer !== false

                    Repeater {
                        model: 7
                        Rectangle {
                            width: 4
                            radius: 2
                            color: "white"
                            opacity: 0.85
                            height: parent.height
                            anchors.bottom: parent.bottom
                            transformOrigin: Item.Bottom
                            SequentialAnimation on scale {
                                loops: Animation.Infinite
                                running: isPlaying
                                NumberAnimation { from: 0.15; to: 1.0; duration: 400 + index * 90; easing.type: Easing.InOutSine }
                                NumberAnimation { from: 1.0; to: 0.15; duration: 500 + index * 70; easing.type: Easing.InOutSine }
                            }
                        }
                    }
                }
            }
        }
    }
}
