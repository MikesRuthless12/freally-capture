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
sources-badge-test-bars = Balken
sources-badge-test-grid = Gitter
sources-badge-test-sweep = Sweep
sources-badge-test-tone = Ton
sources-badge-test-sync = Sync
sources-badge-timer = Timer

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
sources-add-timer = Timer / Uhr
sources-add-nested-scene = Verschachtelte Szene
sources-add-slideshow = Bild-Diashow
sources-add-chat-overlay = Live-Chat-Overlay
sources-add-test-signal = Testsignal
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
sources-testsignal-title = Testsignal hinzufügen
sources-testsignal-pattern-label = Muster
sources-testsignal-bars = SMPTE-Farbbalken
sources-testsignal-grid = Kalibrierungsgitter
sources-testsignal-sweep = Bewegungs-Sweep
sources-testsignal-tone = 1-kHz-Ton (−20 dBFS)
sources-testsignal-flash-beep = A/V-Sync Blitz + Piepton
sources-testsignal-note = Szenen, Encoder, Projektoren und Streamziele ohne angeschlossene Kamera prüfen. Das Blitz-und-Piepton-Muster speist die A/V-Sync-Werkbank.
sources-testsignal-add = Testsignal hinzufügen
sources-timer-title = Timer hinzufügen
sources-timer-mode-label = Modus
sources-timer-wall-clock = Uhrzeit
sources-timer-countdown = Countdown
sources-timer-stopwatch = Stoppuhr
sources-timer-since-live = Zeit seit Live
sources-timer-since-recording = Zeit seit Aufnahme
sources-timer-note = Dauer, Format, Gestaltung und Countdown-Ende-Aktionen liegen in den Eigenschaften der Quelle.
sources-timer-add = Timer hinzufügen

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
mixer-routing = Routing
mixer-routing-title = Audioausgabe-Routing

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Audio-Routing
routing-intro = Weisen Sie Kanalzüge den Spur-Bussen zu und senden Sie dann einen beliebigen Bus an einen physischen Ausgang – eine Zuspielung für einen Hardware-Recorder, Lautsprecher in einem anderen Raum oder einen Kopfhörer-Cue auf einer freien Spur. Der Monitor behält sein eigenes Gerät; diese Routen kommen obendrauf, sodass der Mix unverändert bleibt, wenn keine gesetzt ist.
routing-sends-title = Spur-Sends
routing-no-strips = Keine Audioquellen in dieser Szene.
routing-source = Quelle
routing-track = Spur { $n }
routing-send-aria = { $source } an Spur { $n } senden
routing-outputs-title = Physische Ausgänge
routing-master = Master
routing-off = Aus
routing-default-output = Standardausgabe
routing-device-aria = Ausgabegerät für { $bus }
routing-trim-aria = Ausgangs-Trim für { $bus }
routing-trim-db = { $db } dB
routing-muted = Stumm
routing-device-error = Gerät nicht verfügbar

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Ducking
mixer-ducking-title = Ducking-Matrix
ducking-title = Ducking-Matrix
ducking-intro = Jede Quelle kann jede andere ducken. Eine Zelle senkt das Ziel (Spalte) ab, sobald der Auslöser (Zeile) spricht – wähle eine Zelle, um Tiefe, Schwelle und Timing festzulegen. Jedes Paar ist ein eigenes Ducking, sodass ein Kanalzug von mehreren Auslösern gleichzeitig geduckt werden kann.
ducking-need-two = Füge mindestens zwei Audioquellen hinzu, um zwischen ihnen zu ducken.
ducking-trigger-target = Auslöser ↓ / Ziel →
ducking-cell-aria = { $trigger } duckt { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Entfernen
ducking-amount = Menge
ducking-threshold = Schwelle
ducking-attack = Attack
ducking-release = Release
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Lautheitsnormalisierung
loudness-intro = Führt das Programm sanft auf ein Lautheitsziel mit Spitzenpegel-Grenze, damit Stream und Aufnahmen auf einem gleichbleibenden Pegel landen. Langsam und behutsam — es steuert, es pumpt nie.
loudness-enable = Programm auf das Ziel führen
loudness-target = Ziel
loudness-target-option = { $target } LUFS
loudness-ceiling = Spitzenpegel-Grenze (dBFS)
loudness-note = −14 LUFS eignet sich für YouTube-artige Wiedergabe; −16 ist ein gängiges Streaming-Ziel; −23 ist EBU R128 für den Rundfunk. Dasselbe Ziel wird von der Normalisieren-Aktion nach der Aufnahme verwendet.
loudness-on = LUFS { $target }
loudness-off = Norm. aus

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Soundboard
mixer-soundboard-title = Soundboard
soundboard-title = Soundboard
soundboard-add-pad = + Pad
soundboard-stop-all = Alle stoppen
soundboard-edit = Bearbeiten
soundboard-empty = Noch keine Pads — füge eins hinzu und weise ihm einen lokalen Audioclip zu.
soundboard-new-pad = Neues Pad
soundboard-no-clip = Kein Clip
soundboard-audio-files = Audiodateien
soundboard-name = Name
soundboard-choose-clip = Clip wählen…
soundboard-gain = Verstärkung
soundboard-choke = Choke
soundboard-choke-none = Keine
soundboard-loop = Schleife
soundboard-auto-duck = Auto-Ducking
soundboard-tracks = Spuren
soundboard-hotkey = Hotkey
soundboard-hotkey-placeholder = z. B. Ctrl+Shift+1
soundboard-remove = Entfernen

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Plugins
mixer-plugins-title = Audio-Plugins (CLAP / VST3)
plugins-title = Audio-Plugins
plugins-scanning = Scannen…
plugins-none = Keine CLAP- oder VST3-Plugins in den Standardordnern gefunden.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Speicher
stats-dropped = Verworfen
stats-render = Rendern
stats-gpu = GPU
stats-gpu-compositing = kompositiert
stats-gpu-idle = inaktiv
stats-disk = Datenträger
stats-disk-free = frei
stats-disk-left = Aufn. übrig
stats-disk-rate = ≈ { $rate } MB/s Aufnahme
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
channelstrip-solo-title = Solo (PFL) — der Monitor hört nur soloierte Kanäle; die Programmmischung bleibt unberührt
channelstrip-solo-source = { $name } solo schalten (PFL)
channelstrip-pan-label = Balance (Doppelklick setzt zurück)
channelstrip-pan-aria = Balance von { $name }
channelstrip-mono-label = Auf Mono heruntermischen
channelstrip-automix-label = Automix (Pegelaufteilung)
channelstrip-automix-note = Pegelaufteilung: Der Mixer hält den Summenpegel aller Automix-Kanäle konstant und übergibt ihn an denjenigen, der gerade spricht — ideal für Panels mit mehreren Mikros und für Podcasts. Aus, bis du einen Kanal hinzufügst.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Erzeugt einen echofreien Rückweg für diese Quelle — alle im Programm außer dieser Quelle selbst. Nutze ihn für einen Remote-Gast, damit er seine eigene verzögerte Stimme nicht hört.
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
properties-testtone-note = Ein durchgehender 1-kHz-Sinuston bei −20 dBFS. Pegel und Stummschaltung liegen auf dem Mixer-Kanal; mehr ist nicht einzustellen.
properties-timer-format = Zeitformat (strftime)
properties-timer-format-note = z. B. %H:%M:%S (Standard), %I:%M %p, %A %H:%M — ein ungültiges Muster fällt auf %H:%M:%S zurück.
properties-timer-utc = UTC-Versatz (Minuten)
properties-timer-utc-placeholder = Ortszeit
properties-timer-duration = Dauer (Sekunden)
properties-timer-target = Herunterzählen bis (HH:MM)
properties-timer-target-note = Ein Uhrzeit-Ziel läuft von selbst und wiederholt sich täglich; leer lassen, um die Dauer mit Start/Pause/Reset zu nutzen.
properties-timer-end = Bei null
properties-timer-end-none = Nichts tun
properties-timer-end-flash = Timer blinken lassen
properties-timer-end-switch = Szene wechseln
properties-timer-end-scene = Szene
properties-timer-size = Größe (px)
properties-timer-start = Start
properties-timer-pause = Pause
properties-timer-reset = Zurücksetzen
properties-text-file = Aus Datei lesen (Pfad; leer = Text oben verwenden)
properties-text-binding = Interpretieren als
properties-text-binding-whole = Ganze Datei
properties-text-binding-csv = CSV-Zelle
properties-text-binding-json = JSON-Pointer
properties-text-csv-row = Zeile
properties-text-csv-column = Spalte
properties-text-csv-column-placeholder = Name oder Nummer
properties-text-json-pointer = Pointer
properties-text-file-note = Die Datei wird binnen einer halben Sekunde nach einer Änderung neu gelesen. Atomare Schreiber (Temp + Umbenennen) sind toleriert: der letzte gute Wert bleibt während des Tauschs sichtbar.
avsync-title = A/V-Sync-Kalibrierung
avsync-intro = Spiele das eingebaute Blitz-+-Piepton-Muster über Display und Lautsprecher ab und nimm es mit der Kamera und dem Mikrofon auf, die du ausrichten willst — die Werkbank misst die Lücke dazwischen. Die Schleife läuft über Bildschirm und Lautsprecher, deren kleine Latenzen sind also enthalten.
avsync-video-label = Kamera (Videoquelle)
avsync-audio-label = Mikrofon (Audioquelle)
avsync-pick = Quelle wählen…
avsync-no-video = Füge die Kamera zuerst als Quelle hinzu — die Werkbank misst Quellen, keine rohen Geräte.
avsync-no-audio = Füge das Mikrofon zuerst als Audioquelle hinzu.
avsync-projector = Programm im Vollbild anzeigen auf
avsync-projector-open = Projektor öffnen
avsync-projector-window-title = Programm — A/V-Sync
avsync-start-note = Der Start legt vorübergehend eine „A/V-Sync-Muster“-Quelle über die aktuelle Szene und spielt den Piepton auf dem Monitorgerät. Am Ende des Laufs wird alles entfernt.
avsync-manual = Sync-Versatz (ms, manuell)
avsync-start = Kalibrierung starten
avsync-measuring = Es wird ca. 12 Sekunden gemessen — richte die Kamera auf das blitzende Programm und halte den Raum ruhig…
avsync-flash-seen = Kamera sieht den Blitz
avsync-flash-waiting = Warte, bis die Kamera den Blitz sieht…
avsync-beep-heard = Mikrofon hört den Piepton
avsync-beep-waiting = Warte, bis das Mikrofon den Piepton hört…
avsync-cancel = Abbrechen
avsync-result-offset = Das Video kommt { $offset } ms nach dem Audio an.
avsync-result-detail = Gemessen über { $cycles } Zyklen, ±{ $jitter } ms.
avsync-negative = Das Audio kommt bereits später an als das Video. Audio zu verzögern kann diese Richtung nicht beheben — trägt ein anderer Kanal den Ton dieser Kamera, senke dort den Sync-Versatz.
avsync-over-cap = Die gemessene Lücke liegt über der Sync-Versatz-Grenze von { $max } ms. So eine Lücke heißt meist: falsche Quelle gewählt — Kette prüfen und neu messen.
avsync-applied = Übernommen — der Sync-Versatz des Mikrofons ist jetzt { $offset } ms.
avsync-apply = { $offset } ms auf das Mikrofon anwenden
avsync-again = Erneut messen
avsync-close = Schließen
avsync-error-noFlash = Die Kamera hat den Blitz nie gesehen. Richte sie auf das blitzende Programm (Vollbild hilft), stelle sicher, dass die Quelle live ist, und miss erneut.
avsync-error-noBeep = Das Mikrofon hat den Piepton nie gehört. Stelle sicher, dass das Monitorgerät hörbar und das Mikrofon live ist (nicht durch Push-to-Talk stumm), und miss erneut.
avsync-error-tooFewCycles = Zu wenige saubere Blitz-/Piepton-Zyklen erfasst. Halte das Muster den ganzen Lauf über klar sichtbar und hörbar.
avsync-error-notThePattern = Das Gesehene/Gehörte wiederholt sich nicht im Rhythmus des Musters — vermutlich Raumlicht oder Lärm, nicht das Testsignal.
avsync-error-unstable = Die Zyklen widersprechen sich zu stark für eine einzige Zahl. Kamera stabilisieren, Raumgeräusche senken, erneut messen.
hotkey-audit-title = Hotkey-Übersicht
hotkey-audit-search = Suche
hotkey-audit-filter = Funktion
hotkey-audit-filter-all = Alle Funktionen
hotkey-audit-col-key = Taste
hotkey-audit-col-action = Aktion
hotkey-audit-col-where = Wo
hotkey-audit-col-status = Status
hotkey-audit-ok = OK
hotkey-audit-shared = Von { $count } Belegungen geteilt
hotkey-audit-unregistered = Nicht beim OS registriert (anderweitig belegt oder nicht verfügbar)
hotkey-audit-invalid = Kein gültiges Tastenkürzel
hotkey-audit-empty = Noch keine Hotkeys — in Einstellungen → Hotkeys oder am Mixer-Kanal belegen.
hotkey-audit-export = Spickzettel exportieren
hotkey-audit-exported = Gespeichert unter { $path }
hotkey-audit-note = Tasten belegt und geändert werden in Einstellungen → Hotkeys (globale Aktionen) und an jedem Mixer-Kanal (Push-to-Talk / Push-to-Mute); diese Tabelle prüft und dokumentiert sie.
hotkey-audit-action-record = Aufnahme umschalten
hotkey-audit-action-go-live = Streaming umschalten
hotkey-audit-action-transition = Übergang ausführen
hotkey-audit-action-save-replay = Replay speichern
hotkey-audit-action-add-marker = Marker setzen
hotkey-audit-action-still = Standbild aufnehmen
hotkey-audit-action-panic = Panik-Folie
hotkey-audit-action-timer-toggle = Alle Timer starten/pausieren
hotkey-audit-action-timer-reset = Alle Timer zurücksetzen
hotkey-audit-action-ptt = Push-to-Talk
hotkey-audit-action-ptm = Push-to-Mute
hotkey-audit-feature-recording = Aufnahme
hotkey-audit-feature-streaming = Streaming
hotkey-audit-feature-studio = Studio-Modus
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Marker
hotkey-audit-feature-stills = Standbilder
hotkey-audit-feature-panic = Panik
hotkey-audit-feature-timers = Timer
hotkey-audit-feature-audio = Audio (pro Quelle)
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
properties-deinterlace = Deinterlacing
properties-deinterlace-off = Aus
properties-deinterlace-discard = Verwerfen (ein Feld zeilenverdoppeln)
properties-deinterlace-bob = Bob (Felder abwechseln)
properties-deinterlace-linear = Linear (interpolieren)
properties-deinterlace-blend = Mischen (Felder mitteln)
properties-deinterlace-adaptive = Bewegungsadaptiv (yadif-Klasse)
properties-field-order = Halbbildreihenfolge
properties-field-order-top = Oberes Feld zuerst
properties-field-order-bottom = Unteres Feld zuerst
properties-deinterlace-note = Für Interlaced-Capture-Karten-Signale. Reine CPU, auf jedem OS identisch; eine Änderung startet das Gerät neu (wie ein Formatwechsel).
camera-controls-title = Kamerasteuerung
camera-controls-refresh = Aktualisieren
camera-controls-reset = Profil zurücksetzen
camera-controls-empty = Gerade keine Regler — das Gerät muss streamen (erst einer Szene hinzufügen), und manche Backends melden keine (v. a. macOS). Das ist der ehrliche Stand pro OS.
camera-controls-note = Änderungen wirken live und landen im Geräteprofil, das bei Hotplug und Neustart erneut angewendet wird.
camera-control-brightness = Helligkeit
camera-control-contrast = Kontrast
camera-control-hue = Farbton
camera-control-saturation = Sättigung
camera-control-sharpness = Schärfe
camera-control-gamma = Gamma
camera-control-white-balance = Weißabgleich
camera-control-backlight = Gegenlichtausgleich
camera-control-gain = Verstärkung
camera-control-pan = Schwenken
camera-control-tilt = Neigen
camera-control-zoom = Zoom
camera-control-exposure = Belichtung
camera-control-iris = Blende
camera-control-focus = Fokus
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
audiofilters-name-parametric-eq = Parametrischer EQ
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Rumpelfilter
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Preset
audiofilters-voice-preset-pick = Stimm-Preset…
audiofilters-voice-broadcast = Broadcast-Stimme
audiofilters-voice-podcast = Podcast-Stimme
audiofilters-voice-clean = Klare Stimme
audiofilters-voice-none = Kette leeren
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Zischlaut-Frequenz (Hz)
audiofilters-deesser-amount = Max. Absenkung (dB)
audiofilters-rumble-freq = Low-Cut (Hz)
audiofilters-title = Audiofilter — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Frequenzgang des parametrischen EQ mit Live-Spektrum
eq-band-type = Typ
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Band
eq-remove-band = Band entfernen
eq-type-bell = Glocke
eq-type-lowShelf = Low-Shelf
eq-type-highShelf = High-Shelf
eq-type-notch = Notch
eq-type-highPass = Hochpass
eq-type-lowPass = Tiefpass
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
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galerie
filters-shader-gallery-pick = Voreinstellung laden…
filters-shader-gallery-grayscale = Graustufen
filters-shader-gallery-invert = Invertieren
filters-shader-gallery-scanlines = Scanlinien
filters-shader-gallery-vignette = Vignette
filters-shader-source = Shader-Quelltext (WGSL)
filters-shader-hint = Schreiben Sie ein WGSL-effect(uv, color, p, texel, time), das ein vec4 zurückgibt. Kennzeichnen Sie Parameter mit // @param name min max default für Schieberegler. Ein ungültiger Shader wird ignoriert — die Quelle wird ungefiltert dargestellt, bis er kompiliert.
filters-name-bezier-mask = Bézier-Maske
filters-mask-editor-hint = Ziehen Sie einen Punkt, um ihn zu verschieben, doppelklicken Sie, um einen hinzuzufügen, Rechtsklick auf einen Punkt entfernt ihn.
filters-mask-shape = Form
filters-mask-shape-pick = Voreinstellung…
filters-mask-shape-rectangle = Rechteck
filters-mask-shape-diamond = Raute
filters-mask-shape-hexagon = Sechseck
filters-mask-shape-circle = Kreis
filters-mask-feather = Weiche Kante
filters-mask-export-wipe = Als Wischblende exportieren…
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
recordings-normalize = Normalisieren
recordings-normalizing = Normalisieren…
recordings-normalize-title = Lautheit auf das Ziel normalisieren (schreibt eine Kopie)
recordings-normalized-to = Normalisiert nach { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Nur Audio
audiorec-format = Audioaufnahmeformat
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Audio aufnehmen
audiorec-stop = Stopp
audiorec-pause = Pause
audiorec-resume = Fortsetzen
audiorec-recording = REC { $sec }s
audiorec-saved = { $count } Spurdatei(en) gespeichert


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
output-recording-template = Dateiname für Aufnahmen
output-replay-template = Dateiname für Wiederholungen
output-still-template = Dateiname für Standbilder
output-template-tokens = Platzhalter: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Wiederholungsordner
output-still-folder = Standbildordner
output-same-folder-placeholder = Aufnahmeordner
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
hotkeys-go-live = Live gehen / Stream beenden
hotkeys-transition = Studiomodus-Übergang
hotkeys-save-replay = Wiederholung speichern (letzte N Sekunden)
hotkeys-add-marker = Kapitelmarker setzen (Aufnahme)
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
studio-preview-stinger-matte-label = Track-Matte
studio-preview-stinger-matte-title = Wie ein Track-Matte-Stinger die Transparenz packt: die Füllung und ihre Matte nebeneinander (horizontal) oder übereinander (vertikal)
studio-preview-stinger-duck-label = Programm ducken
studio-preview-stinger-duck-title = Das Programm-Audio unter dem eigenen Audio des Stingers ducken, während er läuft (0 = aus)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = Bewegen (Morph)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Keine
stinger-matte-horizontal = Nebeneinander
stinger-matte-vertical = Übereinander

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
settings-open-about = Über…

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
guides-clear = Alle Hilfslinien entfernen
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

# CAP-M13 — source health dashboard
palette-source-health = Quellenzustand…
palette-av-sync = A/V-Sync-Kalibrierung…
palette-hotkey-audit = Hotkey-Übersicht…
health-title = Quellenzustand
health-col-source = Quelle
health-col-state = Status
health-col-resolution = Auflösung
health-col-fps = FPS
health-col-last-frame = Letztes Bild
health-col-dropped = Verworfen
health-col-retries = Neustarts
health-col-actions = Aktionen
health-state-live = Live
health-state-waiting = Wartet
health-state-error = Fehler
health-state-inactive = Inaktiv
health-restart = Neu starten
health-properties = Eigenschaften
health-empty = Diese Sammlung hat noch keine Quellen.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture beenden?
quit-body = Beim Beenden wird jetzt sicher und in dieser Reihenfolge Folgendes ausgeführt:
quit-consequence-stream = Den Livestream beenden und die Verbindung zum Dienst trennen.
quit-consequence-recording = Die Aufnahme stoppen und ihre Datei(en) finalisieren.
quit-consequence-replay = Den Wiederholungspuffer herunterfahren — nicht gespeichertes Material wird verworfen.
quit-confirm = Sicher beenden
quit-quitting = Wird heruntergefahren…
quit-cancel = Abbrechen

# CAP-M11 — crash-safe recording salvage
salvage-title = Unterbrochene Aufnahmen wiederherstellen?
salvage-body = Die letzte Sitzung endete unerwartet, während diese Aufnahmen noch geschrieben wurden. Die Reparatur legt eine abspielbare Kopie neben dem Original an — die Originaldatei bleibt unverändert.
salvage-repair = Reparieren
salvage-repairing = Wird repariert…
salvage-done = Repariert
salvage-repaired = Repariert → { $name }
salvage-failed = Reparatur fehlgeschlagen: { $error }
salvage-dismiss = Nicht jetzt

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Encoder-Fehler — von { $from } auf { $to } gewechselt. Der Stream hat sich neu verbunden und läuft weiter.
fallback-toast-recording = Encoder-Fehler — von { $from } auf { $to } gewechselt. Die Aufnahme wird in einer neuen Datei fortgesetzt.
fallback-note = Encoder-Fallback: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Programmton ist verstummt
alarm-clipping = Programmton übersteuert
alarm-black = Programmbild ist schwarz
alarm-frozen = Programmbild hat sich länger nicht verändert
alarm-lowDisk = Speicherplatz: noch etwa { $minutes } Min. bei aktueller Bitrate
alarm-dismiss = Alarm schließen
alarm-cleared = Behoben: { $alarm }

# CAP-M22 — panic button
palette-panic = Panik — auf Datenschutz-Tafel schneiden
panic-banner-title = Panik
panic-banner-body = Das Programm zeigt die Datenschutz-Tafel; der Ton ist stumm, Aufnahmen der Quellen gestoppt. Stream und Aufnahme laufen weiter.
panic-restore = Wiederherstellen…
panic-restore-confirm = Programm wiederherstellen?
panic-restore-yes = Wiederherstellen
panic-restore-cancel = Abbrechen
hotkeys-panic = Panik (Datenschutz-Tafel)
hotkeys-timer-toggle = Alle Timer starten/pausieren
hotkeys-timer-reset = Alle Timer zurücksetzen
panic-slate-color = Farbe der Panik-Tafel
panic-slate-image = Bild der Panik-Tafel
panic-slate-image-placeholder = Optionaler Bildpfad

# CAP-M24 — redacted diagnostics bundle
diag-title = Diagnosepaket
diag-intro = Exportiert eine bereinigte .zip (Konfigurations-Snapshot, Encoder-Probe, aktuelle Statistiken — Geheimnisse, Pfade und Namen sind nie enthalten) zum manuellen Anhängen an ein GitHub-Issue. Nichts wird gesendet.
diag-preview = Inhalt ansehen
diag-hide-preview = Vorschau ausblenden
diag-export = .zip exportieren
diag-exported = Exportiert: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Go-Live-Check
preflight-intro = Jeder blockierende Punkt muss grün sein; der Rest sind ehrliche Hinweise.
preflight-item-targets = Stream-Ziele konfiguriert (Key/URL gesetzt)
preflight-item-encoder = Ein nutzbarer Encoder ist verfügbar
preflight-item-sources = Alle Quellen gesund
preflight-item-disk = Speicherplatz für die Aufnahme
preflight-item-mic = Mikrofonpegel
preflight-item-desktopAudio = Desktop-Audio-Pegel
preflight-item-replay = Wiederholungspuffer aktiv
preflight-targets-detail = { $count } aktiviert
preflight-sources-detail = { $count } Quelle(n) mit Fehler
preflight-disk-detail = ~{ $minutes } Min. bei aktueller Bitrate
preflight-fix-stream = Stream-Einstellungen…
preflight-fix-components = Komponenten…
preflight-fix-sources = Quellenzustand…
preflight-fix-replay = Aktivieren
preflight-optional = optional
preflight-hold = Go Live sperren, bis alle Prüfungen grün sind
preflight-cancel = Abbrechen
preflight-go-anyway = Trotzdem live gehen
preflight-go-live = Go Live


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Hintergrund
scenes-backdrop-aria = Hintergrund für { $name }
backdrop-title = Hintergrund — { $name }
backdrop-hint = Ein Hintergrund, der hinter allem in dieser Szene fixiert ist — ein Bild, ein animiertes GIF oder ein Video in Schleife. Deine Aufnahme liegt immer obenauf; zum Zoomen auf der Leinwand scrollen.
backdrop-choose = Bild oder Video wählen…
backdrop-remove = Hintergrund entfernen
backdrop-none = Kein Hintergrund gesetzt.
backdrop-position = Position
backdrop-split-full = Ganze Leinwand
backdrop-split-left = Linke Hälfte
backdrop-split-right = Rechte Hälfte
backdrop-split-top = Obere Hälfte
backdrop-split-bottom = Untere Hälfte
backdrop-sync = Wiedergabe mit der Aufnahme starten
backdrop-sync-hint = Hält das erste Bild, bis du aufnimmst; jeder Take startet das Video von vorn.
backdrop-preview-play = Vorschau abspielen
backdrop-preview-pause = Vorschau pausieren
backdrop-filter-all = Hintergründe (Bilder & Video)
backdrop-filter-images = Bilder
backdrop-filter-media = Video & GIF
sources-backdrop-badge = Hintergrundbild (ganz unten fixiert)
sources-backdrop-pinned = Der Hintergrund bleibt ganz unten fixiert
filters-name-flip = Spiegeln
filters-flip-horizontal = Horizontal
filters-flip-vertical = Vertikal
history-setSceneBackdrop = Hintergrund festlegen
history-setBackdropSplit = Hintergrund verschieben
history-setBackdropSync = Hintergrund-Aufnahmesynchronisation
backdrop-scrub = Wiedergabeposition
backdrop-loop = Schleife
backdrop-reverse = Rückwärts abspielen
backdrop-reverse-hint = Rückwärts rendert einmalig eine umgekehrte Kopie (Videos brauchen die ffmpeg-Komponente; GIFs sofort) — das erste Umschalten kann bei langen Dateien dauern.
filters-scaling = Skalierung
filters-scaling-hint = Pixelgenaue Modi für Retro-/Pixelinhalte; „Ganzzahlig" rastet die gezeichnete Größe zudem auf ganze Vielfache ein (die Griffe zeigen die logische Größe).
filters-scaling-auto = Weich
filters-scaling-nearest = Nächster Nachbar
filters-scaling-integer = Ganzzahlig (ganze ×)
filters-scaling-sharp = Scharfes Bilinear
history-setScaling = Skalierung geändert
hotkeys-zoom-100 = Zoom: zurücksetzen (100 %)
hotkeys-zoom-150 = Zoom: 150 % heranzoomen
hotkeys-zoom-200 = Zoom: 2× heranzoomen
sources-follow-title = Beim Zoomen dem Cursor folgen (Windows; zum Zoomen auf der Leinwand scrollen)
sources-follow-item = Cursor-Verfolgung für { $name } umschalten
filters-autocrop = ✂ Schwarze Balken auto-zuschneiden
filters-autocrop-title = Das nächste Bild auf Letterbox-/Pillarbox-Balken prüfen und zuschneiden (rückgängig machbar). Dunkle Szenen werden nie beschnitten.
filters-autocrop-follow = Bei Auflösungswechsel erneut prüfen
history-autoCrop = Schwarze Balken auto-zugeschnitten
sources-link-audio = Auch den Ton dieser App aufnehmen (verknüpft: Ausblenden stummschaltet, Entfernen entfernt beides)
history-addLinkedWindow = Fenster + verknüpften Ton hinzufügen
sources-hdr-title = Dieses Display ist HDR — Tone-Mapping öffnen (die Leinwand bleibt SDR)
sources-hdr-item = HDR-Tone-Mapping für { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Dieses Display gibt HDR aus. Ohne Tone-Mapping clippen Lichter und die Aufnahme wirkt auf der SDR-Leinwand ausgewaschen. Änderungen greifen beim nächsten Bild.
sources-hdr-enable-suggested = Vorschlag aktivieren (maxRGB, 200 Nits)
sources-hdr-operator = Operator
sources-hdr-op-clip = Clip (aus)
sources-hdr-op-maxrgb = maxRGB (farbtonerhaltend)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408-Knie (SDR exakt)
sources-hdr-paper-white = Papierweiß
sources-hdr-nits = Nits
projector-target-passthrough = Durchleitungs-Monitor (geringe Latenz)
projector-which-device = Gerät
projector-passthrough-none = Füge zuerst ein Display, ein Fenster oder ein Aufnahmegerät hinzu.
projector-passthrough-about = Rohe Gerätebilder — keine Szenen, keine Filter, kein Compositor. Zeigt eine gemessene Latenz; Ton wird weiterhin über den Mixer-Kanal abgehört.
projector-passthrough-hint = Durchleitung — Esc schließt
projector-latency = { $ms } ms
projector-latency-measuring = messe…
automation-title = Automatisierung — Regeln, Makros & Variablen
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Regeln
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = An
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Wenn
automation-then-run = dann ausführen
automation-no-macro = (no macro)
automation-macros = Makros
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Ausführen
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Studio-Variablen
automation-variables-about = Use {"{{"}name{"}}"} in any Text source — it updates live when a macro sets the value.
automation-var-name = name
automation-var-value = value
automation-set-var = Set
automation-trigger-scene = scene switches to
automation-trigger-stream = streaming
automation-trigger-recording = recording
automation-trigger-source-error = a source errors
automation-trigger-audio = audio level
automation-trigger-idle = system idle for
automation-trigger-focus = window focus is
automation-trigger-time = time of day
automation-trigger-file = file changes
automation-scene-name = scene name
automation-starts = starts
automation-stops = stops
automation-any-source = (any source)
automation-source-name = source name
automation-rises-above = rises above
automation-falls-below = falls below
automation-threshold = threshold in dB
automation-idle-seconds = idle seconds
automation-seconds-windows = s (Windows only)
automation-exe = executable name
automation-windows-only = (Windows only)
automation-time = time (HH:MM)
automation-file = file path
rundown-title = Sendungs-Ablaufplan
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Start
rundown-next = Weiter ▸
rundown-stop = Stopp
rundown-idle = Läuft nicht
rundown-next-up = Als Nächstes: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Schritt
rundown-new-step = New step
rundown-step-name = Step name
rundown-step-scene = Scene
rundown-stay = (stay on this scene)
rundown-hold = Hold seconds
rundown-seconds = s
rundown-jump = Jump to this step
rundown-move-up = Move up
rundown-move-down = Move down
rundown-remove = Remove step
automation-layer = Ebene
automation-layer-hint = Feuert nur, während diese Ebene aktiv ist (leer = alle Ebenen). Ebenen sind fest: eine Ebenentaste wechselt und bleibt dort (echte Halte-Ebenen bietet die OS-Hotkey-API nicht).
automation-chord-hint = Eine einfache Taste (Ctrl+Shift+M) oder ein Zwei-Tasten-Akkord (Ctrl+K, 3). Die zweite Taste wird nur belegt, solange der Akkord aussteht.
panel-title = LAN-Panel & Tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Panel bereitstellen
panel-port = Port
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Passwort
panel-show = Zeigen
panel-hide = Verbergen
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Speichern
osc-title = OSC-Steuerfläche
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = OSC empfangen
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ-Kameras
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Kamera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Adresse
ptz-port = Port
ptz-speed = Geschwindigkeit
ptz-zoom = Zoom
ptz-zoom-in = Zoom in
ptz-zoom-out = Zoom out
ptz-move-up = Tilt up
ptz-move-down = Tilt down
ptz-move-left = Pan left
ptz-move-right = Pan right
ptz-move-upLeft = Up and left
ptz-move-upRight = Up and right
ptz-move-downLeft = Down and left
ptz-move-downRight = Down and right
ptz-move-stop = Stop
ptz-presets = Presets
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Abrufen
ptz-store = Speichern
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI-Steuerfläche
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Eingang
midi-output = Ausgang (Rückmeldung)
midi-none = (none)
midi-learn = Lernen
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Aktion
midi-target-action = Action
midi-target-macro = Macro
midi-target-scene = Switch scene
midi-target-volume = Fader
midi-target-mute = Mute
midi-command = Command
midi-macro = Macro
midi-scene = Scene
midi-source = source name
midi-feedback = LED
midi-remove = Remove binding
panel-lan-warning = ⚠ LAN-Verkehr ist unverschlüsselt — das Passwort steht in der URL (Klartext-HTTP). Nur in vertrauenswürdigen Netzen verwenden.
osc-lan-warning = ⚠ OSC hat kein Passwort — jedes Gerät im Netz kann diese Befehle senden. LAN nur in vertrauenswürdigen Netzen.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stats
sources-add-system-stats = Leistungsdaten (HUD)
sources-stats-title = Leistungs-HUD hinzufügen
sources-stats-note = Zeigt die echten Messwerte des Studios für deine Zuschauer im Programm — fps, CPU, Speicher, Renderzeit, verlorene Frames und Live-Bitrate. Welche Zeilen angezeigt werden sowie Größe und Farbe stehen in den Eigenschaften der Quelle. Die GPU-Auslastung wird nicht angezeigt, weil sie nicht gemessen wird.
sources-stats-add = Stats-HUD hinzufügen
properties-stats-show-fps = FPS anzeigen
properties-stats-show-cpu = CPU anzeigen
properties-stats-show-memory = Speicher anzeigen
properties-stats-show-render = Renderzeit anzeigen
properties-stats-show-dropped = Verlorene Frames anzeigen
properties-stats-show-bitrate = Bitrate anzeigen
properties-stats-size = Größe (px)
properties-stats-note = Das HUD rendert kompakte universelle Beschriftungen (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) direkt ins Programm; ohne laufenden Stream zeigt die Bitrate-Zeile „—“.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Visualizer
sources-add-visualizer = Audio-Visualizer
sources-visualizer-title = Audio-Visualizer hinzufügen
sources-visualizer-style-label = Stil
sources-visualizer-style-bars = Spektrum-Balken
sources-visualizer-style-scope = Oszilloskop
sources-visualizer-style-vu = VU-Meter
sources-visualizer-target-label = Hört auf
sources-visualizer-target-master = Master-Mix
sources-visualizer-target-track = Track { $n }
sources-visualizer-note = Zeichnet das Signal, das tatsächlich gemischt wird (post-fader) — eine stummgeschaltete Quelle bleibt flach, genau wie sie klingt. Größe, Farbe, Balkenzahl und Fallrate stehen in den Eigenschaften der Quelle.
sources-visualizer-add = Visualizer hinzufügen
properties-vis-bands = Balken
properties-vis-decay = Fallrate (dB/s)
properties-vis-peak-hold = Peak-Hold-Markierungen
properties-vis-missing-source = (fehlende Quelle)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splits
sources-add-split-timer = Speedrun-Split-Timer
sources-splits-title = Split-Timer hinzufügen
sources-splits-file-label = LiveSplit-.lss-Datei
sources-splits-comparison-label = Vergleichen mit
sources-splits-comparison-pb = Persönliche Bestzeit
sources-splits-comparison-best = Beste Segmente
sources-splits-comparison-average = Durchschnitt
sources-splits-note = Die Datei wird nur gelesen — nie zurückgeschrieben. Die globalen Split- / Undo- / Skip- / Reset-Tasten unter Einstellungen → Hotkeys belegen. Auto-Splitter über Prozessspeicher werden bewusst nicht unterstützt.
sources-splits-add = Split-Timer hinzufügen
properties-splits-size = Größe (px)
properties-splits-ahead = Voraus
properties-splits-behind = Zurück
properties-splits-gold = Gold
properties-splits-split = Split
properties-splits-undo = Rückgängig
properties-splits-skip = Überspringen
properties-splits-reset = Zurücksetzen
properties-splits-note = Die Buttons steuern den laufenden Timer (die globalen Hotkeys tun dasselbe aus jeder App). Der Run wird nie in die .lss-Datei geschrieben.
hotkeys-split-split = Split-Timer: Start / Split
hotkeys-split-undo = Split-Timer: Split rückgängig
hotkeys-split-skip = Split-Timer: Segment überspringen
hotkeys-split-reset = Split-Timer: zurücksetzen
hotkey-audit-action-split-split = Split (Split-Timer)
hotkey-audit-action-split-undo = Split rückgängig
hotkey-audit-action-split-skip = Segment überspringen
hotkey-audit-action-split-reset = Split-Timer zurücksetzen
hotkey-audit-feature-split-timer = Split-Timer

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlist
sources-add-playlist = Medien-Playlist (nahtlos)
sources-playlist-title = Medien-Playlist hinzufügen
sources-playlist-files-label = Dateien (eine pro Zeile, von oben nach unten gespielt)
sources-playlist-browse = Durchsuchen…
sources-playlist-loop = Schleife
sources-playlist-shuffle = Zufällig (eine Ziehung pro Start; eine geloopte Zufallsfolge wiederholt sich)
sources-playlist-hold-last = Letztes Bild am Ende halten
sources-playlist-note = Spielt die gesamte getrimmte Liste nahtlos über die gekennzeichnete ffmpeg-Komponente (nur Wire-Formate — .frec und Standbilder über Medien/Diashow). Einträge sind alle Video oder alle Audio, nie gemischt. Trims, Cue-Punkte und die „Now playing“-Variable stehen in den Eigenschaften.
sources-playlist-add = Playlist hinzufügen
properties-playlist-items = Einträge (von oben nach unten gespielt)
properties-playlist-up = Nach oben
properties-playlist-down = Nach unten
properties-playlist-remove = Eintrag entfernen
properties-playlist-in = Ab (s)
properties-playlist-out = Bis (s)
properties-playlist-cues = Cues (s, kommagetrennt)
properties-playlist-add-item = + Eintrag hinzufügen
properties-playlist-loop = Schleife
properties-playlist-shuffle = Zufällig
properties-playlist-hold-last = Letztes Bild halten
properties-playlist-hw = Hardware-Dekodierung
properties-playlist-variable = „Now playing“-Variable (leer = aus)
properties-playlist-previous = ⏮ Zurück
properties-playlist-next = ⏭ Weiter
properties-playlist-note = Cue-Buttons und Weiter/Zurück steuern die LAUFENDE Playlist; Eintragsänderungen gelten mit „Übernehmen“ (die Playlist startet neu). Setze {"{{"}yourVariable{"}}"} in eine Textquelle, um den laufenden Titel zu zeigen.
hotkeys-playlist-next = Playlist: nächster Eintrag
hotkeys-playlist-previous = Playlist: voriger Eintrag
hotkey-audit-action-playlist-next = Playlist weiter
hotkey-audit-action-playlist-previous = Playlist zurück
hotkey-audit-feature-playlist = Playlist

# Instant replay source (CAP-N10)
sources-badge-replay = Replay
sources-add-replay = Instant Replay
sources-replay-title = Instant Replay hinzufügen
sources-replay-seconds-label = Roll-Länge (Sekunden)
sources-replay-speed-label = Geschwindigkeit
sources-replay-speed-full = 100 % (mit Ton)
sources-replay-speed-half = 50 % Zeitlupe (stumm)
sources-replay-speed-quarter = 25 % Zeitlupe (stumm)
sources-replay-note = Bleibt transparent, bis du rollst. Replay-Puffer scharfschalten (Steuerung) und die Roll-Taste belegen — ein Roll schneidet die letzten Momente des Puffers und spielt sie ins Programm, dann wird die Quelle wieder transparent.
sources-replay-add = Instant Replay hinzufügen
properties-replay-roll = ⏵ Replay rollen
properties-replay-note = Roll schneidet den SCHARFEN Replay-Puffer in einen Clip und spielt ihn mit der gewählten Geschwindigkeit — umgetaktet, nie interpoliert. Zeitlupe ist bewusst stumm. Scrubben/Pause funktionieren während der Wiedergabe; am Ende wird die Quelle wieder transparent.
hotkeys-replay-roll = Instant Replay: rollen
hotkey-audit-action-replay-roll = Instant Replay rollen

# Input-Overlay-Quelle (CAP-N13)
sources-badge-input = Eingabe
sources-add-input-overlay = Eingabe-Overlay (Tasten/Pad)
sources-input-title = Eingabe-Overlay hinzufügen
sources-input-layout-label = Layout
sources-input-layout-wasd = WASD + Maus
sources-input-layout-keyboard = Kompakte Tastatur + Maus
sources-input-layout-gamepad = Gamepad (zwei Sticks)
sources-input-layout-fightstick = Fight Stick
sources-input-color-label = Tasten
sources-input-accent-label = Gedrückt
sources-input-privacy-note = Privatsphäre: Eingaben werden nur gelesen, solange diese Quelle in einer Szene live ist, und nur die festen Tasten des Layouts werden abgefragt — ein momentaner „ist sie gerade gedrückt?“-Blick, nie ein Hook. Nichts wird protokolliert, gespeichert oder irgendwohin gesendet; getippter Text wird nie erfasst.
sources-input-os-note = Tastatur- und Mausstatus wird heute nur unter Windows gelesen — andere Systeme zeichnen die Tasten ungedrückt (ehrlich gesagt, nie vorgetäuscht). Gamepads funktionieren überall über die gilrs-Bibliothek; gezeichnet wird der erste verbundene Controller, ohne Controller bleibt das Layout ungedrückt.
sources-input-add = Eingabe-Overlay hinzufügen

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Cursor-Effekte
filters-cursorfx-hint = Werden unter Windows (das den Cursor selbst zeichnet) direkt ins Capture gezeichnet und erscheinen so in Aufnahmen und Streams. macOS und Linux setzen den Cursor auf OS-Seite zusammen — diese Effekte gibt es daher nur unter Windows. Änderungen wirken sofort.
filters-cursorfx-halo = Cursor-Halo
filters-cursorfx-halo-color = Farbe
filters-cursorfx-halo-radius = Radius (px)
filters-cursorfx-ripples = Klick-Wellen
filters-cursorfx-left-color = Linksklick
filters-cursorfx-right-color = Rechtsklick
filters-cursorfx-keystrokes = Tastenanzeige
filters-cursorfx-keystrokes-hint = Zeigt einen festen Tastensatz (Buchstaben, Ziffern, Modifikatoren, Pfeile) neben dem Cursor, solange gedrückt. Tasten werden nur gelesen, während dies aktiv ist, direkt ins Bild gezeichnet und nie gespeichert oder protokolliert.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Titel
sources-add-title = Titel / Anzeigetafel
sources-title-title = Titel hinzufügen
sources-title-template-label = Vorlage
sources-title-template-lower-third = Bauchbinde (Balken + Name + Untertitel)
sources-title-template-scoreboard = Anzeigetafel (Platte + 4 Felder)
sources-title-template-blank = Leere Fläche
sources-title-width-label = Flächenbreite
sources-title-height-label = Flächenhöhe
sources-title-template-name = Name
sources-title-template-subtitle = Titel
sources-title-template-home = HEIM
sources-title-template-away = GAST
sources-title-note = Titel aus Text-, Bild- und Kastenebenen mit Ein-/Ausblend-Animation, lokal gerendert — keine Browser-Quelle. Ebenen, Datei-Bindungen und {"{{"}Variablen{"}}"} sowie die Live-Steuerung stehen in den Eigenschaften der Quelle.
sources-title-add = Titel hinzufügen
properties-title-layers = Ebenen (der Reihe nach gezeichnet — spätere Zeilen liegen oben)
properties-title-kind-text = Text
properties-title-kind-image = Bild
properties-title-kind-rect = Kasten
properties-title-x = X
properties-title-y = Y
properties-title-outline = Kontur (px)
properties-title-outline-color = Kontur
properties-title-shadow = Schatten
properties-title-animation = Ein-/Ausblenden
properties-title-anim-none = Keine (harter Schnitt)
properties-title-anim-fade = Überblenden
properties-title-anim-slide-left = Nach links gleiten
properties-title-anim-slide-up = Nach oben gleiten
properties-title-anim-wipe = Wischen
properties-title-duration = Dauer (ms)
properties-title-fire-in = ▶ Einblenden
properties-title-fire-out = ◼ Ausblenden
properties-title-set-live = Live setzen
properties-title-set-live-note = Schiebt diesen Text sofort in den LAUFENDEN Titel — ohne Übernehmen, ohne Neustart
properties-title-up = Ebene nach oben
properties-title-down = Ebene nach unten
properties-title-remove = Ebene entfernen
properties-title-add-text = + Text
properties-title-add-image = + Bild
properties-title-add-rect = + Kasten
properties-title-note = Ein-/Ausblenden und „Live setzen" steuern den LAUFENDEN Titel; Ebenen-Änderungen gelten mit „Übernehmen" (der Titel startet neu und blendet erneut ein). Textfelder können an eine überwachte Datei binden (CSV-Zelle / JSON-Wert / ganze Datei) und {"{{"}Variablen{"}}"} interpolieren — „Live setzen" gewinnt gegen beides.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN-Ingest (SRT/RTMP-Listener)
sources-lan-title = LAN-Ingest-Listener hinzufügen
sources-lan-protocol-label = Protokoll
sources-lan-protocol-srt = SRT (verschlüsselbar — empfohlen)
sources-lan-protocol-rtmp = RTMP (keine Authentifizierung)
sources-lan-port-label = Port (1024–65535)
sources-lan-passphrase-label = Passphrase (leer = offen)
sources-lan-passphrase-hint = SRT-Passphrasen haben 10–79 Zeichen; der Sender muss dieselbe verwenden.
sources-lan-open-warning = Keine Passphrase: Jeder in diesem Netzwerk kann diese Quelle unverschlüsselt speisen. Setze eine, außer das Netzwerk gehört dir allein.
sources-lan-rtmp-warning = RTMP hat keine Authentifizierung — jeder in diesem Netzwerk kann an diesen Port senden. Bevorzuge SRT mit Passphrase.
sources-lan-url-label = Richte die Sender-App auf
sources-lan-qr-aria = QR-Code der Ingest-URL
sources-lan-note = Nur LAN: lauscht auf der lokalen Adresse dieser Maschine, nur solange die Quelle existiert, und berührt nie das Internet — nichts verlässt die Maschine, bis ein Sender in deinem Netzwerk zuerst sendet. Die Dekodierung läuft über die klar benannte ffmpeg-Komponente. Die Leinwand zeigt diese URL, bis sich ein Sender verbindet.
sources-lan-add = Lauschen starten
properties-lan-note = Das Übernehmen einer Protokoll-, Port- oder Passphrase-Änderung startet den Listener neu — der Sender muss sich neu verbinden. Der Stream wird in eine 1920×1080-Leinwand eingepasst.

# Freally Link source & output (CAP-N12)
sources-badge-link = Link
sources-add-freally-link = Freally Link (weitere Instanz)
sources-link-title = Freally Link hinzufügen
sources-link-about = Empfängt das Programm einer anderen Freally-Capture-Instanz — Video und Master-Audio — über dein eigenes Netzwerk. Aktiviere zuerst „Freally-Link-Ausgabe“ auf der sendenden Instanz. v1 überträgt Motion-JPEG über TCP: ideal im kabelgebundenen LAN oder guten WLAN, ehrlich beim Bandbreitenbedarf auf schwachen Verbindungen.
sources-link-scan = Netzwerk durchsuchen
sources-link-scanning = Suche läuft…
sources-link-none = Keine Freally-Link-Ausgaben gefunden. Aktiviere „Freally-Link-Ausgabe“ auf der anderen Instanz (Steuerung → LAN-Panel) oder gib unten ihre Adresse ein.
sources-link-host = Adresse
sources-link-port = Port
sources-link-key = Kopplungsschlüssel
sources-link-key-hint = Der Schlüssel aus den „Freally-Link-Ausgabe“-Einstellungen des Senders — ohne ihn liefert der Sender kein einziges Bild.
sources-link-add = Link hinzufügen
properties-link-note = Ohne Verbindung zeigt die Quelle ein „Verbinden“-Bild und versucht es selbstständig mit wachsendem Abstand erneut — sie friert nie auf einem alten Frame ein. Ein Empfänger pro Sender; ein besetzter Sender wird höflich weiter angefragt.
link-title = Freally-Link-Ausgabe
link-about = Teile das Programm dieser Instanz — Video und Master-Audio — mit EINER weiteren Freally-Capture-Instanz in deinem eigenen Netzwerk; dort erscheint es als „Freally Link“-Quelle (Zwei-PC-Streaming, Zusatzmonitore). Standardmäßig aus; nichts kündigt sich an oder lauscht, bis es aktiviert wird. v1 überträgt Motion-JPEG + unkomprimiertes Audio über TCP — gemacht für kabelgebundenes LAN oder gutes WLAN, nie für das Internet.
link-enable = Programm in meinem Netzwerk teilen
link-name = Instanzname
link-key = Kopplungsschlüssel
link-key-hint = Mindestens 8 Zeichen — Empfänger müssen diesen Schlüssel eingeben, bevor auch nur ein Bild geliefert wird.
link-lan-warning = ⚠ Empfänger müssen den Kopplungsschlüssel vorweisen, bevor etwas geliefert wird — der Stream selbst ist in v1 aber unverschlüsselt. Nur in einem vertrauenswürdigen Netzwerk verwenden.
link-serving = Empfänger finden diese Instanz mit „Netzwerk durchsuchen“ oder fügen sie manuell hinzu unter:
link-off-hint = Aktiviere das Teilen, um den Port zu öffnen und diese Instanz für LAN-Scans anzukündigen.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Anwendungsmenü
menu-file = Datei
menu-edit = Bearbeiten
menu-view = Ansicht
menu-docks = Docks
menu-profile = Profil
menu-collection = Szenensammlung
menu-tools = Werkzeuge
menu-help = Hilfe
menu-rename = Umbenennen
menu-remove = Entfernen
menu-import = Importieren
menu-export = Exportieren
menu-file-show-recordings = Aufnahmen anzeigen
menu-file-remux = Zu MP4 remuxen…
menu-file-settings = Einstellungen…
menu-file-show-settings-folder = Einstellungsordner anzeigen
menu-file-exit = Beenden
menu-edit-undo = Rückgängig
menu-edit-redo = Wiederholen
menu-edit-history = Bearbeitungsverlauf…
menu-edit-copy-transform = Transformation kopieren
menu-edit-paste-transform = Transformation einfügen
menu-edit-copy-filters = Filter kopieren
menu-edit-paste-filters = Filter einfügen
menu-edit-transform = Transformation…
menu-edit-lock-preview = Vorschau sperren
menu-view-fullscreen = Vollbild-Oberfläche
menu-stats-dock = Statistik-Dock
menu-view-multiview = Multiview-Monitor…
menu-view-projectors = Projektoren…
menu-view-source-health = Quellenzustand…
menu-view-still = Standbild aufnehmen
menu-docks-browser = Browser-Docks…
menu-docks-lock = Docks sperren
menu-docks-reset = Dock-Layout zurücksetzen
menu-profile-manage = Profile verwalten…
menu-collection-manage = Szenensammlungen verwalten…
menu-collection-import-obs = Aus OBS importieren…
menu-collection-missing = Auf fehlende Dateien prüfen…
menu-tools-wizard = Einrichtungsassistent starten
menu-tools-wizard-title = Der Einrichtungsassistent läuft beim ersten Start; ein erneuter Durchlauf ist noch nicht möglich.
menu-tools-automation = Automatisierungsregeln & Makros…
menu-tools-rundown = Ablaufplan anzeigen…
menu-tools-hotkeys = Hotkey-Übersicht…
menu-tools-av-sync = A/V-Sync-Kalibrierung…
menu-tools-scripts = Lua-Skripte…
menu-tools-components = Komponenten…
menu-tools-midi = MIDI-Steuerung…
menu-tools-ptz = PTZ-Kameras…
menu-tools-remote = Fernsteuerungs-API…
menu-tools-panel = LAN-Panel & Tally…
menu-help-portal = Hilfeportal
menu-help-website = Website besuchen
menu-help-discord = Discord-Server beitreten
menu-help-bug = Fehler melden…
menu-help-updates = Nach Updates suchen…
menu-help-whats-new = Neuigkeiten
menu-help-about = Über…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Einstellungskategorien
settings-cat-general = Allgemein
settings-cat-appearance = Darstellung
settings-cat-streaming = Streaming
settings-cat-output = Ausgabe
settings-cat-replay = Wiederholung
settings-cat-hotkeys = Tastenkürzel
settings-cat-network = Netzwerk
settings-cat-accessibility = Barrierefreiheit
settings-cat-about = Über
settings-ok = OK
settings-cancel = Abbrechen
settings-apply = Anwenden
settings-save = Speichern
settings-loading = Einstellungen werden geladen…
settings-hotkeys-filter = Tastenkürzel filtern
settings-hotkeys-filter-placeholder = Zum Filtern Aktion oder Taste eintippen…
settings-hotkeys-no-match = Kein Tastenkürzel passt zu “{ $query }”.
settings-hotkey-none = Keins
settings-hotkey-group-ctrl = Ctrl + Taste
settings-hotkey-group-ctrl-shift = Ctrl + Shift + Taste
settings-hotkey-group-ctrl-alt = Ctrl + Alt + Taste
settings-hotkey-group-function = Funktionstasten
settings-hotkey-group-numpad = Ziffernblock
settings-panic-section = Panik-Tafel
settings-meter-section = Pegelanzeigen des Mixers
settings-meter-note = Die Farben, die die Pegelanzeigen des Audio-Mixers von leise bis zur Übersteuerung durchlaufen. Die farbenblind-sichere Vorgabe nutzt einen Blau-zu-Orange-Verlauf, der bei Rot-Grün-Schwäche lesbar bleibt.
settings-meter-preset = Pegelfarben
settings-meter-preset-default = Grün / Gelb / Rot
settings-meter-preset-colorblind = Farbenblind-sicher (Blau / Orange)
settings-meter-preset-custom = Benutzerdefiniert
settings-meter-low = Normal
settings-meter-mid = Laut
settings-meter-high = Übersteuerung
settings-meter-preview = Vorschau

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Neuigkeiten
whats-new-loading = Versionshinweise werden geladen…
whats-new-version = Neuigkeiten in Version { $version }
whats-new-empty = Keine Versionshinweise für diese Version.
filters-name-directional-blur = Gerichtete Unschärfe
filters-name-radial-blur = Radiale Unschärfe
filters-name-zoom-blur = Zoom-Unschärfe
filters-name-pixelate = Verpixeln
filters-angle = Winkel (°)
filters-center-x = Zentrum X
filters-center-y = Zentrum Y
filters-block-size = Blockgröße (px)
filters-name-freeze = Einfrieren
filters-freeze-hint = Wenn aktiviert, hält diese Quelle ihr letztes Bild — Programm, Vorschau, Aufnahme und Stream frieren gemeinsam ein. Schalte diesen Filter um, um einzufrieren oder wieder freizugeben.
transform-3d = 3D-Neigung
transform-rotation-x = Neigung X (°)
transform-rotation-y = Neigung Y (°)
transform-perspective = Perspektive
transform-reveal = Ein-/ausblenden
transform-reveal-ms = Einblenden (ms)
sources-clone-title = Klonen (gleiche Quelle, eigene Filter)
sources-clone-item = { $name } klonen
menu-tools-downstream = Downstream-Keyer…
menu-tools-transition-rules = Übergangsregeln…
dsk-title = Downstream-Keyer
dsk-hint = Overlays, die auf die Programmausgabe komponiert werden — über jeder Szene, und sie bleiben erhalten, wenn du Szenen umschaltest (ein Logo, ein LIVE-Badge, eine Bauchbinde). Der oberste Eintrag liegt vorne.
dsk-empty = Noch keine Keyer — füge eine Quelle hinzu, um sie über jeder Szene einzublenden.
dsk-enable = Diesen Keyer aktivieren
dsk-move-up = Nach oben (nach vorne)
dsk-move-down = Nach unten
dsk-remove = Keyer entfernen
dsk-opacity = Deckkraft
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Skalierung
dsk-add = + Keyer hinzufügen
transition-rules-title = Übergangsregeln
transition-rules-hint = Geben Sie einem Szenenpaar einen eigenen Übergang. Wenn Sie von der ersten zur zweiten Szene wechseln, werden diese Art und Dauer anstelle der Standardwerte verwendet (eine Stinger-/Bild-Regel nutzt weiterhin die in den Übergangssteuerungen festgelegte Datei).
transition-rules-empty = Noch keine Regeln — jedes Szenenpaar verwendet den Standardübergang.
transition-rules-from = Von
transition-rules-to = Nach
transition-rules-kind = Übergang
transition-rules-duration = Dauer (ms)
transition-rules-add = Regel hinzufügen
transition-rules-remove = Regel entfernen
