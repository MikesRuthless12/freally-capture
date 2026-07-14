# Freally Capture — pt-BR
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Modo Estúdio
toggle-on = ligado
toggle-off = desligado
stats = Estatísticas
core-ok = núcleo OK
hide-stats-dock = Ocultar o painel de estatísticas
show-stats-dock = Mostrar o painel de estatísticas


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Não foi possível salvar as configurações — a alteração não vai persistir após reiniciar.
studio-mode-leave = Sair do Modo Estúdio
studio-mode-enter-title = Modo Estúdio — edite uma cena de prévia e envie-a ao programa com uma transição
vertical-canvas-title = O segundo canvas de saída (vertical 9:16) — gravável e transmissível de forma independente
app-version = v{ $version }
core-error = núcleo com ERRO
core-unreachable = núcleo inacessível (modo navegador)
connecting-to-core = conectando ao núcleo…
filters-source-fallback = Fonte

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Prévia do programa
preview-program-output = Saída do programa
preview-canvas-editor = Editor de canvas
preview-px-to-edge-label = Pixels até as bordas do quadro
preview-px-to-edge = px até a borda E { $left } · T { $top } · D { $right } · B { $bottom }
preview-program-heading = Programa
preview-no-gpu = Nenhum adaptador de GPU utilizável foi encontrado — o compositor não pode rodar nesta máquina.
preview-starting-compositor = Iniciando o compositor…
preview-empty-scene = Esta cena está vazia — adicione uma fonte em Fontes e depois arraste, redimensione e gire aqui mesmo no canvas.
preview-fps = { $fps } fps
preview-dropped = { $dropped } descartados

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Link de convite recebido
remote-join-with-webcam = Entrar com webcam
remote-dismiss = Dispensar
remote-hosting-guest = Hospedando um convidado remoto
remote-you-are-guest = Você é um convidado remoto
remote-share-view-title = Compartilhe sua tela com o app do convidado (ele vê sua visão ao vivo)
remote-stop-sharing-view = Parar de compartilhar visão
remote-share-my-view = Compartilhar minha visão
remote-allow-center-title = Permita que o convidado troque qual visão fica no centro (você mantém o controle e pode reverter a qualquer momento)
remote-guest-switching = Troca pelo convidado:
remote-stop-screen = Parar tela
remote-share-screen = Compartilhar tela
remote-share-screen-title-guest = Compartilhe sua tela com o anfitrião (ela vira uma fonte que ele pode centralizar)
remote-center-request-label = Solicitação de visão central
remote-center = Centralizar
remote-center-cam-title = Peça ao anfitrião para centralizar sua câmera
remote-center-my-cam = Minha câmera
remote-center-screen-title = Peça ao anfitrião para centralizar sua tela compartilhada
remote-center-my-screen = Minha tela
remote-center-host-title = Devolva o centro para a visão do anfitrião
remote-center-host-view = Visão do anfitrião
remote-end-session = Encerrar sessão
remote-leave = Sair
remote-host-view-heading = Visão do anfitrião
remote-host-shared-view-label = A visão compartilhada do anfitrião
remote-guest-position-label = Posição do convidado
remote-guest-label = Convidado
remote-put-guest = Colocar o convidado { $position }
remote-remove-title = Remover o convidado — ele pode entrar de novo com o mesmo link
remote-remove = Remover
remote-ban-title = Banir o convidado — bloqueia-o e invalida o link de convite
remote-ban = Banir
remote-guest-self-muted = convidado se silenciou
remote-unmute-guest = Reativar áudio do convidado
remote-mute-guest = Silenciar convidado
remote-muted-by-host = Silenciado pelo anfitrião
remote-unmute-mic = Reativar microfone
remote-mute-mic = Silenciar microfone
remote-waiting-for-host = aguardando o anfitrião


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = fonte
sources-fallback-video = vídeo
sources-fallback-error = erro
sources-kind-unknown = ?
sources-missing-source = (fonte ausente)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Tela
sources-badge-window = Janela
sources-badge-portal = Portal
sources-badge-camera = Câmera
sources-badge-image = Imagem
sources-badge-media = Mídia
sources-badge-guest = Convidado
sources-badge-color = Cor
sources-badge-text = Texto
sources-badge-scene = Cena
sources-badge-slides = Slides
sources-badge-chat = Chat
sources-badge-audio-in = Entrada de Áudio
sources-badge-audio-out = Saída de Áudio
sources-badge-app-audio = Áudio de App
sources-badge-test-bars = Barras
sources-badge-test-grid = Grade
sources-badge-test-sweep = Varredura
sources-badge-test-tone = Tom
sources-badge-test-sync = Sinc
sources-badge-timer = Timer

# Add-source menu items
sources-add-display = Captura de Tela
sources-add-window = Captura de Janela
sources-add-game = Captura de Jogo (leia antes)
sources-add-webcam = Dispositivo de Captura de Vídeo
sources-add-image = Imagem
sources-add-media = Mídia (arquivo de vídeo/imagem)
sources-add-remote-guest = Convidado Remoto (teste P2P)
sources-add-color = Cor
sources-add-text = Texto
sources-add-timer = Timer / Relógio
sources-add-nested-scene = Cena Aninhada
sources-add-slideshow = Apresentação de Imagens
sources-add-chat-overlay = Sobreposição de Chat ao Vivo
sources-add-test-signal = Sinal de teste
sources-add-audio-input = Captura de Entrada de Áudio
sources-add-audio-output = Captura de Saída de Áudio
sources-add-app-audio = Áudio de Aplicativo (Windows)
sources-add-existing = Fonte existente…

# Panel header + toolbar buttons
sources-panel-title = Fontes
sources-group-title = Agrupar fontes — escolha dois ou mais itens e depois Criar grupo; itens agrupados movem e mostram/ocultam juntos
sources-group-aria = Agrupar fontes
sources-arrange = Organizar: tela + cantos
sources-add-source = Adicionar uma fonte
sources-browser-source-note = A Fonte de Navegador chega como seu próprio componente sob demanda em um marco futuro (um motor Chromium de ~180 MB — nunca embutido). Hoje: capture uma janela de navegador real com Captura de Janela + um chroma/color key, ou abra chat/alertas como um Dock (Controles → Docks).

# Empty state
sources-empty = Nenhuma fonte nesta cena — adicione uma Captura de Tela, Janela, Webcam, Imagem, Cor ou Texto com "+". Arraste, redimensione e gire no canvas; os botões à direita reordenam a pilha.

# Per-row controls
sources-already-in-group = Já está em { $name }
sources-pick-for-new-group = Escolher para o novo grupo
sources-pick-item-for-group = Escolher { $name } para o novo grupo
sources-hide = Ocultar
sources-show = Mostrar
sources-hide-item = Ocultar { $name }
sources-show-item = Mostrar { $name }
sources-unfocus-title = Desfocar — restaurar o layout
sources-focus-title = Focar — preencher o canvas (Destacar Orador)
sources-unfocus-item = Desfocar { $name }
sources-focus-item = Focar { $name }
sources-center-title = Centralizar — tornar esta a visão central compartilhada (câmeras vão para a barra)
sources-center-item = Centralizar { $name }
sources-rename-item = Renomear { $name }
sources-in-group = No grupo { $name }

# Row status + retry
sources-retry-error = Repetir — { $message }
sources-retry-item = Repetir { $name }
sources-status-error = status: erro
sources-open-privacy-title = Abrir as configurações de privacidade do macOS para esta permissão
sources-open-privacy-item = Abrir configurações de privacidade de { $name }
sources-privacy-settings-button = configurações
sources-status-starting = iniciando…
sources-status-live = ao vivo
sources-status-aria = status: { $state }

# Media row pause/resume
sources-media-resume-title = Retomar o vídeo (ao vivo na transmissão)
sources-media-pause-title = Pausar o vídeo — congela o quadro e silencia, ao vivo na transmissão
sources-media-resume-item = Retomar { $name }
sources-media-pause-item = Pausar { $name }

# Hover controls
sources-unlock = Desbloquear
sources-lock = Bloquear
sources-unlock-item = Desbloquear { $name }
sources-lock-item = Bloquear { $name }
sources-raise-title = Subir na pilha
sources-raise-item = Subir { $name }
sources-lower-title = Descer na pilha
sources-lower-item = Descer { $name }
sources-filters-title = Filtros e mesclagem
sources-filters-item = Filtros de { $name }
sources-properties-title = Propriedades
sources-properties-item = Propriedades de { $name }
sources-remove-title = Remover desta cena
sources-remove-item = Remover { $name }

# Grouping footer
sources-create-group = Criar grupo ({ $count })
sources-cancel = Cancelar

# Groups list
sources-groups-aria = Grupos de fontes
sources-hide-group = Ocultar o grupo
sources-show-group = Mostrar o grupo
sources-item-count = · { $count } itens
sources-ungroup-title = Desagrupar — os itens permanecem onde estão
sources-ungroup-item = Desagrupar { $name }

# Live Chat Overlay picker
sources-chat-title = Adicionar uma Sobreposição de Chat ao Vivo
sources-chat-youtube-label = YouTube — URL de canal, watch ou live_chat (sem chave, sem login)
sources-chat-youtube-placeholder = https://www.youtube.com/@seucanal  ·  ou uma URL watch?v=
sources-chat-twitch-label = Twitch — nome do canal (leitura anônima, sem conta)
sources-chat-twitch-placeholder = seucanal
sources-chat-kick-label = Kick — slug do canal (endpoint público, melhor esforço)
sources-chat-kick-placeholder = seucanal
sources-chat-note = As mensagens aparecem com um horário h:mm:ss AM/PM em um fundo transparente (padrão: canto superior direito; arraste para qualquer lugar). Uma enxurrada de chat apenas descarta as linhas antigas — nunca pode travar a transmissão ou a gravação. O chat do Facebook exige seu próprio token do Graph e ainda não foi implementado — nunca é obrigatório e nunca bloqueia as plataformas acima.
sources-chat-add = Adicionar sobreposição de chat
sources-chat-default-name = Chat ao Vivo

# Image Slideshow picker
sources-slideshow-title = Adicionar uma Apresentação de Imagens
sources-slideshow-empty = Nenhuma imagem ainda — Procurar as adiciona em ordem.
sources-slideshow-remove-slide = Remover slide { $number }
sources-slideshow-browse = Procurar imagens…
sources-slideshow-per-slide-label = Por slide (ms)
sources-slideshow-crossfade-label = Transição cruzada (ms, 0 = corte)
sources-slideshow-loop-label = Repetir (desligado = manter o último slide)
sources-slideshow-shuffle-label = Embaralhar a cada ciclo
sources-slideshow-note = A transição cruzada mescla imagens de tamanho igual; tamanhos diferentes fazem corte seco na fronteira (sem redimensionamento silencioso).
sources-slideshow-add = Adicionar apresentação ({ $count })

# Nested Scene picker
sources-nested-title = Adicionar uma Cena Aninhada
sources-nested-empty = Nenhuma outra cena para aninhar — adicione uma segunda cena primeiro.
sources-nested-scene-name = Cena: { $name }
sources-nested-note = A cena aninhada é renderizada ao vivo no tamanho do canvas do programa e segue suas próprias edições; transformações, filtros e mesclagem se aplicam a ela como a qualquer fonte. Suas fontes de áudio entram no mix enquanto uma cena que a exibe estiver no programa.

# Display / Window capture picker
sources-capture-display-title = Adicionar uma Captura de Tela
sources-capture-window-title = Adicionar uma Captura de Janela
sources-capture-looking = Procurando fontes…
sources-capture-none-displays = Nada para capturar aqui — nenhuma tela foi encontrada.
sources-capture-none-windows = Nada para capturar aqui — nenhuma janela foi encontrada.
sources-capture-portal-note = No Wayland, a caixa de diálogo do sistema escolhe a tela ou janela — os apps não podem capturar globalmente ali, então esse é o caminho honesto (e único).
sources-capture-window-note = As prévias atualizam ao vivo. Uma janela minimizada mostra seu último quadro (ou nenhum) até você restaurá-la.
sources-thumb-no-preview = sem prévia
sources-thumb-loading = carregando…

# Video Capture Device picker
sources-webcam-title = Adicionar um Dispositivo de Captura de Vídeo
sources-webcam-looking = Procurando câmeras…
sources-webcam-none = Nenhuma câmera ou placa de captura foi encontrada.
sources-webcam-format-label = Formato
sources-webcam-format-auto-loading = Automático (carregando formatos…)
sources-webcam-format-auto = Automático (resolução mais alta)
sources-webcam-card-presets-label = Predefinições da placa:
sources-webcam-preset-title = Selecione o modo { $label } que esta placa anuncia
sources-webcam-add = Adicionar câmera

# Audio Input / Output capture picker
sources-audio-output-title = Adicionar uma Captura de Saída de Áudio
sources-audio-input-title = Adicionar uma Captura de Entrada de Áudio
sources-audio-default-output = Saída padrão (o que você ouve)
sources-audio-default-input = Entrada padrão
sources-audio-looking = Procurando dispositivos de áudio…
sources-audio-none-output = Nenhum dispositivo de captura de áudio da área de trabalho foi encontrado aqui.
sources-audio-none-input = Nenhum microfone ou entrada de linha foi encontrado.
sources-audio-input-note = As faixas do mixer ganham um medidor VU, fader, mudo, monitoramento, filtros (redução de ruído, gate, compressor…) e atribuição de trilha. Tudo permanece nesta máquina.

