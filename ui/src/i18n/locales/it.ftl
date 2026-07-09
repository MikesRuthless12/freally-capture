# Freally Capture — it
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Modalità Studio
toggle-on = attivo
toggle-off = disattivo
stats = Statistiche
core-ok = core OK
hide-stats-dock = Nascondi il pannello statistiche
show-stats-dock = Mostra il pannello statistiche


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Impossibile salvare le impostazioni — la modifica non sopravviverà a un riavvio.
studio-mode-leave = Esci dalla Modalità Studio
studio-mode-enter-title = Modalità Studio — modifica una scena in anteprima, poi mandala in onda con una transizione
vertical-canvas-title = Il secondo canvas di uscita (verticale 9:16) — registrabile e trasmettibile in modo indipendente
app-version = v{ $version }
core-error = core ERRORE
core-unreachable = core irraggiungibile (modalità browser)
connecting-to-core = connessione al core…
filters-source-fallback = Sorgente

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Anteprima programma
preview-program-output = Uscita programma
preview-canvas-editor = Editor del canvas
preview-px-to-edge-label = Pixel dai bordi del fotogramma
preview-px-to-edge = px dal bordo S { $left } · A { $top } · D { $right } · B { $bottom }
preview-program-heading = Programma
preview-no-gpu = Nessun adattatore GPU utilizzabile trovato — il compositor non può funzionare su questa macchina.
preview-starting-compositor = Avvio del compositor…
preview-empty-scene = Questa scena è vuota — aggiungi una sorgente in Sorgenti, poi trascinala, ridimensionala e ruotala direttamente qui sul canvas.
preview-fps = { $fps } fps
preview-dropped = { $dropped } persi

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Link d'invito ricevuto
remote-join-with-webcam = Entra con la webcam
remote-dismiss = Ignora
remote-hosting-guest = Stai ospitando un guest remoto
remote-you-are-guest = Sei un guest remoto
remote-share-view-title = Condividi il tuo schermo con l'app del guest (vedrà la tua vista in diretta)
remote-stop-sharing-view = Interrompi condivisione vista
remote-share-my-view = Condividi la mia vista
remote-allow-center-title = Consenti al guest di scegliere quale vista occupa il centro (mantieni il controllo e puoi riprenderlo in qualsiasi momento)
remote-guest-switching = Cambio guest:
remote-stop-screen = Interrompi schermo
remote-share-screen = Condividi schermo
remote-share-screen-title-guest = Condividi il tuo schermo con l'host (diventa una sorgente che può centrare)
remote-center-request-label = Richiesta vista centrale
remote-center = Centra
remote-center-cam-title = Chiedi all'host di centrare la tua camera
remote-center-my-cam = La mia cam
remote-center-screen-title = Chiedi all'host di centrare il tuo schermo condiviso
remote-center-my-screen = Il mio schermo
remote-center-host-title = Restituisci il centro alla vista dell'host
remote-center-host-view = Vista host
remote-end-session = Termina sessione
remote-leave = Esci
remote-host-view-heading = Vista host
remote-host-shared-view-label = La vista condivisa dell'host
remote-guest-position-label = Posizione del guest
remote-guest-label = Guest
remote-put-guest = Metti il guest { $position }
remote-remove-title = Rimuovi il guest — potrà rientrare con lo stesso link
remote-remove = Rimuovi
remote-ban-title = Banna il guest — lo blocca e invalida il link d'invito
remote-ban = Banna
remote-guest-self-muted = guest auto-silenziato
remote-unmute-guest = Riattiva audio guest
remote-mute-guest = Silenzia guest
remote-muted-by-host = Silenziato dall'host
remote-unmute-mic = Riattiva microfono
remote-mute-mic = Silenzia microfono
remote-waiting-for-host = in attesa dell'host


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = sorgente
sources-fallback-video = video
sources-fallback-error = errore
sources-kind-unknown = ?
sources-missing-source = (sorgente mancante)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Schermo
sources-badge-window = Finestra
sources-badge-portal = Portal
sources-badge-camera = Camera
sources-badge-image = Immagine
sources-badge-media = Media
sources-badge-guest = Guest
sources-badge-color = Colore
sources-badge-text = Testo
sources-badge-scene = Scena
sources-badge-slides = Slide
sources-badge-chat = Chat
sources-badge-audio-in = Audio In
sources-badge-audio-out = Audio Out
sources-badge-app-audio = Audio App

# Add-source menu items
sources-add-display = Cattura schermo
sources-add-window = Cattura finestra
sources-add-game = Cattura gioco (leggi prima)
sources-add-webcam = Dispositivo di cattura video
sources-add-image = Immagine
sources-add-media = Media (file video/immagine)
sources-add-remote-guest = Guest remoto (spike P2P)
sources-add-color = Colore
sources-add-text = Testo
sources-add-nested-scene = Scena annidata
sources-add-slideshow = Presentazione immagini
sources-add-chat-overlay = Overlay chat live
sources-add-audio-input = Cattura ingresso audio
sources-add-audio-output = Cattura uscita audio
sources-add-app-audio = Audio applicazione (Windows)
sources-add-existing = Sorgente esistente…

# Panel header + toolbar buttons
sources-panel-title = Sorgenti
sources-group-title = Raggruppa sorgenti — seleziona due o più elementi, poi Crea gruppo; gli elementi raggruppati si spostano e si mostrano/nascondono insieme
sources-group-aria = Raggruppa sorgenti
sources-arrange = Disponi: schermo + angoli
sources-add-source = Aggiungi una sorgente
sources-browser-source-note = La Sorgente Browser arriva come componente on-demand a sé (un motore Chromium da ~180 MB — mai incluso). Per ora: cattura una vera finestra del browser con Cattura finestra + una chiave chroma/colore, oppure apri chat/avvisi come Dock (Controlli → Dock).

# Empty state
sources-empty = Nessuna sorgente in questa scena — aggiungi una Cattura schermo, Finestra, Webcam, Immagine, Colore o Testo con "+". Trascinale, ridimensionale e ruotale sul canvas; i pulsanti a destra riordinano lo stack.

# Per-row controls
sources-already-in-group = Già in { $name }
sources-pick-for-new-group = Seleziona per il nuovo gruppo
sources-pick-item-for-group = Seleziona { $name } per il nuovo gruppo
sources-hide = Nascondi
sources-show = Mostra
sources-hide-item = Nascondi { $name }
sources-show-item = Mostra { $name }
sources-unfocus-title = Togli il focus — ripristina il layout
sources-focus-title = Focus — riempi il canvas (Evidenzia relatore)
sources-unfocus-item = Togli il focus da { $name }
sources-focus-item = Focus su { $name }
sources-center-title = Centra — rendila la vista centrale condivisa (le cam si spostano sulla barra)
sources-center-item = Centra { $name }
sources-rename-item = Rinomina { $name }
sources-in-group = Nel gruppo { $name }

# Row status + retry
sources-retry-error = Riprova — { $message }
sources-retry-item = Riprova { $name }
sources-status-error = stato: errore
sources-open-privacy-title = Apri le impostazioni di privacy di macOS per questa autorizzazione
sources-open-privacy-item = Apri le impostazioni di privacy per { $name }
sources-privacy-settings-button = impostazioni
sources-status-starting = avvio…
sources-status-live = in diretta
sources-status-aria = stato: { $state }

