# Freally Capture — de
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Studiomodus
toggle-on = an
toggle-off = aus
stats = Statistiken
core-ok = Kern OK
hide-stats-dock = Statistik-Dock ausblenden
show-stats-dock = Statistik-Dock einblenden


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Einstellungen konnten nicht gespeichert werden — die Änderung übersteht keinen Neustart.
studio-mode-leave = Studiomodus verlassen
studio-mode-enter-title = Studiomodus — eine Vorschauszene bearbeiten und mit einem Übergang ins Programm übernehmen
vertical-canvas-title = Die zweite (vertikale 9:16) Ausgabe-Leinwand — unabhängig aufnehmbar und streambar
app-version = v{ $version }
core-error = Kern FEHLER
core-unreachable = Kern nicht erreichbar (Browsermodus)
connecting-to-core = Verbinde mit Kern…
filters-source-fallback = Quelle

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Programmvorschau
preview-program-output = Programmausgabe
preview-canvas-editor = Leinwand-Editor
preview-px-to-edge-label = Pixel bis zu den Bildrändern
preview-px-to-edge = px bis Rand L { $left } · O { $top } · R { $right } · U { $bottom }
preview-program-heading = Programm
preview-no-gpu = Kein nutzbarer GPU-Adapter gefunden — der Compositor kann auf diesem Rechner nicht laufen.
preview-starting-compositor = Compositor wird gestartet…
preview-empty-scene = Diese Szene ist leer — füge unter Quellen eine Quelle hinzu und ziehe, skaliere und rotiere sie direkt hier auf der Leinwand.
preview-fps = { $fps } fps
preview-dropped = { $dropped } verworfen

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Einladungslink empfangen
remote-join-with-webcam = Mit Webcam beitreten
remote-dismiss = Verwerfen
remote-hosting-guest = Hostet einen Remote-Gast
remote-you-are-guest = Du bist ein Remote-Gast
remote-share-view-title = Teile deinen Bildschirm mit der App des Gastes (er sieht deine Ansicht live)
remote-stop-sharing-view = Ansichtsfreigabe beenden
remote-share-my-view = Meine Ansicht teilen
remote-allow-center-title = Erlaube dem Gast zu wechseln, welche Ansicht die Mitte belegt (du behältst die Kontrolle und kannst jederzeit zurückwechseln)
remote-guest-switching = Gast-Wechsel:
remote-stop-screen = Bildschirm stoppen
remote-share-screen = Bildschirm teilen
remote-share-screen-title-guest = Teile deinen Bildschirm mit dem Host (er wird zu einer Quelle, die zentriert werden kann)
remote-center-request-label = Anfrage für zentrale Ansicht
remote-center = Zentrieren
remote-center-cam-title = Bitte den Host, deine Kamera zu zentrieren
remote-center-my-cam = Meine Kamera
remote-center-screen-title = Bitte den Host, deinen geteilten Bildschirm zu zentrieren
remote-center-my-screen = Mein Bildschirm
remote-center-host-title = Gib die Mitte an die Ansicht des Hosts zurück
remote-center-host-view = Host-Ansicht
remote-end-session = Sitzung beenden
remote-leave = Verlassen
remote-host-view-heading = Host-Ansicht
remote-host-shared-view-label = Die geteilte Ansicht des Hosts
remote-guest-position-label = Gast-Position
remote-guest-label = Gast
remote-put-guest = Gast { $position } platzieren
remote-remove-title = Gast entfernen — er kann mit demselben Link wieder beitreten
remote-remove = Entfernen
remote-ban-title = Gast sperren — blockiert ihn und macht den Einladungslink ungültig
remote-ban = Sperren
remote-guest-self-muted = Gast hat sich selbst stummgeschaltet
remote-unmute-guest = Gast-Stummschaltung aufheben
remote-mute-guest = Gast stummschalten
remote-muted-by-host = Vom Host stummgeschaltet
remote-unmute-mic = Mikro-Stummschaltung aufheben
remote-mute-mic = Mikro stummschalten
remote-waiting-for-host = warte auf den Host


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = Quelle
sources-fallback-video = Video
sources-fallback-error = Fehler
sources-kind-unknown = ?
sources-missing-source = (fehlende Quelle)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Bildschirm
sources-badge-window = Fenster
sources-badge-portal = Portal
sources-badge-camera = Kamera
sources-badge-image = Bild
sources-badge-media = Medien
sources-badge-guest = Gast
sources-badge-color = Farbe
sources-badge-text = Text
sources-badge-scene = Szene
sources-badge-slides = Folien
sources-badge-chat = Chat
sources-badge-audio-in = Audio-Ein
sources-badge-audio-out = Audio-Aus
sources-badge-app-audio = App-Audio

# Add-source menu items
sources-add-display = Bildschirmaufnahme
sources-add-window = Fensteraufnahme
sources-add-game = Spielaufnahme (zuerst lesen)
sources-add-webcam = Videoaufnahmegerät
sources-add-image = Bild
sources-add-media = Medien (Video-/Bilddatei)
sources-add-remote-guest = Remote-Gast (P2P-Spike)
sources-add-color = Farbe
sources-add-text = Text
sources-add-nested-scene = Verschachtelte Szene
sources-add-slideshow = Bild-Diashow
sources-add-chat-overlay = Live-Chat-Overlay
sources-add-audio-input = Audioeingabeaufnahme
sources-add-audio-output = Audioausgabeaufnahme
sources-add-app-audio = Anwendungsaudio (Windows)
sources-add-existing = Vorhandene Quelle…

# Panel header + toolbar buttons
sources-panel-title = Quellen
sources-group-title = Quellen gruppieren — wähle zwei oder mehr Elemente, dann Gruppe erstellen; gruppierte Elemente bewegen sich und werden zusammen ein-/ausgeblendet
sources-group-aria = Quellen gruppieren
sources-arrange = Anordnen: Bildschirm + Ecken
sources-add-source = Quelle hinzufügen
sources-browser-source-note = Browser-Quelle kommt als eigene bedarfsgesteuerte Komponente in einem eigenen Meilenstein (eine ~180 MB Chromium-Engine — nie mitgeliefert). Heute: nimm ein echtes Browserfenster mit Fensteraufnahme + einem Chroma-/Color-Key auf, oder öffne Chat/Benachrichtigungen als Dock (Steuerung → Docks).

# Empty state
sources-empty = Keine Quellen in dieser Szene — füge mit „+“ eine Bildschirmaufnahme, ein Fenster, eine Webcam, ein Bild, eine Farbe oder Text hinzu. Ziehe, skaliere und rotiere sie auf der Leinwand; die Schaltflächen rechts ordnen den Stapel um.

# Per-row controls
sources-already-in-group = Bereits in { $name }
sources-pick-for-new-group = Für die neue Gruppe auswählen
sources-pick-item-for-group = { $name } für die neue Gruppe auswählen
sources-hide = Ausblenden
sources-show = Einblenden
sources-hide-item = { $name } ausblenden
sources-show-item = { $name } einblenden
sources-unfocus-title = Fokus aufheben — Layout wiederherstellen
sources-focus-title = Fokussieren — Leinwand füllen (Sprecher hervorheben)
sources-unfocus-item = Fokus von { $name } aufheben
sources-focus-item = { $name } fokussieren
sources-center-title = Zentrieren — dies zur geteilten zentralen Ansicht machen (Kameras wandern in die Leiste)
sources-center-item = { $name } zentrieren
sources-rename-item = { $name } umbenennen
sources-in-group = In Gruppe { $name }

# Row status + retry
sources-retry-error = Wiederholen — { $message }
sources-retry-item = { $name } wiederholen
sources-status-error = Status: Fehler
sources-open-privacy-title = macOS-Datenschutzeinstellungen für diese Berechtigung öffnen
sources-open-privacy-item = Datenschutzeinstellungen für { $name } öffnen
sources-privacy-settings-button = Einstellungen
sources-status-starting = startet…
sources-status-live = live
sources-status-aria = Status: { $state }

# Media row pause/resume
sources-media-resume-title = Video fortsetzen (live im Stream)
sources-media-pause-title = Video pausieren — Bild halten und verstummen, live im Stream
sources-media-resume-item = { $name } fortsetzen
sources-media-pause-item = { $name } pausieren

# Hover controls
sources-unlock = Entsperren
sources-lock = Sperren
sources-unlock-item = { $name } entsperren
sources-lock-item = { $name } sperren
sources-raise-title = Im Stapel nach oben
sources-raise-item = { $name } nach oben
sources-lower-title = Im Stapel nach unten
sources-lower-item = { $name } nach unten
sources-filters-title = Filter & Mischmodus
sources-filters-item = Filter für { $name }
sources-properties-title = Eigenschaften
sources-properties-item = Eigenschaften von { $name }
sources-remove-title = Aus dieser Szene entfernen
sources-remove-item = { $name } entfernen

# Grouping footer
sources-create-group = Gruppe erstellen ({ $count })
sources-cancel = Abbrechen

# Groups list
sources-groups-aria = Quellgruppen
sources-hide-group = Gruppe ausblenden
sources-show-group = Gruppe einblenden
sources-item-count = · { $count } Elemente
sources-ungroup-title = Gruppierung aufheben — die Elemente bleiben, wo sie sind
sources-ungroup-item = Gruppierung von { $name } aufheben