# Application Audio picker
sources-appaudio-title = Adicionar Áudio de Aplicativo
sources-appaudio-looking = Procurando apps emitindo som…
sources-appaudio-none = Nenhum app está emitindo som agora — inicie a reprodução no app e depois atualize.
sources-appaudio-refresh = ⟳ Atualizar
sources-appaudio-note = Captura exatamente o áudio daquele app — com seu próprio VU, fader, mudo, filtros e trilha.

# Game Capture picker
sources-game-title = Captura de Jogo
sources-game-checking = Verificando…
sources-game-use-portal = Usar Captura de Tela (Portal)
sources-game-use-window = Usar Captura de Janela

# Image picker
sources-image-title = Adicionar uma Imagem
sources-image-file-label = Arquivo de imagem (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Adicionar imagem

# Path field
sources-browse = Procurar…

# Media picker
sources-media-title = Adicionar Mídia
sources-media-file-label = Arquivo de mídia (mp4, mkv, webm, mov, .frec ou uma imagem)
sources-media-loop-label = Repetir (reiniciar do começo ao terminar)
sources-media-note = .frec é reproduzido pelo codec próprio freally-video — nada para baixar. Os formatos padrão (mp4/mkv/webm/…) são decodificados pelo componente FFmpeg sob demanda; seu áudio chega ao mixer como uma faixa própria.
sources-media-add = Adicionar mídia

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 hora
sources-ttl-1day = 1 dia

# Remote Guest form
sources-remote-copy-failed = não foi possível copiar — selecione o link e copie manualmente
sources-remote-join-failed = falha ao entrar: { $error }
sources-remote-title = Convidado Remoto (teste P2P)
sources-remote-host-heading = Anfitrião — convide um convidado
sources-remote-start-hosting = Começar a hospedar
sources-remote-expires-label = Expira
sources-remote-invite-expiry-aria = Expiração do convite
sources-remote-invite-link-aria = Link de convite
sources-remote-copied = Copiado ✓
sources-remote-copy = Copiar
sources-remote-share-note = Compartilhe este link (Discord / mensagem / e-mail). Ele carrega sua sessão e expira conforme configurado. O convidado o abre e entra com a webcam.
sources-remote-qr-note = Escaneie no celular para entrar direto pelo navegador — câmera + microfone, sem instalação. O link freally:// copiável acima abre no Freally Capture em uma máquina que o tenha.
sources-remote-guest-heading = Convidado — entre com um convite
sources-remote-paste-placeholder = cole o link de convite
sources-remote-invite-input-aria = Link de convite ou id da sessão
sources-remote-join = Entrar com webcam
sources-remote-session-note = Os controles da sessão ao vivo (mudo, encerrar) ficam na barra no topo da janela principal — você pode fechar esta caixa de diálogo.
sources-remote-stop-session = Parar sessão

# Invite QR
sources-invite-qr-aria = QR code do link de convite

# Remote device pickers
sources-devices-output-unavailable = roteamento de saída indisponível — reproduzindo no dispositivo padrão
sources-devices-mic-test-failed = teste de microfone falhou: { $error }
sources-devices-heading = Dispositivos de áudio da sessão
sources-devices-microphone-label = Microfone
sources-devices-microphone-aria = Microfone da sessão
sources-devices-system-default = Padrão do sistema
sources-devices-output-label = Saída
sources-devices-output-aria = Saída de áudio da sessão
sources-devices-stop-test = Parar teste
sources-devices-test = Testar — ouça a si mesmo
sources-devices-testing-note = fale ao microfone — você está ouvindo os dispositivos selecionados ao vivo
sources-devices-idle-note = repassa seu microfone para a saída (fones evitam microfonia)

# TURN relay section
sources-turn-save-failed = não foi possível salvar: { $error }
sources-turn-summary = Rede — retransmissão TURN opcional (avançado)
sources-turn-note-1 = As sessões conectam diretamente (P2P) — grátis, sem retransmissão. Se AMBOS os lados estiverem atrás de NATs restritos, o caminho direto pode falhar; um relay TURN que você mesmo mantém carrega a mídia então. Pular isto é normal — a maioria das conexões funciona só no modo direto.
sources-turn-note-2 = Opção gratuita: o "Always Free" da Oracle Cloud roda o coturn sem custo (nota: a Oracle pede um cartão de crédito no cadastro, mas a configuração Always-Free continua grátis). Passos: 1) crie a VM gratuita, 2) instale o coturn, 3) abra a UDP 3478, 4) defina usuário/senha, 5) informe turn:ip-da-sua-vm:3478 + as credenciais aqui. Sua credencial fica no seu arquivo de configurações local e nunca é registrada.
sources-turn-url-label = URL do TURN
sources-turn-url-placeholder = turn:host:3478 (vazio = só direto)
sources-turn-url-aria = URL do TURN
sources-turn-username-label = Usuário
sources-turn-username-aria = Usuário do TURN
sources-turn-credential-label = Credencial
sources-turn-credential-aria = Credencial do TURN
sources-turn-note-3 = A retransmissão entra em ação quando os três campos estão preenchidos (um servidor TURN exige as credenciais) e se aplica à próxima sessão que você iniciar ou da qual participar. Verifique com uma chamada de teste somente-relay entre suas duas máquinas.
sources-turn-settings-unavailable = configurações indisponíveis (modo navegador)

# Color picker
sources-color-title = Adicionar uma Cor
sources-color-label = Cor
sources-color-width-label = Largura
sources-color-height-label = Altura
sources-color-add = Adicionar cor
sources-testsignal-title = Adicionar um sinal de teste
sources-testsignal-pattern-label = Padrão
sources-testsignal-bars = Barras de cores SMPTE
sources-testsignal-grid = Grade de calibração
sources-testsignal-sweep = Varredura de movimento
sources-testsignal-tone = Tom de 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Flash + bipe de sincronia A/V
sources-testsignal-note = Verifique cenas, codificadores, projetores e destinos de transmissão sem câmera conectada. O padrão flash + bipe alimenta a bancada de sincronia A/V.
sources-testsignal-add = Adicionar sinal de teste
sources-timer-title = Adicionar um timer
sources-timer-mode-label = Modo
sources-timer-wall-clock = Relógio
sources-timer-countdown = Contagem regressiva
sources-timer-stopwatch = Cronômetro
sources-timer-since-live = Tempo desde o ao vivo
sources-timer-since-recording = Tempo desde a gravação
sources-timer-note = Duração, formato, estilo e ações de fim de contagem ficam nas Propriedades da fonte.
sources-timer-add = Adicionar timer

# Text picker
sources-text-title = Adicionar Texto
sources-text-label = Texto
sources-text-default = Texto
sources-text-color-label = Cor
sources-text-color-aria = Cor do texto
sources-text-size-label = Tamanho (px)
sources-text-note = Família da fonte, alinhamento, quebra de linha e RTL ficam nas Propriedades da fonte. O Noto Sans incluído (com Árabe/Hebraico) é o padrão — idêntico em todas as máquinas.
sources-text-add = Adicionar texto

# Existing source picker
sources-existing-title = Adicionar uma fonte existente
sources-existing-empty = Nenhuma fonte existe ainda — adicione uma a qualquer cena primeiro. Fontes existentes são compartilhadas: renomear ou reconfigurar uma atualiza todas as cenas que a exibem.

# Screen + corners layout
sources-slot-off = Desligado
sources-slot-center = Centro (tela)
sources-slot-top-left = Superior Esquerdo
sources-slot-top-right = Superior Direito
sources-slot-bottom-left = Inferior Esquerdo
sources-slot-bottom-right = Inferior Direito
sources-layout-title = Organizar: Tela + cantos
sources-layout-empty = Adicione uma captura de tela e uma ou mais câmeras a esta cena primeiro, depois organize-as aqui.
sources-layout-note = Coloque uma tela no centro e até quatro câmeras nos cantos — seu layout de tutorial / podcast. Cada canto comporta uma webcam, uma janela de chamada capturada ou um clipe de mídia. Você pode arrastar qualquer um deles no canvas depois.
sources-layout-slot-aria = Espaço para { $name }
sources-layout-apply = Aplicar layout


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Controles
controls-start-stop-title-stop = Parar e finalizar a gravação
controls-start-stop-title-start = Grave o feed do programa com a configuração de Configurações → Saída
controls-finalizing = ◌ Finalizando…
controls-stop-recording = ■ Parar Gravação
controls-start-recording = ● Iniciar Gravação
controls-marker-title = Solte um marcador de capítulo neste momento — ele vai para a GRAVAÇÃO (capítulos mkv ou um arquivo auxiliar). Marcadores de transmissão do lado da plataforma exigem contas de plataforma, que este app nunca pede.
controls-marker = ◈ Marcador
controls-pause-title-resume = Retomar — o arquivo continua como uma única linha do tempo contígua
controls-pause-title-pause = Pausar — nenhum quadro é escrito; retomar continua o mesmo arquivo reproduzível
controls-resume-recording = ▶ Retomar Gravação
controls-pause-recording = ⏸ Pausar Gravação
controls-reactions-label = Reações (embutidas no programa)
controls-reactions-title = Faça uma reação flutuar sobre o programa — gravada E transmitida, para que o replay mostre o momento exato. Espectadores no chat também disparam essas (o emoji de reação deles flutua automaticamente); uma enxurrada apenas limita o que aparece na tela.
controls-react = Reagir { $emoji }
controls-virtual-camera-title = A câmera virtual precisa do seu próprio componente de driver assinado por SO (Win11 MFCreateVirtualCamera / Win10 DirectShow / extensão CoreMediaIO do macOS / v4l2loopback do Linux) — ela chega como seu próprio marco. O modelo de feed já está pronto para ela: programa, canvas vertical ou uma única fonte, com um microfone virtual pareado no Windows/Linux (o macOS não tem API de microfone virtual — dito honestamente).
controls-virtual-camera = ⌁ Iniciar Câmera Virtual
controls-saved = Salvo: { $path }

