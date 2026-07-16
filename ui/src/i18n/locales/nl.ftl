# Freally Capture — nl
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Studiomodus
toggle-on = aan
toggle-off = uit
stats = Statistieken
core-ok = kern OK
hide-stats-dock = Verberg het statistiekenpaneel
show-stats-dock = Toon het statistiekenpaneel


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Instellingen konden niet worden opgeslagen — de wijziging overleeft een herstart niet.
studio-mode-leave = Studiomodus verlaten
studio-mode-enter-title = Studiomodus — bewerk een voorbeeldscène en zet deze met een overgang op het programma
vertical-canvas-title = Het tweede (verticale 9:16) uitvoercanvas — apart op te nemen en te streamen
app-version = v{ $version }
core-error = kern FOUT
core-unreachable = kern onbereikbaar (browsermodus)
connecting-to-core = verbinden met kern…
filters-source-fallback = Bron

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Programmavoorbeeld
preview-program-output = Programma-uitvoer
preview-canvas-editor = Canvas-editor
preview-px-to-edge-label = Pixels tot de frameranden
preview-px-to-edge = px tot rand L { $left } · B { $top } · R { $right } · O { $bottom }
preview-program-heading = Programma
preview-no-gpu = Er is geen bruikbare GPU-adapter gevonden — de compositor kan niet draaien op deze machine.
preview-starting-compositor = De compositor starten…
preview-empty-scene = Deze scène is leeg — voeg een bron toe bij Bronnen en versleep, schaal en roteer deze hier op het canvas.
preview-fps = { $fps } fps
preview-dropped = { $dropped } verloren

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Uitnodigingslink ontvangen
remote-join-with-webcam = Deelnemen met webcam
remote-dismiss = Sluiten
remote-hosting-guest = Een externe gast hosten
remote-you-are-guest = Je bent een externe gast
remote-share-view-title = Deel je scherm met de app van de gast (ze zien jouw beeld live)
remote-stop-sharing-view = Delen van beeld stoppen
remote-share-my-view = Mijn beeld delen
remote-allow-center-title = Sta de gast toe te wisselen welk beeld het midden krijgt (jij houdt de controle en kunt altijd terugschakelen)
remote-guest-switching = Gast schakelt:
remote-stop-screen = Scherm stoppen
remote-share-screen = Scherm delen
remote-share-screen-title-guest = Deel je scherm met de host (het wordt een bron die ze kunnen centreren)
remote-center-request-label = Verzoek om beeld te centreren
remote-center = Centreren
remote-center-cam-title = Vraag de host om je camera te centreren
remote-center-my-cam = Mijn cam
remote-center-screen-title = Vraag de host om je gedeelde scherm te centreren
remote-center-my-screen = Mijn scherm
remote-center-host-title = Geef het midden terug aan het beeld van de host
remote-center-host-view = Hostbeeld
remote-end-session = Sessie beëindigen
remote-leave = Verlaten
remote-host-view-heading = Hostbeeld
remote-host-shared-view-label = Het gedeelde beeld van de host
remote-guest-position-label = Gastpositie
remote-guest-label = Gast
remote-put-guest = Zet de gast { $position }
remote-remove-title = Verwijder de gast — ze kunnen opnieuw deelnemen met dezelfde link
remote-remove = Verwijderen
remote-ban-title = Verban de gast — blokkeert ze en maakt de uitnodigingslink ongeldig
remote-ban = Verbannen
remote-guest-self-muted = gast heeft zichzelf gedempt
remote-unmute-guest = Demping gast opheffen
remote-mute-guest = Gast dempen
remote-muted-by-host = Gedempt door host
remote-unmute-mic = Microfoondemping opheffen
remote-mute-mic = Microfoon dempen
remote-waiting-for-host = wachten op de host


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = bron
sources-fallback-video = video
sources-fallback-error = fout
sources-kind-unknown = ?
sources-missing-source = (ontbrekende bron)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Beeldscherm
sources-badge-window = Venster
sources-badge-portal = Portal
sources-badge-camera = Camera
sources-badge-image = Afbeelding
sources-badge-media = Media
sources-badge-guest = Gast
sources-badge-color = Kleur
sources-badge-text = Tekst
sources-badge-scene = Scène
sources-badge-slides = Dia's
sources-badge-chat = Chat
sources-badge-audio-in = Audio-in
sources-badge-audio-out = Audio-uit
sources-badge-app-audio = App-audio
sources-badge-test-bars = Balken
sources-badge-test-grid = Raster
sources-badge-test-sweep = Sweep
sources-badge-test-tone = Toon
sources-badge-test-sync = Sync
sources-badge-timer = Timer

# Add-source menu items
sources-add-display = Beeldschermopname
sources-add-window = Vensteropname
sources-add-game = Spelopname (lees eerst)
sources-add-webcam = Video-opnameapparaat
sources-add-image = Afbeelding
sources-add-media = Media (video-/afbeeldingsbestand)
sources-add-remote-guest = Externe gast (P2P-experiment)
sources-add-color = Kleur
sources-add-text = Tekst
sources-add-timer = Timer / Klok
sources-add-nested-scene = Geneste scène
sources-add-slideshow = Diavoorstelling
sources-add-chat-overlay = Live chat-overlay
sources-add-test-signal = Testsignaal
sources-add-audio-input = Audio-invoeropname
sources-add-audio-output = Audio-uitvoeropname
sources-add-app-audio = Applicatie-audio (Windows)
sources-add-existing = Bestaande bron…

# Panel header + toolbar buttons
sources-panel-title = Bronnen
sources-group-title = Bronnen groeperen — kies twee of meer items en klik op Groep maken; gegroepeerde items verplaatsen en tonen/verbergen samen
sources-group-aria = Bronnen groeperen
sources-arrange = Schikken: scherm + hoeken
sources-add-source = Een bron toevoegen
sources-browser-source-note = Browserbron komt als eigen on-demand component-mijlpaal (een ~180 MB Chromium-engine — nooit meegeleverd). Vandaag: leg een echt browservenster vast met Vensteropname + een chroma-/kleursleutel, of open chat/meldingen als dock (Bediening → Docks).

# Empty state
sources-empty = Geen bronnen in deze scène — voeg met "+" een beeldschermopname, venster, webcam, afbeelding, kleur of tekst toe. Versleep, schaal en roteer ze op het canvas; de knoppen rechts herschikken de stapel.

# Per-row controls
sources-already-in-group = Al in { $name }
sources-pick-for-new-group = Kies voor de nieuwe groep
sources-pick-item-for-group = Kies { $name } voor de nieuwe groep
sources-hide = Verbergen
sources-show = Tonen
sources-hide-item = { $name } verbergen
sources-show-item = { $name } tonen
sources-unfocus-title = Focus opheffen — herstel de indeling
sources-focus-title = Focus — vul het canvas (Spreker uitlichten)
sources-unfocus-item = Focus op { $name } opheffen
sources-focus-item = Focus op { $name }
sources-center-title = Centreren — maak dit het gedeelde middenbeeld (cams verplaatsen naar de balk)
sources-center-item = { $name } centreren
sources-rename-item = { $name } hernoemen
sources-in-group = In groep { $name }

# Row status + retry
sources-retry-error = Opnieuw — { $message }
sources-retry-item = { $name } opnieuw proberen
sources-status-error = status: fout
sources-open-privacy-title = Open de macOS-privacyinstellingen voor deze toestemming
sources-open-privacy-item = Open privacyinstellingen voor { $name }
sources-privacy-settings-button = instellingen
sources-status-starting = starten…
sources-status-live = live
sources-status-aria = status: { $state }

# Media row pause/resume
sources-media-resume-title = Hervat de video (live op de stream)
sources-media-pause-title = Pauzeer de video — bevries het beeld en verstom, live op de stream
sources-media-resume-item = { $name } hervatten
sources-media-pause-item = { $name } pauzeren

# Hover controls
sources-unlock = Ontgrendelen
sources-lock = Vergrendelen
sources-unlock-item = { $name } ontgrendelen
sources-lock-item = { $name } vergrendelen
sources-raise-title = Omhoog in de stapel
sources-raise-item = { $name } omhoog
sources-lower-title = Omlaag in de stapel
sources-lower-item = { $name } omlaag
sources-filters-title = Filters & overvloeien
sources-filters-item = Filters voor { $name }
sources-properties-title = Eigenschappen
sources-properties-item = Eigenschappen van { $name }
sources-remove-title = Uit deze scène verwijderen
sources-remove-item = { $name } verwijderen

# Grouping footer
sources-create-group = Groep maken ({ $count })
sources-cancel = Annuleren

# Groups list
sources-groups-aria = Brongroepen
sources-hide-group = De groep verbergen
sources-show-group = De groep tonen
sources-item-count = · { $count } items
sources-ungroup-title = Groep opheffen — de items blijven waar ze zijn
sources-ungroup-item = Groep { $name } opheffen

# Live Chat Overlay picker
sources-chat-title = Een live chat-overlay toevoegen
sources-chat-youtube-label = YouTube — kanaal-, watch- of live_chat-URL (geen sleutel, geen aanmelding)
sources-chat-youtube-placeholder = https://www.youtube.com/@jouwkanaal  ·  of een watch?v=-URL
sources-chat-twitch-label = Twitch — kanaalnaam (anoniem gelezen, geen account)
sources-chat-twitch-placeholder = jouwkanaal
sources-chat-kick-label = Kick — kanaalslug (openbaar endpoint, best-effort)
sources-chat-kick-placeholder = jouwkanaal
sources-chat-note = Berichten verschijnen met een lopende tijdstempel h:mm:ss AM/PM op een transparante achtergrond (standaard rechtsboven; sleep het waarheen je wilt). Een chatvloed veroudert alleen oude regels — het kan de stream of opname nooit blokkeren. Facebook-chat vereist je eigen Graph-token en is nog niet geïmplementeerd — het is nooit vereist en blokkeert nooit de platforms hierboven.
sources-chat-add = Chat-overlay toevoegen
sources-chat-default-name = Live chat

# Image Slideshow picker
sources-slideshow-title = Een diavoorstelling toevoegen
sources-slideshow-empty = Nog geen afbeeldingen — Bladeren voegt ze op volgorde toe.
sources-slideshow-remove-slide = Dia { $number } verwijderen
sources-slideshow-browse = Afbeeldingen zoeken…
sources-slideshow-per-slide-label = Per dia (ms)
sources-slideshow-crossfade-label = Crossfade (ms, 0 = harde snit)
sources-slideshow-loop-label = Herhalen (uit = laatste dia vasthouden)
sources-slideshow-shuffle-label = Elke cyclus schudden
sources-slideshow-note = De crossfade vermengt afbeeldingen van gelijke grootte; verschillende groottes gaan met een harde snit over op de grens (geen stille herschaling).
sources-slideshow-add = Diavoorstelling toevoegen ({ $count })

# Nested Scene picker
sources-nested-title = Een geneste scène toevoegen
sources-nested-empty = Geen andere scène om te nesten — voeg eerst een tweede scène toe.
sources-nested-scene-name = Scène: { $name }
sources-nested-note = De geneste scène rendert live op de grootte van het programmacanvas en volgt haar eigen bewerkingen; transformaties, filters en overvloeien gelden zoals bij elke bron. Haar audiobronnen voegen zich bij de mix terwijl een scène die haar toont het programma is.

# Display / Window capture picker
sources-capture-display-title = Een beeldschermopname toevoegen
sources-capture-window-title = Een vensteropname toevoegen
sources-capture-looking = Zoeken naar bronnen…
sources-capture-none-displays = Niets om hier vast te leggen — geen beeldschermen gevonden.
sources-capture-none-windows = Niets om hier vast te leggen — geen vensters gevonden.
sources-capture-portal-note = Op Wayland kiest het systeemvenster het scherm of venster — apps kunnen daar niet globaal vastleggen, dus dat is het eerlijke (en enige) pad.
sources-capture-window-note = Voorbeelden werken live bij. Een geminimaliseerd venster toont zijn laatste frame (of geen) totdat je het herstelt.
sources-thumb-no-preview = geen voorbeeld
sources-thumb-loading = laden…

# Video Capture Device picker
sources-webcam-title = Een video-opnameapparaat toevoegen
sources-webcam-looking = Zoeken naar camera's…
sources-webcam-none = Geen camera's of opnamekaarten gevonden.
sources-webcam-format-label = Formaat
sources-webcam-format-auto-loading = Automatisch (formaten laden…)
sources-webcam-format-auto = Automatisch (hoogste resolutie)
sources-webcam-card-presets-label = Kaartvoorinstellingen:
sources-webcam-preset-title = Selecteer de { $label }-modus die deze kaart adverteert
sources-webcam-add = Camera toevoegen

# Audio Input / Output capture picker
sources-audio-output-title = Een audio-uitvoeropname toevoegen
sources-audio-input-title = Een audio-invoeropname toevoegen
sources-audio-default-output = Standaarduitvoer (wat je hoort)
sources-audio-default-input = Standaardinvoer
sources-audio-looking = Zoeken naar audioapparaten…
sources-audio-none-output = Hier is geen apparaat voor desktopaudio-opname gevonden.
sources-audio-none-input = Geen microfoons of lijningangen gevonden.
sources-audio-input-note = Mixerstroken krijgen een VU-meter, fader, demping, monitoring, filters (ruisonderdrukking, gate, compressor…) en tracktoewijzing. Alles blijft op deze machine.

# Application Audio picker
sources-appaudio-title = Applicatie-audio toevoegen
sources-appaudio-looking = Zoeken naar apps die geluid maken…
sources-appaudio-none = Er maken nu geen apps geluid — start weergave in de app en vernieuw dan.
sources-appaudio-refresh = ⟳ Vernieuwen
sources-appaudio-note = Legt precies de audio van die app vast — met eigen VU, fader, demping, filters en track.

# Game Capture picker
sources-game-title = Spelopname
sources-game-checking = Controleren…
sources-game-use-portal = Schermopname gebruiken (Portal)
sources-game-use-window = In plaats daarvan vensteropname gebruiken