# Live Chat Overlay picker
sources-chat-title = Live-Chat-Overlay hinzufügen
sources-chat-youtube-label = YouTube — Kanal-, Watch- oder live_chat-URL (kein Key, keine Anmeldung)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  oder eine watch?v=-URL
sources-chat-twitch-label = Twitch — Kanalname (anonym gelesen, kein Konto)
sources-chat-twitch-placeholder = deinkanal
sources-chat-kick-label = Kick — Kanal-Slug (öffentlicher Endpunkt, best-effort)
sources-chat-kick-placeholder = deinkanal
sources-chat-note = Nachrichten erscheinen mit einem laufenden h:mm:ss AM/PM-Zeitstempel auf transparentem Hintergrund (standardmäßig oben rechts; ziehe es beliebig hin). Eine Chat-Flut lässt nur alte Zeilen verfallen — sie kann Stream oder Aufnahme niemals blockieren. Facebook-Chat benötigt deinen eigenen Graph-Token und ist noch nicht implementiert — er ist nie erforderlich und blockiert nie die oben genannten Plattformen.
sources-chat-add = Chat-Overlay hinzufügen
sources-chat-default-name = Live-Chat

# Image Slideshow picker
sources-slideshow-title = Bild-Diashow hinzufügen
sources-slideshow-empty = Noch keine Bilder — Durchsuchen fügt sie der Reihe nach hinzu.
sources-slideshow-remove-slide = Folie { $number } entfernen
sources-slideshow-browse = Bilder durchsuchen…
sources-slideshow-per-slide-label = Pro Folie (ms)
sources-slideshow-crossfade-label = Überblendung (ms, 0 = harter Schnitt)
sources-slideshow-loop-label = Schleife (aus = letzte Folie halten)
sources-slideshow-shuffle-label = Bei jedem Durchlauf mischen
sources-slideshow-note = Die Überblendung mischt gleich große Bilder; unterschiedliche Größen schneiden an der Grenze hart um (kein stilles Umskalieren).
sources-slideshow-add = Diashow hinzufügen ({ $count })

# Nested Scene picker
sources-nested-title = Verschachtelte Szene hinzufügen
sources-nested-empty = Keine andere Szene zum Verschachteln — füge zuerst eine zweite Szene hinzu.
sources-nested-scene-name = Szene: { $name }
sources-nested-note = Die verschachtelte Szene wird live in der Größe der Programm-Leinwand gerendert und folgt ihren eigenen Bearbeitungen; Transformationen, Filter und Mischmodus gelten für sie wie für jede Quelle. Ihre Audioquellen kommen in den Mix, solange eine Szene, die sie zeigt, das Programm ist.

# Display / Window capture picker
sources-capture-display-title = Bildschirmaufnahme hinzufügen
sources-capture-window-title = Fensteraufnahme hinzufügen
sources-capture-looking = Suche nach Quellen…
sources-capture-none-displays = Nichts aufzunehmen — keine Bildschirme gefunden.
sources-capture-none-windows = Nichts aufzunehmen — keine Fenster gefunden.
sources-capture-portal-note = Unter Wayland wählt der Systemdialog den Bildschirm oder das Fenster — Apps können dort nicht global aufnehmen, also ist das der ehrliche (und einzige) Weg.
sources-capture-window-note = Vorschauen aktualisieren sich live. Ein minimiertes Fenster zeigt sein letztes Bild (oder keines), bis du es wiederherstellst.
sources-thumb-no-preview = keine Vorschau
sources-thumb-loading = lädt…

# Video Capture Device picker
sources-webcam-title = Videoaufnahmegerät hinzufügen
sources-webcam-looking = Suche nach Kameras…
sources-webcam-none = Keine Kameras oder Aufnahmekarten gefunden.
sources-webcam-format-label = Format
sources-webcam-format-auto-loading = Auto (Formate werden geladen…)
sources-webcam-format-auto = Auto (höchste Auflösung)
sources-webcam-card-presets-label = Karten-Voreinstellungen:
sources-webcam-preset-title = Wähle den { $label }-Modus, den diese Karte anbietet
sources-webcam-add = Kamera hinzufügen

# Audio Input / Output capture picker
sources-audio-output-title = Audioausgabeaufnahme hinzufügen
sources-audio-input-title = Audioeingabeaufnahme hinzufügen
sources-audio-default-output = Standardausgabe (was du hörst)
sources-audio-default-input = Standardeingabe
sources-audio-looking = Suche nach Audiogeräten…
sources-audio-none-output = Hier wurde kein Gerät zur Desktop-Audioaufnahme gefunden.
sources-audio-none-input = Keine Mikrofone oder Line-Ins gefunden.
sources-audio-input-note = Mixer-Kanäle erhalten eine VU-Anzeige, einen Fader, Stummschaltung, Monitoring, Filter (Rauschunterdrückung, Gate, Kompressor…) und Spurzuweisung. Alles bleibt auf diesem Rechner.

# Application Audio picker
sources-appaudio-title = Anwendungsaudio hinzufügen
sources-appaudio-looking = Suche nach Apps, die Ton ausgeben…
sources-appaudio-none = Gerade gibt keine App Ton aus — starte die Wiedergabe in der App und aktualisiere dann.
sources-appaudio-refresh = ⟳ Aktualisieren
sources-appaudio-note = Nimmt genau das Audio dieser App auf — mit eigener VU-Anzeige, Fader, Stummschaltung, Filtern und Spur.

# Game Capture picker
sources-game-title = Spielaufnahme
sources-game-checking = Prüfe…
sources-game-use-portal = Bildschirmaufnahme verwenden (Portal)
sources-game-use-window = Stattdessen Fensteraufnahme verwenden

# Image picker
sources-image-title = Bild hinzufügen
sources-image-file-label = Bilddatei (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Bild hinzufügen

# Path field
sources-browse = Durchsuchen…

# Media picker
sources-media-title = Medien hinzufügen
sources-media-file-label = Mediendatei (mp4, mkv, webm, mov, .frec oder ein Bild)
sources-media-loop-label = Schleife (am Ende von vorn beginnen)
sources-media-note = .frec läuft über den eigenen freally-video-Codec — nichts herunterzuladen. Die Wire-Formate (mp4/mkv/webm/…) werden über die bedarfsgesteuerte FFmpeg-Komponente dekodiert; ihr Audio landet als eigener Kanal im Mixer.
sources-media-add = Medien hinzufügen

# Invite expiry options
sources-ttl-15min = 15 Min.
sources-ttl-30min = 30 Min.
sources-ttl-1hour = 1 Stunde
sources-ttl-1day = 1 Tag

# Remote Guest form
sources-remote-copy-failed = Kopieren fehlgeschlagen — markiere den Link und kopiere ihn manuell
sources-remote-join-failed = Beitritt fehlgeschlagen: { $error }
sources-remote-title = Remote-Gast (P2P-Spike)
sources-remote-host-heading = Host — einen Gast einladen
sources-remote-start-hosting = Hosting starten
sources-remote-expires-label = Läuft ab
sources-remote-invite-expiry-aria = Ablauf der Einladung
sources-remote-invite-link-aria = Einladungslink
sources-remote-copied = Kopiert ✓
sources-remote-copy = Kopieren
sources-remote-share-note = Teile diesen Link (Discord / SMS / E-Mail). Er trägt deine Sitzung und läuft wie eingestellt ab. Der Gast öffnet ihn und tritt mit seiner Webcam bei.
sources-remote-qr-note = Auf einem Handy scannen, um direkt aus dem Browser beizutreten — Kamera + Mikro, keine Installation. Der kopierbare freally://-Link oben öffnet sich in Freally Capture auf einem Rechner, der es hat.
sources-remote-guest-heading = Gast — mit einer Einladung beitreten
sources-remote-paste-placeholder = Einladungslink einfügen
sources-remote-invite-input-aria = Einladungslink oder Sitzungs-ID
sources-remote-join = Mit Webcam beitreten
sources-remote-session-note = Die Live-Sitzungssteuerung (Stummschalten, Beenden) bleibt oben in der Leiste des Hauptfensters — du kannst diesen Dialog schließen.
sources-remote-stop-session = Sitzung stoppen

# Invite QR
sources-invite-qr-aria = QR-Code des Einladungslinks

# Remote device pickers
sources-devices-output-unavailable = Ausgabe-Routing nicht verfügbar — Wiedergabe auf dem Standardgerät
sources-devices-mic-test-failed = Mikrofontest fehlgeschlagen: { $error }
sources-devices-heading = Audiogeräte der Sitzung
sources-devices-microphone-label = Mikrofon
sources-devices-microphone-aria = Sitzungsmikrofon
sources-devices-system-default = Systemstandard
sources-devices-output-label = Ausgabe
sources-devices-output-aria = Audioausgabe der Sitzung
sources-devices-stop-test = Test stoppen
sources-devices-test = Test — dich selbst hören
sources-devices-testing-note = sprich ins Mikro — du hörst die gewählten Geräte live
sources-devices-idle-note = leitet dein Mikro zur Ausgabe (Kopfhörer vermeiden Rückkopplung)

# TURN relay section
sources-turn-save-failed = Speichern fehlgeschlagen: { $error }
sources-turn-summary = Netzwerk — optionales TURN-Relay (erweitert)
sources-turn-note-1 = Sitzungen verbinden sich direkt (P2P) — kostenlos, kein Relay nötig. Sitzen BEIDE Seiten hinter strikten NATs, kann der direkte Weg scheitern; ein TURN-Relay, das du selbst betreibst, überträgt dann die Medien. Dies zu überspringen ist in Ordnung — die meisten Verbindungen funktionieren rein direkt.
sources-turn-note-2 = Kostenlose Option: Oracle Cloud „Always Free“ betreibt coturn kostenlos (Hinweis: Oracle fragt bei der Anmeldung nach einer Kreditkarte, aber die Always-Free-Variante bleibt kostenlos). Schritte: 1) die kostenlose VM erstellen, 2) coturn installieren, 3) UDP 3478 öffnen, 4) Benutzer/Passwort festlegen, 5) hier turn:deine-vm-ip:3478 + die Anmeldedaten eintragen. Deine Anmeldedaten bleiben in deiner lokalen Einstellungsdatei und werden nie protokolliert.
sources-turn-url-label = TURN-URL
sources-turn-url-placeholder = turn:host:3478 (leer = nur direkt)
sources-turn-url-aria = TURN-URL
sources-turn-username-label = Benutzername
sources-turn-username-aria = TURN-Benutzername
sources-turn-credential-label = Anmeldedaten
sources-turn-credential-aria = TURN-Anmeldedaten
sources-turn-note-3 = Das Relay greift, sobald alle drei Felder gesetzt sind (ein TURN-Server benötigt die Anmeldedaten), und gilt für die nächste Sitzung, die du startest oder der du beitrittst. Prüfe es mit einem Relay-only-Testanruf zwischen deinen eigenen zwei Rechnern.
sources-turn-settings-unavailable = Einstellungen nicht verfügbar (Browsermodus)