# --- MixerDock.tsx ---
mixer-title = Mixer de Áudio
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Mudar para faixas horizontais
mixer-switch-to-vertical = Mudar para faixas verticais
mixer-layout-aria-vertical = Layout do mixer: vertical — mudar para horizontal
mixer-layout-aria-horizontal = Layout do mixer: horizontal — mudar para vertical
mixer-empty = Nenhuma fonte de áudio nesta cena — adicione uma Captura de Entrada de Áudio (microfone) ou Captura de Saída de Áudio (áudio da área de trabalho) com "+" em Fontes. As faixas ganham um medidor VU, fader, mudo, monitoramento, filtros e atribuição de trilha.
mixer-advanced-title = Áudio — { $name }
mixer-loudness-label = Loudness do programa (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Loudness momentâneo (400 ms)
mixer-short-term-title = Loudness de curto prazo (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Dispositivo de saída do monitor
mixer-default-output = Saída padrão

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memória
stats-dropped = Descartados
stats-render = Renderização
stats-gpu = GPU
stats-gpu-compositing = compondo
stats-gpu-idle = ocioso
stats-vertical-fps = FPS 9:16
stats-targets-label = Destinos de transmissão
stats-shared-encode = · codificação compartilhada
stats-starting = Iniciando o compositor…

# --- ScenesRail.tsx ---
scenes-title = Cenas
scenes-new-scene-name = Cena
scenes-add = Adicionar uma cena
scenes-empty = Conectando ao núcleo do estúdio…
scenes-rename = Renomear { $name }
scenes-on-program = No programa
scenes-preview = Prévia de { $name }
scenes-switch-to = Mudar para { $name }
scenes-move-up = Mover para cima
scenes-move-up-aria = Mover { $name } para cima
scenes-move-down = Mover para baixo
scenes-move-down-aria = Mover { $name } para baixo
scenes-last-stays = A última cena permanece
scenes-remove = Remover esta cena
scenes-remove-aria = Remover { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Nível
channelstrip-monitor-off = Monitor desligado
channelstrip-monitor-only = Só monitor (fora do mix)
channelstrip-monitor-and-output = Monitor e saída
channelstrip-status-error = erro
channelstrip-status-live = ao vivo
channelstrip-status-waiting-audio = aguardando áudio
channelstrip-status = status: { $state }
channelstrip-status-waiting = aguardando
channelstrip-mute = Silenciar
channelstrip-unmute = Reativar áudio
channelstrip-mute-source = Silenciar { $name }
channelstrip-unmute-source = Reativar áudio de { $name }
channelstrip-scene-mix-on = Mix por cena ATIVO — esta faixa substitui o mix global para esta cena (clique para seguir o mix global de novo)
channelstrip-scene-mix-off = Mix por cena — dê a esta faixa seu próprio fader/mudo para a cena atual
channelstrip-scene-mix-label = Mix por cena de { $name }
channelstrip-monitor-cycle = { $mode } — clique para alternar
channelstrip-monitor-mode = Modo de monitor de { $name }: { $mode }
channelstrip-audio-filters-title = Filtros de áudio (redução de ruído, gate, compressor…)
channelstrip-audio-filters-label = Filtros de áudio de { $name }
channelstrip-advanced-title = Deslocamento de sincronia e atalhos push-to-talk
channelstrip-advanced-label = Configurações avançadas de áudio de { $name }
channelstrip-track-assignment = Atribuição de trilha
channelstrip-track = Trilha { $n }
channelstrip-track-assigned = Trilha { $n } (atribuída)
channelstrip-track-label = Trilha { $n } de { $name }
channelstrip-device-error = erro de dispositivo
channelstrip-audio-device-error = erro de dispositivo de áudio
channelstrip-volume-label = Volume de { $name } em decibéis
channelstrip-ptt-hold = Push-to-talk: segure { $key }
channelstrip-sync-offset = Deslocamento de sincronia (ms, 0–{ $max } — atrasa este áudio)
channelstrip-solo-title = Solo (PFL) — o monitor ouve só as faixas em solo; a mixagem do programa fica intacta
channelstrip-solo-source = Solo de { $name } (PFL)
channelstrip-pan-label = Balanço (duplo clique zera)
channelstrip-pan-aria = Balanço de { $name }
channelstrip-mono-label = Reduzir para mono
channelstrip-ptt-hotkey = Atalho push-to-talk (silencioso a menos que pressionado)
channelstrip-ptt-placeholder = ex.: Ctrl+Shift+T ou F13
channelstrip-ptt-aria = Atalho push-to-talk
channelstrip-ptm-hotkey = Atalho push-to-mute (silencioso enquanto pressionado)
channelstrip-ptm-placeholder = ex.: Ctrl+Shift+M
channelstrip-ptm-aria = Atalho push-to-mute
channelstrip-hotkeys-note = Os atalhos funcionam enquanto outros apps estão em foco. No Linux/Wayland, atalhos globais podem ficar indisponíveis — é um limite do compositor, dito honestamente.
channelstrip-apply = Aplicar

# --- LiveButton.tsx ---
livebutton-failure-ended = a transmissão terminou
livebutton-title-live = Encerrar a transmissão — todos os destinos (uma gravação em andamento continua)
livebutton-title-offline = Transmita para todos os destinos habilitados em Configurações → Transmissão
livebutton-end-stream = ■ Encerrar Transmissão
livebutton-aria-reconnecting = Reconectando
livebutton-aria-live = Ao vivo
livebutton-badge-retry = tentativa { $n }
livebutton-badge-live = ao vivo
livebutton-go-live = ⦿ Transmitir

# --- RecDot.tsx ---
recdot-paused-aria = Gravação pausada
recdot-recording-aria = Gravando
recdot-tracks-one = { $count } trilha de áudio gravando
recdot-tracks-other = { $count } trilhas de áudio gravando
recdot-paused = pausado

# --- ReplayControls.tsx ---
replaycontrols-saved = Replay salvo — { $name }
replaycontrols-failure-stopped = o buffer parou
replaycontrols-title-disarm = Desarmar o buffer de replay (descarta o histórico não salvo)
replaycontrols-title-arm = Arme o buffer de replay contínuo — mantém os últimos N segundos prontos para salvar (com sua própria codificação leve; a transmissão e a gravação ficam intactas)
replaycontrols-replay-seconds = ⟲ Replay { $seconds }s
replaycontrols-arm = ⟲ Armar Buffer de Replay
replaycontrols-save-title = Salve os últimos N segundos na pasta de gravações (também no atalho Salvar Replay)
replaycontrols-save = ⤓ Salvar

# --- PropertiesDialog.tsx ---
properties-title = Propriedades — { $name }
properties-name = Nome
properties-cancel = Cancelar
properties-apply = Aplicar
properties-youtube = YouTube — URL de canal / watch / live_chat (sem chave, sem login, nunca)
properties-twitch = Twitch — nome do canal (anônimo)
properties-kick = Kick — slug do canal (endpoint público)
properties-width-px = Largura (px)
properties-lines = Linhas
properties-font-px = Fonte (px)
properties-images = Arquivos de imagem (um caminho por linha, exibidos em ordem)
properties-per-slide = Por slide (ms)
properties-crossfade = Transição cruzada (ms, 0 = corte)
properties-loop-slideshow = Repetir (desligado = manter o último slide)
properties-shuffle = Embaralhar a cada ciclo
properties-nested-scene = Cena que esta fonte compõe (uma cena que já contém esta é rejeitada)
properties-portal-note = O portal ScreenCast do Wayland escolhe a tela ou janela na caixa de diálogo do sistema toda vez que esta fonte inicia — não há nada para configurar aqui, por design.
properties-appaudio-capturing = Capturando o áudio de { $exe }
properties-appaudio-exe-fallback = um aplicativo
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Adicione a fonte de novo para mirar em outro app (um id de processo muda quando o app reinicia).
properties-image-file = Arquivo de imagem
properties-media-file = Arquivo de mídia (mp4, mkv, webm, mov, .frec ou uma imagem)
properties-media-loop = Repetir (reiniciar do começo ao terminar)
properties-media-hwdecode = Decodificação por hardware (recai para software por conta própria)
properties-media-note = .frec é reproduzido pelo codec próprio freally-video — nada para baixar. Outros formatos de vídeo são decodificados pelo componente FFmpeg sob demanda. O áudio do arquivo ganha sua própria faixa no mixer; o deslocamento de sincronia da faixa ajusta finamente o alinhamento A/V. Um clipe sem áudio deixa sua faixa em silêncio.
properties-color = Cor
properties-width = Largura
properties-height = Altura
properties-testtone-note = Uma senoide contínua de 1 kHz a −20 dBFS. Nível e mudo ficam na faixa do mixer; não há mais nada para configurar.
properties-timer-format = Formato de hora (strftime)
properties-timer-format-note = ex.: %H:%M:%S (padrão), %I:%M %p, %A %H:%M — um padrão inválido volta a %H:%M:%S.
properties-timer-utc = Deslocamento UTC (minutos)
properties-timer-utc-placeholder = hora local
properties-timer-duration = Duração (segundos)
properties-timer-target = Contar até (HH:MM)
properties-timer-target-note = Um alvo de relógio corre sozinho e se repete diariamente; deixe vazio para usar a duração com Iniciar/Pausar/Zerar.
properties-timer-end = No zero
properties-timer-end-none = Não fazer nada
properties-timer-end-flash = Piscar o timer
properties-timer-end-switch = Trocar de cena
properties-timer-end-scene = Cena
properties-timer-size = Tamanho (px)
properties-timer-start = Iniciar
properties-timer-pause = Pausar
properties-timer-reset = Zerar
properties-text-file = Ler de arquivo (caminho; vazio = usar o texto acima)
properties-text-binding = Interpretar como
properties-text-binding-whole = Arquivo inteiro
properties-text-binding-csv = Célula CSV
properties-text-binding-json = Ponteiro JSON
properties-text-csv-row = Linha
properties-text-csv-column = Coluna
properties-text-csv-column-placeholder = nome ou número
properties-text-json-pointer = Ponteiro
properties-text-file-note = O arquivo é relido em meio segundo após uma mudança. Escritas atômicas (temp + renomear) são toleradas: o último valor bom permanece na tela durante a troca.
avsync-title = Calibração de sincronia A/V
avsync-intro = Reproduza o padrão integrado de flash + bipe na tela e nos alto-falantes, capture-o com a câmera e o microfone que você quer alinhar — a bancada mede a diferença. O ciclo passa pela tela e pelos alto-falantes, então as pequenas latências deles ficam incluídas.
avsync-video-label = Câmera (fonte de vídeo)
avsync-audio-label = Microfone (fonte de áudio)
avsync-pick = Escolha uma fonte…
avsync-no-video = Adicione a câmera como fonte primeiro — a bancada mede fontes, não dispositivos brutos.
avsync-no-audio = Adicione o microfone como fonte de áudio primeiro.
avsync-projector = Programa em tela cheia em
avsync-projector-open = Abrir projetor
avsync-projector-window-title = Programa — sincronia A/V
avsync-start-note = Ao iniciar, uma fonte temporária "Padrão de sincronia A/V" é adicionada sobre a cena atual e o bipe toca no dispositivo de monitoração. Tudo é removido ao terminar.
avsync-manual = Deslocamento de sincronia (ms, manual)
avsync-start = Iniciar calibração
avsync-measuring = Medindo por cerca de 12 segundos — aponte a câmera para o programa piscante e mantenha a sala calma…
avsync-flash-seen = A câmera vê o flash
avsync-flash-waiting = Aguardando a câmera ver o flash…
avsync-beep-heard = O microfone ouve o bipe
avsync-beep-waiting = Aguardando o microfone ouvir o bipe…
avsync-cancel = Cancelar
avsync-result-offset = O vídeo chega { $offset } ms depois do áudio.
avsync-result-detail = Medido em { $cycles } ciclos, ±{ $jitter } ms.
avsync-negative = O áudio já chega depois do vídeo. Atrasar o áudio não corrige essa direção — se outra faixa carrega o som desta câmera, reduza lá o deslocamento.
avsync-over-cap = A diferença medida passa do teto de { $max } ms. Uma lacuna dessas costuma indicar fonte errada — confira a cadeia e meça de novo.
avsync-applied = Aplicado — o deslocamento do microfone agora é { $offset } ms.
avsync-apply = Aplicar { $offset } ms ao microfone
avsync-again = Medir novamente
avsync-close = Fechar
avsync-error-noFlash = A câmera nunca viu o flash. Aponte-a para o programa piscante (tela cheia ajuda), confirme que a fonte está ativa e meça de novo.
avsync-error-noBeep = O microfone nunca ouviu o bipe. Confirme que o dispositivo de monitoração está audível e que o microfone está ativo (sem push-to-talk), e meça de novo.
avsync-error-tooFewCycles = Ciclos limpos de flash/bipe insuficientes. Mantenha o padrão bem visível e audível durante toda a medição.
avsync-error-notThePattern = O que foi visto ou ouvido não se repete no ritmo do padrão — provavelmente luz ou ruído da sala, não o sinal de teste.
avsync-error-unstable = Os ciclos divergem demais para confiar em um único número. Estabilize a câmera, reduza o ruído e meça de novo.
hotkey-audit-title = Mapa de atalhos
hotkey-audit-search = Buscar
hotkey-audit-filter = Recurso
hotkey-audit-filter-all = Todos os recursos
hotkey-audit-col-key = Tecla
hotkey-audit-col-action = Ação
hotkey-audit-col-where = Onde
hotkey-audit-col-status = Estado
hotkey-audit-ok = OK
hotkey-audit-shared = Compartilhada por { $count } vínculos
hotkey-audit-unregistered = Não registrada no SO (ocupada em outro lugar ou indisponível)
hotkey-audit-invalid = Atalho inválido
hotkey-audit-empty = Nenhum atalho ainda — vincule em Configurações → Atalhos ou numa faixa do mixer.
hotkey-audit-export = Exportar cola
hotkey-audit-exported = Salvo em { $path }
hotkey-audit-note = Vincule e altere teclas em Configurações → Atalhos (ações globais) e em cada faixa do mixer (push-to-talk / push-to-mute); esta tabela audita e documenta tudo.
hotkey-audit-action-record = Alternar gravação
hotkey-audit-action-go-live = Alternar transmissão
hotkey-audit-action-transition = Executar transição
hotkey-audit-action-save-replay = Salvar replay
hotkey-audit-action-add-marker = Adicionar marcador
hotkey-audit-action-still = Capturar still
hotkey-audit-action-panic = Tela de pânico
hotkey-audit-action-timer-toggle = Iniciar/pausar todos os timers
hotkey-audit-action-timer-reset = Zerar todos os timers
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Gravação
hotkey-audit-feature-streaming = Transmissão
hotkey-audit-feature-studio = Modo estúdio
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Marcadores
hotkey-audit-feature-stills = Stills
hotkey-audit-feature-panic = Pânico
hotkey-audit-feature-timers = Timers
hotkey-audit-feature-audio = Áudio (por fonte)
properties-text = Texto
properties-font-family = Família da fonte (do sistema; em branco = padrão)
properties-size-px = Tamanho (px)
properties-text-color = Cor do texto
properties-align = Alinhamento
properties-align-left = esquerda
properties-align-center = centro
properties-align-right = direita
properties-line-spacing = Espaçamento entre linhas
properties-wrap-width = Largura de quebra (px; 0 = desligado)
properties-force-rtl = Forçar da direita para a esquerda
properties-text-note = A renderização usa shaping real (junção árabe, ligaduras) e ordenação bidi de linha. A família Noto Sans incluída (com Árabe/Hebraico) é a padrão; famílias do sistema também funcionam. CJK usa fontes do sistema por enquanto.
properties-repick-capturing = Capturando: { $label }
properties-repick-looking = Procurando fontes…
properties-repick-none-displays = Nenhuma tela encontrada para reescolher.
properties-repick-none-windows = Nenhuma janela encontrada para reescolher.
properties-repick-again = Escolher de novo:
properties-device = Dispositivo
properties-video-current-device = (dispositivo atual)
properties-format = Formato
properties-format-auto-loading = Automático (carregando formatos…)
properties-deinterlace = Desentrelaçamento
properties-deinterlace-off = Desligado
properties-deinterlace-discard = Descartar (duplicar linhas de um campo)
properties-deinterlace-bob = Bob (alternar campos)
properties-deinterlace-linear = Linear (interpolar)
properties-deinterlace-blend = Mesclar (média dos campos)
properties-deinterlace-adaptive = Adaptativo a movimento (classe yadif)
properties-field-order = Ordem dos campos
properties-field-order-top = Campo superior primeiro
properties-field-order-bottom = Campo inferior primeiro
properties-deinterlace-note = Para sinais entrelaçados de placas de captura. CPU pura, idêntico em todos os SOs; mudar reinicia o dispositivo (como trocar o formato).
camera-controls-title = Controles da câmera
camera-controls-refresh = Atualizar
camera-controls-reset = Redefinir perfil
camera-controls-empty = Sem controles agora — o dispositivo precisa estar transmitindo (adicione-o a uma cena primeiro), e alguns backends não informam nenhum (sobretudo macOS). É o estado honesto por SO.
camera-controls-note = As mudanças valem ao vivo e são salvas no perfil do dispositivo, reaplicado ao reconectar e ao reiniciar.
camera-control-brightness = Brilho
camera-control-contrast = Contraste
camera-control-hue = Matiz
camera-control-saturation = Saturação
camera-control-sharpness = Nitidez
camera-control-gamma = Gama
camera-control-white-balance = Balanço de branco
camera-control-backlight = Compensação de contraluz
camera-control-gain = Ganho
camera-control-pan = Pan
camera-control-tilt = Tilt
camera-control-zoom = Zoom
camera-control-exposure = Exposição
camera-control-iris = Íris
camera-control-focus = Foco
properties-format-auto = Automático (resolução mais alta)
properties-audio-capture-of = Capturar o áudio de
properties-audio-default-output = Saída padrão (o que você ouve)
properties-audio-default-input = Entrada padrão
properties-audio-default-suffix = (padrão)
properties-audio-current-device = (dispositivo atual: { $id })

# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Ganho
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Compressor
audiofilters-name-limiter = Limitador
audiofilters-name-eq = EQ de 3 Bandas
audiofilters-name-denoise = Redução de Ruído
audiofilters-name-ducking = Ducking
audiofilters-title = Filtros de áudio — { $name }
audiofilters-chain-header = Cadeia de filtros (o de cima roda primeiro, antes do fader)
audiofilters-add = + Adicionar filtro
audiofilters-add-menu = Adicionar um filtro de áudio
audiofilters-empty = Nenhum filtro ainda — reduza o ruído de um microfone (DSP clássico, sem ML), aplique um gate no ambiente, controle picos com o compressor ou abaixe a música sob sua voz.
audiofilters-enable = Habilitar { $name }
audiofilters-run-earlier = Rodar antes
audiofilters-move-up = Mover { $name } para cima
audiofilters-run-later = Rodar depois
audiofilters-move-down = Mover { $name } para baixo
audiofilters-remove-title = Remover filtro
audiofilters-remove = Remover { $name }
audiofilters-gain-db = Ganho (dB)
audiofilters-open-db = Abrir em (dB)
audiofilters-close-db = Fechar em (dB)
audiofilters-attack-ms = Ataque (ms)
audiofilters-hold-ms = Sustentação (ms)
audiofilters-release-ms = Liberação (ms)
audiofilters-ratio = Proporção (:1)
audiofilters-threshold-db = Limiar (dB)
audiofilters-output-gain-db = Ganho de saída (dB)
audiofilters-ceiling-db = Teto (dB)
audiofilters-low-db = Graves (dB)
audiofilters-mid-db = Médios (dB)
audiofilters-high-db = Agudos (dB)
audiofilters-strength = Intensidade
audiofilters-denoise-note = Supressão espectral própria de DSP clássico — ruído constante (ventiladores, chiado) cai enquanto a fala passa. Sem ML, sem modelos, conforme a carta.
audiofilters-duck-under = Abaixar sob
audiofilters-ducking-trigger = Fonte de disparo do ducking
audiofilters-pick-trigger = (escolha um disparo — ex.: seu microfone)
audiofilters-trigger-at-db = Disparar em (dB)
audiofilters-duck-by-db = Abaixar em (dB)

# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Atraso de Renderização
filters-name-color-correction = Correção de Cor
filters-name-lut = Aplicar LUT
filters-name-blur = Desfoque
filters-name-mask = Máscara de Imagem
filters-name-sharpen = Nitidez
filters-name-scroll = Rolagem
filters-name-crop = Recorte
filters-title = Filtros — { $name }
filters-blend-mode = Modo de mesclagem
filters-chain-header = Cadeia de filtros (o de cima roda primeiro)
filters-add = + Adicionar filtro
filters-add-menu = Adicionar um filtro
filters-empty = Nenhum filtro ainda — aplique chroma key em uma webcam, corrija a cor de uma captura ou role um letreiro.
filters-enable = Habilitar { $name }
filters-run-earlier = Rodar antes
filters-move-up = Mover { $name } para cima
filters-run-later = Rodar depois
filters-move-down = Mover { $name } para baixo
filters-remove-title = Remover filtro
filters-remove = Remover { $name }
filters-key-color-rgb = Cor-chave (qualquer cor, distância RGB)
filters-similarity = Similaridade
filters-smoothness = Suavidade
filters-luma-min = Luma mín (chaveia mais escuros)
filters-luma-max = Luma máx (chaveia mais claros)
filters-delay = Atraso (ms — só vídeo, ex.: para sincronizar com o áudio; limitado a 500)
filters-key-color = Cor-chave
filters-spill = Vazamento
filters-gamma = Gama
filters-brightness = Brilho
filters-contrast = Contraste
filters-saturation = Saturação
filters-hue-shift = Deslocamento de matiz
filters-opacity = Opacidade
filters-cube-file = arquivo .cube
filters-amount = Quantidade
filters-radius = Raio
filters-mask-image = Imagem de máscara
filters-mask-mode = Modo
filters-mask-alpha = alfa
filters-mask-luma = luma
filters-mask-invert = inverter
filters-speed-x = Velocidade X (px/s)
filters-speed-y = Velocidade Y (px/s)
filters-crop-left = esquerda
filters-crop-top = topo
filters-crop-right = direita
filters-crop-bottom = base
filters-crop-aria = recortar { $side }

# --- PickerShell.tsx ---
pickershell-refresh-aria = Atualizar
pickershell-refresh-title = Atualizar a lista
pickershell-close = Fechar


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Relatar um bug
bugreport-intro = Os relatórios são anônimos e opcionais — nada é enviado automaticamente. Você revisará o texto exato abaixo e depois o enviará por uma issue do GitHub pré-preenchida ou pelo seu app de e-mail. Sem dados pessoais (seu caminho de usuário e nome de usuário são ocultados); sem conta, sem servidor.
bugreport-crash-notice = O Freally Capture fechou inesperadamente em uma execução anterior — os detalhes anônimos da falha estão incluídos abaixo. Relatá-los ajuda a corrigir rápido.
bugreport-description-label = O que você estava fazendo quando aconteceu? (opcional)
bugreport-description-placeholder = ex.: a prévia travou quando adicionei uma segunda webcam
bugreport-include-crash = Incluir os detalhes anônimos da falha da última execução
bugreport-preview-label = Exatamente o que será enviado
bugreport-open-github = Abrir issue no GitHub
bugreport-gmail-title = Abre a janela de composição do Gmail no seu navegador, pré-preenchida. Desconectado? O Google mostra a tela de login primeiro.
bugreport-compose-gmail = Compor no Gmail
bugreport-email-title = Abre um rascunho no app de e-mail padrão deste PC (Outlook, Thunderbird, Mail…)
bugreport-send-email = Enviar e-mail
bugreport-copied = Copiado ✓
bugreport-copy-report = Copiar relatório
bugreport-dismiss-crash = Dispensar falha
bugreport-copy-failed = não foi possível copiar — selecione o texto e copie manualmente
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = O QUE ACONTECEU
bugreport-preview-no-description = (nenhuma descrição fornecida)
bugreport-preview-diagnostics = DIAGNÓSTICOS ANÔNIMOS (sem dados pessoais)
bugreport-preview-from = De: Freally Capture
bugreport-preview-crash-excerpt = --- trecho da falha ---

# --- Updates.tsx ---
updates-title = Atualização de software
updates-checking = Verificando atualizações…
updates-uptodate = Você está na versão mais recente.
updates-check-again = Verificar de novo
updates-available = A versão { $version } está disponível
updates-current-version = (você tem a { $current })
updates-release-notes-label = Versão { $version } — Notas da versão
updates-confirm = Deseja atualizar agora? O download é verificado contra a chave de assinatura incluída antes de ser aplicado. O Freally Capture fecha, o instalador roda e a nova versão reabre sozinha.
updates-yes-update-now = Sim, atualizar agora
updates-no-not-now = Não, agora não
updates-downloading = Baixando { $version }…
updates-starting = iniciando…
updates-installed = Atualização instalada.
updates-restart-now = Reiniciar agora
updates-restart-later = Reiniciar depois
updates-try-again = Tentar de novo

# --- Models.tsx ---
models-title = Componentes
models-ffmpeg-heading = FFmpeg — codecs padrão
models-badge-third-party = De terceiros · não embutido
models-ffmpeg-desc = O motor próprio do Freally Capture grava freally-video (.frec) sem perdas, sem nada extra. Gravar os formatos padrão que as plataformas e os reprodutores esperam — H.264/AAC (e HEVC/AV1) em mp4/mkv/mov/webm — usa o FFmpeg, uma ferramenta separada com a qual este app nunca é distribuído: esses codecs têm patentes, então ele permanece opcional e claramente identificado. Ele é baixado sob demanda a partir do build fixado abaixo, verificado por SHA-256 antes do primeiro uso, armazenado em cache por usuário e executado como um processo separado. Sua licença (LGPL/GPL) é própria — veja THIRD-PARTY-NOTICES.
models-checking = Verificando…
models-ffmpeg-not-installed = Não instalado. Disponível: FFmpeg { $version } de { $source } (download de { $size }).
models-ffmpeg-none-pinned = Nenhum build do FFmpeg está fixado para esta plataforma ainda — a gravação em codecs padrão está indisponível aqui. A gravação freally-video sem perdas não é afetada.
models-ffmpeg-download-verify = Baixar e verificar ({ $size })
models-downloading = Baixando…
models-download-of = de
models-cancel = Cancelar
models-ffmpeg-verifying = Verificando o download contra o SHA-256 fixado…
models-ffmpeg-extracting = Descompactando…
models-ffmpeg-ready = Instalado e verificado — { $version }
models-remove = Remover
models-ffmpeg-retry = Repetir download
models-network-note = O download é a única ação de rede neste painel e nunca começa sozinho. Uma soma de verificação inválida aborta a instalação — o app se recusa a rodar bytes pelos quais não pode responder.
models-cef-heading = Runtime da Fonte de Navegador — Chromium (CEF)
models-cef-desc = As fontes de navegador renderizam páginas web (alertas, widgets, sobreposições) pelo Chromium Embedded Framework — um runtime de ~100 MB com o qual este app nunca é distribuído. Ele baixa sob demanda a partir do índice oficial de builds do CEF, é verificado contra o SHA-1 desse índice antes de qualquer coisa ser descompactada e é armazenado em cache por usuário. A fonte de navegador que renderiza por ele chega com seu próprio marco; isto instala o runtime de que ela precisa.
models-cef-download-install = Baixar e instalar
models-cef-unsupported = O CEF não publica build para esta plataforma — as fontes de navegador estão indisponíveis aqui.
models-cef-resolving = Resolvendo o build estável mais recente…
models-cef-verifying = Verificando o download contra o SHA-1 do índice…
models-cef-extracting = Descompactando o runtime…
models-cef-ready = Instalado — CEF { $version }.
models-cef-retry = Repetir
models-integrations-heading = Integrações opcionais
models-badge-never-bundled = Nunca embutido
models-ndi-detected = Detectado
models-ndi-not-installed = Não instalado
models-vst-available = Disponível
models-vst-not-available = Indisponível

# --- Recordings.tsx ---
recordings-title = Gravações
recordings-loading = Lendo a pasta…
recordings-empty = Nenhuma gravação ainda — Iniciar Gravação escreve na pasta definida em Saída.
recordings-frec-label = próprio sem perdas (freally-video)
recordings-remux-title = Reempacotar como mp4 — cópia de stream, sem recodificar, sem mudança de qualidade (precisa do componente FFmpeg)
recordings-remuxing = Remuxando…
recordings-remux-to-mp4 = Remuxar para MP4
recordings-export-mp4-title = Decodifique o .frec próprio e recodifique para MP4 (H.264/AAC) para tocar em qualquer reprodutor — precisa do componente FFmpeg
recordings-exporting = Exportando…
recordings-export-mp4 = Exportar → MP4
recordings-export-mkv-title = Decodifique o .frec próprio e recodifique para MKV para tocar em qualquer reprodutor
recordings-starting = iniciando…
recordings-frames = { $done } / { $total } quadros
recordings-cancel = Cancelar
recordings-export-cancelled = Exportação cancelada.
recordings-exported-to = Exportado para { $path }
recordings-remuxed-to = Remuxado para { $path }

# --- OpenedFrec.tsx ---
openfrec-title = Abrir gravação .frec
openfrec-desc = O Freally Capture grava no formato próprio sem perdas .frec — ele não o reproduz. O Freally Player reproduzirá .frec diretamente quando for lançado. Por enquanto, exporte para MP4/MKV e ele toca em qualquer reprodutor (VLC, o reprodutor do seu SO, qualquer um).
openfrec-exported-to = Exportado para { $path }
openfrec-exporting = Exportando…
openfrec-starting = iniciando…
openfrec-export-mp4 = Exportar → MP4
openfrec-export-mkv = Exportar → MKV

# --- VerticalCanvasDialog.tsx ---
vertical-title = Canvas vertical (9:16)
vertical-enable = Habilite o segundo canvas — gravável e transmissível de forma independente do programa
vertical-scene-label = Cena que este canvas compõe
vertical-width = Largura
vertical-height = Altura
vertical-preview-alt = Prévia do canvas vertical
vertical-note = As posições dos itens são fiéis ao pixel entre os canvas: selecione esta cena na barra de Cenas para organizá-la enquanto esta prévia mostra o resultado vertical. Os destinos de transmissão escolhem este canvas em ⦿ Transmissão…; Configurações → Saída pode gravá-lo junto do arquivo principal.
vertical-close = Fechar

# --- EulaGate.tsx ---
eula-title = Freally Capture — Contrato de Licença
eula-version = v{ $version }
eula-intro = Por favor, leia e aceite este contrato para usar o Freally Capture. Em resumo: é uma ferramenta neutra, e você é o único responsável pelo que captura, grava e transmite — e por ter os direitos sobre isso.
eula-thanks = Obrigado por ler.
eula-scroll-hint = Role até o fim para continuar.
eula-decline = Recusar e Sair
eula-agree = Concordo


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Saída
output-loading = As configurações ainda estão carregando…
output-container-frec = freally-video (.frec) — sem perdas, próprio, nada para baixar
output-container-mkv = MKV — tolerante a falhas; remuxe para mp4 depois
output-container-mp4 = MP4 — toca em todo lugar
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Sem perdas
output-preset-lossless-title = O codec próprio freally-video — exato ao bit, sem download
output-preset-high-label = Alta qualidade
output-preset-high-title = MP4, melhor codificador detectado, CQ 16 quase sem perdas, predefinição Qualidade
output-preset-balanced-label = Equilibrado
output-preset-balanced-title = MKV, melhor codificador detectado, CQ 23, predefinição Equilibrado
output-recording-format = Formato de gravação
output-ffmpeg-warning = Este formato precisa do componente FFmpeg (codecs padrão — não embutido). O .frec sem perdas não precisa de nada.
output-install = Instalar…
output-recordings-folder = Pasta de gravações
output-folder-placeholder = Pasta Vídeos do SO
output-filename-prefix = Prefixo do nome de arquivo
output-recording-template = Nome de arquivo das gravações
output-replay-template = Nome de arquivo dos replays
output-still-template = Nome de arquivo dos quadros
output-template-tokens = Tokens: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Pasta de replays
output-still-folder = Pasta de quadros
output-same-folder-placeholder = Pasta de gravações
output-frame-rate = Taxa de quadros
output-fps-option = { $fps } fps
output-split-every = Dividir a cada (minutos, 0 = desligado)
output-output-width = Largura de saída (0 = canvas; só formatos padrão)
output-output-height = Altura de saída (0 = canvas)
output-record-vertical = Gravar também o canvas vertical (um arquivo paralelo "… (vertical)"; precisa do canvas 9:16 habilitado)
output-audio-tracks = Trilhas de áudio
output-recorded-tracks-group = Trilhas gravadas
output-track-last-one = Pelo menos uma trilha deve gravar
output-record-track-on = Gravar trilha { $index }: ligada
output-record-track-off = Gravar trilha { $index }: desligada
output-encoder-heading = Codificador
output-video-encoder = Codificador de vídeo
output-encoder-auto = Automático — melhor detectado (H.264)
output-encoder-unavailable = — indisponível aqui
output-preset = Predefinição
output-preset-quality = Qualidade
output-preset-balanced-option = Equilibrado
output-preset-performance = Desempenho
output-rate-control = Controle de taxa
output-rc-cqp = CQP (qualidade constante)
output-rc-cbr = CBR (taxa de bits constante)
output-rc-vbr = VBR (taxa de bits variável)
output-cq = CQ (0–51, menor = melhor)
output-bitrate = Taxa de bits (kbps)
output-keyframe = Intervalo de keyframe (s)
output-audio-bitrate = Taxa de bits de áudio (kbps / trilha)
output-presets = Predefinições:

# --- SettingsStream.tsx ---
stream-title = Configurações — Transmissão
stream-target-enabled = Destino { $index } habilitado
stream-target = Destino { $index }
stream-remove = Remover
stream-service = Serviço
stream-canvas = Canvas
stream-canvas-main = Principal (programa)
stream-canvas-vertical = Vertical (9:16 — habilite-o no estúdio)
stream-ingest-srt = URL de ingest SRT
stream-ingest-whip = URL do endpoint WHIP
stream-ingest-url = URL de ingest
stream-ingest-override = (substituição — vazio = a predefinição do serviço)
stream-key-srt = streamid (opcional — anexado como ?streamid=…; tratado como segredo)
stream-key-whip = Token Bearer (opcional — enviado como o cabeçalho Authorization; um segredo)
stream-key-custom = Chave de transmissão (do seu servidor — tratada como segredo)
stream-key-service = Chave de transmissão (do seu painel de criador — tratada como segredo)
stream-key-aria = Chave de transmissão { $index }
stream-key-hide = Ocultar
stream-key-show = Mostrar
stream-encoder = Codificador (H.264 — o que RTMP, SRT e WHIP todos carregam)
stream-encoder-auto = Automático — o melhor codificador H.264 detectado
stream-encoder-unavailable = (indisponível aqui)
stream-video-bitrate = Taxa de bits de vídeo (kbps, CBR)
stream-audio-bitrate = Taxa de bits de áudio (kbps)
stream-fps = FPS
stream-keyframe = Intervalo de keyframe (s)
stream-audio-track = Trilha de áudio (1–6)
stream-output-width = Largura de saída (0 = canvas)
stream-output-height = Altura de saída (0 = canvas)
stream-add-target = + Adicionar destino
stream-go-live-note = O Transmitir publica em todos os destinos habilitados de uma vez, direto para cada plataforma. Destinos com configurações de codificador idênticas compartilham uma única codificação.
stream-auto-record = Iniciar gravação quando eu transmitir (a gravação ainda para de forma independente)
stream-ffmpeg-note-before = Os codecs padrão de transmissão rodam pelo componente ffmpeg sob demanda identificado —
stream-ffmpeg-note-link = gerencie-o aqui
stream-ffmpeg-note-after = . A gravação local continua rodando não importa o que a transmissão faça.
stream-cancel = Cancelar
stream-save = Salvar

# --- SettingsReplay.tsx ---
replay-title = Configurações — Buffer de Replay
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Baixa (3 Mbps)
replay-quality-standard = Padrão (6 Mbps)
replay-quality-high = Alta (12 Mbps)
replay-length-presets = Predefinições de duração
replay-quality-presets = Predefinições de qualidade
replay-length-seconds = Duração (segundos)
replay-video-bitrate = Taxa de bits de vídeo (kbps)
replay-fps = FPS
replay-audio-track = Trilha de áudio (1–6)
replay-note = Enquanto armado, o buffer roda sua própria codificação leve em um anel limitado no disco — cerca de { $mb } MB nestas configurações. Salvar costura o anel sem recodificar e nunca toca na transmissão ou na gravação. As alterações se aplicam na próxima vez que você armar.
replay-cancel = Cancelar
replay-save = Salvar

# --- SettingsRemote.tsx ---
remote-title = Configurações — Controle Remoto
remote-enable = Habilitar a API remota WebSocket
remote-password = Senha (obrigatória — os controladores se autenticam com ela)
remote-password-placeholder = uma senha para seus controladores
remote-password-hide = Ocultar
remote-password-show = Mostrar
remote-port = Porta
remote-allow-lan = Permitir conexões LAN (o padrão é somente esta máquina)
remote-note = Desligado = a porta fica fechada. Ligado = um WebSocket protegido por senha em 127.0.0.1 (ou sua LAN quando optado) que pode trocar cenas, rodar a transição, iniciar/parar a transmissão e a gravação, salvar replays e definir mudos/volumes — as mesmas ações da UI, nada mais. Ele não pode ler arquivos. Trate a senha como qualquer credencial; prefira somente-esta-máquina a menos que você especificamente controle de outro dispositivo.
remote-password-required = Uma senha é obrigatória para habilitar a API remota.
remote-cancel = Cancelar
remote-save = Salvar

# --- SettingsHotkeys.tsx ---
hotkeys-title = Configurações — Atalhos
hotkeys-record = Iniciar / parar gravação
hotkeys-go-live = Transmitir / Encerrar Transmissão
hotkeys-transition = Transição do Modo Estúdio
hotkeys-save-replay = Salvar Replay (últimos N segundos)
hotkeys-add-marker = Soltar um marcador de capítulo (gravação)
hotkeys-note = Os atalhos são globais — disparam enquanto outros apps estão em foco. Em branco = sem vínculo. As teclas de push-to-talk/mute do mixer ficam no menu ⋯ de cada faixa. No Linux/Wayland, atalhos globais podem ficar indisponíveis (um limite do compositor) — os botões continuam funcionando.
hotkeys-cancel = Cancelar
hotkeys-save = Salvar

# --- WorkspaceDialog.tsx ---
workspace-title = Perfis e Coleções de Cenas
workspace-profiles = Perfis
workspace-profiles-hint = Um perfil são suas configurações — destino de transmissão, saída, atalhos. Alterne por programa ou por plataforma.
workspace-collections = Coleções de cenas
workspace-collections-hint = Uma coleção são suas cenas + fontes. Criar duplica a atual como ponto de partida.
workspace-active = Ativo
workspace-switch-to = Mudar para { $name }
workspace-active-marker = ● ativo
workspace-new-name-placeholder = novo nome…
workspace-new-name-label = Novo nome de { $title }
workspace-create = Criar

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importar do OBS…
workspace-import-obs-hint = Traga uma coleção de cenas do OBS (o scenes.json dela). Sua coleção atual é salva antes.
workspace-import-busy = Importando…
workspace-import-title = "{ $name }" importada
workspace-import-summary = { $scenes } cenas · { $sources } fontes · { $items } itens
workspace-import-dismiss = Fechar
workspace-import-clean = Tudo foi importado corretamente.
workspace-import-geometry-caveat = Tamanhos e posições são ajustados a partir do layout do OBS — revise cada cena e selecione novamente os dispositivos de captura.
workspace-import-notes-title = Importado com observações
workspace-import-skipped-title = Não importado
import-note-needsReselect = Selecione novamente dispositivo/monitor/janela
import-note-gameCaptureAsWindow = Captura de jogo → Captura de janela
import-note-referencesFile = Verifique o caminho do arquivo
import-note-filterDropped = Alguns filtros sem suporte
import-note-geometryApproximated = Posição/tamanho aproximados
import-skip-unsupportedKind = Sem tipo de fonte equivalente
import-skip-group = Grupos ainda não são suportados

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Revincular arquivos ausentes…
doctor-title = Arquivos ausentes
doctor-scanning = Verificando…
doctor-all-good = Todos os arquivos referenciados existem. Nada a revincular.
doctor-intro = { $count } arquivos referenciados não foram encontrados neste computador. Aponte cada um para o novo local — toda cena que o usa é corrigida de uma vez.
doctor-relinked = { $count } referências revinculadas.
doctor-uses = usado { $count }×
doctor-locate = Localizar…
doctor-locate-folder = Procurar na pasta…
doctor-locate-folder-hint = Escolha uma pasta; cada arquivo ausente é encontrado pelo nome e revinculado.
doctor-kind-image = imagem
doctor-kind-media = mídia
doctor-kind-slideshow = apresentação
doctor-kind-font = fonte
doctor-kind-lut = LUT
doctor-kind-mask = máscara
history-relinkFiles = Revincular arquivos

# --- ScriptsDialog.tsx ---
scripts-title = Scripts (Lua)
scripts-empty = Nenhum script ainda — adicione um arquivo .lua. Veja scripts/sample.lua para a API: reaja a eventos de transmissão/cena/gravação e comande os mesmos comandos da API remota.
scripts-enable = Habilitar { $path }
scripts-remove = Remover { $path }
scripts-path-label = Caminho do script
scripts-add = Adicionar
scripts-note = Os scripts rodam em sandbox — sem acesso a arquivos ou ao SO; só podem chamar os mesmos comandos do estúdio que a API remota (trocar cenas, transição, gravar/transmitir/replay, mudos). Um erro de script é registrado e contido. As alterações se aplicam em até um segundo.
scripts-error-not-lua = Aponte para um arquivo .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Docks de navegador
browser-dock-empty = Nenhum dock ainda — adicione um popout de chat, uma página de alertas ou seus botões web do Companion.
browser-dock-open = Abrir
browser-dock-remove = Remover { $name }
browser-dock-name-placeholder = nome (ex.: Chat da Twitch)
browser-dock-name-label = Nome do dock
browser-dock-url-label = URL do dock
browser-dock-note = Um dock abre como sua própria janela que você pode posicionar ao lado do estúdio. A página não recebe acesso ao app — apenas renderiza. Só URLs http(s); os docks abrem somente quando você clica em Abrir.
browser-dock-error-name = Dê um nome ao dock (ex.: Chat da Twitch).
browser-dock-error-url = A URL do dock deve começar com http:// ou https://.

# --- studio-preview-pane ---
studio-preview-label = Prévia do Modo Estúdio
studio-preview-heading = Prévia
studio-preview-hint = clique numa cena para carregá-la aqui
studio-preview-empty = A prévia aparecerá aqui.
studio-preview-mirrors = espelha o programa
studio-preview-transition-select = Transição
studio-preview-duration = Duração da transição (ms)
studio-preview-commit-title = Aplicar Prévia → Programa pela transição (a audiência vê)
studio-preview-transitioning = Fazendo transição…
studio-preview-transition-button = Transição ⇄
studio-preview-luma-placeholder = imagem de wipe em tons de cinza (png/jpg)
studio-preview-luma-label = Imagem de Luma wipe
studio-preview-browse = Procurar…
studio-preview-filter-images = Imagens
studio-preview-filter-video = Vídeo
studio-preview-stinger-placeholder = vídeo de stinger (ProRes 4444 .mov mantém o alfa)
studio-preview-stinger-label = Arquivo de vídeo do stinger
studio-preview-stinger-cut-label = Ponto de corte do stinger (ms)
studio-preview-stinger-cut-title = Quando a troca de cena ocorre sob o stinger (ms dentro da transição)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Corte
transition-kind-fade = Fade
transition-kind-slide-left = Deslizar ←
transition-kind-slide-right = Deslizar →
transition-kind-slide-up = Deslizar ↑
transition-kind-slide-down = Deslizar ↓
transition-kind-swipe-left = Varredura ←
transition-kind-swipe-right = Varredura →
transition-kind-luma-linear = Luma wipe (linear)
transition-kind-luma-radial = Luma wipe (radial)
transition-kind-luma-horizontal = Luma wipe (horizontal)
transition-kind-luma-diamond = Luma wipe (diamante)
transition-kind-luma-clock = Luma wipe (relógio)
transition-kind-image = Wipe de imagem (personalizado)
transition-kind-stinger = Stinger (vídeo)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Personalizado (RTMP/RTMPS)
stream-service-srt = SRT (auto-hospedado)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Sobre
about-tagline = Grave e transmita como um estúdio — sem contas, sem nuvem.
about-version = Versão
about-created-by = Criado por
about-project-started = Projeto iniciado
about-first-stable = Primeira versão estável
about-first-stable-pending = Ainda não — a 1.0.0 está em andamento
about-platform = Plataforma
about-local-first = O Freally Capture roda inteiramente na sua máquina. Sem contas, sem telemetria, sem nuvem — a única coisa que sai do seu computador é a transmissão que você escolheu enviar.
about-website = Site
about-issues = Relatar um problema
about-license = Licença
about-eula = EULA
about-third-party = Avisos de terceiros
about-check-updates = Verificar atualizações…

# --- unified settings modal (TASK-906) ---
settings-title = Configurações
settings-language-section = Idioma
settings-language = Idioma da interface
settings-language-system = Padrão do sistema
settings-language-note = Um idioma que você escolher aqui é lembrado. "Padrão do sistema" segue o seu sistema operacional. Textos não traduzidos recorrem ao inglês.
settings-appearance-section = Aparência
settings-theme = Tema
settings-theme-dark = Escuro
settings-theme-light = Claro
settings-theme-custom = Personalizado
settings-accent = Cor de destaque
settings-general-section = Geral
settings-show-stats-dock = Mostrar o painel de estatísticas
settings-open-about = Sobre…

# --- command palette (TASK-904) ---
palette-title = Paleta de comandos
palette-search = Pesquisar cenas, fontes e ações
palette-placeholder = Pesquisar cenas, fontes, ações…
palette-no-results = Nada corresponde a “{ $query }”
palette-hint = ↑ ↓ para mover · Enter para executar · Esc para fechar
palette-group-scenes = Cena
palette-group-sources = Fonte
palette-group-actions = Ação
palette-transition = Transição Prévia → Programa
palette-save-replay = Salvar replay
palette-add-marker = Soltar um marcador de capítulo
palette-vertical-canvas = Canvas vertical (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Bem-vindo ao Freally Capture
wizard-welcome = Dois passos rápidos: veja o que sua máquina consegue fazer e depois comece uma cena. Leva cerca de trinta segundos, e você pode mudar tudo depois.
wizard-local-first = Nada aqui sai do seu computador. O Freally Capture não tem contas, nem telemetria, nem nuvem.
wizard-start = Começar
wizard-skip = Pular
wizard-hardware-title = O que sua máquina consegue fazer
wizard-probing = Verificando sua placa de vídeo e seu processador…
wizard-encoder = Codificador
wizard-canvas = Canvas
wizard-bitrate = Taxa de bits
wizard-probe-found = Encontrado: { $gpus } · { $cores } núcleos físicos
wizard-no-gpu = sem GPU dedicada
wizard-apply = Usar estas configurações
wizard-keep-current = Manter o que eu tenho
wizard-template-title = Comece com uma cena
wizard-template-screen = Capturar minha tela
wizard-template-screen-note = Adiciona uma Captura de Tela do seu monitor principal. O ponto de partida mais comum.
wizard-template-empty = Começar vazio
wizard-template-empty-note = Uma cena vazia. Adicione as fontes você mesmo com o botão +.
wizard-done = Tudo pronto.
wizard-done-hint = Pressione Ctrl+K a qualquer momento para pesquisar cenas, fontes e ações. As configurações ficam atrás do botão ⚙.
wizard-close = Começar a transmitir

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Sua placa de vídeo consegue codificar o vídeo sozinha, o que deixa o processador livre para o resto do estúdio.
autoconfig-reason-software = Nenhum codificador por hardware utilizável foi encontrado, então o processador fará a codificação. Funciona, apenas custa mais CPU.
autoconfig-reason-quality-hardware = 1080p a 60 quadros por segundo, com uma taxa de bits que toda grande plataforma aceita.
autoconfig-reason-quality-software = 30 quadros por segundo, porque a codificação por software a 60 descarta quadros na maioria dos processadores.
autoconfig-reason-quality-low-cores = Uma taxa de bits mais baixa, porque este processador tem poucos núcleos e a codificação por software vai disputá-los com o compositor.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Gravação iniciada
announce-recording-paused = Gravação pausada
announce-recording-stopped = Gravação encerrada
announce-live-started = Você está ao vivo
announce-live-ended = Transmissão encerrada
announce-reconnecting = Conexão perdida, reconectando
announce-stream-failed = Falha na transmissão
announce-frames-dropped = { $count } quadros descartados

# CAP-M01 — undo/redo edit history
palette-undo = Desfazer
palette-redo = Refazer
palette-edit-history = Histórico de edições…
history-title = Histórico de edições
history-empty = Nada para desfazer ainda.
history-current = Estado atual
history-close = Fechar
history-addScene = Adicionar cena
history-renameScene = Renomear cena
history-removeScene = Remover cena
history-reorderScene = Reordenar cenas
history-addSource = Adicionar fonte
history-removeSource = Remover fonte
history-reorderSource = Reordenar fontes
history-renameSource = Renomear fonte
history-transformSource = Mover fonte
history-toggleVisibility = Alternar visibilidade
history-toggleLock = Alternar bloqueio
history-setBlendMode = Alterar modo de mesclagem
history-editSourceProperties = Editar propriedades
history-applyLayout = Organizar layout
history-moveToSeat = Mover para o lugar
history-groupSources = Agrupar fontes
history-ungroupSources = Desagrupar fontes
history-toggleGroupVisibility = Alternar grupo
history-setSceneAudio = Áudio da cena
history-setVerticalCanvas = Tela vertical
history-addFilter = Adicionar filtro
history-removeFilter = Remover filtro
history-reorderFilter = Reordenar filtros
history-editFilter = Editar filtro
history-toggleFilter = Alternar filtro
history-setVolume = Ajustar volume
history-toggleMute = Alternar mudo
history-setMonitor = Alterar monitoramento
history-setTracks = Alterar faixas
history-setSyncOffset = Ajustar sincronia A/V
history-setAudioHotkeys = Atalhos de áudio

# CAP-M04 — alignment aids
settings-alignment-section = Auxiliares de alinhamento
settings-smart-guides = Guias inteligentes (encaixar ao arrastar)
settings-safe-areas = Sobreposições de área segura
settings-rulers = Réguas
align-group = Alinhar à tela
align-left = Alinhar à esquerda
align-hcenter = Centralizar horizontalmente
align-right = Alinhar à direita
align-top = Alinhar ao topo
align-vcenter = Centralizar verticalmente
align-bottom = Alinhar à base

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Alinhar e distribuir seleção
arrange-left = Alinhar bordas esquerdas
arrange-hcenter = Centralizar horizontalmente
arrange-right = Alinhar bordas direitas
arrange-top = Alinhar bordas superiores
arrange-vcenter = Centralizar verticalmente
arrange-bottom = Alinhar bordas inferiores
distribute-h = Distribuir horizontalmente
distribute-v = Distribuir verticalmente
guides-group = Guias
guides-add-v = Adicionar guia vertical
guides-add-h = Adicionar guia horizontal
guides-clear = Limpar todas as guias
history-arrangeItems = Organizar itens
history-editGuides = Editar guias

# CAP-M05 — edit transform + copy/paste
transform-title = Editar transformação — { $name }
transform-anchor = Âncora
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotação
transform-crop = Recorte
transform-crop-left = Esquerda
transform-crop-top = Topo
transform-crop-right = Direita
transform-crop-bottom = Base
transform-no-size = Tamanho e recorte ficam disponíveis quando a fonte informa suas dimensões.
transform-copy = Copiar transformação
transform-paste = Colar transformação
transform-close = Fechar
filters-copy = Copiar filtros ({ $count })
filters-paste = Colar filtros ({ $count })
palette-edit-transform = Editar transformação…
history-pasteFilters = Colar filtros

# CAP-M26 — keying workbench
workbench-title = Bancada de chroma key — { $name }
workbench-mode-keyed = Com chave
workbench-mode-source = Fonte
workbench-mode-matte = Matte
workbench-mode-split = Dividido
workbench-eyedropper = Conta-gotas
workbench-eyedropper-hint = Clique na fonte para amostrar a cor-chave.
workbench-loupe = Lupa
workbench-split = Divisão
workbench-preview-alt = Prévia da bancada de chroma key
workbench-tune = Ajustar
workbench-close = Fechar

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Clique em uma cena para cortar para ela.
multiview-hint-stage = Clique em uma cena para prepará-la na prévia.
palette-multiview = Monitor multiview

# CAP-M07 — projectors
projector-title = Abrir projetor
projector-source = Fonte
projector-target-program = Programa
projector-target-preview = Prévia
projector-target-scene = Cena…
projector-target-source = Fonte…
projector-target-multiview = Multiview
projector-which-scene = Qual cena
projector-which-source = Qual fonte
projector-none = Nada para mostrar
projector-display = Tela
projector-windowed = Janela flutuante (esta tela)
projector-display-option = Tela { $n } — { $w }×{ $h }
projector-primary = (principal)
projector-open = Abrir
projector-cancel = Cancelar
projector-exit-hint = Pressione Esc para sair
palette-projector = Abrir projetor…

# CAP-M08 — still-frame grab
palette-still = Capturar quadro…
still-saved-toast = Quadro salvo: { $name }
still-failed-toast = Falha ao capturar o quadro: { $error }
hotkeys-still = Capturar quadro

# CAP-M13 — source health dashboard
palette-source-health = Saúde das fontes…
palette-av-sync = Calibração de sincronia A/V…
palette-hotkey-audit = Mapa de atalhos…
health-title = Saúde das Fontes
health-col-source = Fonte
health-col-state = Estado
health-col-resolution = Resolução
health-col-fps = FPS
health-col-last-frame = Último quadro
health-col-dropped = Descartados
health-col-retries = Reinícios
health-col-actions = Ações
health-state-live = Ao vivo
health-state-waiting = Aguardando
health-state-error = Erro
health-state-inactive = Inativa
health-restart = Reiniciar
health-properties = Propriedades
health-empty = Esta coleção ainda não tem fontes.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Sair do Freally Capture?
quit-body = Sair agora fará o seguinte com segurança, nesta ordem:
quit-consequence-stream = Encerrar a transmissão ao vivo e desconectar do serviço.
quit-consequence-recording = Parar a gravação e finalizar seus arquivos.
quit-consequence-replay = Desligar o buffer de replay — as imagens não salvas serão descartadas.
quit-confirm = Sair com segurança
quit-quitting = Encerrando…
quit-cancel = Cancelar

# CAP-M11 — crash-safe recording salvage
salvage-title = Recuperar gravações interrompidas?
salvage-body = A última sessão terminou inesperadamente enquanto estas gravações ainda estavam sendo escritas. O reparo cria uma cópia reproduzível ao lado do original — o arquivo original nunca é alterado.
salvage-repair = Reparar
salvage-repairing = Reparando…
salvage-done = Reparado
salvage-repaired = Reparado → { $name }
salvage-failed = Falha no reparo: { $error }
salvage-dismiss = Agora não

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Falha do codificador — alternado de { $from } para { $to }. A transmissão reconectou e segue no ar.
fallback-toast-recording = Falha do codificador — alternado de { $from } para { $to }. A gravação continua em um novo arquivo.
fallback-note = Codificador reserva: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = O áudio do programa ficou mudo
alarm-clipping = O áudio do programa está saturando
alarm-black = A imagem do programa está preta
alarm-frozen = A imagem do programa não muda há um tempo
alarm-lowDisk = Espaço em disco: restam cerca de { $minutes } min na taxa de bits atual
alarm-dismiss = Dispensar alarme
alarm-cleared = Resolvido: { $alarm }

# CAP-M22 — panic button
palette-panic = Pânico — cortar para a placa de privacidade
panic-banner-title = Pânico
panic-banner-body = O programa mostra a placa de privacidade; todo o áudio está mudo e as capturas paradas. Transmissão e gravação continuam.
panic-restore = Restaurar…
panic-restore-confirm = Restaurar o programa?
panic-restore-yes = Restaurar
panic-restore-cancel = Cancelar
hotkeys-panic = Pânico (placa de privacidade)
hotkeys-timer-toggle = Iniciar/pausar todos os timers
hotkeys-timer-reset = Zerar todos os timers
panic-slate-color = Cor da placa de pânico
panic-slate-image = Imagem da placa de pânico
panic-slate-image-placeholder = Caminho de imagem opcional

# CAP-M24 — redacted diagnostics bundle
diag-title = Pacote de diagnóstico
diag-intro = Exporta um .zip higienizado (instantâneo de configuração, sondagem de codificadores, estatísticas recentes — segredos, caminhos e nomes nunca são incluídos) para anexar manualmente a um issue do GitHub. Nada é enviado.
diag-preview = Ver conteúdo
diag-hide-preview = Ocultar prévia
diag-export = Exportar .zip
diag-exported = Exportado: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Pré-transmissão
preflight-intro = Todo item bloqueante deve estar verde; o resto são lembretes honestos.
preflight-item-targets = Destinos configurados (chave/URL)
preflight-item-encoder = Codificador utilizável disponível
preflight-item-sources = Todas as fontes saudáveis
preflight-item-disk = Espaço em disco para a gravação
preflight-item-mic = Medição do microfone
preflight-item-desktopAudio = Medição do áudio da área de trabalho
preflight-item-replay = Buffer de replay armado
preflight-targets-detail = { $count } habilitado(s)
preflight-sources-detail = { $count } fonte(s) com erro
preflight-disk-detail = ~{ $minutes } min na taxa atual
preflight-fix-stream = Configurações de stream…
preflight-fix-components = Componentes…
preflight-fix-sources = Saúde das fontes…
preflight-fix-replay = Armar
preflight-optional = opcional
preflight-hold = Segurar o Go Live até tudo ficar verde
preflight-cancel = Cancelar
preflight-go-anyway = Transmitir mesmo assim
preflight-go-live = Transmitir


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Plano de fundo
scenes-backdrop-aria = Plano de fundo de { $name }
backdrop-title = Plano de fundo — { $name }
backdrop-hint = Um papel de parede fixado atrás de tudo nesta cena — uma imagem, um GIF animado ou um vídeo em loop. Sua captura fica sempre por cima; role sobre a tela para dar zoom.
backdrop-choose = Escolher imagem ou vídeo…
backdrop-remove = Remover plano de fundo
backdrop-none = Nenhum plano de fundo.
backdrop-position = Posição
backdrop-split-full = Tela inteira
backdrop-split-left = Metade esquerda
backdrop-split-right = Metade direita
backdrop-split-top = Metade superior
backdrop-split-bottom = Metade inferior
backdrop-sync = Iniciar a reprodução quando a gravação começar
backdrop-sync-hint = Fica no primeiro quadro até você gravar; cada take reinicia o vídeo do começo.
backdrop-preview-play = Reproduzir prévia
backdrop-preview-pause = Pausar prévia
backdrop-filter-all = Planos de fundo (imagens e vídeo)
backdrop-filter-images = Imagens
backdrop-filter-media = Vídeo e GIF
sources-backdrop-badge = Papel de parede (fixado embaixo)
sources-backdrop-pinned = O plano de fundo permanece fixado embaixo
filters-name-flip = Inverter
filters-flip-horizontal = Horizontal
filters-flip-vertical = Vertical
history-setSceneBackdrop = Definir plano de fundo
history-setBackdropSplit = Mover plano de fundo
history-setBackdropSync = Sincronizar fundo com a gravação
backdrop-scrub = Posição de reprodução
backdrop-loop = Loop
backdrop-reverse = Reproduzir ao contrário
backdrop-reverse-hint = O modo reverso gera uma cópia invertida uma única vez (vídeos exigem o componente ffmpeg; GIFs invertem na hora) — a primeira troca pode demorar em arquivos longos.
filters-scaling = Escala
filters-scaling-hint = Modos pixel-perfect para conteúdo retrô/pixel; Inteiro também trava o tamanho desenhado em múltiplos exatos (as alças mostram o tamanho lógico).
filters-scaling-auto = Suave
filters-scaling-nearest = Vizinho mais próximo
filters-scaling-integer = Inteiro (× exatos)
filters-scaling-sharp = Bilinear nítido
history-setScaling = Alterar escala
hotkeys-zoom-100 = Zoom: redefinir (100%)
hotkeys-zoom-150 = Zoom: aproximar 150%
hotkeys-zoom-200 = Zoom: aproximar 2×
sources-follow-title = Seguir o cursor durante o zoom (Windows; role sobre a tela para dar zoom)
sources-follow-item = Alternar seguir cursor de { $name }
filters-autocrop = ✂ Cortar barras pretas
filters-autocrop-title = Analisa o próximo quadro em busca de barras e as corta (reversível). Cenas escuras nunca são cortadas.
filters-autocrop-follow = Verificar novamente ao mudar a resolução
history-autoCrop = Corte automático de barras
sources-link-audio = Capturar também o áudio deste app (vinculado: ocultar silencia, remover a janela o remove)
history-addLinkedWindow = Adicionar janela + áudio vinculado
sources-hdr-title = Esta tela é HDR — abra o tone mapping (a tela do programa segue SDR)
sources-hdr-item = Tone mapping HDR de { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Esta tela emite HDR. Sem tone mapping, os realces estouram e a captura fica desbotada no canvas SDR. As mudanças valem no próximo quadro.
sources-hdr-enable-suggested = Ativar sugerido (maxRGB, 200 nits)
sources-hdr-operator = Operador
sources-hdr-op-clip = Clip (desligado)
sources-hdr-op-maxrgb = maxRGB (preserva o matiz)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = Joelho BT.2408 (SDR exato)
sources-hdr-paper-white = Branco de papel
sources-hdr-nits = nits
projector-target-passthrough = Monitor de passagem (baixa latência)
projector-which-device = Dispositivo
projector-passthrough-none = Adicione antes uma tela, janela ou dispositivo de captura.
projector-passthrough-about = Quadros crus do dispositivo — sem cenas, sem filtros, sem compositor. Mostra a latência medida; o áudio continua sendo monitorado pelo canal do mixer.
projector-passthrough-hint = Passagem — Esc fecha
projector-latency = { $ms } ms
projector-latency-measuring = medindo…
automation-title = Automação — regras, macros e variáveis
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Regras
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Ativa
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Quando
automation-then-run = então executar
automation-no-macro = (no macro)
automation-macros = Macros
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Executar
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Variáveis do estúdio
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
rundown-title = Espelho do programa
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Iniciar
rundown-next = Próximo ▸
rundown-stop = Parar
rundown-idle = Parado
rundown-next-up = A seguir: { $name }
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
automation-layer = Camada
automation-layer-hint = Só dispara com esta camada ativa (vazio = todas). Camadas são fixas: a tecla de camada troca e permanece (a API global do SO não tem camada por pressionar e segurar).
automation-chord-hint = Uma tecla simples (Ctrl+Shift+M) ou um acorde de duas etapas (Ctrl+K, 3). A segunda tecla só é reservada enquanto o acorde está pendente.
panel-title = Painel LAN e tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Servir o painel
panel-port = Porta
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Senha
panel-show = Mostrar
panel-hide = Ocultar
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Salvar
osc-title = Superfície de controle OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Escutar OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = Câmeras PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Câmera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Endereço
ptz-port = Porta
ptz-speed = Velocidade
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
ptz-recall = Chamar
ptz-store = Salvar
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = Superfície de controle MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Entrada
midi-output = Saída (feedback)
midi-none = (none)
midi-learn = Aprender
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Ação
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
panel-lan-warning = ⚠ O tráfego LAN não é criptografado — a senha viaja na URL por HTTP. Use apenas em uma rede confiável.
osc-lan-warning = ⚠ O OSC não tem senha — qualquer dispositivo na rede pode enviar estes comandos. Use o modo LAN apenas em rede confiável.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stats
sources-add-system-stats = Estatísticas de desempenho (HUD)
sources-stats-title = Adicionar um HUD de desempenho
sources-stats-note = Mostra no programa os números reais medidos do estúdio para seus espectadores — fps, CPU, memória, tempo de render, quadros perdidos e bitrate ao vivo. Quais linhas aparecem, o tamanho e a cor ficam nas Propriedades da fonte. O uso de GPU não é mostrado porque não é medido.
sources-stats-add = Adicionar HUD de estatísticas
properties-stats-show-fps = Mostrar FPS
properties-stats-show-cpu = Mostrar CPU
properties-stats-show-memory = Mostrar memória
properties-stats-show-render = Mostrar tempo de render
properties-stats-show-dropped = Mostrar quadros perdidos
properties-stats-show-bitrate = Mostrar bitrate
properties-stats-size = Tamanho (px)
properties-stats-note = O HUD desenha rótulos compactos universais (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) direto no programa; sem transmissão, a linha de bitrate mostra “—”.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Visualizador
sources-add-visualizer = Visualizador de áudio
sources-visualizer-title = Adicionar um visualizador de áudio
sources-visualizer-style-label = Estilo
sources-visualizer-style-bars = Barras de espectro
sources-visualizer-style-scope = Osciloscópio
sources-visualizer-style-vu = Medidores VU
sources-visualizer-target-label = Escuta
sources-visualizer-target-master = Mix master
sources-visualizer-target-track = Faixa { $n }
sources-visualizer-note = Desenha o sinal que realmente é mixado (pós-fader) — uma fonte mutada fica plana, exatamente como soa. Tamanho, cor, número de barras e taxa de queda ficam nas Propriedades da fonte.
sources-visualizer-add = Adicionar visualizador
properties-vis-bands = Barras
properties-vis-decay = Taxa de queda (dB/s)
properties-vis-peak-hold = Marcadores de pico
properties-vis-missing-source = (fonte ausente)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splits
sources-add-split-timer = Timer de splits (speedrun)
sources-splits-title = Adicionar um timer de splits
sources-splits-file-label = Arquivo .lss do LiveSplit
sources-splits-comparison-label = Comparar com
sources-splits-comparison-pb = Recorde pessoal
sources-splits-comparison-best = Melhores segmentos
sources-splits-comparison-average = Média
sources-splits-note = O arquivo é importado somente leitura — nada é gravado de volta. Vincule as teclas globais Split / Undo / Skip / Reset em Configurações → Atalhos. Auto-splitters por memória de processo não são suportados de propósito.
sources-splits-add = Adicionar timer de splits
properties-splits-size = Tamanho (px)
properties-splits-ahead = À frente
properties-splits-behind = Atrás
properties-splits-gold = Ouro
properties-splits-split = Split
properties-splits-undo = Desfazer
properties-splits-skip = Pular
properties-splits-reset = Zerar
properties-splits-note = Os botões controlam o timer ao vivo (os atalhos globais fazem o mesmo de qualquer app). A corrida nunca é gravada no arquivo .lss.
hotkeys-split-split = Timer de splits: iniciar / split
hotkeys-split-undo = Timer de splits: desfazer split
hotkeys-split-skip = Timer de splits: pular segmento
hotkeys-split-reset = Timer de splits: zerar
hotkey-audit-action-split-split = Split (timer de splits)
hotkey-audit-action-split-undo = Desfazer split
hotkey-audit-action-split-skip = Pular segmento
hotkey-audit-action-split-reset = Zerar timer de splits
hotkey-audit-feature-split-timer = Timer de splits

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlist
sources-add-playlist = Playlist de mídia (sem cortes)
sources-playlist-title = Adicionar uma playlist de mídia
sources-playlist-files-label = Arquivos (um por linha, tocados de cima para baixo)
sources-playlist-browse = Procurar…
sources-playlist-loop = Repetir
sources-playlist-shuffle = Aleatório (um sorteio por início; repetindo, mantém essa ordem)
sources-playlist-hold-last = Manter o último quadro no fim
sources-playlist-note = Toca a lista aparada inteira sem cortes pelo componente ffmpeg rotulado (só formatos wire — .frec e imagens vão por Mídia/Apresentação). Itens são todos vídeo ou todos áudio, nunca misturados. Aparas, cues e a variável «now playing» ficam nas Propriedades.
sources-playlist-add = Adicionar playlist
properties-playlist-items = Itens (de cima para baixo)
properties-playlist-up = Subir
properties-playlist-down = Descer
properties-playlist-remove = Remover item
properties-playlist-in = De (s)
properties-playlist-out = Até (s)
properties-playlist-cues = Cues (s, separados por vírgula)
properties-playlist-add-item = + Adicionar item
properties-playlist-loop = Repetir
properties-playlist-shuffle = Aleatório
properties-playlist-hold-last = Manter último quadro
properties-playlist-hw = Decodificação por hardware
properties-playlist-variable = Variável «now playing» (vazio = desligado)
properties-playlist-previous = ⏮ Anterior
properties-playlist-next = ⏭ Próximo
properties-playlist-note = Os botões de cue e Próximo/Anterior controlam a playlist AO VIVO; mudanças de itens valem no Aplicar (a playlist reinicia). Coloque {"{{"}yourVariable{"}}"} numa fonte de Texto para mostrar o item em reprodução.
hotkeys-playlist-next = Playlist: próximo item
hotkeys-playlist-previous = Playlist: item anterior
hotkey-audit-action-playlist-next = Playlist próximo
hotkey-audit-action-playlist-previous = Playlist anterior
hotkey-audit-feature-playlist = Playlist

# Instant replay source (CAP-N10)
sources-badge-replay = Replay
sources-add-replay = Replay instantâneo
sources-replay-title = Adicionar um replay instantâneo
sources-replay-seconds-label = Duração do roll (segundos)
sources-replay-speed-label = Velocidade
sources-replay-speed-full = 100% (com áudio)
sources-replay-speed-half = Câmera lenta 50% (mudo)
sources-replay-speed-quarter = Câmera lenta 25% (mudo)
sources-replay-note = Fica transparente até você rolar. Arme o buffer de replay (Controles) e vincule a tecla Roll — o roll recorta os últimos momentos do buffer, toca no programa e volta a ficar transparente.
sources-replay-add = Adicionar replay
properties-replay-roll = ⏵ Rolar replay
properties-replay-note = Roll recorta o buffer ARMADO num clipe e o toca na velocidade escolhida — retemporizado, nunca interpolado. A câmera lenta é muda de propósito. Scrub e pausa funcionam durante a reprodução; no fim a fonte volta à transparência.
hotkeys-replay-roll = Replay instantâneo: rolar
hotkey-audit-action-replay-roll = Rolar replay instantâneo

# Input overlay source (CAP-N13)
sources-badge-input = Entrada
sources-add-input-overlay = Overlay de entrada (teclas/controle)
sources-input-title = Adicionar um overlay de entrada
sources-input-layout-label = Layout
sources-input-layout-wasd = WASD + mouse
sources-input-layout-keyboard = Teclado compacto + mouse
sources-input-layout-gamepad = Controle (dois analógicos)
sources-input-layout-fightstick = Controle arcade
sources-input-color-label = Teclas
sources-input-accent-label = Pressionado
sources-input-privacy-note = Privacidade: a entrada só é lida enquanto esta fonte está ao vivo em uma cena, e apenas as teclas fixas do layout são consultadas — uma leitura pontual de "está pressionada agora?", nunca um hook. Nada é registrado, armazenado ou enviado a lugar nenhum; o texto digitado nunca é capturado.
sources-input-os-note = O estado do teclado e do mouse hoje só é lido no Windows — outros sistemas desenham as teclas soltas (dito honestamente, nunca simulado). Controles funcionam em qualquer sistema pela biblioteca gilrs; o primeiro controle conectado é desenhado e, sem nenhum, o layout fica solto.
sources-input-add = Adicionar overlay de entrada

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Efeitos do cursor
filters-cursorfx-hint = No Windows (que desenha o próprio cursor), eles são pintados diretamente na captura e aparecem em gravações e transmissões. macOS e Linux compõem o cursor no sistema, então esses efeitos são exclusivos do Windows. As mudanças valem na hora.
filters-cursorfx-halo = Halo do cursor
filters-cursorfx-halo-color = Cor
filters-cursorfx-halo-radius = Raio (px)
filters-cursorfx-ripples = Ondas de clique
filters-cursorfx-left-color = Clique esquerdo
filters-cursorfx-right-color = Clique direito
filters-cursorfx-keystrokes = Teclas fantasma
filters-cursorfx-keystrokes-hint = Mostra um conjunto fixo de teclas (letras, dígitos, modificadores, setas) perto do cursor enquanto estiverem pressionadas. As teclas só são lidas com isto ativo, desenhadas direto no quadro e nunca armazenadas ou registradas.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Título
sources-add-title = Título / Placar
sources-title-title = Adicionar um título
sources-title-template-label = Começar de
sources-title-template-lower-third = Tarja inferior (barra + nome + subtítulo)
sources-title-template-scoreboard = Placar (placa + 4 células)
sources-title-template-blank = Tela em branco
sources-title-width-label = Largura da tela
sources-title-height-label = Altura da tela
sources-title-template-name = Nome
sources-title-template-subtitle = Título
sources-title-template-home = CASA
sources-title-template-away = VISITANTE
sources-title-note = Títulos em camadas (texto / imagem / caixa) com animação de entrada/saída, compostos localmente — sem fonte de navegador. Camadas, vínculos a arquivos e {"{{"}variáveis{"}}"} e os controles ao vivo ficam nas Propriedades da fonte.
sources-title-add = Adicionar título
properties-title-layers = Camadas (desenhadas em ordem — linhas posteriores ficam por cima)
properties-title-kind-text = Texto
properties-title-kind-image = Imagem
properties-title-kind-rect = Caixa
properties-title-x = X
properties-title-y = Y
properties-title-outline = Contorno (px)
properties-title-outline-color = Contorno
properties-title-shadow = Sombra
properties-title-animation = Animação de entrada/saída
properties-title-anim-none = Nenhuma (corte)
properties-title-anim-fade = Esmaecer
properties-title-anim-slide-left = Deslizar à esquerda
properties-title-anim-slide-up = Deslizar para cima
properties-title-anim-wipe = Varredura
properties-title-duration = Duração (ms)
properties-title-fire-in = ▶ Disparar entrada
properties-title-fire-out = ◼ Disparar saída
properties-title-set-live = Definir ao vivo
properties-title-set-live-note = Empurra este texto para o título AO VIVO agora — sem Aplicar, sem reinício
properties-title-up = Subir camada
properties-title-down = Descer camada
properties-title-remove = Remover camada
properties-title-add-text = + Texto
properties-title-add-image = + Imagem
properties-title-add-rect = + Caixa
properties-title-note = Disparar entrada/saída e "Definir ao vivo" controlam o título EM EXECUÇÃO; edições de camadas valem no Aplicar (o título reinicia e entra de novo). Células de texto podem vincular-se a um arquivo observado (célula CSV / valor JSON / arquivo inteiro) e interpolar {"{{"}variáveis{"}}"} — "Definir ao vivo" vence os dois.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = Ingest LAN (escuta SRT/RTMP)
sources-lan-title = Adicionar uma escuta de ingest LAN
sources-lan-protocol-label = Protocolo
sources-lan-protocol-srt = SRT (criptografável — recomendado)
sources-lan-protocol-rtmp = RTMP (sem autenticação)
sources-lan-port-label = Porta (1024–65535)
sources-lan-passphrase-label = Senha (vazia = aberto)
sources-lan-passphrase-hint = Senhas SRT têm de 10 a 79 caracteres; o remetente precisa usar a mesma.
sources-lan-open-warning = Sem senha: qualquer pessoa nesta rede pode alimentar esta fonte, sem criptografia. Defina uma, a menos que a rede seja só sua.
sources-lan-rtmp-warning = RTMP não tem autenticação — qualquer pessoa nesta rede pode enviar para esta porta. Prefira SRT com senha.
sources-lan-url-label = Aponte o app do remetente para
sources-lan-qr-aria = Código QR da URL de ingest
sources-lan-note = Somente LAN: escuta no endereço local desta máquina, apenas enquanto a fonte existir, e nunca toca a internet — nada sai da máquina até que um remetente da sua rede envie primeiro. A decodificação usa o componente ffmpeg claramente rotulado. A tela mostra esta URL até um remetente conectar.
sources-lan-add = Começar a escutar
properties-lan-note = Aplicar uma mudança de protocolo, porta ou senha reinicia a escuta — o remetente precisa reconectar. O stream é ajustado a uma tela de 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Link
sources-add-freally-link = Freally Link (outra instância)
sources-link-title = Adicionar um Freally Link
sources-link-about = Recebe o programa de outra instância do Freally Capture — vídeo e áudio master — pela sua própria rede. Ative primeiro “Saída Freally Link” na instância transmissora. A v1 transmite motion-JPEG por TCP: ótimo em LAN cabeada ou Wi-Fi bom, honesto sobre a banda em conexões fracas.
sources-link-scan = Varrer a LAN
sources-link-scanning = Varrendo…
sources-link-none = Nenhuma saída Freally Link encontrada. Ative “Saída Freally Link” na outra instância (Controles → Painel LAN) ou digite o endereço abaixo.
sources-link-host = Endereço
sources-link-port = Porta
sources-link-key = Chave de pareamento
sources-link-key-hint = A chave das configurações de "Saída Freally Link" do remetente — sem ela, o remetente não envia um único quadro.
sources-link-add = Adicionar link
properties-link-note = Sem conexão, a fonte mostra uma tela de “conectando” e tenta de novo sozinha com espera crescente — nunca congela em um quadro antigo. Um receptor por transmissor; um transmissor ocupado é retentado com educação.
link-title = Saída Freally Link
link-about = Compartilhe o programa desta instância — vídeo e áudio master — com UMA outra instância do Freally Capture na sua própria rede; lá ele aparece como fonte “Freally Link” (streaming com dois PCs, monitores auxiliares). Desligado por padrão; nada anuncia nem escuta até você ativar. A v1 transmite motion-JPEG + áudio sem compressão por TCP — feito para LAN cabeada ou Wi-Fi bom, nunca para a internet.
link-enable = Compartilhar o programa na minha rede
link-name = Nome da instância
link-key = Chave de pareamento
link-key-hint = Pelo menos 8 caracteres — os receptores precisam digitar esta chave antes de um único quadro ser enviado.
link-lan-warning = ⚠ Os receptores precisam apresentar a chave de pareamento antes de receber qualquer coisa, mas o fluxo em si não é criptografado na v1 — use apenas em uma rede confiável.
link-serving = Receptores encontram esta instância com “Varrer a LAN” ou a adicionam manualmente em:
link-off-hint = Ative o compartilhamento para abrir a porta e anunciar esta instância às varreduras da LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Menu do aplicativo
menu-file = Arquivo
menu-edit = Editar
menu-view = Exibir
menu-docks = Docks
menu-profile = Perfil
menu-collection = Coleção de cenas
menu-tools = Ferramentas
menu-help = Ajuda
menu-rename = Renomear
menu-remove = Remover
menu-import = Importar
menu-export = Exportar
menu-file-show-recordings = Mostrar gravações
menu-file-remux = Remuxar para MP4…
menu-file-settings = Configurações…
menu-file-show-settings-folder = Mostrar pasta de configurações
menu-file-exit = Sair
menu-edit-undo = Desfazer
menu-edit-redo = Refazer
menu-edit-history = Histórico de edições…
menu-edit-copy-transform = Copiar transformação
menu-edit-paste-transform = Colar transformação
menu-edit-copy-filters = Copiar filtros
menu-edit-paste-filters = Colar filtros
menu-edit-transform = Transformação…
menu-edit-lock-preview = Bloquear a pré-visualização
menu-view-fullscreen = Interface em tela cheia
menu-stats-dock = Painel de estatísticas
menu-view-multiview = Monitor multiview…
menu-view-projectors = Projetores…
menu-view-source-health = Saúde das fontes…
menu-view-still = Capturar quadro
menu-docks-browser = Docks de navegador…
menu-docks-lock = Bloquear docks
menu-docks-reset = Redefinir layout dos docks
menu-profile-manage = Gerenciar perfis…
menu-collection-manage = Gerenciar coleções de cenas…
menu-collection-import-obs = Importar do OBS…
menu-collection-missing = Verificar arquivos ausentes…
menu-tools-wizard = Executar assistente de configuração
menu-tools-wizard-title = O assistente de configuração roda na primeira inicialização; ainda não é possível executá-lo novamente.
menu-tools-automation = Regras de automação e macros…
menu-tools-rundown = Mostrar espelho do programa…
menu-tools-hotkeys = Mapa de atalhos…
menu-tools-av-sync = Calibração de sincronia A/V…
menu-tools-scripts = Scripts Lua…
menu-tools-components = Componentes…
menu-tools-midi = Controle MIDI…
menu-tools-ptz = Câmeras PTZ…
menu-tools-remote = API de controle remoto…
menu-tools-panel = Painel LAN e tally…
menu-help-portal = Portal de ajuda
menu-help-website = Visitar o site
menu-help-discord = Entrar no servidor do Discord
menu-help-bug = Relatar um bug…
menu-help-updates = Verificar atualizações…
menu-help-whats-new = Novidades
menu-help-about = Sobre…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Categorias de configurações
settings-cat-general = Geral
settings-cat-appearance = Aparência
settings-cat-streaming = Transmissão
settings-cat-output = Saída
settings-cat-replay = Replay
settings-cat-hotkeys = Atalhos
settings-cat-network = Rede
settings-cat-accessibility = Acessibilidade
settings-cat-about = Sobre
settings-ok = OK
settings-cancel = Cancelar
settings-apply = Aplicar
settings-save = Salvar
settings-loading = Carregando as configurações…
settings-hotkeys-filter = Filtrar atalhos
settings-hotkeys-filter-placeholder = Digite para filtrar ações ou teclas…
settings-hotkeys-no-match = Nenhum atalho corresponde a “{ $query }”.
settings-hotkey-none = Nenhum
settings-hotkey-group-ctrl = Ctrl + tecla
settings-hotkey-group-ctrl-shift = Ctrl + Shift + tecla
settings-hotkey-group-ctrl-alt = Ctrl + Alt + tecla
settings-hotkey-group-function = Teclas de função
settings-hotkey-group-numpad = Teclado numérico
settings-panic-section = Placa de pânico
settings-meter-section = Medidores de nível do mixer
settings-meter-note = As cores que os medidores de nível do Mixer de Áudio percorrem, do silêncio ao clipping. A predefinição segura para daltônicos usa uma rampa de azul → laranja que continua legível com deficiência vermelho-verde.
settings-meter-preset = Cores do medidor
settings-meter-preset-default = Verde / amarelo / vermelho
settings-meter-preset-colorblind = Seguro para daltônicos (azul / laranja)
settings-meter-preset-custom = Personalizado
settings-meter-low = Normal
settings-meter-mid = Alto
settings-meter-high = Clipping
settings-meter-preview = Pré-visualização