# Image picker
sources-image-title = Een afbeelding toevoegen
sources-image-file-label = Afbeeldingsbestand (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Afbeelding toevoegen

# Path field
sources-browse = Bladeren…

# Media picker
sources-media-title = Media toevoegen
sources-media-file-label = Mediabestand (mp4, mkv, webm, mov, .frec of een afbeelding)
sources-media-loop-label = Herhalen (aan het einde opnieuw vanaf het begin)
sources-media-note = .frec speelt af via de eigen freally-video-codec — niets te downloaden. De wire-formaten (mp4/mkv/webm/…) decoderen via het on-demand FFmpeg-component; hun audio komt in de mixer als eigen strook.
sources-media-add = Media toevoegen

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 uur
sources-ttl-1day = 1 dag

# Remote Guest form
sources-remote-copy-failed = kopiëren mislukt — selecteer de link en kopieer handmatig
sources-remote-join-failed = deelnemen mislukt: { $error }
sources-remote-title = Externe gast (P2P-experiment)
sources-remote-host-heading = Host — nodig een gast uit
sources-remote-start-hosting = Hosten starten
sources-remote-expires-label = Verloopt
sources-remote-invite-expiry-aria = Vervaltijd uitnodiging
sources-remote-invite-link-aria = Uitnodigingslink
sources-remote-copied = Gekopieerd ✓
sources-remote-copy = Kopiëren
sources-remote-share-note = Deel deze link (Discord / sms / e-mail). Hij bevat je sessie en verloopt zoals ingesteld. De gast opent hem en neemt deel met de webcam.
sources-remote-qr-note = Scan op een telefoon om direct vanuit de browser deel te nemen — camera + microfoon, geen installatie. De kopieerbare freally://-link hierboven opent in Freally Capture op een machine die het heeft.
sources-remote-guest-heading = Gast — deelnemen met een uitnodiging
sources-remote-paste-placeholder = plak de uitnodigingslink
sources-remote-invite-input-aria = Uitnodigingslink of sessie-id
sources-remote-join = Deelnemen met webcam
sources-remote-session-note = De live sessiebediening (dempen, beëindigen) blijft op de balk boven aan het hoofdvenster — je kunt dit venster sluiten.
sources-remote-stop-session = Sessie stoppen

# Invite QR
sources-invite-qr-aria = QR-code van uitnodigingslink

# Remote device pickers
sources-devices-output-unavailable = uitvoerroutering niet beschikbaar — speelt af op het standaardapparaat
sources-devices-mic-test-failed = microfoontest mislukt: { $error }
sources-devices-heading = Audioapparaten van de sessie
sources-devices-microphone-label = Microfoon
sources-devices-microphone-aria = Sessiemicrofoon
sources-devices-system-default = Systeemstandaard
sources-devices-output-label = Uitvoer
sources-devices-output-aria = Audio-uitvoer van de sessie
sources-devices-stop-test = Test stoppen
sources-devices-test = Test — hoor jezelf
sources-devices-testing-note = praat in de microfoon — je hoort de geselecteerde apparaten live
sources-devices-idle-note = loopt je microfoon terug naar de uitvoer (koptelefoon voorkomt feedback)

# TURN relay section
sources-turn-save-failed = opslaan mislukt: { $error }
sources-turn-summary = Netwerk — optionele TURN-relay (geavanceerd)
sources-turn-note-1 = Sessies verbinden rechtstreeks (P2P) — gratis, geen relay nodig. Als BEIDE kanten achter strikte NAT's zitten, kan het directe pad falen; een TURN-relay die je zelf draait vervoert de media dan. Dit overslaan is prima — de meeste verbindingen werken alleen direct.
sources-turn-note-2 = Gratis optie: Oracle Cloud "Always Free" draait coturn kosteloos (let op: Oracle vraagt bij aanmelding om een creditcard, maar de Always-Free-vorm blijft gratis). Stappen: 1) maak de gratis VM, 2) installeer coturn, 3) open UDP 3478, 4) stel een gebruiker/wachtwoord in, 5) voer turn:jouw-vm-ip:3478 + de inloggegevens hier in. Je inloggegeven blijft in je lokale instellingenbestand en wordt nooit gelogd.
sources-turn-url-label = TURN-URL
sources-turn-url-placeholder = turn:host:3478 (leeg = alleen direct)
sources-turn-url-aria = TURN-URL
sources-turn-username-label = Gebruikersnaam
sources-turn-username-aria = TURN-gebruikersnaam
sources-turn-credential-label = Inloggegeven
sources-turn-credential-aria = TURN-inloggegeven
sources-turn-note-3 = De relay wordt actief zodra alle drie de velden zijn ingesteld (een TURN-server vereist de inloggegevens) en geldt voor de volgende sessie die je start of waaraan je deelneemt. Verifieer het met een relay-only-testgesprek tussen je eigen twee machines.
sources-turn-settings-unavailable = instellingen niet beschikbaar (browsermodus)

# Color picker
sources-color-title = Een kleur toevoegen
sources-color-label = Kleur
sources-color-width-label = Breedte
sources-color-height-label = Hoogte
sources-color-add = Kleur toevoegen
sources-testsignal-title = Testsignaal toevoegen
sources-testsignal-pattern-label = Patroon
sources-testsignal-bars = SMPTE-kleurbalken
sources-testsignal-grid = Kalibratieraster
sources-testsignal-sweep = Bewegingssweep
sources-testsignal-tone = 1 kHz-toon (−20 dBFS)
sources-testsignal-flash-beep = A/V-sync flits + piep
sources-testsignal-note = Controleer scènes, encoders, projectoren en streamdoelen zonder aangesloten camera. Het flits-en-piep-patroon voedt de A/V-sync-werkbank.
sources-testsignal-add = Testsignaal toevoegen
sources-timer-title = Timer toevoegen
sources-timer-mode-label = Modus
sources-timer-wall-clock = Klok
sources-timer-countdown = Aftellen
sources-timer-stopwatch = Stopwatch
sources-timer-since-live = Tijd sinds live
sources-timer-since-recording = Tijd sinds opname
sources-timer-note = Duur, formaat, stijl en einde-aftellen-acties staan in de Eigenschappen van de bron.
sources-timer-add = Timer toevoegen

# Text picker
sources-text-title = Tekst toevoegen
sources-text-label = Tekst
sources-text-default = Tekst
sources-text-color-label = Kleur
sources-text-color-aria = Tekstkleur
sources-text-size-label = Grootte (px)
sources-text-note = Lettertypefamilie, uitlijning, terugloop en RTL staan in de Eigenschappen van de bron. Het meegeleverde Noto Sans (incl. Arabisch/Hebreeuws) is de standaard — identiek op elke machine.
sources-text-add = Tekst toevoegen

# Existing source picker
sources-existing-title = Een bestaande bron toevoegen
sources-existing-empty = Er bestaan nog geen bronnen — voeg er eerst een toe aan een scène. Bestaande bronnen worden gedeeld: hernoemen of herconfigureren werkt elke scène bij die de bron toont.

# Screen + corners layout
sources-slot-off = Uit
sources-slot-center = Midden (scherm)
sources-slot-top-left = Linksboven
sources-slot-top-right = Rechtsboven
sources-slot-bottom-left = Linksonder
sources-slot-bottom-right = Rechtsonder
sources-layout-title = Schikken: scherm + hoeken
sources-layout-empty = Voeg eerst een schermopname en een of meer camera's toe aan deze scène, schik ze dan hier.
sources-layout-note = Zet een scherm in het midden en tot vier camera's in de hoeken — je uitleg-/podcastindeling. Elke hoek bevat een webcam, een vastgelegd gespreksvenster of een mediaclip. Je kunt ze daarna allemaal op het canvas verslepen.
sources-layout-slot-aria = Plek voor { $name }
sources-layout-apply = Indeling toepassen


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Bediening
controls-start-stop-title-stop = Stop en rond de opname af
controls-start-stop-title-start = Neem het programmasignaal op met de configuratie in Instellingen → Uitvoer
controls-finalizing = ◌ Afronden…
controls-stop-recording = ■ Opname stoppen
controls-start-recording = ● Opname starten
controls-marker-title = Zet op dit moment een hoofdstukmarkering — die belandt in de OPNAME (mkv-hoofdstukken of een sidecar-bestand). Platformzijdige streammarkeringen vereisen platformaccounts, waar deze app nooit om vraagt.
controls-marker = ◈ Markering
controls-pause-title-resume = Hervatten — het bestand loopt door als één aaneengesloten tijdlijn
controls-pause-title-pause = Pauzeren — er worden geen frames geschreven; hervatten vervolgt hetzelfde afspeelbare bestand
controls-resume-recording = ▶ Opname hervatten
controls-pause-recording = ⏸ Opname pauzeren
controls-reactions-label = Reacties (ingebakken in het programma)
controls-reactions-title = Laat een reactie over het programma zweven — opgenomen ÉN gestreamd, zodat de herhaling het exacte moment toont. Kijkers in de chat activeren deze ook (hun reactie-emoji zweven automatisch); een vloed beperkt alleen wat op het scherm staat.
controls-react = Reageer { $emoji }
controls-virtual-camera-title = De virtuele camera vereist per OS een eigen ondertekend stuurprogramma-component (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO-extensie / Linux v4l2loopback) — het komt als eigen mijlpaal. Het signaalmodel is er klaar voor: programma, verticaal canvas of één bron, met een gekoppelde virtuele microfoon op Windows/Linux (macOS heeft geen virtuele-microfoon-API — eerlijk gezegd).
controls-virtual-camera = ⌁ Virtuele camera starten
controls-saved = Opgeslagen: { $path }