# Color picker
sources-color-title = Farbe hinzufügen
sources-color-label = Farbe
sources-color-width-label = Breite
sources-color-height-label = Höhe
sources-color-add = Farbe hinzufügen

# Text picker
sources-text-title = Text hinzufügen
sources-text-label = Text
sources-text-default = Text
sources-text-color-label = Farbe
sources-text-color-aria = Textfarbe
sources-text-size-label = Größe (px)
sources-text-note = Schriftfamilie, Ausrichtung, Umbruch und RTL findest du in den Eigenschaften der Quelle. Die mitgelieferte Noto Sans (inkl. Arabisch/Hebräisch) ist die Standardschrift — auf jedem Rechner identisch.
sources-text-add = Text hinzufügen

# Existing source picker
sources-existing-title = Vorhandene Quelle hinzufügen
sources-existing-empty = Es gibt noch keine Quellen — füge zuerst einer beliebigen Szene eine hinzu. Vorhandene Quellen werden geteilt: Umbenennen oder Neukonfigurieren aktualisiert jede Szene, die sie zeigt.

# Screen + corners layout
sources-slot-off = Aus
sources-slot-center = Mitte (Bildschirm)
sources-slot-top-left = Oben links
sources-slot-top-right = Oben rechts
sources-slot-bottom-left = Unten links
sources-slot-bottom-right = Unten rechts
sources-layout-title = Anordnen: Bildschirm + Ecken
sources-layout-empty = Füge dieser Szene zuerst eine Bildschirmaufnahme und eine oder mehrere Kameras hinzu, dann ordne sie hier an.
sources-layout-note = Setze einen Bildschirm in die Mitte und bis zu vier Kameras in die Ecken — dein Erklär-/Podcast-Layout. Jede Ecke hält eine Webcam, ein aufgenommenes Anruffenster oder einen Medienclip. Du kannst jedes davon danach auf der Leinwand ziehen.
sources-layout-slot-aria = Platz für { $name }
sources-layout-apply = Layout anwenden


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Steuerung
controls-start-stop-title-stop = Aufnahme stoppen und abschließen
controls-start-stop-title-start = Den Programmfeed mit der Konfiguration aus Einstellungen → Ausgabe aufnehmen
controls-finalizing = ◌ Abschließen…
controls-stop-recording = ■ Aufnahme stoppen
controls-start-recording = ● Aufnahme starten
controls-marker-title = Setze in diesem Moment einen Kapitelmarker — er landet in der AUFNAHME (mkv-Kapitel oder eine Sidecar-Datei). Plattformseitige Stream-Marker brauchen Plattform-Konten, nach denen diese App nie fragt.
controls-marker = ◈ Marker
controls-pause-title-resume = Fortsetzen — die Datei läuft als eine zusammenhängende Zeitleiste weiter
controls-pause-title-pause = Pausieren — es werden keine Bilder geschrieben; Fortsetzen führt dieselbe abspielbare Datei weiter
controls-resume-recording = ▶ Aufnahme fortsetzen
controls-pause-recording = ⏸ Aufnahme pausieren
controls-reactions-label = Reaktionen (ins Programm eingebrannt)
controls-reactions-title = Lasse eine Reaktion über dem Programm schweben — aufgenommen UND gestreamt, damit die Wiederholung den genauen Moment zeigt. Zuschauer im Chat lösen diese ebenfalls aus (ihr Reaktions-Emoji schwebt automatisch); eine Flut begrenzt nur, was auf dem Bildschirm ist.
controls-react = Reagieren { $emoji }
controls-virtual-camera-title = Die virtuelle Kamera braucht pro Betriebssystem eine eigene signierte Treiberkomponente (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO-Erweiterung / Linux v4l2loopback) — sie kommt als eigener Meilenstein. Das Feed-Modell ist dafür bereit: Programm, vertikale Leinwand oder eine einzelne Quelle, mit einem gepaarten virtuellen Mikro unter Windows/Linux (macOS hat keine Virtual-Mic-API — ehrlich gesagt).
controls-virtual-camera = ⌁ Virtuelle Kamera starten
controls-files-title = Fertige Aufnahmen + die Aktion zum Remux nach mp4
controls-files = ▤ Dateien…
controls-output-title = Aufnahmeformat, Encoder, Ordner, Spuren und Aufteilung
controls-output = ⚙ Ausgabe…
controls-stream-title = Live-gehen-Ziel: Dienst, Streamschlüssel, Encoder, Bitrate
controls-stream = ⦿ Stream…
controls-codecs-title = Die bedarfsgesteuerte ffmpeg-Wire-Codec-Komponente (klar gekennzeichnet, nie mitgeliefert)
controls-codecs = ⬡ Codecs…
controls-replay-title = Länge des Wiederholungspuffers + Qualitätsvoreinstellungen
controls-replay = ⟲ Wiederholung…
controls-keys-title = Globale Tastenkürzel: Aufnahme, Live gehen, Übergang, Wiederholung speichern
controls-keys = ⌨ Tasten…
controls-scripts-title = Sandbox-Lua-Skripte: auf Live-/Szenen-/Aufnahmeereignisse reagieren und das Studio steuern
controls-scripts = ⚡ Skripte…
controls-docks-title = Browser-Docks: ein Chat-Popout, eine Benachrichtigungsseite oder Companion-Schaltflächen als Fenster neben dem Studio öffnen
controls-docks = ⧉ Docks…
controls-remote-title = WebSocket-Remote-API für Stream-Deck-/Companion-Controller (standardmäßig aus)
controls-remote = ⌁ Remote…
controls-profiles-title = Profile (Einstellungen) + Szenensammlungen — umschaltbare Snapshots
controls-profiles = ▣ Profile…
controls-bug-title = Einen Fehler melden — anonym, opt-in (nichts wird automatisch gesendet)
controls-bug = 🐞 Fehler melden…
controls-updates-title = Nach Updates suchen — signiert, verifiziert, nichts wird ohne Klick heruntergeladen
controls-updates = ⭳ Nach Updates suchen…
controls-saved = Gespeichert: { $path }

