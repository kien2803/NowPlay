/*
    NowPlay lockscreen media layer.
    Full-screen blurred album art background + large art + controls.
    Reads MPRIS directly via Plasma's private Mpris module — same data
    source as the stock MediaControls.
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
                visible: false // only a blur source, never drawn raw
            }
            FastBlur {
                anchors.fill: parent
                source: bgArt
                radius: 80
                visible: bgArt.status === Image.Ready
            }
            // Dim so the password UI stays readable on bright covers.
            Rectangle {
                anchors.fill: parent
                color: "black"
                opacity: bgArt.status === Image.Ready ? 0.55 : 0.25
            }

            // --- Large foreground card ---
            ColumnLayout {
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: parent.bottom
                anchors.bottomMargin: Kirigami.Units.gridUnit * 6
                spacing: Kirigami.Units.largeSpacing
                width: Math.min(parent.width * 0.6, Kirigami.Units.gridUnit * 24)

                Image {
                    id: art
                    source: model.artUrl
                    Layout.preferredWidth: Kirigami.Units.gridUnit * 14
                    Layout.preferredHeight: Kirigami.Units.gridUnit * 14
                    Layout.alignment: Qt.AlignHCenter
                    fillMode: Image.PreserveAspectFit
                    asynchronous: true
                    visible: false

                    Rectangle {
                        id: artMask
                        anchors.fill: parent
                        radius: Kirigami.Units.gridUnit
                        visible: false
                    }
                }
                OpacityMask {
                    Layout.preferredWidth: Kirigami.Units.gridUnit * 14
                    Layout.preferredHeight: Kirigami.Units.gridUnit * 14
                    Layout.alignment: Qt.AlignHCenter
                    source: art
                    maskSource: artMask
                    visible: art.status === Image.Ready

                    DropShadow {
                        anchors.fill: parent
                        source: parent
                        radius: 24
                        samples: 33
                        color: Qt.rgba(0, 0, 0, 0.6)
                        verticalOffset: 4
                    }
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
                        textFormat: Text.PlainText
                    }
                    PlasmaExtras.DescriptiveLabel {
                        Layout.fillWidth: true
                        text: model.artist || ""
                        elide: Text.ElideRight
                        maximumLineCount: 1
                        horizontalAlignment: Text.AlignHCenter
                        font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
                        color: Qt.rgba(1, 1, 1, 0.75)
                        textFormat: Text.PlainText
                    }
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
                        icon.width: Kirigami.Units.gridUnit * 1.2
                        icon.height: Kirigami.Units.gridUnit * 1.2
                        onClicked: model.container.Previous()
                        Accessible.name: i18ndc("plasma_shell_org.kde.plasma.desktop", "@action:button Accessible only", "Previous track")
                    }
                    PlasmaComponents3.ToolButton {
                        Layout.preferredWidth: Kirigami.Units.gridUnit * 3.5
                        Layout.preferredHeight: Layout.preferredWidth
                        focusPolicy: Qt.TabFocus
                        enabled: model.canControl
                        icon.name: model.playbackStatus === Mpris.PlaybackStatus.Playing ? "media-playback-pause" : "media-playback-start"
                        icon.width: Kirigami.Units.gridUnit * 1.6
                        icon.height: Kirigami.Units.gridUnit * 1.6
                        onClicked: model.container.PlayPause()
                        Accessible.name: i18ndc("plasma_shell_org.kde.plasma.desktop", "@action:button Accessible only", "Play or Pause media")
                    }
                    PlasmaComponents3.ToolButton {
                        Layout.preferredWidth: Kirigami.Units.gridUnit * 2.5
                        Layout.preferredHeight: Layout.preferredWidth
                        visible: model.canGoBack || model.canGoNext
                        enabled: model.canGoNext
                        focusPolicy: Qt.TabFocus
                        icon.name: LayoutMirroring.enabled ? "media-skip-backward" : "media-skip-forward"
                        icon.width: Kirigami.Units.gridUnit * 1.2
                        icon.height: Kirigami.Units.gridUnit * 1.2
                        onClicked: model.container.Next()
                        Accessible.name: i18ndc("plasma_shell_org.kde.plasma.desktop", "@action:button Accessible only", "Next track")
                    }
                }
            }
        }
    }
}