# Media row pause/resume
sources-media-resume-title = Riprendi il video (in diretta sullo stream)
sources-media-pause-title = Metti in pausa il video — blocca il fotogramma e togli l'audio, in diretta sullo stream
sources-media-resume-item = Riprendi { $name }
sources-media-pause-item = Metti in pausa { $name }

# Hover controls
sources-unlock = Sblocca
sources-lock = Blocca
sources-unlock-item = Sblocca { $name }
sources-lock-item = Blocca { $name }
sources-raise-title = Alza nello stack
sources-raise-item = Alza { $name }
sources-lower-title = Abbassa nello stack
sources-lower-item = Abbassa { $name }
sources-filters-title = Filtri e fusione
sources-filters-item = Filtri per { $name }
sources-properties-title = Proprietà
sources-properties-item = Proprietà di { $name }
sources-remove-title = Rimuovi da questa scena
sources-remove-item = Rimuovi { $name }

# Grouping footer
sources-create-group = Crea gruppo ({ $count })
sources-cancel = Annulla

# Groups list
sources-groups-aria = Gruppi di sorgenti
sources-hide-group = Nascondi il gruppo
sources-show-group = Mostra il gruppo
sources-item-count = · { $count } elementi
sources-ungroup-title = Separa — gli elementi restano dove sono
sources-ungroup-item = Separa { $name }

# Live Chat Overlay picker
sources-chat-title = Aggiungi un overlay chat live
sources-chat-youtube-label = YouTube — URL del canale, watch o live_chat (nessuna chiave, nessun accesso)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  o un URL watch?v=
sources-chat-twitch-label = Twitch — nome del canale (lettura anonima, nessun account)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — slug del canale (endpoint pubblico, best-effort)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = I messaggi appaiono con un timestamp scorrevole h:mm:ss AM/PM su sfondo trasparente (di default in alto a destra; trascinalo dove vuoi). Un flood in chat fa solo scorrere via le righe vecchie — non può mai bloccare lo stream o la registrazione. La chat di Facebook richiede un tuo token Graph e non è ancora implementata — non è mai necessaria e non blocca mai le piattaforme qui sopra.
sources-chat-add = Aggiungi overlay chat
sources-chat-default-name = Chat live

# Image Slideshow picker
sources-slideshow-title = Aggiungi una presentazione immagini
sources-slideshow-empty = Ancora nessuna immagine — Sfoglia le aggiunge in ordine.
sources-slideshow-remove-slide = Rimuovi la slide { $number }
sources-slideshow-browse = Sfoglia immagini…
sources-slideshow-per-slide-label = Per slide (ms)
sources-slideshow-crossfade-label = Dissolvenza incrociata (ms, 0 = taglio)
sources-slideshow-loop-label = Ripeti (off = mantieni l'ultima slide)
sources-slideshow-shuffle-label = Ordine casuale a ogni ciclo
sources-slideshow-note = La dissolvenza incrociata fonde immagini di ugual dimensione; dimensioni diverse fanno un taglio netto al confine (nessun ridimensionamento silenzioso).
sources-slideshow-add = Aggiungi presentazione ({ $count })

# Nested Scene picker
sources-nested-title = Aggiungi una scena annidata
sources-nested-empty = Nessun'altra scena da annidare — aggiungi prima una seconda scena.
sources-nested-scene-name = Scena: { $name }
sources-nested-note = La scena annidata viene renderizzata in diretta alla dimensione del canvas del programma e segue le proprie modifiche; trasformazioni, filtri e fusione si applicano come a qualsiasi sorgente. Le sue sorgenti audio entrano nel mix quando una scena che la mostra è il programma.

# Display / Window capture picker
sources-capture-display-title = Aggiungi una Cattura schermo
sources-capture-window-title = Aggiungi una Cattura finestra
sources-capture-looking = Ricerca sorgenti…
sources-capture-none-displays = Niente da catturare qui — nessuno schermo trovato.
sources-capture-none-windows = Niente da catturare qui — nessuna finestra trovata.
sources-capture-portal-note = Su Wayland, la finestra di sistema sceglie lo schermo o la finestra — lì le app non possono catturare globalmente, quindi questo è il percorso onesto (e unico).
sources-capture-window-note = Le anteprime si aggiornano in diretta. Una finestra minimizzata mostra il suo ultimo fotogramma (o nulla) finché non la ripristini.
sources-thumb-no-preview = nessuna anteprima
sources-thumb-loading = caricamento…

# Video Capture Device picker
sources-webcam-title = Aggiungi un dispositivo di cattura video
sources-webcam-looking = Ricerca camere…
sources-webcam-none = Nessuna camera o scheda di acquisizione trovata.
sources-webcam-format-label = Formato
sources-webcam-format-auto-loading = Auto (caricamento formati…)
sources-webcam-format-auto = Auto (risoluzione massima)
sources-webcam-card-presets-label = Preset della scheda:
sources-webcam-preset-title = Seleziona la modalità { $label } che questa scheda pubblicizza
sources-webcam-add = Aggiungi camera

# Audio Input / Output capture picker
sources-audio-output-title = Aggiungi una Cattura uscita audio
sources-audio-input-title = Aggiungi una Cattura ingresso audio
sources-audio-default-output = Uscita predefinita (ciò che senti)
sources-audio-default-input = Ingresso predefinito
sources-audio-looking = Ricerca dispositivi audio…
sources-audio-none-output = Nessun dispositivo di cattura audio desktop trovato qui.
sources-audio-none-input = Nessun microfono o ingresso di linea trovato.
sources-audio-input-note = Le strisce del mixer ottengono un misuratore VU, fader, muto, monitoraggio, filtri (denoise, gate, compressore…) e assegnazione traccia. Tutto resta su questa macchina.

# Application Audio picker
sources-appaudio-title = Aggiungi Audio applicazione
sources-appaudio-looking = Ricerca app che stanno emettendo suono…
sources-appaudio-none = Nessuna app sta emettendo suono in questo momento — avvia la riproduzione nell'app, poi aggiorna.
sources-appaudio-refresh = ⟳ Aggiorna
sources-appaudio-note = Cattura esattamente l'audio di quell'app — con VU, fader, muto, filtri e traccia propri.

# Game Capture picker
sources-game-title = Cattura gioco
sources-game-checking = Controllo…
sources-game-use-portal = Usa Cattura schermo (Portal)
sources-game-use-window = Usa invece Cattura finestra

# Image picker
sources-image-title = Aggiungi un'immagine
sources-image-file-label = File immagine (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Aggiungi immagine

# Path field
sources-browse = Sfoglia…

# Media picker
sources-media-title = Aggiungi media
sources-media-file-label = File media (mp4, mkv, webm, mov, .frec o un'immagine)
sources-media-loop-label = Ripeti (riparti dall'inizio alla fine)
sources-media-note = .frec viene riprodotto tramite il codec proprietario freally-video — niente da scaricare. I formati di rete (mp4/mkv/webm/…) vengono decodificati tramite il componente FFmpeg on-demand; il loro audio arriva nel mixer come striscia propria.
sources-media-add = Aggiungi media

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 ora
sources-ttl-1day = 1 giorno

# Remote Guest form
sources-remote-copy-failed = copia non riuscita — seleziona il link e copialo manualmente
sources-remote-join-failed = accesso non riuscito: { $error }
sources-remote-title = Guest remoto (spike P2P)
sources-remote-host-heading = Host — invita un guest
sources-remote-start-hosting = Inizia a ospitare
sources-remote-expires-label = Scade
sources-remote-invite-expiry-aria = Scadenza dell'invito
sources-remote-invite-link-aria = Link d'invito
sources-remote-copied = Copiato ✓
sources-remote-copy = Copia
sources-remote-share-note = Condividi questo link (Discord / messaggio / email). Contiene la tua sessione e scade come impostato. Il guest lo apre ed entra con la sua webcam.
sources-remote-qr-note = Scansiona da un telefono per entrare direttamente dal browser — camera + mic, nessuna installazione. Il link freally:// copiabile qui sopra si apre in Freally Capture su una macchina che lo possiede.
sources-remote-guest-heading = Guest — entra con un invito
sources-remote-paste-placeholder = incolla il link d'invito
sources-remote-invite-input-aria = Link d'invito o id della sessione
sources-remote-join = Entra con la webcam
sources-remote-session-note = I controlli della sessione live (muto, termina) restano sulla barra in alto nella finestra principale — puoi chiudere questa finestra di dialogo.
sources-remote-stop-session = Interrompi sessione

# Invite QR
sources-invite-qr-aria = Codice QR del link d'invito

# Remote device pickers
sources-devices-output-unavailable = instradamento uscita non disponibile — riproduzione sul dispositivo predefinito
sources-devices-mic-test-failed = test del microfono non riuscito: { $error }
sources-devices-heading = Dispositivi audio della sessione
sources-devices-microphone-label = Microfono
sources-devices-microphone-aria = Microfono della sessione
sources-devices-system-default = Predefinito di sistema
sources-devices-output-label = Uscita
sources-devices-output-aria = Uscita audio della sessione
sources-devices-stop-test = Interrompi test
sources-devices-test = Test — ascoltati
sources-devices-testing-note = parla nel microfono — stai ascoltando in diretta i dispositivi selezionati
sources-devices-idle-note = reindirizza il microfono all'uscita (le cuffie evitano il feedback)

# TURN relay section
sources-turn-save-failed = salvataggio non riuscito: { $error }
sources-turn-summary = Rete — relay TURN opzionale (avanzato)
sources-turn-note-1 = Le sessioni si connettono direttamente (P2P) — gratis, nessun relay necessario. Se ENTRAMBI i lati sono dietro NAT restrittivi il percorso diretto può fallire; in tal caso un relay TURN gestito da te trasporta il media. Saltare questo va bene — la maggior parte delle connessioni funziona con la sola connessione diretta.
sources-turn-note-2 = Opzione gratuita: Oracle Cloud "Always Free" esegue coturn senza costi (nota: Oracle chiede una carta di credito alla registrazione, ma la configurazione Always-Free resta gratuita). Passaggi: 1) crea la VM gratuita, 2) installa coturn, 3) apri UDP 3478, 4) imposta utente/password, 5) inserisci qui turn:ip-della-tua-vm:3478 + le credenziali. La tua credenziale resta nel file delle impostazioni locali e non viene mai registrata.
sources-turn-url-label = URL TURN
sources-turn-url-placeholder = turn:host:3478 (vuoto = solo diretto)
sources-turn-url-aria = URL TURN
sources-turn-username-label = Nome utente
sources-turn-username-aria = Nome utente TURN
sources-turn-credential-label = Credenziale
sources-turn-credential-aria = Credenziale TURN
sources-turn-note-3 = Il relay si attiva una volta impostati tutti e tre i campi (un server TURN richiede le credenziali) e si applica alla prossima sessione che avvii o a cui ti unisci. Verificalo con una chiamata di test solo-relay tra le tue due macchine.
sources-turn-settings-unavailable = impostazioni non disponibili (modalità browser)

