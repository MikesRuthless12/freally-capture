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
sources-badge-test-bars = Barre
sources-badge-test-grid = Griglia
sources-badge-test-sweep = Sweep
sources-badge-test-tone = Tono
sources-badge-test-sync = Sync
sources-badge-timer = Timer

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
sources-add-timer = Timer / Orologio
sources-add-nested-scene = Scena annidata
sources-add-slideshow = Presentazione immagini
sources-add-chat-overlay = Overlay chat live
sources-add-test-signal = Segnale di prova
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
sources-stream-hide = Nascondi nello stream
sources-stream-show = Mostra nello stream
sources-stream-hide-item = Nascondi { $name } nello stream
sources-stream-show-item = Mostra { $name } nello stream
sources-record-hide = Nascondi nella registrazione
sources-record-show = Mostra nella registrazione
sources-record-hide-item = Nascondi { $name } nella registrazione
sources-record-show-item = Mostra { $name } nella registrazione
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
sources-testsignal-title = Aggiungi un segnale di prova
sources-testsignal-pattern-label = Pattern
sources-testsignal-bars = Barre di colore SMPTE
sources-testsignal-grid = Griglia di calibrazione
sources-testsignal-sweep = Sweep di movimento
sources-testsignal-tone = Tono a 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Flash + bip di sincronia A/V
sources-testsignal-note = Verifica scene, encoder, proiettori e destinazioni di streaming senza una camera collegata. Il pattern flash + bip alimenta il banco di sincronia A/V.
sources-testsignal-add = Aggiungi segnale di prova
sources-timer-title = Aggiungi un timer
sources-timer-mode-label = Modalità
sources-timer-wall-clock = Orologio
sources-timer-countdown = Conto alla rovescia
sources-timer-stopwatch = Cronometro
sources-timer-since-live = Tempo dal live
sources-timer-since-recording = Tempo dalla registrazione
sources-timer-note = Durata, formato, stile e azioni di fine conto vivono nelle Proprietà della sorgente.
sources-timer-add = Aggiungi timer

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
controls-iso-lanes = Corsie ISO in registrazione accanto al programma: { $count }
controls-pause-title-resume = Riprendi — il file continua come un'unica timeline contigua
controls-pause-title-pause = Metti in pausa — nessun fotogramma viene scritto; riprendendo continua lo stesso file riproducibile
controls-resume-recording = ▶ Riprendi registrazione
controls-pause-recording = ⏸ Metti in pausa la registrazione
controls-reactions-label = Reazioni (incorporate nel programma)
controls-reactions-title = Fai fluttuare una reazione sul programma — registrata E trasmessa, così il replay mostra il momento esatto. Anche gli spettatori in chat le attivano (le loro emoji di reazione fluttuano automaticamente); un flood limita solo quanto appare sullo schermo.
controls-react = Reagisci { $emoji }
controls-virtual-camera-title = La camera virtuale richiede un componente driver firmato per ogni OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / estensione CoreMediaIO macOS / v4l2loopback Linux) — arriva come milestone a sé. Il modello del feed è pronto: programma, canvas verticale o una singola sorgente, con un mic virtuale abbinato su Windows/Linux (macOS non ha API per il mic virtuale — detto onestamente).
controls-virtual-camera = ⌁ Avvia camera virtuale
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
mixer-routing = Instradamento
mixer-routing-title = Instradamento uscita audio

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Instradamento audio
routing-intro = Assegna i canali ai bus delle tracce, poi invia qualsiasi bus a un'uscita fisica: un segnale per un registratore hardware, altoparlanti in un'altra stanza o un cue in cuffia su una traccia libera. Il monitor mantiene il proprio dispositivo; questi instradamenti si aggiungono sopra, quindi senza nessuno impostato il mix resta invariato.
routing-sends-title = Invii alle tracce
routing-no-strips = Nessuna sorgente audio in questa scena.
routing-source = Sorgente
routing-track = Traccia { $n }
routing-send-aria = Invia { $source } alla traccia { $n }
routing-outputs-title = Uscite fisiche
routing-master = Master
routing-off = Off
routing-default-output = Uscita predefinita
routing-device-aria = Dispositivo di uscita per { $bus }
routing-trim-aria = Trim di uscita per { $bus }
routing-trim-db = { $db } dB
routing-muted = Muto
routing-device-error = Dispositivo non disponibile

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Ducking
mixer-ducking-title = Matrice di ducking
ducking-title = Matrice di ducking
ducking-intro = Qualsiasi sorgente può abbassare tutte le altre. Una cella abbassa la destinazione (colonna) ogni volta che il trigger (riga) parla — seleziona una cella per impostarne profondità, soglia e tempi. Ogni coppia è un ducking a sé, quindi un canale può essere abbassato da più trigger contemporaneamente.
ducking-need-two = Aggiungi almeno due sorgenti audio per applicare il ducking tra loro.
ducking-trigger-target = Trigger ↓ / Destinazione →
ducking-cell-aria = { $trigger } abbassa { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Rimuovi
ducking-amount = Quantità
ducking-threshold = Soglia
ducking-attack = Attacco
ducking-release = Rilascio
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Normalizzazione del loudness
loudness-intro = Porta gradualmente il programma verso un target di loudness con un tetto di picco, così stream e registrazioni restano a un livello costante. Lento e delicato — guida, non pompa mai.
loudness-enable = Porta il programma al target
loudness-target = Target
loudness-target-option = { $target } LUFS
loudness-ceiling = Tetto di picco (dBFS)
loudness-note = −14 LUFS va bene per la riproduzione in stile YouTube; −16 è un target di streaming comune; −23 è la trasmissione EBU R128. Lo stesso target è usato dall'azione Normalizza dopo la registrazione.
ltc-badge = LTC
ltc-title = Timecode SMPTE (LTC)
ltc-intro = Genera timecode lineare SMPTE su una traccia e leggi l'LTC in ingresso da qualsiasi input audio — timecode audio classico per sincronizzare registratori e telecamere esterni in post. Completamente offline.
ltc-generate = Genera LTC su una traccia
ltc-track = Traccia timecode
ltc-track-option = Traccia { $track }
ltc-fps = Frequenza fotogrammi
ltc-read = Leggi LTC da
ltc-read-off = Spento
ltc-decoded = Timecode in ingresso
ltc-no-lock = nessun segnale
ltc-note = Il generatore si sincronizza sull'ora del giorno, non-drop. Registra la sua traccia (assegnala nelle impostazioni di Uscita) o instradala a un'uscita per alimentare apparati esterni. Il lettore alimenta la riga di timecode dell'overlay statistiche e marca i capitoli.
loudness-on = LUFS { $target }
loudness-off = Norm. off

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Soundboard
mixer-soundboard-title = Soundboard
soundboard-title = Soundboard
soundboard-add-pad = + Pad
soundboard-stop-all = Ferma tutto
soundboard-edit = Modifica
soundboard-empty = Ancora nessun pad — aggiungine uno e assegnagli un clip audio locale.
soundboard-new-pad = Nuovo pad
soundboard-no-clip = Nessun clip
soundboard-audio-files = File audio
soundboard-name = Nome
soundboard-choose-clip = Scegli clip…
soundboard-gain = Guadagno
soundboard-choke = Choke
soundboard-choke-none = Nessuno
soundboard-loop = Loop
soundboard-auto-duck = Auto-ducking
soundboard-tracks = Tracce
soundboard-hotkey = Scorciatoia
soundboard-hotkey-placeholder = es. Ctrl+Shift+1
soundboard-remove = Rimuovi

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Plugin
mixer-plugins-title = Plugin audio (CLAP / VST3)
plugins-title = Plugin audio
plugins-scanning = Scansione…
plugins-none = Nessun plugin CLAP o VST3 trovato nelle cartelle standard.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memoria
stats-dropped = Persi
stats-render = Render
stats-gpu = GPU
stats-gpu-compositing = compositing
stats-gpu-idle = inattivo
stats-disk = Disco
stats-disk-free = liberi
stats-disk-left = Reg. rimasta
stats-disk-rate = ≈ { $rate } MB/s registrazione
stats-vertical-fps = FPS 9:16
stats-targets-label = Destinazioni stream
stats-rehearsal-note = Prova — i target pubblicano solo su un ricevitore locale
stats-timeline-open = Timeline
timeline-title = Timeline della sessione
timeline-empty = Ancora nulla di registrato — la timeline registra mentre trasmetti o registri.
timeline-live = LIVE — sta ancora registrando
timeline-fit = Adatta
timeline-legend-fps = fps
timeline-legend-behind = coda encoder (frame di ritardo)
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
channelstrip-solo-title = Solo (PFL) — il monitor sente solo le strisce in solo; il mix di programma resta intatto
channelstrip-solo-source = Solo di { $name } (PFL)
channelstrip-pan-label = Bilanciamento (doppio clic per azzerare)
channelstrip-pan-aria = Bilanciamento di { $name }
channelstrip-mono-label = Downmix in mono
channelstrip-automix-label = Auto-mix (condivisione del guadagno)
channelstrip-automix-note = Condivisione del guadagno: il mixer mantiene costante il livello combinato di tutte le strisce in auto-mix e lo affida a chi sta parlando — ideale per panel multi-microfono e podcast. Disattivo finché non aggiungi una striscia.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Genera un ritorno senza eco per questa sorgente — tutti nel programma tranne la sorgente stessa. Usalo per un guest remoto così non sente la propria voce ritardata.
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
livebutton-rehearse = Prova generale
livebutton-rehearse-title = Esegui l'intero show verso un ricevitore locale — non viene inviato nulla
livebutton-end-rehearsal = Termina la prova
livebutton-title-rehearsing = È in corso una prova — nulla lascia questa macchina
livebutton-badge-rehearsal = PROVA
livebutton-aria-rehearsal = Prova in corso
livebutton-rehearsal-banner = Prova — nulla lascia questa macchina


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
properties-testtone-note = Una sinusoide continua a 1 kHz a −20 dBFS. Livello e muto vivono sulla sua striscia del mixer; non c'è altro da configurare.
properties-timer-format = Formato ora (strftime)
properties-timer-format-note = es. %H:%M:%S (predefinito), %I:%M %p, %A %H:%M — un pattern non valido torna a %H:%M:%S.
properties-timer-utc = Scostamento UTC (minuti)
properties-timer-utc-placeholder = ora locale
properties-timer-duration = Durata (secondi)
properties-timer-target = Conto alla rovescia fino a (HH:MM)
properties-timer-target-note = Un orario obiettivo corre da solo e si ripete ogni giorno; lascialo vuoto per usare la durata con Avvia/Pausa/Reset.
properties-timer-end = Allo zero
properties-timer-end-none = Niente
properties-timer-end-flash = Lampeggia il timer
properties-timer-end-switch = Cambia scena
properties-timer-end-scene = Scena
properties-timer-size = Dimensione (px)
properties-timer-start = Avvia
properties-timer-pause = Pausa
properties-timer-reset = Reset
properties-text-file = Leggi da file (percorso; vuoto = usa il testo sopra)
properties-text-binding = Interpreta come
properties-text-binding-whole = File intero
properties-text-binding-csv = Cella CSV
properties-text-binding-json = Puntatore JSON
properties-text-csv-row = Riga
properties-text-csv-column = Colonna
properties-text-csv-column-placeholder = nome o numero
properties-text-json-pointer = Puntatore
properties-text-file-note = Il file viene riletto entro mezzo secondo da una modifica. Le scritture atomiche (temp + rinomina) sono tollerate: l'ultimo valore buono resta a schermo durante lo scambio.
avsync-title = Calibrazione sincronia A/V
avsync-intro = Riproduci il pattern integrato flash + bip su schermo e casse, catturalo con la camera e il microfono da allineare: il banco misura lo scarto. Il giro passa da schermo e casse, quindi le loro piccole latenze sono incluse.
avsync-video-label = Camera (sorgente video)
avsync-audio-label = Microfono (sorgente audio)
avsync-pick = Scegli una sorgente…
avsync-no-video = Aggiungi prima la camera come sorgente — il banco misura sorgenti, non dispositivi grezzi.
avsync-no-audio = Aggiungi prima il microfono come sorgente audio.
avsync-projector = Programma a schermo intero su
avsync-projector-open = Apri proiettore
avsync-projector-window-title = Programma — sincronia A/V
avsync-start-note = L'avvio aggiunge temporaneamente una sorgente «Pattern di sincronia A/V» sopra la scena corrente e suona il bip sul dispositivo di monitoraggio. Alla fine viene rimosso tutto.
avsync-manual = Offset di sincronia (ms, manuale)
avsync-start = Avvia calibrazione
avsync-measuring = Misurazione di circa 12 secondi — punta la camera sul programma lampeggiante e tieni la stanza tranquilla…
avsync-flash-seen = La camera vede il flash
avsync-flash-waiting = In attesa che la camera veda il flash…
avsync-beep-heard = Il microfono sente il bip
avsync-beep-waiting = In attesa che il microfono senta il bip…
avsync-cancel = Annulla
avsync-result-offset = Il video arriva { $offset } ms dopo l'audio.
avsync-result-detail = Misurato su { $cycles } cicli, ±{ $jitter } ms.
avsync-negative = L'audio arriva già dopo il video. Ritardare l'audio non corregge questa direzione — se un'altra striscia porta il suono di questa camera, abbassa lì il suo offset.
avsync-over-cap = Lo scarto misurato supera il tetto di { $max } ms. Uno scarto così di solito indica la sorgente sbagliata — controlla la catena e rimisura.
avsync-applied = Applicato — l'offset del microfono ora è { $offset } ms.
avsync-apply = Applica { $offset } ms al microfono
avsync-again = Misura di nuovo
avsync-close = Chiudi
avsync-error-noFlash = La camera non ha mai visto il flash. Puntala sul programma lampeggiante (lo schermo intero aiuta), verifica che la sorgente sia attiva e rimisura.
avsync-error-noBeep = Il microfono non ha mai sentito il bip. Verifica che il dispositivo di monitoraggio sia udibile e che il microfono sia attivo (non bloccato dal push-to-talk), poi rimisura.
avsync-error-tooFewCycles = Cicli flash/bip puliti insufficienti. Tieni il pattern ben visibile e udibile per tutta la misura.
avsync-error-notThePattern = Ciò che si è visto o sentito non si ripete al ritmo del pattern — probabilmente luce o rumore della stanza, non il segnale di prova.
avsync-error-unstable = I cicli sono troppo discordi per fidarsi di un solo numero. Stabilizza la camera, riduci il rumore e rimisura.
hotkey-audit-title = Mappa dei tasti
hotkey-audit-search = Cerca
hotkey-audit-filter = Funzione
hotkey-audit-filter-all = Tutte le funzioni
hotkey-audit-col-key = Tasto
hotkey-audit-col-action = Azione
hotkey-audit-col-where = Dove
hotkey-audit-col-status = Stato
hotkey-audit-ok = OK
hotkey-audit-shared = Condivisa da { $count } assegnazioni
hotkey-audit-unregistered = Non registrata nel sistema (occupata altrove o non disponibile)
hotkey-audit-invalid = Scorciatoia non valida
hotkey-audit-empty = Nessun tasto assegnato — assegnali in Impostazioni → Tasti o su una striscia del mixer.
hotkey-audit-export = Esporta promemoria
hotkey-audit-exported = Salvato in { $path }
hotkey-audit-note = Assegna e cambia i tasti in Impostazioni → Tasti (azioni globali) e su ogni striscia del mixer (push-to-talk / push-to-mute); questa tabella li verifica e documenta.
hotkey-audit-action-record = Attiva/disattiva registrazione
hotkey-audit-action-go-live = Attiva/disattiva streaming
hotkey-audit-action-transition = Esegui transizione
hotkey-audit-action-save-replay = Salva replay
hotkey-audit-action-add-marker = Aggiungi marcatore
hotkey-audit-action-still = Cattura fermo immagine
hotkey-audit-action-panic = Schermo di emergenza
hotkey-audit-action-timer-toggle = Avvia/pausa tutti i timer
hotkey-audit-action-timer-reset = Reset di tutti i timer
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Registrazione
hotkey-audit-feature-streaming = Streaming
hotkey-audit-feature-studio = Modalità studio
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Marcatori
hotkey-audit-feature-stills = Fermi immagine
hotkey-audit-feature-panic = Emergenza
hotkey-audit-feature-timers = Timer
hotkey-audit-feature-audio = Audio (per sorgente)
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
properties-deinterlace = Deinterlacciamento
properties-deinterlace-off = Spento
properties-deinterlace-discard = Scarta (raddoppia le righe di un campo)
properties-deinterlace-bob = Bob (campi alternati)
properties-deinterlace-linear = Lineare (interpola)
properties-deinterlace-blend = Fusione (media dei campi)
properties-deinterlace-adaptive = Adattivo al movimento (classe yadif)
properties-field-order = Ordine dei campi
properties-field-order-top = Prima il campo superiore
properties-field-order-bottom = Prima il campo inferiore
properties-deinterlace-note = Per segnali interlacciati da schede di acquisizione. CPU pura, identico su ogni OS; cambiarlo riavvia il dispositivo (come un cambio di formato).
camera-controls-title = Controlli camera
camera-controls-refresh = Aggiorna
camera-controls-reset = Reimposta profilo
camera-controls-empty = Nessun controllo al momento — il dispositivo deve trasmettere (aggiungilo prima a una scena), e alcuni backend non ne riportano (soprattutto macOS). È lo stato onesto per OS.
camera-controls-note = Le modifiche si applicano dal vivo e si salvano nel profilo del dispositivo, riapplicato a ricollegamento e riavvio.
camera-control-brightness = Luminosità
camera-control-contrast = Contrasto
camera-control-hue = Tonalità
camera-control-saturation = Saturazione
camera-control-sharpness = Nitidezza
camera-control-gamma = Gamma
camera-control-white-balance = Bilanciamento del bianco
camera-control-backlight = Compensazione controluce
camera-control-gain = Guadagno
camera-control-pan = Pan
camera-control-tilt = Tilt
camera-control-zoom = Zoom
camera-control-exposure = Esposizione
camera-control-iris = Iride
camera-control-focus = Messa a fuoco
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
audiofilters-name-parametric-eq = EQ parametrico
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Filtro anti-rumble
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Preset
audiofilters-voice-preset-pick = Preset voce…
audiofilters-voice-broadcast = Voce broadcast
audiofilters-voice-podcast = Voce podcast
audiofilters-voice-clean = Voce pulita
audiofilters-voice-none = Svuota catena
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Frequenza sibilanti (Hz)
audiofilters-deesser-amount = Riduzione max (dB)
audiofilters-rumble-freq = Taglia-bassi (Hz)
audiofilters-title = Filtri audio — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Curva di risposta dell'EQ parametrico con spettro in tempo reale
eq-band-type = Tipo
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Banda
eq-remove-band = Rimuovi banda
eq-type-bell = Campana
eq-type-lowShelf = Shelf basso
eq-type-highShelf = Shelf alto
eq-type-notch = Notch
eq-type-highPass = Passa-alto
eq-type-lowPass = Passa-basso
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
filters-name-perspective = Prospettiva
filters-name-fade-loop = Ciclo di dissolvenza
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
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galleria
filters-shader-gallery-pick = Carica un preset…
filters-shader-gallery-grayscale = Scala di grigi
filters-shader-gallery-invert = Inverti
filters-shader-gallery-scanlines = Scanline
filters-shader-gallery-vignette = Vignettatura
filters-shader-source = Codice shader (WGSL)
filters-shader-hint = Scrivi un effect(uv, color, p, texel, time) in WGSL che restituisce un vec4. Annota i parametri con // @param name min max default per i cursori. Uno shader non valido viene ignorato — la sorgente viene mostrata senza filtro finché non compila.
filters-name-bezier-mask = Maschera Bézier
filters-mask-editor-hint = Trascina un punto per spostarlo, doppio clic per aggiungerne uno, clic destro su un punto per rimuoverlo.
filters-mask-shape = Forma
filters-mask-shape-pick = Preset…
filters-mask-shape-rectangle = Rettangolo
filters-mask-shape-diamond = Rombo
filters-mask-shape-hexagon = Esagono
filters-mask-shape-circle = Cerchio
filters-mask-feather = Sfumatura
filters-mask-export-wipe = Esporta come tendina…
filters-mask-image = Immagine maschera
filters-mask-mode = Modalità
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = inverti
filters-speed-x = Velocità X (px/s)
filters-speed-y = Velocità Y (px/s)
filters-tilt = Inclinazione
filters-far-fade = Dissolvenza del bordo lontano
filters-fade-in-s = Comparsa (s)
filters-visible-s = Visibile (s)
filters-fade-out-s = Scomparsa (s)
filters-hidden-s = Nascosto (s)
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
recordings-trim = Ritaglia
recordings-trim-title = Taglia una clip da questa registrazione — i tagli allineati ai keyframe si esportano senza ricodifica
recordings-verify = Verifica
recordings-verify-title = Controlla l'integrità del file — struttura del contenitore, continuità, interleave A/V, durata
recordings-verifying = Verifica in corso…
verify-dismiss = Chiudi
verify-verdict-pass = { $name } — integrità OK
verify-verdict-warn = { $name } — verificato con avvisi
verify-verdict-fail = { $name } — problemi trovati
verify-container = Contenitore
verify-video-continuity = Continuità video
verify-audio-continuity = Continuità audio
verify-av-interleave = Interleave A/V
verify-duration = Durata
recordings-alpha-label = alfa
recordings-prores-title = Esporta un master .mov ProRes 4444 che preserva l'alfa (per il montaggio)
recordings-qtrle-title = Esporta un .mov QuickTime Animation che preserva l'alfa (compatibilità massima)
trim-title = Ritaglia — { $name }
trim-loading = Lettura del file…
trim-preview-alt = Fotogramma di anteprima
trim-position = Posizione di riproduzione
trim-step-second-back = Indietro di un secondo
trim-step-frame-back = Indietro di un fotogramma
trim-step-frame-forward = Avanti di un fotogramma
trim-step-second-forward = Avanti di un secondo
trim-snap = Keyframe
trim-snap-title = Aggancia al keyframe più vicino — un taglio lì si esporta senza ricodifica
trim-set-in = Punto di ingresso
trim-set-out = Punto di uscita
trim-range-invalid = Il punto di uscita deve venire dopo quello di ingresso.
trim-copy-badge = ✓ Esporta senza ricodifica — il punto di ingresso cade su un keyframe.
trim-reencode-badge = Verrà ricodificato: il punto di ingresso è tra due keyframe (usa "Keyframe" per un taglio senza perdita).
trim-export = Esporta clip
trim-export-916 = 9:16
trim-export-916-title = Esporta verticale riinquadrato (ritaglio centrato alla dimensione del canvas verticale) — ricodifica sempre
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
recordings-normalize = Normalizza
recordings-normalizing = Normalizzazione…
recordings-normalize-title = Normalizza il loudness al target (scrive una copia)
recordings-normalized-to = Normalizzato in { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Solo audio
audiorec-format = Formato di registrazione audio
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Registra audio
audiorec-stop = Interrompi
audiorec-pause = Pausa
audiorec-resume = Riprendi
audiorec-recording = REC { $sec }s
audiorec-saved = Salvati { $count } file di traccia


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
output-recording-template = Nome file delle registrazioni
output-replay-template = Nome file dei replay
output-still-template = Nome file dei fotogrammi
output-template-tokens = Token: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Cartella dei replay
output-still-folder = Cartella dei fotogrammi
output-same-folder-placeholder = Cartella delle registrazioni
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
bench-open = Esegui benchmark encoder…
bench-title = Benchmark encoder
bench-intro = Esegue brevi scale di codifica misurate (ogni encoder rilevato × preset × risoluzione) su questa macchina — circa un minuto, tutto offline, nulla lascia il computer. I fallimenti sono elencati, mai nascosti. Ferma prima stream o registrazioni.
bench-start = Avvia benchmark
bench-rerun = Esegui di nuovo
bench-running = Misurazione… { $done } / { $total }
bench-cancel = Annulla
bench-col-encoder = Encoder
bench-col-preset = Preset
bench-col-rung = Gradino
bench-col-achieved = fps
bench-col-headroom = Margine
bench-failed = fallito
bench-rec-title = Raccomandazione (misurata)
bench-rec-body = { $encoder } con { $preset }, { $width }×{ $height } @ { $fps } fps — misurato { $headroom }× tempo reale. Bitrate stream suggerito: { $bitrate } kbps.
bench-rec-none = Nulla regge il tempo reale su questa macchina — riduci risoluzione del canvas o fps e riprova.
bench-apply = Applica alle impostazioni di registrazione
bench-applied = Applicato ✓
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
output-iso-heading = Registrazione ISO
output-iso-explainer = Registra le sorgenti selezionate pulite, ognuna nel proprio file accanto al programma — prima della composizione, a dimensione e frame rate del canvas, così ogni file cade allineato sulla timeline di montaggio. Due corsie sono comode su una GPU di fascia media; ogni corsia extra costa un altro render e un'altra codifica.
output-iso-none = Ancora nessuna sorgente nella collezione.
output-iso-source-on = "{ $name }" viene registrata nel proprio file ISO — clicca per fermare
output-iso-source-off = Registra "{ $name }" nel proprio file ISO
output-iso-post-filter = Registra con i filtri della sorgente (post-filtro); deselezionato registra la sorgente grezza
output-iso-format = Formato ISO
output-iso-encoder = Encoder video ISO
output-alpha-frec = Registra con trasparenza (alfa) — il programma su sfondo trasparente
output-alpha-title = Il registratore riceve un proprio render trasparente; anteprima e stream restano normali. Esporta in ProRes 4444 o QTRLE dalla lista registrazioni — MP4/MKV appiattiscono l'alfa.
output-split-events = Inizia un nuovo file anche a… (ogni parte inizia esattamente sull'evento; durata minima 1 s)
output-split-on-scene = cambio scena
output-split-on-marker = marcatore
output-split-on-rundown = passo della scaletta
output-auto-markers = Inserisci marcatori di capitolo automaticamente sugli eventi dello studio (cambio scena, salvataggio replay, riconnessione, frame persi, allarmi, regole)
output-auto-markers-title = I marcatori tipizzati finiscono nei capitoli della registrazione (mkv) o nel file .chapters.txt, accanto alla scorciatoia manuale
output-pipeline-heading = Pipeline post-registrazione
output-pipeline-explainer = Al termine di una registrazione, esegue questi passi sul file principale, in ordine, in background. Un set di azioni chiuso — per scelta non esiste il passo «esegui comando». La catena si ferma al primo errore.
output-pipeline-enabled = Esegui la pipeline dopo ogni registrazione
output-pipeline-add = Aggiungi un passo…
output-pipeline-up = Sposta su
output-pipeline-down = Sposta giù
output-pipeline-remove = Rimuovi passo
output-pipeline-template = Modello di rinomina (token CAP-M25)
output-pipeline-folder = Cartella
pipeline-queue = Pipeline post-registrazione
pipeline-verify = Verifica
pipeline-remux = Remux in MP4
pipeline-normalize = Normalizza il volume
pipeline-rename = Rinomina
pipeline-move = Sposta in cartella
pipeline-copy = Copia in cartella
pipeline-reveal = Mostra nel file manager
pipeline-luaEvent = Notifica gli script Lua
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
stream-session-report = Scrivi un report della sessione (HTML + Markdown) accanto alla registrazione al termine
stream-simulator-title = Simulatore di rete (prove)
stream-simulator-note = Modella solo i ricevitori locali della prova — allena riconnessioni e uplink deboli. Un vero Go Live non viene mai degradato.
stream-simulator-profile = Profilo
stream-simulator-off = Spento
stream-simulator-hotel-wifi = Wi-Fi da hotel
stream-simulator-mobile-hotspot = Hotspot mobile
stream-simulator-custom = Personalizzato
stream-simulator-bandwidth = Banda (kbps, 0 = illimitata)
stream-simulator-latency = Latenza (ms)
stream-simulator-jitter = Jitter (± ms)
stream-simulator-outage-every = Interruzione ogni (s, 0 = mai)
stream-simulator-outage-len = Durata interruzione (s)
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
hotkeys-go-live = Vai in diretta / Termina stream
hotkeys-transition = Transizione Modalità Studio
hotkeys-save-replay = Salva replay (ultimi N secondi)
hotkeys-add-marker = Inserisci un marcatore di capitolo (registrazione)
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

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importa da OBS…
workspace-import-obs-hint = Importa una raccolta di scene di OBS (il suo scenes.json). La raccolta attuale viene salvata prima.
workspace-import-busy = Importazione…
workspace-import-title = «{ $name }» importata
workspace-import-summary = { $scenes } scene · { $sources } sorgenti · { $items } elementi
workspace-import-dismiss = Chiudi
workspace-import-clean = Tutto importato correttamente.
workspace-import-geometry-caveat = Dimensioni e posizioni sono adattate dal layout di OBS: controlla ogni scena e riseleziona i dispositivi di cattura.
workspace-import-notes-title = Importato con note
workspace-import-skipped-title = Non importato
import-note-needsReselect = Riseleziona dispositivo/monitor/finestra
import-note-gameCaptureAsWindow = Cattura di gioco → Cattura finestra
import-note-referencesFile = Controlla il percorso del file
import-note-filterDropped = Alcuni filtri non supportati
import-note-geometryApproximated = Posizione/dimensione approssimate
import-skip-unsupportedKind = Nessun tipo di sorgente equivalente
import-skip-group = I gruppi non sono ancora supportati

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Ricollega file mancanti…
doctor-title = File mancanti
doctor-scanning = Scansione…
doctor-all-good = Tutti i file referenziati esistono. Niente da ricollegare.
doctor-intro = { $count } file referenziati non si trovano su questo computer. Indica la nuova posizione di ciascuno: ogni scena che lo usa viene corretta in un colpo solo.
doctor-relinked = { $count } riferimenti ricollegati.
doctor-uses = usato { $count }×
doctor-locate = Individua…
doctor-locate-folder = Cerca nella cartella…
doctor-locate-folder-hint = Scegli una cartella; ogni file mancante viene trovato per nome e ricollegato.
doctor-kind-image = immagine
doctor-kind-media = media
doctor-kind-slideshow = presentazione
doctor-kind-font = carattere
doctor-kind-lut = LUT
doctor-kind-mask = maschera
history-relinkFiles = Ricollega file

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
studio-preview-stinger-matte-label = Traccia matte
studio-preview-stinger-matte-title = Come uno stinger con traccia matte racchiude la trasparenza: il riempimento e la sua matte affiancati (orizzontale) o impilati (verticale)
studio-preview-stinger-duck-label = Abbassa il programma
studio-preview-stinger-duck-title = Abbassa l'audio del programma sotto l'audio dello stinger mentre viene riprodotto (0 = disattivato)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = Spostamento (morph)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Nessuna
stinger-matte-horizontal = Affiancati
stinger-matte-vertical = Impilati

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Personalizzato (RTMP/RTMPS)
stream-service-srt = SRT (auto-ospitato)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Informazioni
about-tagline = Registra e trasmetti come uno studio — nessun account, nessun cloud.
about-version = Versione
about-created-by = Creato da
about-project-started = Progetto avviato
about-first-stable = Prima versione stabile
about-first-stable-pending = Non ancora — la 1.0.0 è in corso
about-platform = Piattaforma
about-local-first = Freally Capture funziona interamente sulla tua macchina. Nessun account, nessuna telemetria, nessun cloud — l'unica cosa che lascia il tuo computer è lo stream che hai scelto di inviare.
about-website = Sito web
about-issues = Segnala un problema
about-license = Licenza
about-eula = EULA
about-third-party = Note sulle terze parti
about-check-updates = Controlla aggiornamenti…

# --- unified settings modal (TASK-906) ---
settings-title = Impostazioni
settings-language-section = Lingua
settings-language = Lingua dell'interfaccia
settings-language-system = Predefinita di sistema
settings-language-note = La lingua che scegli qui viene ricordata. "Predefinita di sistema" segue il tuo sistema operativo. Il testo non tradotto ricade sull'inglese.
settings-appearance-section = Aspetto
settings-theme = Tema
settings-theme-dark = Scuro
settings-theme-light = Chiaro
settings-theme-custom = Personalizzato
settings-accent = Accento
settings-general-section = Generale
settings-show-stats-dock = Mostra il pannello statistiche
settings-open-about = Informazioni…

# --- command palette (TASK-904) ---
palette-title = Riquadro comandi
palette-search = Cerca scene, sorgenti e azioni
palette-placeholder = Cerca scene, sorgenti, azioni…
palette-no-results = Nessun risultato per “{ $query }”
palette-hint = ↑ ↓ per spostarti · Enter per eseguire · Esc per chiudere
palette-group-scenes = Scena
palette-group-sources = Sorgente
palette-group-actions = Azione
palette-transition = Transizione Anteprima → Programma
palette-save-replay = Salva replay
palette-add-marker = Inserisci un marcatore di capitolo
palette-vertical-canvas = Canvas verticale (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Ti diamo il benvenuto in Freally Capture
wizard-welcome = Due passaggi veloci: vediamo cosa può fare il tuo computer, poi iniziamo una scena. Ci vogliono circa trenta secondi e potrai cambiare tutto in seguito.
wizard-local-first = Niente di tutto questo lascia il tuo computer. Freally Capture non ha account, né telemetria, né cloud.
wizard-start = Iniziamo
wizard-skip = Salta
wizard-hardware-title = Cosa può fare il tuo computer
wizard-probing = Controlliamo la scheda grafica e il processore…
wizard-encoder = Encoder
wizard-canvas = Canvas
wizard-bitrate = Bitrate
wizard-probe-found = Trovato: { $gpus } · { $cores } core fisici
wizard-no-gpu = nessuna GPU dedicata
wizard-apply = Usa queste impostazioni
wizard-keep-current = Mantieni le mie impostazioni
wizard-template-title = Inizia con una scena
wizard-template-screen = Cattura il mio schermo
wizard-template-screen-note = Aggiunge una Cattura schermo del tuo monitor principale. Il punto di partenza più comune.
wizard-template-empty = Inizia da vuoto
wizard-template-empty-note = Una scena vuota. Aggiungi tu le sorgenti con il pulsante +.
wizard-done = È tutto pronto.
wizard-done-hint = Premi Ctrl+K in qualsiasi momento per cercare scene, sorgenti e azioni. Le impostazioni si trovano dietro il pulsante ⚙.
wizard-close = Inizia a trasmettere

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = La tua scheda grafica può codificare il video da sola, lasciando il processore libero per il resto dello studio.
autoconfig-reason-software = Non è stato trovato un encoder hardware utilizzabile, quindi codificherà il processore. Funziona, costa solo più CPU.
autoconfig-reason-quality-hardware = 1080p a 60 fotogrammi al secondo, a un bitrate accettato da ogni grande piattaforma.
autoconfig-reason-quality-software = 30 fotogrammi al secondo, perché la codifica software a 60 perde fotogrammi sulla maggior parte dei processori.
autoconfig-reason-quality-low-cores = Un bitrate più basso, perché questo processore ha pochi core e la codifica software se li contende con il compositor.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Registrazione avviata
announce-recording-paused = Registrazione in pausa
announce-recording-stopped = Registrazione interrotta
announce-live-started = Sei in diretta
announce-live-ended = Diretta terminata
announce-reconnecting = Connessione persa, riconnessione in corso
announce-stream-failed = Diretta non riuscita
announce-frames-dropped = Fotogrammi persi: { $count }

# CAP-M01 — undo/redo edit history
palette-undo = Annulla
palette-redo = Ripeti
palette-edit-history = Cronologia modifiche…
history-title = Cronologia modifiche
history-empty = Nulla da annullare per ora.
history-current = Stato attuale
history-close = Chiudi
history-addScene = Aggiungi scena
history-renameScene = Rinomina scena
history-removeScene = Rimuovi scena
history-reorderScene = Riordina scene
history-addSource = Aggiungi sorgente
history-removeSource = Rimuovi sorgente
history-reorderSource = Riordina sorgenti
history-renameSource = Rinomina sorgente
history-transformSource = Sposta sorgente
history-toggleVisibility = Attiva/disattiva visibilità
history-toggleOutputVisibility = Attiva/disattiva visibilità per uscita
history-toggleLock = Attiva/disattiva blocco
history-setBlendMode = Cambia modalità di fusione
history-editSourceProperties = Modifica proprietà
history-applyLayout = Disponi layout
history-moveToSeat = Sposta nel posto
history-groupSources = Raggruppa sorgenti
history-ungroupSources = Separa sorgenti
history-toggleGroupVisibility = Attiva/disattiva gruppo
history-setSceneAudio = Audio scena
history-setVerticalCanvas = Tela verticale
history-addFilter = Aggiungi filtro
history-removeFilter = Rimuovi filtro
history-reorderFilter = Riordina filtri
history-editFilter = Modifica filtro
history-toggleFilter = Attiva/disattiva filtro
history-setVolume = Regola volume
history-toggleMute = Attiva/disattiva muto
history-setMonitor = Cambia monitoraggio
history-setTracks = Cambia tracce
history-setSyncOffset = Regola sync A/V
history-setAudioHotkeys = Scorciatoie audio

# CAP-M04 — alignment aids
settings-alignment-section = Aiuti di allineamento
settings-smart-guides = Guide intelligenti (aggancio durante il trascinamento)
settings-safe-areas = Overlay area sicura
settings-rulers = Righelli
align-group = Allinea alla tela
align-left = Allinea a sinistra
align-hcenter = Centra orizzontalmente
align-right = Allinea a destra
align-top = Allinea in alto
align-vcenter = Centra verticalmente
align-bottom = Allinea in basso

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Allinea e distribuisci selezione
arrange-left = Allinea bordi sinistri
arrange-hcenter = Centra orizzontalmente
arrange-right = Allinea bordi destri
arrange-top = Allinea bordi superiori
arrange-vcenter = Centra verticalmente
arrange-bottom = Allinea bordi inferiori
distribute-h = Distribuisci orizzontalmente
distribute-v = Distribuisci verticalmente
guides-group = Guide
guides-add-v = Aggiungi guida verticale
guides-add-h = Aggiungi guida orizzontale
guides-clear = Rimuovi tutte le guide
history-arrangeItems = Disponi elementi
history-editGuides = Modifica guide

# CAP-M05 — edit transform + copy/paste
transform-title = Modifica trasformazione — { $name }
transform-anchor = Ancoraggio
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotazione
transform-crop = Ritaglio
transform-crop-left = Sinistra
transform-crop-top = Alto
transform-crop-right = Destra
transform-crop-bottom = Basso
transform-no-size = Dimensioni e ritaglio saranno disponibili quando la sorgente comunicherà le sue dimensioni.
transform-copy = Copia trasformazione
transform-paste = Incolla trasformazione
transform-close = Chiudi
filters-copy = Copia filtri ({ $count })
filters-paste = Incolla filtri ({ $count })
palette-edit-transform = Modifica trasformazione…
history-pasteFilters = Incolla filtri

# CAP-M26 — keying workbench
workbench-title = Banco di keying — { $name }
workbench-mode-keyed = Con chiave
workbench-mode-source = Sorgente
workbench-mode-matte = Matte
workbench-mode-split = Diviso
workbench-eyedropper = Contagocce
workbench-eyedropper-hint = Fai clic sulla sorgente per campionare il colore chiave.
workbench-loupe = Lente
workbench-split = Divisione
workbench-preview-alt = Anteprima del banco di keying
workbench-tune = Regola
workbench-close = Chiudi

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Fai clic su una scena per passarci.
multiview-hint-stage = Fai clic su una scena per prepararla in anteprima.
palette-multiview = Monitor multiview

# CAP-M07 — projectors
projector-title = Apri proiettore
projector-source = Sorgente
projector-target-program = Programma
projector-target-preview = Anteprima
projector-target-scene = Scena…
projector-target-source = Sorgente…
projector-target-multiview = Multiview
projector-which-scene = Quale scena
projector-which-source = Quale sorgente
projector-none = Niente da mostrare
projector-display = Schermo
projector-windowed = Finestra mobile (questo schermo)
projector-display-option = Schermo { $n } — { $w }×{ $h }
projector-primary = (principale)
projector-open = Apri
projector-cancel = Annulla
projector-exit-hint = Premi Esc per uscire
palette-projector = Apri proiettore…

# CAP-M08 — still-frame grab
palette-still = Cattura fotogramma…
still-saved-toast = Fotogramma salvato: { $name }
still-failed-toast = Cattura fotogramma non riuscita: { $error }
hotkeys-still = Cattura fotogramma

# CAP-M13 — source health dashboard
palette-source-health = Salute delle sorgenti…
palette-av-sync = Calibrazione sincronia A/V…
palette-hotkey-audit = Mappa dei tasti…
health-title = Salute delle sorgenti
health-col-source = Sorgente
health-col-state = Stato
health-col-resolution = Risoluzione
health-col-fps = FPS
health-col-last-frame = Ultimo fotogramma
health-col-dropped = Scartati
health-col-retries = Riavvii
health-col-actions = Azioni
health-state-live = In diretta
health-state-waiting = In attesa
health-state-error = Errore
health-state-inactive = Inattiva
health-restart = Riavvia
health-properties = Proprietà
health-empty = Questa raccolta non ha ancora sorgenti.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Uscire da Freally Capture?
quit-body = Uscendo ora verrà eseguito in sicurezza, nell'ordine:
quit-consequence-stream = Terminare la diretta e disconnettersi dal servizio.
quit-consequence-recording = Fermare la registrazione e finalizzarne i file.
quit-consequence-replay = Spegnere il buffer di replay — il materiale non salvato viene scartato.
quit-confirm = Esci in sicurezza
quit-quitting = Chiusura…
quit-cancel = Annulla

# CAP-M11 — crash-safe recording salvage
salvage-title = Recuperare le registrazioni interrotte?
salvage-body = L'ultima sessione è terminata in modo imprevisto mentre queste registrazioni erano ancora in scrittura. La riparazione crea una copia riproducibile accanto all'originale — il file originale non viene mai modificato.
salvage-repair = Ripara
salvage-repairing = Riparazione…
salvage-done = Riparato
salvage-repaired = Riparato → { $name }
salvage-failed = Riparazione non riuscita: { $error }
salvage-dismiss = Non ora

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Guasto dell'encoder — passato da { $from } a { $to }. La diretta si è riconnessa e continua.
fallback-toast-recording = Guasto dell'encoder — passato da { $from } a { $to }. La registrazione continua in un nuovo file.
fallback-note = Encoder di riserva: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = L'audio del programma è diventato muto
alarm-clipping = L'audio del programma sta saturando
alarm-black = L'immagine del programma è nera
alarm-frozen = L'immagine del programma non cambia da un po'
alarm-lowDisk = Spazio su disco: circa { $minutes } min rimasti al bitrate attuale
alarm-dismiss = Ignora allarme
alarm-cleared = Risolto: { $alarm }

# CAP-M22 — panic button
palette-panic = Panico — taglia sulla schermata privacy
panic-banner-title = Panico
panic-banner-body = Il programma mostra la schermata privacy; tutto l'audio è muto e le catture sono ferme. Diretta e registrazione restano attive.
panic-restore = Ripristina…
panic-restore-confirm = Ripristinare il programma?
panic-restore-yes = Ripristina
panic-restore-cancel = Annulla
hotkeys-panic = Panico (schermata privacy)
hotkeys-timer-toggle = Avvia/pausa tutti i timer
hotkeys-timer-reset = Reset di tutti i timer
panic-slate-color = Colore della schermata di panico
panic-slate-image = Immagine della schermata di panico
panic-slate-image-placeholder = Percorso immagine facoltativo

# CAP-M24 — redacted diagnostics bundle
diag-title = Pacchetto diagnostico
diag-intro = Esporta uno .zip espurgato (istantanea della configurazione, sonda degli encoder, statistiche recenti — mai segreti, percorsi o nomi) da allegare a mano a un issue GitHub. Nulla viene inviato.
diag-preview = Vedi contenuto
diag-hide-preview = Nascondi anteprima
diag-export = Esporta .zip
diag-exported = Esportato: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Controllo pre-diretta
preflight-intro = Ogni punto bloccante deve essere verde; il resto sono promemoria onesti.
preflight-item-targets = Destinazioni configurate (chiave/URL)
preflight-item-encoder = È disponibile un encoder utilizzabile
preflight-item-sources = Tutte le sorgenti sane
preflight-item-disk = Spazio su disco per la registrazione
preflight-item-mic = Livello del microfono
preflight-item-desktopAudio = Livello dell'audio desktop
preflight-item-replay = Buffer di replay armato
preflight-targets-detail = { $count } abilitati
preflight-sources-detail = { $count } sorgente/i in errore
preflight-disk-detail = ~{ $minutes } min al bitrate attuale
preflight-fix-stream = Impostazioni stream…
preflight-fix-components = Componenti…
preflight-fix-sources = Salute delle sorgenti…
preflight-fix-replay = Arma
preflight-optional = facoltativo
preflight-hold = Blocca la diretta finché tutto non è verde
preflight-cancel = Annulla
preflight-go-anyway = Vai in diretta comunque
preflight-go-live = Vai in diretta


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Sfondo
scenes-backdrop-aria = Sfondo di { $name }
backdrop-title = Sfondo — { $name }
backdrop-hint = Uno sfondo fissato dietro tutto in questa scena: un’immagine, una GIF animata o un video in loop. La tua cattura resta sempre sopra; scorri sul canvas per lo zoom.
backdrop-choose = Scegli immagine o video…
backdrop-remove = Rimuovi lo sfondo
backdrop-none = Nessuno sfondo.
backdrop-position = Posizione
backdrop-split-full = Canvas intero
backdrop-split-left = Metà sinistra
backdrop-split-right = Metà destra
backdrop-split-top = Metà superiore
backdrop-split-bottom = Metà inferiore
backdrop-sync = Avvia la riproduzione all’inizio della registrazione
backdrop-sync-hint = Resta sul primo fotogramma finché non registri; ogni ripresa fa ripartire il video dall’inizio.
backdrop-preview-play = Anteprima riproduzione
backdrop-preview-pause = Pausa anteprima
backdrop-filter-all = Sfondi (immagini e video)
backdrop-filter-images = Immagini
backdrop-filter-media = Video e GIF
sources-backdrop-badge = Sfondo (fissato in fondo)
sources-backdrop-pinned = Lo sfondo resta fissato in fondo
filters-name-flip = Rifletti
filters-flip-horizontal = Orizzontale
filters-flip-vertical = Verticale
history-setSceneBackdrop = Imposta sfondo
history-setBackdropSplit = Sposta sfondo
history-setBackdropSync = Sincronizzazione sfondo-registrazione
backdrop-scrub = Posizione di riproduzione
backdrop-loop = Loop
backdrop-reverse = Riproduci al contrario
backdrop-reverse-hint = L’inversione genera una copia al contrario una sola volta (i video richiedono il componente ffmpeg; le GIF si invertono all’istante) — il primo passaggio può richiedere tempo sui file lunghi.
filters-scaling = Scalatura
filters-scaling-hint = Modalità pixel-perfect per contenuti retro/pixel; Intero aggancia anche la dimensione disegnata a multipli esatti (le maniglie mostrano la dimensione logica).
filters-scaling-auto = Morbida
filters-scaling-nearest = Nearest neighbor
filters-scaling-integer = Intera (× esatti)
filters-scaling-sharp = Bilineare nitida
history-setScaling = Cambia scalatura
hotkeys-zoom-100 = Zoom: ripristina (100%)
hotkeys-zoom-150 = Zoom: avvicina al 150%
hotkeys-zoom-200 = Zoom: avvicina 2×
sources-follow-title = Segui il cursore durante lo zoom (Windows; scorri sul canvas per zoomare)
sources-follow-item = Attiva/disattiva il segui-cursore di { $name }
filters-autocrop = ✂ Ritaglia bande nere
filters-autocrop-title = Analizza il prossimo fotogramma alla ricerca di bande nere e le ritaglia (annullabile). Le scene scure non vengono mai ritagliate.
filters-autocrop-follow = Ricontrolla al cambio di risoluzione
history-autoCrop = Ritaglio automatico bande nere
sources-link-audio = Cattura anche l’audio di questa app (collegato: nascondere silenzia, rimuovere la finestra lo rimuove)
history-addLinkedWindow = Aggiungi finestra + audio collegato
sources-hdr-title = Questo display è HDR — apri il tone mapping (il canvas resta SDR)
sources-hdr-item = Tone mapping HDR di { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Questo display emette HDR. Senza tone mapping le alte luci si tagliano e la cattura appare slavata sul canvas SDR. Le modifiche si applicano al fotogramma successivo.
sources-hdr-enable-suggested = Attiva suggerito (maxRGB, 200 nit)
sources-hdr-operator = Operatore
sources-hdr-op-clip = Clip (disattivato)
sources-hdr-op-maxrgb = maxRGB (tinta preservata)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = Ginocchio BT.2408 (SDR esatto)
sources-hdr-paper-white = Bianco carta
sources-hdr-nits = nit
projector-target-passthrough = Monitor passthrough (bassa latenza)
projector-which-device = Dispositivo
projector-passthrough-none = Aggiungi prima un display, una finestra o un dispositivo di acquisizione.
projector-passthrough-about = Fotogrammi grezzi del dispositivo: niente scene, niente filtri, niente compositor. Mostra una latenza misurata; l’audio si monitora ancora dal canale del mixer.
projector-passthrough-hint = Passthrough — Esc chiude
projector-latency = { $ms } ms
projector-latency-measuring = misurazione…
automation-title = Automazione — regole, macro e variabili
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Regole
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Attiva
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Quando
automation-then-run = allora esegui
automation-no-macro = (no macro)
automation-macros = Macro
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Esegui
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Variabili dello studio
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
rundown-title = Scaletta dello show
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Avvia
rundown-next = Avanti ▸
rundown-stop = Ferma
rundown-idle = Non in esecuzione
rundown-next-up = Prossimo: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Passo
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
automation-layer = Livello
automation-layer-hint = Si attiva solo con questo livello attivo (vuoto = tutti). I livelli sono persistenti: un tasto livello cambia e resta (l’API globale del SO non offre livelli a pressione mantenuta).
automation-chord-hint = Un tasto semplice (Ctrl+Shift+M) o un accordo a due battute (Ctrl+K, 3). Il secondo tasto è riservato solo mentre l’accordo è in attesa.
panel-title = Pannello LAN e tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Avvia il pannello
panel-port = Porta
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Password
panel-show = Mostra
panel-hide = Nascondi
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Salva
osc-title = Superficie di controllo OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Ascolta OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = Telecamere PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Telecamera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Indirizzo
ptz-port = Porta
ptz-speed = Velocità
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
ptz-presets = Preset
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Richiama
ptz-store = Memorizza
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = Superficie di controllo MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Ingresso
midi-output = Uscita (feedback)
midi-none = (none)
midi-learn = Apprendi
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Azione
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
panel-lan-warning = ⚠ Il traffico LAN non è cifrato — la password viaggia nell’URL via HTTP. Usalo solo su una rete affidabile.
osc-lan-warning = ⚠ OSC non ha password — qualsiasi dispositivo in rete può inviare questi comandi. Usa LAN solo su una rete affidabile.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stats
sources-add-system-stats = Statistiche di prestazione (HUD)
sources-stats-title = Aggiungi un HUD delle prestazioni
sources-stats-note = Mostra nel programma i numeri reali misurati dello studio per i tuoi spettatori: fps, CPU, memoria, tempo di render, fotogrammi persi e bitrate in diretta. Le righe da mostrare, la dimensione e il colore sono nelle Proprietà della sorgente. L'uso della GPU non è mostrato perché non viene misurato.
sources-stats-add = Aggiungi HUD statistiche
properties-stats-show-fps = Mostra FPS
properties-stats-show-cpu = Mostra CPU
properties-stats-show-memory = Mostra memoria
properties-stats-show-render = Mostra tempo di render
properties-stats-show-dropped = Mostra fotogrammi persi
properties-stats-show-bitrate = Mostra bitrate
properties-stats-show-timecode = Mostra timecode (LTC)
properties-stats-size = Dimensione (px)
properties-stats-note = L'HUD disegna etichette compatte universali (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) direttamente nel programma; senza streaming la riga del bitrate mostra «—».

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Visualizzatore
sources-add-visualizer = Visualizzatore audio
sources-visualizer-title = Aggiungi un visualizzatore audio
sources-visualizer-style-label = Stile
sources-visualizer-style-bars = Barre di spettro
sources-visualizer-style-scope = Oscilloscopio
sources-visualizer-style-vu = VU meter
sources-visualizer-target-label = Ascolta
sources-visualizer-target-master = Mix master
sources-visualizer-target-track = Traccia { $n }
sources-visualizer-note = Disegna il segnale che viene davvero mixato (post-fader): una sorgente silenziata resta piatta, esattamente come suona. Dimensione, colore, numero di barre e velocità di caduta sono nelle Proprietà della sorgente.
sources-visualizer-add = Aggiungi visualizzatore
properties-vis-bands = Barre
properties-vis-decay = Velocità di caduta (dB/s)
properties-vis-peak-hold = Indicatori di picco
properties-vis-missing-source = (sorgente mancante)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splits
sources-add-split-timer = Timer split (speedrun)
sources-splits-title = Aggiungi un timer split
sources-splits-file-label = File .lss di LiveSplit
sources-splits-comparison-label = Confronta con
sources-splits-comparison-pb = Record personale
sources-splits-comparison-best = Segmenti migliori
sources-splits-comparison-average = Media
sources-splits-note = Il file è importato in sola lettura — non vi viene mai scritto nulla. Assegna i tasti globali Split / Undo / Skip / Reset in Impostazioni → Scorciatoie. Gli auto-splitter via memoria di processo non sono supportati di proposito.
sources-splits-add = Aggiungi timer split
properties-splits-size = Dimensione (px)
properties-splits-ahead = In vantaggio
properties-splits-behind = In ritardo
properties-splits-gold = Oro
properties-splits-split = Split
properties-splits-undo = Annulla
properties-splits-skip = Salta
properties-splits-reset = Azzera
properties-splits-note = I pulsanti guidano il timer dal vivo (le scorciatoie globali fanno lo stesso da qualsiasi app). La run non viene mai scritta nel file .lss.
hotkeys-split-split = Timer split: avvia / split
hotkeys-split-undo = Timer split: annulla split
hotkeys-split-skip = Timer split: salta segmento
hotkeys-split-reset = Timer split: azzera
hotkey-audit-action-split-split = Split (timer split)
hotkey-audit-action-split-undo = Annulla split
hotkey-audit-action-split-skip = Salta segmento
hotkey-audit-action-split-reset = Azzera timer split
hotkey-audit-feature-split-timer = Timer split

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlist
sources-add-playlist = Playlist multimediale (senza stacchi)
sources-playlist-title = Aggiungi una playlist multimediale
sources-playlist-files-label = File (uno per riga, riprodotti dall'alto in basso)
sources-playlist-browse = Sfoglia…
sources-playlist-loop = Loop
sources-playlist-shuffle = Casuale (un'estrazione per avvio; in loop ripete quell'ordine)
sources-playlist-hold-last = Mantieni l'ultimo fotogramma alla fine
sources-playlist-note = Riproduce l'intera lista rifilata senza stacchi tramite il componente ffmpeg etichettato (solo formati wire — .frec e immagini via Media/Presentazione). Gli elementi sono tutti video o tutti audio, mai misti. Rifilature, cue e la variabile «now playing» sono nelle Proprietà.
sources-playlist-add = Aggiungi playlist
properties-playlist-items = Elementi (dall'alto in basso)
properties-playlist-up = Sposta su
properties-playlist-down = Sposta giù
properties-playlist-remove = Rimuovi elemento
properties-playlist-in = Da (s)
properties-playlist-out = A (s)
properties-playlist-cues = Cue (s, separati da virgole)
properties-playlist-add-item = + Aggiungi elemento
properties-playlist-loop = Loop
properties-playlist-shuffle = Casuale
properties-playlist-hold-last = Mantieni ultimo fotogramma
properties-playlist-hw = Decodifica hardware
properties-playlist-variable = Variabile «now playing» (vuota = off)
properties-playlist-previous = ⏮ Precedente
properties-playlist-next = ⏭ Successivo
properties-playlist-note = I pulsanti cue e Successivo/Precedente guidano la playlist DAL VIVO; le modifiche agli elementi valgono con Applica (la playlist riparte). Metti {"{{"}yourVariable{"}}"} in una sorgente Testo per mostrare l'elemento in riproduzione.
hotkeys-playlist-next = Playlist: elemento successivo
hotkeys-playlist-previous = Playlist: elemento precedente
hotkey-audit-action-playlist-next = Playlist successivo
hotkey-audit-action-playlist-previous = Playlist precedente
hotkey-audit-feature-playlist = Playlist

# Instant replay source (CAP-N10)
sources-badge-replay = Replay
sources-add-replay = Replay istantaneo
sources-replay-title = Aggiungi un replay istantaneo
sources-replay-seconds-label = Durata del roll (secondi)
sources-replay-speed-label = Velocità
sources-replay-speed-full = 100% (con audio)
sources-replay-speed-half = Slow motion 50% (muto)
sources-replay-speed-quarter = Slow motion 25% (muto)
sources-replay-note = Resta trasparente finché non lanci il replay. Arma il buffer replay (Controlli) e assegna il tasto Roll — un roll cattura gli ultimi istanti del buffer, li riproduce nel programma e torna trasparente.
sources-replay-add = Aggiungi replay
properties-replay-roll = ⏵ Lancia replay
properties-replay-note = Roll cattura il buffer ARMATO in una clip e la riproduce alla velocità scelta — ritemporizzata, mai interpolata. Lo slow motion è muto di proposito. Scrub e pausa funzionano durante la riproduzione; alla fine la sorgente torna trasparente.
hotkeys-replay-roll = Replay istantaneo: lancia
hotkey-audit-action-replay-roll = Lancia replay istantaneo

# Input overlay source (CAP-N13)
sources-badge-input = Input
sources-add-input-overlay = Overlay input (tasti/pad)
sources-input-title = Aggiungi un overlay di input
sources-input-layout-label = Layout
sources-input-layout-wasd = WASD + mouse
sources-input-layout-keyboard = Tastiera compatta + mouse
sources-input-layout-gamepad = Gamepad (due stick)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Tasti
sources-input-accent-label = Premuto
sources-input-privacy-note = Privacy: l'input viene letto solo mentre questa sorgente è dal vivo in una scena, e vengono interrogati solo i tasti fissi del layout — una lettura istantanea «è premuto adesso?», mai un hook. Nulla viene registrato, salvato o inviato da nessuna parte; il testo digitato non viene mai catturato.
sources-input-os-note = Lo stato di tastiera e mouse oggi si legge solo su Windows — gli altri sistemi disegnano i tasti non premuti (detto onestamente, mai finto). I gamepad funzionano ovunque tramite la libreria gilrs; viene disegnato il primo controller collegato e, senza controller, il layout resta non premuto.
sources-input-add = Aggiungi overlay input

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Effetti del cursore
filters-cursorfx-hint = Su Windows (che disegna il cursore da sé) vengono dipinti direttamente nella cattura e compaiono quindi in registrazioni e streaming. macOS e Linux compongono il cursore a livello di sistema: questi effetti sono perciò solo per Windows. Le modifiche si applicano subito.
filters-cursorfx-halo = Alone del cursore
filters-cursorfx-halo-color = Colore
filters-cursorfx-halo-radius = Raggio (px)
filters-cursorfx-ripples = Onde di clic
filters-cursorfx-left-color = Clic sinistro
filters-cursorfx-right-color = Clic destro
filters-cursorfx-keystrokes = Tasti in sovrimpressione
filters-cursorfx-keystrokes-hint = Mostra un insieme fisso di tasti (lettere, cifre, modificatori, frecce) accanto al cursore finché restano premuti. I tasti vengono letti solo mentre l'opzione è attiva, disegnati direttamente nel fotogramma e mai salvati né registrati nei log.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Titolo
sources-add-title = Titolo / Tabellone
sources-title-title = Aggiungi un titolo
sources-title-template-label = Parti da
sources-title-template-lower-third = Terzo inferiore (barra + nome + sottotitolo)
sources-title-template-scoreboard = Tabellone (targa + 4 celle)
sources-title-template-blank = Tela vuota
sources-title-width-label = Larghezza tela
sources-title-height-label = Altezza tela
sources-title-template-name = Nome
sources-title-template-subtitle = Titolo
sources-title-template-home = CASA
sources-title-template-away = OSPITI
sources-title-note = Titoli a livelli (testo / immagine / riquadro) con animazione di entrata/uscita, composti in locale — nessuna sorgente browser. Livelli, collegamenti a file e {"{{"}variabili{"}}"} e i controlli dal vivo sono nelle Proprietà della sorgente.
sources-title-add = Aggiungi titolo
properties-title-layers = Livelli (disegnati in ordine — le righe successive stanno sopra)
properties-title-kind-text = Testo
properties-title-kind-image = Immagine
properties-title-kind-rect = Riquadro
properties-title-x = X
properties-title-y = Y
properties-title-outline = Contorno (px)
properties-title-outline-color = Contorno
properties-title-shadow = Ombra
properties-title-animation = Animazione entrata/uscita
properties-title-anim-none = Nessuna (stacco)
properties-title-anim-fade = Dissolvenza
properties-title-anim-slide-left = Scorri a sinistra
properties-title-anim-slide-up = Scorri in alto
properties-title-anim-wipe = Tendina
properties-title-duration = Durata (ms)
properties-title-fire-in = ▶ Lancia entrata
properties-title-fire-out = ◼ Lancia uscita
properties-title-set-live = Metti in onda
properties-title-set-live-note = Spinge subito questo testo nel titolo IN ONDA — senza Applica, senza riavvio
properties-title-up = Sposta livello su
properties-title-down = Sposta livello giù
properties-title-remove = Rimuovi livello
properties-title-add-text = + Testo
properties-title-add-image = + Immagine
properties-title-add-rect = + Riquadro
properties-title-note = Lancia entrata/uscita e «Metti in onda» comandano il titolo IN ESECUZIONE; le modifiche ai livelli valgono con Applica (il titolo si riavvia e rientra). Le celle di testo possono collegarsi a un file osservato (cella CSV / valore JSON / file intero) e interpolare {"{{"}variabili{"}}"} — «Metti in onda» vince su entrambi.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = Ingest LAN (listener SRT/RTMP)
sources-lan-title = Aggiungi un listener di ingest LAN
sources-lan-protocol-label = Protocollo
sources-lan-protocol-srt = SRT (cifrabile — consigliato)
sources-lan-protocol-rtmp = RTMP (nessuna autenticazione)
sources-lan-port-label = Porta (1024–65535)
sources-lan-passphrase-label = Passphrase (vuota = aperto)
sources-lan-passphrase-hint = Le passphrase SRT sono di 10–79 caratteri; il mittente deve usare la stessa.
sources-lan-open-warning = Nessuna passphrase: chiunque su questa rete può alimentare questa sorgente, in chiaro. Impostane una a meno che la rete non sia solo tua.
sources-lan-rtmp-warning = RTMP non ha autenticazione — chiunque su questa rete può inviare a questa porta. Preferisci SRT con passphrase.
sources-lan-url-label = Punta l'app del mittente a
sources-lan-qr-aria = Codice QR dell'URL di ingest
sources-lan-note = Solo LAN: ascolta sull'indirizzo locale di questa macchina, solo finché la sorgente esiste, e non tocca mai internet — nulla lascia la macchina finché un mittente della tua rete non invia per primo. La decodifica passa dal componente ffmpeg chiaramente etichettato. Il canvas mostra questo URL finché un mittente non si collega.
sources-lan-add = Inizia ad ascoltare
properties-lan-note = Applicare una modifica di protocollo, porta o passphrase riavvia il listener — il mittente deve ricollegarsi. Lo stream viene adattato a un canvas 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Link
sources-add-freally-link = Freally Link (altra istanza)
sources-link-title = Aggiungi un Freally Link
sources-link-about = Riceve il programma di un'altra istanza di Freally Capture — video e audio master — sulla tua rete. Attiva prima «Uscita Freally Link» sull'istanza mittente. v1 trasmette motion-JPEG su TCP: perfetto su LAN cablata o buon Wi-Fi, onesto sulla banda con collegamenti deboli.
sources-link-scan = Scansiona la LAN
sources-link-scanning = Scansione…
sources-link-none = Nessuna uscita Freally Link trovata. Attiva «Uscita Freally Link» sull'altra istanza (Controlli → Pannello LAN) oppure digita il suo indirizzo qui sotto.
sources-link-host = Indirizzo
sources-link-port = Porta
sources-link-key = Chiave di associazione
sources-link-key-hint = La chiave nelle impostazioni «Uscita Freally Link» del mittente: senza, il mittente non serve un solo fotogramma.
sources-link-add = Aggiungi link
properties-link-note = Senza connessione la sorgente mostra una schermata «connessione» e riprova da sola con attese crescenti — non si blocca mai su un fotogramma vecchio. Un ricevitore per mittente; un mittente occupato viene ritentato con cortesia.
link-title = Uscita Freally Link
link-about = Condividi il programma di questa istanza — video e audio master — con UNA sola altra istanza di Freally Capture sulla tua rete; lì appare come sorgente «Freally Link» (streaming a due PC, monitor di servizio). Disattivata per impostazione predefinita; nulla si annuncia o resta in ascolto finché non la attivi. v1 trasmette motion-JPEG + audio non compresso su TCP — pensata per LAN cablata o buon Wi-Fi, mai per Internet.
link-enable = Condividi il programma sulla mia rete
link-name = Nome dell'istanza
link-key = Chiave di associazione
link-key-hint = Almeno 8 caratteri: i ricevitori devono inserire questa chiave prima che venga servito un solo fotogramma.
link-lan-warning = ⚠ I ricevitori devono presentare la chiave di associazione prima di ricevere qualsiasi cosa, ma il flusso in sé non è cifrato in v1: usalo solo su una rete fidata.
link-serving = I ricevitori trovano questa istanza con «Scansiona la LAN» o la aggiungono manualmente a:
link-off-hint = Attiva la condivisione per aprire la porta e annunciare questa istanza alle scansioni LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Menu dell'applicazione
menu-file = File
menu-edit = Modifica
menu-view = Visualizza
menu-docks = Dock
menu-profile = Profilo
menu-collection = Collezione di scene
menu-tools = Strumenti
menu-help = Aiuto
menu-rename = Rinomina
menu-remove = Rimuovi
menu-import = Importa
menu-export = Esporta
menu-file-show-recordings = Mostra registrazioni
menu-file-remux = Remux in MP4…
menu-file-settings = Impostazioni…
menu-file-show-settings-folder = Mostra cartella impostazioni
menu-file-exit = Esci
menu-edit-undo = Annulla
menu-edit-redo = Ripeti
menu-edit-history = Cronologia modifiche…
menu-edit-copy-transform = Copia trasformazione
menu-edit-paste-transform = Incolla trasformazione
menu-edit-copy-filters = Copia filtri
menu-edit-paste-filters = Incolla filtri
menu-edit-transform = Trasformazione…
menu-edit-lock-preview = Blocca anteprima
menu-view-fullscreen = Interfaccia a schermo intero
menu-stats-dock = Pannello statistiche
menu-view-multiview = Monitor multiview…
menu-view-projectors = Proiettori…
menu-view-source-health = Salute delle sorgenti…
menu-view-still = Cattura fotogramma
menu-docks-browser = Dock del browser…
menu-docks-lock = Blocca i dock
menu-docks-reset = Ripristina layout dei dock
menu-profile-manage = Gestisci profili…
menu-collection-manage = Gestisci collezioni di scene…
menu-collection-import-obs = Importa da OBS…
menu-collection-missing = Controlla file mancanti…
menu-tools-wizard = Esegui la configurazione guidata
menu-tools-wizard-title = La configurazione guidata parte al primo avvio; non è ancora possibile rieseguirla.
menu-tools-automation = Regole di automazione e macro…
menu-tools-rundown = Mostra scaletta…
menu-tools-hotkeys = Mappa dei tasti…
menu-tools-av-sync = Calibrazione sincronia A/V…
menu-tools-scripts = Script Lua…
menu-tools-components = Componenti…
menu-tools-midi = Controllo MIDI…
menu-tools-ptz = Telecamere PTZ…
menu-tools-remote = API di controllo remoto…
menu-tools-panel = Pannello LAN e tally…
menu-help-portal = Portale di aiuto
menu-help-website = Visita il sito web
menu-help-discord = Unisciti al server Discord
menu-help-bug = Segnala un bug…
menu-help-updates = Controlla aggiornamenti…
menu-help-whats-new = Novità
menu-help-about = Informazioni…
menu-help-more-apps = Altre app Freally…
moreapps-title = Altre app Freally

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Categorie delle impostazioni
settings-cat-general = Generale
settings-cat-appearance = Aspetto
settings-cat-streaming = Streaming
settings-cat-output = Uscita
settings-cat-replay = Replay
settings-cat-hotkeys = Scorciatoie
settings-cat-network = Rete
settings-cat-accessibility = Accessibilità
settings-cat-about = Informazioni
settings-ok = OK
settings-cancel = Annulla
settings-apply = Applica
settings-save = Salva
settings-loading = Caricamento delle impostazioni…
settings-hotkeys-filter = Filtra le scorciatoie
settings-hotkeys-filter-placeholder = Digita per filtrare azioni o tasti…
settings-hotkeys-no-match = Nessuna scorciatoia corrisponde a “{ $query }”.
settings-hotkey-none = Nessuno
settings-hotkey-group-ctrl = Ctrl + tasto
settings-hotkey-group-ctrl-shift = Ctrl + Shift + tasto
settings-hotkey-group-ctrl-alt = Ctrl + Alt + tasto
settings-hotkey-group-function = Tasti funzione
settings-hotkey-group-numpad = Tastierino numerico
settings-panic-section = Schermata di panico
settings-meter-section = Indicatori di livello del mixer
settings-meter-note = I colori che gli indicatori di livello del mixer audio attraversano, dal silenzio al clipping. Il preset per daltonici usa una rampa blu → arancione che resta leggibile con deficit rosso-verde.
settings-meter-preset = Colori dell'indicatore
settings-meter-preset-default = Verde / giallo / rosso
settings-meter-preset-colorblind = Per daltonici (blu / arancione)
settings-meter-preset-custom = Personalizzato
settings-meter-low = Normale
settings-meter-mid = Alto
settings-meter-high = Clipping
settings-meter-preview = Anteprima

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Novità
whats-new-loading = Caricamento note di rilascio…
whats-new-version = Novità nella versione { $version }
whats-new-empty = Nessuna nota di rilascio per questa versione.
filters-name-directional-blur = Sfocatura direzionale
filters-name-radial-blur = Sfocatura radiale
filters-name-zoom-blur = Sfocatura zoom
filters-name-pixelate = Pixel
filters-angle = Angolo (°)
filters-center-x = Centro X
filters-center-y = Centro Y
filters-block-size = Dimensione blocco (px)
filters-name-freeze = Congela
filters-freeze-hint = Quando è attivo, questa sorgente mantiene l'ultimo fotogramma — programma, anteprima, registrazione e streaming si congelano insieme. Attiva/disattiva questo filtro per congelare o scongelare.
transform-3d = Inclinazione 3D
transform-rotation-x = Inclinazione X (°)
transform-rotation-y = Inclinazione Y (°)
transform-perspective = Prospettiva
transform-reveal = Mostra/nascondi
transform-reveal-ms = Dissolvenza in ingresso (ms)
sources-clone-title = Clona (stessa sorgente, filtri propri)
sources-clone-item = Clona { $name }
menu-tools-downstream = Keyer di uscita…
menu-tools-transition-rules = Regole di transizione…
dsk-title = Keyer di uscita
dsk-hint = Sovrapposizioni composte sull'uscita del programma — sopra ogni scena, e restano fisse quando cambi scena (un logo, un badge IN DIRETTA, una sottopancia). Il primo della lista è in primo piano.
dsk-empty = Ancora nessun keyer — aggiungi una sorgente per sovrapporla a ogni scena.
dsk-enable = Attiva questo keyer
dsk-move-up = Sposta su (in primo piano)
dsk-move-down = Sposta giù
dsk-remove = Rimuovi keyer
dsk-opacity = Opacità
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Scala
dsk-add = + Aggiungi keyer
transition-rules-title = Regole di transizione
transition-rules-hint = Assegna a una coppia di scene la propria transizione. Quando passi dalla prima scena alla seconda, vengono usati questo tipo e questa durata al posto di quelli predefiniti (una regola Stinger/Immagine usa comunque il file impostato nei controlli di transizione).
transition-rules-empty = Ancora nessuna regola — ogni coppia di scene usa la transizione predefinita.
transition-rules-from = Da
transition-rules-to = A
transition-rules-kind = Transizione
transition-rules-duration = Durata (ms)
transition-rules-add = Aggiungi regola
transition-rules-remove = Rimuovi regola
