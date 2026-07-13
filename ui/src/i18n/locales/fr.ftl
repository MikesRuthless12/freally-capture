# Freally Capture — fr
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Mode Studio
toggle-on = activé
toggle-off = désactivé
stats = Statistiques
core-ok = cœur OK
hide-stats-dock = Masquer le panneau de statistiques
show-stats-dock = Afficher le panneau de statistiques


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Impossible d'enregistrer les paramètres — la modification ne survivra pas à un redémarrage.
studio-mode-leave = Quitter le mode Studio
studio-mode-enter-title = Mode Studio — modifiez une scène d'aperçu, puis validez-la vers le programme avec une transition
vertical-canvas-title = Le second canevas de sortie (vertical 9:16) — enregistrable et diffusable indépendamment
app-version = v{ $version }
core-error = cœur ERREUR
core-unreachable = cœur inaccessible (mode navigateur)
connecting-to-core = connexion au cœur…
filters-source-fallback = Source

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Aperçu du programme
preview-program-output = Sortie programme
preview-canvas-editor = Éditeur de canevas
preview-px-to-edge-label = Pixels jusqu'aux bords du cadre
preview-px-to-edge = px jusqu'au bord G { $left } · H { $top } · D { $right } · B { $bottom }
preview-program-heading = Programme
preview-no-gpu = Aucun adaptateur GPU utilisable n'a été trouvé — le compositeur ne peut pas fonctionner sur cette machine.
preview-starting-compositor = Démarrage du compositeur…
preview-empty-scene = Cette scène est vide — ajoutez une source dans Sources, puis déplacez-la, redimensionnez-la et faites-la pivoter directement ici sur le canevas.
preview-fps = { $fps } fps
preview-dropped = { $dropped } perdues

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Lien d'invitation reçu
remote-join-with-webcam = Rejoindre avec la webcam
remote-dismiss = Ignorer
remote-hosting-guest = Hébergement d'un invité distant
remote-you-are-guest = Vous êtes un invité distant
remote-share-view-title = Partagez votre écran vers l'application de l'invité (il voit votre vue en direct)
remote-stop-sharing-view = Arrêter le partage de la vue
remote-share-my-view = Partager ma vue
remote-allow-center-title = Autoriser l'invité à changer quelle vue occupe le centre (vous gardez le contrôle et pouvez reprendre à tout moment)
remote-guest-switching = Changement par l'invité :
remote-stop-screen = Arrêter l'écran
remote-share-screen = Partager l'écran
remote-share-screen-title-guest = Partagez votre écran avec l'hôte (il devient une source qu'il peut centrer)
remote-center-request-label = Demande de vue centrale
remote-center = Centrer
remote-center-cam-title = Demander à l'hôte de centrer votre caméra
remote-center-my-cam = Ma caméra
remote-center-screen-title = Demander à l'hôte de centrer votre écran partagé
remote-center-my-screen = Mon écran
remote-center-host-title = Rendre le centre à la vue de l'hôte
remote-center-host-view = Vue de l'hôte
remote-end-session = Terminer la session
remote-leave = Quitter
remote-host-view-heading = Vue de l'hôte
remote-host-shared-view-label = La vue partagée de l'hôte
remote-guest-position-label = Position de l'invité
remote-guest-label = Invité
remote-put-guest = Placer l'invité { $position }
remote-remove-title = Retirer l'invité — il peut rejoindre à nouveau avec le même lien
remote-remove = Retirer
remote-ban-title = Bannir l'invité — le bloque et invalide le lien d'invitation
remote-ban = Bannir
remote-guest-self-muted = invité en sourdine
remote-unmute-guest = Réactiver le son de l'invité
remote-mute-guest = Couper le son de l'invité
remote-muted-by-host = Coupé par l'hôte
remote-unmute-mic = Réactiver le micro
remote-mute-mic = Couper le micro
remote-waiting-for-host = en attente de l'hôte


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = source
sources-fallback-video = vidéo
sources-fallback-error = erreur
sources-kind-unknown = ?
sources-missing-source = (source manquante)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Écran
sources-badge-window = Fenêtre
sources-badge-portal = Portail
sources-badge-camera = Caméra
sources-badge-image = Image
sources-badge-media = Média
sources-badge-guest = Invité
sources-badge-color = Couleur
sources-badge-text = Texte
sources-badge-scene = Scène
sources-badge-slides = Diapos
sources-badge-chat = Chat
sources-badge-audio-in = Entrée audio
sources-badge-audio-out = Sortie audio
sources-badge-app-audio = Audio d'app
sources-badge-test-bars = Barres
sources-badge-test-grid = Grille
sources-badge-test-sweep = Balayage
sources-badge-test-tone = Ton
sources-badge-test-sync = Sync
sources-badge-timer = Minuteur