# Color picker
sources-color-title = Aggiungi un colore
sources-color-label = Colore
sources-color-width-label = Larghezza
sources-color-height-label = Altezza
sources-color-add = Aggiungi colore

# Text picker
sources-text-title = Aggiungi testo
sources-text-label = Testo
sources-text-default = Testo
sources-text-color-label = Colore
sources-text-color-aria = Colore del testo
sources-text-size-label = Dimensione (px)
sources-text-note = Famiglia del carattere, allineamento, a capo e RTL si trovano nelle Proprietà della sorgente. Il carattere Noto Sans incluso (con Arabo/Ebraico) è quello predefinito — identico su ogni macchina.
sources-text-add = Aggiungi testo

# Existing source picker
sources-existing-title = Aggiungi una sorgente esistente
sources-existing-empty = Non esiste ancora nessuna sorgente — aggiungine una a una scena qualsiasi. Le sorgenti esistenti sono condivise: rinominarne o riconfigurarne una aggiorna ogni scena che la mostra.

# Screen + corners layout
sources-slot-off = Off
sources-slot-center = Centro (schermo)
sources-slot-top-left = In alto a sinistra
sources-slot-top-right = In alto a destra
sources-slot-bottom-left = In basso a sinistra
sources-slot-bottom-right = In basso a destra
sources-layout-title = Disponi: schermo + angoli
sources-layout-empty = Aggiungi prima una cattura schermo e una o più camere a questa scena, poi disponile qui.
sources-layout-note = Metti uno schermo al centro e fino a quattro camere negli angoli — il tuo layout da explainer / podcast. Ogni angolo contiene una webcam, una finestra di chiamata catturata o una clip media. Puoi poi trascinare ognuno di essi sul canvas.
sources-layout-slot-aria = Slot per { $name }
sources-layout-apply = Applica layout


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Controlli
controls-start-stop-title-stop = Interrompi e finalizza la registrazione
controls-start-stop-title-start = Registra il feed del programma con la configurazione Impostazioni → Uscita
controls-finalizing = ◌ Finalizzazione…
controls-stop-recording = ■ Interrompi registrazione
controls-start-recording = ● Avvia registrazione
controls-marker-title = Inserisci un marcatore di capitolo in questo momento — finisce nella REGISTRAZIONE (capitoli mkv, o un file sidecar). I marcatori di stream lato piattaforma richiedono account della piattaforma, che questa app non chiede mai.
controls-marker = ◈ Marcatore
controls-pause-title-resume = Riprendi — il file continua come un'unica timeline contigua
controls-pause-title-pause = Metti in pausa — nessun fotogramma viene scritto; riprendendo continua lo stesso file riproducibile
controls-resume-recording = ▶ Riprendi registrazione
controls-pause-recording = ⏸ Metti in pausa la registrazione
controls-reactions-label = Reazioni (incorporate nel programma)
controls-reactions-title = Fai fluttuare una reazione sul programma — registrata E trasmessa, così il replay mostra il momento esatto. Anche gli spettatori in chat le attivano (le loro emoji di reazione fluttuano automaticamente); un flood limita solo quanto appare sullo schermo.
controls-react = Reagisci { $emoji }
controls-virtual-camera-title = La camera virtuale richiede un componente driver firmato per ogni OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / estensione CoreMediaIO macOS / v4l2loopback Linux) — arriva come milestone a sé. Il modello del feed è pronto: programma, canvas verticale o una singola sorgente, con un mic virtuale abbinato su Windows/Linux (macOS non ha API per il mic virtuale — detto onestamente).
controls-virtual-camera = ⌁ Avvia camera virtuale
controls-files-title = Registrazioni completate + l'azione di remux in mp4
controls-files = ▤ File…
controls-output-title = Formato di registrazione, encoder, cartella, tracce e suddivisione
controls-output = ⚙ Uscita…
controls-stream-title = Destinazione Vai in diretta: servizio, chiave di stream, encoder, bitrate
controls-stream = ⦿ Stream…
controls-codecs-title = Il componente codec di rete ffmpeg on-demand (chiaramente etichettato, mai incluso)
controls-codecs = ⬡ Codec…
controls-replay-title = Durata del buffer di replay + preset di qualità
controls-replay = ⟲ Replay…
controls-keys-title = Scorciatoie globali: registra, Vai in diretta, transizione, salva replay
controls-keys = ⌨ Tasti…
controls-scripts-title = Script Lua in sandbox: reagiscono a eventi go-live/scena/registrazione, pilotano lo studio
controls-scripts = ⚡ Script…
controls-docks-title = Dock browser: apri un popout della chat, una pagina di avvisi o i pulsanti Companion come finestra accanto allo studio
controls-docks = ⧉ Dock…
controls-remote-title = API remota WebSocket per controller Stream Deck / Companion (disattivata di default)
controls-remote = ⌁ Remoto…
controls-profiles-title = Profili (impostazioni) + collezioni di scene — snapshot commutabili
controls-profiles = ▣ Profili…
controls-bug-title = Segnala un bug — anonimo, opt-in (nulla viene inviato automaticamente)
controls-bug = 🐞 Segnala un bug…
controls-updates-title = Controlla aggiornamenti — firmati, verificati, nulla viene scaricato senza un clic
controls-updates = ⭳ Controlla aggiornamenti…
controls-saved = Salvato: { $path }

