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
sources-badge-test-bars = Barras
sources-badge-test-grid = Rejilla
sources-badge-test-sweep = Barrido
sources-badge-test-tone = Tono
sources-badge-test-sync = Sinc
sources-badge-timer = Temporizador

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
sources-add-timer = Temporizador / Reloj
sources-add-nested-scene = Escena anidada
sources-add-slideshow = Presentación de imágenes
sources-add-chat-overlay = Superposición de chat en directo
sources-add-test-signal = Señal de prueba
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
sources-stream-hide = Ocultar en el stream
sources-stream-show = Mostrar en el stream
sources-stream-hide-item = Ocultar { $name } en el stream
sources-stream-show-item = Mostrar { $name } en el stream
sources-record-hide = Ocultar en la grabación
sources-record-show = Mostrar en la grabación
sources-record-hide-item = Ocultar { $name } en la grabación
sources-record-show-item = Mostrar { $name } en la grabación
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
sources-testsignal-title = Añadir una señal de prueba
sources-testsignal-pattern-label = Patrón
sources-testsignal-bars = Barras de color SMPTE
sources-testsignal-grid = Rejilla de calibración
sources-testsignal-sweep = Barrido de movimiento
sources-testsignal-tone = Tono de 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Destello + pitido de sincronía A/V
sources-testsignal-note = Verifica escenas, codificadores, proyectores y destinos de emisión sin cámara conectada. El patrón de destello + pitido alimenta el banco de sincronía A/V.
sources-testsignal-add = Añadir señal de prueba
sources-timer-title = Añadir un temporizador
sources-timer-mode-label = Modo
sources-timer-wall-clock = Reloj
sources-timer-countdown = Cuenta atrás
sources-timer-stopwatch = Cronómetro
sources-timer-since-live = Tiempo en directo
sources-timer-since-recording = Tiempo grabando
sources-timer-note = La duración, el formato, el estilo y las acciones de fin de cuenta están en las Propiedades de la fuente.
sources-timer-add = Añadir temporizador

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
controls-iso-lanes = Pistas ISO grabando junto al programa: { $count }
controls-pause-title-resume = Reanudar — el archivo continúa como una única línea de tiempo contigua
controls-pause-title-pause = Pausar — no se escriben fotogramas; al reanudar continúa el mismo archivo reproducible
controls-resume-recording = ▶ Reanudar grabación
controls-pause-recording = ⏸ Pausar grabación
controls-reactions-label = Reacciones (integradas en el programa)
controls-reactions-title = Haz flotar una reacción sobre el programa — grabada Y transmitida, para que la repetición muestre el momento exacto. Los espectadores del chat también las activan (su emoji de reacción flota automáticamente); una avalancha solo limita lo que hay en pantalla.
controls-react = Reaccionar { $emoji }
controls-virtual-camera-title = La cámara virtual necesita su propio componente de controlador firmado por SO (Win11 MFCreateVirtualCamera / Win10 DirectShow / extensión CoreMediaIO de macOS / v4l2loopback de Linux) — llega como su propio hito. El modelo de señal está listo para ella: programa, lienzo vertical o una sola fuente, con un micrófono virtual emparejado en Windows/Linux (macOS no tiene API de micrófono virtual — dicho con honestidad).
controls-virtual-camera = ⌁ Iniciar cámara virtual
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
mixer-routing = Enrutamiento
mixer-routing-title = Enrutamiento de salida de audio

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Enrutamiento de audio
routing-intro = Asigna los canales a los buses de pista y luego envía cualquier bus a una salida física: una señal para un grabador de hardware, altavoces en otra sala o una escucha por auriculares en una pista libre. El monitor conserva su propio dispositivo; estas rutas se añaden por encima, así que sin ninguna configurada la mezcla no cambia.
routing-sends-title = Envíos a pistas
routing-no-strips = No hay fuentes de audio en esta escena.
routing-source = Fuente
routing-track = Pista { $n }
routing-send-aria = Enviar { $source } a la pista { $n }
routing-outputs-title = Salidas físicas
routing-master = Master
routing-off = Desactivado
routing-default-output = Salida predeterminada
routing-device-aria = Dispositivo de salida para { $bus }
routing-trim-aria = Trim de salida para { $bus }
routing-trim-db = { $db } dB
routing-muted = Silenciado
routing-device-error = Dispositivo no disponible

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Atenuación
mixer-ducking-title = Matriz de atenuación
ducking-title = Matriz de atenuación
ducking-intro = Cualquier fuente puede atenuar a las demás. Una celda baja el destino (columna) cada vez que suena el disparador (fila): elige una celda para ajustar su profundidad, umbral y tiempos. Cada par es su propia atenuación, así que un canal puede ser atenuado por varios disparadores a la vez.
ducking-need-two = Añade al menos dos fuentes de audio para atenuar entre ellas.
ducking-trigger-target = Disparador ↓ / Destino →
ducking-cell-aria = { $trigger } atenúa a { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Quitar
ducking-amount = Cantidad
ducking-threshold = Umbral
ducking-attack = Ataque
ducking-release = Liberación
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Normalización de sonoridad
loudness-intro = Lleva el programa hacia un objetivo de sonoridad con un techo de picos, para que tu transmisión y tus grabaciones queden en un nivel constante. Lento y suave — dirige, nunca bombea.
loudness-enable = Llevar el programa al objetivo
loudness-target = Objetivo
loudness-target-option = { $target } LUFS
loudness-ceiling = Techo de picos (dBFS)
loudness-note = −14 LUFS va bien para reproducción tipo YouTube; −16 es un objetivo común de streaming; −23 es difusión EBU R128. La acción Normalizar posterior a la grabación usa el mismo objetivo.
ltc-badge = LTC
ltc-title = Código de tiempo SMPTE (LTC)
ltc-intro = Genera código de tiempo lineal SMPTE en una pista y lee el LTC entrante desde cualquier entrada de audio — código de tiempo de audio clásico para sincronizar grabadoras y cámaras externas en posproducción. Totalmente sin conexión.
ltc-generate = Generar LTC en una pista
ltc-track = Pista de código de tiempo
ltc-track-option = Pista { $track }
ltc-fps = Cuadros por segundo
ltc-read = Leer LTC de
ltc-read-off = Desactivado
ltc-decoded = Código de tiempo entrante
ltc-no-lock = sin señal
ltc-note = El generador se sincroniza con la hora del día, sin descarte. Graba su pista (asígnala en Salida) o enrútala a una salida para alimentar equipos externos. El lector alimenta la línea de código de tiempo del overlay de estadísticas y marca los capítulos.
loudness-on = LUFS { $target }
loudness-off = Norm. desactivada

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Tablero de sonidos
mixer-soundboard-title = Tablero de sonidos
soundboard-title = Tablero de sonidos
soundboard-add-pad = + Pad
soundboard-stop-all = Detener todo
soundboard-edit = Editar
soundboard-empty = Aún no hay pads — añade uno y asígnale un clip de audio local.
soundboard-new-pad = Nuevo pad
soundboard-no-clip = Sin clip
soundboard-audio-files = Archivos de audio
soundboard-name = Nombre
soundboard-choose-clip = Elegir clip…
soundboard-gain = Ganancia
soundboard-choke = Choke
soundboard-choke-none = Ninguno
soundboard-loop = Bucle
soundboard-auto-duck = Auto-atenuar
soundboard-tracks = Pistas
soundboard-hotkey = Atajo
soundboard-hotkey-placeholder = p. ej. Ctrl+Shift+1
soundboard-remove = Quitar

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Plugins
mixer-plugins-title = Plugins de audio (CLAP / VST3)
plugins-title = Plugins de audio
plugins-scanning = Buscando…
plugins-none = No se encontraron plugins CLAP ni VST3 en las carpetas estándar.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memoria
stats-dropped = Descartados
stats-render = Renderizado
stats-gpu = GPU
stats-gpu-compositing = componiendo
stats-gpu-idle = inactiva
stats-disk = Disco
stats-disk-free = libre
stats-disk-left = Grab. restante
stats-disk-rate = ≈ { $rate } MB/s grabando
stats-vertical-fps = 9:16 FPS
stats-targets-label = Destinos de transmisión
stats-rehearsal-note = Ensayo — los destinos publican solo en un receptor local
stats-timeline-open = Línea de tiempo
timeline-title = Línea de tiempo de la sesión
timeline-empty = Aún no hay nada grabado — la línea de tiempo registra mientras transmites o grabas.
timeline-live = EN VIVO — aún grabando
timeline-fit = Ajustar
timeline-legend-fps = fps
timeline-legend-behind = cola del codificador (frames de retraso)
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
channelstrip-solo-title = Solo (PFL) — el monitoreo oye solo los canales en solo; la mezcla del programa no cambia
channelstrip-solo-source = Solo de { $name } (PFL)
channelstrip-pan-label = Balance (doble clic restablece)
channelstrip-pan-aria = Balance de { $name }
channelstrip-mono-label = Mezclar a mono
channelstrip-automix-label = Automezcla (reparto de ganancia)
channelstrip-automix-note = Reparto de ganancia: el mezclador mantiene estable el nivel combinado de todas las tiras en automezcla y se lo cede a quien esté hablando — ideal para paneles multimicrófono y pódcasts. Desactivado hasta que añadas una tira.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Genera un retorno sin eco para esta fuente — todos los del programa excepto la propia fuente. Úsalo con un invitado remoto para que no oiga su propia voz retardada.
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
livebutton-rehearse = Ensayar
livebutton-rehearse-title = Ejecuta todo el show contra un receptor local — no se envía nada
livebutton-end-rehearsal = Terminar ensayo
livebutton-title-rehearsing = Hay un ensayo en curso — nada sale de esta máquina
livebutton-badge-rehearsal = ENSAYO
livebutton-aria-rehearsal = Ensayando
livebutton-rehearsal-banner = Ensayo — nada sale de esta máquina


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
properties-testtone-note = Una senoidal continua de 1 kHz a −20 dBFS. El nivel y el silencio viven en su canal del mezclador; no hay nada más que configurar.
properties-timer-format = Formato de hora (strftime)
properties-timer-format-note = p. ej. %H:%M:%S (predeterminado), %I:%M %p, %A %H:%M — un patrón inválido vuelve a %H:%M:%S.
properties-timer-utc = Desfase UTC (minutos)
properties-timer-utc-placeholder = hora local
properties-timer-duration = Duración (segundos)
properties-timer-target = Cuenta atrás hasta (HH:MM)
properties-timer-target-note = Un objetivo de reloj corre solo y se repite a diario; déjalo vacío para usar la duración con Iniciar/Pausar/Reiniciar.
properties-timer-end = Al llegar a cero
properties-timer-end-none = No hacer nada
properties-timer-end-flash = Parpadear el temporizador
properties-timer-end-switch = Cambiar de escena
properties-timer-end-scene = Escena
properties-timer-size = Tamaño (px)
properties-timer-start = Iniciar
properties-timer-pause = Pausar
properties-timer-reset = Reiniciar
properties-text-file = Leer de archivo (ruta; vacío = usar el texto de arriba)
properties-text-binding = Interpretar como
properties-text-binding-whole = Archivo completo
properties-text-binding-csv = Celda CSV
properties-text-binding-json = Puntero JSON
properties-text-csv-row = Fila
properties-text-csv-column = Columna
properties-text-csv-column-placeholder = nombre o número
properties-text-json-pointer = Puntero
properties-text-file-note = El archivo se relee en medio segundo tras un cambio. Se toleran escrituras atómicas (temporal + renombrado): el último valor bueno permanece en pantalla durante el cambio.
avsync-title = Calibración de sincronía A/V
avsync-intro = Reproduce el patrón integrado de destello + pitido por tu pantalla y altavoces, cáptalo con la cámara y el micrófono que quieres alinear, y el banco mide la diferencia. El bucle pasa por pantalla y altavoces, así que sus pequeñas latencias quedan incluidas.
avsync-video-label = Cámara (fuente de vídeo)
avsync-audio-label = Micrófono (fuente de audio)
avsync-pick = Elige una fuente…
avsync-no-video = Añade primero la cámara como fuente — el banco mide fuentes, no dispositivos en bruto.
avsync-no-audio = Añade primero el micrófono como fuente de audio.
avsync-projector = Programa a pantalla completa en
avsync-projector-open = Abrir proyector
avsync-projector-window-title = Programa — sincronía A/V
avsync-start-note = Al empezar se añade temporalmente una fuente «Patrón de sincronía A/V» sobre la escena actual y el pitido suena por el dispositivo de monitoreo. Todo se retira al terminar.
avsync-manual = Desfase de sincronía (ms, manual)
avsync-start = Iniciar calibración
avsync-measuring = Midiendo unos 12 segundos — apunta la cámara al programa parpadeante y mantén la sala tranquila…
avsync-flash-seen = La cámara ve el destello
avsync-flash-waiting = Esperando a que la cámara vea el destello…
avsync-beep-heard = El micrófono oye el pitido
avsync-beep-waiting = Esperando a que el micrófono oiga el pitido…
avsync-cancel = Cancelar
avsync-result-offset = El vídeo llega { $offset } ms después del audio.
avsync-result-detail = Medido en { $cycles } ciclos, ±{ $jitter } ms.
avsync-negative = El audio ya llega más tarde que el vídeo. Retrasar el audio no corrige esta dirección — si otra pista lleva el sonido de esta cámara, baja allí su desfase.
avsync-over-cap = La diferencia medida supera el tope de { $max } ms del desfase. Una brecha así suele indicar una fuente mal elegida — revisa la cadena y vuelve a medir.
avsync-applied = Aplicado — el desfase del micrófono ahora es de { $offset } ms.
avsync-apply = Aplicar { $offset } ms al micrófono
avsync-again = Medir de nuevo
avsync-close = Cerrar
avsync-error-noFlash = La cámara nunca vio el destello. Apúntala al programa parpadeante (pantalla completa ayuda), confirma que la fuente está activa y vuelve a medir.
avsync-error-noBeep = El micrófono nunca oyó el pitido. Confirma que el dispositivo de monitoreo se oye y que el micrófono está activo (sin pulsar-para-hablar), y vuelve a medir.
avsync-error-tooFewCycles = No se captaron suficientes ciclos limpios de destello/pitido. Mantén el patrón bien visible y audible durante toda la medición.
avsync-error-notThePattern = Lo visto u oído no se repite al ritmo del patrón — probablemente luz o ruido de la sala, no la señal de prueba.
avsync-error-unstable = Los ciclos discrepan demasiado para dar una sola cifra. Estabiliza la cámara, reduce el ruido y mide otra vez.
hotkey-audit-title = Mapa de atajos
hotkey-audit-search = Buscar
hotkey-audit-filter = Función
hotkey-audit-filter-all = Todas las funciones
hotkey-audit-col-key = Tecla
hotkey-audit-col-action = Acción
hotkey-audit-col-where = Dónde
hotkey-audit-col-status = Estado
hotkey-audit-ok = OK
hotkey-audit-shared = Compartida por { $count } asignaciones
hotkey-audit-unregistered = No registrada en el SO (ocupada por otra app o no disponible)
hotkey-audit-invalid = No es un atajo válido
hotkey-audit-empty = Aún no hay atajos — asígnalos en Ajustes → Atajos o en un canal del mezclador.
hotkey-audit-export = Exportar chuleta
hotkey-audit-exported = Guardado en { $path }
hotkey-audit-note = Asigna y cambia teclas en Ajustes → Atajos (acciones globales) y en cada canal del mezclador (pulsar-para-hablar / silenciar); esta tabla las audita y documenta.
hotkey-audit-action-record = Alternar grabación
hotkey-audit-action-go-live = Alternar emisión
hotkey-audit-action-transition = Ejecutar transición
hotkey-audit-action-save-replay = Guardar repetición
hotkey-audit-action-add-marker = Añadir marcador
hotkey-audit-action-still = Capturar imagen fija
hotkey-audit-action-panic = Pantalla de pánico
hotkey-audit-action-timer-toggle = Iniciar/pausar todos los temporizadores
hotkey-audit-action-timer-reset = Reiniciar todos los temporizadores
hotkey-audit-action-ptt = Pulsar para hablar
hotkey-audit-action-ptm = Pulsar para silenciar
hotkey-audit-feature-recording = Grabación
hotkey-audit-feature-streaming = Emisión
hotkey-audit-feature-studio = Modo estudio
hotkey-audit-feature-replay = Repetición
hotkey-audit-feature-markers = Marcadores
hotkey-audit-feature-stills = Imágenes fijas
hotkey-audit-feature-panic = Pánico
hotkey-audit-feature-timers = Temporizadores
hotkey-audit-feature-audio = Audio (por fuente)
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
properties-deinterlace = Desentrelazado
properties-deinterlace-off = Apagado
properties-deinterlace-discard = Descartar (duplicar líneas de un campo)
properties-deinterlace-bob = Bob (alternar campos)
properties-deinterlace-linear = Lineal (interpolar)
properties-deinterlace-blend = Mezcla (promediar campos)
properties-deinterlace-adaptive = Adaptativo al movimiento (clase yadif)
properties-field-order = Orden de campos
properties-field-order-top = Campo superior primero
properties-field-order-bottom = Campo inferior primero
properties-deinterlace-note = Para señales entrelazadas de capturadoras. CPU pura, idéntico en todos los SO; cambiarlo reinicia el dispositivo (como un cambio de formato).
camera-controls-title = Controles de cámara
camera-controls-refresh = Actualizar
camera-controls-reset = Restablecer perfil
camera-controls-empty = No hay controles ahora — el dispositivo debe estar transmitiendo (añádelo antes a una escena), y algunos backends no informan ninguno (sobre todo macOS). Es el estado honesto por SO.
camera-controls-note = Los cambios se aplican en vivo y se guardan en el perfil del dispositivo, que se reaplica al reconectar y al reiniciar.
camera-control-brightness = Brillo
camera-control-contrast = Contraste
camera-control-hue = Tono
camera-control-saturation = Saturación
camera-control-sharpness = Nitidez
camera-control-gamma = Gamma
camera-control-white-balance = Balance de blancos
camera-control-backlight = Compensación de contraluz
camera-control-gain = Ganancia
camera-control-pan = Paneo
camera-control-tilt = Inclinación
camera-control-zoom = Zoom
camera-control-exposure = Exposición
camera-control-iris = Iris
camera-control-focus = Enfoque
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
audiofilters-name-parametric-eq = Ecualizador paramétrico
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Filtro antirretumbo
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Preajuste
audiofilters-voice-preset-pick = Preajuste de voz…
audiofilters-voice-broadcast = Voz de emisión
audiofilters-voice-podcast = Voz de pódcast
audiofilters-voice-clean = Voz limpia
audiofilters-voice-none = Vaciar cadena
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Frecuencia de sibilancia (Hz)
audiofilters-deesser-amount = Reducción máx. (dB)
audiofilters-rumble-freq = Corte de graves (Hz)
audiofilters-title = Filtros de audio — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Curva de respuesta del ecualizador paramétrico con espectro en vivo
eq-band-type = Tipo
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Banda
eq-remove-band = Quitar banda
eq-type-bell = Campana
eq-type-lowShelf = Shelf bajo
eq-type-highShelf = Shelf alto
eq-type-notch = Notch
eq-type-highPass = Paso alto
eq-type-lowPass = Paso bajo
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
filters-name-perspective = Perspectiva
filters-name-fade-loop = Bucle de fundido
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
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galería
filters-shader-gallery-pick = Cargar un preajuste…
filters-shader-gallery-grayscale = Escala de grises
filters-shader-gallery-invert = Invertir
filters-shader-gallery-scanlines = Líneas de escaneo
filters-shader-gallery-vignette = Viñeta
filters-shader-source = Código del shader (WGSL)
filters-shader-hint = Escribe un effect(uv, color, p, texel, time) en WGSL que devuelva un vec4. Anota los parámetros con // @param name min max default para crear deslizadores. Un shader no válido se ignora: la fuente se muestra sin filtrar hasta que compila.
filters-name-bezier-mask = Máscara Bézier
filters-mask-editor-hint = Arrastra un punto para moverlo, doble clic para añadir uno, clic derecho en un punto para eliminarlo.
filters-mask-shape = Forma
filters-mask-shape-pick = Preajuste…
filters-mask-shape-rectangle = Rectángulo
filters-mask-shape-diamond = Rombo
filters-mask-shape-hexagon = Hexágono
filters-mask-shape-circle = Círculo
filters-mask-feather = Difuminado
filters-mask-export-wipe = Exportar como cortinilla…
filters-mask-image = Imagen de máscara
filters-mask-mode = Modo
filters-mask-alpha = alfa
filters-mask-luma = luminancia
filters-mask-invert = invertir
filters-speed-x = Velocidad X (px/s)
filters-speed-y = Velocidad Y (px/s)
filters-tilt = Inclinación
filters-far-fade = Fundido del borde lejano
filters-fade-in-s = Aparecer (s)
filters-visible-s = Visible (s)
filters-fade-out-s = Desvanecer (s)
filters-hidden-s = Oculto (s)
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
recordings-trim = Recortar
recordings-trim-title = Corta un clip de esta grabación — los cortes alineados a fotogramas clave se exportan sin recodificar
recordings-verify = Verificar
recordings-verify-title = Comprueba la integridad del archivo — estructura del contenedor, continuidad, intercalado A/V, duración
recordings-verifying = Verificando…
verify-dismiss = Cerrar
verify-verdict-pass = { $name } — integridad correcta
verify-verdict-warn = { $name } — verificado con avisos
verify-verdict-fail = { $name } — se encontraron problemas
verify-container = Contenedor
verify-video-continuity = Continuidad de vídeo
verify-audio-continuity = Continuidad de audio
verify-av-interleave = Intercalado A/V
verify-duration = Duración
recordings-alpha-label = alfa
recordings-prores-title = Exportar un máster .mov ProRes 4444 que conserva el alfa (para edición)
recordings-qtrle-title = Exportar un .mov QuickTime Animation que conserva el alfa (compatibilidad máxima)
trim-title = Recortar — { $name }
trim-loading = Leyendo el archivo…
trim-preview-alt = Fotograma de vista previa
trim-position = Posición de reproducción
trim-step-second-back = Un segundo atrás
trim-step-frame-back = Un fotograma atrás
trim-step-frame-forward = Un fotograma adelante
trim-step-second-forward = Un segundo adelante
trim-snap = Fotograma clave
trim-snap-title = Ajustar al fotograma clave más cercano — un corte ahí se exporta sin recodificar
trim-set-in = Punto de entrada
trim-set-out = Punto de salida
trim-range-invalid = El punto de salida debe ir después del de entrada.
trim-copy-badge = ✓ Se exporta sin recodificar — el punto de entrada cae en un fotograma clave.
trim-reencode-badge = Se recodificará: el punto de entrada está entre fotogramas clave (usa «Fotograma clave» para un corte sin pérdida).
trim-export = Exportar clip
trim-export-916 = 9:16
trim-export-916-title = Exportar vertical reencuadrado (recorte centrado al tamaño del lienzo vertical) — siempre recodifica
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
recordings-normalize = Normalizar
recordings-normalizing = Normalizando…
recordings-normalize-title = Normalizar la sonoridad al objetivo (escribe una copia)
recordings-normalized-to = Normalizado a { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Solo audio
audiorec-format = Formato de grabación de audio
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Grabar audio
audiorec-stop = Detener
audiorec-pause = Pausar
audiorec-resume = Reanudar
audiorec-recording = REC { $sec }s
audiorec-saved = Se guardaron { $count } archivo(s) de pista


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
output-recording-template = Nombre de archivo de grabación
output-replay-template = Nombre de archivo de repetición
output-still-template = Nombre de archivo de fotograma
output-template-tokens = Variables: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Carpeta de repeticiones
output-still-folder = Carpeta de fotogramas
output-same-folder-placeholder = Carpeta de grabaciones
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
bench-open = Ejecutar benchmark de codificadores…
bench-title = Benchmark de codificadores
bench-intro = Ejecuta escaleras de codificación medidas (cada codificador detectado × preajuste × resolución) en esta máquina — un minuto, todo sin conexión, nada sale de tu equipo. Los fallos se listan, no se ocultan. Detén antes cualquier stream o grabación.
bench-start = Iniciar benchmark
bench-rerun = Ejecutar de nuevo
bench-running = Midiendo… { $done } / { $total }
bench-cancel = Cancelar
bench-col-encoder = Codificador
bench-col-preset = Preajuste
bench-col-rung = Escalón
bench-col-achieved = fps
bench-col-headroom = Margen
bench-failed = falló
bench-rec-title = Recomendación (medida)
bench-rec-body = { $encoder } con { $preset }, { $width }×{ $height } @ { $fps } fps — medido { $headroom }× tiempo real. Bitrate de stream sugerido: { $bitrate } kbps.
bench-rec-none = Nada sostiene tiempo real en esta máquina — baja la resolución del lienzo o los fps y vuelve a medir.
bench-apply = Aplicar a los ajustes de grabación
bench-applied = Aplicado ✓
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
output-iso-heading = Grabación ISO
output-iso-explainer = Graba las fuentes seleccionadas limpias, cada una en su propio archivo junto al programa — antes de la composición, al tamaño y fps del lienzo, para que cada archivo caiga alineado en la línea de tiempo del editor. Dos pistas van bien en una GPU de gama media; cada pista extra cuesta otro render y otra codificación.
output-iso-none = Aún no hay fuentes en la colección.
output-iso-source-on = Grabando «{ $name }» como su propio archivo ISO — clic para detener
output-iso-source-off = Grabar «{ $name }» como su propio archivo ISO
output-iso-post-filter = Grabar con los filtros de la fuente (post-filtro); sin marcar se graba la fuente sin procesar
output-iso-format = Formato ISO
output-iso-encoder = Codificador de vídeo ISO
output-alpha-frec = Grabar con transparencia (alfa) — el programa sobre un fondo transparente
output-alpha-title = El grabador recibe su propio render transparente; la vista previa y el stream siguen normales. Exporta a ProRes 4444 o QTRLE desde la lista de grabaciones — MP4/MKV aplanan el alfa.
output-split-events = Empezar también un archivo nuevo al… (cada parte empieza exactamente en el evento; duración mínima 1 s)
output-split-on-scene = cambiar de escena
output-split-on-marker = marcador
output-split-on-rundown = paso del guion
output-auto-markers = Soltar marcadores de capítulo automáticamente en eventos del estudio (cambio de escena, guardado de repetición, reconexión, fotogramas perdidos, alarmas, reglas)
output-auto-markers-title = Los marcadores tipados quedan en los capítulos de la grabación (mkv) o en el archivo .chapters.txt, junto al atajo manual
output-pipeline-heading = Canal posgrabación
output-pipeline-explainer = Al finalizar una grabación, ejecuta estos pasos sobre el archivo principal, en orden y en segundo plano. Un conjunto de acciones cerrado — no hay paso «ejecutar comando», por diseño. La cadena se detiene en el primer fallo.
output-pipeline-enabled = Ejecutar el canal tras cada grabación
output-pipeline-add = Añadir un paso…
output-pipeline-up = Subir
output-pipeline-down = Bajar
output-pipeline-remove = Quitar paso
output-pipeline-template = Plantilla de renombrado (tokens CAP-M25)
output-pipeline-folder = Carpeta
pipeline-queue = Canal posgrabación
pipeline-verify = Verificar
pipeline-remux = Remuxar a MP4
pipeline-normalize = Normalizar sonoridad
pipeline-rename = Renombrar
pipeline-move = Mover a carpeta
pipeline-copy = Copiar a carpeta
pipeline-reveal = Mostrar en el explorador
pipeline-luaEvent = Notificar a los scripts Lua
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
stream-session-report = Escribir un informe de sesión (HTML + Markdown) junto a la grabación al terminar
stream-simulator-title = Simulador de red (ensayos)
stream-simulator-note = Solo modela los receptores locales del ensayo — practica reconexiones y subidas lentas. Un Go Live real nunca se degrada.
stream-simulator-profile = Perfil
stream-simulator-off = Apagado
stream-simulator-hotel-wifi = Wi-Fi de hotel
stream-simulator-mobile-hotspot = Punto de acceso móvil
stream-simulator-custom = Personalizado
stream-simulator-bandwidth = Ancho de banda (kbps, 0 = sin límite)
stream-simulator-latency = Latencia (ms)
stream-simulator-jitter = Jitter (± ms)
stream-simulator-outage-every = Corte cada (s, 0 = nunca)
stream-simulator-outage-len = Duración del corte (s)
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
hotkeys-go-live = En vivo / Finalizar transmisión
hotkeys-transition = Transición del Modo Estudio
hotkeys-save-replay = Guardar repetición (últimos N segundos)
hotkeys-add-marker = Colocar un marcador de capítulo (grabación)
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
studio-preview-stinger-matte-label = Mate de pista
studio-preview-stinger-matte-title = Cómo un stinger con mate de pista integra la transparencia: el relleno y su mate uno al lado del otro (horizontal) o apilados (vertical)
studio-preview-stinger-duck-label = Atenuar el programa
studio-preview-stinger-duck-title = Atenúa el audio del programa por debajo del propio audio del stinger mientras se reproduce (0 = desactivado)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = Mover (morphing)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Ninguno
stinger-matte-horizontal = Lado a lado
stinger-matte-vertical = Apilado

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
settings-open-about = Acerca de…

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
history-toggleOutputVisibility = Alternar visibilidad por salida
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
guides-clear = Quitar todas las guías
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

# CAP-M13 — source health dashboard
palette-source-health = Salud de las fuentes…
palette-av-sync = Calibración de sincronía A/V…
palette-hotkey-audit = Mapa de atajos…
health-title = Salud de las fuentes
health-col-source = Fuente
health-col-state = Estado
health-col-resolution = Resolución
health-col-fps = FPS
health-col-last-frame = Último fotograma
health-col-dropped = Descartados
health-col-retries = Reinicios
health-col-actions = Acciones
health-state-live = En vivo
health-state-waiting = Esperando
health-state-error = Error
health-state-inactive = Inactiva
health-restart = Reiniciar
health-properties = Propiedades
health-empty = Esta colección aún no tiene fuentes.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = ¿Salir de Freally Capture?
quit-body = Al salir ahora se hará lo siguiente de forma segura y en orden:
quit-consequence-stream = Finalizar la transmisión en vivo y desconectarse del servicio.
quit-consequence-recording = Detener la grabación y finalizar sus archivos.
quit-consequence-replay = Apagar el búfer de repetición: el material no guardado se descarta.
quit-confirm = Salir de forma segura
quit-quitting = Cerrando…
quit-cancel = Cancelar

# CAP-M11 — crash-safe recording salvage
salvage-title = ¿Recuperar grabaciones interrumpidas?
salvage-body = La última sesión terminó de forma inesperada mientras estas grabaciones aún se escribían. La reparación crea una copia reproducible junto al original — el archivo original nunca se modifica.
salvage-repair = Reparar
salvage-repairing = Reparando…
salvage-done = Reparado
salvage-repaired = Reparado → { $name }
salvage-failed = La reparación falló: { $error }
salvage-dismiss = Ahora no

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Fallo del codificador: se cambió de { $from } a { $to }. La transmisión se reconectó y sigue activa.
fallback-toast-recording = Fallo del codificador: se cambió de { $from } a { $to }. La grabación continúa en un archivo nuevo.
fallback-note = Codificador de respaldo: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = El audio del programa está en silencio
alarm-clipping = El audio del programa está saturando
alarm-black = La imagen del programa está en negro
alarm-frozen = La imagen del programa lleva un rato sin cambiar
alarm-lowDisk = Espacio en disco: quedan unos { $minutes } min a la tasa de bits actual
alarm-dismiss = Descartar alarma
alarm-cleared = Resuelto: { $alarm }

# CAP-M22 — panic button
palette-panic = Pánico — cortar a la placa de privacidad
panic-banner-title = Pánico
panic-banner-body = El programa muestra la placa de privacidad; todo el audio está silenciado y las capturas detenidas. La transmisión y la grabación siguen activas.
panic-restore = Restaurar…
panic-restore-confirm = ¿Restaurar el programa?
panic-restore-yes = Restaurar
panic-restore-cancel = Cancelar
hotkeys-panic = Pánico (placa de privacidad)
hotkeys-timer-toggle = Iniciar/pausar todos los temporizadores
hotkeys-timer-reset = Reiniciar todos los temporizadores
panic-slate-color = Color de la placa de pánico
panic-slate-image = Imagen de la placa de pánico
panic-slate-image-placeholder = Ruta de imagen opcional

# CAP-M24 — redacted diagnostics bundle
diag-title = Paquete de diagnóstico
diag-intro = Exporta un .zip depurado (instantánea de configuración, sondeo de codificadores, estadísticas recientes — nunca incluye secretos, rutas ni nombres) para adjuntarlo a mano a un issue de GitHub. No se envía nada.
diag-preview = Ver contenido
diag-hide-preview = Ocultar vista previa
diag-export = Exportar .zip
diag-exported = Exportado: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Verificación previa
preflight-intro = Cada punto bloqueante debe estar en verde; el resto son avisos honestos.
preflight-item-targets = Destinos configurados (clave/URL)
preflight-item-encoder = Hay un codificador utilizable
preflight-item-sources = Todas las fuentes sanas
preflight-item-disk = Espacio en disco para la grabación
preflight-item-mic = Medición del micrófono
preflight-item-desktopAudio = Medición del audio del escritorio
preflight-item-replay = Búfer de repetición armado
preflight-targets-detail = { $count } habilitados
preflight-sources-detail = { $count } fuente(s) con error
preflight-disk-detail = ~{ $minutes } min a la tasa actual
preflight-fix-stream = Ajustes de stream…
preflight-fix-components = Componentes…
preflight-fix-sources = Salud de las fuentes…
preflight-fix-replay = Armar
preflight-optional = opcional
preflight-hold = Bloquear Go Live hasta que todo esté en verde
preflight-cancel = Cancelar
preflight-go-anyway = Salir en vivo igualmente
preflight-go-live = Salir en vivo


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Fondo
scenes-backdrop-aria = Fondo de { $name }
backdrop-title = Fondo — { $name }
backdrop-hint = Un fondo fijado detrás de todo en esta escena: una imagen, un GIF animado o un vídeo en bucle. Tu captura siempre queda encima; desplázate sobre el lienzo para hacer zoom.
backdrop-choose = Elegir imagen o vídeo…
backdrop-remove = Quitar el fondo
backdrop-none = Sin fondo.
backdrop-position = Posición
backdrop-split-full = Lienzo completo
backdrop-split-left = Mitad izquierda
backdrop-split-right = Mitad derecha
backdrop-split-top = Mitad superior
backdrop-split-bottom = Mitad inferior
backdrop-sync = Iniciar la reproducción al empezar a grabar
backdrop-sync-hint = Se queda en el primer fotograma hasta que grabas; cada toma inicia el vídeo desde el principio.
backdrop-preview-play = Previsualizar reproducción
backdrop-preview-pause = Pausar la previsualización
backdrop-filter-all = Fondos (imágenes y vídeo)
backdrop-filter-images = Imágenes
backdrop-filter-media = Vídeo y GIF
sources-backdrop-badge = Fondo de pantalla (fijado abajo del todo)
sources-backdrop-pinned = El fondo permanece fijado abajo del todo
filters-name-flip = Voltear
filters-flip-horizontal = Horizontal
filters-flip-vertical = Vertical
history-setSceneBackdrop = Establecer fondo
history-setBackdropSplit = Mover el fondo
history-setBackdropSync = Sincronización del fondo con la grabación
backdrop-scrub = Posición de reproducción
backdrop-loop = Bucle
backdrop-reverse = Reproducir al revés
backdrop-reverse-hint = Invertir genera una copia al revés una sola vez (los vídeos requieren el componente ffmpeg; los GIF se invierten al instante); el primer cambio puede tardar con archivos largos.
filters-scaling = Escalado
filters-scaling-hint = Modos píxel-perfectos para contenido retro/píxel; Entero además ajusta el tamaño dibujado a múltiplos exactos (los tiradores muestran el tamaño lógico).
filters-scaling-auto = Suave
filters-scaling-nearest = Vecino más próximo
filters-scaling-integer = Entero (× exactos)
filters-scaling-sharp = Bilineal nítido
history-setScaling = Cambiar escalado
hotkeys-zoom-100 = Zoom: restablecer (100 %)
hotkeys-zoom-150 = Zoom: acercar al 150 %
hotkeys-zoom-200 = Zoom: acercar 2×
sources-follow-title = Seguir el cursor durante el zoom (Windows; desplázate sobre el lienzo para hacer zoom)
sources-follow-item = Alternar el seguimiento del cursor de { $name }
filters-autocrop = ✂ Recortar bandas negras
filters-autocrop-title = Analiza el siguiente fotograma en busca de bandas y las recorta (reversible). Las escenas oscuras nunca se recortan.
filters-autocrop-follow = Volver a comprobar al cambiar la resolución
history-autoCrop = Recorte automático de bandas
sources-link-audio = Capturar también el audio de esta app (vinculado: ocultar lo silencia, quitar la ventana lo quita)
history-addLinkedWindow = Añadir ventana + audio vinculado
sources-hdr-title = Esta pantalla es HDR — abre el mapeo tonal (el lienzo sigue en SDR)
sources-hdr-item = Mapeo tonal HDR de { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Esta pantalla emite HDR. Sin mapeo tonal, las luces se recortan y la captura se ve lavada en el lienzo SDR. Los cambios se aplican en el siguiente fotograma.
sources-hdr-enable-suggested = Activar sugerido (maxRGB, 200 nits)
sources-hdr-operator = Operador
sources-hdr-op-clip = Recorte (desactivado)
sources-hdr-op-maxrgb = maxRGB (conserva el tono)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = Rodilla BT.2408 (SDR exacto)
sources-hdr-paper-white = Blanco de papel
sources-hdr-nits = nits
projector-target-passthrough = Monitor de paso (baja latencia)
projector-which-device = Dispositivo
projector-passthrough-none = Añade primero una pantalla, ventana o dispositivo de captura.
projector-passthrough-about = Fotogramas crudos del dispositivo: sin escenas, sin filtros, sin compositor. Muestra una latencia medida; el audio sigue monitorizándose por el canal del mezclador.
projector-passthrough-hint = Paso directo — Esc cierra
projector-latency = { $ms } ms
projector-latency-measuring = midiendo…
automation-title = Automatización — reglas, macros y variables
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Reglas
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Activa
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Cuando
automation-then-run = entonces ejecutar
automation-no-macro = (no macro)
automation-macros = Macros
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Ejecutar
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Variables del estudio
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
rundown-title = Escaleta del programa
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Iniciar
rundown-next = Siguiente ▸
rundown-stop = Detener
rundown-idle = Detenida
rundown-next-up = Siguiente: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Paso
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
automation-layer = Capa
automation-layer-hint = Solo se dispara mientras esta capa está activa (vacío = todas). Las capas son fijas: una tecla de capa cambia y se queda (la API global del SO no permite capas por mantener pulsado).
automation-chord-hint = Una tecla simple (Ctrl+Shift+M) o un acorde de dos pulsaciones (Ctrl+K, 3). La segunda tecla solo se reserva mientras el acorde está pendiente.
panel-title = Panel LAN y tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Servir el panel
panel-port = Puerto
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Contraseña
panel-show = Mostrar
panel-hide = Ocultar
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Guardar
osc-title = Superficie de control OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Escuchar OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = Cámaras PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Cámara
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Dirección
ptz-port = Puerto
ptz-speed = Velocidad
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
ptz-recall = Recuperar
ptz-store = Guardar
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = Superficie de control MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Entrada
midi-output = Salida (feedback)
midi-none = (none)
midi-learn = Aprender
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Hace
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
panel-lan-warning = ⚠ El tráfico LAN no está cifrado — la contraseña viaja en la URL por HTTP. Úsalo solo en una red de confianza.
osc-lan-warning = ⚠ OSC no tiene contraseña — cualquier dispositivo de la red puede enviar estos comandos. Usa LAN solo en una red de confianza.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stats
sources-add-system-stats = Estadísticas de rendimiento (HUD)
sources-stats-title = Añadir un HUD de rendimiento
sources-stats-note = Muestra en el programa los números reales medidos del estudio para tus espectadores: fps, CPU, memoria, tiempo de render, fotogramas perdidos y bitrate en vivo. Qué líneas se muestran, el tamaño y el color están en las Propiedades de la fuente. El uso de GPU no se muestra porque no se mide.
sources-stats-add = Añadir HUD de estadísticas
properties-stats-show-fps = Mostrar FPS
properties-stats-show-cpu = Mostrar CPU
properties-stats-show-memory = Mostrar memoria
properties-stats-show-render = Mostrar tiempo de render
properties-stats-show-dropped = Mostrar fotogramas perdidos
properties-stats-show-bitrate = Mostrar bitrate
properties-stats-show-timecode = Mostrar código de tiempo (LTC)
properties-stats-size = Tamaño (px)
properties-stats-note = El HUD dibuja etiquetas compactas universales (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) directamente en el programa; sin transmisión, la línea de bitrate muestra «—».

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Visualizador
sources-add-visualizer = Visualizador de audio
sources-visualizer-title = Añadir un visualizador de audio
sources-visualizer-style-label = Estilo
sources-visualizer-style-bars = Barras de espectro
sources-visualizer-style-scope = Osciloscopio
sources-visualizer-style-vu = Vúmetros
sources-visualizer-target-label = Escucha a
sources-visualizer-target-master = Mezcla maestra
sources-visualizer-target-track = Pista { $n }
sources-visualizer-note = Dibuja la señal que realmente se mezcla (post-fader): una fuente silenciada se ve plana, igual que suena. El tamaño, el color, el número de barras y la caída están en las Propiedades de la fuente.
sources-visualizer-add = Añadir visualizador
properties-vis-bands = Barras
properties-vis-decay = Velocidad de caída (dB/s)
properties-vis-peak-hold = Marcadores de pico
properties-vis-missing-source = (fuente ausente)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splits
sources-add-split-timer = Cronómetro de splits (speedrun)
sources-splits-title = Añadir un cronómetro de splits
sources-splits-file-label = Archivo .lss de LiveSplit
sources-splits-comparison-label = Comparar con
sources-splits-comparison-pb = Mejor marca personal
sources-splits-comparison-best = Mejores segmentos
sources-splits-comparison-average = Promedio
sources-splits-note = Importa el archivo en solo lectura: nunca se escribe nada de vuelta. Asigna las teclas globales Split / Undo / Skip / Reset en Ajustes → Atajos. Los auto-splitters por memoria de proceso no están soportados deliberadamente.
sources-splits-add = Añadir cronómetro
properties-splits-size = Tamaño (px)
properties-splits-ahead = Por delante
properties-splits-behind = Por detrás
properties-splits-gold = Oro
properties-splits-split = Split
properties-splits-undo = Deshacer
properties-splits-skip = Saltar
properties-splits-reset = Reiniciar
properties-splits-note = Los botones controlan el cronómetro en vivo (los atajos globales hacen lo mismo desde cualquier app). La carrera nunca se guarda en el archivo .lss.
hotkeys-split-split = Cronómetro: iniciar / split
hotkeys-split-undo = Cronómetro: deshacer split
hotkeys-split-skip = Cronómetro: saltar segmento
hotkeys-split-reset = Cronómetro: reiniciar
hotkey-audit-action-split-split = Split (cronómetro)
hotkey-audit-action-split-undo = Deshacer split
hotkey-audit-action-split-skip = Saltar segmento
hotkey-audit-action-split-reset = Reiniciar cronómetro
hotkey-audit-feature-split-timer = Cronómetro de splits

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlist
sources-add-playlist = Lista de reproducción (sin cortes)
sources-playlist-title = Añadir una lista de reproducción
sources-playlist-files-label = Archivos (uno por línea, se reproducen de arriba abajo)
sources-playlist-browse = Examinar…
sources-playlist-loop = Bucle
sources-playlist-shuffle = Aleatorio (un sorteo por inicio; en bucle repite ese orden)
sources-playlist-hold-last = Mantener el último fotograma al final
sources-playlist-note = Reproduce toda la lista recortada sin cortes a través del componente ffmpeg etiquetado (solo formatos wire; .frec e imágenes van por Medios/Presentación). Los elementos son todos vídeo o todos audio, nunca mezclados. Recortes, cues y la variable «now playing» están en Propiedades.
sources-playlist-add = Añadir lista
properties-playlist-items = Elementos (de arriba abajo)
properties-playlist-up = Subir
properties-playlist-down = Bajar
properties-playlist-remove = Quitar elemento
properties-playlist-in = Desde (s)
properties-playlist-out = Hasta (s)
properties-playlist-cues = Cues (s, separados por comas)
properties-playlist-add-item = + Añadir elemento
properties-playlist-loop = Bucle
properties-playlist-shuffle = Aleatorio
properties-playlist-hold-last = Mantener último fotograma
properties-playlist-hw = Decodificación por hardware
properties-playlist-variable = Variable «now playing» (vacío = desactivado)
properties-playlist-previous = ⏮ Anterior
properties-playlist-next = ⏭ Siguiente
properties-playlist-note = Los botones de cue y Siguiente/Anterior controlan la lista EN VIVO; los cambios de elementos se aplican con Aplicar (la lista se reinicia). Pon {"{{"}yourVariable{"}}"} en una fuente de Texto para mostrar el elemento en reproducción.
hotkeys-playlist-next = Lista: siguiente elemento
hotkeys-playlist-previous = Lista: elemento anterior
hotkey-audit-action-playlist-next = Lista: siguiente
hotkey-audit-action-playlist-previous = Lista: anterior
hotkey-audit-feature-playlist = Lista de reproducción

# Instant replay source (CAP-N10)
sources-badge-replay = Replay
sources-add-replay = Repetición instantánea
sources-replay-title = Añadir una repetición instantánea
sources-replay-seconds-label = Duración del roll (segundos)
sources-replay-speed-label = Velocidad
sources-replay-speed-full = 100% (con audio)
sources-replay-speed-half = Cámara lenta 50% (sin audio)
sources-replay-speed-quarter = Cámara lenta 25% (sin audio)
sources-replay-note = Permanece transparente hasta que lances la repetición. Arma el búfer de replay (Controles) y asigna la tecla Roll: un roll captura los últimos momentos del búfer, los reproduce en el programa y vuelve a transparente.
sources-replay-add = Añadir repetición
properties-replay-roll = ⏵ Lanzar repetición
properties-replay-note = Roll captura el búfer ARMADO en un clip y lo reproduce a la velocidad elegida — retemporizado, nunca interpolado. La cámara lenta es muda a propósito. Scrub y pausa funcionan durante la reproducción; al final la fuente vuelve a transparente.
hotkeys-replay-roll = Repetición: lanzar
hotkey-audit-action-replay-roll = Lanzar repetición

# Input overlay source (CAP-N13)
sources-badge-input = Entrada
sources-add-input-overlay = Overlay de entrada (teclas/mando)
sources-input-title = Añadir un overlay de entrada
sources-input-layout-label = Disposición
sources-input-layout-wasd = WASD + ratón
sources-input-layout-keyboard = Teclado compacto + ratón
sources-input-layout-gamepad = Mando (dos sticks)
sources-input-layout-fightstick = Stick arcade
sources-input-color-label = Teclas
sources-input-accent-label = Pulsado
sources-input-privacy-note = Privacidad: la entrada se lee solo mientras esta fuente está en vivo en una escena, y solo se consultan las teclas fijas de la disposición — una lectura puntual de «¿está pulsada ahora?», nunca un hook. Nada se registra, se guarda ni se envía a ninguna parte; el texto escrito nunca se captura.
sources-input-os-note = El estado del teclado y el ratón hoy solo se lee en Windows — otros sistemas dibujan las teclas sin pulsar (dicho honestamente, nunca fingido). Los mandos funcionan en todas partes mediante la biblioteca gilrs; se dibuja el primer mando conectado y, si no hay ninguno, la disposición queda sin pulsar.
sources-input-add = Añadir overlay de entrada

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Efectos del cursor
filters-cursorfx-hint = En Windows (que dibuja el cursor por su cuenta) se pintan directamente en la captura, así que aparecen en grabaciones y transmisiones. macOS y Linux componen el cursor en el sistema, por lo que estos efectos son solo de Windows. Los cambios se aplican al instante.
filters-cursorfx-halo = Halo del cursor
filters-cursorfx-halo-color = Color
filters-cursorfx-halo-radius = Radio (px)
filters-cursorfx-ripples = Ondas de clic
filters-cursorfx-left-color = Clic izquierdo
filters-cursorfx-right-color = Clic derecho
filters-cursorfx-keystrokes = Teclas fantasma
filters-cursorfx-keystrokes-hint = Muestra un conjunto fijo de teclas (letras, dígitos, modificadores, flechas) junto al cursor mientras se mantienen pulsadas. Las teclas solo se leen mientras esto está activo, se dibujan directamente en el fotograma y nunca se guardan ni se registran.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Título
sources-add-title = Título / Marcador
sources-title-title = Añadir un título
sources-title-template-label = Empezar desde
sources-title-template-lower-third = Rótulo inferior (barra + nombre + subtítulo)
sources-title-template-scoreboard = Marcador (placa + 4 celdas)
sources-title-template-blank = Lienzo vacío
sources-title-width-label = Ancho del lienzo
sources-title-height-label = Alto del lienzo
sources-title-template-name = Nombre
sources-title-template-subtitle = Título
sources-title-template-home = LOCAL
sources-title-template-away = VISITA
sources-title-note = Títulos por capas (texto / imagen / caja) con animación de entrada/salida, compuestos localmente — sin fuente de navegador. Las capas, los enlaces a archivos y {"{{"}variables{"}}"} y los controles en vivo están en las Propiedades de la fuente.
sources-title-add = Añadir título
properties-title-layers = Capas (dibujadas en orden — las filas posteriores quedan encima)
properties-title-kind-text = Texto
properties-title-kind-image = Imagen
properties-title-kind-rect = Caja
properties-title-x = X
properties-title-y = Y
properties-title-outline = Contorno (px)
properties-title-outline-color = Contorno
properties-title-shadow = Sombra
properties-title-animation = Animación entrada/salida
properties-title-anim-none = Ninguna (corte)
properties-title-anim-fade = Fundido
properties-title-anim-slide-left = Deslizar a la izquierda
properties-title-anim-slide-up = Deslizar hacia arriba
properties-title-anim-wipe = Barrido
properties-title-duration = Duración (ms)
properties-title-fire-in = ▶ Lanzar entrada
properties-title-fire-out = ◼ Lanzar salida
properties-title-set-live = Poner en vivo
properties-title-set-live-note = Empuja este texto al título EN VIVO ahora — sin Aplicar, sin reinicio
properties-title-up = Subir capa
properties-title-down = Bajar capa
properties-title-remove = Quitar capa
properties-title-add-text = + Texto
properties-title-add-image = + Imagen
properties-title-add-rect = + Caja
properties-title-note = Lanzar entrada/salida y «Poner en vivo» controlan el título EN MARCHA; los cambios de capas se aplican con Aplicar (el título se reinicia y vuelve a entrar). Las celdas de texto pueden enlazarse a un archivo vigilado (celda CSV / valor JSON / archivo completo) e interpolar {"{{"}variables{"}}"} — «Poner en vivo» gana a ambos.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = Ingesta LAN (escucha SRT/RTMP)
sources-lan-title = Añadir una escucha de ingesta LAN
sources-lan-protocol-label = Protocolo
sources-lan-protocol-srt = SRT (cifrable — recomendado)
sources-lan-protocol-rtmp = RTMP (sin autenticación)
sources-lan-port-label = Puerto (1024–65535)
sources-lan-passphrase-label = Contraseña (vacía = abierto)
sources-lan-passphrase-hint = Las contraseñas SRT tienen de 10 a 79 caracteres; el emisor debe usar la misma.
sources-lan-open-warning = Sin contraseña: cualquiera en esta red puede alimentar esta fuente, sin cifrar. Pon una salvo que la red sea solo tuya.
sources-lan-rtmp-warning = RTMP no tiene autenticación — cualquiera en esta red puede enviar a este puerto. Prefiere SRT con contraseña.
sources-lan-url-label = Apunta la app del emisor a
sources-lan-qr-aria = Código QR de la URL de ingesta
sources-lan-note = Solo LAN: escucha en la dirección local de esta máquina, solo mientras la fuente exista, y nunca toca internet — nada sale de la máquina hasta que un emisor de tu red envía primero. La decodificación usa el componente ffmpeg claramente etiquetado. El lienzo muestra esta URL hasta que un emisor se conecta.
sources-lan-add = Empezar a escuchar
properties-lan-note = Aplicar un cambio de protocolo, puerto o contraseña reinicia la escucha — el emisor debe reconectarse. El stream se ajusta a un lienzo de 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Enlace
sources-add-freally-link = Freally Link (otra instancia)
sources-link-title = Añadir un Freally Link
sources-link-about = Recibe el programa de otra instancia de Freally Capture — vídeo y audio máster — por tu propia red. Activa primero «Salida Freally Link» en la instancia emisora. v1 transmite motion-JPEG por TCP: ideal en LAN por cable o Wi-Fi buena, honesto con el ancho de banda en enlaces débiles.
sources-link-scan = Buscar en la LAN
sources-link-scanning = Buscando…
sources-link-none = No se encontraron salidas Freally Link. Activa «Salida Freally Link» en la otra instancia (Controles → Panel LAN) o escribe su dirección abajo.
sources-link-host = Dirección
sources-link-port = Puerto
sources-link-key = Clave de emparejamiento
sources-link-key-hint = La clave de los ajustes de «Salida Freally Link» del emisor: sin ella, el emisor no sirve ni un solo fotograma.
sources-link-add = Añadir enlace
properties-link-note = Sin conexión, la fuente muestra una pantalla de «conectando» y reintenta sola con espera creciente — nunca se congela en un fotograma viejo. Un receptor por emisor; a un emisor ocupado se le reintenta con cortesía.
link-title = Salida Freally Link
link-about = Comparte el programa de esta instancia — vídeo y audio máster — con UNA sola instancia más de Freally Capture en tu propia red; allí aparece como fuente «Freally Link» (streaming con dos PC, monitores auxiliares). Desactivado por defecto; nada se anuncia ni escucha hasta activarlo. v1 transmite motion-JPEG + audio sin comprimir por TCP — pensado para LAN por cable o Wi-Fi buena, nunca para internet.
link-enable = Compartir el programa en mi red
link-name = Nombre de la instancia
link-key = Clave de emparejamiento
link-key-hint = Al menos 8 caracteres: los receptores deben introducir esta clave antes de recibir un solo fotograma.
link-lan-warning = ⚠ Los receptores deben presentar la clave de emparejamiento antes de recibir nada, pero el flujo en sí no va cifrado en v1: úsalo solo en una red de confianza.
link-serving = Los receptores pueden encontrar esta instancia con «Buscar en la LAN» o añadirla manualmente en:
link-off-hint = Activa el uso compartido para abrir el puerto y anunciar esta instancia a los escaneos de la LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Menú de la aplicación
menu-file = Archivo
menu-edit = Editar
menu-view = Ver
menu-docks = Docks
menu-profile = Perfil
menu-collection = Colección de escenas
menu-tools = Herramientas
menu-help = Ayuda
menu-rename = Cambiar nombre
menu-remove = Eliminar
menu-import = Importar
menu-export = Exportar
menu-file-show-recordings = Mostrar grabaciones
menu-file-remux = Remultiplexar a MP4…
menu-file-settings = Ajustes…
menu-file-show-settings-folder = Mostrar carpeta de ajustes
menu-file-exit = Salir
menu-edit-undo = Deshacer
menu-edit-redo = Rehacer
menu-edit-history = Historial de ediciones…
menu-edit-copy-transform = Copiar transformación
menu-edit-paste-transform = Pegar transformación
menu-edit-copy-filters = Copiar filtros
menu-edit-paste-filters = Pegar filtros
menu-edit-transform = Transformación…
menu-edit-lock-preview = Bloquear la vista previa
menu-view-fullscreen = Interfaz a pantalla completa
menu-stats-dock = Panel de estadísticas
menu-view-multiview = Monitor multiview…
menu-view-projectors = Proyectores…
menu-view-source-health = Salud de las fuentes…
menu-view-still = Capturar fotograma
menu-docks-browser = Docks de navegador…
menu-docks-lock = Bloquear docks
menu-docks-reset = Restablecer disposición de docks
menu-profile-manage = Gestionar perfiles…
menu-collection-manage = Gestionar colecciones de escenas…
menu-collection-import-obs = Importar desde OBS…
menu-collection-missing = Comprobar archivos faltantes…
menu-tools-wizard = Ejecutar el asistente de configuración
menu-tools-wizard-title = El asistente de configuración se ejecuta en el primer arranque; aún no se puede volver a ejecutar.
menu-tools-automation = Reglas de automatización y macros…
menu-tools-rundown = Mostrar escaleta…
menu-tools-hotkeys = Mapa de atajos…
menu-tools-av-sync = Calibración de sincronía A/V…
menu-tools-scripts = Scripts Lua…
menu-tools-components = Componentes…
menu-tools-midi = Control MIDI…
menu-tools-ptz = Cámaras PTZ…
menu-tools-remote = API de control remoto…
menu-tools-panel = Panel LAN y tally…
menu-help-portal = Portal de ayuda
menu-help-website = Visitar el sitio web
menu-help-discord = Unirse al servidor de Discord
menu-help-bug = Informar de un error…
menu-help-updates = Buscar actualizaciones…
menu-help-whats-new = Novedades
menu-help-about = Acerca de…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Categorías de ajustes
settings-cat-general = General
settings-cat-appearance = Apariencia
settings-cat-streaming = Transmisión
settings-cat-output = Salida
settings-cat-replay = Repetición
settings-cat-hotkeys = Atajos
settings-cat-network = Red
settings-cat-accessibility = Accesibilidad
settings-cat-about = Acerca de
settings-ok = Aceptar
settings-cancel = Cancelar
settings-apply = Aplicar
settings-save = Guardar
settings-loading = Cargando la configuración…
settings-hotkeys-filter = Filtrar atajos
settings-hotkeys-filter-placeholder = Escribe para filtrar acciones o teclas…
settings-hotkeys-no-match = Ningún atajo coincide con “{ $query }”.
settings-hotkey-none = Ninguno
settings-hotkey-group-ctrl = Ctrl + tecla
settings-hotkey-group-ctrl-shift = Ctrl + Shift + tecla
settings-hotkey-group-ctrl-alt = Ctrl + Alt + tecla
settings-hotkey-group-function = Teclas de función
settings-hotkey-group-numpad = Teclado numérico
settings-panic-section = Placa de pánico
settings-meter-section = Medidores de nivel del mezclador
settings-meter-note = Los colores que recorren los medidores de nivel del mezclador de audio, del silencio a la saturación. El preajuste apto para daltonismo usa una rampa de azul → naranja que sigue siendo legible con deficiencia rojo-verde.
settings-meter-preset = Colores del medidor
settings-meter-preset-default = Verde / amarillo / rojo
settings-meter-preset-colorblind = Apto para daltonismo (azul / naranja)
settings-meter-preset-custom = Personalizado
settings-meter-low = Normal
settings-meter-mid = Alto
settings-meter-high = Saturación
settings-meter-preview = Vista previa

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Novedades
whats-new-loading = Cargando notas de la versión…
whats-new-version = Novedades de la versión { $version }
whats-new-empty = No hay notas para esta versión.
filters-name-directional-blur = Desenfoque direccional
filters-name-radial-blur = Desenfoque radial
filters-name-zoom-blur = Desenfoque de zoom
filters-name-pixelate = Pixelar
filters-angle = Ángulo (°)
filters-center-x = Centro X
filters-center-y = Centro Y
filters-block-size = Tamaño de bloque (px)
filters-name-freeze = Congelar
filters-freeze-hint = Mientras está activado, esta fuente mantiene su último fotograma — programa, previsualización, grabación y transmisión se congelan a la vez. Activa o desactiva este filtro para congelar o descongelar.
transform-3d = Inclinación 3D
transform-rotation-x = Inclinación X (°)
transform-rotation-y = Inclinación Y (°)
transform-perspective = Perspectiva
transform-reveal = Mostrar/ocultar
transform-reveal-ms = Fundido de aparición (ms)
sources-clone-title = Clonar (misma fuente, filtros propios)
sources-clone-item = Clonar { $name }
menu-tools-downstream = Keyers de salida…
menu-tools-transition-rules = Reglas de transición…
dsk-title = Keyers de salida
dsk-hint = Superposiciones compuestas sobre la salida del programa — por encima de cada escena, y permanecen al cambiar de escena (un logotipo, una insignia EN VIVO, un rótulo inferior). El primero de la lista se dibuja encima.
dsk-empty = Aún no hay keyers — añade una fuente para superponerla en todas las escenas.
dsk-enable = Activar este keyer
dsk-move-up = Subir (al frente)
dsk-move-down = Bajar
dsk-remove = Eliminar keyer
dsk-opacity = Opacidad
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Escala
dsk-add = + Añadir keyer
transition-rules-title = Reglas de transición
transition-rules-hint = Da a un par de escenas su propia transición. Cuando pasas de la primera escena a la segunda, se usan este tipo y esta duración en lugar de los predeterminados (una regla Stinger/Imagen sigue usando el archivo definido en los controles de transición).
transition-rules-empty = Aún no hay reglas: cada par de escenas usa la transición predeterminada.
transition-rules-from = De
transition-rules-to = A
transition-rules-kind = Transición
transition-rules-duration = Duración (ms)
transition-rules-add = Añadir regla
transition-rules-remove = Quitar regla
