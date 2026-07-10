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
sources-add-nested-scene = Cena Aninhada
sources-add-slideshow = Apresentação de Imagens
sources-add-chat-overlay = Sobreposição de Chat ao Vivo
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
controls-files-title = Gravações concluídas + a ação de remuxar para mp4
controls-files = ▤ Arquivos…
controls-output-title = Formato de gravação, codificador, pasta, trilhas e divisão
controls-output = ⚙ Saída…
controls-stream-title = Destino do Transmitir: serviço, chave de transmissão, codificador, taxa de bits
controls-stream = ⦿ Transmissão…
controls-codecs-title = O componente de codecs padrão do ffmpeg sob demanda (claramente identificado, nunca embutido)
controls-codecs = ⬡ Codecs…
controls-replay-title = Duração do buffer de replay + predefinições de qualidade
controls-replay = ⟲ Replay…
controls-keys-title = Teclas de atalho globais: gravar, Transmitir, transição, salvar replay
controls-keys = ⌨ Teclas…
controls-scripts-title = Scripts Lua em sandbox: reajam a eventos de transmissão/cena/gravação, comandem o estúdio
controls-scripts = ⚡ Scripts…
controls-docks-title = Docks de navegador: abra um popout de chat, página de alertas ou botões do Companion como janela ao lado do estúdio
controls-docks = ⧉ Docks…
controls-remote-title = API remota WebSocket para controladores Stream Deck / Companion (desligada por padrão)
controls-remote = ⌁ Remoto…
controls-profiles-title = Perfis (configurações) + coleções de cenas — snapshots alternáveis
controls-profiles = ▣ Perfis…
controls-bug-title = Relatar um bug — anônimo, opcional (nada é enviado automaticamente)
controls-bug = 🐞 Relatar um bug…
controls-updates-title = Verificar atualizações — assinadas, verificadas, nada baixa sem um clique
controls-updates = ⭳ Verificar atualizações…
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
hotkeys-record-placeholder = ex.: Ctrl+Shift+R
hotkeys-go-live = Transmitir / Encerrar Transmissão
hotkeys-go-live-placeholder = ex.: Ctrl+Shift+L
hotkeys-transition = Transição do Modo Estúdio
hotkeys-transition-placeholder = ex.: Ctrl+Shift+T ou F13
hotkeys-save-replay = Salvar Replay (últimos N segundos)
hotkeys-save-replay-placeholder = ex.: Ctrl+Shift+S
hotkeys-add-marker = Soltar um marcador de capítulo (gravação)
hotkeys-add-marker-placeholder = ex.: Ctrl+Shift+K
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
settings-more-section = Mais configurações
settings-open-output = Gravação…
settings-open-stream = Transmissão…
settings-open-replay = Replay…
settings-open-hotkeys = Atalhos…
settings-open-remote = API remota…
settings-open-about = Sobre…
controls-settings = ⚙ Configurações…
controls-settings-title = Idioma, aparência e as preferências gerais do app
