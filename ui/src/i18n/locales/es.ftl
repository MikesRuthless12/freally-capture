# Freally Capture — es
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Modo Estudio
toggle-on = activado
toggle-off = desactivado
stats = Estadísticas
core-ok = núcleo OK
hide-stats-dock = Ocultar el panel de estadísticas
show-stats-dock = Mostrar el panel de estadísticas


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = No se pudo guardar la configuración — el cambio no sobrevivirá a un reinicio.
studio-mode-leave = Salir del Modo Estudio
studio-mode-enter-title = Modo Estudio — edita una escena en la vista previa y pásala al programa con una transición
vertical-canvas-title = El segundo lienzo de salida (vertical 9:16) — se puede grabar y transmitir de forma independiente
app-version = v{ $version }
core-error = núcleo ERROR
core-unreachable = núcleo inaccesible (modo navegador)
connecting-to-core = conectando con el núcleo…
filters-source-fallback = Fuente

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Vista previa del programa
preview-program-output = Salida del programa
preview-canvas-editor = Editor de lienzo
preview-px-to-edge-label = Píxeles hasta los bordes del cuadro
preview-px-to-edge = px al borde Izq { $left } · Sup { $top } · Der { $right } · Inf { $bottom }
preview-program-heading = Programa
preview-no-gpu = No se encontró ningún adaptador de GPU utilizable — el compositor no puede ejecutarse en esta máquina.
preview-starting-compositor = Iniciando el compositor…
preview-empty-scene = Esta escena está vacía — añade una fuente en Fuentes y luego arrástrala, escálala y rótala aquí mismo en el lienzo.
preview-fps = { $fps } fps
preview-dropped = { $dropped } descartados

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Enlace de invitación recibido
remote-join-with-webcam = Unirse con webcam
remote-dismiss = Descartar
remote-hosting-guest = Alojando a un invitado remoto
remote-you-are-guest = Eres un invitado remoto
remote-share-view-title = Comparte tu pantalla con la app del invitado (verá tu vista en directo)
remote-stop-sharing-view = Dejar de compartir vista
remote-share-my-view = Compartir mi vista
remote-allow-center-title = Permite al invitado cambiar qué vista ocupa el centro (mantienes el control y puedes revertirlo cuando quieras)
remote-guest-switching = Cambio por el invitado:
remote-stop-screen = Detener pantalla
remote-share-screen = Compartir pantalla
remote-share-screen-title-guest = Comparte tu pantalla con el anfitrión (se convierte en una fuente que puede centrar)
remote-center-request-label = Solicitud de vista central
remote-center = Centrar
remote-center-cam-title = Pide al anfitrión que centre tu cámara
remote-center-my-cam = Mi cámara
remote-center-screen-title = Pide al anfitrión que centre tu pantalla compartida
remote-center-my-screen = Mi pantalla
remote-center-host-title = Devuelve el centro a la vista del anfitrión
remote-center-host-view = Vista del anfitrión
remote-end-session = Finalizar sesión
remote-leave = Salir
remote-host-view-heading = Vista del anfitrión
remote-host-shared-view-label = La vista compartida del anfitrión
remote-guest-position-label = Posición del invitado
remote-guest-label = Invitado
remote-put-guest = Poner al invitado { $position }
remote-remove-title = Quitar al invitado — puede volver a unirse con el mismo enlace
remote-remove = Quitar
remote-ban-title = Vetar al invitado — lo bloquea e invalida el enlace de invitación
remote-ban = Vetar
remote-guest-self-muted = invitado con micrófono silenciado
remote-unmute-guest = Reactivar el sonido del invitado
remote-mute-guest = Silenciar al invitado
remote-muted-by-host = Silenciado por el anfitrión
remote-unmute-mic = Reactivar micrófono
remote-mute-mic = Silenciar micrófono
remote-waiting-for-host = esperando al anfitrión


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = fuente
sources-fallback-video = vídeo
sources-fallback-error = error
sources-kind-unknown = ?
sources-missing-source = (fuente faltante)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Pantalla
sources-badge-window = Ventana
sources-badge-portal = Portal
sources-badge-camera = Cámara
sources-badge-image = Imagen
sources-badge-media = Multimedia
sources-badge-guest = Invitado
sources-badge-color = Color
sources-badge-text = Texto
sources-badge-scene = Escena
sources-badge-slides = Diapositivas
sources-badge-chat = Chat
sources-badge-audio-in = Entrada de audio
sources-badge-audio-out = Salida de audio
sources-badge-app-audio = Audio de app

# Add-source menu items
sources-add-display = Captura de pantalla
sources-add-window = Captura de ventana
sources-add-game = Captura de juego (lee primero)
sources-add-webcam = Dispositivo de captura de vídeo
sources-add-image = Imagen
sources-add-media = Multimedia (archivo de vídeo/imagen)
sources-add-remote-guest = Invitado remoto (prueba P2P)
sources-add-color = Color
sources-add-text = Texto
sources-add-nested-scene = Escena anidada
sources-add-slideshow = Presentación de imágenes
sources-add-chat-overlay = Superposición de chat en directo
sources-add-audio-input = Captura de entrada de audio
sources-add-audio-output = Captura de salida de audio
sources-add-app-audio = Audio de aplicación (Windows)
sources-add-existing = Fuente existente…

# Panel header + toolbar buttons
sources-panel-title = Fuentes
sources-group-title = Agrupar fuentes — elige dos o más elementos y luego Crear grupo; los elementos agrupados se mueven y se muestran/ocultan juntos
sources-group-aria = Agrupar fuentes
sources-arrange = Organizar: pantalla + esquinas
sources-add-source = Añadir una fuente
sources-browser-source-note = La Fuente de navegador llega como su propio componente bajo demanda (un motor Chromium de ~180 MB — nunca incluido). Por ahora: captura una ventana de navegador real con Captura de ventana + una clave de croma/color, o abre el chat/las alertas como un Dock (Controles → Docks).

# Empty state
sources-empty = No hay fuentes en esta escena — añade una Captura de pantalla, Ventana, Webcam, Imagen, Color o Texto con «+». Arrástralas, escálalas y rótalas en el lienzo; los botones de la derecha reordenan la pila.

# Per-row controls
sources-already-in-group = Ya está en { $name }
sources-pick-for-new-group = Elegir para el nuevo grupo
sources-pick-item-for-group = Elegir { $name } para el nuevo grupo
sources-hide = Ocultar
sources-show = Mostrar
sources-hide-item = Ocultar { $name }
sources-show-item = Mostrar { $name }
sources-unfocus-title = Quitar el foco — restaura el diseño
sources-focus-title = Enfocar — llena el lienzo (Resaltar al que habla)
sources-unfocus-item = Quitar el foco de { $name }
sources-focus-item = Enfocar { $name }
sources-center-title = Centrar — conviértela en la vista central compartida (las cámaras se mueven a la barra)
sources-center-item = Centrar { $name }
sources-rename-item = Renombrar { $name }
sources-in-group = En el grupo { $name }

# Row status + retry
sources-retry-error = Reintentar — { $message }
sources-retry-item = Reintentar { $name }
sources-status-error = estado: error
sources-open-privacy-title = Abre la configuración de privacidad de macOS para este permiso
sources-open-privacy-item = Abrir configuración de privacidad para { $name }
sources-privacy-settings-button = configuración
sources-status-starting = iniciando…
sources-status-live = en directo
sources-status-aria = estado: { $state }

# Media row pause/resume
sources-media-resume-title = Reanudar el vídeo (en directo en la transmisión)
sources-media-pause-title = Pausar el vídeo — congela el fotograma y silencia, en directo en la transmisión
sources-media-resume-item = Reanudar { $name }
sources-media-pause-item = Pausar { $name }

# Hover controls
sources-unlock = Desbloquear
sources-lock = Bloquear
sources-unlock-item = Desbloquear { $name }
sources-lock-item = Bloquear { $name }
sources-raise-title = Subir en la pila
sources-raise-item = Subir { $name }
sources-lower-title = Bajar en la pila
sources-lower-item = Bajar { $name }
sources-filters-title = Filtros y fusión
sources-filters-item = Filtros de { $name }
sources-properties-title = Propiedades
sources-properties-item = Propiedades de { $name }
sources-remove-title = Quitar de esta escena
sources-remove-item = Quitar { $name }

# Grouping footer
sources-create-group = Crear grupo ({ $count })
sources-cancel = Cancelar

# Groups list
sources-groups-aria = Grupos de fuentes
sources-hide-group = Ocultar el grupo
sources-show-group = Mostrar el grupo
sources-item-count = · { $count } elementos
sources-ungroup-title = Desagrupar — los elementos se quedan donde están
sources-ungroup-item = Desagrupar { $name }