# --- MixerDock.tsx ---
mixer-title = Mixer audio
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Passa alle strisce orizzontali
mixer-switch-to-vertical = Passa alle strisce verticali
mixer-layout-aria-vertical = Layout mixer: verticale — passa a orizzontale
mixer-layout-aria-horizontal = Layout mixer: orizzontale — passa a verticale
mixer-empty = Nessuna sorgente audio in questa scena — aggiungi una Cattura ingresso audio (mic) o una Cattura uscita audio (audio desktop) con "+" in Sorgenti. Le strisce ottengono un misuratore VU, fader, muto, monitoraggio, filtri e assegnazione traccia.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Loudness del programma (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Loudness momentaneo (400 ms)
mixer-short-term-title = Loudness a breve termine (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Dispositivo di uscita del monitor
mixer-default-output = Uscita predefinita

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memoria
stats-dropped = Persi
stats-render = Render
stats-gpu = GPU
stats-gpu-compositing = compositing
stats-gpu-idle = inattivo
stats-vertical-fps = FPS 9:16
stats-targets-label = Destinazioni stream
stats-shared-encode = · codifica condivisa
stats-starting = Avvio del compositor…

# --- ScenesRail.tsx ---
scenes-title = Scene
scenes-new-scene-name = Scena
scenes-add = Aggiungi una scena
scenes-empty = Connessione al core dello studio…
scenes-rename = Rinomina { $name }
scenes-on-program = In onda
scenes-preview = Anteprima { $name }
scenes-switch-to = Passa a { $name }
scenes-move-up = Sposta su
scenes-move-up-aria = Sposta { $name } su
scenes-move-down = Sposta giù
scenes-move-down-aria = Sposta { $name } giù
scenes-last-stays = L'ultima scena resta
scenes-remove = Rimuovi questa scena
scenes-remove-aria = Rimuovi { $name }


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
channelstrip-level = Livello
channelstrip-monitor-off = Monitor off
channelstrip-monitor-only = Solo monitor (non nel mix)
channelstrip-monitor-and-output = Monitor e uscita
channelstrip-status-error = errore
channelstrip-status-live = in diretta
channelstrip-status-waiting-audio = in attesa dell'audio
channelstrip-status = stato: { $state }
channelstrip-status-waiting = in attesa
channelstrip-mute = Muto
channelstrip-unmute = Riattiva audio
channelstrip-mute-source = Silenzia { $name }
channelstrip-unmute-source = Riattiva l'audio di { $name }
channelstrip-scene-mix-on = Mix per scena ATTIVO — questa striscia sovrascrive il mix globale per questa scena (clicca per seguire di nuovo il mix globale)
channelstrip-scene-mix-off = Mix per scena — dai a questa striscia il proprio fader/muto per la scena corrente
channelstrip-scene-mix-label = Mix per scena per { $name }
channelstrip-monitor-cycle = { $mode } — clicca per scorrere
channelstrip-monitor-mode = Modalità monitor di { $name }: { $mode }
channelstrip-audio-filters-title = Filtri audio (denoise, gate, compressore…)
channelstrip-audio-filters-label = Filtri audio per { $name }
channelstrip-advanced-title = Offset di sincronizzazione e scorciatoie push-to-talk
channelstrip-advanced-label = Impostazioni audio avanzate per { $name }
channelstrip-track-assignment = Assegnazione traccia
channelstrip-track = Traccia { $n }
channelstrip-track-assigned = Traccia { $n } (assegnata)
channelstrip-track-label = Traccia { $n } per { $name }
channelstrip-device-error = errore dispositivo
channelstrip-audio-device-error = errore dispositivo audio
channelstrip-volume-label = Volume di { $name } in decibel
channelstrip-ptt-hold = Push-to-talk: tieni premuto { $key }
channelstrip-sync-offset = Offset di sincronizzazione (ms, 0–{ $max } — ritarda questo audio)
channelstrip-ptt-hotkey = Scorciatoia push-to-talk (silenziosa se non premuta)
channelstrip-ptt-placeholder = es. Ctrl+Shift+T o F13
channelstrip-ptt-aria = Scorciatoia push-to-talk
channelstrip-ptm-hotkey = Scorciatoia push-to-mute (silenziosa mentre è premuta)
channelstrip-ptm-placeholder = es. Ctrl+Shift+M
channelstrip-ptm-aria = Scorciatoia push-to-mute
channelstrip-hotkeys-note = Le scorciatoie funzionano mentre altre app hanno il focus. Su Linux/Wayland le scorciatoie globali potrebbero non essere disponibili — è un limite del compositor, detto onestamente.
channelstrip-apply = Applica


# --- LiveButton.tsx ---
livebutton-failure-ended = lo stream è terminato
livebutton-title-live = Termina lo stream — ogni destinazione (una registrazione in corso continua)
livebutton-title-offline = Vai in diretta su ogni destinazione abilitata in Impostazioni → Stream
livebutton-end-stream = ■ Termina stream
livebutton-aria-reconnecting = Riconnessione
livebutton-aria-live = In diretta
livebutton-badge-retry = tentativo { $n }
livebutton-badge-live = in diretta
livebutton-go-live = ⦿ Vai in diretta


# --- RecDot.tsx ---
recdot-paused-aria = Registrazione in pausa
recdot-recording-aria = Registrazione
recdot-tracks-one = { $count } traccia audio in registrazione
recdot-tracks-other = { $count } tracce audio in registrazione
recdot-paused = in pausa


# --- ReplayControls.tsx ---
replaycontrols-saved = Replay salvato — { $name }
replaycontrols-failure-stopped = il buffer si è fermato
replaycontrols-title-disarm = Disarma il buffer di replay (scarta la cronologia non salvata)
replaycontrols-title-arm = Arma il buffer di replay a scorrimento — tiene pronti gli ultimi N secondi da salvare (con la propria codifica leggera; lo stream e la registrazione restano intatti)
replaycontrols-replay-seconds = ⟲ Replay { $seconds }s
replaycontrols-arm = ⟲ Arma buffer di replay
replaycontrols-save-title = Salva gli ultimi N secondi nella cartella delle registrazioni (anche con la scorciatoia Salva replay)
replaycontrols-save = ⤓ Salva


# --- PropertiesDialog.tsx ---
properties-title = Proprietà — { $name }
properties-name = Nome
properties-cancel = Annulla
properties-apply = Applica
properties-youtube = YouTube — URL del canale / watch / live_chat (nessuna chiave, nessun accesso, mai)
properties-twitch = Twitch — nome del canale (anonimo)
properties-kick = Kick — slug del canale (endpoint pubblico)
properties-width-px = Larghezza (px)
properties-lines = Righe
properties-font-px = Carattere (px)
properties-images = File immagine (un percorso per riga, mostrati in ordine)
properties-per-slide = Per slide (ms)
properties-crossfade = Dissolvenza incrociata (ms, 0 = taglio)
properties-loop-slideshow = Ripeti (off = mantieni l'ultima slide)
properties-shuffle = Ordine casuale a ogni ciclo
properties-nested-scene = Scena che questa sorgente compone (una scena che contiene già questa viene rifiutata)
properties-portal-note = Il portal ScreenCast di Wayland sceglie schermo o finestra nella finestra di sistema ogni volta che questa sorgente si avvia — non c'è nulla da configurare qui, per scelta.
properties-appaudio-capturing = Cattura audio da { $exe }
properties-appaudio-exe-fallback = un'applicazione
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Aggiungi di nuovo la sorgente per puntare a un'app diversa (l'id di processo cambia quando l'app si riavvia).
properties-image-file = File immagine
properties-media-file = File media (mp4, mkv, webm, mov, .frec o un'immagine)
properties-media-loop = Ripeti (riparti dall'inizio alla fine)
properties-media-hwdecode = Decodifica hardware (torna da sola al software)
properties-media-note = .frec viene riprodotto tramite il codec proprietario freally-video — niente da scaricare. Gli altri formati video vengono decodificati tramite il componente FFmpeg on-demand. L'audio del file ottiene la propria striscia del mixer; l'offset di sincronizzazione della striscia regola con precisione l'allineamento A/V. Una clip senza audio lascia la sua striscia silenziosa.
properties-color = Colore
properties-width = Larghezza
properties-height = Altezza
properties-text = Testo
properties-font-family = Famiglia del carattere (di sistema; vuoto = predefinito)
properties-size-px = Dimensione (px)
properties-text-color = Colore del testo
properties-align = Allineamento
properties-align-left = sinistra
properties-align-center = centro
properties-align-right = destra
properties-line-spacing = Interlinea
properties-wrap-width = Larghezza di a capo (px; 0 = off)
properties-force-rtl = Forza destra-a-sinistra
properties-text-note = Il rendering usa una vera composizione tipografica (giunzione araba, legature) e ordinamento bidi delle righe. La famiglia Noto Sans inclusa (con Arabo/Ebraico) è quella predefinita; funzionano anche le famiglie di sistema. Il CJK usa per ora i caratteri di sistema.
properties-repick-capturing = Cattura: { $label }
properties-repick-looking = Ricerca sorgenti…
properties-repick-none-displays = Nessuno schermo trovato da riselezionare.
properties-repick-none-windows = Nessuna finestra trovata da riselezionare.
properties-repick-again = Seleziona di nuovo:
properties-device = Dispositivo
properties-video-current-device = (dispositivo attuale)
properties-format = Formato
properties-format-auto-loading = Auto (caricamento formati…)
properties-format-auto = Auto (risoluzione massima)
properties-audio-capture-of = Cattura l'audio di
properties-audio-default-output = Uscita predefinita (ciò che senti)
properties-audio-default-input = Ingresso predefinito
properties-audio-default-suffix = (predefinito)
properties-audio-current-device = (dispositivo attuale: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Guadagno
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Compressore
audiofilters-name-limiter = Limiter
audiofilters-name-eq = EQ a 3 bande
audiofilters-name-denoise = Denoise
audiofilters-name-ducking = Ducking
audiofilters-title = Filtri audio — { $name }
audiofilters-chain-header = Catena di filtri (il primo in alto viene eseguito per primo, prima del fader)
audiofilters-add = + Aggiungi filtro
audiofilters-add-menu = Aggiungi un filtro audio
audiofilters-empty = Ancora nessun filtro — fai il denoise di un mic (DSP classico, no ML), applica il gate alla stanza, doma i picchi con il compressore o abbassa la musica sotto la tua voce.
audiofilters-enable = Abilita { $name }
audiofilters-run-earlier = Esegui prima
audiofilters-move-up = Sposta { $name } su
audiofilters-run-later = Esegui dopo
audiofilters-move-down = Sposta { $name } giù
audiofilters-remove-title = Rimuovi filtro
audiofilters-remove = Rimuovi { $name }
audiofilters-gain-db = Guadagno (dB)
audiofilters-open-db = Apre a (dB)
audiofilters-close-db = Chiude a (dB)
audiofilters-attack-ms = Attacco (ms)
audiofilters-hold-ms = Tenuta (ms)
audiofilters-release-ms = Rilascio (ms)
audiofilters-ratio = Rapporto (:1)
audiofilters-threshold-db = Soglia (dB)
audiofilters-output-gain-db = Guadagno di uscita (dB)
audiofilters-ceiling-db = Tetto (dB)
audiofilters-low-db = Bassi (dB)
audiofilters-mid-db = Medi (dB)
audiofilters-high-db = Alti (dB)
audiofilters-strength = Intensità
audiofilters-denoise-note = Soppressione spettrale DSP classica proprietaria — il rumore costante (ventole, fruscio) cala mentre la voce passa. No ML, no modelli, come da charter.
audiofilters-duck-under = Abbassa sotto
audiofilters-ducking-trigger = Sorgente trigger del ducking
audiofilters-pick-trigger = (scegli un trigger — es. il tuo mic)
audiofilters-trigger-at-db = Attiva a (dB)
audiofilters-duck-by-db = Abbassa di (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Ritardo di rendering
filters-name-color-correction = Correzione colore
filters-name-lut = Applica LUT
filters-name-blur = Sfocatura
filters-name-mask = Maschera immagine
filters-name-sharpen = Nitidezza
filters-name-scroll = Scorrimento
filters-name-crop = Ritaglio
filters-title = Filtri — { $name }
filters-blend-mode = Modalità di fusione
filters-chain-header = Catena di filtri (il primo in alto viene eseguito per primo)
filters-add = + Aggiungi filtro
filters-add-menu = Aggiungi un filtro
filters-empty = Ancora nessun filtro — applica un chroma key a una webcam, correggi il colore di una cattura o fai scorrere un ticker.
filters-enable = Abilita { $name }
filters-run-earlier = Esegui prima
filters-move-up = Sposta { $name } su
filters-run-later = Esegui dopo
filters-move-down = Sposta { $name } giù
filters-remove-title = Rimuovi filtro
filters-remove = Rimuovi { $name }
filters-key-color-rgb = Colore chiave (qualsiasi colore, distanza RGB)
filters-similarity = Similarità
filters-smoothness = Morbidezza
filters-luma-min = Luma min (esclude i toni più scuri)
filters-luma-max = Luma max (esclude i toni più chiari)
filters-delay = Ritardo (ms — solo video, es. per sincronizzare con l'audio; max 500)
filters-key-color = Colore chiave
filters-spill = Spill
filters-gamma = Gamma
filters-brightness = Luminosità
filters-contrast = Contrasto
filters-saturation = Saturazione
filters-hue-shift = Spostamento tonalità
filters-opacity = Opacità
filters-cube-file = File .cube
filters-amount = Quantità
filters-radius = Raggio
filters-mask-image = Immagine maschera
filters-mask-mode = Modalità
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = inverti
filters-speed-x = Velocità X (px/s)
filters-speed-y = Velocità Y (px/s)
filters-crop-left = sinistra
filters-crop-top = alto
filters-crop-right = destra
filters-crop-bottom = basso
filters-crop-aria = ritaglia { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Aggiorna
pickershell-refresh-title = Aggiorna l'elenco
pickershell-close = Chiudi


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Segnala un bug
bugreport-intro = Le segnalazioni sono anonime e opt-in — nulla viene inviato automaticamente. Rivedrai il testo esatto qui sotto, poi lo invierai tramite una issue GitHub precompilata o la tua app di posta. Nessun dato personale (il percorso home e il nome utente sono oscurati); nessun account, nessun server.
bugreport-crash-notice = Freally Capture si è chiuso inaspettatamente in un'esecuzione precedente — i dettagli anonimi del crash sono inclusi qui sotto. Segnalarli aiuta a risolverlo in fretta.
bugreport-description-label = Cosa stavi facendo quando è successo? (facoltativo)
bugreport-description-placeholder = es. l'anteprima si è bloccata quando ho aggiunto una seconda webcam
bugreport-include-crash = Includi i dettagli anonimi del crash dell'ultima esecuzione
bugreport-preview-label = Esattamente ciò che verrà inviato
bugreport-open-github = Apri issue GitHub
bugreport-gmail-title = Apre la finestra di composizione di Gmail nel browser, precompilata. Non hai effettuato l'accesso? Google mostra prima la sua schermata di login.
bugreport-compose-gmail = Componi in Gmail
bugreport-email-title = Apre una bozza nell'app di posta predefinita di questo PC (Outlook, Thunderbird, Mail…)
bugreport-send-email = Invia email
bugreport-copied = Copiato ✓
bugreport-copy-report = Copia segnalazione
bugreport-dismiss-crash = Ignora crash
bugreport-copy-failed = copia non riuscita — seleziona il testo e copialo manualmente
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = COSA È SUCCESSO
bugreport-preview-no-description = (nessuna descrizione fornita)
bugreport-preview-diagnostics = DIAGNOSTICA ANONIMA (nessun dato personale)
bugreport-preview-from = Da: Freally Capture
bugreport-preview-crash-excerpt = --- estratto del crash ---


# --- Updates.tsx ---
updates-title = Aggiornamento software
updates-checking = Ricerca aggiornamenti…
updates-uptodate = Hai la versione più recente.
updates-check-again = Controlla di nuovo
updates-available = La versione { $version } è disponibile
updates-current-version = (hai la { $current })
updates-release-notes-label = Versione { $version } — Note di rilascio
updates-confirm = Vuoi aggiornare ora? Il download viene verificato con la chiave di firma inclusa prima di essere applicato. Freally Capture si chiude, l'installer viene eseguito e la nuova versione si riapre da sola.
updates-yes-update-now = Sì, aggiorna ora
updates-no-not-now = No, non ora
updates-downloading = Download di { $version }…
updates-starting = avvio…
updates-installed = Aggiornamento installato.
updates-restart-now = Riavvia ora
updates-restart-later = Riavvia più tardi
updates-try-again = Riprova


# --- Models.tsx ---
models-title = Componenti
models-ffmpeg-heading = FFmpeg — codec di rete
models-badge-third-party = Terze parti · non incluso
models-ffmpeg-desc = Il motore proprietario di Freally Capture registra freally-video (.frec) senza perdite e senza nulla di aggiuntivo. Registrare i formati di rete che piattaforme e player si aspettano — H.264/AAC (e HEVC/AV1) in mp4/mkv/mov/webm — usa FFmpeg, uno strumento separato che questa app non include mai: quei codec sono gravati da brevetti, quindi resta opzionale e chiaramente etichettato. Viene scaricato on-demand dalla build fissata qui sotto, verificato con SHA-256 prima del primo uso, memorizzato per utente e gestito come processo separato. La sua licenza (LGPL/GPL) è a sé — vedi THIRD-PARTY-NOTICES.
models-checking = Controllo…
models-ffmpeg-not-installed = Non installato. Disponibile: FFmpeg { $version } da { $source } ({ $size } di download).
models-ffmpeg-none-pinned = Nessuna build di FFmpeg è ancora fissata per questa piattaforma — la registrazione con codec di rete non è disponibile qui. La registrazione freally-video senza perdite non è influenzata.
models-ffmpeg-download-verify = Scarica e verifica ({ $size })
models-downloading = Download…
models-download-of = di
models-cancel = Annulla
models-ffmpeg-verifying = Verifica del download con lo SHA-256 fissato…
models-ffmpeg-extracting = Estrazione…
models-ffmpeg-ready = Installato e verificato — { $version }
models-remove = Rimuovi
models-ffmpeg-retry = Riprova il download
models-network-note = Il download è l'unica azione di rete di questo pannello e non parte mai da solo. Un checksum errato annulla l'installazione — l'app si rifiuta di eseguire byte di cui non può garantire.
models-cef-heading = Runtime della Sorgente Browser — Chromium (CEF)
models-cef-desc = Le sorgenti browser renderizzano pagine web (avvisi, widget, overlay) tramite Chromium Embedded Framework — un runtime da ~100 MB che questa app non include mai. Viene scaricato on-demand dall'indice ufficiale delle build CEF, verificato con lo SHA-1 di quell'indice prima di estrarre qualsiasi cosa e memorizzato per utente. La sorgente browser che ne fa uso arriva con la propria milestone; questo installa il runtime che le serve.
models-cef-download-install = Scarica e installa
models-cef-unsupported = CEF non pubblica alcuna build per questa piattaforma — le sorgenti browser non sono disponibili qui.
models-cef-resolving = Risoluzione dell'ultima build stabile…
models-cef-verifying = Verifica del download con lo SHA-1 dell'indice…
models-cef-extracting = Estrazione del runtime…
models-cef-ready = Installato — CEF { $version }.
models-cef-retry = Riprova
models-integrations-heading = Integrazioni opzionali
models-badge-never-bundled = Mai incluso
models-ndi-detected = Rilevato
models-ndi-not-installed = Non installato
models-vst-available = Disponibile
models-vst-not-available = Non disponibile


# --- Recordings.tsx ---
recordings-title = Registrazioni
recordings-loading = Lettura della cartella…
recordings-empty = Ancora nessuna registrazione — Avvia registrazione scrive nella cartella impostata in Uscita.
recordings-frec-label = senza perdite proprietario (freally-video)
recordings-remux-title = Reincapsula come mp4 — copia dello stream, senza ricodifica, senza perdita di qualità (richiede il componente FFmpeg)
recordings-remuxing = Reincapsulamento…
recordings-remux-to-mp4 = Remux in MP4
recordings-export-mp4-title = Decodifica il .frec proprietario e ricodifica in MP4 (H.264/AAC) così si riproduce in qualsiasi player — richiede il componente FFmpeg
recordings-exporting = Esportazione…
recordings-export-mp4 = Esporta → MP4
recordings-export-mkv-title = Decodifica il .frec proprietario e ricodifica in MKV così si riproduce in qualsiasi player
recordings-starting = avvio…
recordings-frames = { $done } / { $total } fotogrammi
recordings-cancel = Annulla
recordings-export-cancelled = Esportazione annullata.
recordings-exported-to = Esportato in { $path }
recordings-remuxed-to = Reincapsulato in { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Apri registrazione .frec
openfrec-desc = Freally Capture registra il formato proprietario .frec senza perdite — non lo riproduce. Freally Player riprodurrà i .frec direttamente quando verrà rilasciato. Per ora, esportalo in MP4/MKV e si riproduce in qualsiasi player (VLC, il player del tuo OS, qualsiasi cosa).
openfrec-exported-to = Esportato in { $path }
openfrec-exporting = Esportazione…
openfrec-starting = avvio…
openfrec-export-mp4 = Esporta → MP4
openfrec-export-mkv = Esporta → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Canvas verticale (9:16)
vertical-enable = Abilita il secondo canvas — registrabile e trasmettibile in modo indipendente dal programma
vertical-scene-label = Scena che questo canvas compone
vertical-width = Larghezza
vertical-height = Altezza
vertical-preview-alt = Anteprima del canvas verticale
vertical-note = Le posizioni degli elementi sono fedeli al pixel tra i canvas: seleziona questa scena nella barra Scene per disporla mentre questa anteprima mostra il risultato verticale. Le destinazioni stream scelgono questo canvas in ⦿ Stream…; Impostazioni → Uscita può registrarlo insieme al file principale.
vertical-close = Chiudi


# --- EulaGate.tsx ---
eula-title = Freally Capture — Contratto di licenza
eula-version = v{ $version }
eula-intro = Leggi e accetta questo contratto per usare Freally Capture. In breve: è uno strumento neutrale, e sei l'unico responsabile di ciò che catturi, registri e trasmetti — e di averne i diritti.
eula-thanks = Grazie per aver letto.
eula-scroll-hint = Scorri fino alla fine per continuare.
eula-decline = Rifiuta ed esci
eula-agree = Accetto


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Uscita
output-loading = Le impostazioni sono ancora in caricamento…
output-container-frec = freally-video (.frec) — senza perdite, proprietario, niente da scaricare
output-container-mkv = MKV — tollerante ai crash; reincapsula in mp4 in seguito
output-container-mp4 = MP4 — si riproduce ovunque
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Senza perdite
output-preset-lossless-title = Il codec proprietario freally-video — esatto al bit, nessun download
output-preset-high-label = Alta qualità
output-preset-high-title = MP4, miglior encoder rilevato, CQ 16 quasi senza perdite, preset Qualità
output-preset-balanced-label = Bilanciato
output-preset-balanced-title = MKV, miglior encoder rilevato, CQ 23, preset Bilanciato
output-recording-format = Formato di registrazione
output-ffmpeg-warning = Questo formato richiede il componente FFmpeg (codec di rete — non incluso). Il .frec senza perdite non richiede nulla.
output-install = Installa…
output-recordings-folder = Cartella delle registrazioni
output-folder-placeholder = Cartella Video dell'OS
output-filename-prefix = Prefisso del nome file
output-frame-rate = Frame rate
output-fps-option = { $fps } fps
output-split-every = Suddividi ogni (minuti, 0 = off)
output-output-width = Larghezza di uscita (0 = canvas; solo formati di rete)
output-output-height = Altezza di uscita (0 = canvas)
output-record-vertical = Registra anche il canvas verticale (un file parallelo "… (verticale)"; richiede il canvas 9:16 abilitato)
output-audio-tracks = Tracce audio
output-recorded-tracks-group = Tracce registrate
output-track-last-one = Almeno una traccia deve registrare
output-record-track-on = Registra traccia { $index }: on
output-record-track-off = Registra traccia { $index }: off
output-encoder-heading = Encoder
output-video-encoder = Encoder video
output-encoder-auto = Auto — miglior rilevato (H.264)
output-encoder-unavailable = — non disponibile qui
output-preset = Preset
output-preset-quality = Qualità
output-preset-balanced-option = Bilanciato
output-preset-performance = Prestazioni
output-rate-control = Controllo del bitrate
output-rc-cqp = CQP (qualità costante)
output-rc-cbr = CBR (bitrate costante)
output-rc-vbr = VBR (bitrate variabile)
output-cq = CQ (0–51, più basso = migliore)
output-bitrate = Bitrate (kbps)
output-keyframe = Intervallo keyframe (s)
output-audio-bitrate = Bitrate audio (kbps / traccia)
output-presets = Preset:

# --- SettingsStream.tsx ---
stream-title = Impostazioni — Stream
stream-target-enabled = Destinazione { $index } abilitata
stream-target = Destinazione { $index }
stream-remove = Rimuovi
stream-service = Servizio
stream-canvas = Canvas
stream-canvas-main = Principale (programma)
stream-canvas-vertical = Verticale (9:16 — abilitalo nello studio)
stream-ingest-srt = URL di ingest SRT
stream-ingest-whip = URL endpoint WHIP
stream-ingest-url = URL di ingest
stream-ingest-override = (override — vuoto = il preset del servizio)
stream-key-srt = streamid (facoltativo — aggiunto come ?streamid=…; trattato come segreto)
stream-key-whip = Token Bearer (facoltativo — inviato come header Authorization; un segreto)
stream-key-custom = Chiave di stream (dal tuo server — trattata come segreto)
stream-key-service = Chiave di stream (dalla tua dashboard creator — trattata come segreto)
stream-key-aria = Chiave di stream { $index }
stream-key-hide = Nascondi
stream-key-show = Mostra
stream-encoder = Encoder (H.264 — ciò che RTMP, SRT e WHIP trasportano tutti)
stream-encoder-auto = Auto — il miglior encoder H.264 rilevato
stream-encoder-unavailable = (non disponibile qui)
stream-video-bitrate = Bitrate video (kbps, CBR)
stream-audio-bitrate = Bitrate audio (kbps)
stream-fps = FPS
stream-keyframe = Intervallo keyframe (s)
stream-audio-track = Traccia audio (1–6)
stream-output-width = Larghezza di uscita (0 = canvas)
stream-output-height = Altezza di uscita (0 = canvas)
stream-add-target = + Aggiungi destinazione
stream-go-live-note = Vai in diretta pubblica su ogni destinazione abilitata contemporaneamente, direttamente su ciascuna piattaforma. Le destinazioni con impostazioni di encoder identiche condividono un'unica codifica.
stream-auto-record = Avvia la registrazione quando vado in diretta (la registrazione si ferma comunque in modo indipendente)
stream-ffmpeg-note-before = I codec di rete per lo streaming passano attraverso il componente ffmpeg on-demand etichettato —
stream-ffmpeg-note-link = gestiscilo qui
stream-ffmpeg-note-after = . La registrazione locale continua qualunque cosa faccia lo stream.
stream-cancel = Annulla
stream-save = Salva

# --- SettingsReplay.tsx ---
replay-title = Impostazioni — Buffer di replay
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Bassa (3 Mbps)
replay-quality-standard = Standard (6 Mbps)
replay-quality-high = Alta (12 Mbps)
replay-length-presets = Preset di durata
replay-quality-presets = Preset di qualità
replay-length-seconds = Durata (secondi)
replay-video-bitrate = Bitrate video (kbps)
replay-fps = FPS
replay-audio-track = Traccia audio (1–6)
replay-note = Quando è armato, il buffer esegue la propria codifica leggera in un ring su disco limitato — circa { $mb } MB con queste impostazioni. Il salvataggio unisce il ring senza ricodificare e non tocca mai lo stream o la registrazione. Le modifiche si applicano alla prossima volta che armi.
replay-cancel = Annulla
replay-save = Salva

# --- SettingsRemote.tsx ---
remote-title = Impostazioni — Controllo remoto
remote-enable = Abilita l'API remota WebSocket
remote-password = Password (obbligatoria — i controller si autenticano con essa)
remote-password-placeholder = una password per i tuoi controller
remote-password-hide = Nascondi
remote-password-show = Mostra
remote-port = Porta
remote-allow-lan = Consenti connessioni LAN (di default solo questa macchina)
remote-note = Off = la porta è chiusa. On = un WebSocket protetto da password su 127.0.0.1 (o la tua LAN se abilitata) che può cambiare scena, eseguire la transizione, avviare/interrompere stream e registrazione, salvare replay e impostare muto/volumi — le stesse azioni dell'interfaccia, nulla di più. Non può leggere file. Tratta la password come qualsiasi credenziale; preferisci solo-questa-macchina a meno che tu non controlli specificamente da un altro dispositivo.
remote-password-required = È necessaria una password per abilitare l'API remota.
remote-cancel = Annulla
remote-save = Salva

# --- SettingsHotkeys.tsx ---
hotkeys-title = Impostazioni — Scorciatoie
hotkeys-record = Avvia / interrompi registrazione
hotkeys-record-placeholder = es. Ctrl+Shift+R
hotkeys-go-live = Vai in diretta / Termina stream
hotkeys-go-live-placeholder = es. Ctrl+Shift+L
hotkeys-transition = Transizione Modalità Studio
hotkeys-transition-placeholder = es. Ctrl+Shift+T o F13
hotkeys-save-replay = Salva replay (ultimi N secondi)
hotkeys-save-replay-placeholder = es. Ctrl+Shift+S
hotkeys-add-marker = Inserisci un marcatore di capitolo (registrazione)
hotkeys-add-marker-placeholder = es. Ctrl+Shift+K
hotkeys-note = Le scorciatoie sono globali — si attivano mentre altre app hanno il focus. Vuoto = non assegnata. I tasti push-to-talk/mute del mixer si trovano nel menu ⋯ di ogni striscia. Su Linux/Wayland le scorciatoie globali potrebbero non essere disponibili (un limite del compositor) — i pulsanti continuano a funzionare.
hotkeys-cancel = Annulla
hotkeys-save = Salva

# --- WorkspaceDialog.tsx ---
workspace-title = Profili e collezioni di scene
workspace-profiles = Profili
workspace-profiles-hint = Un profilo è le tue impostazioni — destinazione stream, uscita, scorciatoie. Cambialo per show o per piattaforma.
workspace-collections = Collezioni di scene
workspace-collections-hint = Una collezione è le tue scene + sorgenti. Crea duplica quella corrente come punto di partenza.
workspace-active = Attiva
workspace-switch-to = Passa a { $name }
workspace-active-marker = ● attiva
workspace-new-name-placeholder = nuovo nome…
workspace-new-name-label = Nuovo nome { $title }
workspace-create = Crea

# --- ScriptsDialog.tsx ---
scripts-title = Script (Lua)
scripts-empty = Ancora nessuno script — aggiungi un file .lua. Vedi scripts/sample.lua per l'API: reagisci a eventi go-live/scena/registrazione e piloti gli stessi comandi dell'API remota.
scripts-enable = Abilita { $path }
scripts-remove = Rimuovi { $path }
scripts-path-label = Percorso dello script
scripts-add = Aggiungi
scripts-note = Gli script vengono eseguiti in sandbox — nessun accesso a file o OS; possono solo chiamare gli stessi comandi dello studio dell'API remota (cambio scena, transizione, registrazione/stream/replay, muto). Un errore in uno script viene registrato e contenuto. Le modifiche si applicano entro un secondo.
scripts-error-not-lua = Punta a un file .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Dock browser
browser-dock-empty = Ancora nessun dock — aggiungi un popout della chat, una pagina di avvisi o i tuoi pulsanti web Companion.
browser-dock-open = Apri
browser-dock-remove = Rimuovi { $name }
browser-dock-name-placeholder = nome (es. Chat Twitch)
browser-dock-name-label = Nome del dock
browser-dock-url-label = URL del dock
browser-dock-note = Un dock si apre come finestra a sé che puoi posizionare accanto allo studio. La pagina non ottiene alcun accesso all'app — la renderizza soltanto. Solo URL http(s); i dock si aprono solo quando clicchi Apri.
browser-dock-error-name = Dai un nome al dock (es. Chat Twitch).
browser-dock-error-url = Un URL del dock deve iniziare con http:// o https://.

# --- studio-preview-pane ---
studio-preview-label = Anteprima Modalità Studio
studio-preview-heading = Anteprima
studio-preview-hint = clicca una scena per caricarla qui
studio-preview-empty = L'anteprima apparirà qui.
studio-preview-mirrors = rispecchia il programma
studio-preview-transition-select = Transizione
studio-preview-duration = Durata della transizione (ms)
studio-preview-commit-title = Manda Anteprima → Programma attraverso la transizione (il pubblico la vede)
studio-preview-transitioning = Transizione in corso…
studio-preview-transition-button = Transizione ⇄
studio-preview-luma-placeholder = immagine wipe in scala di grigi (png/jpg)
studio-preview-luma-label = Immagine Luma wipe
studio-preview-browse = Sfoglia…
studio-preview-filter-images = Immagini
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = video stinger (ProRes 4444 .mov mantiene l'alpha)
studio-preview-stinger-label = File video dello stinger
studio-preview-stinger-cut-label = Punto di taglio dello stinger (ms)
studio-preview-stinger-cut-title = Quando il cambio scena avviene sotto lo stinger (ms dall'inizio della transizione)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Taglio
transition-kind-fade = Dissolvenza
transition-kind-slide-left = Scorri ←
transition-kind-slide-right = Scorri →
transition-kind-slide-up = Scorri ↑
transition-kind-slide-down = Scorri ↓
transition-kind-swipe-left = Spazza ←
transition-kind-swipe-right = Spazza →
transition-kind-luma-linear = Luma wipe (lineare)
transition-kind-luma-radial = Luma wipe (radiale)
transition-kind-luma-horizontal = Luma wipe (orizzontale)
transition-kind-luma-diamond = Luma wipe (rombo)
transition-kind-luma-clock = Luma wipe (orologio)
transition-kind-image = Wipe immagine (personalizzato)
transition-kind-stinger = Stinger (video)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Personalizzato (RTMP/RTMPS)
stream-service-srt = SRT (auto-ospitato)
stream-service-whip = WHIP (WebRTC)