# Add-source menu items
sources-add-display = Capture d'écran
sources-add-window = Capture de fenêtre
sources-add-game = Capture de jeu (à lire d'abord)
sources-add-webcam = Périphérique de capture vidéo
sources-add-image = Image
sources-add-media = Média (fichier vidéo/image)
sources-add-remote-guest = Invité distant (essai P2P)
sources-add-color = Couleur
sources-add-text = Texte
sources-add-timer = Minuteur / Horloge
sources-add-nested-scene = Scène imbriquée
sources-add-slideshow = Diaporama d'images
sources-add-chat-overlay = Overlay de chat en direct
sources-add-test-signal = Signal de test
sources-add-audio-input = Capture d'entrée audio
sources-add-audio-output = Capture de sortie audio
sources-add-app-audio = Audio d'application (Windows)
sources-add-existing = Source existante…

# Panel header + toolbar buttons
sources-panel-title = Sources
sources-group-title = Grouper des sources — choisissez au moins deux éléments, puis Créer le groupe ; les éléments groupés se déplacent et s'affichent/se masquent ensemble
sources-group-aria = Grouper les sources
sources-arrange = Disposer : écran + coins
sources-add-source = Ajouter une source
sources-browser-source-note = La source navigateur arrive comme son propre composant à la demande (un moteur Chromium d'environ 180 Mo — jamais intégré). Aujourd'hui : capturez une vraie fenêtre de navigateur avec Capture de fenêtre + une incrustation chroma/couleur, ou ouvrez le chat/les alertes en tant que dock (Contrôles → Docks).

# Empty state
sources-empty = Aucune source dans cette scène — ajoutez une Capture d'écran, une Fenêtre, une Webcam, une Image, une Couleur ou du Texte avec « + ». Déplacez-les, redimensionnez-les et faites-les pivoter sur le canevas ; les boutons de droite réordonnent la pile.

# Per-row controls
sources-already-in-group = Déjà dans { $name }
sources-pick-for-new-group = Choisir pour le nouveau groupe
sources-pick-item-for-group = Choisir { $name } pour le nouveau groupe
sources-hide = Masquer
sources-show = Afficher
sources-hide-item = Masquer { $name }
sources-show-item = Afficher { $name }
sources-unfocus-title = Défocaliser — rétablir la disposition
sources-focus-title = Focaliser — remplir le canevas (mettre en avant l'intervenant)
sources-unfocus-item = Défocaliser { $name }
sources-focus-item = Focaliser { $name }
sources-center-title = Centrer — en faire la vue centrale partagée (les caméras passent sur le rail)
sources-center-item = Centrer { $name }
sources-rename-item = Renommer { $name }
sources-in-group = Dans le groupe { $name }

# Row status + retry
sources-retry-error = Réessayer — { $message }
sources-retry-item = Réessayer { $name }
sources-status-error = état : erreur
sources-open-privacy-title = Ouvrir les paramètres de confidentialité macOS pour cette autorisation
sources-open-privacy-item = Ouvrir les paramètres de confidentialité pour { $name }
sources-privacy-settings-button = paramètres
sources-status-starting = démarrage…
sources-status-live = en direct
sources-status-aria = état : { $state }

# Media row pause/resume
sources-media-resume-title = Reprendre la vidéo (en direct sur le flux)
sources-media-pause-title = Mettre la vidéo en pause — fige l'image et coupe le son, en direct sur le flux
sources-media-resume-item = Reprendre { $name }
sources-media-pause-item = Mettre en pause { $name }

# Hover controls
sources-unlock = Déverrouiller
sources-lock = Verrouiller
sources-unlock-item = Déverrouiller { $name }
sources-lock-item = Verrouiller { $name }
sources-raise-title = Monter dans la pile
sources-raise-item = Monter { $name }
sources-lower-title = Descendre dans la pile
sources-lower-item = Descendre { $name }
sources-filters-title = Filtres et fusion
sources-filters-item = Filtres pour { $name }
sources-properties-title = Propriétés
sources-properties-item = Propriétés de { $name }
sources-remove-title = Retirer de cette scène
sources-remove-item = Retirer { $name }

# Grouping footer
sources-create-group = Créer le groupe ({ $count })
sources-cancel = Annuler

# Groups list
sources-groups-aria = Groupes de sources
sources-hide-group = Masquer le groupe
sources-show-group = Afficher le groupe
sources-item-count = · { $count } éléments
sources-ungroup-title = Dégrouper — les éléments restent où ils sont
sources-ungroup-item = Dégrouper { $name }

# Live Chat Overlay picker
sources-chat-title = Ajouter un overlay de chat en direct
sources-chat-youtube-label = YouTube — URL de chaîne, de vidéo (watch) ou live_chat (sans clé, sans connexion)
sources-chat-youtube-placeholder = https://www.youtube.com/@votrechaine  ·  ou une URL watch?v=
sources-chat-twitch-label = Twitch — nom de la chaîne (lecture anonyme, sans compte)
sources-chat-twitch-placeholder = votrechaine
sources-chat-kick-label = Kick — identifiant (slug) de la chaîne (point d'accès public, au mieux)
sources-chat-kick-placeholder = votrechaine
sources-chat-note = Les messages apparaissent avec un horodatage h:mm:ss AM/PM sur fond transparent (par défaut en haut à droite ; déplacez-le où vous voulez). Un afflux de chat ne fait qu'évacuer les anciennes lignes — il ne peut jamais bloquer le flux ni l'enregistrement. Le chat Facebook nécessite votre propre jeton Graph et n'est pas encore implémenté — il n'est jamais requis et ne bloque jamais les plateformes ci-dessus.
sources-chat-add = Ajouter l'overlay de chat
sources-chat-default-name = Chat en direct

# Image Slideshow picker
sources-slideshow-title = Ajouter un diaporama d'images
sources-slideshow-empty = Aucune image pour l'instant — Parcourir les ajoute dans l'ordre.
sources-slideshow-remove-slide = Retirer la diapo { $number }
sources-slideshow-browse = Parcourir les images…
sources-slideshow-per-slide-label = Par diapo (ms)
sources-slideshow-crossfade-label = Fondu enchaîné (ms, 0 = coupe)
sources-slideshow-loop-label = Boucle (désactivé = garder la dernière diapo)
sources-slideshow-shuffle-label = Mélanger à chaque cycle
sources-slideshow-note = Le fondu enchaîné mélange des images de même taille ; des tailles différentes se coupent net à la limite (pas de redimensionnement silencieux).
sources-slideshow-add = Ajouter le diaporama ({ $count })

# Nested Scene picker
sources-nested-title = Ajouter une scène imbriquée
sources-nested-empty = Aucune autre scène à imbriquer — ajoutez d'abord une deuxième scène.
sources-nested-scene-name = Scène : { $name }
sources-nested-note = La scène imbriquée s'affiche en direct à la taille du canevas programme et suit ses propres modifications ; les transformations, filtres et fusions s'y appliquent comme à toute source. Ses sources audio rejoignent le mix lorsqu'une scène qui l'affiche est au programme.

# Display / Window capture picker
sources-capture-display-title = Ajouter une capture d'écran
sources-capture-window-title = Ajouter une capture de fenêtre
sources-capture-looking = Recherche de sources…
sources-capture-none-displays = Rien à capturer ici — aucun écran trouvé.
sources-capture-none-windows = Rien à capturer ici — aucune fenêtre trouvée.
sources-capture-portal-note = Sous Wayland, la boîte de dialogue système choisit l'écran ou la fenêtre — les applications ne peuvent pas capturer globalement là-bas, c'est donc la voie honnête (et la seule).
sources-capture-window-note = Les aperçus se mettent à jour en direct. Une fenêtre réduite affiche sa dernière image (ou aucune) jusqu'à ce que vous la restauriez.
sources-thumb-no-preview = pas d'aperçu
sources-thumb-loading = chargement…

# Video Capture Device picker
sources-webcam-title = Ajouter un périphérique de capture vidéo
sources-webcam-looking = Recherche de caméras…
sources-webcam-none = Aucune caméra ni carte de capture trouvée.
sources-webcam-format-label = Format
sources-webcam-format-auto-loading = Auto (chargement des formats…)
sources-webcam-format-auto = Auto (résolution la plus élevée)
sources-webcam-card-presets-label = Préréglages de carte :
sources-webcam-preset-title = Sélectionner le mode { $label } annoncé par cette carte
sources-webcam-add = Ajouter la caméra

# Audio Input / Output capture picker
sources-audio-output-title = Ajouter une capture de sortie audio
sources-audio-input-title = Ajouter une capture d'entrée audio
sources-audio-default-output = Sortie par défaut (ce que vous entendez)
sources-audio-default-input = Entrée par défaut
sources-audio-looking = Recherche de périphériques audio…
sources-audio-none-output = Aucun périphérique de capture audio du bureau trouvé ici.
sources-audio-none-input = Aucun microphone ni entrée ligne trouvé.
sources-audio-input-note = Les tranches du mixeur reçoivent un VU-mètre, un fader, une coupure du son, du monitoring, des filtres (débruitage, gate, compresseur…) et l'affectation de pistes. Tout reste sur cette machine.

# Application Audio picker
sources-appaudio-title = Ajouter l'audio d'application
sources-appaudio-looking = Recherche d'applications émettant du son…
sources-appaudio-none = Aucune application n'émet de son pour l'instant — lancez la lecture dans l'application, puis actualisez.
sources-appaudio-refresh = ⟳ Actualiser
sources-appaudio-note = Capture exactement l'audio de cette application — son propre VU, fader, coupure du son, filtres et piste.

# Game Capture picker
sources-game-title = Capture de jeu
sources-game-checking = Vérification…
sources-game-use-portal = Utiliser la capture d'écran (Portail)
sources-game-use-window = Utiliser plutôt la capture de fenêtre

# Image picker
sources-image-title = Ajouter une image
sources-image-file-label = Fichier image (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Ajouter l'image

# Path field
sources-browse = Parcourir…

# Media picker
sources-media-title = Ajouter un média
sources-media-file-label = Fichier multimédia (mp4, mkv, webm, mov, .frec ou une image)
sources-media-loop-label = Boucle (redémarrer depuis le début à la fin)
sources-media-note = .frec se lit via le codec propriétaire freally-video — rien à télécharger. Les formats de diffusion (mp4/mkv/webm/…) se décodent via le composant FFmpeg à la demande ; leur audio arrive dans le mixeur comme sa propre tranche.
sources-media-add = Ajouter le média

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 heure
sources-ttl-1day = 1 jour

# Remote Guest form
sources-remote-copy-failed = copie impossible — sélectionnez le lien et copiez-le manuellement
sources-remote-join-failed = échec de la connexion : { $error }
sources-remote-title = Invité distant (essai P2P)
sources-remote-host-heading = Hôte — inviter un invité
sources-remote-start-hosting = Démarrer l'hébergement
sources-remote-expires-label = Expire
sources-remote-invite-expiry-aria = Expiration de l'invitation
sources-remote-invite-link-aria = Lien d'invitation
sources-remote-copied = Copié ✓
sources-remote-copy = Copier
sources-remote-share-note = Partagez ce lien (Discord / SMS / e-mail). Il porte votre session et expire au délai défini. L'invité l'ouvre et rejoint avec sa webcam.
sources-remote-qr-note = Scannez sur un téléphone pour rejoindre directement depuis le navigateur — caméra + micro, sans installation. Le lien freally:// copiable ci-dessus s'ouvre dans Freally Capture sur une machine qui l'a.
sources-remote-guest-heading = Invité — rejoindre avec une invitation
sources-remote-paste-placeholder = collez le lien d'invitation
sources-remote-invite-input-aria = Lien d'invitation ou identifiant de session
sources-remote-join = Rejoindre avec la webcam
sources-remote-session-note = Les contrôles de session en direct (couper le son, terminer) restent sur la barre en haut de la fenêtre principale — vous pouvez fermer cette boîte de dialogue.
sources-remote-stop-session = Arrêter la session

# Invite QR
sources-invite-qr-aria = QR code du lien d'invitation

# Remote device pickers
sources-devices-output-unavailable = routage de sortie indisponible — lecture sur le périphérique par défaut
sources-devices-mic-test-failed = échec du test micro : { $error }
sources-devices-heading = Périphériques audio de la session
sources-devices-microphone-label = Microphone
sources-devices-microphone-aria = Microphone de la session
sources-devices-system-default = Valeur par défaut du système
sources-devices-output-label = Sortie
sources-devices-output-aria = Sortie audio de la session
sources-devices-stop-test = Arrêter le test
sources-devices-test = Test — entendez-vous
sources-devices-testing-note = parlez dans le micro — vous entendez les périphériques sélectionnés en direct
sources-devices-idle-note = renvoie votre micro vers la sortie (un casque évite le larsen)

# TURN relay section
sources-turn-save-failed = enregistrement impossible : { $error }
sources-turn-summary = Réseau — relais TURN optionnel (avancé)
sources-turn-note-1 = Les sessions se connectent directement (P2P) — gratuit, sans relais nécessaire. Si les DEUX côtés sont derrière des NAT stricts, la voie directe peut échouer ; un relais TURN que vous hébergez vous-même achemine alors le média. Ignorer ceci convient — la plupart des connexions fonctionnent en direct uniquement.
sources-turn-note-2 = Option gratuite : le palier « Always Free » d'Oracle Cloud exécute coturn sans frais (remarque : Oracle demande une carte de crédit à l'inscription, mais l'offre Always-Free reste gratuite). Étapes : 1) créez la VM gratuite, 2) installez coturn, 3) ouvrez l'UDP 3478, 4) définissez un utilisateur/mot de passe, 5) saisissez turn:ip-de-votre-vm:3478 + les identifiants ici. Votre identifiant reste dans votre fichier de paramètres local et n'est jamais journalisé.
sources-turn-url-label = URL TURN
sources-turn-url-placeholder = turn:host:3478 (vide = direct uniquement)
sources-turn-url-aria = URL TURN
sources-turn-username-label = Nom d'utilisateur
sources-turn-username-aria = Nom d'utilisateur TURN
sources-turn-credential-label = Identifiant
sources-turn-credential-aria = Identifiant TURN
sources-turn-note-3 = Le relais s'active une fois les trois champs renseignés (un serveur TURN exige les identifiants) et s'applique à la prochaine session que vous démarrez ou rejoignez. Vérifiez-le avec un appel de test en relais uniquement entre vos deux machines.
sources-turn-settings-unavailable = paramètres indisponibles (mode navigateur)

# Color picker
sources-color-title = Ajouter une couleur
sources-color-label = Couleur
sources-color-width-label = Largeur
sources-color-height-label = Hauteur
sources-color-add = Ajouter la couleur
sources-testsignal-title = Ajouter un signal de test
sources-testsignal-pattern-label = Motif
sources-testsignal-bars = Barres de couleur SMPTE
sources-testsignal-grid = Grille de calibrage
sources-testsignal-sweep = Balayage de mouvement
sources-testsignal-tone = Tonalité 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Flash + bip de synchro A/V
sources-testsignal-note = Vérifiez scènes, encodeurs, projecteurs et destinations de diffusion sans caméra branchée. Le motif flash + bip alimente l'atelier de synchro A/V.
sources-testsignal-add = Ajouter le signal de test
sources-timer-title = Ajouter un minuteur
sources-timer-mode-label = Mode
sources-timer-wall-clock = Horloge
sources-timer-countdown = Compte à rebours
sources-timer-stopwatch = Chronomètre
sources-timer-since-live = Temps depuis le direct
sources-timer-since-recording = Temps depuis l'enregistrement
sources-timer-note = La durée, le format, le style et les actions de fin de compte se règlent dans les Propriétés de la source.
sources-timer-add = Ajouter le minuteur

# Text picker
sources-text-title = Ajouter du texte
sources-text-label = Texte
sources-text-default = Texte
sources-text-color-label = Couleur
sources-text-color-aria = Couleur du texte
sources-text-size-label = Taille (px)
sources-text-note = La police, l'alignement, le retour à la ligne et le sens RTL se règlent dans les propriétés de la source. La police Noto Sans intégrée (arabe/hébreu inclus) est celle par défaut — identique sur chaque machine.
sources-text-add = Ajouter le texte

# Existing source picker
sources-existing-title = Ajouter une source existante
sources-existing-empty = Aucune source n'existe encore — ajoutez-en une à une scène d'abord. Les sources existantes sont partagées : en renommer ou en reconfigurer une met à jour toutes les scènes qui l'affichent.

# Screen + corners layout
sources-slot-off = Désactivé
sources-slot-center = Centre (écran)
sources-slot-top-left = En haut à gauche
sources-slot-top-right = En haut à droite
sources-slot-bottom-left = En bas à gauche
sources-slot-bottom-right = En bas à droite
sources-layout-title = Disposer : écran + coins
sources-layout-empty = Ajoutez d'abord une capture d'écran et une ou plusieurs caméras à cette scène, puis disposez-les ici.
sources-layout-note = Placez un écran au centre et jusqu'à quatre caméras dans les coins — votre disposition explicatif / podcast. Chaque coin accueille une webcam, une fenêtre d'appel capturée ou un clip média. Vous pouvez ensuite les déplacer sur le canevas.
sources-layout-slot-aria = Emplacement pour { $name }
sources-layout-apply = Appliquer la disposition


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Contrôles
controls-start-stop-title-stop = Arrêter et finaliser l'enregistrement
controls-start-stop-title-start = Enregistrer le flux programme avec la configuration Paramètres → Sortie
controls-finalizing = ◌ Finalisation…
controls-stop-recording = ■ Arrêter l'enregistrement
controls-start-recording = ● Démarrer l'enregistrement
controls-marker-title = Déposer un marqueur de chapitre à cet instant — il arrive dans l'ENREGISTREMENT (chapitres mkv ou fichier annexe). Les marqueurs de flux côté plateforme nécessitent des comptes de plateforme, que cette application ne demande jamais.
controls-marker = ◈ Marqueur
controls-pause-title-resume = Reprendre — le fichier continue comme une seule chronologie continue
controls-pause-title-pause = Pause — aucune image n'est écrite ; reprendre continue le même fichier lisible
controls-resume-recording = ▶ Reprendre l'enregistrement
controls-pause-recording = ⏸ Mettre l'enregistrement en pause
controls-reactions-label = Réactions (intégrées au programme)
controls-reactions-title = Faites flotter une réaction sur le programme — enregistrée ET diffusée, pour que la relecture montre l'instant exact. Les spectateurs du chat les déclenchent aussi (leur emoji de réaction flotte automatiquement) ; un afflux ne fait que limiter ce qui est à l'écran.
controls-react = Réagir { $emoji }
controls-virtual-camera-title = La caméra virtuelle nécessite son propre composant pilote signé par OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / extension CoreMediaIO macOS / v4l2loopback Linux) — elle arrive comme sa propre étape. Le modèle de flux est prêt pour elle : programme, canevas vertical ou une seule source, avec un micro virtuel associé sous Windows/Linux (macOS n'a pas d'API de micro virtuel — dit honnêtement).
controls-virtual-camera = ⌁ Démarrer la caméra virtuelle
controls-files-title = Enregistrements terminés + l'action de remux vers mp4
controls-files = ▤ Fichiers…
controls-output-title = Format d'enregistrement, encodeur, dossier, pistes et découpage
controls-output = ⚙ Sortie…
controls-stream-title = Cible pour passer en direct : service, clé de flux, encodeur, débit
controls-stream = ⦿ Diffuser…
controls-codecs-title = Le composant ffmpeg de codecs de diffusion à la demande (clairement étiqueté, jamais intégré)
controls-codecs = ⬡ Codecs…
controls-replay-title = Durée du tampon de relecture + préréglages de qualité
controls-replay = ⟲ Relecture…
controls-keys-title = Raccourcis globaux : enregistrer, passer en direct, transition, enregistrer la relecture
controls-keys = ⌨ Touches…
controls-scripts-title = Scripts Lua en bac à sable : réagissez aux événements de passage en direct/scène/enregistrement, pilotez le studio
controls-scripts = ⚡ Scripts…
controls-docks-title = Docks navigateur : ouvrez un chat détaché, une page d'alertes ou des boutons Companion comme fenêtre à côté du studio
controls-docks = ⧉ Docks…
controls-remote-title = API distante WebSocket pour contrôleurs Stream Deck / Companion (désactivée par défaut)
controls-remote = ⌁ Distant…
controls-profiles-title = Profils (paramètres) + collections de scènes — instantanés commutables
controls-profiles = ▣ Profils…
controls-bug-title = Signaler un bug — anonyme, sur adhésion (rien n'est envoyé automatiquement)
controls-bug = 🐞 Signaler un bug…
controls-updates-title = Vérifier les mises à jour — signées, vérifiées, rien ne se télécharge sans un clic
controls-updates = ⭳ Vérifier les mises à jour…
controls-saved = Enregistré : { $path }

# --- MixerDock.tsx ---
mixer-title = Mixeur audio
mixer-monitor-error = monitoring : { $error }
mixer-switch-to-horizontal = Passer aux tranches horizontales
mixer-switch-to-vertical = Passer aux tranches verticales
mixer-layout-aria-vertical = Disposition du mixeur : verticale — passer à l'horizontale
mixer-layout-aria-horizontal = Disposition du mixeur : horizontale — passer à la verticale
mixer-empty = Aucune source audio dans cette scène — ajoutez une Capture d'entrée audio (micro) ou une Capture de sortie audio (audio du bureau) avec « + » dans Sources. Les tranches reçoivent un VU-mètre, un fader, une coupure du son, du monitoring, des filtres et l'affectation de pistes.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Loudness du programme (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Loudness momentané (400 ms)
mixer-short-term-title = Loudness à court terme (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitoring
mixer-monitor-device-aria = Périphérique de sortie du monitoring
mixer-default-output = Sortie par défaut

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Mémoire
stats-dropped = Perdues
stats-render = Rendu
stats-gpu = GPU
stats-gpu-compositing = composition
stats-gpu-idle = inactif
stats-vertical-fps = FPS 9:16
stats-targets-label = Cibles de diffusion
stats-shared-encode = · encodage partagé
stats-starting = Démarrage du compositeur…

# --- ScenesRail.tsx ---
scenes-title = Scènes
scenes-new-scene-name = Scène
scenes-add = Ajouter une scène
scenes-empty = Connexion au cœur du studio…
scenes-rename = Renommer { $name }
scenes-on-program = Au programme
scenes-preview = Aperçu { $name }
scenes-switch-to = Basculer vers { $name }
scenes-move-up = Monter
scenes-move-up-aria = Monter { $name }
scenes-move-down = Descendre
scenes-move-down-aria = Descendre { $name }
scenes-last-stays = La dernière scène reste
scenes-remove = Retirer cette scène
scenes-remove-aria = Retirer { $name }


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
channelstrip-monitor-off = Monitoring désactivé
channelstrip-monitor-only = Monitoring seul (pas dans le mix)
channelstrip-monitor-and-output = Monitoring et sortie
channelstrip-status-error = erreur
channelstrip-status-live = en direct
channelstrip-status-waiting-audio = en attente d'audio
channelstrip-status = état : { $state }
channelstrip-status-waiting = en attente
channelstrip-mute = Muet
channelstrip-unmute = Réactiver le son
channelstrip-mute-source = Couper le son de { $name }
channelstrip-unmute-source = Réactiver le son de { $name }
channelstrip-scene-mix-on = Mix par scène ACTIVÉ — cette tranche remplace le mix global pour cette scène (cliquez pour suivre à nouveau le mix global)
channelstrip-scene-mix-off = Mix par scène — donnez à cette tranche son propre fader/coupure pour la scène actuelle
channelstrip-scene-mix-label = Mix par scène pour { $name }
channelstrip-monitor-cycle = { $mode } — cliquez pour changer
channelstrip-monitor-mode = Mode de monitoring de { $name } : { $mode }
channelstrip-audio-filters-title = Filtres audio (débruitage, gate, compresseur…)
channelstrip-audio-filters-label = Filtres audio pour { $name }
channelstrip-advanced-title = Décalage de synchro et raccourcis push-to-talk
channelstrip-advanced-label = Paramètres audio avancés pour { $name }
channelstrip-track-assignment = Affectation de pistes
channelstrip-track = Piste { $n }
channelstrip-track-assigned = Piste { $n } (affectée)
channelstrip-track-label = Piste { $n } pour { $name }
channelstrip-device-error = erreur de périphérique
channelstrip-audio-device-error = erreur de périphérique audio
channelstrip-volume-label = Volume de { $name } en décibels
channelstrip-ptt-hold = Push-to-talk : maintenir { $key }
channelstrip-sync-offset = Décalage de synchro (ms, 0–{ $max } — retarde cet audio)
channelstrip-solo-title = Solo (PFL) — l'écoute n'entend que les tranches en solo ; le mix programme est intact
channelstrip-solo-source = Solo de { $name } (PFL)
channelstrip-pan-label = Balance (double-clic pour réinitialiser)
channelstrip-pan-aria = Balance de { $name }
channelstrip-mono-label = Réduire en mono
channelstrip-ptt-hotkey = Raccourci push-to-talk (muet sauf s'il est maintenu)
channelstrip-ptt-placeholder = ex. Ctrl+Maj+T ou F13
channelstrip-ptt-aria = Raccourci push-to-talk
channelstrip-ptm-hotkey = Raccourci push-to-mute (muet tant qu'il est maintenu)
channelstrip-ptm-placeholder = ex. Ctrl+Maj+M
channelstrip-ptm-aria = Raccourci push-to-mute
channelstrip-hotkeys-note = Les raccourcis fonctionnent pendant que d'autres applications ont le focus. Sous Linux/Wayland, les raccourcis globaux peuvent être indisponibles — c'est une limite du compositeur, dit honnêtement.
channelstrip-apply = Appliquer


# --- LiveButton.tsx ---
livebutton-failure-ended = le flux s'est terminé
livebutton-title-live = Terminer le flux — toutes les cibles (un enregistrement en cours continue)
livebutton-title-offline = Passez en direct sur chaque cible activée dans Paramètres → Diffusion
livebutton-end-stream = ■ Terminer le flux
livebutton-aria-reconnecting = Reconnexion
livebutton-aria-live = En direct
livebutton-badge-retry = essai { $n }
livebutton-badge-live = en direct
livebutton-go-live = ⦿ Passer en direct


# --- RecDot.tsx ---
recdot-paused-aria = Enregistrement en pause
recdot-recording-aria = Enregistrement
recdot-tracks-one = { $count } piste audio en enregistrement
recdot-tracks-other = { $count } pistes audio en enregistrement
recdot-paused = en pause


# --- ReplayControls.tsx ---
replaycontrols-saved = Relecture enregistrée — { $name }
replaycontrols-failure-stopped = le tampon s'est arrêté
replaycontrols-title-disarm = Désarmer le tampon de relecture (abandonne l'historique non enregistré)
replaycontrols-title-arm = Armer le tampon de relecture continu — garde les N dernières secondes prêtes à être enregistrées (son propre encodage léger ; le flux et l'enregistrement sont intacts)
replaycontrols-replay-seconds = ⟲ Relecture { $seconds } s
replaycontrols-arm = ⟲ Armer le tampon de relecture
replaycontrols-save-title = Enregistrer les N dernières secondes dans le dossier des enregistrements (aussi via le raccourci Enregistrer la relecture)
replaycontrols-save = ⤓ Enregistrer


# --- PropertiesDialog.tsx ---
properties-title = Propriétés — { $name }
properties-name = Nom
properties-cancel = Annuler
properties-apply = Appliquer
properties-youtube = YouTube — URL de chaîne / vidéo (watch) / live_chat (sans clé, sans connexion, jamais)
properties-twitch = Twitch — nom de la chaîne (anonyme)
properties-kick = Kick — identifiant (slug) de la chaîne (point d'accès public)
properties-width-px = Largeur (px)
properties-lines = Lignes
properties-font-px = Police (px)
properties-images = Fichiers image (un chemin par ligne, affichés dans l'ordre)
properties-per-slide = Par diapo (ms)
properties-crossfade = Fondu enchaîné (ms, 0 = coupe)
properties-loop-slideshow = Boucle (désactivé = garder la dernière diapo)
properties-shuffle = Mélanger à chaque cycle
properties-nested-scene = Scène que cette source compose (une scène qui contient déjà celle-ci est rejetée)
properties-portal-note = Le portail ScreenCast de Wayland choisit l'écran ou la fenêtre dans la boîte de dialogue système chaque fois que cette source démarre — il n'y a rien à configurer ici, par conception.
properties-appaudio-capturing = Capture de l'audio de { $exe }
properties-appaudio-exe-fallback = une application
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Ré-ajoutez la source pour cibler une autre application (un identifiant de processus change quand l'application redémarre).
properties-image-file = Fichier image
properties-media-file = Fichier multimédia (mp4, mkv, webm, mov, .frec ou une image)
properties-media-loop = Boucle (redémarrer depuis le début à la fin)
properties-media-hwdecode = Décodage matériel (revient au logiciel de lui-même)
properties-media-note = .frec se lit via le codec propriétaire freally-video — rien à télécharger. Les autres formats vidéo se décodent via le composant FFmpeg à la demande. L'audio du fichier obtient sa propre tranche de mixeur ; le décalage de synchro de la tranche affine l'alignement A/V. Un clip sans audio laisse sa tranche silencieuse.
properties-color = Couleur
properties-width = Largeur
properties-height = Hauteur
properties-testtone-note = Une sinusoïde continue à 1 kHz à −20 dBFS. Le niveau et la coupure se règlent sur sa tranche de mixage ; rien d'autre à configurer.
properties-timer-format = Format de l'heure (strftime)
properties-timer-format-note = p. ex. %H:%M:%S (par défaut), %I:%M %p, %A %H:%M — un motif invalide retombe sur %H:%M:%S.
properties-timer-utc = Décalage UTC (minutes)
properties-timer-utc-placeholder = heure locale
properties-timer-duration = Durée (secondes)
properties-timer-target = Compte à rebours jusqu'à (HH:MM)
properties-timer-target-note = Une cible horaire tourne toute seule et se répète chaque jour ; laissez vide pour utiliser la durée avec Démarrer/Pause/Réinitialiser.
properties-timer-end = À zéro
properties-timer-end-none = Ne rien faire
properties-timer-end-flash = Faire clignoter le minuteur
properties-timer-end-switch = Changer de scène
properties-timer-end-scene = Scène
properties-timer-size = Taille (px)
properties-timer-start = Démarrer
properties-timer-pause = Pause
properties-timer-reset = Réinitialiser
properties-text-file = Lire depuis un fichier (chemin ; vide = utiliser le texte ci-dessus)
properties-text-binding = Interpréter comme
properties-text-binding-whole = Fichier entier
properties-text-binding-csv = Cellule CSV
properties-text-binding-json = Pointeur JSON
properties-text-csv-row = Ligne
properties-text-csv-column = Colonne
properties-text-csv-column-placeholder = nom ou numéro
properties-text-json-pointer = Pointeur
properties-text-file-note = Le fichier est relu dans la demi-seconde qui suit un changement. Les écritures atomiques (temp + renommage) sont tolérées : la dernière bonne valeur reste affichée pendant l'échange.
avsync-title = Calibrage de synchro A/V
avsync-intro = Diffusez le motif flash + bip intégré sur votre écran et vos enceintes, capturez-le avec la caméra et le micro à aligner : l'atelier mesure l'écart. La boucle passe par l'écran et les enceintes, leurs petites latences sont donc incluses.
avsync-video-label = Caméra (source vidéo)
avsync-audio-label = Microphone (source audio)
avsync-pick = Choisir une source…
avsync-no-video = Ajoutez d'abord la caméra comme source — l'atelier mesure des sources, pas des périphériques bruts.
avsync-no-audio = Ajoutez d'abord le microphone comme source audio.
avsync-projector = Programme en plein écran sur
avsync-projector-open = Ouvrir le projecteur
avsync-projector-window-title = Programme — synchro A/V
avsync-start-note = Le démarrage ajoute temporairement une source « Motif de synchro A/V » au-dessus de la scène et joue le bip sur le périphérique d'écoute. Tout est retiré à la fin.
avsync-manual = Décalage de synchro (ms, manuel)
avsync-start = Lancer le calibrage
avsync-measuring = Mesure d'environ 12 secondes — pointez la caméra vers le programme clignotant et gardez la pièce calme…
avsync-flash-seen = La caméra voit le flash
avsync-flash-waiting = En attente que la caméra voie le flash…
avsync-beep-heard = Le micro entend le bip
avsync-beep-waiting = En attente que le micro entende le bip…
avsync-cancel = Annuler
avsync-result-offset = La vidéo arrive { $offset } ms après l'audio.
avsync-result-detail = Mesuré sur { $cycles } cycles, ±{ $jitter } ms.
avsync-negative = L'audio arrive déjà après la vidéo. Retarder l'audio ne corrige pas ce sens — si une autre tranche porte le son de cette caméra, baissez-y son décalage.
avsync-over-cap = L'écart mesuré dépasse le plafond de { $max } ms. Un tel écart signifie souvent une mauvaise source — vérifiez la chaîne et remesurez.
avsync-applied = Appliqué — le décalage du micro est maintenant de { $offset } ms.
avsync-apply = Appliquer { $offset } ms au microphone
avsync-again = Remesurer
avsync-close = Fermer
avsync-error-noFlash = La caméra n'a jamais vu le flash. Pointez-la vers le programme clignotant (le plein écran aide), vérifiez que la source est active, puis remesurez.
avsync-error-noBeep = Le micro n'a jamais entendu le bip. Vérifiez que le périphérique d'écoute est audible et que le micro est actif (pas bloqué en appuyer-pour-parler), puis remesurez.
avsync-error-tooFewCycles = Pas assez de cycles flash/bip propres. Gardez le motif bien visible et audible pendant toute la mesure.
avsync-error-notThePattern = Ce qui a été vu ou entendu ne se répète pas au rythme du motif — sans doute la lumière ou le bruit de la pièce, pas le signal de test.
avsync-error-unstable = Les cycles divergent trop pour donner un seul chiffre. Stabilisez la caméra, réduisez le bruit et remesurez.
hotkey-audit-title = Carte des raccourcis
hotkey-audit-search = Recherche
hotkey-audit-filter = Fonction
hotkey-audit-filter-all = Toutes les fonctions
hotkey-audit-col-key = Touche
hotkey-audit-col-action = Action
hotkey-audit-col-where = Où
hotkey-audit-col-status = État
hotkey-audit-ok = OK
hotkey-audit-shared = Partagée par { $count } affectations
hotkey-audit-unregistered = Non enregistrée auprès de l'OS (prise ailleurs ou indisponible)
hotkey-audit-invalid = Raccourci invalide
hotkey-audit-empty = Aucun raccourci pour l'instant — affectez-les dans Réglages → Raccourcis ou sur une tranche du mixeur.
hotkey-audit-export = Exporter l'antisèche
hotkey-audit-exported = Enregistré dans { $path }
hotkey-audit-note = Affectez et modifiez les touches dans Réglages → Raccourcis (actions globales) et sur chaque tranche du mixeur (appuyer-pour-parler / couper) ; ce tableau les audite et les documente.
hotkey-audit-action-record = Basculer l'enregistrement
hotkey-audit-action-go-live = Basculer la diffusion
hotkey-audit-action-transition = Exécuter la transition
hotkey-audit-action-save-replay = Sauver le replay
hotkey-audit-action-add-marker = Ajouter un marqueur
hotkey-audit-action-still = Capturer une image
hotkey-audit-action-panic = Écran de panique
hotkey-audit-action-timer-toggle = Démarrer/pauser tous les minuteurs
hotkey-audit-action-timer-reset = Réinitialiser tous les minuteurs
hotkey-audit-action-ptt = Appuyer pour parler
hotkey-audit-action-ptm = Appuyer pour couper
hotkey-audit-feature-recording = Enregistrement
hotkey-audit-feature-streaming = Diffusion
hotkey-audit-feature-studio = Mode studio
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Marqueurs
hotkey-audit-feature-stills = Images fixes
hotkey-audit-feature-panic = Panique
hotkey-audit-feature-timers = Minuteurs
hotkey-audit-feature-audio = Audio (par source)
properties-text = Texte
properties-font-family = Famille de police (système ; vide = par défaut)
properties-size-px = Taille (px)
properties-text-color = Couleur du texte
properties-align = Alignement
properties-align-left = gauche
properties-align-center = centre
properties-align-right = droite
properties-line-spacing = Interligne
properties-wrap-width = Largeur de retour à la ligne (px ; 0 = désactivé)
properties-force-rtl = Forcer le sens de droite à gauche
properties-text-note = Le rendu utilise un vrai façonnage (liaisons arabes, ligatures) et l'ordre de ligne bidi. La famille Noto Sans intégrée (arabe/hébreu inclus) est celle par défaut ; les familles système fonctionnent aussi. Le CJK utilise les polices système pour l'instant.
properties-repick-capturing = Capture : { $label }
properties-repick-looking = Recherche de sources…
properties-repick-none-displays = Aucun écran trouvé à re-sélectionner.
properties-repick-none-windows = Aucune fenêtre trouvée à re-sélectionner.
properties-repick-again = Re-sélectionner :
properties-device = Périphérique
properties-video-current-device = (périphérique actuel)
properties-format = Format
properties-format-auto-loading = Auto (chargement des formats…)
properties-deinterlace = Désentrelacement
properties-deinterlace-off = Désactivé
properties-deinterlace-discard = Rejet (doubler les lignes d'un champ)
properties-deinterlace-bob = Bob (alterner les champs)
properties-deinterlace-linear = Linéaire (interpoler)
properties-deinterlace-blend = Fusion (moyenner les champs)
properties-deinterlace-adaptive = Adaptatif au mouvement (classe yadif)
properties-field-order = Ordre des trames
properties-field-order-top = Trame haute d'abord
properties-field-order-bottom = Trame basse d'abord
properties-deinterlace-note = Pour les flux entrelacés des cartes d'acquisition. CPU pur, identique sur chaque OS ; le changer redémarre le périphérique (comme un changement de format).
camera-controls-title = Réglages caméra
camera-controls-refresh = Actualiser
camera-controls-reset = Réinitialiser le profil
camera-controls-empty = Aucun réglage pour l'instant — le périphérique doit diffuser (ajoutez-le d'abord à une scène), et certains backends n'en exposent aucun (macOS notamment). C'est l'état honnête par OS.
camera-controls-note = Les changements s'appliquent en direct et s'enregistrent dans le profil du périphérique, réappliqué au rebranchement et au redémarrage.
camera-control-brightness = Luminosité
camera-control-contrast = Contraste
camera-control-hue = Teinte
camera-control-saturation = Saturation
camera-control-sharpness = Netteté
camera-control-gamma = Gamma
camera-control-white-balance = Balance des blancs
camera-control-backlight = Compensation de contre-jour
camera-control-gain = Gain
camera-control-pan = Panoramique
camera-control-tilt = Inclinaison
camera-control-zoom = Zoom
camera-control-exposure = Exposition
camera-control-iris = Iris
camera-control-focus = Mise au point
properties-format-auto = Auto (résolution la plus élevée)
properties-audio-capture-of = Capturer l'audio de
properties-audio-default-output = Sortie par défaut (ce que vous entendez)
properties-audio-default-input = Entrée par défaut
properties-audio-default-suffix = (par défaut)
properties-audio-current-device = (périphérique actuel : { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Gain
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Compresseur
audiofilters-name-limiter = Limiteur
audiofilters-name-eq = Égaliseur 3 bandes
audiofilters-name-denoise = Débruitage
audiofilters-name-ducking = Ducking
audiofilters-title = Filtres audio — { $name }
audiofilters-chain-header = Chaîne de filtres (le premier s'exécute en premier, avant le fader)
audiofilters-add = + Ajouter un filtre
audiofilters-add-menu = Ajouter un filtre audio
audiofilters-empty = Aucun filtre pour l'instant — débruitez un micro (DSP classique, sans IA), appliquez un gate à la pièce, maîtrisez les crêtes avec le compresseur ou faites du ducking sur la musique sous votre voix.
audiofilters-enable = Activer { $name }
audiofilters-run-earlier = Exécuter plus tôt
audiofilters-move-up = Monter { $name }
audiofilters-run-later = Exécuter plus tard
audiofilters-move-down = Descendre { $name }
audiofilters-remove-title = Retirer le filtre
audiofilters-remove = Retirer { $name }
audiofilters-gain-db = Gain (dB)
audiofilters-open-db = Ouverture à (dB)
audiofilters-close-db = Fermeture à (dB)
audiofilters-attack-ms = Attaque (ms)
audiofilters-hold-ms = Maintien (ms)
audiofilters-release-ms = Relâchement (ms)
audiofilters-ratio = Ratio (:1)
audiofilters-threshold-db = Seuil (dB)
audiofilters-output-gain-db = Gain de sortie (dB)
audiofilters-ceiling-db = Plafond (dB)
audiofilters-low-db = Basses (dB)
audiofilters-mid-db = Médiums (dB)
audiofilters-high-db = Aigus (dB)
audiofilters-strength = Intensité
audiofilters-denoise-note = Suppression spectrale DSP classique propriétaire — le bruit constant (ventilateurs, souffle) baisse tandis que la voix passe. Sans IA, sans modèles, conformément à la charte.
audiofilters-duck-under = Atténuer sous
audiofilters-ducking-trigger = Source déclencheuse du ducking
audiofilters-pick-trigger = (choisissez un déclencheur — ex. votre micro)
audiofilters-trigger-at-db = Déclencher à (dB)
audiofilters-duck-by-db = Atténuer de (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Incrustation chroma
filters-name-color-key = Incrustation couleur
filters-name-luma-key = Incrustation luma
filters-name-render-delay = Délai de rendu
filters-name-color-correction = Correction des couleurs
filters-name-lut = Appliquer un LUT
filters-name-blur = Flou
filters-name-mask = Masque d'image
filters-name-sharpen = Netteté
filters-name-scroll = Défilement
filters-name-crop = Rognage
filters-title = Filtres — { $name }
filters-blend-mode = Mode de fusion
filters-chain-header = Chaîne de filtres (le premier s'exécute en premier)
filters-add = + Ajouter un filtre
filters-add-menu = Ajouter un filtre
filters-empty = Aucun filtre pour l'instant — incrustez une webcam en chroma, corrigez les couleurs d'une capture ou faites défiler un bandeau.
filters-enable = Activer { $name }
filters-run-earlier = Exécuter plus tôt
filters-move-up = Monter { $name }
filters-run-later = Exécuter plus tard
filters-move-down = Descendre { $name }
filters-remove-title = Retirer le filtre
filters-remove = Retirer { $name }
filters-key-color-rgb = Couleur clé (n'importe quelle couleur, distance RGB)
filters-similarity = Similarité
filters-smoothness = Lissage
filters-luma-min = Luma min (exclut les tons plus sombres)
filters-luma-max = Luma max (exclut les tons plus clairs)
filters-delay = Délai (ms — vidéo uniquement, ex. pour synchroniser avec l'audio ; plafonné à 500)
filters-key-color = Couleur clé
filters-spill = Débordement
filters-gamma = Gamma
filters-brightness = Luminosité
filters-contrast = Contraste
filters-saturation = Saturation
filters-hue-shift = Décalage de teinte
filters-opacity = Opacité
filters-cube-file = Fichier .cube
filters-amount = Quantité
filters-radius = Rayon
filters-mask-image = Image de masque
filters-mask-mode = Mode
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = inverser
filters-speed-x = Vitesse X (px/s)
filters-speed-y = Vitesse Y (px/s)
filters-crop-left = gauche
filters-crop-top = haut
filters-crop-right = droite
filters-crop-bottom = bas
filters-crop-aria = rogner { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Actualiser
pickershell-refresh-title = Actualiser la liste
pickershell-close = Fermer


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Signaler un bug
bugreport-intro = Les rapports sont anonymes et sur adhésion — rien n'est envoyé automatiquement. Vous relirez le texte exact ci-dessous, puis vous le soumettrez via un ticket GitHub pré-rempli ou votre application de messagerie. Aucune donnée personnelle (votre chemin personnel et votre nom d'utilisateur sont masqués) ; aucun compte, aucun serveur.
bugreport-crash-notice = Freally Capture s'est fermé inopinément lors d'une exécution précédente — les détails anonymes du plantage sont inclus ci-dessous. Les signaler aide à corriger rapidement.
bugreport-description-label = Que faisiez-vous quand c'est arrivé ? (facultatif)
bugreport-description-placeholder = ex. l'aperçu s'est figé quand j'ai ajouté une deuxième webcam
bugreport-include-crash = Inclure les détails anonymes du plantage de la dernière exécution
bugreport-preview-label = Exactement ce qui sera envoyé
bugreport-open-github = Ouvrir un ticket GitHub
bugreport-gmail-title = Ouvre la fenêtre de rédaction de Gmail dans votre navigateur, pré-remplie. Déconnecté ? Google affiche d'abord son écran de connexion.
bugreport-compose-gmail = Rédiger dans Gmail
bugreport-email-title = Ouvre un brouillon dans l'application de messagerie par défaut de ce PC (Outlook, Thunderbird, Mail…)
bugreport-send-email = Envoyer un e-mail
bugreport-copied = Copié ✓
bugreport-copy-report = Copier le rapport
bugreport-dismiss-crash = Ignorer le plantage
bugreport-copy-failed = copie impossible — sélectionnez le texte et copiez-le manuellement
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = CE QUI S'EST PASSÉ
bugreport-preview-no-description = (aucune description fournie)
bugreport-preview-diagnostics = DIAGNOSTICS ANONYMES (aucune donnée personnelle)
bugreport-preview-from = De : Freally Capture
bugreport-preview-crash-excerpt = --- extrait du plantage ---


# --- Updates.tsx ---
updates-title = Mise à jour du logiciel
updates-checking = Recherche de mises à jour…
updates-uptodate = Vous avez la dernière version.
updates-check-again = Vérifier à nouveau
updates-available = La version { $version } est disponible
updates-current-version = (vous avez { $current })
updates-release-notes-label = Version { $version } — Notes de version
updates-confirm = Voulez-vous mettre à jour maintenant ? Le téléchargement est vérifié avec la clé de signature intégrée avant d'être appliqué. Freally Capture se ferme, l'installateur s'exécute et la nouvelle version se rouvre d'elle-même.
updates-yes-update-now = Oui, mettre à jour maintenant
updates-no-not-now = Non, pas maintenant
updates-downloading = Téléchargement de { $version }…
updates-starting = démarrage…
updates-installed = Mise à jour installée.
updates-restart-now = Redémarrer maintenant
updates-restart-later = Redémarrer plus tard
updates-try-again = Réessayer


# --- Models.tsx ---
models-title = Composants
models-ffmpeg-heading = FFmpeg — codecs de diffusion
models-badge-third-party = Tiers · non intégré
models-ffmpeg-desc = Le moteur propre à Freally Capture enregistre le freally-video (.frec) sans perte, sans rien de plus. Enregistrer les formats de diffusion attendus par les plateformes et les lecteurs — H.264/AAC (et HEVC/AV1) en mp4/mkv/mov/webm — utilise FFmpeg, un outil distinct que cette application ne fournit jamais : ces codecs sont grevés de brevets, il reste donc optionnel et clairement étiqueté. Il est téléchargé à la demande depuis la version épinglée ci-dessous, vérifié par SHA-256 avant la première utilisation, mis en cache par utilisateur et piloté comme un processus distinct. Sa licence (LGPL/GPL) est la sienne — voir THIRD-PARTY-NOTICES.
models-checking = Vérification…
models-ffmpeg-not-installed = Non installé. Disponible : FFmpeg { $version } depuis { $source } (téléchargement de { $size }).
models-ffmpeg-none-pinned = Aucune version de FFmpeg n'est encore épinglée pour cette plateforme — l'enregistrement en codecs de diffusion est indisponible ici. L'enregistrement freally-video sans perte n'est pas affecté.
models-ffmpeg-download-verify = Télécharger et vérifier ({ $size })
models-downloading = Téléchargement…
models-download-of = sur
models-cancel = Annuler
models-ffmpeg-verifying = Vérification du téléchargement avec le SHA-256 épinglé…
models-ffmpeg-extracting = Décompression…
models-ffmpeg-ready = Installé et vérifié — { $version }
models-remove = Retirer
models-ffmpeg-retry = Réessayer le téléchargement
models-network-note = Le téléchargement est la seule action réseau de ce panneau et ne démarre jamais de lui-même. Une somme de contrôle erronée annule l'installation — l'application refuse d'exécuter des octets qu'elle ne peut garantir.
models-cef-heading = Runtime de source navigateur — Chromium (CEF)
models-cef-desc = Les sources navigateur affichent des pages web (alertes, widgets, overlays) via Chromium Embedded Framework — un runtime d'environ 100 Mo que cette application ne fournit jamais. Il se télécharge à la demande depuis l'index de builds CEF officiel, est vérifié avec le SHA-1 de cet index avant toute décompression, et est mis en cache par utilisateur. La source navigateur qui s'affiche via lui arrive avec sa propre étape ; ceci installe le runtime dont elle a besoin.
models-cef-download-install = Télécharger et installer
models-cef-unsupported = CEF ne publie aucune build pour cette plateforme — les sources navigateur sont indisponibles ici.
models-cef-resolving = Résolution de la dernière build stable…
models-cef-verifying = Vérification du téléchargement avec le SHA-1 de l'index…
models-cef-extracting = Décompression du runtime…
models-cef-ready = Installé — CEF { $version }.
models-cef-retry = Réessayer
models-integrations-heading = Intégrations optionnelles
models-badge-never-bundled = Jamais intégré
models-ndi-detected = Détecté
models-ndi-not-installed = Non installé
models-vst-available = Disponible
models-vst-not-available = Non disponible


# --- Recordings.tsx ---
recordings-title = Enregistrements
recordings-loading = Lecture du dossier…
recordings-empty = Aucun enregistrement pour l'instant — Démarrer l'enregistrement écrit dans le dossier défini dans Sortie.
recordings-frec-label = sans perte propriétaire (freally-video)
recordings-remux-title = Réencapsuler en mp4 — copie du flux, sans réencodage, sans changement de qualité (nécessite le composant FFmpeg)
recordings-remuxing = Remux…
recordings-remux-to-mp4 = Remux vers MP4
recordings-export-mp4-title = Décoder le .frec propriétaire et réencoder en MP4 (H.264/AAC) pour qu'il se lise dans n'importe quel lecteur — nécessite le composant FFmpeg
recordings-exporting = Exportation…
recordings-export-mp4 = Exporter → MP4
recordings-export-mkv-title = Décoder le .frec propriétaire et réencoder en MKV pour qu'il se lise dans n'importe quel lecteur
recordings-starting = démarrage…
recordings-frames = { $done } / { $total } images
recordings-cancel = Annuler
recordings-export-cancelled = Exportation annulée.
recordings-exported-to = Exporté vers { $path }
recordings-remuxed-to = Remuxé vers { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Ouvrir un enregistrement .frec
openfrec-desc = Freally Capture enregistre le format sans perte propriétaire .frec — il ne le lit pas. Freally Player lira le .frec directement à sa sortie. Pour l'instant, exportez-le en MP4/MKV et il se lit dans n'importe quel lecteur (VLC, le lecteur de votre OS, n'importe lequel).
openfrec-exported-to = Exporté vers { $path }
openfrec-exporting = Exportation…
openfrec-starting = démarrage…
openfrec-export-mp4 = Exporter → MP4
openfrec-export-mkv = Exporter → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Canevas vertical (9:16)
vertical-enable = Activer le second canevas — enregistrable et diffusable indépendamment du programme
vertical-scene-label = Scène que ce canevas compose
vertical-width = Largeur
vertical-height = Hauteur
vertical-preview-alt = Aperçu du canevas vertical
vertical-note = Les positions des éléments sont exactes au pixel entre les canevas : sélectionnez cette scène dans le rail Scènes pour la disposer tandis que cet aperçu montre le résultat vertical. Les cibles de diffusion choisissent ce canevas dans ⦿ Diffuser… ; Paramètres → Sortie peut l'enregistrer à côté du fichier principal.
vertical-close = Fermer


# --- EulaGate.tsx ---
eula-title = Freally Capture — Contrat de licence
eula-version = v{ $version }
eula-intro = Veuillez lire et accepter ce contrat pour utiliser Freally Capture. En bref : c'est un outil neutre, et vous êtes seul responsable de ce que vous capturez, enregistrez et diffusez — et de détenir les droits correspondants.
eula-thanks = Merci d'avoir lu.
eula-scroll-hint = Faites défiler jusqu'à la fin pour continuer.
eula-decline = Refuser et quitter
eula-agree = J'accepte


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Sortie
output-loading = Les paramètres sont encore en cours de chargement…
output-container-frec = freally-video (.frec) — sans perte, propriétaire, rien à télécharger
output-container-mkv = MKV — tolérant aux plantages ; remux vers mp4 plus tard
output-container-mp4 = MP4 — se lit partout
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Sans perte
output-preset-lossless-title = Le codec propriétaire freally-video — au bit près, sans téléchargement
output-preset-high-label = Haute qualité
output-preset-high-title = MP4, meilleur encodeur détecté, CQ 16 quasi sans perte, préréglage Qualité
output-preset-balanced-label = Équilibré
output-preset-balanced-title = MKV, meilleur encodeur détecté, CQ 23, préréglage Équilibré
output-recording-format = Format d'enregistrement
output-ffmpeg-warning = Ce format nécessite le composant FFmpeg (codecs de diffusion — non intégré). Le .frec sans perte n'a besoin de rien.
output-install = Installer…
output-recordings-folder = Dossier des enregistrements
output-folder-placeholder = Dossier Vidéos de l'OS
output-filename-prefix = Préfixe de nom de fichier
output-recording-template = Nom de fichier des enregistrements
output-replay-template = Nom de fichier des relectures
output-still-template = Nom de fichier des images
output-template-tokens = Jetons : {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Dossier des relectures
output-still-folder = Dossier des images
output-same-folder-placeholder = Dossier des enregistrements
output-frame-rate = Fréquence d'images
output-fps-option = { $fps } fps
output-split-every = Découper tous les (minutes, 0 = désactivé)
output-output-width = Largeur de sortie (0 = canevas ; formats de diffusion uniquement)
output-output-height = Hauteur de sortie (0 = canevas)
output-record-vertical = Enregistrer aussi le canevas vertical (un fichier parallèle « … (vertical) » ; nécessite le canevas 9:16 activé)
output-audio-tracks = Pistes audio
output-recorded-tracks-group = Pistes enregistrées
output-track-last-one = Au moins une piste doit être enregistrée
output-record-track-on = Enregistrer la piste { $index } : activé
output-record-track-off = Enregistrer la piste { $index } : désactivé
output-encoder-heading = Encodeur
output-video-encoder = Encodeur vidéo
output-encoder-auto = Auto — meilleur détecté (H.264)
output-encoder-unavailable = — indisponible ici
output-preset = Préréglage
output-preset-quality = Qualité
output-preset-balanced-option = Équilibré
output-preset-performance = Performance
output-rate-control = Contrôle du débit
output-rc-cqp = CQP (qualité constante)
output-rc-cbr = CBR (débit constant)
output-rc-vbr = VBR (débit variable)
output-cq = CQ (0–51, plus bas = meilleur)
output-bitrate = Débit (kbps)
output-keyframe = Intervalle d'image clé (s)
output-audio-bitrate = Débit audio (kbps / piste)
output-presets = Préréglages :

# --- SettingsStream.tsx ---
stream-title = Paramètres — Diffusion
stream-target-enabled = Cible { $index } activée
stream-target = Cible { $index }
stream-remove = Retirer
stream-service = Service
stream-canvas = Canevas
stream-canvas-main = Principal (programme)
stream-canvas-vertical = Vertical (9:16 — activez-le dans le studio)
stream-ingest-srt = URL d'ingestion SRT
stream-ingest-whip = URL du point de terminaison WHIP
stream-ingest-url = URL d'ingestion
stream-ingest-override = (remplacement — vide = le préréglage du service)
stream-key-srt = streamid (facultatif — ajouté en ?streamid=… ; traité comme un secret)
stream-key-whip = Jeton Bearer (facultatif — envoyé comme en-tête Authorization ; un secret)
stream-key-custom = Clé de flux (depuis votre serveur — traitée comme un secret)
stream-key-service = Clé de flux (depuis votre tableau de bord créateur — traitée comme un secret)
stream-key-aria = Clé de flux { $index }
stream-key-hide = Masquer
stream-key-show = Afficher
stream-encoder = Encodeur (H.264 — ce que RTMP, SRT et WHIP transportent tous)
stream-encoder-auto = Auto — le meilleur encodeur H.264 détecté
stream-encoder-unavailable = (indisponible ici)
stream-video-bitrate = Débit vidéo (kbps, CBR)
stream-audio-bitrate = Débit audio (kbps)
stream-fps = FPS
stream-keyframe = Intervalle d'image clé (s)
stream-audio-track = Piste audio (1–6)
stream-output-width = Largeur de sortie (0 = canevas)
stream-output-height = Hauteur de sortie (0 = canevas)
stream-add-target = + Ajouter une cible
stream-go-live-note = Passer en direct publie sur chaque cible activée en même temps, directement vers chaque plateforme. Les cibles ayant des réglages d'encodeur identiques partagent un seul encodage.
stream-auto-record = Démarrer l'enregistrement quand je passe en direct (l'enregistrement s'arrête tout de même indépendamment)
stream-ffmpeg-note-before = Les codecs de diffusion passent par le composant ffmpeg à la demande étiqueté —
stream-ffmpeg-note-link = gérez-le ici
stream-ffmpeg-note-after = . L'enregistrement local continue quoi que fasse le flux.
stream-cancel = Annuler
stream-save = Enregistrer

# --- SettingsReplay.tsx ---
replay-title = Paramètres — Tampon de relecture
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Basse (3 Mbps)
replay-quality-standard = Standard (6 Mbps)
replay-quality-high = Haute (12 Mbps)
replay-length-presets = Préréglages de durée
replay-quality-presets = Préréglages de qualité
replay-length-seconds = Durée (secondes)
replay-video-bitrate = Débit vidéo (kbps)
replay-fps = FPS
replay-audio-track = Piste audio (1–6)
replay-note = Une fois armé, le tampon exécute son propre encodage léger dans un anneau borné sur disque — environ { $mb } Mo à ces réglages. L'enregistrement recolle l'anneau sans réencodage et ne touche jamais au flux ni à l'enregistrement. Les changements s'appliquent au prochain armement.
replay-cancel = Annuler
replay-save = Enregistrer

# --- SettingsRemote.tsx ---
remote-title = Paramètres — Contrôle à distance
remote-enable = Activer l'API distante WebSocket
remote-password = Mot de passe (requis — les contrôleurs s'authentifient avec)
remote-password-placeholder = un mot de passe pour vos contrôleurs
remote-password-hide = Masquer
remote-password-show = Afficher
remote-port = Port
remote-allow-lan = Autoriser les connexions LAN (par défaut, cette machine uniquement)
remote-note = Désactivé = le port est fermé. Activé = un WebSocket protégé par mot de passe sur 127.0.0.1 (ou votre LAN si vous y consentez) qui peut changer de scène, exécuter la transition, démarrer/arrêter le flux et l'enregistrement, enregistrer des relectures et régler les coupures/volumes — les mêmes actions que l'interface, rien de plus. Il ne peut pas lire de fichiers. Traitez le mot de passe comme tout identifiant ; préférez cette machine uniquement, sauf si vous contrôlez spécifiquement depuis un autre appareil.
remote-password-required = Un mot de passe est requis pour activer l'API distante.
remote-cancel = Annuler
remote-save = Enregistrer

# --- SettingsHotkeys.tsx ---
hotkeys-title = Paramètres — Raccourcis
hotkeys-record = Démarrer / arrêter l'enregistrement
hotkeys-record-placeholder = ex. Ctrl+Maj+R
hotkeys-go-live = Passer en direct / Terminer le flux
hotkeys-go-live-placeholder = ex. Ctrl+Maj+L
hotkeys-transition = Transition du mode Studio
hotkeys-transition-placeholder = ex. Ctrl+Maj+T ou F13
hotkeys-save-replay = Enregistrer la relecture (N dernières secondes)
hotkeys-save-replay-placeholder = ex. Ctrl+Maj+S
hotkeys-add-marker = Déposer un marqueur de chapitre (enregistrement)
hotkeys-add-marker-placeholder = ex. Ctrl+Maj+K
hotkeys-note = Les raccourcis sont globaux — ils se déclenchent pendant que d'autres applications ont le focus. Vide = non attribué. Les touches push-to-talk/mute du mixeur se trouvent dans le menu ⋯ de chaque tranche. Sous Linux/Wayland, les raccourcis globaux peuvent être indisponibles (une limite du compositeur) — les boutons continuent de fonctionner.
hotkeys-cancel = Annuler
hotkeys-save = Enregistrer

# --- WorkspaceDialog.tsx ---
workspace-title = Profils et collections de scènes
workspace-profiles = Profils
workspace-profiles-hint = Un profil est vos paramètres — cible de diffusion, sortie, raccourcis. Changez selon l'émission ou la plateforme.
workspace-collections = Collections de scènes
workspace-collections-hint = Une collection est vos scènes + sources. Créer duplique l'actuelle comme point de départ.
workspace-active = Active
workspace-switch-to = Basculer vers { $name }
workspace-active-marker = ● active
workspace-new-name-placeholder = nouveau nom…
workspace-new-name-label = Nouveau nom de { $title }
workspace-create = Créer

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importer depuis OBS…
workspace-import-obs-hint = Importez une collection de scènes OBS (son scenes.json). Votre collection actuelle est d'abord enregistrée.
workspace-import-busy = Importation…
workspace-import-title = « { $name } » importée
workspace-import-summary = { $scenes } scènes · { $sources } sources · { $items } éléments
workspace-import-dismiss = Fermer
workspace-import-clean = Tout a été importé correctement.
workspace-import-geometry-caveat = Les tailles et positions sont ajustées d'après la disposition OBS — vérifiez chaque scène et resélectionnez les périphériques de capture.
workspace-import-notes-title = Importé avec remarques
workspace-import-skipped-title = Non importé
import-note-needsReselect = Resélectionner l'appareil/l'écran/la fenêtre
import-note-gameCaptureAsWindow = Capture de jeu → Capture de fenêtre
import-note-referencesFile = Vérifier le chemin du fichier
import-note-filterDropped = Certains filtres non pris en charge
import-note-geometryApproximated = Position/taille approximées
import-skip-unsupportedKind = Aucun type de source équivalent
import-skip-group = Les groupes ne sont pas encore pris en charge

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Relier les fichiers manquants…
doctor-title = Fichiers manquants
doctor-scanning = Analyse…
doctor-all-good = Tous les fichiers référencés sont présents. Rien à relier.
doctor-intro = { $count } fichiers référencés sont introuvables sur cet ordinateur. Indiquez le nouvel emplacement de chacun — chaque scène qui l'utilise est corrigée d'un coup.
doctor-relinked = { $count } références reliées.
doctor-uses = utilisé { $count }×
doctor-locate = Localiser…
doctor-locate-folder = Chercher dans un dossier…
doctor-locate-folder-hint = Choisissez un dossier ; chaque fichier manquant est retrouvé par son nom et relié.
doctor-kind-image = image
doctor-kind-media = média
doctor-kind-slideshow = diaporama
doctor-kind-font = police
doctor-kind-lut = LUT
doctor-kind-mask = masque
history-relinkFiles = Relier les fichiers

# --- ScriptsDialog.tsx ---
scripts-title = Scripts (Lua)
scripts-empty = Aucun script pour l'instant — ajoutez un fichier .lua. Voir scripts/sample.lua pour l'API : réagissez aux événements de passage en direct/scène/enregistrement et pilotez les mêmes commandes que l'API distante.
scripts-enable = Activer { $path }
scripts-remove = Retirer { $path }
scripts-path-label = Chemin du script
scripts-add = Ajouter
scripts-note = Les scripts s'exécutent en bac à sable — aucun accès aux fichiers ni à l'OS ; ils ne peuvent appeler que les mêmes commandes du studio que l'API distante (changer de scène, transition, enregistrer/diffuser/relecture, coupures). Une erreur de script est journalisée et contenue. Les changements s'appliquent en moins d'une seconde.
scripts-error-not-lua = Pointez vers un fichier .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Docks navigateur
browser-dock-empty = Aucun dock pour l'instant — ajoutez un chat détaché, une page d'alertes ou vos boutons web Companion.
browser-dock-open = Ouvrir
browser-dock-remove = Retirer { $name }
browser-dock-name-placeholder = nom (ex. Chat Twitch)
browser-dock-name-label = Nom du dock
browser-dock-url-label = URL du dock
browser-dock-note = Un dock s'ouvre comme sa propre fenêtre que vous pouvez placer à côté du studio. La page n'a aucun accès à l'application — elle ne fait que s'afficher. URLs http(s) uniquement ; les docks ne s'ouvrent que lorsque vous cliquez sur Ouvrir.
browser-dock-error-name = Nommez le dock (ex. Chat Twitch).
browser-dock-error-url = Une URL de dock doit commencer par http:// ou https://.

# --- studio-preview-pane ---
studio-preview-label = Aperçu du Mode Studio
studio-preview-heading = Aperçu
studio-preview-hint = cliquez sur une scène pour la charger ici
studio-preview-empty = L’aperçu apparaîtra ici.
studio-preview-mirrors = reflète le direct
studio-preview-transition-select = Transition
studio-preview-duration = Durée de la transition (ms)
studio-preview-commit-title = Valider l’aperçu → le direct via la transition (le public le voit)
studio-preview-transitioning = Transition en cours…
studio-preview-transition-button = Transition ⇄
studio-preview-luma-placeholder = image de balayage en niveaux de gris (png/jpg)
studio-preview-luma-label = Image de balayage Luma
studio-preview-browse = Parcourir…
studio-preview-filter-images = Images
studio-preview-filter-video = Vidéo
studio-preview-stinger-placeholder = vidéo Stinger (le ProRes 4444 .mov conserve son canal alpha)
studio-preview-stinger-label = Fichier vidéo Stinger
studio-preview-stinger-cut-label = Point de coupe du Stinger (ms)
studio-preview-stinger-cut-title = Moment où le changement de scène se produit sous le Stinger (ms dans la transition)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Coupe
transition-kind-fade = Fondu
transition-kind-slide-left = Glissement ←
transition-kind-slide-right = Glissement →
transition-kind-slide-up = Glissement ↑
transition-kind-slide-down = Glissement ↓
transition-kind-swipe-left = Balayage ←
transition-kind-swipe-right = Balayage →
transition-kind-luma-linear = Balayage Luma (linéaire)
transition-kind-luma-radial = Balayage Luma (radial)
transition-kind-luma-horizontal = Balayage Luma (horizontal)
transition-kind-luma-diamond = Balayage Luma (losange)
transition-kind-luma-clock = Balayage Luma (horloge)
transition-kind-image = Balayage image (personnalisé)
transition-kind-stinger = Stinger (vidéo)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Personnalisé (RTMP/RTMPS)
stream-service-srt = SRT (auto-hébergé)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = À propos
about-tagline = Enregistrez et diffusez comme un studio — sans compte ni cloud.
about-version = Version
about-created-by = Créé par
about-project-started = Projet démarré
about-first-stable = Première version stable
about-first-stable-pending = Pas encore — la 1.0.0 est en cours
about-platform = Plateforme
about-local-first = Freally Capture fonctionne entièrement sur votre machine. Aucun compte, aucune télémétrie, aucun cloud — la seule chose qui quitte votre ordinateur est le flux que vous avez choisi d'envoyer.
about-website = Site web
about-issues = Signaler un problème
about-license = Licence
about-eula = EULA
about-third-party = Mentions des tiers
about-check-updates = Vérifier les mises à jour…

# --- unified settings modal (TASK-906) ---
settings-title = Paramètres
settings-language-section = Langue
settings-language = Langue de l'interface
settings-language-system = Valeur par défaut du système
settings-language-note = Une langue choisie ici est mémorisée. « Valeur par défaut du système » suit votre système d'exploitation. Le texte non traduit revient à l'anglais.
settings-appearance-section = Apparence
settings-theme = Thème
settings-theme-dark = Sombre
settings-theme-light = Clair
settings-theme-custom = Personnalisé
settings-accent = Accent
settings-general-section = Général
settings-show-stats-dock = Afficher le panneau de statistiques
settings-more-section = Plus de paramètres
settings-open-output = Enregistrement…
settings-open-stream = Diffusion…
settings-open-replay = Relecture…
settings-open-hotkeys = Raccourcis…
settings-open-remote = API distante…
settings-open-about = À propos…
controls-settings = ⚙ Paramètres…
controls-settings-title = Langue, apparence et préférences globales de l'application

# --- command palette (TASK-904) ---
palette-title = Palette de commandes
palette-search = Rechercher des scènes, sources et actions
palette-placeholder = Rechercher des scènes, sources, actions…
palette-no-results = Aucun résultat pour “{ $query }”
palette-hint = ↑ ↓ pour se déplacer · Enter pour exécuter · Esc pour fermer
palette-group-scenes = Scène
palette-group-sources = Source
palette-group-actions = Action
palette-transition = Transition Aperçu → Direct
palette-save-replay = Enregistrer la rediffusion
palette-add-marker = Ajouter un marqueur de chapitre
palette-vertical-canvas = Canevas vertical (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Bienvenue dans Freally Capture
wizard-welcome = Deux étapes rapides : voir ce dont votre machine est capable, puis créer une scène. Cela prend une trentaine de secondes, et vous pourrez tout changer plus tard.
wizard-local-first = Rien de tout cela ne quitte votre ordinateur. Freally Capture n'a ni comptes, ni télémétrie, ni cloud.
wizard-start = C'est parti
wizard-skip = Ignorer
wizard-hardware-title = Ce dont votre machine est capable
wizard-probing = Analyse de votre carte graphique et de votre processeur…
wizard-encoder = Encodeur
wizard-canvas = Canevas
wizard-bitrate = Débit
wizard-probe-found = Trouvé : { $gpus } · { $cores } cœurs physiques
wizard-no-gpu = aucun GPU dédié
wizard-apply = Utiliser ces paramètres
wizard-keep-current = Garder ce que j'ai
wizard-template-title = Commencer avec une scène
wizard-template-screen = Capturer mon écran
wizard-template-screen-note = Ajoute une Capture d'écran de votre moniteur principal. Le point de départ le plus courant.
wizard-template-empty = Partir de zéro
wizard-template-empty-note = Une scène vide. Ajoutez vous-même des sources avec le bouton +.
wizard-done = Tout est prêt.
wizard-done-hint = Appuyez sur Ctrl+K à tout moment pour rechercher des scènes, des sources et des actions. Les paramètres se trouvent derrière le bouton ⚙.
wizard-close = Commencer à diffuser

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Votre carte graphique peut encoder la vidéo toute seule, ce qui laisse le processeur libre pour le reste du studio.
autoconfig-reason-software = Aucun encodeur matériel utilisable n'a été trouvé, c'est donc le processeur qui encodera. Ça fonctionne, mais ça sollicite davantage le CPU.
autoconfig-reason-quality-hardware = 1080p à 60 images par seconde, à un débit accepté par toutes les grandes plateformes.
autoconfig-reason-quality-software = 30 images par seconde, car l'encodage logiciel à 60 fait perdre des images sur la plupart des processeurs.
autoconfig-reason-quality-low-cores = Un débit plus faible, car ce processeur a peu de cœurs et l'encodage logiciel les disputera au compositeur.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Enregistrement démarré
announce-recording-paused = Enregistrement en pause
announce-recording-stopped = Enregistrement arrêté
announce-live-started = Vous êtes en direct
announce-live-ended = Diffusion terminée
announce-reconnecting = Connexion perdue, reconnexion en cours
announce-stream-failed = Échec de la diffusion
announce-frames-dropped = Images perdues : { $count }

# CAP-M01 — undo/redo edit history
palette-undo = Annuler
palette-redo = Rétablir
palette-edit-history = Historique des modifications…
history-title = Historique des modifications
history-empty = Rien à annuler pour l’instant.
history-current = État actuel
history-close = Fermer
history-addScene = Ajouter une scène
history-renameScene = Renommer la scène
history-removeScene = Supprimer la scène
history-reorderScene = Réorganiser les scènes
history-addSource = Ajouter une source
history-removeSource = Supprimer la source
history-reorderSource = Réorganiser les sources
history-renameSource = Renommer la source
history-transformSource = Déplacer la source
history-toggleVisibility = Basculer la visibilité
history-toggleLock = Basculer le verrouillage
history-setBlendMode = Changer le mode de fusion
history-editSourceProperties = Modifier les propriétés
history-applyLayout = Organiser la disposition
history-moveToSeat = Placer à l’emplacement
history-groupSources = Grouper les sources
history-ungroupSources = Dégrouper les sources
history-toggleGroupVisibility = Basculer le groupe
history-setSceneAudio = Audio de la scène
history-setVerticalCanvas = Canevas vertical
history-addFilter = Ajouter un filtre
history-removeFilter = Supprimer le filtre
history-reorderFilter = Réorganiser les filtres
history-editFilter = Modifier le filtre
history-toggleFilter = Basculer le filtre
history-setVolume = Ajuster le volume
history-toggleMute = Basculer la sourdine
history-setMonitor = Changer le monitoring
history-setTracks = Changer les pistes
history-setSyncOffset = Ajuster la synchro A/V
history-setAudioHotkeys = Raccourcis audio

# CAP-M04 — alignment aids
settings-alignment-section = Aides d’alignement
settings-smart-guides = Repères intelligents (aimantation au déplacement)
settings-safe-areas = Zones de sécurité
settings-rulers = Règles
align-group = Aligner sur le canevas
align-left = Aligner à gauche
align-hcenter = Centrer horizontalement
align-right = Aligner à droite
align-top = Aligner en haut
align-vcenter = Centrer verticalement
align-bottom = Aligner en bas

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Aligner et répartir la sélection
arrange-left = Aligner les bords gauches
arrange-hcenter = Centrer horizontalement
arrange-right = Aligner les bords droits
arrange-top = Aligner les bords supérieurs
arrange-vcenter = Centrer verticalement
arrange-bottom = Aligner les bords inférieurs
distribute-h = Répartir horizontalement
distribute-v = Répartir verticalement
guides-group = Repères
guides-add-v = Ajouter un repère vertical
guides-add-h = Ajouter un repère horizontal
history-arrangeItems = Organiser les éléments
history-editGuides = Modifier les repères

# CAP-M05 — edit transform + copy/paste
transform-title = Modifier la transformation — { $name }
transform-anchor = Ancre
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotation
transform-crop = Rognage
transform-crop-left = Gauche
transform-crop-top = Haut
transform-crop-right = Droite
transform-crop-bottom = Bas
transform-no-size = La taille et le rognage seront disponibles dès que la source communiquera ses dimensions.
transform-copy = Copier la transformation
transform-paste = Coller la transformation
transform-close = Fermer
filters-copy = Copier les filtres ({ $count })
filters-paste = Coller les filtres ({ $count })
palette-edit-transform = Modifier la transformation…
history-pasteFilters = Coller les filtres

# CAP-M26 — keying workbench
workbench-title = Atelier d'incrustation — { $name }
workbench-mode-keyed = Incrusté
workbench-mode-source = Source
workbench-mode-matte = Cache
workbench-mode-split = Partagé
workbench-eyedropper = Pipette
workbench-eyedropper-hint = Cliquez sur la source pour prélever la couleur d'incrustation.
workbench-loupe = Loupe
workbench-split = Partage
workbench-preview-alt = Aperçu de l'atelier d'incrustation
workbench-tune = Régler
workbench-close = Fermer

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Cliquez sur une scène pour y couper.
multiview-hint-stage = Cliquez sur une scène pour la préparer en aperçu.
palette-multiview = Moniteur multiview

# CAP-M07 — projectors
projector-title = Ouvrir un projecteur
projector-source = Source
projector-target-program = Programme
projector-target-preview = Aperçu
projector-target-scene = Scène…
projector-target-source = Source…
projector-target-multiview = Multiview
projector-which-scene = Quelle scène
projector-which-source = Quelle source
projector-none = Rien à afficher
projector-display = Écran
projector-windowed = Fenêtre flottante (cet écran)
projector-display-option = Écran { $n } — { $w }×{ $h }
projector-primary = (principal)
projector-open = Ouvrir
projector-cancel = Annuler
projector-exit-hint = Appuyez sur Échap pour quitter
palette-projector = Ouvrir un projecteur…

# CAP-M08 — still-frame grab
palette-still = Capturer une image…
still-saved-toast = Image enregistrée : { $name }
still-failed-toast = Échec de la capture d'image : { $error }
hotkeys-still = Capturer une image
hotkeys-still-placeholder = p. ex. Ctrl+Shift+P

# CAP-M13 — source health dashboard
palette-source-health = Santé des sources…
palette-av-sync = Calibrage de synchro A/V…
palette-hotkey-audit = Carte des raccourcis…
health-title = Santé des sources
health-col-source = Source
health-col-state = État
health-col-resolution = Résolution
health-col-fps = FPS
health-col-last-frame = Dernière image
health-col-dropped = Ignorées
health-col-retries = Redémarrages
health-col-actions = Actions
health-state-live = En direct
health-state-waiting = En attente
health-state-error = Erreur
health-state-inactive = Inactive
health-restart = Redémarrer
health-properties = Propriétés
health-empty = Cette collection n'a pas encore de sources.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Quitter Freally Capture ?
quit-body = Quitter maintenant effectuera en toute sécurité, dans l'ordre :
quit-consequence-stream = Terminer le direct et se déconnecter du service.
quit-consequence-recording = Arrêter l'enregistrement et finaliser ses fichiers.
quit-consequence-replay = Arrêter le tampon de relecture — les images non enregistrées sont perdues.
quit-confirm = Quitter en sécurité
quit-quitting = Fermeture…
quit-cancel = Annuler

# CAP-M11 — crash-safe recording salvage
salvage-title = Récupérer les enregistrements interrompus ?
salvage-body = La dernière session s'est terminée de façon inattendue pendant l'écriture de ces enregistrements. La réparation crée une copie lisible à côté de l'original — le fichier d'origine n'est jamais modifié.
salvage-repair = Réparer
salvage-repairing = Réparation…
salvage-done = Réparé
salvage-repaired = Réparé → { $name }
salvage-failed = Échec de la réparation : { $error }
salvage-dismiss = Pas maintenant

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Panne de l'encodeur — passage de { $from } à { $to }. Le direct s'est reconnecté et continue.
fallback-toast-recording = Panne de l'encodeur — passage de { $from } à { $to }. L'enregistrement continue dans un nouveau fichier.
fallback-note = Encodeur de repli : { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Le son du programme est devenu muet
alarm-clipping = Le son du programme sature
alarm-black = L'image du programme est noire
alarm-frozen = L'image du programme n'a pas changé depuis un moment
alarm-lowDisk = Espace disque : environ { $minutes } min restantes au débit actuel
alarm-dismiss = Ignorer l'alarme
alarm-cleared = Résolu : { $alarm }

# CAP-M22 — panic button
palette-panic = Panique — couper vers l'écran de confidentialité
panic-banner-title = Panique
panic-banner-body = Le programme affiche l'écran de confidentialité ; tout l'audio est coupé et les captures arrêtées. Le direct et l'enregistrement continuent.
panic-restore = Restaurer…
panic-restore-confirm = Restaurer le programme ?
panic-restore-yes = Restaurer
panic-restore-cancel = Annuler
hotkeys-panic = Panique (écran de confidentialité)
hotkeys-panic-placeholder = p. ex. Ctrl+Shift+F12
hotkeys-timer-toggle = Démarrer/mettre en pause tous les minuteurs
hotkeys-timer-toggle-placeholder = p. ex. Ctrl+Shift+T
hotkeys-timer-reset = Réinitialiser tous les minuteurs
hotkeys-timer-reset-placeholder = p. ex. Ctrl+Shift+0
panic-slate-color = Couleur de l'écran de panique
panic-slate-image = Image de l'écran de panique
panic-slate-image-placeholder = Chemin d'image facultatif

# CAP-M24 — redacted diagnostics bundle
diag-title = Paquet de diagnostic
diag-intro = Exporte un .zip expurgé (instantané de configuration, sonde d'encodeurs, statistiques récentes — jamais de secrets, chemins ni noms) à joindre à la main à un ticket GitHub. Rien n'est envoyé.
diag-preview = Voir le contenu
diag-hide-preview = Masquer l'aperçu
diag-export = Exporter le .zip
diag-exported = Exporté : { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Vérification avant direct
preflight-intro = Chaque point bloquant doit être vert ; le reste est un rappel honnête.
preflight-item-targets = Cibles configurées (clé/URL)
preflight-item-encoder = Un encodeur utilisable est disponible
preflight-item-sources = Toutes les sources saines
preflight-item-disk = Espace disque pour l'enregistrement
preflight-item-mic = Niveau du micro
preflight-item-desktopAudio = Niveau de l'audio du bureau
preflight-item-replay = Tampon de relecture armé
preflight-targets-detail = { $count } activée(s)
preflight-sources-detail = { $count } source(s) en erreur
preflight-disk-detail = ~{ $minutes } min au débit actuel
preflight-fix-stream = Paramètres du stream…
preflight-fix-components = Composants…
preflight-fix-sources = Santé des sources…
preflight-fix-replay = Armer
preflight-optional = facultatif
preflight-hold = Bloquer le direct tant que tout n'est pas vert
preflight-cancel = Annuler
preflight-go-anyway = Passer en direct quand même
preflight-go-live = Passer en direct