# Live Chat Overlay picker
sources-chat-title = Añadir una superposición de chat en directo
sources-chat-youtube-label = YouTube — URL de canal, de vídeo (watch) o de live_chat (sin clave, sin iniciar sesión)
sources-chat-youtube-placeholder = https://www.youtube.com/@tucanal  ·  o una URL watch?v=
sources-chat-twitch-label = Twitch — nombre del canal (lectura anónima, sin cuenta)
sources-chat-twitch-placeholder = tucanal
sources-chat-kick-label = Kick — slug del canal (endpoint público, con el mejor esfuerzo)
sources-chat-kick-placeholder = tucanal
sources-chat-note = Los mensajes aparecen con una marca de tiempo h:mm:ss a. m./p. m. sobre un fondo transparente (por defecto arriba a la derecha; arrástralo a donde quieras). Una avalancha de chat solo envejece las líneas antiguas — nunca puede bloquear la transmisión ni la grabación. El chat de Facebook necesita tu propio token de Graph y aún no está implementado — nunca es obligatorio y nunca condiciona las plataformas anteriores.
sources-chat-add = Añadir superposición de chat
sources-chat-default-name = Chat en directo

# Image Slideshow picker
sources-slideshow-title = Añadir una presentación de imágenes
sources-slideshow-empty = Aún no hay imágenes — Examinar las añade en orden.
sources-slideshow-remove-slide = Quitar diapositiva { $number }
sources-slideshow-browse = Examinar imágenes…
sources-slideshow-per-slide-label = Por diapositiva (ms)
sources-slideshow-crossfade-label = Fundido cruzado (ms, 0 = corte)
sources-slideshow-loop-label = Bucle (desactivado = mantener la última diapositiva)
sources-slideshow-shuffle-label = Mezclar en cada ciclo
sources-slideshow-note = El fundido cruzado combina imágenes del mismo tamaño; los tamaños distintos cortan en seco en el límite (sin reescalado silencioso).
sources-slideshow-add = Añadir presentación ({ $count })

# Nested Scene picker
sources-nested-title = Añadir una escena anidada
sources-nested-empty = No hay otra escena que anidar — añade primero una segunda escena.
sources-nested-scene-name = Escena: { $name }
sources-nested-note = La escena anidada se renderiza en directo al tamaño del lienzo del programa y sigue sus propias ediciones; las transformaciones, los filtros y la fusión se le aplican como a cualquier fuente. Sus fuentes de audio se unen a la mezcla mientras una escena que la muestra sea el programa.

# Display / Window capture picker
sources-capture-display-title = Añadir una captura de pantalla
sources-capture-window-title = Añadir una captura de ventana
sources-capture-looking = Buscando fuentes…
sources-capture-none-displays = No hay nada que capturar aquí — no se encontraron pantallas.
sources-capture-none-windows = No hay nada que capturar aquí — no se encontraron ventanas.
sources-capture-portal-note = En Wayland, el diálogo del sistema elige la pantalla o la ventana — las apps no pueden capturar de forma global ahí, así que ese es el camino honesto (y el único).
sources-capture-window-note = Las vistas previas se actualizan en directo. Una ventana minimizada muestra su último fotograma (o ninguno) hasta que la restauras.
sources-thumb-no-preview = sin vista previa
sources-thumb-loading = cargando…

# Video Capture Device picker
sources-webcam-title = Añadir un dispositivo de captura de vídeo
sources-webcam-looking = Buscando cámaras…
sources-webcam-none = No se encontraron cámaras ni capturadoras.
sources-webcam-format-label = Formato
sources-webcam-format-auto-loading = Automático (cargando formatos…)
sources-webcam-format-auto = Automático (máxima resolución)
sources-webcam-card-presets-label = Preajustes de la capturadora:
sources-webcam-preset-title = Selecciona el modo { $label } que anuncia esta capturadora
sources-webcam-add = Añadir cámara

# Audio Input / Output capture picker
sources-audio-output-title = Añadir una captura de salida de audio
sources-audio-input-title = Añadir una captura de entrada de audio
sources-audio-default-output = Salida predeterminada (lo que oyes)
sources-audio-default-input = Entrada predeterminada
sources-audio-looking = Buscando dispositivos de audio…
sources-audio-none-output = No se encontró aquí ningún dispositivo de captura de audio de escritorio.
sources-audio-none-input = No se encontraron micrófonos ni entradas de línea.
sources-audio-input-note = Las tiras del mezclador reciben un vúmetro, un fader, silencio, monitorización, filtros (reducción de ruido, puerta, compresor…) y asignación de pista. Todo permanece en esta máquina.

# Application Audio picker
sources-appaudio-title = Añadir audio de aplicación
sources-appaudio-looking = Buscando apps que emitan sonido…
sources-appaudio-none = Ninguna app está emitiendo sonido ahora mismo — inicia la reproducción en la app y luego actualiza.
sources-appaudio-refresh = ⟳ Actualizar
sources-appaudio-note = Captura exactamente el audio de esa app — con su propio vúmetro, fader, silencio, filtros y pista.

# Game Capture picker
sources-game-title = Captura de juego
sources-game-checking = Comprobando…
sources-game-use-portal = Usar Captura de pantalla (Portal)
sources-game-use-window = Usar Captura de ventana en su lugar

# Image picker
sources-image-title = Añadir una imagen
sources-image-file-label = Archivo de imagen (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Añadir imagen

# Path field
sources-browse = Examinar…

# Media picker
sources-media-title = Añadir multimedia
sources-media-file-label = Archivo multimedia (mp4, mkv, webm, mov, .frec o una imagen)
sources-media-loop-label = Bucle (reiniciar desde el principio al terminar)
sources-media-note = .frec se reproduce a través del códec propio freally-video — no hay nada que descargar. Los formatos de distribución (mp4/mkv/webm/…) se decodifican mediante el componente FFmpeg bajo demanda; su audio llega al mezclador como su propia tira.
sources-media-add = Añadir multimedia

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 hora
sources-ttl-1day = 1 día

# Remote Guest form
sources-remote-copy-failed = no se pudo copiar — selecciona el enlace y cópialo manualmente
sources-remote-join-failed = fallo al unirse: { $error }
sources-remote-title = Invitado remoto (prueba P2P)
sources-remote-host-heading = Anfitrión — invita a un invitado
sources-remote-start-hosting = Empezar a alojar
sources-remote-expires-label = Caduca
sources-remote-invite-expiry-aria = Caducidad de la invitación
sources-remote-invite-link-aria = Enlace de invitación
sources-remote-copied = Copiado ✓
sources-remote-copy = Copiar
sources-remote-share-note = Comparte este enlace (Discord / mensaje / correo). Lleva tu sesión y caduca según lo configurado. El invitado lo abre y se une con su webcam.
sources-remote-qr-note = Escanéalo en un móvil para unirte directamente desde el navegador — cámara + micrófono, sin instalar nada. El enlace freally:// copiable de arriba se abre en Freally Capture en una máquina que lo tenga.
sources-remote-guest-heading = Invitado — únete con una invitación
sources-remote-paste-placeholder = pega el enlace de invitación
sources-remote-invite-input-aria = Enlace de invitación o id de sesión
sources-remote-join = Unirse con webcam
sources-remote-session-note = Los controles de la sesión en directo (silenciar, finalizar) permanecen en la barra de la parte superior de la ventana principal — puedes cerrar este diálogo.
sources-remote-stop-session = Detener sesión

# Invite QR
sources-invite-qr-aria = Código QR del enlace de invitación

# Remote device pickers
sources-devices-output-unavailable = enrutamiento de salida no disponible — reproduciendo en el dispositivo predeterminado
sources-devices-mic-test-failed = fallo en la prueba de micrófono: { $error }
sources-devices-heading = Dispositivos de audio de la sesión
sources-devices-microphone-label = Micrófono
sources-devices-microphone-aria = Micrófono de la sesión
sources-devices-system-default = Predeterminado del sistema
sources-devices-output-label = Salida
sources-devices-output-aria = Salida de audio de la sesión
sources-devices-stop-test = Detener prueba
sources-devices-test = Probar — escúchate
sources-devices-testing-note = habla al micrófono — estás escuchando en directo los dispositivos seleccionados
sources-devices-idle-note = envía tu micrófono a la salida en bucle (los auriculares evitan el acople)

# TURN relay section
sources-turn-save-failed = no se pudo guardar: { $error }
sources-turn-summary = Red — relé TURN opcional (avanzado)
sources-turn-note-1 = Las sesiones se conectan directamente (P2P) — gratis, sin necesidad de relé. Si AMBOS lados están tras NAT estrictos, la ruta directa puede fallar; entonces un relé TURN que ejecutes tú mismo transporta el medio. Omitir esto está bien — la mayoría de las conexiones funcionan solo en modo directo.
sources-turn-note-2 = Opción gratuita: Oracle Cloud «Always Free» ejecuta coturn sin coste (nota: Oracle pide una tarjeta de crédito al registrarte, pero la instancia Always-Free sigue siendo gratuita). Pasos: 1) crea la VM gratuita, 2) instala coturn, 3) abre el UDP 3478, 4) define un usuario/contraseña, 5) introduce aquí turn:your-vm-ip:3478 + las credenciales. Tu credencial permanece en tu archivo de configuración local y nunca se registra.
sources-turn-url-label = URL de TURN
sources-turn-url-placeholder = turn:host:3478 (vacío = solo directo)
sources-turn-url-aria = URL de TURN
sources-turn-username-label = Usuario
sources-turn-username-aria = Usuario de TURN
sources-turn-credential-label = Credencial
sources-turn-credential-aria = Credencial de TURN
sources-turn-note-3 = El relé se activa una vez definidos los tres campos (un servidor TURN requiere las credenciales) y se aplica a la próxima sesión que inicies o a la que te unas. Verifícalo con una llamada de prueba solo con relé entre tus dos máquinas.
sources-turn-settings-unavailable = configuración no disponible (modo navegador)