# --- MixerDock.tsx ---
mixer-title = Audiomixer
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Overschakelen naar horizontale stroken
mixer-switch-to-vertical = Overschakelen naar verticale stroken
mixer-layout-aria-vertical = Mixerindeling: verticaal — overschakelen naar horizontaal
mixer-layout-aria-horizontal = Mixerindeling: horizontaal — overschakelen naar verticaal
mixer-empty = Geen audiobronnen in deze scène — voeg met "+" bij Bronnen een audio-invoeropname (microfoon) of audio-uitvoeropname (desktopaudio) toe. Stroken krijgen een VU-meter, fader, demping, monitoring, filters en tracktoewijzing.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Programmaluidheid (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Momentane luidheid (400 ms)
mixer-short-term-title = Kortetermijnluidheid (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Monitoruitvoerapparaat
mixer-default-output = Standaarduitvoer
mixer-routing = Routering
mixer-routing-title = Audio-uitvoerroutering

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Audioroutering
routing-intro = Wijs strooks toe aan trackbussen en stuur vervolgens elke bus naar een fysieke uitgang — een feed voor een hardwarerecorder, luidsprekers in een andere kamer of een koptelefooncue op een vrije track. De monitor houdt zijn eigen apparaat; deze routes komen er bovenop, dus zonder ingestelde route blijft de mix ongewijzigd.
routing-sends-title = Track-sends
routing-no-strips = Geen audiobronnen in deze scène.
routing-source = Bron
routing-track = Track { $n }
routing-send-aria = { $source } naar track { $n } sturen
routing-outputs-title = Fysieke uitgangen
routing-master = Master
routing-off = Uit
routing-default-output = Standaarduitvoer
routing-device-aria = Uitvoerapparaat voor { $bus }
routing-trim-aria = Uitvoer-trim voor { $bus }
routing-trim-db = { $db } dB
routing-muted = Gedempt
routing-device-error = Apparaat niet beschikbaar

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Ducking
mixer-ducking-title = Ducking-matrix
ducking-title = Ducking-matrix
ducking-intro = Elke bron kan elke andere ducken. Een cel verlaagt het doel (kolom) zodra de trigger (rij) klinkt — kies een cel om de diepte, drempel en timing in te stellen. Elk paar is zijn eigen ducking, dus één kanaal kan door meerdere triggers tegelijk worden geduckt.
ducking-need-two = Voeg minstens twee audiobronnen toe om ertussen te ducken.
ducking-trigger-target = Trigger ↓ / Doel →
ducking-cell-aria = { $trigger } duckt { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Verwijderen
ducking-amount = Hoeveelheid
ducking-threshold = Drempel
ducking-attack = Attack
ducking-release = Release
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Luidheidsnormalisatie
loudness-intro = Stuurt het programma geleidelijk naar een luidheidsdoel met een piekplafond, zodat je stream en opnames op een consistent niveau uitkomen. Langzaam en voorzichtig — het stuurt, het pompt nooit.
loudness-enable = Programma naar het doel sturen
loudness-target = Doel
loudness-target-option = { $target } LUFS
loudness-ceiling = Piekplafond (dBFS)
loudness-note = −14 LUFS past bij YouTube-achtige weergave; −16 is een gangbaar streamingdoel; −23 is EBU R128-uitzending. Dezelfde doelwaarde wordt gebruikt door de Normaliseren-actie na de opname.
loudness-on = LUFS { $target }
loudness-off = Norm. uit

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Soundboard
mixer-soundboard-title = Soundboard
soundboard-title = Soundboard
soundboard-add-pad = + Pad
soundboard-stop-all = Alles stoppen
soundboard-edit = Bewerken
soundboard-empty = Nog geen pads — voeg er een toe en wijs een lokale audioclip toe.
soundboard-new-pad = Nieuwe pad
soundboard-no-clip = Geen clip
soundboard-audio-files = Audiobestanden
soundboard-name = Naam
soundboard-choose-clip = Clip kiezen…
soundboard-gain = Versterking
soundboard-choke = Choke
soundboard-choke-none = Geen
soundboard-loop = Herhalen
soundboard-auto-duck = Auto-ducking
soundboard-tracks = Tracks
soundboard-hotkey = Sneltoets
soundboard-hotkey-placeholder = bijv. Ctrl+Shift+1
soundboard-remove = Verwijderen

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Plug-ins
mixer-plugins-title = Audioplug-ins (CLAP / VST3)
plugins-title = Audioplug-ins
plugins-scanning = Scannen…
plugins-none = Geen CLAP- of VST3-plug-ins gevonden in de standaardmappen.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Geheugen
stats-dropped = Verloren
stats-render = Render
stats-gpu = GPU
stats-gpu-compositing = compositen
stats-gpu-idle = inactief
stats-disk = Schijf
stats-disk-free = vrij
stats-disk-left = Opn. resterend
stats-disk-rate = ≈ { $rate } MB/s opname
stats-vertical-fps = 9:16 FPS
stats-targets-label = Streamdoelen
stats-shared-encode = · gedeelde encode
stats-starting = De compositor starten…

# --- ScenesRail.tsx ---
scenes-title = Scènes
scenes-new-scene-name = Scène
scenes-add = Een scène toevoegen
scenes-empty = Verbinden met de studiokern…
scenes-rename = { $name } hernoemen
scenes-on-program = Op programma
scenes-preview = Voorbeeld van { $name }
scenes-switch-to = Overschakelen naar { $name }
scenes-move-up = Omhoog verplaatsen
scenes-move-up-aria = { $name } omhoog verplaatsen
scenes-move-down = Omlaag verplaatsen
scenes-move-down-aria = { $name } omlaag verplaatsen
scenes-last-stays = De laatste scène blijft
scenes-remove = Deze scène verwijderen
scenes-remove-aria = { $name } verwijderen


# =============================================================
# --- components ---
# =============================================================
# components
# Extracted user-visible strings from ui/src/components/*.tsx:
#   ChannelStrip, LiveButton, RecDot, ReplayControls,
#   PropertiesDialog, AudioFiltersDialog, FiltersDialog, PickerShell.
# (Panel.tsx and NumberField.tsx render only caller-supplied props — no literals.)
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- ChannelStrip.tsx ---
channelstrip-level = Niveau
channelstrip-monitor-off = Monitor uit
channelstrip-monitor-only = Alleen monitor (niet in de mix)
channelstrip-monitor-and-output = Monitor en uitvoer
channelstrip-status-error = fout
channelstrip-status-live = live
channelstrip-status-waiting-audio = wachten op audio
channelstrip-status = status: { $state }
channelstrip-status-waiting = wachten
channelstrip-mute = Dempen
channelstrip-unmute = Demping opheffen
channelstrip-mute-source = { $name } dempen
channelstrip-unmute-source = Demping van { $name } opheffen
channelstrip-scene-mix-on = Mix per scène AAN — deze strook overschrijft de globale mix voor deze scène (klik om de globale mix weer te volgen)
channelstrip-scene-mix-off = Mix per scène — geef deze strook een eigen fader/demping voor de huidige scène
channelstrip-scene-mix-label = Mix per scène voor { $name }
channelstrip-monitor-cycle = { $mode } — klik om te wisselen
channelstrip-monitor-mode = Monitormodus van { $name }: { $mode }
channelstrip-audio-filters-title = Audiofilters (ruisonderdrukking, gate, compressor…)
channelstrip-audio-filters-label = Audiofilters voor { $name }
channelstrip-advanced-title = Sync-offset & push-to-talk-sneltoetsen
channelstrip-advanced-label = Geavanceerde audio-instellingen voor { $name }
channelstrip-track-assignment = Tracktoewijzing
channelstrip-track = Track { $n }
channelstrip-track-assigned = Track { $n } (toegewezen)
channelstrip-track-label = Track { $n } voor { $name }
channelstrip-device-error = apparaatfout
channelstrip-audio-device-error = audioapparaatfout
channelstrip-volume-label = Volume van { $name } in decibel
channelstrip-ptt-hold = Push-to-talk: houd { $key } ingedrukt
channelstrip-sync-offset = Sync-offset (ms, 0–{ $max } — vertraagt deze audio)
channelstrip-solo-title = Solo (PFL) — de monitor hoort alleen gesoleerde strips; de programmamix blijft intact
channelstrip-solo-source = { $name } solo (PFL)
channelstrip-pan-label = Balans (dubbelklik herstelt)
channelstrip-pan-aria = Balans van { $name }
channelstrip-mono-label = Downmixen naar mono
channelstrip-automix-label = Auto-mix (gain-sharing)
channelstrip-automix-note = Gain-sharing: de mixer houdt het gecombineerde niveau van alle auto-mix-stroken stabiel en geeft het aan wie er spreekt — ideaal voor panels met meerdere microfoons en podcasts. Uit totdat je een strook toevoegt.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Genereert een echovrije return voor deze bron — iedereen in het programma behalve deze bron zelf. Gebruik het voor een externe gast zodat die zijn eigen vertraagde stem niet hoort.
channelstrip-ptt-hotkey = Push-to-talk-sneltoets (stil tenzij ingedrukt)
channelstrip-ptt-placeholder = bijv. Ctrl+Shift+T of F13
channelstrip-ptt-aria = Push-to-talk-sneltoets
channelstrip-ptm-hotkey = Push-to-mute-sneltoets (stil terwijl ingedrukt)
channelstrip-ptm-placeholder = bijv. Ctrl+Shift+M
channelstrip-ptm-aria = Push-to-mute-sneltoets
channelstrip-hotkeys-note = Sneltoetsen werken terwijl andere apps de focus hebben. Op Linux/Wayland zijn globale sneltoetsen mogelijk niet beschikbaar — dat is een compositorbeperking, eerlijk gezegd.
channelstrip-apply = Toepassen


# --- LiveButton.tsx ---
livebutton-failure-ended = de stream is beëindigd
livebutton-title-live = Beëindig de stream — elk doel (een lopende opname gaat door)
livebutton-title-offline = Ga live naar elk ingeschakeld Instellingen → Stream-doel
livebutton-end-stream = ■ Stream beëindigen
livebutton-aria-reconnecting = Opnieuw verbinden
livebutton-aria-live = Live
livebutton-badge-retry = poging { $n }
livebutton-badge-live = live
livebutton-go-live = ⦿ Ga live


# --- RecDot.tsx ---
recdot-paused-aria = Opname gepauzeerd
recdot-recording-aria = Opnemen
recdot-tracks-one = { $count } audiotrack opnemen
recdot-tracks-other = { $count } audiotracks opnemen
recdot-paused = gepauzeerd


# --- ReplayControls.tsx ---
replaycontrols-saved = Replay opgeslagen — { $name }
replaycontrols-failure-stopped = de buffer is gestopt
replaycontrols-title-disarm = Ontgrendel de replaybuffer (verwijdert de niet-opgeslagen historie)
replaycontrols-title-arm = Activeer de doorlopende replaybuffer — houdt de laatste N seconden klaar om op te slaan (eigen lichtgewicht encode; de stream en opname blijven onaangeroerd)
replaycontrols-replay-seconds = ⟲ Replay { $seconds }s
replaycontrols-arm = ⟲ Replaybuffer activeren
replaycontrols-save-title = Sla de laatste N seconden op in de opnamemap (ook via de sneltoets Replay opslaan)
replaycontrols-save = ⤓ Opslaan


# --- PropertiesDialog.tsx ---
properties-title = Eigenschappen — { $name }
properties-name = Naam
properties-cancel = Annuleren
properties-apply = Toepassen
properties-youtube = YouTube — kanaal- / watch- / live_chat-URL (nooit een sleutel of aanmelding)
properties-twitch = Twitch — kanaalnaam (anoniem)
properties-kick = Kick — kanaalslug (openbaar endpoint)
properties-width-px = Breedte (px)
properties-lines = Regels
properties-font-px = Lettertype (px)
properties-images = Afbeeldingsbestanden (één pad per regel, in volgorde getoond)
properties-per-slide = Per dia (ms)
properties-crossfade = Crossfade (ms, 0 = harde snit)
properties-loop-slideshow = Herhalen (uit = laatste dia vasthouden)
properties-shuffle = Elke cyclus schudden
properties-nested-scene = Scène die deze bron samenstelt (een scène die deze al bevat wordt geweigerd)
properties-portal-note = De Wayland ScreenCast-portal kiest het scherm of venster in het systeemvenster telkens als deze bron start — er valt hier niets in te stellen, met opzet.
properties-appaudio-capturing = Audio vastleggen van { $exe }
properties-appaudio-exe-fallback = een applicatie
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Voeg de bron opnieuw toe om een andere app te richten (een proces-id verandert wanneer de app opnieuw start).
properties-image-file = Afbeeldingsbestand
properties-media-file = Mediabestand (mp4, mkv, webm, mov, .frec of een afbeelding)
properties-media-loop = Herhalen (aan het einde opnieuw vanaf het begin)
properties-media-hwdecode = Hardware-decodering (valt vanzelf terug op software)
properties-media-note = .frec speelt af via de eigen freally-video-codec — niets te downloaden. Andere videoformaten decoderen via het on-demand FFmpeg-component. De audio van het bestand krijgt een eigen mixerstrook; de sync-offset van de strook stelt de A/V-uitlijning fijn af. Een clip zonder audio laat zijn strook stil.
properties-color = Kleur
properties-width = Breedte
properties-height = Hoogte
properties-testtone-note = Een continue 1 kHz-sinus op −20 dBFS. Niveau en dempen zitten op de mixerstrip; verder valt er niets in te stellen.
properties-timer-format = Tijdformaat (strftime)
properties-timer-format-note = bijv. %H:%M:%S (standaard), %I:%M %p, %A %H:%M — een ongeldig patroon valt terug op %H:%M:%S.
properties-timer-utc = UTC-verschuiving (minuten)
properties-timer-utc-placeholder = lokale tijd
properties-timer-duration = Duur (seconden)
properties-timer-target = Aftellen tot (HH:MM)
properties-timer-target-note = Een kloktijd-doel loopt vanzelf en herhaalt dagelijks; laat leeg om de duur met Start/Pauze/Reset te gebruiken.
properties-timer-end = Bij nul
properties-timer-end-none = Niets doen
properties-timer-end-flash = Timer laten knipperen
properties-timer-end-switch = Scène wisselen
properties-timer-end-scene = Scène
properties-timer-size = Grootte (px)
properties-timer-start = Start
properties-timer-pause = Pauze
properties-timer-reset = Reset
properties-text-file = Uit bestand lezen (pad; leeg = tekst hierboven)
properties-text-binding = Interpreteren als
properties-text-binding-whole = Heel bestand
properties-text-binding-csv = CSV-cel
properties-text-binding-json = JSON-pointer
properties-text-csv-row = Rij
properties-text-csv-column = Kolom
properties-text-csv-column-placeholder = naam of nummer
properties-text-json-pointer = Pointer
properties-text-file-note = Het bestand wordt binnen een halve seconde na een wijziging opnieuw gelezen. Atomaire schrijvers (temp + hernoemen) worden verdragen: de laatste goede waarde blijft tijdens de wissel in beeld.
avsync-title = A/V-sync-kalibratie
avsync-intro = Speel het ingebouwde flits + piep-patroon af via je scherm en speakers, vang het met de camera en microfoon die je wilt uitlijnen — de werkbank meet het verschil. De lus loopt via scherm en speakers, dus hun kleine latenties tellen mee.
avsync-video-label = Camera (videobron)
avsync-audio-label = Microfoon (audiobron)
avsync-pick = Kies een bron…
avsync-no-video = Voeg de camera eerst als bron toe — de werkbank meet bronnen, geen kale apparaten.
avsync-no-audio = Voeg de microfoon eerst als audiobron toe.
avsync-projector = Programma schermvullend tonen op
avsync-projector-open = Projector openen
avsync-projector-window-title = Programma — A/V-sync
avsync-start-note = Starten legt tijdelijk een "A/V-sync-patroon"-bron boven op de huidige scène en speelt de piep op het monitorapparaat. Alles wordt na afloop verwijderd.
avsync-manual = Sync-offset (ms, handmatig)
avsync-start = Kalibratie starten
avsync-measuring = Meting van zo'n 12 seconden — richt de camera op het knipperende programma en houd de ruimte rustig…
avsync-flash-seen = Camera ziet de flits
avsync-flash-waiting = Wachten tot de camera de flits ziet…
avsync-beep-heard = Microfoon hoort de piep
avsync-beep-waiting = Wachten tot de microfoon de piep hoort…
avsync-cancel = Annuleren
avsync-result-offset = De video komt { $offset } ms na de audio aan.
avsync-result-detail = Gemeten over { $cycles } cycli, ±{ $jitter } ms.
avsync-negative = De audio komt al later aan dan de video. Audio vertragen lost deze richting niet op — draagt een andere strip het geluid van deze camera, verlaag daar dan de offset.
avsync-over-cap = Het gemeten verschil ligt boven de offsetgrens van { $max } ms. Zo'n gat betekent meestal de verkeerde bron — controleer de keten en meet opnieuw.
avsync-applied = Toegepast — de sync-offset van de microfoon is nu { $offset } ms.
avsync-apply = { $offset } ms toepassen op de microfoon
avsync-again = Opnieuw meten
avsync-close = Sluiten
avsync-error-noFlash = De camera zag de flits nooit. Richt hem op het knipperende programma (schermvullend helpt), controleer of de bron live is en meet opnieuw.
avsync-error-noBeep = De microfoon hoorde de piep nooit. Controleer of het monitorapparaat hoorbaar is en de microfoon live (niet door push-to-talk gedempt), en meet opnieuw.
avsync-error-tooFewCycles = Te weinig schone flits/piep-cycli. Houd het patroon de hele meting goed zichtbaar en hoorbaar.
avsync-error-notThePattern = Wat gezien of gehoord is herhaalt zich niet in het ritme van het patroon — waarschijnlijk kamerlicht of lawaai, niet het testsignaal.
avsync-error-unstable = De cycli spreken elkaar te veel tegen voor één getal. Stabiliseer de camera, verminder het lawaai en meet opnieuw.
hotkey-audit-title = Sneltoetsenkaart
hotkey-audit-search = Zoeken
hotkey-audit-filter = Functie
hotkey-audit-filter-all = Alle functies
hotkey-audit-col-key = Toets
hotkey-audit-col-action = Actie
hotkey-audit-col-where = Waar
hotkey-audit-col-status = Status
hotkey-audit-ok = OK
hotkey-audit-shared = Gedeeld door { $count } toewijzingen
hotkey-audit-unregistered = Niet bij het OS geregistreerd (elders in gebruik of niet beschikbaar)
hotkey-audit-invalid = Geen geldige sneltoets
hotkey-audit-empty = Nog geen sneltoetsen — wijs ze toe in Instellingen → Sneltoetsen of op een mixerstrip.
hotkey-audit-export = Spiekbriefje exporteren
hotkey-audit-exported = Opgeslagen in { $path }
hotkey-audit-note = Toetsen toewijzen en wijzigen doe je in Instellingen → Sneltoetsen (globale acties) en op elke mixerstrip (push-to-talk / push-to-mute); deze tabel controleert en documenteert ze.
hotkey-audit-action-record = Opname wisselen
hotkey-audit-action-go-live = Streamen wisselen
hotkey-audit-action-transition = Overgang uitvoeren
hotkey-audit-action-save-replay = Replay opslaan
hotkey-audit-action-add-marker = Markering toevoegen
hotkey-audit-action-still = Still vastleggen
hotkey-audit-action-panic = Paniekscherm
hotkey-audit-action-timer-toggle = Alle timers starten/pauzeren
hotkey-audit-action-timer-reset = Alle timers resetten
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Opname
hotkey-audit-feature-streaming = Streamen
hotkey-audit-feature-studio = Studiomodus
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Markeringen
hotkey-audit-feature-stills = Stills
hotkey-audit-feature-panic = Paniek
hotkey-audit-feature-timers = Timers
hotkey-audit-feature-audio = Audio (per bron)
properties-text = Tekst
properties-font-family = Lettertypefamilie (systeem; leeg = standaard)
properties-size-px = Grootte (px)
properties-text-color = Tekstkleur
properties-align = Uitlijnen
properties-align-left = links
properties-align-center = midden
properties-align-right = rechts
properties-line-spacing = Regelafstand
properties-wrap-width = Terugloopbreedte (px; 0 = uit)
properties-force-rtl = Rechts-naar-links forceren
properties-text-note = Rendering gebruikt echte shaping (Arabische verbindingen, ligaturen) en bidi-regelvolgorde. De meegeleverde Noto Sans-familie (incl. Arabisch/Hebreeuws) is de standaard; systeemfamilies werken ook. CJK gebruikt voorlopig systeemlettertypen.
properties-repick-capturing = Vastleggen: { $label }
properties-repick-looking = Zoeken naar bronnen…
properties-repick-none-displays = Geen beeldschermen gevonden om opnieuw te kiezen.
properties-repick-none-windows = Geen vensters gevonden om opnieuw te kiezen.
properties-repick-again = Opnieuw kiezen:
properties-device = Apparaat
properties-video-current-device = (huidig apparaat)
properties-format = Formaat
properties-format-auto-loading = Automatisch (formaten laden…)
properties-deinterlace = Deinterlacing
properties-deinterlace-off = Uit
properties-deinterlace-discard = Verwerpen (één veld lijnverdubbelen)
properties-deinterlace-bob = Bob (velden afwisselen)
properties-deinterlace-linear = Lineair (interpoleren)
properties-deinterlace-blend = Mengen (velden middelen)
properties-deinterlace-adaptive = Bewegingsadaptief (yadif-klasse)
properties-field-order = Veldvolgorde
properties-field-order-top = Bovenste veld eerst
properties-field-order-bottom = Onderste veld eerst
properties-deinterlace-note = Voor interlaced capturekaart-signalen. Pure CPU, identiek op elk OS; wijzigen herstart het apparaat (zoals een formaatwissel).
camera-controls-title = Camerabediening
camera-controls-refresh = Vernieuwen
camera-controls-reset = Profiel resetten
camera-controls-empty = Nu geen regelaars — het apparaat moet streamen (voeg het eerst aan een scène toe), en sommige backends melden er geen (vooral macOS). Dit is de eerlijke stand per OS.
camera-controls-note = Wijzigingen gelden live en worden in het apparaatprofiel opgeslagen, dat bij herverbinden en herstart opnieuw wordt toegepast.
camera-control-brightness = Helderheid
camera-control-contrast = Contrast
camera-control-hue = Tint
camera-control-saturation = Verzadiging
camera-control-sharpness = Scherpte
camera-control-gamma = Gamma
camera-control-white-balance = Witbalans
camera-control-backlight = Tegenlichtcompensatie
camera-control-gain = Versterking
camera-control-pan = Pannen
camera-control-tilt = Kantelen
camera-control-zoom = Zoom
camera-control-exposure = Belichting
camera-control-iris = Iris
camera-control-focus = Scherpstelling
properties-format-auto = Automatisch (hoogste resolutie)
properties-audio-capture-of = Leg de audio vast van
properties-audio-default-output = Standaarduitvoer (wat je hoort)
properties-audio-default-input = Standaardinvoer
properties-audio-default-suffix = (standaard)
properties-audio-current-device = (huidig apparaat: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Versterking
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Compressor
audiofilters-name-limiter = Limiter
audiofilters-name-eq = 3-Bands EQ
audiofilters-name-denoise = Ruisonderdrukking
audiofilters-name-ducking = Ducking
audiofilters-name-parametric-eq = Parametrische EQ
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Rommelfilter
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Preset
audiofilters-voice-preset-pick = Stem-preset…
audiofilters-voice-broadcast = Broadcaststem
audiofilters-voice-podcast = Podcaststem
audiofilters-voice-clean = Heldere stem
audiofilters-voice-none = Keten wissen
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Sisfrequentie (Hz)
audiofilters-deesser-amount = Max. reductie (dB)
audiofilters-rumble-freq = Low-cut (Hz)
audiofilters-title = Audiofilters — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Frequentiekromme van de parametrische EQ met live spectrum
eq-band-type = Type
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Band
eq-remove-band = Band verwijderen
eq-type-bell = Bell
eq-type-lowShelf = Low shelf
eq-type-highShelf = High shelf
eq-type-notch = Notch
eq-type-highPass = Hoogdoorlaat
eq-type-lowPass = Laagdoorlaat
audiofilters-chain-header = Filterketen (bovenste draait eerst, vóór de fader)
audiofilters-add = + Filter toevoegen
audiofilters-add-menu = Een audiofilter toevoegen
audiofilters-empty = Nog geen filters — onderdruk ruis op een microfoon (klassieke DSP, geen ML), gate de ruimte, temper pieken met de compressor of duck muziek onder je stem.
audiofilters-enable = { $name } inschakelen
audiofilters-run-earlier = Eerder uitvoeren
audiofilters-move-up = { $name } omhoog
audiofilters-run-later = Later uitvoeren
audiofilters-move-down = { $name } omlaag
audiofilters-remove-title = Filter verwijderen
audiofilters-remove = { $name } verwijderen
audiofilters-gain-db = Versterking (dB)
audiofilters-open-db = Openen bij (dB)
audiofilters-close-db = Sluiten bij (dB)
audiofilters-attack-ms = Attack (ms)
audiofilters-hold-ms = Hold (ms)
audiofilters-release-ms = Release (ms)
audiofilters-ratio = Ratio (:1)
audiofilters-threshold-db = Drempel (dB)
audiofilters-output-gain-db = Uitvoerversterking (dB)
audiofilters-ceiling-db = Plafond (dB)
audiofilters-low-db = Laag (dB)
audiofilters-mid-db = Midden (dB)
audiofilters-high-db = Hoog (dB)
audiofilters-strength = Sterkte
audiofilters-denoise-note = Eigen klassieke-DSP spectrale onderdrukking — constante ruis (ventilatoren, sis) daalt terwijl spraak doorkomt. Geen ML, geen modellen, conform het charter.
audiofilters-duck-under = Ducken onder
audiofilters-ducking-trigger = Triggerbron voor ducking
audiofilters-pick-trigger = (kies een trigger — bijv. je microfoon)
audiofilters-trigger-at-db = Triggeren bij (dB)
audiofilters-duck-by-db = Ducken met (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Rendervertraging
filters-name-color-correction = Kleurcorrectie
filters-name-lut = LUT toepassen
filters-name-blur = Vervagen
filters-name-mask = Afbeeldingsmasker
filters-name-sharpen = Verscherpen
filters-name-scroll = Scrollen
filters-name-crop = Bijsnijden
filters-title = Filters — { $name }
filters-blend-mode = Overvloeimodus
filters-chain-header = Filterketen (bovenste draait eerst)
filters-add = + Filter toevoegen
filters-add-menu = Een filter toevoegen
filters-empty = Nog geen filters — chroma-key een webcam, kleurcorrigeer een opname of scroll een ticker.
filters-enable = { $name } inschakelen
filters-run-earlier = Eerder uitvoeren
filters-move-up = { $name } omhoog
filters-run-later = Later uitvoeren
filters-move-down = { $name } omlaag
filters-remove-title = Filter verwijderen
filters-remove = { $name } verwijderen
filters-key-color-rgb = Sleutelkleur (elke kleur, RGB-afstand)
filters-similarity = Gelijkenis
filters-smoothness = Gladheid
filters-luma-min = Luma min (donkerder valt weg)
filters-luma-max = Luma max (lichter valt weg)
filters-delay = Vertraging (ms — alleen video, bijv. om met audio te synchroniseren; gemaximeerd op 500)
filters-key-color = Sleutelkleur
filters-spill = Spill
filters-gamma = Gamma
filters-brightness = Helderheid
filters-contrast = Contrast
filters-saturation = Verzadiging
filters-hue-shift = Tintverschuiving
filters-opacity = Dekking
filters-cube-file = .cube-bestand
filters-amount = Hoeveelheid
filters-radius = Straal
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galerij
filters-shader-gallery-pick = Voorinstelling laden…
filters-shader-gallery-grayscale = Grijstinten
filters-shader-gallery-invert = Inverteren
filters-shader-gallery-scanlines = Scanlijnen
filters-shader-gallery-vignette = Vignet
filters-shader-source = Shaderbroncode (WGSL)
filters-shader-hint = Schrijf een WGSL-effect(uv, color, p, texel, time) dat een vec4 teruggeeft. Annoteer parameters met // @param name min max default voor schuifregelaars. Een ongeldige shader wordt genegeerd — de bron wordt ongefilterd weergegeven totdat hij compileert.
filters-name-bezier-mask = Bézier-masker
filters-mask-editor-hint = Sleep een punt om het te verplaatsen, dubbelklik om er een toe te voegen, rechtsklik op een punt om het te verwijderen.
filters-mask-shape = Vorm
filters-mask-shape-pick = Voorinstelling…
filters-mask-shape-rectangle = Rechthoek
filters-mask-shape-diamond = Ruit
filters-mask-shape-hexagon = Zeshoek
filters-mask-shape-circle = Cirkel
filters-mask-feather = Zachte rand
filters-mask-export-wipe = Exporteren als veeg…
filters-mask-image = Maskerafbeelding
filters-mask-mode = Modus
filters-mask-alpha = alfa
filters-mask-luma = luma
filters-mask-invert = omkeren
filters-speed-x = Snelheid X (px/s)
filters-speed-y = Snelheid Y (px/s)
filters-crop-left = links
filters-crop-top = boven
filters-crop-right = rechts
filters-crop-bottom = onder
filters-crop-aria = bijsnijden { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Vernieuwen
pickershell-refresh-title = De lijst vernieuwen
pickershell-close = Sluiten


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Meld een bug
bugreport-intro = Meldingen zijn anoniem en opt-in — er wordt niets automatisch verzonden. Je beoordeelt de exacte tekst hieronder en verzendt deze via een vooraf ingevulde GitHub-issue of je e-mailapp. Geen persoonlijke gegevens (je thuispad en gebruikersnaam worden geredigeerd); geen account, geen server.
bugreport-crash-notice = Freally Capture is bij een vorige run onverwacht gesloten — de anonieme crashdetails staan hieronder. Ze melden helpt het snel te verhelpen.
bugreport-description-label = Wat was je aan het doen toen het gebeurde? (optioneel)
bugreport-description-placeholder = bijv. het voorbeeld bevroor toen ik een tweede webcam toevoegde
bugreport-include-crash = De anonieme crashdetails van de laatste run meesturen
bugreport-preview-label = Precies wat er verzonden wordt
bugreport-open-github = GitHub-issue openen
bugreport-gmail-title = Opent Gmails opstelvenster in je browser, vooraf ingevuld. Afgemeld? Google toont eerst het inlogscherm.
bugreport-compose-gmail = Opstellen in Gmail
bugreport-email-title = Opent een concept in de mailapp die deze pc standaard gebruikt (Outlook, Thunderbird, Mail…)
bugreport-send-email = E-mail verzenden
bugreport-copied = Gekopieerd ✓
bugreport-copy-report = Rapport kopiëren
bugreport-dismiss-crash = Crash negeren
bugreport-copy-failed = kopiëren mislukt — selecteer de tekst en kopieer handmatig
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = WAT ER GEBEURDE
bugreport-preview-no-description = (geen beschrijving opgegeven)
bugreport-preview-diagnostics = ANONIEME DIAGNOSTIEK (geen persoonlijke gegevens)
bugreport-preview-from = Van: Freally Capture
bugreport-preview-crash-excerpt = --- crashfragment ---


# --- Updates.tsx ---
updates-title = Software-update
updates-checking = Controleren op updates…
updates-uptodate = Je hebt de nieuwste versie.
updates-check-again = Opnieuw controleren
updates-available = Versie { $version } is beschikbaar
updates-current-version = (je hebt { $current })
updates-release-notes-label = Versie { $version } — Releaseopmerkingen
updates-confirm = Wil je nu bijwerken? De download wordt geverifieerd tegen de meegeleverde ondertekeningssleutel voordat deze wordt toegepast. Freally Capture sluit, het installatieprogramma draait en de nieuwe versie opent vanzelf opnieuw.
updates-yes-update-now = Ja, nu bijwerken
updates-no-not-now = Nee, nu niet
updates-downloading = { $version } downloaden…
updates-starting = starten…
updates-installed = Update geïnstalleerd.
updates-restart-now = Nu herstarten
updates-restart-later = Later herstarten
updates-try-again = Opnieuw proberen


# --- Models.tsx ---
models-title = Componenten
models-ffmpeg-heading = FFmpeg — wire-codecs
models-badge-third-party = Van derden · niet meegeleverd
models-ffmpeg-desc = De eigen engine van Freally Capture neemt verliesloos freally-video (.frec) op zonder iets extra's. Opnemen in de wire-formaten die platforms en spelers verwachten — H.264/AAC (en HEVC/AV1) in mp4/mkv/mov/webm — gebruikt FFmpeg, een aparte tool die deze app nooit meelevert: die codecs zijn patentbelast, dus het blijft optioneel en duidelijk gelabeld. Het wordt op aanvraag gedownload van de vastgepinde build hieronder, SHA-256-geverifieerd vóór eerste gebruik, per gebruiker gecachet en als apart proces aangestuurd. De licentie (LGPL/GPL) is de zijne — zie THIRD-PARTY-NOTICES.
models-checking = Controleren…
models-ffmpeg-not-installed = Niet geïnstalleerd. Beschikbaar: FFmpeg { $version } van { $source } ({ $size } download).
models-ffmpeg-none-pinned = Er is nog geen FFmpeg-build vastgepind voor dit platform — wire-codec-opname is hier niet beschikbaar. Verliesloze freally-video-opname wordt niet beïnvloed.
models-ffmpeg-download-verify = Downloaden & verifiëren ({ $size })
models-downloading = Downloaden…
models-download-of = van
models-cancel = Annuleren
models-ffmpeg-verifying = De download verifiëren tegen de vastgepinde SHA-256…
models-ffmpeg-extracting = Uitpakken…
models-ffmpeg-ready = Geïnstalleerd & geverifieerd — { $version }
models-remove = Verwijderen
models-ffmpeg-retry = Download opnieuw proberen
models-network-note = De download is de enige netwerkactie op dit paneel en start nooit uit zichzelf. Een mislukte checksum breekt de installatie af — de app weigert bytes te draaien waar hij niet voor kan instaan.
models-cef-heading = Browserbron-runtime — Chromium (CEF)
models-cef-desc = Browserbronnen renderen webpagina's (meldingen, widgets, overlays) via Chromium Embedded Framework — een ~100 MB runtime die deze app nooit meelevert. Het downloadt op aanvraag van de officiële CEF-buildindex, wordt geverifieerd tegen de SHA-1 van die index voordat er iets wordt uitgepakt, en wordt per gebruiker gecachet. De browserbron die erdoor rendert komt met een eigen mijlpaal; dit installeert de runtime die hij nodig heeft.
models-cef-download-install = Downloaden & installeren
models-cef-unsupported = CEF publiceert geen build voor dit platform — browserbronnen zijn hier niet beschikbaar.
models-cef-resolving = De nieuwste stabiele build bepalen…
models-cef-verifying = De download verifiëren tegen de index-SHA-1…
models-cef-extracting = De runtime uitpakken…
models-cef-ready = Geïnstalleerd — CEF { $version }.
models-cef-retry = Opnieuw proberen
models-integrations-heading = Optionele integraties
models-badge-never-bundled = Nooit meegeleverd
models-ndi-detected = Gedetecteerd
models-ndi-not-installed = Niet geïnstalleerd
models-vst-available = Beschikbaar
models-vst-not-available = Niet beschikbaar


# --- Recordings.tsx ---
recordings-title = Opnamen
recordings-loading = De map lezen…
recordings-empty = Nog geen opnamen — Opname starten schrijft naar de map die is ingesteld bij Uitvoer.
recordings-frec-label = eigen verliesloos (freally-video)
recordings-remux-title = Herverpakken als mp4 — stream copy, geen herencodering, geen kwaliteitsverlies (vereist het FFmpeg-component)
recordings-remuxing = Hermuxen…
recordings-remux-to-mp4 = Hermuxen naar MP4
recordings-export-mp4-title = Decodeer de eigen .frec en herencodeer naar MP4 (H.264/AAC) zodat het in elke speler afspeelt — vereist het FFmpeg-component
recordings-exporting = Exporteren…
recordings-export-mp4 = Exporteren → MP4
recordings-export-mkv-title = Decodeer de eigen .frec en herencodeer naar MKV zodat het in elke speler afspeelt
recordings-starting = starten…
recordings-frames = { $done } / { $total } frames
recordings-cancel = Annuleren
recordings-export-cancelled = Export geannuleerd.
recordings-exported-to = Geëxporteerd naar { $path }
recordings-remuxed-to = Gehermuxed naar { $path }
recordings-normalize = Normaliseren
recordings-normalizing = Normaliseren…
recordings-normalize-title = Luidheid naar het doel normaliseren (schrijft een kopie)
recordings-normalized-to = Genormaliseerd naar { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Alleen audio
audiorec-format = Audio-opnameformaat
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Audio opnemen
audiorec-stop = Stoppen
audiorec-pause = Pauzeren
audiorec-resume = Hervatten
audiorec-recording = REC { $sec }s
audiorec-saved = { $count } trackbestand(en) opgeslagen


# --- OpenedFrec.tsx ---
openfrec-title = .frec-opname openen
openfrec-desc = Freally Capture neemt op in het eigen verliesloze .frec-formaat — het speelt het niet af. Freally Player speelt .frec direct af zodra het uitkomt. Exporteer het voorlopig naar MP4/MKV en het speelt in elke speler (VLC, je OS-speler, alles).
openfrec-exported-to = Geëxporteerd naar { $path }
openfrec-exporting = Exporteren…
openfrec-starting = starten…
openfrec-export-mp4 = Exporteren → MP4
openfrec-export-mkv = Exporteren → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Verticaal canvas (9:16)
vertical-enable = Schakel het tweede canvas in — apart van het programma op te nemen en te streamen
vertical-scene-label = Scène die dit canvas samenstelt
vertical-width = Breedte
vertical-height = Hoogte
vertical-preview-alt = Voorbeeld verticaal canvas
vertical-note = Itemposities zijn pixelnauwkeurig over canvassen heen: selecteer deze scène in de Scènes-balk om haar te schikken terwijl dit voorbeeld het verticale resultaat toont. Streamdoelen kiezen dit canvas in ⦿ Stream…; Instellingen → Uitvoer kan het naast het hoofdbestand opnemen.
vertical-close = Sluiten


# --- EulaGate.tsx ---
eula-title = Freally Capture — Licentieovereenkomst
eula-version = v{ $version }
eula-intro = Lees en accepteer deze overeenkomst om Freally Capture te gebruiken. Kort gezegd: het is een neutraal hulpmiddel, en jij bent als enige verantwoordelijk voor wat je vastlegt, opneemt en uitzendt — en voor het bezit van de rechten daarop.
eula-thanks = Bedankt voor het lezen.
eula-scroll-hint = Scroll naar het einde om door te gaan.
eula-decline = Weigeren & afsluiten
eula-agree = Ik ga akkoord


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Uitvoer
output-loading = Instellingen worden nog geladen…
output-container-frec = freally-video (.frec) — verliesloos, eigen, niets te downloaden
output-container-mkv = MKV — crashbestendig; hermux later naar mp4
output-container-mp4 = MP4 — speelt overal
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Verliesloos
output-preset-lossless-title = De eigen freally-video-codec — bit-exact, geen download
output-preset-high-label = Hoge kwaliteit
output-preset-high-title = MP4, best gedetecteerde encoder, bijna verliesloos CQ 16, voorinstelling Kwaliteit
output-preset-balanced-label = Gebalanceerd
output-preset-balanced-title = MKV, best gedetecteerde encoder, CQ 23, voorinstelling Gebalanceerd
output-recording-format = Opnameformaat
output-ffmpeg-warning = Dit formaat vereist het FFmpeg-component (wire-codecs — niet meegeleverd). Verliesloze .frec vereist niets.
output-install = Installeren…
output-recordings-folder = Opnamemap
output-folder-placeholder = OS-videomap
output-filename-prefix = Bestandsnaamvoorvoegsel
output-recording-template = Bestandsnaam voor opnamen
output-replay-template = Bestandsnaam voor replays
output-still-template = Bestandsnaam voor stilstaande beelden
output-template-tokens = Tokens: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Map voor replays
output-still-folder = Map voor stilstaande beelden
output-same-folder-placeholder = Opnamemap
output-frame-rate = Framerate
output-fps-option = { $fps } fps
output-split-every = Splitsen elke (minuten, 0 = uit)
output-output-width = Uitvoerbreedte (0 = canvas; alleen wire-formaten)
output-output-height = Uitvoerhoogte (0 = canvas)
output-record-vertical = Neem ook het verticale canvas op (een parallel "… (verticaal)"-bestand; vereist het 9:16-canvas ingeschakeld)
output-audio-tracks = Audiotracks
output-recorded-tracks-group = Opgenomen tracks
output-track-last-one = Minstens één track moet opnemen
output-record-track-on = Track { $index } opnemen: aan
output-record-track-off = Track { $index } opnemen: uit
output-encoder-heading = Encoder
output-video-encoder = Video-encoder
output-encoder-auto = Automatisch — best gedetecteerd (H.264)
output-encoder-unavailable = — hier niet beschikbaar
output-preset = Voorinstelling
output-preset-quality = Kwaliteit
output-preset-balanced-option = Gebalanceerd
output-preset-performance = Prestaties
output-rate-control = Bitratebeheer
output-rc-cqp = CQP (constante kwaliteit)
output-rc-cbr = CBR (constante bitrate)
output-rc-vbr = VBR (variabele bitrate)
output-cq = CQ (0–51, lager = beter)
output-bitrate = Bitrate (kbps)
output-keyframe = Keyframe-interval (s)
output-audio-bitrate = Audiobitrate (kbps / track)
output-presets = Voorinstellingen:

# --- SettingsStream.tsx ---
stream-title = Instellingen — Stream
stream-target-enabled = Doel { $index } ingeschakeld
stream-target = Doel { $index }
stream-remove = Verwijderen
stream-service = Dienst
stream-canvas = Canvas
stream-canvas-main = Hoofd (programma)
stream-canvas-vertical = Verticaal (9:16 — schakel het in de studio in)
stream-ingest-srt = SRT-ingest-URL
stream-ingest-whip = WHIP-endpoint-URL
stream-ingest-url = Ingest-URL
stream-ingest-override = (overschrijven — leeg = de voorinstelling van de dienst)
stream-key-srt = streamid (optioneel — toegevoegd als ?streamid=…; behandeld als geheim)
stream-key-whip = Bearer-token (optioneel — verzonden als de Authorization-header; een geheim)
stream-key-custom = Streamsleutel (van je server — behandeld als geheim)
stream-key-service = Streamsleutel (van je creator-dashboard — behandeld als geheim)
stream-key-aria = Streamsleutel { $index }
stream-key-hide = Verbergen
stream-key-show = Tonen
stream-encoder = Encoder (H.264 — wat RTMP, SRT en WHIP allemaal dragen)
stream-encoder-auto = Automatisch — de best gedetecteerde H.264-encoder
stream-encoder-unavailable = (hier niet beschikbaar)
stream-video-bitrate = Videobitrate (kbps, CBR)
stream-audio-bitrate = Audiobitrate (kbps)
stream-fps = FPS
stream-keyframe = Keyframe-interval (s)
stream-audio-track = Audiotrack (1–6)
stream-output-width = Uitvoerbreedte (0 = canvas)
stream-output-height = Uitvoerhoogte (0 = canvas)
stream-add-target = + Doel toevoegen
stream-go-live-note = Go Live publiceert tegelijk naar elk ingeschakeld doel, direct naar elk platform. Doelen met identieke encoder-instellingen delen één encode.
stream-auto-record = Opname starten wanneer ik live ga (de opname stopt nog steeds onafhankelijk)
stream-ffmpeg-note-before = Streaming-wire-codecs draaien via het gelabelde on-demand ffmpeg-component —
stream-ffmpeg-note-link = beheer het hier
stream-ffmpeg-note-after = . De lokale opname blijft doorlopen, wat de stream ook doet.
stream-cancel = Annuleren
stream-save = Opslaan

# --- SettingsReplay.tsx ---
replay-title = Instellingen — Replaybuffer
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Laag (3 Mbps)
replay-quality-standard = Standaard (6 Mbps)
replay-quality-high = Hoog (12 Mbps)
replay-length-presets = Lengtevoorinstellingen
replay-quality-presets = Kwaliteitsvoorinstellingen
replay-length-seconds = Lengte (seconden)
replay-video-bitrate = Videobitrate (kbps)
replay-fps = FPS
replay-audio-track = Audiotrack (1–6)
replay-note = Terwijl geactiveerd draait de buffer een eigen lichtgewicht encode in een begrensde on-disk-ring — ongeveer { $mb } MB bij deze instellingen. Opslaan stitcht de ring zonder herencodering en raakt de stream of opname nooit aan. Wijzigingen gelden de volgende keer dat je activeert.
replay-cancel = Annuleren
replay-save = Opslaan

# --- SettingsRemote.tsx ---
remote-title = Instellingen — Afstandsbediening
remote-enable = De WebSocket-remote-API inschakelen
remote-password = Wachtwoord (vereist — controllers authenticeren ermee)
remote-password-placeholder = een wachtwoord voor je controllers
remote-password-hide = Verbergen
remote-password-show = Tonen
remote-port = Poort
remote-allow-lan = LAN-verbindingen toestaan (standaard alleen deze machine)
remote-note = Uit = de poort is gesloten. Aan = een met een wachtwoord beveiligde WebSocket op 127.0.0.1 (of je LAN indien ingeschakeld) die scènes kan wisselen, de overgang uitvoeren, de stream en opname starten/stoppen, replays opslaan en dempingen/volumes instellen — dezelfde acties als de UI, niets meer. Het kan geen bestanden lezen. Behandel het wachtwoord als elke inloggegeven; kies bij voorkeur alleen-deze-machine tenzij je specifiek vanaf een ander apparaat bestuurt.
remote-password-required = Er is een wachtwoord vereist om de remote-API in te schakelen.
remote-cancel = Annuleren
remote-save = Opslaan

# --- SettingsHotkeys.tsx ---
hotkeys-title = Instellingen — Sneltoetsen
hotkeys-record = Opname starten / stoppen
hotkeys-go-live = Go Live / Stream beëindigen
hotkeys-transition = Studiomodus-overgang
hotkeys-save-replay = Replay opslaan (laatste N seconden)
hotkeys-add-marker = Hoofdstukmarkering zetten (opname)
hotkeys-note = Sneltoetsen zijn globaal — ze werken terwijl andere apps de focus hebben. Leeg = niet toegewezen. Mixer push-to-talk/-mute-toetsen staan in het ⋯-menu van elke strook. Op Linux/Wayland zijn globale sneltoetsen mogelijk niet beschikbaar (een compositorbeperking) — de knoppen blijven werken.
hotkeys-cancel = Annuleren
hotkeys-save = Opslaan

# --- WorkspaceDialog.tsx ---
workspace-title = Profielen & scèneverzamelingen
workspace-profiles = Profielen
workspace-profiles-hint = Een profiel zijn je instellingen — streamdoel, uitvoer, sneltoetsen. Wissel per show of per platform.
workspace-collections = Scèneverzamelingen
workspace-collections-hint = Een verzameling zijn je scènes + bronnen. Maken dupliceert de huidige als startpunt.
workspace-active = Actief
workspace-switch-to = Overschakelen naar { $name }
workspace-active-marker = ● actief
workspace-new-name-placeholder = nieuwe naam…
workspace-new-name-label = Nieuwe naam voor { $title }
workspace-create = Maken

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importeren uit OBS…
workspace-import-obs-hint = Haal een OBS-scènecollectie binnen (de scenes.json). Je huidige collectie wordt eerst opgeslagen.
workspace-import-busy = Importeren…
workspace-import-title = "{ $name }" geïmporteerd
workspace-import-summary = { $scenes } scènes · { $sources } bronnen · { $items } items
workspace-import-dismiss = Sluiten
workspace-import-clean = Alles is netjes geïmporteerd.
workspace-import-geometry-caveat = Groottes en posities worden aangepast op basis van de OBS-indeling — controleer elke scène en kies opnameapparaten opnieuw.
workspace-import-notes-title = Geïmporteerd met opmerkingen
workspace-import-skipped-title = Niet geïmporteerd
import-note-needsReselect = Kies apparaat/monitor/venster opnieuw
import-note-gameCaptureAsWindow = Spelopname → Vensteropname
import-note-referencesFile = Controleer het bestandspad
import-note-filterDropped = Sommige filters niet ondersteund
import-note-geometryApproximated = Positie/grootte benaderd
import-skip-unsupportedKind = Geen gelijkwaardig brontype
import-skip-group = Groepen worden nog niet ondersteund

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Ontbrekende bestanden opnieuw koppelen…
doctor-title = Ontbrekende bestanden
doctor-scanning = Scannen…
doctor-all-good = Alle bestanden waarnaar wordt verwezen bestaan. Niets om te koppelen.
doctor-intro = { $count } bestanden waarnaar wordt verwezen zijn niet op deze computer gevonden. Wijs elk een nieuwe locatie toe — elke scène die het gebruikt wordt meteen hersteld.
doctor-relinked = { $count } verwijzingen opnieuw gekoppeld.
doctor-uses = { $count }× gebruikt
doctor-locate = Zoeken…
doctor-locate-folder = In map zoeken…
doctor-locate-folder-hint = Kies een map; elk ontbrekend bestand wordt op naam gevonden en opnieuw gekoppeld.
doctor-kind-image = afbeelding
doctor-kind-media = media
doctor-kind-slideshow = diavoorstelling
doctor-kind-font = lettertype
doctor-kind-lut = LUT
doctor-kind-mask = masker
history-relinkFiles = Bestanden opnieuw koppelen

# --- ScriptsDialog.tsx ---
scripts-title = Scripts (Lua)
scripts-empty = Nog geen scripts — voeg een .lua-bestand toe. Zie scripts/sample.lua voor de API: reageer op go-live-/scène-/opname-events en bestuur dezelfde commando's als de remote-API.
scripts-enable = { $path } inschakelen
scripts-remove = { $path } verwijderen
scripts-path-label = Scriptpad
scripts-add = Toevoegen
scripts-note = Scripts draaien sandboxed — geen bestands- of OS-toegang; ze kunnen alleen dezelfde studiocommando's aanroepen als de remote-API (scènes wisselen, overgang, opnemen/streamen/replay, dempingen). Een scriptfout wordt gelogd en ingeperkt. Wijzigingen gelden binnen een seconde.
scripts-error-not-lua = Wijs naar een .lua-bestand.

# --- BrowserDock.tsx ---
browser-dock-title = Browserdocks
browser-dock-empty = Nog geen docks — voeg een chat-popout, een meldingenpagina of je Companion-webknoppen toe.
browser-dock-open = Openen
browser-dock-remove = { $name } verwijderen
browser-dock-name-placeholder = naam (bijv. Twitch-chat)
browser-dock-name-label = Docknaam
browser-dock-url-label = Dock-URL
browser-dock-note = Een dock opent als eigen venster dat je naast de studio kunt plaatsen. De pagina krijgt geen toegang tot de app — hij rendert alleen. Alleen http(s)-URL's; docks openen alleen wanneer je op Openen klikt.
browser-dock-error-name = Geef de dock een naam (bijv. Twitch-chat).
browser-dock-error-url = Een dock-URL moet beginnen met http:// of https://.

# --- studio-preview-pane ---
studio-preview-label = Studiomodus-voorbeeld
studio-preview-heading = Voorbeeld
studio-preview-hint = klik op een scène om die hier te laden
studio-preview-empty = Het voorbeeld verschijnt hier.
studio-preview-mirrors = spiegelt programma
studio-preview-transition-select = Overgang
studio-preview-duration = Overgangsduur (ms)
studio-preview-commit-title = Voorbeeld → Programma vastleggen via de overgang (het publiek ziet dit)
studio-preview-transitioning = Bezig met overgang…
studio-preview-transition-button = Overgang ⇄
studio-preview-luma-placeholder = grijswaarde-wipe-afbeelding (png/jpg)
studio-preview-luma-label = Luma-wipe-afbeelding
studio-preview-browse = Bladeren…
studio-preview-filter-images = Afbeeldingen
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = stingervideo (ProRes 4444 .mov behoudt zijn alpha)
studio-preview-stinger-label = Stingervideobestand
studio-preview-stinger-cut-label = Stinger-snijpunt (ms)
studio-preview-stinger-cut-title = Wanneer de scènewissel plaatsvindt onder de stinger (ms in de overgang)
studio-preview-stinger-matte-label = Track matte
studio-preview-stinger-matte-title = Hoe een track-matte-stinger transparantie inpakt: de vulling en zijn matte naast elkaar (horizontaal) of gestapeld (verticaal)
studio-preview-stinger-duck-label = Programma ducken
studio-preview-stinger-duck-title = Duck het programma-audio onder de eigen audio van de stinger terwijl deze speelt (0 = uit)
studio-preview-stinger-duck-unit = dB

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Snijden
transition-kind-fade = Vervagen
transition-kind-slide-left = Schuiven ←
transition-kind-slide-right = Schuiven →
transition-kind-slide-up = Schuiven ↑
transition-kind-slide-down = Schuiven ↓
transition-kind-swipe-left = Vegen ←
transition-kind-swipe-right = Vegen →
transition-kind-luma-linear = Luma-wipe (lineair)
transition-kind-luma-radial = Luma-wipe (radiaal)
transition-kind-luma-horizontal = Luma-wipe (horizontaal)
transition-kind-luma-diamond = Luma-wipe (ruit)
transition-kind-luma-clock = Luma-wipe (klok)
transition-kind-image = Afbeeldingswipe (aangepast)
transition-kind-stinger = Stinger (video)
transition-kind-move = Verplaatsen (morph)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Geen
stinger-matte-horizontal = Naast elkaar
stinger-matte-vertical = Gestapeld

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Aangepast (RTMP/RTMPS)
stream-service-srt = SRT (zelf gehost)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Over
about-tagline = Neem op en stream als een studio — geen accounts, geen cloud.
about-version = Versie
about-created-by = Gemaakt door
about-project-started = Project gestart
about-first-stable = Eerste stabiele release
about-first-stable-pending = Nog niet — 1.0.0 is in ontwikkeling
about-platform = Platform
about-local-first = Freally Capture draait volledig op je machine. Geen accounts, geen telemetrie, geen cloud — het enige dat je computer verlaat is de stream die je zelf besloot te versturen.
about-website = Website
about-issues = Een probleem melden
about-license = Licentie
about-eula = EULA
about-third-party = Kennisgevingen van derden
about-check-updates = Controleren op updates…

# --- unified settings modal (TASK-906) ---
settings-title = Instellingen
settings-language-section = Taal
settings-language = Interfacetaal
settings-language-system = Systeemstandaard
settings-language-note = Een taal die je hier kiest wordt onthouden. "Systeemstandaard" volgt je besturingssysteem. Niet-vertaalde tekst valt terug op Engels.
settings-appearance-section = Weergave
settings-theme = Thema
settings-theme-dark = Donker
settings-theme-light = Licht
settings-theme-custom = Aangepast
settings-accent = Accent
settings-general-section = Algemeen
settings-show-stats-dock = Toon het statistiekenpaneel
settings-open-about = Over…

# --- command palette (TASK-904) ---
palette-title = Opdrachtenpalet
palette-search = Zoek scènes, bronnen en acties
palette-placeholder = Zoek scènes, bronnen, acties…
palette-no-results = Niets komt overeen met “{ $query }”
palette-hint = ↑ ↓ om te verplaatsen · Enter om uit te voeren · Esc om te sluiten
palette-group-scenes = Scène
palette-group-sources = Bron
palette-group-actions = Actie
palette-transition = Overgang Voorbeeld → Programma
palette-save-replay = Replay opslaan
palette-add-marker = Zet een hoofdstukmarkering
palette-vertical-canvas = Verticaal (9:16) canvas…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Welkom bij Freally Capture
wizard-welcome = Twee snelle stappen: kijk wat je computer aankan en start dan een scène. Het kost ongeveer dertig seconden, en je kunt later alles nog aanpassen.
wizard-local-first = Niets hiervan verlaat je computer. Freally Capture heeft geen accounts, geen telemetrie en geen cloud.
wizard-start = Aan de slag
wizard-skip = Overslaan
wizard-hardware-title = Wat je computer aankan
wizard-probing = Je grafische kaart en processor controleren…
wizard-encoder = Encoder
wizard-canvas = Canvas
wizard-bitrate = Bitrate
wizard-probe-found = Gevonden: { $gpus } · { $cores } fysieke cores
wizard-no-gpu = geen aparte GPU
wizard-apply = Deze instellingen gebruiken
wizard-keep-current = Houden wat ik heb
wizard-template-title = Begin met een scène
wizard-template-screen = Mijn scherm vastleggen
wizard-template-screen-note = Voegt een Beeldschermopname van je hoofdmonitor toe. De meest gebruikelijke plek om te beginnen.
wizard-template-empty = Leeg beginnen
wizard-template-empty-note = Een lege scène. Voeg zelf bronnen toe met de +-knop.
wizard-done = Je bent er klaar voor.
wizard-done-hint = Druk op elk moment op Ctrl+K om scènes, bronnen en acties te zoeken. De instellingen vind je achter de ⚙-knop.
wizard-close = Beginnen met streamen

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Je grafische kaart kan video zelf encoderen, waardoor de processor vrij blijft voor de rest van de studio.
autoconfig-reason-software = Er is geen bruikbare hardware-encoder gevonden, dus de processor encodeert — dat werkt prima, het kost alleen meer CPU.
autoconfig-reason-quality-hardware = 1080p op 60 beelden per seconde, met een bitrate die elk groot platform accepteert.
autoconfig-reason-quality-software = 30 beelden per seconde, omdat software-encodering op 60 bij de meeste processors beelden laat vallen.
autoconfig-reason-quality-low-cores = Een lagere bitrate, omdat deze processor weinig cores heeft en de software-encodering met de compositor moet concurreren om die cores.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Opname gestart
announce-recording-paused = Opname gepauzeerd
announce-recording-stopped = Opname gestopt
announce-live-started = Je bent live
announce-live-ended = Stream beëindigd
announce-reconnecting = Verbinding verbroken, opnieuw verbinden
announce-stream-failed = Stream mislukt
announce-frames-dropped = { $count } frames verloren

# CAP-M01 — undo/redo edit history
palette-undo = Ongedaan maken
palette-redo = Opnieuw
palette-edit-history = Bewerkingsgeschiedenis…
history-title = Bewerkingsgeschiedenis
history-empty = Nog niets om ongedaan te maken.
history-current = Huidige staat
history-close = Sluiten
history-addScene = Scène toevoegen
history-renameScene = Scène hernoemen
history-removeScene = Scène verwijderen
history-reorderScene = Scènes herschikken
history-addSource = Bron toevoegen
history-removeSource = Bron verwijderen
history-reorderSource = Bronnen herschikken
history-renameSource = Bron hernoemen
history-transformSource = Bron verplaatsen
history-toggleVisibility = Zichtbaarheid wisselen
history-toggleLock = Vergrendeling wisselen
history-setBlendMode = Mengmodus wijzigen
history-editSourceProperties = Eigenschappen bewerken
history-applyLayout = Lay-out schikken
history-moveToSeat = Naar plek verplaatsen
history-groupSources = Bronnen groeperen
history-ungroupSources = Groepering opheffen
history-toggleGroupVisibility = Groep wisselen
history-setSceneAudio = Scène-audio
history-setVerticalCanvas = Verticaal canvas
history-addFilter = Filter toevoegen
history-removeFilter = Filter verwijderen
history-reorderFilter = Filters herschikken
history-editFilter = Filter bewerken
history-toggleFilter = Filter wisselen
history-setVolume = Volume aanpassen
history-toggleMute = Demping wisselen
history-setMonitor = Monitoring wijzigen
history-setTracks = Sporen wijzigen
history-setSyncOffset = A/V-sync aanpassen
history-setAudioHotkeys = Audio-sneltoetsen

# CAP-M04 — alignment aids
settings-alignment-section = Uitlijnhulpen
settings-smart-guides = Slimme hulplijnen (uitlijnen tijdens slepen)
settings-safe-areas = Veilige-zone-overlays
settings-rulers = Linialen
align-group = Uitlijnen op canvas
align-left = Links uitlijnen
align-hcenter = Horizontaal centreren
align-right = Rechts uitlijnen
align-top = Boven uitlijnen
align-vcenter = Verticaal centreren
align-bottom = Onder uitlijnen

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Selectie uitlijnen & verdelen
arrange-left = Linkerranden uitlijnen
arrange-hcenter = Horizontaal centreren
arrange-right = Rechterranden uitlijnen
arrange-top = Bovenranden uitlijnen
arrange-vcenter = Verticaal centreren
arrange-bottom = Onderranden uitlijnen
distribute-h = Horizontaal verdelen
distribute-v = Verticaal verdelen
guides-group = Hulplijnen
guides-add-v = Verticale hulplijn toevoegen
guides-add-h = Horizontale hulplijn toevoegen
guides-clear = Alle hulplijnen wissen
history-arrangeItems = Items schikken
history-editGuides = Hulplijnen bewerken

# CAP-M05 — edit transform + copy/paste
transform-title = Transformatie bewerken — { $name }
transform-anchor = Anker
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotatie
transform-crop = Bijsnijden
transform-crop-left = Links
transform-crop-top = Boven
transform-crop-right = Rechts
transform-crop-bottom = Onder
transform-no-size = Grootte en bijsnijden komen beschikbaar zodra de bron zijn afmetingen meldt.
transform-copy = Transformatie kopiëren
transform-paste = Transformatie plakken
transform-close = Sluiten
filters-copy = Filters kopiëren ({ $count })
filters-paste = Filters plakken ({ $count })
palette-edit-transform = Transformatie bewerken…
history-pasteFilters = Filters plakken

# CAP-M26 — keying workbench
workbench-title = Keying-werkbank — { $name }
workbench-mode-keyed = Gekeyd
workbench-mode-source = Bron
workbench-mode-matte = Matte
workbench-mode-split = Gesplitst
workbench-eyedropper = Pipet
workbench-eyedropper-hint = Klik op de bron om de sleutelkleur te bemonsteren.
workbench-loupe = Loep
workbench-split = Splitsing
workbench-preview-alt = Voorbeeld van keying-werkbank
workbench-tune = Afstellen
workbench-close = Sluiten

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Klik op een scène om ernaar te schakelen.
multiview-hint-stage = Klik op een scène om deze in preview te zetten.
palette-multiview = Multiview-monitor

# CAP-M07 — projectors
projector-title = Projector openen
projector-source = Bron
projector-target-program = Programma
projector-target-preview = Voorbeeld
projector-target-scene = Scène…
projector-target-source = Bron…
projector-target-multiview = Multiview
projector-which-scene = Welke scène
projector-which-source = Welke bron
projector-none = Niets om te tonen
projector-display = Scherm
projector-windowed = Zwevend venster (dit scherm)
projector-display-option = Scherm { $n } — { $w }×{ $h }
projector-primary = (primair)
projector-open = Openen
projector-cancel = Annuleren
projector-exit-hint = Druk op Esc om af te sluiten
palette-projector = Projector openen…

# CAP-M08 — still-frame grab
palette-still = Stilstaand beeld vastleggen…
still-saved-toast = Beeld opgeslagen: { $name }
still-failed-toast = Beeld vastleggen mislukt: { $error }
hotkeys-still = Beeld vastleggen

# CAP-M13 — source health dashboard
palette-source-health = Brongezondheid…
palette-av-sync = A/V-sync-kalibratie…
palette-hotkey-audit = Sneltoetsenkaart…
health-title = Brongezondheid
health-col-source = Bron
health-col-state = Status
health-col-resolution = Resolutie
health-col-fps = FPS
health-col-last-frame = Laatste frame
health-col-dropped = Verworpen
health-col-retries = Herstarts
health-col-actions = Acties
health-state-live = Live
health-state-waiting = Wachtend
health-state-error = Fout
health-state-inactive = Inactief
health-restart = Herstarten
health-properties = Eigenschappen
health-empty = Deze collectie heeft nog geen bronnen.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture afsluiten?
quit-body = Nu afsluiten voert het volgende veilig en in volgorde uit:
quit-consequence-stream = De livestream beëindigen en de verbinding met de dienst verbreken.
quit-consequence-recording = De opname stoppen en de bestanden afronden.
quit-consequence-replay = De replaybuffer afsluiten — niet-opgeslagen replaybeelden gaan verloren.
quit-confirm = Veilig afsluiten
quit-quitting = Bezig met afsluiten…
quit-cancel = Annuleren

# CAP-M11 — crash-safe recording salvage
salvage-title = Onderbroken opnamen herstellen?
salvage-body = De vorige sessie eindigde onverwacht terwijl deze opnamen nog werden geschreven. Herstellen maakt een afspeelbare kopie naast het origineel — het originele bestand wordt nooit gewijzigd.
salvage-repair = Herstellen
salvage-repairing = Bezig met herstellen…
salvage-done = Hersteld
salvage-repaired = Hersteld → { $name }
salvage-failed = Herstellen mislukt: { $error }
salvage-dismiss = Niet nu

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Encoderfout — overgeschakeld van { $from } naar { $to }. De stream is opnieuw verbonden en blijft actief.
fallback-toast-recording = Encoderfout — overgeschakeld van { $from } naar { $to }. De opname gaat verder in een nieuw bestand.
fallback-note = Encoder-terugval: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Programmageluid is stil geworden
alarm-clipping = Programmageluid clipt
alarm-black = Programmabeeld is zwart
alarm-frozen = Programmabeeld is al even niet veranderd
alarm-lowDisk = Schijfruimte: nog ongeveer { $minutes } min bij de huidige bitrate
alarm-dismiss = Alarm sluiten
alarm-cleared = Opgelost: { $alarm }

# CAP-M22 — panic button
palette-panic = Paniek — naar privacyscherm snijden
panic-banner-title = Paniek
panic-banner-body = Het programma toont het privacyscherm; alle audio is gedempt en captures zijn gestopt. Stream en opname blijven actief.
panic-restore = Herstellen…
panic-restore-confirm = Programma herstellen?
panic-restore-yes = Herstellen
panic-restore-cancel = Annuleren
hotkeys-panic = Paniek (privacyscherm)
hotkeys-timer-toggle = Alle timers starten/pauzeren
hotkeys-timer-reset = Alle timers resetten
panic-slate-color = Kleur privacyscherm
panic-slate-image = Afbeelding privacyscherm
panic-slate-image-placeholder = Optioneel afbeeldingspad

# CAP-M24 — redacted diagnostics bundle
diag-title = Diagnosebundel
diag-intro = Exporteert een geredigeerde .zip (configuratiesnapshot, encoderprobe, recente statistieken — geheimen, paden en namen zitten er nooit in) om handmatig aan een GitHub-issue te hangen. Er wordt niets verzonden.
diag-preview = Inhoud bekijken
diag-hide-preview = Voorbeeld verbergen
diag-export = .zip exporteren
diag-exported = Geëxporteerd: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Pre-flight vóór live
preflight-intro = Elk blokkerend punt moet groen zijn; de rest zijn eerlijke hints.
preflight-item-targets = Streamdoelen ingesteld (sleutel/URL)
preflight-item-encoder = Bruikbare encoder beschikbaar
preflight-item-sources = Alle bronnen gezond
preflight-item-disk = Schijfruimte voor de opname
preflight-item-mic = Microfoonmeting
preflight-item-desktopAudio = Desktopaudio-meting
preflight-item-replay = Replaybuffer actief
preflight-targets-detail = { $count } ingeschakeld
preflight-sources-detail = { $count } bron(nen) met fout
preflight-disk-detail = ~{ $minutes } min bij huidige bitrate
preflight-fix-stream = Streaminstellingen…
preflight-fix-components = Componenten…
preflight-fix-sources = Brongezondheid…
preflight-fix-replay = Activeren
preflight-optional = optioneel
preflight-hold = Go Live blokkeren tot alles groen is
preflight-cancel = Annuleren
preflight-go-anyway = Toch live gaan
preflight-go-live = Ga live


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Achtergrond
scenes-backdrop-aria = Achtergrond van { $name }
backdrop-title = Achtergrond — { $name }
backdrop-hint = Een achtergrond vastgezet achter alles in deze scène — een afbeelding, een geanimeerde GIF of een video in een lus. Je opname ligt er altijd bovenop; scrol op het canvas om te zoomen.
backdrop-choose = Kies afbeelding of video…
backdrop-remove = Achtergrond verwijderen
backdrop-none = Geen achtergrond ingesteld.
backdrop-position = Positie
backdrop-split-full = Volledig canvas
backdrop-split-left = Linkerhelft
backdrop-split-right = Rechterhelft
backdrop-split-top = Bovenste helft
backdrop-split-bottom = Onderste helft
backdrop-sync = Afspelen starten zodra de opname start
backdrop-sync-hint = Blijft op het eerste beeld staan tot je opneemt; elke take start de video vanaf het begin.
backdrop-preview-play = Voorbeeld afspelen
backdrop-preview-pause = Voorbeeld pauzeren
backdrop-filter-all = Achtergronden (afbeeldingen en video)
backdrop-filter-images = Afbeeldingen
backdrop-filter-media = Video en GIF
sources-backdrop-badge = Achtergrond (onderaan vastgezet)
sources-backdrop-pinned = De achtergrond blijft onderaan vastgezet
filters-name-flip = Spiegelen
filters-flip-horizontal = Horizontaal
filters-flip-vertical = Verticaal
history-setSceneBackdrop = Achtergrond instellen
history-setBackdropSplit = Achtergrond verplaatsen
history-setBackdropSync = Achtergrond-opnamesynchronisatie
backdrop-scrub = Afspeelpositie
backdrop-loop = Lus
backdrop-reverse = Achterstevoren afspelen
backdrop-reverse-hint = Omkeren rendert eenmalig een omgekeerde kopie (video's vereisen de ffmpeg-component; GIF's keren direct om) — de eerste keer kan bij lange bestanden even duren.
filters-scaling = Schaling
filters-scaling-hint = Pixel-perfecte modi voor retro-/pixelcontent; Geheel klikt de getekende grootte bovendien vast op hele veelvouden (de grepen tonen de logische grootte).
filters-scaling-auto = Vloeiend
filters-scaling-nearest = Dichtstbijzijnde buur
filters-scaling-integer = Geheel (hele ×)
filters-scaling-sharp = Scherp bilineair
history-setScaling = Schaling wijzigen
hotkeys-zoom-100 = Zoom: herstellen (100%)
hotkeys-zoom-150 = Zoom: inzoomen 150%
hotkeys-zoom-200 = Zoom: inzoomen 2×
sources-follow-title = Cursor volgen tijdens zoomen (Windows; scrol op het canvas om te zoomen)
sources-follow-item = Cursor volgen wisselen voor { $name }
filters-autocrop = ✂ Zwarte balken auto-bijsnijden
filters-autocrop-title = Scant het volgende beeld op letterbox-/pillarbox-balken en snijdt ze bij (ongedaan te maken). Donkere scènes worden nooit bijgesneden.
filters-autocrop-follow = Opnieuw controleren bij resolutiewijziging
history-autoCrop = Zwarte balken automatisch bijgesneden
sources-link-audio = Ook het geluid van deze app opnemen (gekoppeld: verbergen dempt, venster verwijderen verwijdert het)
history-addLinkedWindow = Venster + gekoppeld geluid toevoegen
sources-hdr-title = Dit scherm is HDR — open de tone-mapping (het canvas blijft SDR)
sources-hdr-item = HDR-tone-mapping voor { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Dit scherm levert HDR. Zonder tone-mapping clippen hoge lichten en oogt de opname flets op het SDR-canvas. Wijzigingen gelden vanaf het volgende beeld.
sources-hdr-enable-suggested = Voorstel inschakelen (maxRGB, 200 nits)
sources-hdr-operator = Operator
sources-hdr-op-clip = Clippen (uit)
sources-hdr-op-maxrgb = maxRGB (tintbehoudend)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408-knik (SDR exact)
sources-hdr-paper-white = Papierwit
sources-hdr-nits = nits
projector-target-passthrough = Doorgeefmonitor (lage latentie)
projector-which-device = Apparaat
projector-passthrough-none = Voeg eerst een scherm, venster of opnameapparaat toe.
projector-passthrough-about = Ruwe apparaatbeelden — geen scènes, geen filters, geen compositor. Toont een gemeten latentie; audio blijft meeluisteren via het mixerkanaal.
projector-passthrough-hint = Doorgifte — Esc sluit
projector-latency = { $ms } ms
projector-latency-measuring = meten…
automation-title = Automatisering — regels, macro's en variabelen
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Regels
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Aan
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Wanneer
automation-then-run = voer dan uit
automation-no-macro = (no macro)
automation-macros = Macro's
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Uitvoeren
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Studio-variabelen
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
rundown-title = Draaiboek van de show
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Start
rundown-next = Volgende ▸
rundown-stop = Stop
rundown-idle = Niet actief
rundown-next-up = Hierna: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Stap
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
automation-layer = Laag
automation-layer-hint = Vuurt alleen terwijl deze laag actief is (leeg = alle lagen). Lagen blijven staan: een laagtoets schakelt en blijft (de OS-API biedt geen vasthoud-laag).
automation-chord-hint = Een enkele toets (Ctrl+Shift+M) of een tweeslag-akkoord (Ctrl+K, 3). De tweede toets wordt alleen geclaimd zolang het akkoord loopt.
panel-title = LAN-paneel & tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Paneel serveren
panel-port = Poort
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Wachtwoord
panel-show = Tonen
panel-hide = Verbergen
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Opslaan
osc-title = OSC-bedieningsoppervlak
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Naar OSC luisteren
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ-camera's
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Camera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Adres
ptz-port = Poort
ptz-speed = Snelheid
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
ptz-recall = Oproepen
ptz-store = Opslaan
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI-bedieningsoppervlak
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Ingang
midi-output = Uitgang (feedback)
midi-none = (none)
midi-learn = Leren
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Actie
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
panel-lan-warning = ⚠ LAN-verkeer is niet versleuteld — het wachtwoord staat in de URL via HTTP. Gebruik dit alleen op een vertrouwd netwerk.
osc-lan-warning = ⚠ OSC heeft geen wachtwoord — elk apparaat op je netwerk kan deze opdrachten sturen. Gebruik LAN alleen op een vertrouwd netwerk.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stats
sources-add-system-stats = Prestatiestatistieken (HUD)
sources-stats-title = Een prestatie-HUD toevoegen
sources-stats-note = Toont de echte gemeten cijfers van de studio in het programma voor je kijkers — fps, CPU, geheugen, rendertijd, verloren frames en live bitrate. Welke regels zichtbaar zijn, plus grootte en kleur, staan in de Eigenschappen van de bron. GPU-gebruik wordt niet getoond omdat het niet wordt gemeten.
sources-stats-add = Stats-HUD toevoegen
properties-stats-show-fps = FPS tonen
properties-stats-show-cpu = CPU tonen
properties-stats-show-memory = Geheugen tonen
properties-stats-show-render = Rendertijd tonen
properties-stats-show-dropped = Verloren frames tonen
properties-stats-show-bitrate = Bitrate tonen
properties-stats-size = Grootte (px)
properties-stats-note = De HUD tekent compacte universele labels (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) rechtstreeks in het programma; zonder stream toont de bitrate-regel “—”.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Visualizer
sources-add-visualizer = Audiovisualizer
sources-visualizer-title = Een audiovisualizer toevoegen
sources-visualizer-style-label = Stijl
sources-visualizer-style-bars = Spectrumbalken
sources-visualizer-style-scope = Oscilloscoop
sources-visualizer-style-vu = VU-meters
sources-visualizer-target-label = Luistert naar
sources-visualizer-target-master = Master-mix
sources-visualizer-target-track = Track { $n }
sources-visualizer-note = Tekent het signaal dat echt wordt gemixt (post-fader) — een gedempte bron blijft vlak, precies zoals hij klinkt. Grootte, kleur, aantal balken en valtempo staan in de Eigenschappen van de bron.
sources-visualizer-add = Visualizer toevoegen
properties-vis-bands = Balken
properties-vis-decay = Valtempo (dB/s)
properties-vis-peak-hold = Peak-hold-markeringen
properties-vis-missing-source = (bron ontbreekt)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splits
sources-add-split-timer = Speedrun-splittimer
sources-splits-title = Een splittimer toevoegen
sources-splits-file-label = LiveSplit-.lss-bestand
sources-splits-comparison-label = Vergelijken met
sources-splits-comparison-pb = Persoonlijk record
sources-splits-comparison-best = Beste segmenten
sources-splits-comparison-average = Gemiddelde
sources-splits-note = Het bestand wordt alleen-lezen geïmporteerd — er wordt nooit iets teruggeschreven. Wijs de globale Split- / Undo- / Skip- / Reset-toetsen toe in Instellingen → Sneltoetsen. Auto-splitters via procesgeheugen worden bewust niet ondersteund.
sources-splits-add = Splittimer toevoegen
properties-splits-size = Grootte (px)
properties-splits-ahead = Voor
properties-splits-behind = Achter
properties-splits-gold = Goud
properties-splits-split = Split
properties-splits-undo = Ongedaan maken
properties-splits-skip = Overslaan
properties-splits-reset = Reset
properties-splits-note = De knoppen besturen de lopende timer (de globale sneltoetsen doen hetzelfde vanuit elke app). De run wordt nooit naar het .lss-bestand geschreven.
hotkeys-split-split = Splittimer: start / split
hotkeys-split-undo = Splittimer: split ongedaan maken
hotkeys-split-skip = Splittimer: segment overslaan
hotkeys-split-reset = Splittimer: reset
hotkey-audit-action-split-split = Split (splittimer)
hotkey-audit-action-split-undo = Split ongedaan maken
hotkey-audit-action-split-skip = Segment overslaan
hotkey-audit-action-split-reset = Splittimer resetten
hotkey-audit-feature-split-timer = Splittimer

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlist
sources-add-playlist = Media-playlist (naadloos)
sources-playlist-title = Een media-playlist toevoegen
sources-playlist-files-label = Bestanden (één per regel, van boven naar beneden gespeeld)
sources-playlist-browse = Bladeren…
sources-playlist-loop = Herhalen
sources-playlist-shuffle = Willekeurig (één trekking per start; herhaald blijft die volgorde)
sources-playlist-hold-last = Laatste frame vasthouden aan het einde
sources-playlist-note = Speelt de hele bijgeknipte lijst naadloos via de gelabelde ffmpeg-component (alleen wire-formaten — .frec en stilstaande beelden via Media/Diavoorstelling). Items zijn allemaal video of allemaal audio, nooit gemengd. Trims, cuepunten en de 'now playing'-variabele staan in Eigenschappen.
sources-playlist-add = Playlist toevoegen
properties-playlist-items = Items (van boven naar beneden)
properties-playlist-up = Omhoog
properties-playlist-down = Omlaag
properties-playlist-remove = Item verwijderen
properties-playlist-in = Vanaf (s)
properties-playlist-out = Tot (s)
properties-playlist-cues = Cues (s, kommagescheiden)
properties-playlist-add-item = + Item toevoegen
properties-playlist-loop = Herhalen
properties-playlist-shuffle = Willekeurig
properties-playlist-hold-last = Laatste frame vasthouden
properties-playlist-hw = Hardwaredecodering
properties-playlist-variable = 'Now playing'-variabele (leeg = uit)
properties-playlist-previous = ⏮ Vorige
properties-playlist-next = ⏭ Volgende
properties-playlist-note = De cue-knoppen en Volgende/Vorige sturen de LOPENDE playlist; itemwijzigingen gelden bij Toepassen (de playlist herstart). Zet {"{{"}yourVariable{"}}"} in een Tekstbron om het spelende item te tonen.
hotkeys-playlist-next = Playlist: volgend item
hotkeys-playlist-previous = Playlist: vorig item
hotkey-audit-action-playlist-next = Playlist volgende
hotkey-audit-action-playlist-previous = Playlist vorige
hotkey-audit-feature-playlist = Playlist

# Instant replay source (CAP-N10)
sources-badge-replay = Replay
sources-add-replay = Instant replay
sources-replay-title = Een instant replay toevoegen
sources-replay-seconds-label = Roll-lengte (seconden)
sources-replay-speed-label = Snelheid
sources-replay-speed-full = 100% (met audio)
sources-replay-speed-half = 50% slow motion (stil)
sources-replay-speed-quarter = 25% slow motion (stil)
sources-replay-note = Blijft transparant tot je rolt. Activeer de replaybuffer (Bediening) en wijs de Roll-toets toe — een roll knipt de laatste momenten van de buffer, speelt ze in het programma en wordt daarna weer transparant.
sources-replay-add = Instant replay toevoegen
properties-replay-roll = ⏵ Replay rollen
properties-replay-note = Roll knipt de ACTIEVE replaybuffer tot een clip en speelt die op de gekozen snelheid — hertimed, nooit geïnterpoleerd. Slow motion is bewust stil. Scrubben en pauzeren werken tijdens het afspelen; aan het einde wordt de bron weer transparant.
hotkeys-replay-roll = Instant replay: rollen
hotkey-audit-action-replay-roll = Instant replay rollen

# Input overlay source (CAP-N13)
sources-badge-input = Invoer
sources-add-input-overlay = Invoer-overlay (toetsen/pad)
sources-input-title = Invoer-overlay toevoegen
sources-input-layout-label = Indeling
sources-input-layout-wasd = WASD + muis
sources-input-layout-keyboard = Compact toetsenbord + muis
sources-input-layout-gamepad = Gamepad (twee sticks)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Toetsen
sources-input-accent-label = Ingedrukt
sources-input-privacy-note = Privacy: invoer wordt alleen gelezen zolang deze bron live in een scène staat, en alleen de vaste toetsen van de indeling worden gepolst — een momentopname „is hij nu ingedrukt?”, nooit een hook. Er wordt niets gelogd, opgeslagen of ergens heen gestuurd; getypte tekst wordt nooit vastgelegd.
sources-input-os-note = Toetsenbord- en muisstatus wordt vandaag alleen op Windows gelezen — andere systemen tekenen de toetsen niet-ingedrukt (eerlijk gezegd, nooit nagebootst). Gamepads werken overal via de gilrs-bibliotheek; de eerste aangesloten controller wordt getekend, en zonder controller blijft de indeling niet-ingedrukt.
sources-input-add = Invoer-overlay toevoegen

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Cursoreffecten
filters-cursorfx-hint = Op Windows (dat de cursor zelf tekent) worden ze rechtstreeks in de capture getekend en verschijnen ze dus in opnames en streams. macOS en Linux stellen de cursor op OS-niveau samen, dus deze effecten zijn alleen voor Windows. Wijzigingen gelden direct.
filters-cursorfx-halo = Cursorhalo
filters-cursorfx-halo-color = Kleur
filters-cursorfx-halo-radius = Straal (px)
filters-cursorfx-ripples = Klikgolven
filters-cursorfx-left-color = Linksklik
filters-cursorfx-right-color = Rechtsklik
filters-cursorfx-keystrokes = Toetsweergave
filters-cursorfx-keystrokes-hint = Toont een vaste set toetsen (letters, cijfers, modificatietoetsen, pijlen) naast de cursor zolang ze ingedrukt zijn. Toetsen worden alleen gelezen terwijl dit aanstaat, rechtstreeks in het beeld getekend en nooit opgeslagen of gelogd.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Titel
sources-add-title = Titel / Scorebord
sources-title-title = Titel toevoegen
sources-title-template-label = Begin met
sources-title-template-lower-third = Lower third (balk + naam + ondertitel)
sources-title-template-scoreboard = Scorebord (plaat + 4 cellen)
sources-title-template-blank = Leeg canvas
sources-title-width-label = Canvasbreedte
sources-title-height-label = Canvashoogte
sources-title-template-name = Naam
sources-title-template-subtitle = Titel
sources-title-template-home = THUIS
sources-title-template-away = UIT
sources-title-note = Titels in lagen (tekst / afbeelding / vak) met een in-/uit-animatie, lokaal samengesteld — geen browserbron. Lagen, bestandskoppelingen en {"{{"}variabelen{"}}"} en de live-bediening staan in de Eigenschappen van de bron.
sources-title-add = Titel toevoegen
properties-title-layers = Lagen (op volgorde getekend — latere rijen liggen bovenop)
properties-title-kind-text = Tekst
properties-title-kind-image = Afbeelding
properties-title-kind-rect = Vak
properties-title-x = X
properties-title-y = Y
properties-title-outline = Omlijning (px)
properties-title-outline-color = Omlijning
properties-title-shadow = Schaduw
properties-title-animation = In-/uit-animatie
properties-title-anim-none = Geen (harde overgang)
properties-title-anim-fade = Vervagen
properties-title-anim-slide-left = Naar links schuiven
properties-title-anim-slide-up = Omhoog schuiven
properties-title-anim-wipe = Veeg
properties-title-duration = Duur (ms)
properties-title-fire-in = ▶ Inkomen starten
properties-title-fire-out = ◼ Uitgaan starten
properties-title-set-live = Live zetten
properties-title-set-live-note = Duwt deze tekst nu in de LIVE titel — zonder Toepassen, zonder herstart
properties-title-up = Laag omhoog
properties-title-down = Laag omlaag
properties-title-remove = Laag verwijderen
properties-title-add-text = + Tekst
properties-title-add-image = + Afbeelding
properties-title-add-rect = + Vak
properties-title-note = In/uit starten en „Live zetten" sturen de LOPENDE titel; laagwijzigingen gelden bij Toepassen (de titel herstart en komt opnieuw in). Tekstcellen kunnen aan een bewaakt bestand koppelen (CSV-cel / JSON-waarde / heel bestand) en {"{{"}variabelen{"}}"} interpoleren — „Live zetten" wint van beide.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN-ingest (SRT/RTMP-listener)
sources-lan-title = Een LAN-ingest-listener toevoegen
sources-lan-protocol-label = Protocol
sources-lan-protocol-srt = SRT (versleutelbaar — aanbevolen)
sources-lan-protocol-rtmp = RTMP (geen authenticatie)
sources-lan-port-label = Poort (1024–65535)
sources-lan-passphrase-label = Wachtwoordzin (leeg = open)
sources-lan-passphrase-hint = SRT-wachtwoordzinnen zijn 10–79 tekens; de zender moet dezelfde gebruiken.
sources-lan-open-warning = Geen wachtwoordzin: iedereen op dit netwerk kan deze bron onversleuteld voeden. Stel er een in tenzij het netwerk alleen van jou is.
sources-lan-rtmp-warning = RTMP heeft geen authenticatie — iedereen op dit netwerk kan naar deze poort zenden. Gebruik liever SRT met een wachtwoordzin.
sources-lan-url-label = Richt de app van de zender op
sources-lan-qr-aria = QR-code van de ingest-URL
sources-lan-note = Alleen LAN: luistert op het lokale adres van deze machine, alleen zolang de bron bestaat, en raakt nooit het internet — niets verlaat de machine totdat een zender op jouw netwerk eerst zendt. Decodering loopt via de duidelijk gelabelde ffmpeg-component. Het canvas toont deze URL totdat een zender verbindt.
sources-lan-add = Beginnen met luisteren
properties-lan-note = Het toepassen van een protocol-, poort- of wachtwoordzinwijziging herstart de listener — de zender moet opnieuw verbinden. De stream wordt in een canvas van 1920×1080 gepast.

# Freally Link source & output (CAP-N12)
sources-badge-link = Link
sources-add-freally-link = Freally Link (andere instantie)
sources-link-title = Een Freally Link toevoegen
sources-link-about = Ontvangt het programma van een andere Freally Capture-instantie — video en masteraudio — via je eigen netwerk. Zet eerst “Freally Link-uitvoer” aan op de zendende instantie. v1 streamt motion-JPEG over TCP: prima op bekabeld LAN of goede wifi, eerlijk over bandbreedte op zwakke verbindingen.
sources-link-scan = LAN scannen
sources-link-scanning = Scannen…
sources-link-none = Geen Freally Link-uitvoer gevonden. Zet “Freally Link-uitvoer” aan op de andere instantie (Bediening → LAN-paneel) of typ hieronder het adres.
sources-link-host = Adres
sources-link-port = Poort
sources-link-key = Koppelsleutel
sources-link-key-hint = De sleutel uit de instellingen "Freally Link-uitvoer" van de zender — zonder levert de zender geen enkel frame.
sources-link-add = Link toevoegen
properties-link-note = Zonder verbinding toont de bron een “verbinden”-beeld en probeert het zelf opnieuw met oplopende wachttijd — hij blijft nooit hangen op een oud frame. Eén ontvanger per zender; een bezette zender wordt beleefd opnieuw geprobeerd.
link-title = Freally Link-uitvoer
link-about = Deel het programma van deze instantie — video en masteraudio — met ÉÉN andere Freally Capture op je eigen netwerk; daar verschijnt het als een “Freally Link”-bron (streamen met twee pc's, extra monitoren). Standaard uit; niets kondigt aan of luistert totdat je het inschakelt. v1 streamt motion-JPEG + ongecomprimeerde audio over TCP — gemaakt voor bekabeld LAN of goede wifi, nooit voor internet.
link-enable = Deel het programma op mijn netwerk
link-name = Instantienaam
link-key = Koppelsleutel
link-key-hint = Minstens 8 tekens — ontvangers moeten deze sleutel invoeren voordat er ook maar één frame wordt geleverd.
link-lan-warning = ⚠ Ontvangers moeten de koppelsleutel tonen voordat er iets wordt geleverd, maar de stream zelf is in v1 niet versleuteld — gebruik dit alleen op een vertrouwd netwerk.
link-serving = Ontvangers vinden deze instantie met “LAN scannen” of voegen haar handmatig toe op:
link-off-hint = Schakel delen in om de poort te openen en deze instantie aan LAN-scans te melden.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Applicatiemenu
menu-file = Bestand
menu-edit = Bewerken
menu-view = Beeld
menu-docks = Docks
menu-profile = Profiel
menu-collection = Scèneverzameling
menu-tools = Extra
menu-help = Help
menu-rename = Hernoemen
menu-remove = Verwijderen
menu-import = Importeren
menu-export = Exporteren
menu-file-show-recordings = Opnamen tonen
menu-file-remux = Hermuxen naar MP4…
menu-file-settings = Instellingen…
menu-file-show-settings-folder = Instellingenmap tonen
menu-file-exit = Afsluiten
menu-edit-undo = Ongedaan maken
menu-edit-redo = Opnieuw
menu-edit-history = Bewerkingsgeschiedenis…
menu-edit-copy-transform = Transformatie kopiëren
menu-edit-paste-transform = Transformatie plakken
menu-edit-copy-filters = Filters kopiëren
menu-edit-paste-filters = Filters plakken
menu-edit-transform = Transformatie…
menu-edit-lock-preview = Voorbeeld vergrendelen
menu-view-fullscreen = Interface op volledig scherm
menu-stats-dock = Statistiekenpaneel
menu-view-multiview = Multiview-monitor…
menu-view-projectors = Projectoren…
menu-view-source-health = Brongezondheid…
menu-view-still = Stilstaand beeld vastleggen
menu-docks-browser = Browserdocks…
menu-docks-lock = Docks vergrendelen
menu-docks-reset = Dock-indeling herstellen
menu-profile-manage = Profielen beheren…
menu-collection-manage = Scèneverzamelingen beheren…
menu-collection-import-obs = Importeren uit OBS…
menu-collection-missing = Controleren op ontbrekende bestanden…
menu-tools-wizard = Installatiewizard uitvoeren
menu-tools-wizard-title = De installatiewizard draait bij de eerste start; opnieuw uitvoeren kan nog niet.
menu-tools-automation = Automatiseringsregels & macro's…
menu-tools-rundown = Draaiboek tonen…
menu-tools-hotkeys = Sneltoetsenkaart…
menu-tools-av-sync = A/V-sync-kalibratie…
menu-tools-scripts = Lua-scripts…
menu-tools-components = Componenten…
menu-tools-midi = MIDI-bediening…
menu-tools-ptz = PTZ-camera's…
menu-tools-remote = API voor afstandsbediening…
menu-tools-panel = LAN-paneel & tally…
menu-help-portal = Helpportaal
menu-help-website = Website bezoeken
menu-help-discord = Word lid van de Discord-server
menu-help-bug = Meld een bug…
menu-help-updates = Controleren op updates…
menu-help-whats-new = Wat is er nieuw
menu-help-about = Over…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Instellingscategorieën
settings-cat-general = Algemeen
settings-cat-appearance = Weergave
settings-cat-streaming = Streaming
settings-cat-output = Uitvoer
settings-cat-replay = Replay
settings-cat-hotkeys = Sneltoetsen
settings-cat-network = Netwerk
settings-cat-accessibility = Toegankelijkheid
settings-cat-about = Over
settings-ok = OK
settings-cancel = Annuleren
settings-apply = Toepassen
settings-save = Opslaan
settings-loading = Instellingen laden…
settings-hotkeys-filter = Sneltoetsen filteren
settings-hotkeys-filter-placeholder = Typ om acties of toetsen te filteren…
settings-hotkeys-no-match = Geen sneltoets komt overeen met “{ $query }”.
settings-hotkey-none = Geen
settings-hotkey-group-ctrl = Ctrl + toets
settings-hotkey-group-ctrl-shift = Ctrl + Shift + toets
settings-hotkey-group-ctrl-alt = Ctrl + Alt + toets
settings-hotkey-group-function = Functietoetsen
settings-hotkey-group-numpad = Numeriek toetsenblok
settings-panic-section = Privacyscherm
settings-meter-section = Niveaumeters van de mixer
settings-meter-note = De kleuren die de niveaumeters van de audiomixer doorlopen, van stil tot oversturing. De kleurenblind-veilige voorinstelling gebruikt een blauw → oranje verloop dat leesbaar blijft bij rood-groen kleurenblindheid.
settings-meter-preset = Meterkleuren
settings-meter-preset-default = Groen / geel / rood
settings-meter-preset-colorblind = Kleurenblind-veilig (blauw / oranje)
settings-meter-preset-custom = Aangepast
settings-meter-low = Normaal
settings-meter-mid = Luid
settings-meter-high = Oversturing
settings-meter-preview = Voorbeeld

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Wat is er nieuw
whats-new-loading = Release-opmerkingen laden…
whats-new-version = Nieuw in versie { $version }
whats-new-empty = Geen release-opmerkingen voor deze versie.
filters-name-directional-blur = Directionele vervaging
filters-name-radial-blur = Radiale vervaging
filters-name-zoom-blur = Zoomvervaging
filters-name-pixelate = Pixeleren
filters-angle = Hoek (°)
filters-center-x = Centrum X
filters-center-y = Centrum Y
filters-block-size = Blokgrootte (px)
filters-name-freeze = Bevriezen
filters-freeze-hint = Wanneer ingeschakeld houdt deze bron het laatste frame vast — programma, voorbeeld, opname en stream bevriezen allemaal samen. Schakel dit filter om te bevriezen of te ontdooien.
transform-3d = 3D-kanteling
transform-rotation-x = Kanteling X (°)
transform-rotation-y = Kanteling Y (°)
transform-perspective = Perspectief
transform-reveal = Tonen/verbergen
transform-reveal-ms = Infaden (ms)
sources-clone-title = Klonen (zelfde feed, eigen filters)
sources-clone-item = { $name } klonen
menu-tools-downstream = Downstream-keyers…
menu-tools-transition-rules = Overgangsregels…
dsk-title = Downstream-keyers
dsk-hint = Overlays die op de programma-uitvoer worden samengesteld — boven elke scène, en ze blijven staan wanneer je van scène wisselt (een logo, een LIVE-badge, een naambalk). Bovenaan de lijst wordt vooraan getekend.
dsk-empty = Nog geen keyers — voeg een bron toe om die over elke scène te leggen.
dsk-enable = Deze keyer inschakelen
dsk-move-up = Omhoog (naar voren)
dsk-move-down = Omlaag
dsk-remove = Keyer verwijderen
dsk-opacity = Dekking
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Schaal
dsk-add = + Keyer toevoegen
transition-rules-title = Overgangsregels
transition-rules-hint = Geef een scènepaar een eigen overgang. Wanneer je van de eerste scène naar de tweede overgaat, worden dit type en deze duur gebruikt in plaats van de standaard (een Stinger-/Afbeeldingsregel gebruikt nog steeds het bestand dat in de overgangsbediening is ingesteld).
transition-rules-empty = Nog geen regels — elk scènepaar gebruikt de standaardovergang.
transition-rules-from = Van
transition-rules-to = Naar
transition-rules-kind = Overgang
transition-rules-duration = Duur (ms)
transition-rules-add = Regel toevoegen
transition-rules-remove = Regel verwijderen