# --- MixerDock.tsx ---
mixer-title = Audio-Mixer
mixer-monitor-error = Monitor: { $error }
mixer-switch-to-horizontal = Zu horizontalen Kanälen wechseln
mixer-switch-to-vertical = Zu vertikalen Kanälen wechseln
mixer-layout-aria-vertical = Mixer-Layout: vertikal — zu horizontal wechseln
mixer-layout-aria-horizontal = Mixer-Layout: horizontal — zu vertikal wechseln
mixer-empty = Keine Audioquellen in dieser Szene — füge mit „+“ unter Quellen eine Audioeingabeaufnahme (Mikro) oder Audioausgabeaufnahme (Desktop-Audio) hinzu. Kanäle erhalten eine VU-Anzeige, einen Fader, Stummschaltung, Monitoring, Filter und Spurzuweisung.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Programmlautheit (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Momentane Lautheit (400 ms)
mixer-short-term-title = Kurzzeit-Lautheit (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Monitor-Ausgabegerät
mixer-default-output = Standardausgabe

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Speicher
stats-dropped = Verworfen
stats-render = Rendern
stats-gpu = GPU
stats-gpu-compositing = kompositiert
stats-gpu-idle = inaktiv
stats-vertical-fps = 9:16 FPS
stats-targets-label = Stream-Ziele
stats-shared-encode = · geteilte Kodierung
stats-starting = Compositor wird gestartet…

# --- ScenesRail.tsx ---
scenes-title = Szenen
scenes-new-scene-name = Szene
scenes-add = Szene hinzufügen
scenes-empty = Verbinde mit dem Studio-Kern…
scenes-rename = { $name } umbenennen
scenes-on-program = Im Programm
scenes-preview = { $name } in der Vorschau
scenes-switch-to = Zu { $name } wechseln
scenes-move-up = Nach oben
scenes-move-up-aria = { $name } nach oben
scenes-move-down = Nach unten
scenes-move-down-aria = { $name } nach unten
scenes-last-stays = Die letzte Szene bleibt
scenes-remove = Diese Szene entfernen
scenes-remove-aria = { $name } entfernen


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Pegel
channelstrip-monitor-off = Monitor aus
channelstrip-monitor-only = Nur Monitor (nicht im Mix)
channelstrip-monitor-and-output = Monitor und Ausgabe
channelstrip-status-error = Fehler
channelstrip-status-live = live
channelstrip-status-waiting-audio = warte auf Audio
channelstrip-status = Status: { $state }
channelstrip-status-waiting = wartet
channelstrip-mute = Stumm
channelstrip-unmute = Stumm aufheben
channelstrip-mute-source = { $name } stummschalten
channelstrip-unmute-source = Stummschaltung von { $name } aufheben
channelstrip-scene-mix-on = Pro-Szene-Mix AN — dieser Kanal überschreibt für diese Szene den globalen Mix (klicken, um wieder dem globalen Mix zu folgen)
channelstrip-scene-mix-off = Pro-Szene-Mix — gib diesem Kanal einen eigenen Fader/Stummschaltung für die aktuelle Szene
channelstrip-scene-mix-label = Pro-Szene-Mix für { $name }
channelstrip-monitor-cycle = { $mode } — klicken zum Durchschalten
channelstrip-monitor-mode = Monitor-Modus von { $name }: { $mode }
channelstrip-audio-filters-title = Audiofilter (Rauschunterdrückung, Gate, Kompressor…)
channelstrip-audio-filters-label = Audiofilter für { $name }
channelstrip-advanced-title = Sync-Versatz & Push-to-Talk-Tasten
channelstrip-advanced-label = Erweiterte Audioeinstellungen für { $name }
channelstrip-track-assignment = Spurzuweisung
channelstrip-track = Spur { $n }
channelstrip-track-assigned = Spur { $n } (zugewiesen)
channelstrip-track-label = Spur { $n } für { $name }
channelstrip-device-error = Gerätefehler
channelstrip-audio-device-error = Audiogerätefehler
channelstrip-volume-label = Lautstärke von { $name } in Dezibel
channelstrip-ptt-hold = Push-to-Talk: { $key } halten
channelstrip-sync-offset = Sync-Versatz (ms, 0–{ $max } — verzögert dieses Audio)
channelstrip-ptt-hotkey = Push-to-Talk-Taste (stumm, außer gehalten)
channelstrip-ptt-placeholder = z. B. Ctrl+Shift+T oder F13
channelstrip-ptt-aria = Push-to-Talk-Taste
channelstrip-ptm-hotkey = Push-to-Mute-Taste (stumm, solange gehalten)
channelstrip-ptm-placeholder = z. B. Ctrl+Shift+M
channelstrip-ptm-aria = Push-to-Mute-Taste
channelstrip-hotkeys-note = Tastenkürzel funktionieren, während andere Apps im Fokus sind. Unter Linux/Wayland sind globale Tastenkürzel evtl. nicht verfügbar — das ist eine Compositor-Grenze, ehrlich gesagt.
channelstrip-apply = Anwenden


# --- LiveButton.tsx ---
livebutton-failure-ended = der Stream wurde beendet
livebutton-title-live = Den Stream beenden — jedes Ziel (eine laufende Aufnahme läuft weiter)
livebutton-title-offline = Zu jedem aktivierten Ziel unter Einstellungen → Stream live gehen
livebutton-end-stream = ■ Stream beenden
livebutton-aria-reconnecting = Neuverbindung
livebutton-aria-live = Live
livebutton-badge-retry = Versuch { $n }
livebutton-badge-live = live
livebutton-go-live = ⦿ Live gehen


# --- RecDot.tsx ---
recdot-paused-aria = Aufnahme pausiert
recdot-recording-aria = Aufnahme
recdot-tracks-one = { $count } Audiospur wird aufgenommen
recdot-tracks-other = { $count } Audiospuren werden aufgenommen
recdot-paused = pausiert


# --- ReplayControls.tsx ---
replaycontrols-saved = Wiederholung gespeichert — { $name }
replaycontrols-failure-stopped = der Puffer wurde gestoppt
replaycontrols-title-disarm = Wiederholungspuffer deaktivieren (verwirft den ungespeicherten Verlauf)
replaycontrols-title-arm = Den rollenden Wiederholungspuffer aktivieren — hält die letzten N Sekunden zum Speichern bereit (eigene leichtgewichtige Kodierung; Stream und Aufnahme bleiben unberührt)
replaycontrols-replay-seconds = ⟲ Wiederholung { $seconds }s
replaycontrols-arm = ⟲ Wiederholungspuffer aktivieren
replaycontrols-save-title = Die letzten N Sekunden in den Aufnahmeordner speichern (auch über das Wiederholung-speichern-Tastenkürzel)
replaycontrols-save = ⤓ Speichern


# --- PropertiesDialog.tsx ---
properties-title = Eigenschaften — { $name }
properties-name = Name
properties-cancel = Abbrechen
properties-apply = Anwenden
properties-youtube = YouTube — Kanal- / Watch- / live_chat-URL (kein Key, keine Anmeldung, nie)
properties-twitch = Twitch — Kanalname (anonym)
properties-kick = Kick — Kanal-Slug (öffentlicher Endpunkt)
properties-width-px = Breite (px)
properties-lines = Zeilen
properties-font-px = Schrift (px)
properties-images = Bilddateien (ein Pfad pro Zeile, in Reihenfolge gezeigt)
properties-per-slide = Pro Folie (ms)
properties-crossfade = Überblendung (ms, 0 = harter Schnitt)
properties-loop-slideshow = Schleife (aus = letzte Folie halten)
properties-shuffle = Bei jedem Durchlauf mischen
properties-nested-scene = Szene, die diese Quelle zusammensetzt (eine Szene, die diese bereits enthält, wird abgelehnt)
properties-portal-note = Das Wayland-ScreenCast-Portal wählt bei jedem Start dieser Quelle den Bildschirm oder das Fenster im Systemdialog — hier gibt es per Design nichts zu konfigurieren.
properties-appaudio-capturing = Nimmt Audio von { $exe } auf
properties-appaudio-exe-fallback = eine Anwendung
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Füge die Quelle erneut hinzu, um eine andere App anzuvisieren (eine Prozess-ID ändert sich, wenn die App neu startet).
properties-image-file = Bilddatei
properties-media-file = Mediendatei (mp4, mkv, webm, mov, .frec oder ein Bild)
properties-media-loop = Schleife (am Ende von vorn beginnen)
properties-media-hwdecode = Hardware-Dekodierung (fällt bei Bedarf selbst auf Software zurück)
properties-media-note = .frec läuft über den eigenen freally-video-Codec — nichts herunterzuladen. Andere Videoformate werden über die bedarfsgesteuerte FFmpeg-Komponente dekodiert. Das Audio der Datei bekommt einen eigenen Mixer-Kanal; der Sync-Versatz des Kanals justiert die A/V-Ausrichtung fein. Ein Clip ohne Audio lässt seinen Kanal stumm.
properties-color = Farbe
properties-width = Breite
properties-height = Höhe
properties-text = Text
properties-font-family = Schriftfamilie (System; leer = Standard)
properties-size-px = Größe (px)
properties-text-color = Textfarbe
properties-align = Ausrichtung
properties-align-left = links
properties-align-center = zentriert
properties-align-right = rechts
properties-line-spacing = Zeilenabstand
properties-wrap-width = Umbruchbreite (px; 0 = aus)
properties-force-rtl = Rechts-nach-links erzwingen
properties-text-note = Das Rendern nutzt echtes Shaping (arabische Verbindungen, Ligaturen) und Bidi-Zeilenanordnung. Die mitgelieferte Noto-Sans-Familie (inkl. Arabisch/Hebräisch) ist der Standard; Systemfamilien funktionieren auch. CJK nutzt vorerst Systemschriften.
properties-repick-capturing = Nimmt auf: { $label }
properties-repick-looking = Suche nach Quellen…
properties-repick-none-displays = Keine Bildschirme zum erneuten Auswählen gefunden.
properties-repick-none-windows = Keine Fenster zum erneuten Auswählen gefunden.
properties-repick-again = Erneut auswählen:
properties-device = Gerät
properties-video-current-device = (aktuelles Gerät)
properties-format = Format
properties-format-auto-loading = Auto (Formate werden geladen…)
properties-format-auto = Auto (höchste Auflösung)
properties-audio-capture-of = Audio aufnehmen von
properties-audio-default-output = Standardausgabe (was du hörst)
properties-audio-default-input = Standardeingabe
properties-audio-default-suffix = (Standard)
properties-audio-current-device = (aktuelles Gerät: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Verstärkung
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Kompressor
audiofilters-name-limiter = Limiter
audiofilters-name-eq = 3-Band-EQ
audiofilters-name-denoise = Rauschunterdrückung
audiofilters-name-ducking = Ducking
audiofilters-title = Audiofilter — { $name }
audiofilters-chain-header = Filterkette (oben läuft zuerst, vor dem Fader)
audiofilters-add = + Filter hinzufügen
audiofilters-add-menu = Einen Audiofilter hinzufügen
audiofilters-empty = Noch keine Filter — entrausche ein Mikro (klassisches DSP, kein ML), gate den Raum, zähme Spitzen mit dem Kompressor oder ducke Musik unter deine Stimme.
audiofilters-enable = { $name } aktivieren
audiofilters-run-earlier = Früher ausführen
audiofilters-move-up = { $name } nach oben
audiofilters-run-later = Später ausführen
audiofilters-move-down = { $name } nach unten
audiofilters-remove-title = Filter entfernen
audiofilters-remove = { $name } entfernen
audiofilters-gain-db = Verstärkung (dB)
audiofilters-open-db = Öffnen bei (dB)
audiofilters-close-db = Schließen bei (dB)
audiofilters-attack-ms = Attack (ms)
audiofilters-hold-ms = Halten (ms)
audiofilters-release-ms = Release (ms)
audiofilters-ratio = Verhältnis (:1)
audiofilters-threshold-db = Schwelle (dB)
audiofilters-output-gain-db = Ausgangsverstärkung (dB)
audiofilters-ceiling-db = Obergrenze (dB)
audiofilters-low-db = Tiefen (dB)
audiofilters-mid-db = Mitten (dB)
audiofilters-high-db = Höhen (dB)
audiofilters-strength = Stärke
audiofilters-denoise-note = Eigene klassische DSP-Spektralunterdrückung — gleichmäßiges Rauschen (Lüfter, Rauschen) sinkt, während Sprache durchkommt. Kein ML, keine Modelle, gemäß Charter.
audiofilters-duck-under = Ducken unter
audiofilters-ducking-trigger = Auslösequelle für Ducking
audiofilters-pick-trigger = (einen Auslöser wählen — z. B. dein Mikro)
audiofilters-trigger-at-db = Auslösen bei (dB)
audiofilters-duck-by-db = Ducken um (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Render-Verzögerung
filters-name-color-correction = Farbkorrektur
filters-name-lut = LUT anwenden
filters-name-blur = Weichzeichnen
filters-name-mask = Bildmaske
filters-name-sharpen = Schärfen
filters-name-scroll = Scrollen
filters-name-crop = Zuschneiden
filters-title = Filter — { $name }
filters-blend-mode = Mischmodus
filters-chain-header = Filterkette (oben läuft zuerst)
filters-add = + Filter hinzufügen
filters-add-menu = Einen Filter hinzufügen
filters-empty = Noch keine Filter — chroma-keye eine Webcam, korrigiere die Farbe einer Aufnahme oder scrolle einen Ticker.
filters-enable = { $name } aktivieren
filters-run-earlier = Früher ausführen
filters-move-up = { $name } nach oben
filters-run-later = Später ausführen
filters-move-down = { $name } nach unten
filters-remove-title = Filter entfernen
filters-remove = { $name } entfernen
filters-key-color-rgb = Key-Farbe (beliebige Farbe, RGB-Abstand)
filters-similarity = Ähnlichkeit
filters-smoothness = Weichheit
filters-luma-min = Luma min (Dunkleres ausschlüsseln)
filters-luma-max = Luma max (Helleres ausschlüsseln)
filters-delay = Verzögerung (ms — nur Video, z. B. zur Audio-Synchronisation; max. 500)
filters-key-color = Key-Farbe
filters-spill = Spill
filters-gamma = Gamma
filters-brightness = Helligkeit
filters-contrast = Kontrast
filters-saturation = Sättigung
filters-hue-shift = Farbtonverschiebung
filters-opacity = Deckkraft
filters-cube-file = .cube-Datei
filters-amount = Menge
filters-radius = Radius
filters-mask-image = Maskenbild
filters-mask-mode = Modus
filters-mask-alpha = Alpha
filters-mask-luma = Luma
filters-mask-invert = invertieren
filters-speed-x = Geschwindigkeit X (px/s)
filters-speed-y = Geschwindigkeit Y (px/s)
filters-crop-left = links
filters-crop-top = oben
filters-crop-right = rechts
filters-crop-bottom = unten
filters-crop-aria = { $side } zuschneiden


# --- PickerShell.tsx ---
pickershell-refresh-aria = Aktualisieren
pickershell-refresh-title = Die Liste aktualisieren
pickershell-close = Schließen


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Fehler melden
bugreport-intro = Meldungen sind anonym und opt-in — nichts wird automatisch gesendet. Du prüfst den genauen Text unten und sendest ihn dann über ein vorausgefülltes GitHub-Issue oder deine E-Mail-App. Keine persönlichen Daten (dein Home-Pfad und Benutzername werden geschwärzt); kein Konto, kein Server.
bugreport-crash-notice = Freally Capture wurde bei einem früheren Lauf unerwartet geschlossen — die anonymen Absturzdetails sind unten enthalten. Sie zu melden hilft, es schnell zu beheben.
bugreport-description-label = Was hast du getan, als es passierte? (optional)
bugreport-description-placeholder = z. B. die Vorschau fror ein, als ich eine zweite Webcam hinzufügte
bugreport-include-crash = Die anonymen Absturzdetails des letzten Laufs einschließen
bugreport-preview-label = Genau das, was gesendet wird
bugreport-open-github = GitHub-Issue öffnen
bugreport-gmail-title = Öffnet das Gmail-Verfassen-Fenster in deinem Browser, vorausgefüllt. Abgemeldet? Google zeigt zuerst seinen Anmeldebildschirm.
bugreport-compose-gmail = In Gmail verfassen
bugreport-email-title = Öffnet einen Entwurf in der Mail-App, die dieser PC standardmäßig nutzt (Outlook, Thunderbird, Mail…)
bugreport-send-email = E-Mail senden
bugreport-copied = Kopiert ✓
bugreport-copy-report = Bericht kopieren
bugreport-dismiss-crash = Absturz verwerfen
bugreport-copy-failed = Kopieren fehlgeschlagen — markiere den Text und kopiere ihn manuell
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = WAS PASSIERT IST
bugreport-preview-no-description = (keine Beschreibung angegeben)
bugreport-preview-diagnostics = ANONYME DIAGNOSE (keine persönlichen Daten)
bugreport-preview-from = Von: Freally Capture
bugreport-preview-crash-excerpt = --- Absturz-Auszug ---


# --- Updates.tsx ---
updates-title = Software-Update
updates-checking = Suche nach Updates…
updates-uptodate = Du hast die neueste Version.
updates-check-again = Erneut suchen
updates-available = Version { $version } ist verfügbar
updates-current-version = (du hast { $current })
updates-release-notes-label = Version { $version } — Versionshinweise
updates-confirm = Möchtest du jetzt aktualisieren? Der Download wird vor der Anwendung gegen den mitgelieferten Signierschlüssel verifiziert. Freally Capture schließt sich, der Installer läuft, und die neue Version öffnet sich von selbst wieder.
updates-yes-update-now = Ja, jetzt aktualisieren
updates-no-not-now = Nein, nicht jetzt
updates-downloading = { $version } wird heruntergeladen…
updates-starting = startet…
updates-installed = Update installiert.
updates-restart-now = Jetzt neu starten
updates-restart-later = Später neu starten
updates-try-again = Erneut versuchen


# --- Models.tsx ---
models-title = Komponenten
models-ffmpeg-heading = FFmpeg — Wire-Codecs
models-badge-third-party = Drittanbieter · nicht mitgeliefert
models-ffmpeg-desc = Freally Captures eigene Engine nimmt verlustfrei freally-video (.frec) ohne Zusätze auf. Die Aufnahme der Wire-Formate, die Plattformen und Player erwarten — H.264/AAC (und HEVC/AV1) in mp4/mkv/mov/webm — nutzt FFmpeg, ein separates Tool, das diese App nie mitliefert: diese Codecs sind patentbelastet, daher bleibt es optional und klar gekennzeichnet. Es wird bei Bedarf vom unten angehefteten Build heruntergeladen, vor der ersten Nutzung per SHA-256 verifiziert, pro Benutzer zwischengespeichert und als separater Prozess betrieben. Seine Lizenz (LGPL/GPL) ist seine eigene — siehe THIRD-PARTY-NOTICES.
models-checking = Prüfe…
models-ffmpeg-not-installed = Nicht installiert. Verfügbar: FFmpeg { $version } von { $source } ({ $size } Download).
models-ffmpeg-none-pinned = Für diese Plattform ist noch kein FFmpeg-Build angeheftet — Wire-Codec-Aufnahme ist hier nicht verfügbar. Verlustfreie freally-video-Aufnahme ist nicht betroffen.
models-ffmpeg-download-verify = Herunterladen & verifizieren ({ $size })
models-downloading = Wird heruntergeladen…
models-download-of = von
models-cancel = Abbrechen
models-ffmpeg-verifying = Der Download wird gegen den angehefteten SHA-256 verifiziert…
models-ffmpeg-extracting = Wird entpackt…
models-ffmpeg-ready = Installiert & verifiziert — { $version }
models-remove = Entfernen
models-ffmpeg-retry = Download wiederholen
models-network-note = Der Download ist die einzige Netzwerkaktion in diesem Panel und startet nie von selbst. Eine fehlgeschlagene Prüfsumme bricht die Installation ab — die App weigert sich, Bytes auszuführen, für die sie nicht bürgen kann.
models-cef-heading = Browser-Quelle-Laufzeit — Chromium (CEF)
models-cef-desc = Browser-Quellen rendern Webseiten (Benachrichtigungen, Widgets, Overlays) über Chromium Embedded Framework — eine ~100 MB große Laufzeit, die diese App nie mitliefert. Sie wird bei Bedarf vom offiziellen CEF-Build-Index heruntergeladen, vor dem Entpacken gegen die SHA-1 dieses Index verifiziert und pro Benutzer zwischengespeichert. Die Browser-Quelle, die darüber rendert, kommt mit einem eigenen Meilenstein; dies installiert die Laufzeit, die sie braucht.
models-cef-download-install = Herunterladen & installieren
models-cef-unsupported = CEF veröffentlicht keinen Build für diese Plattform — Browser-Quellen sind hier nicht verfügbar.
models-cef-resolving = Löse den neuesten stabilen Build auf…
models-cef-verifying = Der Download wird gegen die Index-SHA-1 verifiziert…
models-cef-extracting = Laufzeit wird entpackt…
models-cef-ready = Installiert — CEF { $version }.
models-cef-retry = Wiederholen
models-integrations-heading = Optionale Integrationen
models-badge-never-bundled = Nie mitgeliefert
models-ndi-detected = Erkannt
models-ndi-not-installed = Nicht installiert
models-vst-available = Verfügbar
models-vst-not-available = Nicht verfügbar


# --- Recordings.tsx ---
recordings-title = Aufnahmen
recordings-loading = Ordner wird gelesen…
recordings-empty = Noch keine Aufnahmen — Aufnahme starten schreibt in den unter Ausgabe festgelegten Ordner.
recordings-frec-label = eigenes verlustfreies (freally-video)
recordings-remux-title = Als mp4 neu verpacken — Stream-Copy, keine Neukodierung, keine Qualitätsänderung (benötigt die FFmpeg-Komponente)
recordings-remuxing = Wird neu verpackt…
recordings-remux-to-mp4 = Zu MP4 remuxen
recordings-export-mp4-title = Das eigene .frec dekodieren und zu MP4 (H.264/AAC) neu kodieren, damit es in jedem Player läuft — benötigt die FFmpeg-Komponente
recordings-exporting = Wird exportiert…
recordings-export-mp4 = Export → MP4
recordings-export-mkv-title = Das eigene .frec dekodieren und zu MKV neu kodieren, damit es in jedem Player läuft
recordings-starting = startet…
recordings-frames = { $done } / { $total } Bilder
recordings-cancel = Abbrechen
recordings-export-cancelled = Export abgebrochen.
recordings-exported-to = Exportiert nach { $path }
recordings-remuxed-to = Neu verpackt nach { $path }


# --- OpenedFrec.tsx ---
openfrec-title = .frec-Aufnahme öffnen
openfrec-desc = Freally Capture nimmt das eigene verlustfreie .frec-Format auf — spielt es aber nicht ab. Freally Player wird .frec direkt abspielen, sobald er veröffentlicht ist. Exportiere es vorerst zu MP4/MKV, dann läuft es in jedem Player (VLC, deinem OS-Player, allem).
openfrec-exported-to = Exportiert nach { $path }
openfrec-exporting = Wird exportiert…
openfrec-starting = startet…
openfrec-export-mp4 = Export → MP4
openfrec-export-mkv = Export → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Vertikale Leinwand (9:16)
vertical-enable = Die zweite Leinwand aktivieren — unabhängig vom Programm aufnehmbar und streambar
vertical-scene-label = Szene, die diese Leinwand zusammensetzt
vertical-width = Breite
vertical-height = Höhe
vertical-preview-alt = Vorschau der vertikalen Leinwand
vertical-note = Element-Positionen sind über Leinwände hinweg pixelgenau: wähle diese Szene in der Szenenleiste, um sie anzuordnen, während diese Vorschau das vertikale Ergebnis zeigt. Stream-Ziele wählen diese Leinwand in ⦿ Stream…; Einstellungen → Ausgabe kann sie parallel zur Hauptdatei aufnehmen.
vertical-close = Schließen


# --- EulaGate.tsx ---
eula-title = Freally Capture — Lizenzvereinbarung
eula-version = v{ $version }
eula-intro = Bitte lies und akzeptiere diese Vereinbarung, um Freally Capture zu nutzen. Kurz gesagt: es ist ein neutrales Werkzeug, und du bist allein verantwortlich für das, was du aufnimmst, aufzeichnest und sendest — und dafür, die Rechte daran zu haben.
eula-thanks = Danke fürs Lesen.
eula-scroll-hint = Scrolle zum Ende, um fortzufahren.
eula-decline = Ablehnen & Beenden
eula-agree = Ich stimme zu


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Ausgabe
output-loading = Einstellungen werden noch geladen…
output-container-frec = freally-video (.frec) — verlustfrei, eigen, nichts herunterzuladen
output-container-mkv = MKV — absturztolerant; später zu mp4 remuxen
output-container-mp4 = MP4 — läuft überall
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Verlustfrei
output-preset-lossless-title = Der eigene freally-video-Codec — bit-genau, kein Download
output-preset-high-label = Hohe Qualität
output-preset-high-title = MP4, bester erkannter Encoder, nahezu verlustfreies CQ 16, Preset Qualität
output-preset-balanced-label = Ausgewogen
output-preset-balanced-title = MKV, bester erkannter Encoder, CQ 23, Preset Ausgewogen
output-recording-format = Aufnahmeformat
output-ffmpeg-warning = Dieses Format benötigt die FFmpeg-Komponente (Wire-Codecs — nicht mitgeliefert). Verlustfreies .frec braucht nichts.
output-install = Installieren…
output-recordings-folder = Aufnahmeordner
output-folder-placeholder = OS-Videoordner
output-filename-prefix = Dateinamen-Präfix
output-frame-rate = Bildrate
output-fps-option = { $fps } fps
output-split-every = Aufteilen alle (Minuten, 0 = aus)
output-output-width = Ausgabebreite (0 = Leinwand; nur Wire-Formate)
output-output-height = Ausgabehöhe (0 = Leinwand)
output-record-vertical = Auch die vertikale Leinwand aufnehmen (eine parallele „… (vertikal)“-Datei; benötigt die aktivierte 9:16-Leinwand)
output-audio-tracks = Audiospuren
output-recorded-tracks-group = Aufgenommene Spuren
output-track-last-one = Mindestens eine Spur muss aufnehmen
output-record-track-on = Spur { $index } aufnehmen: an
output-record-track-off = Spur { $index } aufnehmen: aus
output-encoder-heading = Encoder
output-video-encoder = Video-Encoder
output-encoder-auto = Auto — bester erkannter (H.264)
output-encoder-unavailable = — hier nicht verfügbar
output-preset = Preset
output-preset-quality = Qualität
output-preset-balanced-option = Ausgewogen
output-preset-performance = Leistung
output-rate-control = Ratensteuerung
output-rc-cqp = CQP (konstante Qualität)
output-rc-cbr = CBR (konstante Bitrate)
output-rc-vbr = VBR (variable Bitrate)
output-cq = CQ (0–51, niedriger = besser)
output-bitrate = Bitrate (kbps)
output-keyframe = Keyframe-Intervall (s)
output-audio-bitrate = Audio-Bitrate (kbps / Spur)
output-presets = Presets:

# --- SettingsStream.tsx ---
stream-title = Einstellungen — Stream
stream-target-enabled = Ziel { $index } aktiviert
stream-target = Ziel { $index }
stream-remove = Entfernen
stream-service = Dienst
stream-canvas = Leinwand
stream-canvas-main = Haupt (Programm)
stream-canvas-vertical = Vertikal (9:16 — im Studio aktivieren)
stream-ingest-srt = SRT-Ingest-URL
stream-ingest-whip = WHIP-Endpunkt-URL
stream-ingest-url = Ingest-URL
stream-ingest-override = (Überschreiben — leer = die Dienst-Voreinstellung)
stream-key-srt = streamid (optional — als ?streamid=… angehängt; als Geheimnis behandelt)
stream-key-whip = Bearer-Token (optional — als Authorization-Header gesendet; ein Geheimnis)
stream-key-custom = Streamschlüssel (von deinem Server — als Geheimnis behandelt)
stream-key-service = Streamschlüssel (aus deinem Creator-Dashboard — als Geheimnis behandelt)
stream-key-aria = Streamschlüssel { $index }
stream-key-hide = Ausblenden
stream-key-show = Einblenden
stream-encoder = Encoder (H.264 — was RTMP, SRT und WHIP alle übertragen)
stream-encoder-auto = Auto — der beste erkannte H.264-Encoder
stream-encoder-unavailable = (hier nicht verfügbar)
stream-video-bitrate = Video-Bitrate (kbps, CBR)
stream-audio-bitrate = Audio-Bitrate (kbps)
stream-fps = FPS
stream-keyframe = Keyframe-Intervall (s)
stream-audio-track = Audiospur (1–6)
stream-output-width = Ausgabebreite (0 = Leinwand)
stream-output-height = Ausgabehöhe (0 = Leinwand)
stream-add-target = + Ziel hinzufügen
stream-go-live-note = Live gehen veröffentlicht auf jedem aktivierten Ziel gleichzeitig, direkt an jede Plattform. Ziele mit identischen Encoder-Einstellungen teilen sich eine einzige Kodierung.
stream-auto-record = Aufnahme starten, wenn ich live gehe (die Aufnahme stoppt trotzdem unabhängig)
stream-ffmpeg-note-before = Streaming-Wire-Codecs laufen über die gekennzeichnete bedarfsgesteuerte ffmpeg-Komponente —
stream-ffmpeg-note-link = hier verwalten
stream-ffmpeg-note-after = . Die lokale Aufnahme läuft weiter, egal was der Stream macht.
stream-cancel = Abbrechen
stream-save = Speichern

# --- SettingsReplay.tsx ---
replay-title = Einstellungen — Wiederholungspuffer
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 Min.
replay-length-2min = 2 Min.
replay-length-5min = 5 Min.
replay-quality-low = Niedrig (3 Mbps)
replay-quality-standard = Standard (6 Mbps)
replay-quality-high = Hoch (12 Mbps)
replay-length-presets = Längen-Voreinstellungen
replay-quality-presets = Qualitäts-Voreinstellungen
replay-length-seconds = Länge (Sekunden)
replay-video-bitrate = Video-Bitrate (kbps)
replay-fps = FPS
replay-audio-track = Audiospur (1–6)
replay-note = Während aktiviert läuft der Puffer seine eigene leichtgewichtige Kodierung in einen begrenzten Ring auf der Festplatte — etwa { $mb } MB bei diesen Einstellungen. Speichern fügt den Ring ohne Neukodierung zusammen und berührt weder Stream noch Aufnahme. Änderungen gelten beim nächsten Aktivieren.
replay-cancel = Abbrechen
replay-save = Speichern

# --- SettingsRemote.tsx ---
remote-title = Einstellungen — Fernsteuerung
remote-enable = Die WebSocket-Remote-API aktivieren
remote-password = Passwort (erforderlich — Controller authentifizieren sich damit)
remote-password-placeholder = ein Passwort für deine Controller
remote-password-hide = Ausblenden
remote-password-show = Einblenden
remote-port = Port
remote-allow-lan = LAN-Verbindungen erlauben (Standard ist nur dieser Rechner)
remote-note = Aus = der Port ist geschlossen. An = ein passwortgeschütztes WebSocket auf 127.0.0.1 (oder deinem LAN, wenn aktiviert), das Szenen wechseln, den Übergang ausführen, Stream und Aufnahme starten/stoppen, Wiederholungen speichern und Stummschaltungen/Lautstärken setzen kann — dieselben Aktionen wie die UI, nicht mehr. Es kann keine Dateien lesen. Behandle das Passwort wie jede Anmeldeinformation; bevorzuge nur-dieser-Rechner, außer du steuerst gezielt von einem anderen Gerät.
remote-password-required = Zum Aktivieren der Remote-API ist ein Passwort erforderlich.
remote-cancel = Abbrechen
remote-save = Speichern

# --- SettingsHotkeys.tsx ---
hotkeys-title = Einstellungen — Tastenkürzel
hotkeys-record = Aufnahme starten / stoppen
hotkeys-record-placeholder = z. B. Ctrl+Shift+R
hotkeys-go-live = Live gehen / Stream beenden
hotkeys-go-live-placeholder = z. B. Ctrl+Shift+L
hotkeys-transition = Studiomodus-Übergang
hotkeys-transition-placeholder = z. B. Ctrl+Shift+T oder F13
hotkeys-save-replay = Wiederholung speichern (letzte N Sekunden)
hotkeys-save-replay-placeholder = z. B. Ctrl+Shift+S
hotkeys-add-marker = Kapitelmarker setzen (Aufnahme)
hotkeys-add-marker-placeholder = z. B. Ctrl+Shift+K
hotkeys-note = Tastenkürzel sind global — sie feuern, während andere Apps im Fokus sind. Leer = nicht belegt. Mixer-Push-to-Talk/Mute-Tasten findest du im ⋯-Menü jedes Kanals. Unter Linux/Wayland sind globale Tastenkürzel evtl. nicht verfügbar (eine Compositor-Grenze) — die Schaltflächen funktionieren weiter.
hotkeys-cancel = Abbrechen
hotkeys-save = Speichern

# --- WorkspaceDialog.tsx ---
workspace-title = Profile & Szenensammlungen
workspace-profiles = Profile
workspace-profiles-hint = Ein Profil sind deine Einstellungen — Stream-Ziel, Ausgabe, Tastenkürzel. Wechsle je Show oder je Plattform.
workspace-collections = Szenensammlungen
workspace-collections-hint = Eine Sammlung sind deine Szenen + Quellen. Erstellen dupliziert die aktuelle als Ausgangspunkt.
workspace-active = Aktiv
workspace-switch-to = Zu { $name } wechseln
workspace-active-marker = ● aktiv
workspace-new-name-placeholder = neuer Name…
workspace-new-name-label = Neuer { $title }-Name
workspace-create = Erstellen

# --- OBS import (CAP-M02) ---
workspace-import-obs = Aus OBS importieren…
workspace-import-obs-hint = Eine OBS-Szenensammlung (deren scenes.json) übernehmen. Deine aktuelle Sammlung wird zuvor gespeichert.
workspace-import-busy = Importiere…
workspace-import-title = „{ $name }" importiert
workspace-import-summary = { $scenes } Szenen · { $sources } Quellen · { $items } Elemente
workspace-import-dismiss = Schließen
workspace-import-clean = Alles wurde sauber übernommen.
workspace-import-geometry-caveat = Größen und Positionen werden aus dem OBS-Layout eingepasst – prüfe jede Szene und wähle Aufnahmegeräte neu.
workspace-import-notes-title = Mit Hinweisen importiert
workspace-import-skipped-title = Nicht importiert
import-note-needsReselect = Gerät/Monitor/Fenster neu wählen
import-note-gameCaptureAsWindow = Spielaufnahme → Fensteraufnahme
import-note-referencesFile = Dateipfad prüfen
import-note-filterDropped = Einige Filter nicht unterstützt
import-note-geometryApproximated = Position/Größe angenähert
import-skip-unsupportedKind = Kein passender Quellentyp
import-skip-group = Gruppen noch nicht unterstützt

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Fehlende Dateien neu verknüpfen…
doctor-title = Fehlende Dateien
doctor-scanning = Suche…
doctor-all-good = Alle referenzierten Dateien sind vorhanden. Nichts neu zu verknüpfen.
doctor-intro = { $count } referenzierte Dateien wurden auf diesem Computer nicht gefunden. Weise jeder ihren neuen Ort zu – jede Szene, die sie nutzt, wird sofort korrigiert.
doctor-relinked = { $count } Verweise neu verknüpft.
doctor-uses = { $count }× verwendet
doctor-locate = Suchen…
doctor-locate-folder = In Ordner suchen…
doctor-locate-folder-hint = Ordner wählen; jede fehlende Datei wird per Name zugeordnet und neu verknüpft.
doctor-kind-image = Bild
doctor-kind-media = Medien
doctor-kind-slideshow = Diashow
doctor-kind-font = Schriftart
doctor-kind-lut = LUT
doctor-kind-mask = Maske
history-relinkFiles = Dateien neu verknüpfen

# --- ScriptsDialog.tsx ---
scripts-title = Skripte (Lua)
scripts-empty = Noch keine Skripte — füge eine .lua-Datei hinzu. Siehe scripts/sample.lua für die API: auf Live-/Szenen-/Aufnahmeereignisse reagieren und dieselben Befehle wie die Remote-API steuern.
scripts-enable = { $path } aktivieren
scripts-remove = { $path } entfernen
scripts-path-label = Skriptpfad
scripts-add = Hinzufügen
scripts-note = Skripte laufen in einer Sandbox — kein Datei- oder OS-Zugriff; sie können nur dieselben Studio-Befehle aufrufen wie die Remote-API (Szenen wechseln, Übergang, Aufnahme/Stream/Wiederholung, Stummschaltungen). Ein Skriptfehler wird protokolliert und eingegrenzt. Änderungen gelten binnen einer Sekunde.
scripts-error-not-lua = Zeige auf eine .lua-Datei.

# --- BrowserDock.tsx ---
browser-dock-title = Browser-Docks
browser-dock-empty = Noch keine Docks — füge ein Chat-Popout, eine Benachrichtigungsseite oder deine Companion-Webschaltflächen hinzu.
browser-dock-open = Öffnen
browser-dock-remove = { $name } entfernen
browser-dock-name-placeholder = Name (z. B. Twitch-Chat)
browser-dock-name-label = Dock-Name
browser-dock-url-label = Dock-URL
browser-dock-note = Ein Dock öffnet sich als eigenes Fenster, das du neben das Studio setzen kannst. Die Seite erhält keinen Zugriff auf die App — sie rendert nur. Nur http(s)-URLs; Docks öffnen nur, wenn du auf Öffnen klickst.
browser-dock-error-name = Benenne das Dock (z. B. Twitch-Chat).
browser-dock-error-url = Eine Dock-URL muss mit http:// oder https:// beginnen.

# --- studio-preview-pane ---
studio-preview-label = Studiomodus-Vorschau
studio-preview-heading = Vorschau
studio-preview-hint = Klicke auf eine Szene, um sie hier zu laden
studio-preview-empty = Die Vorschau erscheint hier.
studio-preview-mirrors = spiegelt das Programm
studio-preview-transition-select = Übergang
studio-preview-duration = Übergangsdauer (ms)
studio-preview-commit-title = Vorschau → Programm über den Übergang übernehmen (das Publikum sieht es)
studio-preview-transitioning = Übergang läuft…
studio-preview-transition-button = Übergang ⇄
studio-preview-luma-placeholder = Graustufen-Wischbild (png/jpg)
studio-preview-luma-label = Luma-Wischbild
studio-preview-browse = Durchsuchen…
studio-preview-filter-images = Bilder
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = Stinger-Video (ProRes 4444 .mov behält den Alphakanal)
studio-preview-stinger-label = Stinger-Videodatei
studio-preview-stinger-cut-label = Stinger-Schnittpunkt (ms)
studio-preview-stinger-cut-title = Wann der Szenenwechsel unter dem Stinger stattfindet (ms nach Übergangsbeginn)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Schnitt
transition-kind-fade = Überblenden
transition-kind-slide-left = Schieben ←
transition-kind-slide-right = Schieben →
transition-kind-slide-up = Schieben ↑
transition-kind-slide-down = Schieben ↓
transition-kind-swipe-left = Wischen ←
transition-kind-swipe-right = Wischen →
transition-kind-luma-linear = Luma-Wischen (linear)
transition-kind-luma-radial = Luma-Wischen (radial)
transition-kind-luma-horizontal = Luma-Wischen (horizontal)
transition-kind-luma-diamond = Luma-Wischen (Raute)
transition-kind-luma-clock = Luma-Wischen (Uhr)
transition-kind-image = Bild-Wischen (benutzerdefiniert)
transition-kind-stinger = Stinger (Video)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Benutzerdefiniert (RTMP/RTMPS)
stream-service-srt = SRT (selbstgehostet)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Über
about-tagline = Aufnehmen und streamen wie ein Studio — keine Konten, keine Cloud.
about-version = Version
about-created-by = Erstellt von
about-project-started = Projektbeginn
about-first-stable = Erste stabile Version
about-first-stable-pending = Noch nicht — 1.0.0 ist in Arbeit
about-platform = Plattform
about-local-first = Freally Capture läuft vollständig auf deinem Rechner. Keine Konten, keine Telemetrie, keine Cloud — das Einzige, was deinen Computer verlässt, ist der Stream, den du zum Senden gewählt hast.
about-website = Website
about-issues = Ein Problem melden
about-license = Lizenz
about-eula = EULA
about-third-party = Drittanbieter-Hinweise
about-check-updates = Nach Updates suchen…

# --- unified settings modal (TASK-906) ---
settings-title = Einstellungen
settings-language-section = Sprache
settings-language = Oberflächensprache
settings-language-system = Systemstandard
settings-language-note = Eine hier gewählte Sprache wird gespeichert. „Systemstandard“ folgt deinem Betriebssystem. Nicht übersetzter Text fällt auf Englisch zurück.
settings-appearance-section = Darstellung
settings-theme = Design
settings-theme-dark = Dunkel
settings-theme-light = Hell
settings-theme-custom = Benutzerdefiniert
settings-accent = Akzent
settings-general-section = Allgemein
settings-show-stats-dock = Statistik-Dock einblenden
settings-more-section = Weitere Einstellungen
settings-open-output = Aufnahme…
settings-open-stream = Streaming…
settings-open-replay = Wiederholung…
settings-open-hotkeys = Tastenkürzel…
settings-open-remote = Remote-API…
settings-open-about = Über…
controls-settings = ⚙ Einstellungen…
controls-settings-title = Sprache, Darstellung und die app-weiten Einstellungen

# --- command palette (TASK-904) ---
palette-title = Befehlspalette
palette-search = Szenen, Quellen und Aktionen durchsuchen
palette-placeholder = Szenen, Quellen, Aktionen durchsuchen…
palette-no-results = Nichts passt zu “{ $query }”
palette-hint = ↑ ↓ zum Navigieren · Enter zum Ausführen · Esc zum Schließen
palette-group-scenes = Szene
palette-group-sources = Quelle
palette-group-actions = Aktion
palette-transition = Übergang Vorschau → Programm
palette-save-replay = Wiederholung speichern
palette-add-marker = Kapitelmarker setzen
palette-vertical-canvas = Vertikale (9:16) Leinwand…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Willkommen bei Freally Capture
wizard-welcome = Zwei kurze Schritte: schau, was dein Rechner leisten kann, und starte dann eine Szene. Das dauert etwa dreißig Sekunden, und du kannst später alles ändern.
wizard-local-first = Nichts davon verlässt deinen Computer. Freally Capture hat keine Konten, keine Telemetrie und keine Cloud.
wizard-start = Los geht's
wizard-skip = Überspringen
wizard-hardware-title = Was dein Rechner leisten kann
wizard-probing = Deine Grafikkarte und dein Prozessor werden geprüft…
wizard-encoder = Encoder
wizard-canvas = Leinwand
wizard-bitrate = Bitrate
wizard-probe-found = Gefunden: { $gpus } · { $cores } physische Kerne
wizard-no-gpu = keine dedizierte GPU
wizard-apply = Diese Einstellungen verwenden
wizard-keep-current = Behalten, was ich habe
wizard-template-title = Mit einer Szene beginnen
wizard-template-screen = Meinen Bildschirm aufnehmen
wizard-template-screen-note = Fügt eine Bildschirmaufnahme deines Hauptmonitors hinzu. Der häufigste Startpunkt.
wizard-template-empty = Leer beginnen
wizard-template-empty-note = Eine leere Szene. Füge Quellen selbst mit der +-Schaltfläche hinzu.
wizard-done = Alles eingerichtet.
wizard-done-hint = Drücke jederzeit Ctrl+K, um Szenen, Quellen und Aktionen zu durchsuchen. Die Einstellungen liegen hinter der ⚙-Schaltfläche.
wizard-close = Streaming starten

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Deine Grafikkarte kann Video selbst kodieren, was den Prozessor für den Rest des Studios frei lässt.
autoconfig-reason-software = Kein nutzbarer Hardware-Encoder gefunden, also kodiert der Prozessor. Das funktioniert, es kostet nur mehr CPU.
autoconfig-reason-quality-hardware = 1080p mit 60 Bildern pro Sekunde, bei einer Bitrate, die jede große Plattform akzeptiert.
autoconfig-reason-quality-software = 30 Bilder pro Sekunde, weil Software-Kodierung bei 60 auf den meisten Prozessoren Bilder verwirft.
autoconfig-reason-quality-low-cores = Eine niedrigere Bitrate, weil dieser Prozessor wenige Kerne hat und die Software-Kodierung mit dem Compositor um sie konkurriert.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Aufnahme gestartet
announce-recording-paused = Aufnahme pausiert
announce-recording-stopped = Aufnahme gestoppt
announce-live-started = Du bist live
announce-live-ended = Stream beendet
announce-reconnecting = Verbindung verloren, verbinde neu
announce-stream-failed = Stream fehlgeschlagen
announce-frames-dropped = { $count } Bilder verworfen

# CAP-M01 — undo/redo edit history
palette-undo = Rückgängig
palette-redo = Wiederholen
palette-edit-history = Bearbeitungsverlauf…
history-title = Bearbeitungsverlauf
history-empty = Noch nichts zum Rückgängigmachen.
history-current = Aktueller Stand
history-close = Schließen
history-addScene = Szene hinzufügen
history-renameScene = Szene umbenennen
history-removeScene = Szene entfernen
history-reorderScene = Szenen neu anordnen
history-addSource = Quelle hinzufügen
history-removeSource = Quelle entfernen
history-reorderSource = Quellen neu anordnen
history-renameSource = Quelle umbenennen
history-transformSource = Quelle verschieben
history-toggleVisibility = Sichtbarkeit umschalten
history-toggleLock = Sperre umschalten
history-setBlendMode = Mischmodus ändern
history-editSourceProperties = Eigenschaften bearbeiten
history-applyLayout = Layout anordnen
history-moveToSeat = Auf Platz setzen
history-groupSources = Quellen gruppieren
history-ungroupSources = Gruppierung aufheben
history-toggleGroupVisibility = Gruppe umschalten
history-setSceneAudio = Szenen-Audio
history-setVerticalCanvas = Vertikale Leinwand
history-addFilter = Filter hinzufügen
history-removeFilter = Filter entfernen
history-reorderFilter = Filter neu anordnen
history-editFilter = Filter bearbeiten
history-toggleFilter = Filter umschalten
history-setVolume = Lautstärke anpassen
history-toggleMute = Stumm umschalten
history-setMonitor = Monitoring ändern
history-setTracks = Spuren ändern
history-setSyncOffset = A/V-Sync anpassen
history-setAudioHotkeys = Audio-Tastenkürzel

# CAP-M04 — alignment aids
settings-alignment-section = Ausrichtungshilfen
settings-smart-guides = Intelligente Hilfslinien (beim Ziehen einrasten)
settings-safe-areas = Sichtbereichs-Overlays
settings-rulers = Lineale
align-group = Am Canvas ausrichten
align-left = Links ausrichten
align-hcenter = Horizontal zentrieren
align-right = Rechts ausrichten
align-top = Oben ausrichten
align-vcenter = Vertikal zentrieren
align-bottom = Unten ausrichten

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Auswahl ausrichten & verteilen
arrange-left = Linke Kanten ausrichten
arrange-hcenter = Horizontal zentrieren
arrange-right = Rechte Kanten ausrichten
arrange-top = Obere Kanten ausrichten
arrange-vcenter = Vertikal zentrieren
arrange-bottom = Untere Kanten ausrichten
distribute-h = Horizontal verteilen
distribute-v = Vertikal verteilen
guides-group = Hilfslinien
guides-add-v = Vertikale Hilfslinie hinzufügen
guides-add-h = Horizontale Hilfslinie hinzufügen
history-arrangeItems = Elemente anordnen
history-editGuides = Hilfslinien bearbeiten

# CAP-M05 — edit transform + copy/paste
transform-title = Transformation bearbeiten — { $name }
transform-anchor = Ankerpunkt
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Drehung
transform-crop = Zuschnitt
transform-crop-left = Links
transform-crop-top = Oben
transform-crop-right = Rechts
transform-crop-bottom = Unten
transform-no-size = Größe und Zuschnitt sind verfügbar, sobald die Quelle ihre Maße meldet.
transform-copy = Transformation kopieren
transform-paste = Transformation einfügen
transform-close = Schließen
filters-copy = Filter kopieren ({ $count })
filters-paste = Filter einfügen ({ $count })
palette-edit-transform = Transformation bearbeiten…
history-pasteFilters = Filter einfügen

# CAP-M26 — keying workbench
workbench-title = Keying-Werkbank — { $name }
workbench-mode-keyed = Gekeyt
workbench-mode-source = Quelle
workbench-mode-matte = Matte
workbench-mode-split = Geteilt
workbench-eyedropper = Pipette
workbench-eyedropper-hint = Klicken Sie auf die Quelle, um die Key-Farbe zu übernehmen.
workbench-loupe = Lupe
workbench-split = Teilung
workbench-preview-alt = Vorschau der Keying-Werkbank
workbench-tune = Anpassen
workbench-close = Schließen

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Klicken Sie auf eine Szene, um zu ihr zu schneiden.
multiview-hint-stage = Klicken Sie auf eine Szene, um sie in der Vorschau vorzubereiten.
palette-multiview = Multiview-Monitor

# CAP-M07 — projectors
projector-title = Projektor öffnen
projector-source = Quelle
projector-target-program = Programm
projector-target-preview = Vorschau
projector-target-scene = Szene…
projector-target-source = Quelle…
projector-target-multiview = Multiview
projector-which-scene = Welche Szene
projector-which-source = Welche Quelle
projector-none = Nichts zum Anzeigen
projector-display = Bildschirm
projector-windowed = Schwebendes Fenster (dieser Bildschirm)
projector-display-option = Bildschirm { $n } — { $w }×{ $h }
projector-primary = (primär)
projector-open = Öffnen
projector-cancel = Abbrechen
projector-exit-hint = Zum Beenden Esc drücken
palette-projector = Projektor öffnen…

# CAP-M08 — still-frame grab
palette-still = Standbild aufnehmen…
still-saved-toast = Standbild gespeichert: { $name }
still-failed-toast = Standbildaufnahme fehlgeschlagen: { $error }
hotkeys-still = Standbild aufnehmen
hotkeys-still-placeholder = z. B. Ctrl+Shift+P