# Color picker
sources-color-title = Añadir un color
sources-color-label = Color
sources-color-width-label = Anchura
sources-color-height-label = Altura
sources-color-add = Añadir color

# Text picker
sources-text-title = Añadir texto
sources-text-label = Texto
sources-text-default = Texto
sources-text-color-label = Color
sources-text-color-aria = Color del texto
sources-text-size-label = Tamaño (px)
sources-text-note = La familia tipográfica, la alineación, el ajuste de línea y el RTL están en las Propiedades de la fuente. La Noto Sans incluida (con árabe/hebreo) es la predeterminada — idéntica en cada máquina.
sources-text-add = Añadir texto

# Existing source picker
sources-existing-title = Añadir una fuente existente
sources-existing-empty = Aún no existe ninguna fuente — añade una a cualquier escena primero. Las fuentes existentes se comparten: renombrar o reconfigurar una actualiza todas las escenas que la muestran.

# Screen + corners layout
sources-slot-off = Desactivado
sources-slot-center = Centro (pantalla)
sources-slot-top-left = Superior izquierda
sources-slot-top-right = Superior derecha
sources-slot-bottom-left = Inferior izquierda
sources-slot-bottom-right = Inferior derecha
sources-layout-title = Organizar: pantalla + esquinas
sources-layout-empty = Añade primero una captura de pantalla y una o más cámaras a esta escena, y luego organízalas aquí.
sources-layout-note = Pon una pantalla en el centro y hasta cuatro cámaras en las esquinas — tu diseño de explicación / pódcast. Cada esquina admite una webcam, una ventana de llamada capturada o un clip multimedia. Después puedes arrastrar cualquiera de ellas por el lienzo.
sources-layout-slot-aria = Ranura para { $name }
sources-layout-apply = Aplicar diseño


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Controles
controls-start-stop-title-stop = Detener y finalizar la grabación
controls-start-stop-title-start = Graba la señal del programa con la configuración de Ajustes → Salida
controls-finalizing = ◌ Finalizando…
controls-stop-recording = ■ Detener grabación
controls-start-recording = ● Iniciar grabación
controls-marker-title = Coloca un marcador de capítulo en este momento — cae en la GRABACIÓN (capítulos mkv, o un archivo adjunto). Los marcadores de transmisión del lado de la plataforma necesitan cuentas de plataforma, algo que esta app nunca pide.
controls-marker = ◈ Marcador
controls-pause-title-resume = Reanudar — el archivo continúa como una única línea de tiempo contigua
controls-pause-title-pause = Pausar — no se escriben fotogramas; al reanudar continúa el mismo archivo reproducible
controls-resume-recording = ▶ Reanudar grabación
controls-pause-recording = ⏸ Pausar grabación
controls-reactions-label = Reacciones (integradas en el programa)
controls-reactions-title = Haz flotar una reacción sobre el programa — grabada Y transmitida, para que la repetición muestre el momento exacto. Los espectadores del chat también las activan (su emoji de reacción flota automáticamente); una avalancha solo limita lo que hay en pantalla.
controls-react = Reaccionar { $emoji }
controls-virtual-camera-title = La cámara virtual necesita su propio componente de controlador firmado por SO (Win11 MFCreateVirtualCamera / Win10 DirectShow / extensión CoreMediaIO de macOS / v4l2loopback de Linux) — llega como su propio hito. El modelo de señal está listo para ella: programa, lienzo vertical o una sola fuente, con un micrófono virtual emparejado en Windows/Linux (macOS no tiene API de micrófono virtual — dicho con honestidad).
controls-virtual-camera = ⌁ Iniciar cámara virtual
controls-files-title = Grabaciones terminadas + la acción de remultiplexar a mp4
controls-files = ▤ Archivos…
controls-output-title = Formato de grabación, codificador, carpeta, pistas y división
controls-output = ⚙ Salida…
controls-stream-title = Objetivo de En vivo: servicio, clave de transmisión, codificador, tasa de bits
controls-stream = ⦿ Transmitir…
controls-codecs-title = El componente ffmpeg de códecs de distribución bajo demanda (claramente etiquetado, nunca incluido)
controls-codecs = ⬡ Códecs…
controls-replay-title = Duración del búfer de repetición + preajustes de calidad
controls-replay = ⟲ Repetición…
controls-keys-title = Atajos globales: grabar, En vivo, transición, guardar repetición
controls-keys = ⌨ Teclas…
controls-scripts-title = Scripts de Lua en entorno aislado: reacciona a eventos de en vivo/escena/grabación, controla el estudio
controls-scripts = ⚡ Scripts…
controls-docks-title = Docks de navegador: abre una ventana emergente de chat, una página de alertas o botones de Companion como una ventana junto al estudio
controls-docks = ⧉ Docks…
controls-remote-title = API remota WebSocket para controladores Stream Deck / Companion (desactivada por defecto)
controls-remote = ⌁ Remoto…
controls-profiles-title = Perfiles (configuración) + colecciones de escenas — instantáneas intercambiables
controls-profiles = ▣ Perfiles…
controls-bug-title = Informar de un error — anónimo, voluntario (nada se envía automáticamente)
controls-bug = 🐞 Informar de un error…
controls-updates-title = Buscar actualizaciones — firmadas, verificadas, nada se descarga sin un clic
controls-updates = ⭳ Buscar actualizaciones…
controls-saved = Guardado: { $path }

# --- MixerDock.tsx ---
mixer-title = Mezclador de audio
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Cambiar a tiras horizontales
mixer-switch-to-vertical = Cambiar a tiras verticales
mixer-layout-aria-vertical = Disposición del mezclador: vertical — cambiar a horizontal
mixer-layout-aria-horizontal = Disposición del mezclador: horizontal — cambiar a vertical
mixer-empty = No hay fuentes de audio en esta escena — añade una Captura de entrada de audio (micrófono) o una Captura de salida de audio (audio de escritorio) con «+» en Fuentes. Las tiras reciben un vúmetro, un fader, silencio, monitorización, filtros y asignación de pista.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Sonoridad del programa (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Sonoridad momentánea (400 ms)
mixer-short-term-title = Sonoridad a corto plazo (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Dispositivo de salida del monitor
mixer-default-output = Salida predeterminada

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memoria
stats-dropped = Descartados
stats-render = Renderizado
stats-gpu = GPU
stats-gpu-compositing = componiendo
stats-gpu-idle = inactiva
stats-vertical-fps = 9:16 FPS
stats-targets-label = Destinos de transmisión
stats-shared-encode = · codificación compartida
stats-starting = Iniciando el compositor…

# --- ScenesRail.tsx ---
scenes-title = Escenas
scenes-new-scene-name = Escena
scenes-add = Añadir una escena
scenes-empty = Conectando con el núcleo del estudio…
scenes-rename = Renombrar { $name }
scenes-on-program = En el programa
scenes-preview = Previsualizar { $name }
scenes-switch-to = Cambiar a { $name }
scenes-move-up = Subir
scenes-move-up-aria = Subir { $name }
scenes-move-down = Bajar
scenes-move-down-aria = Bajar { $name }
scenes-last-stays = La última escena permanece
scenes-remove = Quitar esta escena
scenes-remove-aria = Quitar { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Nivel
channelstrip-monitor-off = Monitor desactivado
channelstrip-monitor-only = Solo monitor (no en la mezcla)
channelstrip-monitor-and-output = Monitor y salida
channelstrip-status-error = error
channelstrip-status-live = en directo
channelstrip-status-waiting-audio = esperando audio
channelstrip-status = estado: { $state }
channelstrip-status-waiting = esperando
channelstrip-mute = Silenciar
channelstrip-unmute = Reactivar sonido
channelstrip-mute-source = Silenciar { $name }
channelstrip-unmute-source = Reactivar el sonido de { $name }
channelstrip-scene-mix-on = Mezcla por escena ACTIVADA — esta tira anula la mezcla global para esta escena (haz clic para volver a seguir la mezcla global)
channelstrip-scene-mix-off = Mezcla por escena — da a esta tira su propio fader/silencio para la escena actual
channelstrip-scene-mix-label = Mezcla por escena para { $name }
channelstrip-monitor-cycle = { $mode } — haz clic para alternar
channelstrip-monitor-mode = Modo de monitor de { $name }: { $mode }
channelstrip-audio-filters-title = Filtros de audio (reducción de ruido, puerta, compresor…)
channelstrip-audio-filters-label = Filtros de audio para { $name }
channelstrip-advanced-title = Desfase de sincronización y atajos de pulsar para hablar
channelstrip-advanced-label = Ajustes de audio avanzados para { $name }
channelstrip-track-assignment = Asignación de pista
channelstrip-track = Pista { $n }
channelstrip-track-assigned = Pista { $n } (asignada)
channelstrip-track-label = Pista { $n } para { $name }
channelstrip-device-error = error de dispositivo
channelstrip-audio-device-error = error de dispositivo de audio
channelstrip-volume-label = Volumen de { $name } en decibelios
channelstrip-ptt-hold = Pulsar para hablar: mantén { $key }
channelstrip-sync-offset = Desfase de sincronización (ms, 0–{ $max } — retrasa este audio)
channelstrip-ptt-hotkey = Atajo de pulsar para hablar (silencioso salvo que se mantenga)
channelstrip-ptt-placeholder = p. ej. Ctrl+Shift+T o F13
channelstrip-ptt-aria = Atajo de pulsar para hablar
channelstrip-ptm-hotkey = Atajo de pulsar para silenciar (silencioso mientras se mantiene)
channelstrip-ptm-placeholder = p. ej. Ctrl+Shift+M
channelstrip-ptm-aria = Atajo de pulsar para silenciar
channelstrip-hotkeys-note = Los atajos funcionan mientras otras apps están enfocadas. En Linux/Wayland, los atajos globales pueden no estar disponibles — es un límite del compositor, dicho con honestidad.
channelstrip-apply = Aplicar


# --- LiveButton.tsx ---
livebutton-failure-ended = la transmisión finalizó
livebutton-title-live = Finaliza la transmisión — todos los destinos (una grabación en curso continúa)
livebutton-title-offline = Ponte en vivo en cada destino activado en Ajustes → Transmisión
livebutton-end-stream = ■ Finalizar transmisión
livebutton-aria-reconnecting = Reconectando
livebutton-aria-live = En directo
livebutton-badge-retry = reintento { $n }
livebutton-badge-live = en directo
livebutton-go-live = ⦿ En vivo


# --- RecDot.tsx ---
recdot-paused-aria = Grabación en pausa
recdot-recording-aria = Grabando
recdot-tracks-one = grabando { $count } pista de audio
recdot-tracks-other = grabando { $count } pistas de audio
recdot-paused = en pausa


# --- ReplayControls.tsx ---
replaycontrols-saved = Repetición guardada — { $name }
replaycontrols-failure-stopped = el búfer se detuvo
replaycontrols-title-disarm = Desarmar el búfer de repetición (descarta el historial no guardado)
replaycontrols-title-arm = Armar el búfer de repetición continuo — mantiene los últimos N segundos listos para guardar (su propia codificación ligera; la transmisión y la grabación no se tocan)
replaycontrols-replay-seconds = ⟲ Repetición { $seconds } s
replaycontrols-arm = ⟲ Armar búfer de repetición
replaycontrols-save-title = Guarda los últimos N segundos en la carpeta de grabaciones (también con el atajo Guardar repetición)
replaycontrols-save = ⤓ Guardar


# --- PropertiesDialog.tsx ---
properties-title = Propiedades — { $name }
properties-name = Nombre
properties-cancel = Cancelar
properties-apply = Aplicar
properties-youtube = YouTube — URL de canal / vídeo (watch) / live_chat (sin clave, sin iniciar sesión, nunca)
properties-twitch = Twitch — nombre del canal (anónimo)
properties-kick = Kick — slug del canal (endpoint público)
properties-width-px = Anchura (px)
properties-lines = Líneas
properties-font-px = Fuente (px)
properties-images = Archivos de imagen (una ruta por línea, en orden)
properties-per-slide = Por diapositiva (ms)
properties-crossfade = Fundido cruzado (ms, 0 = corte)
properties-loop-slideshow = Bucle (desactivado = mantener la última diapositiva)
properties-shuffle = Mezclar en cada ciclo
properties-nested-scene = Escena que compone esta fuente (se rechaza una escena que ya contiene esta)
properties-portal-note = El portal ScreenCast de Wayland elige la pantalla o la ventana en el diálogo del sistema cada vez que se inicia esta fuente — aquí no hay nada que configurar, por diseño.
properties-appaudio-capturing = Capturando audio de { $exe }
properties-appaudio-exe-fallback = una aplicación
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Vuelve a añadir la fuente para apuntar a otra app (un id de proceso cambia cuando la app se reinicia).
properties-image-file = Archivo de imagen
properties-media-file = Archivo multimedia (mp4, mkv, webm, mov, .frec o una imagen)
properties-media-loop = Bucle (reiniciar desde el principio al terminar)
properties-media-hwdecode = Decodificación por hardware (recurre al software por su cuenta)
properties-media-note = .frec se reproduce a través del códec propio freally-video — no hay nada que descargar. Otros formatos de vídeo se decodifican mediante el componente FFmpeg bajo demanda. El audio del archivo obtiene su propia tira en el mezclador; el desfase de sincronización de la tira ajusta con precisión la alineación A/V. Un clip sin audio deja su tira en silencio.
properties-color = Color
properties-width = Anchura
properties-height = Altura
properties-text = Texto
properties-font-family = Familia tipográfica (del sistema; en blanco = predeterminada)
properties-size-px = Tamaño (px)
properties-text-color = Color del texto
properties-align = Alinear
properties-align-left = izquierda
properties-align-center = centro
properties-align-right = derecha
properties-line-spacing = Interlineado
properties-wrap-width = Ancho de ajuste (px; 0 = desactivado)
properties-force-rtl = Forzar de derecha a izquierda
properties-text-note = El renderizado usa modelado real (unión árabe, ligaduras) y ordenación de líneas bidi. La familia Noto Sans incluida (con árabe/hebreo) es la predeterminada; las familias del sistema también funcionan. CJK usa fuentes del sistema por ahora.
properties-repick-capturing = Capturando: { $label }
properties-repick-looking = Buscando fuentes…
properties-repick-none-displays = No se encontraron pantallas para volver a elegir.
properties-repick-none-windows = No se encontraron ventanas para volver a elegir.
properties-repick-again = Volver a elegir:
properties-device = Dispositivo
properties-video-current-device = (dispositivo actual)
properties-format = Formato
properties-format-auto-loading = Automático (cargando formatos…)
properties-format-auto = Automático (máxima resolución)
properties-audio-capture-of = Capturar el audio de
properties-audio-default-output = Salida predeterminada (lo que oyes)
properties-audio-default-input = Entrada predeterminada
properties-audio-default-suffix = (predeterminado)
properties-audio-current-device = (dispositivo actual: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Ganancia
audiofilters-name-noise-gate = Puerta de ruido
audiofilters-name-compressor = Compresor
audiofilters-name-limiter = Limitador
audiofilters-name-eq = Ecualizador de 3 bandas
audiofilters-name-denoise = Reducción de ruido
audiofilters-name-ducking = Atenuación automática
audiofilters-title = Filtros de audio — { $name }
audiofilters-chain-header = Cadena de filtros (el primero se ejecuta antes, previo al fader)
audiofilters-add = + Añadir filtro
audiofilters-add-menu = Añadir un filtro de audio
audiofilters-empty = Aún no hay filtros — reduce el ruido de un micrófono (DSP clásico, sin ML), pon una puerta a la sala, doma los picos con el compresor o atenúa la música bajo tu voz.
audiofilters-enable = Activar { $name }
audiofilters-run-earlier = Ejecutar antes
audiofilters-move-up = Subir { $name }
audiofilters-run-later = Ejecutar después
audiofilters-move-down = Bajar { $name }
audiofilters-remove-title = Quitar filtro
audiofilters-remove = Quitar { $name }
audiofilters-gain-db = Ganancia (dB)
audiofilters-open-db = Abrir a (dB)
audiofilters-close-db = Cerrar a (dB)
audiofilters-attack-ms = Ataque (ms)
audiofilters-hold-ms = Retención (ms)
audiofilters-release-ms = Liberación (ms)
audiofilters-ratio = Relación (:1)
audiofilters-threshold-db = Umbral (dB)
audiofilters-output-gain-db = Ganancia de salida (dB)
audiofilters-ceiling-db = Techo (dB)
audiofilters-low-db = Graves (dB)
audiofilters-mid-db = Medios (dB)
audiofilters-high-db = Agudos (dB)
audiofilters-strength = Intensidad
audiofilters-denoise-note = Supresión espectral propia de DSP clásico — el ruido constante (ventiladores, siseo) baja mientras la voz pasa. Sin ML, sin modelos, según la carta fundacional.
audiofilters-duck-under = Atenuar bajo
audiofilters-ducking-trigger = Fuente que activa la atenuación
audiofilters-pick-trigger = (elige un disparador — p. ej. tu micrófono)
audiofilters-trigger-at-db = Activar a (dB)
audiofilters-duck-by-db = Atenuar en (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Clave de croma
filters-name-color-key = Clave de color
filters-name-luma-key = Clave de luminancia
filters-name-render-delay = Retardo de renderizado
filters-name-color-correction = Corrección de color
filters-name-lut = Aplicar LUT
filters-name-blur = Desenfoque
filters-name-mask = Máscara de imagen
filters-name-sharpen = Nitidez
filters-name-scroll = Desplazamiento
filters-name-crop = Recorte
filters-title = Filtros — { $name }
filters-blend-mode = Modo de fusión
filters-chain-header = Cadena de filtros (el primero se ejecuta antes)
filters-add = + Añadir filtro
filters-add-menu = Añadir un filtro
filters-empty = Aún no hay filtros — aplica clave de croma a una webcam, corrige el color de una captura o desplaza un rótulo.
filters-enable = Activar { $name }
filters-run-earlier = Ejecutar antes
filters-move-up = Subir { $name }
filters-run-later = Ejecutar después
filters-move-down = Bajar { $name }
filters-remove-title = Quitar filtro
filters-remove = Quitar { $name }
filters-key-color-rgb = Color clave (cualquier color, distancia RGB)
filters-similarity = Similitud
filters-smoothness = Suavidad
filters-luma-min = Luminancia mínima (excluye los tonos más oscuros)
filters-luma-max = Luminancia máxima (excluye los tonos más claros)
filters-delay = Retardo (ms — solo vídeo, p. ej. para sincronizar con el audio; limitado a 500)
filters-key-color = Color clave
filters-spill = Derrame
filters-gamma = Gamma
filters-brightness = Brillo
filters-contrast = Contraste
filters-saturation = Saturación
filters-hue-shift = Cambio de tono
filters-opacity = Opacidad
filters-cube-file = archivo .cube
filters-amount = Cantidad
filters-radius = Radio
filters-mask-image = Imagen de máscara
filters-mask-mode = Modo
filters-mask-alpha = alfa
filters-mask-luma = luminancia
filters-mask-invert = invertir
filters-speed-x = Velocidad X (px/s)
filters-speed-y = Velocidad Y (px/s)
filters-crop-left = izquierda
filters-crop-top = arriba
filters-crop-right = derecha
filters-crop-bottom = abajo
filters-crop-aria = recortar { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Actualizar
pickershell-refresh-title = Actualizar la lista
pickershell-close = Cerrar


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Informar de un error
bugreport-intro = Los informes son anónimos y voluntarios — nada se envía automáticamente. Revisarás el texto exacto de abajo y luego lo enviarás mediante una incidencia de GitHub prerrellenada o tu app de correo. Sin datos personales (tu ruta de inicio y tu nombre de usuario se ocultan); sin cuenta, sin servidor.
bugreport-crash-notice = Freally Capture se cerró inesperadamente en una ejecución anterior — los detalles anónimos del fallo se incluyen abajo. Informar de ellos ayuda a corregirlo rápido.
bugreport-description-label = ¿Qué estabas haciendo cuando ocurrió? (opcional)
bugreport-description-placeholder = p. ej. la vista previa se congeló al añadir una segunda webcam
bugreport-include-crash = Incluir los detalles anónimos del fallo de la última ejecución
bugreport-preview-label = Exactamente lo que se enviará
bugreport-open-github = Abrir incidencia en GitHub
bugreport-gmail-title = Abre la ventana de redacción de Gmail en tu navegador, prerrellenada. ¿Sesión cerrada? Google muestra primero su pantalla de inicio de sesión.
bugreport-compose-gmail = Redactar en Gmail
bugreport-email-title = Abre un borrador en la app de correo que este PC usa por defecto (Outlook, Thunderbird, Mail…)
bugreport-send-email = Enviar correo
bugreport-copied = Copiado ✓
bugreport-copy-report = Copiar informe
bugreport-dismiss-crash = Descartar fallo
bugreport-copy-failed = no se pudo copiar — selecciona el texto y cópialo manualmente
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = QUÉ OCURRIÓ
bugreport-preview-no-description = (no se proporcionó descripción)
bugreport-preview-diagnostics = DIAGNÓSTICOS ANÓNIMOS (sin datos personales)
bugreport-preview-from = De: Freally Capture
bugreport-preview-crash-excerpt = --- extracto del fallo ---


# --- Updates.tsx ---
updates-title = Actualización de software
updates-checking = Buscando actualizaciones…
updates-uptodate = Tienes la última versión.
updates-check-again = Volver a comprobar
updates-available = La versión { $version } está disponible
updates-current-version = (tienes { $current })
updates-release-notes-label = Versión { $version } — Notas de la versión
updates-confirm = ¿Quieres actualizar ahora? La descarga se verifica con la clave de firma incluida antes de aplicarse. Freally Capture se cierra, el instalador se ejecuta y la nueva versión se reabre sola.
updates-yes-update-now = Sí, actualizar ahora
updates-no-not-now = No, ahora no
updates-downloading = Descargando { $version }…
updates-starting = iniciando…
updates-installed = Actualización instalada.
updates-restart-now = Reiniciar ahora
updates-restart-later = Reiniciar más tarde
updates-try-again = Reintentar


# --- Models.tsx ---
models-title = Componentes
models-ffmpeg-heading = FFmpeg — códecs de distribución
models-badge-third-party = De terceros · no incluido
models-ffmpeg-desc = El motor propio de Freally Capture graba freally-video (.frec) sin pérdidas sin nada adicional. Grabar los formatos de distribución que las plataformas y los reproductores esperan — H.264/AAC (y HEVC/AV1) en mp4/mkv/mov/webm — usa FFmpeg, una herramienta aparte con la que esta app nunca se distribuye: esos códecs están sujetos a patentes, así que sigue siendo opcional y claramente etiquetado. Se descarga bajo demanda desde la compilación fijada de abajo, se verifica con SHA-256 antes del primer uso, se almacena en caché por usuario y se ejecuta como un proceso aparte. Su licencia (LGPL/GPL) es la suya propia — consulta THIRD-PARTY-NOTICES.
models-checking = Comprobando…
models-ffmpeg-not-installed = No instalado. Disponible: FFmpeg { $version } de { $source } (descarga de { $size }).
models-ffmpeg-none-pinned = Aún no hay ninguna compilación de FFmpeg fijada para esta plataforma — la grabación con códecs de distribución no está disponible aquí. La grabación freally-video sin pérdidas no se ve afectada.
models-ffmpeg-download-verify = Descargar y verificar ({ $size })
models-downloading = Descargando…
models-download-of = de
models-cancel = Cancelar
models-ffmpeg-verifying = Verificando la descarga con el SHA-256 fijado…
models-ffmpeg-extracting = Descomprimiendo…
models-ffmpeg-ready = Instalado y verificado — { $version }
models-remove = Quitar
models-ffmpeg-retry = Reintentar descarga
models-network-note = La descarga es la única acción de red de este panel y nunca se inicia sola. Una suma de comprobación fallida aborta la instalación — la app se niega a ejecutar bytes de los que no puede responder.
models-cef-heading = Entorno de ejecución de la Fuente de navegador — Chromium (CEF)
models-cef-desc = Las fuentes de navegador renderizan páginas web (alertas, widgets, superposiciones) mediante Chromium Embedded Framework — un entorno de ejecución de ~100 MB con el que esta app nunca se distribuye. Se descarga bajo demanda desde el índice oficial de compilaciones de CEF, se verifica con el SHA-1 de ese índice antes de descomprimir nada y se almacena en caché por usuario. La fuente de navegador que se renderiza con él llega con su propio hito; esto instala el entorno de ejecución que necesita.
models-cef-download-install = Descargar e instalar
models-cef-unsupported = CEF no publica ninguna compilación para esta plataforma — las fuentes de navegador no están disponibles aquí.
models-cef-resolving = Resolviendo la última compilación estable…
models-cef-verifying = Verificando la descarga con el SHA-1 del índice…
models-cef-extracting = Descomprimiendo el entorno de ejecución…
models-cef-ready = Instalado — CEF { $version }.
models-cef-retry = Reintentar
models-integrations-heading = Integraciones opcionales
models-badge-never-bundled = Nunca incluido
models-ndi-detected = Detectado
models-ndi-not-installed = No instalado
models-vst-available = Disponible
models-vst-not-available = No disponible


# --- Recordings.tsx ---
recordings-title = Grabaciones
recordings-loading = Leyendo la carpeta…
recordings-empty = Aún no hay grabaciones — Iniciar grabación escribe en la carpeta configurada en Salida.
recordings-frec-label = propio sin pérdidas (freally-video)
recordings-remux-title = Remultiplexar como mp4 — copia directa del flujo, sin recodificar, sin cambio de calidad (necesita el componente FFmpeg)
recordings-remuxing = Remultiplexando…
recordings-remux-to-mp4 = Remultiplexar a MP4
recordings-export-mp4-title = Decodifica el .frec propio y recodifica a MP4 (H.264/AAC) para que se reproduzca en cualquier reproductor — necesita el componente FFmpeg
recordings-exporting = Exportando…
recordings-export-mp4 = Exportar → MP4
recordings-export-mkv-title = Decodifica el .frec propio y recodifica a MKV para que se reproduzca en cualquier reproductor
recordings-starting = iniciando…
recordings-frames = { $done } / { $total } fotogramas
recordings-cancel = Cancelar
recordings-export-cancelled = Exportación cancelada.
recordings-exported-to = Exportado a { $path }
recordings-remuxed-to = Remultiplexado a { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Abrir grabación .frec
openfrec-desc = Freally Capture graba el formato propio .frec sin pérdidas — no lo reproduce. Freally Player reproducirá .frec directamente cuando se lance. Por ahora, expórtalo a MP4/MKV y se reproduce en cualquier reproductor (VLC, el reproductor de tu SO, lo que sea).
openfrec-exported-to = Exportado a { $path }
openfrec-exporting = Exportando…
openfrec-starting = iniciando…
openfrec-export-mp4 = Exportar → MP4
openfrec-export-mkv = Exportar → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Lienzo vertical (9:16)
vertical-enable = Activar el segundo lienzo — se puede grabar y transmitir de forma independiente del programa
vertical-scene-label = Escena que compone este lienzo
vertical-width = Anchura
vertical-height = Altura
vertical-preview-alt = Vista previa del lienzo vertical
vertical-note = Las posiciones de los elementos son exactas al píxel entre lienzos: selecciona esta escena en la barra de Escenas para organizarla mientras esta vista previa muestra el resultado vertical. Los destinos de transmisión eligen este lienzo en ⦿ Transmitir…; Ajustes → Salida puede grabarlo junto al archivo principal.
vertical-close = Cerrar


# --- EulaGate.tsx ---
eula-title = Freally Capture — Acuerdo de licencia
eula-version = v{ $version }
eula-intro = Lee y acepta este acuerdo para usar Freally Capture. En resumen: es una herramienta neutral, y eres el único responsable de lo que capturas, grabas y emites — y de tener los derechos sobre ello.
eula-thanks = Gracias por leer.
eula-scroll-hint = Desplázate hasta el final para continuar.
eula-decline = Rechazar y salir
eula-agree = Acepto


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Salida
output-loading = La configuración aún se está cargando…
output-container-frec = freally-video (.frec) — sin pérdidas, propio, nada que descargar
output-container-mkv = MKV — tolerante a fallos; remultiplexa a mp4 después
output-container-mp4 = MP4 — se reproduce en todas partes
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Sin pérdidas
output-preset-lossless-title = El códec propio freally-video — exacto al bit, sin descarga
output-preset-high-label = Alta calidad
output-preset-high-title = MP4, mejor codificador detectado, CQ 16 casi sin pérdidas, preajuste Calidad
output-preset-balanced-label = Equilibrado
output-preset-balanced-title = MKV, mejor codificador detectado, CQ 23, preajuste Equilibrado
output-recording-format = Formato de grabación
output-ffmpeg-warning = Este formato necesita el componente FFmpeg (códecs de distribución — no incluidos). El .frec sin pérdidas no necesita nada.
output-install = Instalar…
output-recordings-folder = Carpeta de grabaciones
output-folder-placeholder = Carpeta de Vídeos del SO
output-filename-prefix = Prefijo del nombre de archivo
output-frame-rate = Frecuencia de fotogramas
output-fps-option = { $fps } fps
output-split-every = Dividir cada (minutos, 0 = desactivado)
output-output-width = Anchura de salida (0 = lienzo; solo formatos de distribución)
output-output-height = Altura de salida (0 = lienzo)
output-record-vertical = Grabar también el lienzo vertical (un archivo paralelo «… (vertical)»; necesita el lienzo 9:16 activado)
output-audio-tracks = Pistas de audio
output-recorded-tracks-group = Pistas grabadas
output-track-last-one = Al menos una pista debe grabar
output-record-track-on = Grabar pista { $index }: activada
output-record-track-off = Grabar pista { $index }: desactivada
output-encoder-heading = Codificador
output-video-encoder = Codificador de vídeo
output-encoder-auto = Automático — mejor detectado (H.264)
output-encoder-unavailable = — no disponible aquí
output-preset = Preajuste
output-preset-quality = Calidad
output-preset-balanced-option = Equilibrado
output-preset-performance = Rendimiento
output-rate-control = Control de tasa
output-rc-cqp = CQP (calidad constante)
output-rc-cbr = CBR (tasa de bits constante)
output-rc-vbr = VBR (tasa de bits variable)
output-cq = CQ (0–51, menor = mejor)
output-bitrate = Tasa de bits (kbps)
output-keyframe = Intervalo de fotogramas clave (s)
output-audio-bitrate = Tasa de bits de audio (kbps / pista)
output-presets = Preajustes:

# --- SettingsStream.tsx ---
stream-title = Ajustes — Transmisión
stream-target-enabled = Destino { $index } activado
stream-target = Destino { $index }
stream-remove = Quitar
stream-service = Servicio
stream-canvas = Lienzo
stream-canvas-main = Principal (programa)
stream-canvas-vertical = Vertical (9:16 — actívalo en el estudio)
stream-ingest-srt = URL de ingesta SRT
stream-ingest-whip = URL del endpoint WHIP
stream-ingest-url = URL de ingesta
stream-ingest-override = (anular — vacío = el preajuste del servicio)
stream-key-srt = streamid (opcional — añadido como ?streamid=…; tratado como secreto)
stream-key-whip = Token Bearer (opcional — enviado como la cabecera Authorization; un secreto)
stream-key-custom = Clave de transmisión (de tu servidor — tratada como secreto)
stream-key-service = Clave de transmisión (de tu panel de creador — tratada como secreto)
stream-key-aria = Clave de transmisión { $index }
stream-key-hide = Ocultar
stream-key-show = Mostrar
stream-encoder = Codificador (H.264 — lo que transportan RTMP, SRT y WHIP)
stream-encoder-auto = Automático — el mejor codificador H.264 detectado
stream-encoder-unavailable = (no disponible aquí)
stream-video-bitrate = Tasa de bits de vídeo (kbps, CBR)
stream-audio-bitrate = Tasa de bits de audio (kbps)
stream-fps = FPS
stream-keyframe = Intervalo de fotogramas clave (s)
stream-audio-track = Pista de audio (1–6)
stream-output-width = Anchura de salida (0 = lienzo)
stream-output-height = Altura de salida (0 = lienzo)
stream-add-target = + Añadir destino
stream-go-live-note = En vivo publica en todos los destinos activados a la vez, directo a cada plataforma. Los destinos con ajustes de codificador idénticos comparten una única codificación.
stream-auto-record = Iniciar grabación cuando me ponga en vivo (la grabación se detiene igualmente de forma independiente)
stream-ffmpeg-note-before = Los códecs de distribución de la transmisión se ejecutan mediante el componente ffmpeg bajo demanda etiquetado —
stream-ffmpeg-note-link = gestiónalo aquí
stream-ffmpeg-note-after = . La grabación local sigue funcionando pase lo que pase con la transmisión.
stream-cancel = Cancelar
stream-save = Guardar

# --- SettingsReplay.tsx ---
replay-title = Ajustes — Búfer de repetición
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Baja (3 Mbps)
replay-quality-standard = Estándar (6 Mbps)
replay-quality-high = Alta (12 Mbps)
replay-length-presets = Preajustes de duración
replay-quality-presets = Preajustes de calidad
replay-length-seconds = Duración (segundos)
replay-video-bitrate = Tasa de bits de vídeo (kbps)
replay-fps = FPS
replay-audio-track = Pista de audio (1–6)
replay-note = Mientras está armado, el búfer ejecuta su propia codificación ligera en un anillo acotado en disco — unos { $mb } MB con estos ajustes. Guardar une el anillo sin recodificar y nunca toca la transmisión ni la grabación. Los cambios se aplican la próxima vez que lo armes.
replay-cancel = Cancelar
replay-save = Guardar

# --- SettingsRemote.tsx ---
remote-title = Ajustes — Control remoto
remote-enable = Activar la API remota WebSocket
remote-password = Contraseña (obligatoria — los controladores se autentican con ella)
remote-password-placeholder = una contraseña para tus controladores
remote-password-hide = Ocultar
remote-password-show = Mostrar
remote-port = Puerto
remote-allow-lan = Permitir conexiones LAN (por defecto, solo esta máquina)
remote-note = Desactivado = el puerto está cerrado. Activado = un WebSocket protegido con contraseña en 127.0.0.1 (o tu LAN si lo permites) que puede cambiar escenas, ejecutar la transición, iniciar/detener la transmisión y la grabación, guardar repeticiones y ajustar silencios/volúmenes — las mismas acciones que la interfaz, nada más. No puede leer archivos. Trata la contraseña como cualquier credencial; prefiere solo-esta-máquina salvo que controles específicamente desde otro dispositivo.
remote-password-required = Se requiere una contraseña para activar la API remota.
remote-cancel = Cancelar
remote-save = Guardar

# --- SettingsHotkeys.tsx ---
hotkeys-title = Ajustes — Atajos
hotkeys-record = Iniciar / detener grabación
hotkeys-record-placeholder = p. ej. Ctrl+Shift+R
hotkeys-go-live = En vivo / Finalizar transmisión
hotkeys-go-live-placeholder = p. ej. Ctrl+Shift+L
hotkeys-transition = Transición del Modo Estudio
hotkeys-transition-placeholder = p. ej. Ctrl+Shift+T o F13
hotkeys-save-replay = Guardar repetición (últimos N segundos)
hotkeys-save-replay-placeholder = p. ej. Ctrl+Shift+S
hotkeys-add-marker = Colocar un marcador de capítulo (grabación)
hotkeys-add-marker-placeholder = p. ej. Ctrl+Shift+K
hotkeys-note = Los atajos son globales — se activan mientras otras apps están enfocadas. En blanco = sin asignar. Las teclas de pulsar para hablar/silenciar del mezclador están en el menú ⋯ de cada tira. En Linux/Wayland, los atajos globales pueden no estar disponibles (un límite del compositor) — los botones siguen funcionando.
hotkeys-cancel = Cancelar
hotkeys-save = Guardar

# --- WorkspaceDialog.tsx ---
workspace-title = Perfiles y colecciones de escenas
workspace-profiles = Perfiles
workspace-profiles-hint = Un perfil es tu configuración — destino de transmisión, salida, atajos. Cambia según el programa o la plataforma.
workspace-collections = Colecciones de escenas
workspace-collections-hint = Una colección son tus escenas + fuentes. Crear duplica la actual como punto de partida.
workspace-active = Activo
workspace-switch-to = Cambiar a { $name }
workspace-active-marker = ● activo
workspace-new-name-placeholder = nuevo nombre…
workspace-new-name-label = Nombre del nuevo { $title }
workspace-create = Crear

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importar desde OBS…
workspace-import-obs-hint = Trae una colección de escenas de OBS (su scenes.json). Tu colección actual se guarda antes.
workspace-import-busy = Importando…
workspace-import-title = «{ $name }» importada
workspace-import-summary = { $scenes } escenas · { $sources } fuentes · { $items } elementos
workspace-import-dismiss = Cerrar
workspace-import-clean = Todo se importó correctamente.
workspace-import-geometry-caveat = Los tamaños y posiciones se ajustan desde el diseño de OBS: revisa cada escena y vuelve a elegir los dispositivos de captura.
workspace-import-notes-title = Importado con avisos
workspace-import-skipped-title = No importado
import-note-needsReselect = Vuelve a elegir dispositivo/monitor/ventana
import-note-gameCaptureAsWindow = Captura de juego → Captura de ventana
import-note-referencesFile = Comprueba la ruta del archivo
import-note-filterDropped = Algunos filtros no compatibles
import-note-geometryApproximated = Posición/tamaño aproximados
import-skip-unsupportedKind = Sin tipo de fuente equivalente
import-skip-group = Los grupos aún no se admiten

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Volver a vincular archivos…
doctor-title = Archivos faltantes
doctor-scanning = Buscando…
doctor-all-good = Todos los archivos referenciados existen. Nada que revincular.
doctor-intro = No se encuentran { $count } archivos referenciados en este equipo. Indica la nueva ubicación de cada uno: cada escena que lo usa se corrige a la vez.
doctor-relinked = { $count } referencias revinculadas.
doctor-uses = usado { $count }×
doctor-locate = Localizar…
doctor-locate-folder = Buscar en carpeta…
doctor-locate-folder-hint = Elige una carpeta; cada archivo faltante se busca por nombre y se revincula.
doctor-kind-image = imagen
doctor-kind-media = medio
doctor-kind-slideshow = presentación
doctor-kind-font = fuente
doctor-kind-lut = LUT
doctor-kind-mask = máscara
history-relinkFiles = Revincular archivos

# --- ScriptsDialog.tsx ---
scripts-title = Scripts (Lua)
scripts-empty = Aún no hay scripts — añade un archivo .lua. Consulta scripts/sample.lua para la API: reacciona a eventos de en vivo/escena/grabación y controla los mismos comandos que la API remota.
scripts-enable = Activar { $path }
scripts-remove = Quitar { $path }
scripts-path-label = Ruta del script
scripts-add = Añadir
scripts-note = Los scripts se ejecutan en entorno aislado — sin acceso a archivos ni al SO; solo pueden llamar a los mismos comandos del estudio que la API remota (cambiar escenas, transición, grabar/transmitir/repetir, silencios). Un error de script se registra y se contiene. Los cambios se aplican en un segundo.
scripts-error-not-lua = Apunta a un archivo .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Docks de navegador
browser-dock-empty = Aún no hay docks — añade una ventana emergente de chat, una página de alertas o tus botones web de Companion.
browser-dock-open = Abrir
browser-dock-remove = Quitar { $name }
browser-dock-name-placeholder = nombre (p. ej. Chat de Twitch)
browser-dock-name-label = Nombre del dock
browser-dock-url-label = URL del dock
browser-dock-note = Un dock se abre como su propia ventana que puedes colocar junto al estudio. La página no obtiene acceso a la app — solo se renderiza. Solo URLs http(s); los docks se abren solo cuando haces clic en Abrir.
browser-dock-error-name = Nombra el dock (p. ej. Chat de Twitch).
browser-dock-error-url = La URL de un dock debe empezar por http:// o https://.

# --- studio-preview-pane ---
studio-preview-label = Vista previa del Modo Estudio
studio-preview-heading = Vista previa
studio-preview-hint = haz clic en una escena para cargarla aquí
studio-preview-empty = La vista previa aparecerá aquí.
studio-preview-mirrors = refleja el programa
studio-preview-transition-select = Transición
studio-preview-duration = Duración de la transición (ms)
studio-preview-commit-title = Pasar Vista previa → Programa mediante la transición (la audiencia lo ve)
studio-preview-transitioning = En transición…
studio-preview-transition-button = Transición ⇄
studio-preview-luma-placeholder = imagen de barrido en escala de grises (png/jpg)
studio-preview-luma-label = Imagen de barrido Luma
studio-preview-browse = Examinar…
studio-preview-filter-images = Imágenes
studio-preview-filter-video = Vídeo
studio-preview-stinger-placeholder = vídeo de stinger (ProRes 4444 .mov conserva su alfa)
studio-preview-stinger-label = Archivo de vídeo de stinger
studio-preview-stinger-cut-label = Punto de corte del stinger (ms)
studio-preview-stinger-cut-title = Cuándo se produce el cambio de escena bajo el stinger (ms dentro de la transición)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Corte
transition-kind-fade = Fundido
transition-kind-slide-left = Deslizar ←
transition-kind-slide-right = Deslizar →
transition-kind-slide-up = Deslizar ↑
transition-kind-slide-down = Deslizar ↓
transition-kind-swipe-left = Barrer ←
transition-kind-swipe-right = Barrer →
transition-kind-luma-linear = Barrido Luma (lineal)
transition-kind-luma-radial = Barrido Luma (radial)
transition-kind-luma-horizontal = Barrido Luma (horizontal)
transition-kind-luma-diamond = Barrido Luma (rombo)
transition-kind-luma-clock = Barrido Luma (reloj)
transition-kind-image = Barrido de imagen (personalizado)
transition-kind-stinger = Stinger (vídeo)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Personalizado (RTMP/RTMPS)
stream-service-srt = SRT (autoalojado)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Acerca de
about-tagline = Graba y transmite como un estudio — sin cuentas, sin nube.
about-version = Versión
about-created-by = Creado por
about-project-started = Proyecto iniciado
about-first-stable = Primera versión estable
about-first-stable-pending = Aún no — la 1.0.0 está en curso
about-platform = Plataforma
about-local-first = Freally Capture se ejecuta por completo en tu máquina. Sin cuentas, sin telemetría, sin nube — lo único que sale de tu ordenador es la transmisión que has decidido enviar.
about-website = Sitio web
about-issues = Informar de un problema
about-license = Licencia
about-eula = EULA
about-third-party = Avisos de terceros
about-check-updates = Buscar actualizaciones…

# --- unified settings modal (TASK-906) ---
settings-title = Ajustes
settings-language-section = Idioma
settings-language = Idioma de la interfaz
settings-language-system = Predeterminado del sistema
settings-language-note = El idioma que elijas aquí se recuerda. «Predeterminado del sistema» sigue tu sistema operativo. El texto sin traducir recurre al inglés.
settings-appearance-section = Apariencia
settings-theme = Tema
settings-theme-dark = Oscuro
settings-theme-light = Claro
settings-theme-custom = Personalizado
settings-accent = Color de acento
settings-general-section = General
settings-show-stats-dock = Mostrar el panel de estadísticas
settings-more-section = Más ajustes
settings-open-output = Grabación…
settings-open-stream = Transmisión…
settings-open-replay = Repetición…
settings-open-hotkeys = Atajos…
settings-open-remote = API remota…
settings-open-about = Acerca de…
controls-settings = ⚙ Ajustes…
controls-settings-title = Idioma, apariencia y las preferencias de toda la app

# --- command palette (TASK-904) ---
palette-title = Paleta de comandos
palette-search = Buscar escenas, fuentes y acciones
palette-placeholder = Buscar escenas, fuentes, acciones…
palette-no-results = Nada coincide con “{ $query }”
palette-hint = ↑ ↓ para moverse · Enter para ejecutar · Esc para cerrar
palette-group-scenes = Escena
palette-group-sources = Fuente
palette-group-actions = Acción
palette-transition = Transición Vista previa → Programa
palette-save-replay = Guardar repetición
palette-add-marker = Colocar un marcador de capítulo
palette-vertical-canvas = Lienzo vertical (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Te damos la bienvenida a Freally Capture
wizard-welcome = Dos pasos rápidos: comprobamos lo que puede hacer tu equipo y luego creas una escena. Lleva unos treinta segundos, y todo se puede cambiar más tarde.
wizard-local-first = Nada de esto sale de tu ordenador. Freally Capture no tiene cuentas, ni telemetría, ni nube.
wizard-start = Empezar
wizard-skip = Omitir
wizard-hardware-title = Lo que puede hacer tu equipo
wizard-probing = Comprobando tu tarjeta gráfica y tu procesador…
wizard-encoder = Codificador
wizard-canvas = Lienzo
wizard-bitrate = Tasa de bits
wizard-probe-found = Encontrado: { $gpus } · { $cores } núcleos físicos
wizard-no-gpu = sin GPU dedicada
wizard-apply = Usar esta configuración
wizard-keep-current = Mantener lo que tengo
wizard-template-title = Empieza con una escena
wizard-template-screen = Capturar mi pantalla
wizard-template-screen-note = Añade una Captura de pantalla de tu monitor principal. El punto de partida más habitual.
wizard-template-empty = Empezar en blanco
wizard-template-empty-note = Una escena vacía. Las fuentes las añades tú con el botón +.
wizard-done = Todo listo.
wizard-done-hint = Pulsa Ctrl+K en cualquier momento para buscar escenas, fuentes y acciones. Los ajustes están detrás del botón ⚙.
wizard-close = Empezar a transmitir

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Tu tarjeta gráfica puede codificar vídeo por su cuenta, lo que deja el procesador libre para el resto del estudio.
autoconfig-reason-software = No se encontró un codificador por hardware utilizable, así que codificará el procesador. Funciona, solo que consume más CPU.
autoconfig-reason-quality-hardware = 1080p a 60 fotogramas por segundo, con una tasa de bits que aceptan todas las plataformas principales.
autoconfig-reason-quality-software = 30 fotogramas por segundo, porque la codificación por software a 60 descarta fotogramas en la mayoría de los procesadores.
autoconfig-reason-quality-low-cores = Una tasa de bits más baja, porque este procesador tiene pocos núcleos y la codificación por software competirá con el compositor por ellos.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Grabación iniciada
announce-recording-paused = Grabación en pausa
announce-recording-stopped = Grabación detenida
announce-live-started = Estás en directo
announce-live-ended = Transmisión finalizada
announce-reconnecting = Conexión perdida, reconectando
announce-stream-failed = Falló la transmisión
announce-frames-dropped = Fotogramas descartados: { $count }

# CAP-M01 — undo/redo edit history
palette-undo = Deshacer
palette-redo = Rehacer
palette-edit-history = Historial de ediciones…
history-title = Historial de ediciones
history-empty = Aún no hay nada que deshacer.
history-current = Estado actual
history-close = Cerrar
history-addScene = Añadir escena
history-renameScene = Renombrar escena
history-removeScene = Eliminar escena
history-reorderScene = Reordenar escenas
history-addSource = Añadir fuente
history-removeSource = Eliminar fuente
history-reorderSource = Reordenar fuentes
history-renameSource = Renombrar fuente
history-transformSource = Mover fuente
history-toggleVisibility = Alternar visibilidad
history-toggleLock = Alternar bloqueo
history-setBlendMode = Cambiar modo de fusión
history-editSourceProperties = Editar propiedades
history-applyLayout = Organizar diseño
history-moveToSeat = Mover a posición
history-groupSources = Agrupar fuentes
history-ungroupSources = Desagrupar fuentes
history-toggleGroupVisibility = Alternar grupo
history-setSceneAudio = Audio de escena
history-setVerticalCanvas = Lienzo vertical
history-addFilter = Añadir filtro
history-removeFilter = Eliminar filtro
history-reorderFilter = Reordenar filtros
history-editFilter = Editar filtro
history-toggleFilter = Alternar filtro
history-setVolume = Ajustar volumen
history-toggleMute = Alternar silencio
history-setMonitor = Cambiar monitorización
history-setTracks = Cambiar pistas
history-setSyncOffset = Ajustar sincronía A/V
history-setAudioHotkeys = Atajos de audio

# CAP-M04 — alignment aids
settings-alignment-section = Ayudas de alineación
settings-smart-guides = Guías inteligentes (ajustar al arrastrar)
settings-safe-areas = Superposiciones de zona segura
settings-rulers = Reglas
align-group = Alinear al lienzo
align-left = Alinear a la izquierda
align-hcenter = Centrar horizontalmente
align-right = Alinear a la derecha
align-top = Alinear arriba
align-vcenter = Centrar verticalmente
align-bottom = Alinear abajo

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Alinear y distribuir selección
arrange-left = Alinear bordes izquierdos
arrange-hcenter = Centrar horizontalmente
arrange-right = Alinear bordes derechos
arrange-top = Alinear bordes superiores
arrange-vcenter = Centrar verticalmente
arrange-bottom = Alinear bordes inferiores
distribute-h = Distribuir horizontalmente
distribute-v = Distribuir verticalmente
guides-group = Guías
guides-add-v = Añadir guía vertical
guides-add-h = Añadir guía horizontal
history-arrangeItems = Organizar elementos
history-editGuides = Editar guías

# CAP-M05 — edit transform + copy/paste
transform-title = Editar transformación — { $name }
transform-anchor = Ancla
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotación
transform-crop = Recorte
transform-crop-left = Izquierda
transform-crop-top = Arriba
transform-crop-right = Derecha
transform-crop-bottom = Abajo
transform-no-size = El tamaño y el recorte estarán disponibles cuando la fuente informe sus dimensiones.
transform-copy = Copiar transformación
transform-paste = Pegar transformación
transform-close = Cerrar
filters-copy = Copiar filtros ({ $count })
filters-paste = Pegar filtros ({ $count })
palette-edit-transform = Editar transformación…
history-pasteFilters = Pegar filtros

# CAP-M26 — keying workbench
workbench-title = Mesa de recorte — { $name }
workbench-mode-keyed = Recortado
workbench-mode-source = Fuente
workbench-mode-matte = Mate
workbench-mode-split = Dividido
workbench-eyedropper = Cuentagotas
workbench-eyedropper-hint = Haz clic en la fuente para muestrear el color clave.
workbench-loupe = Lupa
workbench-split = División
workbench-preview-alt = Vista previa de la mesa de recorte
workbench-tune = Ajustar
workbench-close = Cerrar

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Haz clic en una escena para cortar a ella.
multiview-hint-stage = Haz clic en una escena para prepararla en la vista previa.
palette-multiview = Monitor multiview

# CAP-M07 — projectors
projector-title = Abrir proyector
projector-source = Fuente
projector-target-program = Programa
projector-target-preview = Vista previa
projector-target-scene = Escena…
projector-target-source = Fuente…
projector-target-multiview = Multiview
projector-which-scene = Qué escena
projector-which-source = Qué fuente
projector-none = No hay nada que mostrar
projector-display = Pantalla
projector-windowed = Ventana flotante (esta pantalla)
projector-display-option = Pantalla { $n } — { $w }×{ $h }
projector-primary = (principal)
projector-open = Abrir
projector-cancel = Cancelar
projector-exit-hint = Pulsa Esc para salir
palette-projector = Abrir proyector…

# CAP-M08 — still-frame grab
palette-still = Capturar fotograma…
still-saved-toast = Fotograma guardado: { $name }
still-failed-toast = Error al capturar el fotograma: { $error }
hotkeys-still = Capturar fotograma
hotkeys-still-placeholder = p. ej. Ctrl+Shift+P
