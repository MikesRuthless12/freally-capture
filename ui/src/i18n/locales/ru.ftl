# Freally Capture — ru
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Студийный режим
toggle-on = вкл.
toggle-off = выкл.
stats = Статистика
core-ok = ядро в норме
hide-stats-dock = Скрыть панель статистики
show-stats-dock = Показать панель статистики


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Не удалось сохранить настройки — изменение не переживёт перезапуск.
studio-mode-leave = Выйти из студийного режима
studio-mode-enter-title = Студийный режим — редактируйте сцену в превью и выводите её в программу с переходом
vertical-canvas-title = Второй (вертикальный 9:16) холст вывода — записывается и транслируется независимо
app-version = v{ $version }
core-error = ОШИБКА ядра
core-unreachable = ядро недоступно (режим браузера)
connecting-to-core = подключение к ядру…
filters-source-fallback = Источник

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Превью программы
preview-program-output = Вывод программы
preview-canvas-editor = Редактор холста
preview-px-to-edge-label = Пиксели до краёв кадра
preview-px-to-edge = пикс. до края Л { $left } · В { $top } · П { $right } · Н { $bottom }
preview-program-heading = Программа
preview-no-gpu = Не найден подходящий GPU-адаптер — компоновщик не может работать на этой машине.
preview-starting-compositor = Запуск компоновщика…
preview-empty-scene = Эта сцена пуста — добавьте источник в разделе «Источники», затем перетаскивайте, масштабируйте и вращайте его прямо здесь на холсте.
preview-fps = { $fps } fps
preview-dropped = { $dropped } пропущено

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Получена ссылка-приглашение
remote-join-with-webcam = Подключиться с веб-камерой
remote-dismiss = Закрыть
remote-hosting-guest = Приём удалённого гостя
remote-you-are-guest = Вы — удалённый гость
remote-share-view-title = Поделитесь экраном с приложением гостя (он видит ваш вид в реальном времени)
remote-stop-sharing-view = Остановить показ вида
remote-share-my-view = Показать мой вид
remote-allow-center-title = Разрешить гостю переключать, чей вид в центре (вы сохраняете контроль и можете вернуть его в любой момент)
remote-guest-switching = Переключение гостем:
remote-stop-screen = Остановить экран
remote-share-screen = Показать экран
remote-share-screen-title-guest = Поделитесь своим экраном с ведущим (он становится источником, который можно вывести в центр)
remote-center-request-label = Запрос на вывод в центр
remote-center = В центр
remote-center-cam-title = Попросить ведущего вывести вашу камеру в центр
remote-center-my-cam = Моя камера
remote-center-screen-title = Попросить ведущего вывести ваш общий экран в центр
remote-center-my-screen = Мой экран
remote-center-host-title = Вернуть центр виду ведущего
remote-center-host-view = Вид ведущего
remote-end-session = Завершить сеанс
remote-leave = Покинуть
remote-host-view-heading = Вид ведущего
remote-host-shared-view-label = Общий вид ведущего
remote-guest-position-label = Позиция гостя
remote-guest-label = Гость
remote-put-guest = Поместить гостя { $position }
remote-remove-title = Удалить гостя — он может вернуться по той же ссылке
remote-remove = Удалить
remote-ban-title = Забанить гостя — блокирует его и аннулирует ссылку-приглашение
remote-ban = Забанить
remote-guest-self-muted = гость сам себя заглушил
remote-unmute-guest = Включить звук гостя
remote-mute-guest = Заглушить гостя
remote-muted-by-host = Заглушён ведущим
remote-unmute-mic = Включить микрофон
remote-mute-mic = Заглушить микрофон
remote-waiting-for-host = ожидание ведущего


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = источник
sources-fallback-video = видео
sources-fallback-error = ошибка
sources-kind-unknown = ?
sources-missing-source = (источник отсутствует)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Дисплей
sources-badge-window = Окно
sources-badge-portal = Портал
sources-badge-camera = Камера
sources-badge-image = Изображение
sources-badge-media = Медиа
sources-badge-guest = Гость
sources-badge-color = Цвет
sources-badge-text = Текст
sources-badge-scene = Сцена
sources-badge-slides = Слайды
sources-badge-chat = Чат
sources-badge-audio-in = Аудиовход
sources-badge-audio-out = Аудиовыход
sources-badge-app-audio = Звук прил.
sources-badge-test-bars = Полосы
sources-badge-test-grid = Сетка
sources-badge-test-sweep = Развёртка
sources-badge-test-tone = Тон
sources-badge-test-sync = Синхро
sources-badge-timer = Таймер

# Add-source menu items
sources-add-display = Захват дисплея
sources-add-window = Захват окна
sources-add-game = Захват игры (сначала прочтите)
sources-add-webcam = Устройство захвата видео
sources-add-image = Изображение
sources-add-media = Медиа (файл видео/изображения)
sources-add-remote-guest = Удалённый гость (P2P-прототип)
sources-add-color = Цвет
sources-add-text = Текст
sources-add-timer = Таймер / Часы
sources-add-nested-scene = Вложенная сцена
sources-add-slideshow = Слайд-шоу изображений
sources-add-chat-overlay = Оверлей живого чата
sources-add-test-signal = Тестовый сигнал
sources-add-audio-input = Захват аудиовхода
sources-add-audio-output = Захват аудиовыхода
sources-add-app-audio = Звук приложения (Windows)
sources-add-existing = Существующий источник…

# Panel header + toolbar buttons
sources-panel-title = Источники
sources-group-title = Сгруппировать источники — выберите два или более элемента, затем «Создать группу»; сгруппированные элементы перемещаются и показываются/скрываются вместе
sources-group-aria = Сгруппировать источники
sources-arrange = Расположить: экран + углы
sources-add-source = Добавить источник
sources-browser-source-note = Браузерный источник поставляется как отдельный компонент по запросу (движок Chromium ~180 МБ — никогда не включается в сборку). Сейчас: захватите настоящее окно браузера через «Захват окна» + хромакей/ключ по цвету, либо откройте чат/уведомления как док (Управление → Доки).

# Empty state
sources-empty = В этой сцене нет источников — добавьте «Захват дисплея», окно, веб-камеру, изображение, цвет или текст через «+». Перетаскивайте, масштабируйте и вращайте их на холсте; кнопки справа меняют порядок в стеке.

# Per-row controls
sources-already-in-group = Уже в { $name }
sources-pick-for-new-group = Выбрать для новой группы
sources-pick-item-for-group = Выбрать { $name } для новой группы
sources-hide = Скрыть
sources-show = Показать
sources-hide-item = Скрыть { $name }
sources-show-item = Показать { $name }
sources-unfocus-title = Снять фокус — восстановить компоновку
sources-focus-title = Фокус — заполнить холст (Выделить говорящего)
sources-unfocus-item = Снять фокус с { $name }
sources-focus-item = Сфокусировать { $name }
sources-center-title = В центр — сделать это общим центральным видом (камеры уходят на панель)
sources-center-item = В центр { $name }
sources-rename-item = Переименовать { $name }
sources-in-group = В группе { $name }

# Row status + retry
sources-retry-error = Повторить — { $message }
sources-retry-item = Повторить { $name }
sources-status-error = статус: ошибка
sources-open-privacy-title = Открыть настройки конфиденциальности macOS для этого разрешения
sources-open-privacy-item = Открыть настройки конфиденциальности для { $name }
sources-privacy-settings-button = настройки
sources-status-starting = запуск…
sources-status-live = активно
sources-status-aria = статус: { $state }

# Media row pause/resume
sources-media-resume-title = Возобновить видео (в прямом эфире)
sources-media-pause-title = Приостановить видео — удержать кадр и заглушить звук, в прямом эфире
sources-media-resume-item = Возобновить { $name }
sources-media-pause-item = Приостановить { $name }

# Hover controls
sources-unlock = Разблокировать
sources-lock = Заблокировать
sources-unlock-item = Разблокировать { $name }
sources-lock-item = Заблокировать { $name }
sources-raise-title = Поднять в стеке
sources-raise-item = Поднять { $name }
sources-lower-title = Опустить в стеке
sources-lower-item = Опустить { $name }
sources-filters-title = Фильтры и наложение
sources-filters-item = Фильтры для { $name }
sources-properties-title = Свойства
sources-properties-item = Свойства { $name }
sources-remove-title = Удалить из этой сцены
sources-remove-item = Удалить { $name }

# Grouping footer
sources-create-group = Создать группу ({ $count })
sources-cancel = Отмена

# Groups list
sources-groups-aria = Группы источников
sources-hide-group = Скрыть группу
sources-show-group = Показать группу
sources-item-count = · { $count } элем.
sources-ungroup-title = Разгруппировать — элементы остаются на месте
sources-ungroup-item = Разгруппировать { $name }

# Live Chat Overlay picker
sources-chat-title = Добавить оверлей живого чата
sources-chat-youtube-label = YouTube — URL канала, watch или live_chat (без ключа, без входа)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  или URL watch?v=
sources-chat-twitch-label = Twitch — имя канала (читается анонимно, без аккаунта)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — слаг канала (публичный эндпоинт, по возможности)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Сообщения появляются с бегущей меткой времени ч:мм:сс AM/PM на прозрачном фоне (по умолчанию сверху справа; перетащите куда угодно). Наплыв сообщений лишь вытесняет старые строки — он не может застопорить трансляцию или запись. Чат Facebook требует вашего собственного токена Graph и пока не реализован — он никогда не требуется и не блокирует платформы выше.
sources-chat-add = Добавить оверлей чата
sources-chat-default-name = Живой чат

# Image Slideshow picker
sources-slideshow-title = Добавить слайд-шоу изображений
sources-slideshow-empty = Пока нет изображений — «Обзор» добавляет их по порядку.
sources-slideshow-remove-slide = Удалить слайд { $number }
sources-slideshow-browse = Обзор изображений…
sources-slideshow-per-slide-label = На слайд (мс)
sources-slideshow-crossfade-label = Перекрёстное затухание (мс, 0 = резкий переход)
sources-slideshow-loop-label = Зациклить (выкл. = удерживать последний слайд)
sources-slideshow-shuffle-label = Перемешивать каждый цикл
sources-slideshow-note = Перекрёстное затухание смешивает изображения одинакового размера; изображения разного размера меняются резко на границе (без скрытого масштабирования).
sources-slideshow-add = Добавить слайд-шоу ({ $count })

# Nested Scene picker
sources-nested-title = Добавить вложенную сцену
sources-nested-empty = Нет другой сцены для вложения — сначала добавьте вторую сцену.
sources-nested-scene-name = Сцена: { $name }
sources-nested-note = Вложенная сцена рендерится в реальном времени в размере холста программы и следует своим правкам; трансформации, фильтры и наложение применяются к ней как к любому источнику. Её аудиоисточники входят в микс, пока сцена, показывающая её, находится в программе.

# Display / Window capture picker
sources-capture-display-title = Добавить захват дисплея
sources-capture-window-title = Добавить захват окна
sources-capture-looking = Поиск источников…
sources-capture-none-displays = Здесь нечего захватывать — дисплеи не найдены.
sources-capture-none-windows = Здесь нечего захватывать — окна не найдены.
sources-capture-portal-note = В Wayland системный диалог выбирает экран или окно — приложения не могут захватывать глобально, так что это честный (и единственный) путь.
sources-capture-window-note = Превью обновляется в реальном времени. Свёрнутое окно показывает свой последний кадр (или ничего), пока вы его не развернёте.
sources-thumb-no-preview = нет превью
sources-thumb-loading = загрузка…

# Video Capture Device picker
sources-webcam-title = Добавить устройство захвата видео
sources-webcam-looking = Поиск камер…
sources-webcam-none = Камеры или платы захвата не найдены.
sources-webcam-format-label = Формат
sources-webcam-format-auto-loading = Авто (загрузка форматов…)
sources-webcam-format-auto = Авто (максимальное разрешение)
sources-webcam-card-presets-label = Пресеты платы:
sources-webcam-preset-title = Выберите режим { $label }, который заявляет эта плата
sources-webcam-add = Добавить камеру

# Audio Input / Output capture picker
sources-audio-output-title = Добавить захват аудиовыхода
sources-audio-input-title = Добавить захват аудиовхода
sources-audio-default-output = Устройство вывода по умолчанию (то, что вы слышите)
sources-audio-default-input = Устройство ввода по умолчанию
sources-audio-looking = Поиск аудиоустройств…
sources-audio-none-output = Здесь не найдено устройство захвата звука рабочего стола.
sources-audio-none-input = Микрофоны или линейные входы не найдены.
sources-audio-input-note = Полоски микшера получают индикатор громкости, фейдер, отключение звука, мониторинг, фильтры (шумоподавление, ворота, компрессор…) и назначение на дорожки. Всё остаётся на этой машине.

# Application Audio picker
sources-appaudio-title = Добавить звук приложения
sources-appaudio-looking = Поиск приложений, издающих звук…
sources-appaudio-none = Сейчас ни одно приложение не издаёт звук — запустите воспроизведение в приложении, затем обновите.
sources-appaudio-refresh = ⟳ Обновить
sources-appaudio-note = Захватывает звук именно этого приложения — со своим индикатором громкости, фейдером, отключением звука, фильтрами и дорожкой.

# Game Capture picker
sources-game-title = Захват игры
sources-game-checking = Проверка…
sources-game-use-portal = Использовать захват экрана (портал)
sources-game-use-window = Использовать захват окна вместо этого

# Image picker
sources-image-title = Добавить изображение
sources-image-file-label = Файл изображения (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Добавить изображение

# Path field
sources-browse = Обзор…

# Media picker
sources-media-title = Добавить медиа
sources-media-file-label = Медиафайл (mp4, mkv, webm, mov, .frec или изображение)
sources-media-loop-label = Зациклить (в конце начинать сначала)
sources-media-note = .frec воспроизводится через собственный кодек freally-video — ничего скачивать не нужно. Проводные форматы (mp4/mkv/webm/…) декодируются через компонент FFmpeg по запросу; их звук попадает в микшер как отдельная полоска.
sources-media-add = Добавить медиа

# Invite expiry options
sources-ttl-15min = 15 мин
sources-ttl-30min = 30 мин
sources-ttl-1hour = 1 час
sources-ttl-1day = 1 день

# Remote Guest form
sources-remote-copy-failed = не удалось скопировать — выделите ссылку и скопируйте вручную
sources-remote-join-failed = не удалось подключиться: { $error }
sources-remote-title = Удалённый гость (P2P-прототип)
sources-remote-host-heading = Ведущий — пригласить гостя
sources-remote-start-hosting = Начать приём
sources-remote-expires-label = Истекает
sources-remote-invite-expiry-aria = Срок действия приглашения
sources-remote-invite-link-aria = Ссылка-приглашение
sources-remote-copied = Скопировано ✓
sources-remote-copy = Копировать
sources-remote-share-note = Поделитесь этой ссылкой (Discord / сообщение / email). Она несёт ваш сеанс и истекает в заданный срок. Гость открывает её и подключается со своей веб-камерой.
sources-remote-qr-note = Отсканируйте на телефоне, чтобы подключиться прямо из браузера — камера + микрофон, без установки. Копируемая ссылка freally:// выше открывается в Freally Capture на машине, где оно установлено.
sources-remote-guest-heading = Гость — подключиться по приглашению
sources-remote-paste-placeholder = вставьте ссылку-приглашение
sources-remote-invite-input-aria = Ссылка-приглашение или ID сеанса
sources-remote-join = Подключиться с веб-камерой
sources-remote-session-note = Управление живым сеансом (заглушить, завершить) остаётся на панели вверху главного окна — этот диалог можно закрыть.
sources-remote-stop-session = Остановить сеанс

# Invite QR
sources-invite-qr-aria = QR-код ссылки-приглашения

# Remote device pickers
sources-devices-output-unavailable = маршрутизация вывода недоступна — воспроизведение на устройстве по умолчанию
sources-devices-mic-test-failed = проверка микрофона не удалась: { $error }
sources-devices-heading = Аудиоустройства сеанса
sources-devices-microphone-label = Микрофон
sources-devices-microphone-aria = Микрофон сеанса
sources-devices-system-default = Системное по умолчанию
sources-devices-output-label = Вывод
sources-devices-output-aria = Аудиовыход сеанса
sources-devices-stop-test = Остановить проверку
sources-devices-test = Проверка — услышать себя
sources-devices-testing-note = говорите в микрофон — вы слышите выбранные устройства в реальном времени
sources-devices-idle-note = зацикливает микрофон на вывод (наушники избавят от обратной связи)

# TURN relay section
sources-turn-save-failed = не удалось сохранить: { $error }
sources-turn-summary = Сеть — необязательный ретранслятор TURN (для опытных)
sources-turn-note-1 = Сеансы соединяются напрямую (P2P) — бесплатно, ретранслятор не нужен. Если ОБЕ стороны находятся за строгими NAT, прямой путь может не сработать; тогда медиа несёт ретранслятор TURN, который вы запускаете сами. Пропустить это нормально — большинство соединений работают напрямую.
sources-turn-note-2 = Бесплатный вариант: Oracle Cloud «Always Free» запускает coturn бесплатно (учтите: Oracle просит кредитную карту при регистрации, но конфигурация Always-Free остаётся бесплатной). Шаги: 1) создайте бесплатную ВМ, 2) установите coturn, 3) откройте UDP 3478, 4) задайте пользователя/пароль, 5) введите turn:ip-вашей-вм:3478 + учётные данные здесь. Ваши учётные данные остаются в локальном файле настроек и никогда не логируются.
sources-turn-url-label = URL TURN
sources-turn-url-placeholder = turn:host:3478 (пусто = только напрямую)
sources-turn-url-aria = URL TURN
sources-turn-username-label = Имя пользователя
sources-turn-username-aria = Имя пользователя TURN
sources-turn-credential-label = Учётные данные
sources-turn-credential-aria = Учётные данные TURN
sources-turn-note-3 = Ретранслятор включается, когда заполнены все три поля (серверу TURN нужны учётные данные), и применяется к следующему сеансу, который вы начнёте или к которому подключитесь. Проверьте его тестовым звонком «только через ретранслятор» между двумя своими машинами.
sources-turn-settings-unavailable = настройки недоступны (режим браузера)

# Color picker
sources-color-title = Добавить цвет
sources-color-label = Цвет
sources-color-width-label = Ширина
sources-color-height-label = Высота
sources-color-add = Добавить цвет
sources-testsignal-title = Добавить тестовый сигнал
sources-testsignal-pattern-label = Шаблон
sources-testsignal-bars = Цветные полосы SMPTE
sources-testsignal-grid = Калибровочная сетка
sources-testsignal-sweep = Развёртка движения
sources-testsignal-tone = Тон 1 кГц (−20 dBFS)
sources-testsignal-flash-beep = Вспышка + сигнал синхронизации A/V
sources-testsignal-note = Проверяйте сцены, кодировщики, проекторы и цели трансляции без подключённой камеры. Шаблон «вспышка + сигнал» питает стенд синхронизации A/V.
sources-testsignal-add = Добавить тестовый сигнал
sources-timer-title = Добавить таймер
sources-timer-mode-label = Режим
sources-timer-wall-clock = Настенные часы
sources-timer-countdown = Обратный отсчёт
sources-timer-stopwatch = Секундомер
sources-timer-since-live = Время с начала эфира
sources-timer-since-recording = Время с начала записи
sources-timer-note = Длительность, формат, оформление и действия по окончании отсчёта — в свойствах источника.
sources-timer-add = Добавить таймер

# Text picker
sources-text-title = Добавить текст
sources-text-label = Текст
sources-text-default = Текст
sources-text-color-label = Цвет
sources-text-color-aria = Цвет текста
sources-text-size-label = Размер (пикс.)
sources-text-note = Семейство шрифта, выравнивание, перенос и RTL находятся в свойствах источника. По умолчанию используется встроенный Noto Sans (вкл. арабский/иврит) — одинаковый на каждой машине.
sources-text-add = Добавить текст

# Existing source picker
sources-existing-title = Добавить существующий источник
sources-existing-empty = Источников ещё нет — сначала добавьте один в любую сцену. Существующие источники общие: переименование или перенастройка одного обновляет каждую сцену, где он показан.

# Screen + corners layout
sources-slot-off = Выкл.
sources-slot-center = Центр (экран)
sources-slot-top-left = Сверху слева
sources-slot-top-right = Сверху справа
sources-slot-bottom-left = Снизу слева
sources-slot-bottom-right = Снизу справа
sources-layout-title = Расположить: экран + углы
sources-layout-empty = Сначала добавьте в эту сцену захват экрана и одну или несколько камер, затем расположите их здесь.
sources-layout-note = Поместите экран в центр и до четырёх камер по углам — ваша компоновка для объяснений / подкаста. В каждом углу — веб-камера, захваченное окно звонка или медиаклип. Любой из них можно потом перетащить на холсте.
sources-layout-slot-aria = Слот для { $name }
sources-layout-apply = Применить компоновку


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Управление
controls-start-stop-title-stop = Остановить и финализировать запись
controls-start-stop-title-start = Записать поток программы с настройками из «Настройки → Вывод»
controls-finalizing = ◌ Финализация…
controls-stop-recording = ■ Остановить запись
controls-start-recording = ● Начать запись
controls-marker-title = Поставить метку главы в этот момент — она попадает в ЗАПИСЬ (главы mkv или отдельный файл). Метки потока на стороне платформы требуют аккаунтов платформ, которых это приложение никогда не запрашивает.
controls-marker = ◈ Метка
controls-pause-title-resume = Возобновить — файл продолжается как единая непрерывная дорожка
controls-pause-title-pause = Пауза — кадры не пишутся; возобновление продолжает тот же воспроизводимый файл
controls-resume-recording = ▶ Возобновить запись
controls-pause-recording = ⏸ Приостановить запись
controls-reactions-label = Реакции (впечатаны в программу)
controls-reactions-title = Пустить реакцию поверх программы — записывается И транслируется, так что в повторе виден точный момент. Зрители в чате тоже их запускают (их эмодзи-реакции всплывают автоматически); наплыв лишь ограничивает то, что на экране.
controls-react = Реакция { $emoji }
controls-virtual-camera-title = Виртуальной камере нужен собственный подписанный компонент драйвера для каждой ОС (Win11 MFCreateVirtualCamera / Win10 DirectShow / расширение CoreMediaIO для macOS / v4l2loopback для Linux) — она поставляется отдельной вехой. Модель вывода к ней готова: программа, вертикальный холст или один источник, с парным виртуальным микрофоном в Windows/Linux (в macOS нет API виртуального микрофона — говорим честно).
controls-virtual-camera = ⌁ Запустить виртуальную камеру
controls-files-title = Готовые записи + действие ремукса в mp4
controls-files = ▤ Файлы…
controls-output-title = Формат записи, кодировщик, папка, дорожки и разбиение
controls-output = ⚙ Вывод…
controls-stream-title = Цель эфира: сервис, ключ потока, кодировщик, битрейт
controls-stream = ⦿ Трансляция…
controls-codecs-title = Компонент проводных кодеков ffmpeg по запросу (чётко помечен, никогда не включается в сборку)
controls-codecs = ⬡ Кодеки…
controls-replay-title = Длина буфера повтора + пресеты качества
controls-replay = ⟲ Повтор…
controls-keys-title = Глобальные горячие клавиши: запись, эфир, переход, сохранить повтор
controls-keys = ⌨ Клавиши…
controls-scripts-title = Изолированные скрипты Lua: реагируют на события эфира/сцены/записи, управляют студией
controls-scripts = ⚡ Скрипты…
controls-docks-title = Браузерные доки: откройте всплывающий чат, страницу уведомлений или кнопки Companion как окно рядом со студией
controls-docks = ⧉ Доки…
controls-remote-title = Удалённый WebSocket API для контроллеров Stream Deck / Companion (по умолчанию выключен)
controls-remote = ⌁ Удалённое управление…
controls-profiles-title = Профили (настройки) + коллекции сцен — переключаемые снимки
controls-profiles = ▣ Профили…
controls-bug-title = Сообщить об ошибке — анонимно, по желанию (ничего не отправляется автоматически)
controls-bug = 🐞 Сообщить об ошибке…
controls-updates-title = Проверить обновления — подписанные, проверенные, ничего не скачивается без клика
controls-updates = ⭳ Проверить обновления…
controls-saved = Сохранено: { $path }

# --- MixerDock.tsx ---
mixer-title = Аудиомикшер
mixer-monitor-error = мониторинг: { $error }
mixer-switch-to-horizontal = Переключить на горизонтальные полоски
mixer-switch-to-vertical = Переключить на вертикальные полоски
mixer-layout-aria-vertical = Компоновка микшера: вертикальная — переключить на горизонтальную
mixer-layout-aria-horizontal = Компоновка микшера: горизонтальная — переключить на вертикальную
mixer-empty = В этой сцене нет аудиоисточников — добавьте «Захват аудиовхода» (микрофон) или «Захват аудиовыхода» (звук рабочего стола) через «+» в разделе «Источники». Полоски получают индикатор громкости, фейдер, отключение звука, мониторинг, фильтры и назначение на дорожки.
mixer-advanced-title = Аудио — { $name }
mixer-loudness-label = Громкость программы (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Мгновенная громкость (400 мс)
mixer-short-term-title = Кратковременная громкость (3 с)
mixer-lufs-short = S { $value }
mixer-monitor-label = Мониторинг
mixer-monitor-device-aria = Устройство вывода мониторинга
mixer-default-output = Устройство вывода по умолчанию

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Память
stats-dropped = Пропущено
stats-render = Рендеринг
stats-gpu = GPU
stats-gpu-compositing = компоновка
stats-gpu-idle = простой
stats-vertical-fps = 9:16 FPS
stats-targets-label = Цели трансляции
stats-shared-encode = · общее кодирование
stats-starting = Запуск компоновщика…

# --- ScenesRail.tsx ---
scenes-title = Сцены
scenes-new-scene-name = Сцена
scenes-add = Добавить сцену
scenes-empty = Подключение к ядру студии…
scenes-rename = Переименовать { $name }
scenes-on-program = В программе
scenes-preview = Превью { $name }
scenes-switch-to = Переключиться на { $name }
scenes-move-up = Вверх
scenes-move-up-aria = Переместить { $name } вверх
scenes-move-down = Вниз
scenes-move-down-aria = Переместить { $name } вниз
scenes-last-stays = Последняя сцена остаётся
scenes-remove = Удалить эту сцену
scenes-remove-aria = Удалить { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Уровень
channelstrip-monitor-off = Мониторинг выкл.
channelstrip-monitor-only = Только мониторинг (не в миксе)
channelstrip-monitor-and-output = Мониторинг и вывод
channelstrip-status-error = ошибка
channelstrip-status-live = активно
channelstrip-status-waiting-audio = ожидание звука
channelstrip-status = статус: { $state }
channelstrip-status-waiting = ожидание
channelstrip-mute = Заглушить
channelstrip-unmute = Включить звук
channelstrip-mute-source = Заглушить { $name }
channelstrip-unmute-source = Включить звук { $name }
channelstrip-scene-mix-on = Микс по сцене ВКЛ. — эта полоска переопределяет глобальный микс для этой сцены (нажмите, чтобы снова следовать глобальному миксу)
channelstrip-scene-mix-off = Микс по сцене — дать этой полоске собственный фейдер/отключение звука для текущей сцены
channelstrip-scene-mix-label = Микс по сцене для { $name }
channelstrip-monitor-cycle = { $mode } — нажмите для переключения
channelstrip-monitor-mode = Режим мониторинга { $name }: { $mode }
channelstrip-audio-filters-title = Аудиофильтры (шумоподавление, ворота, компрессор…)
channelstrip-audio-filters-label = Аудиофильтры для { $name }
channelstrip-advanced-title = Смещение синхронизации и горячие клавиши push-to-talk
channelstrip-advanced-label = Расширенные настройки звука для { $name }
channelstrip-track-assignment = Назначение дорожек
channelstrip-track = Дорожка { $n }
channelstrip-track-assigned = Дорожка { $n } (назначена)
channelstrip-track-label = Дорожка { $n } для { $name }
channelstrip-device-error = ошибка устройства
channelstrip-audio-device-error = ошибка аудиоустройства
channelstrip-volume-label = Громкость { $name } в децибелах
channelstrip-ptt-hold = Push-to-talk: удерживайте { $key }
channelstrip-sync-offset = Смещение синхронизации (мс, 0–{ $max } — задерживает этот звук)
channelstrip-solo-title = Соло (PFL) — в мониторе слышны только соло-полосы; программный микс не меняется
channelstrip-solo-source = Соло { $name } (PFL)
channelstrip-pan-label = Баланс (двойной щелчок сбрасывает)
channelstrip-pan-aria = Баланс { $name }
channelstrip-mono-label = Свести в моно
channelstrip-ptt-hotkey = Горячая клавиша push-to-talk (без звука, пока не удерживается)
channelstrip-ptt-placeholder = напр. Ctrl+Shift+T или F13
channelstrip-ptt-aria = Горячая клавиша push-to-talk
channelstrip-ptm-hotkey = Горячая клавиша push-to-mute (без звука, пока удерживается)
channelstrip-ptm-placeholder = напр. Ctrl+Shift+M
channelstrip-ptm-aria = Горячая клавиша push-to-mute
channelstrip-hotkeys-note = Горячие клавиши работают, пока сфокусированы другие приложения. В Linux/Wayland глобальные горячие клавиши могут быть недоступны — это ограничение компоновщика, говорим честно.
channelstrip-apply = Применить

# --- LiveButton.tsx ---
livebutton-failure-ended = трансляция завершена
livebutton-title-live = Завершить трансляцию — на всех целях (идущая запись продолжается)
livebutton-title-offline = Выйти в эфир на каждую включённую цель «Настройки → Трансляция»
livebutton-end-stream = ■ Завершить трансляцию
livebutton-aria-reconnecting = Переподключение
livebutton-aria-live = В эфире
livebutton-badge-retry = попытка { $n }
livebutton-badge-live = в эфире
livebutton-go-live = ⦿ В эфир

# --- RecDot.tsx ---
recdot-paused-aria = Запись приостановлена
recdot-recording-aria = Запись
recdot-tracks-one = записывается { $count } аудиодорожка
recdot-tracks-other = записывается аудиодорожек: { $count }
recdot-paused = пауза

# --- ReplayControls.tsx ---
replaycontrols-saved = Повтор сохранён — { $name }
replaycontrols-failure-stopped = буфер остановлен
replaycontrols-title-disarm = Отключить буфер повтора (отбрасывает несохранённую историю)
replaycontrols-title-arm = Включить скользящий буфер повтора — держит последние N секунд готовыми к сохранению (собственное лёгкое кодирование; трансляция и запись не затрагиваются)
replaycontrols-replay-seconds = ⟲ Повтор { $seconds } с
replaycontrols-arm = ⟲ Включить буфер повтора
replaycontrols-save-title = Сохранить последние N секунд в папку записей (также по горячей клавише «Сохранить повтор»)
replaycontrols-save = ⤓ Сохранить

# --- PropertiesDialog.tsx ---
properties-title = Свойства — { $name }
properties-name = Имя
properties-cancel = Отмена
properties-apply = Применить
properties-youtube = YouTube — URL канала / watch / live_chat (без ключа, без входа, никогда)
properties-twitch = Twitch — имя канала (анонимно)
properties-kick = Kick — слаг канала (публичный эндпоинт)
properties-width-px = Ширина (пикс.)
properties-lines = Строки
properties-font-px = Шрифт (пикс.)
properties-images = Файлы изображений (по одному пути на строку, показываются по порядку)
properties-per-slide = На слайд (мс)
properties-crossfade = Перекрёстное затухание (мс, 0 = резкий переход)
properties-loop-slideshow = Зациклить (выкл. = удерживать последний слайд)
properties-shuffle = Перемешивать каждый цикл
properties-nested-scene = Сцена, которую составляет этот источник (сцена, уже содержащая эту, отклоняется)
properties-portal-note = Портал ScreenCast Wayland выбирает экран или окно в системном диалоге при каждом запуске этого источника — здесь нечего настраивать, так задумано.
properties-appaudio-capturing = Захват звука из { $exe }
properties-appaudio-exe-fallback = приложение
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Повторно добавьте источник, чтобы нацелиться на другое приложение (идентификатор процесса меняется при перезапуске приложения).
properties-image-file = Файл изображения
properties-media-file = Медиафайл (mp4, mkv, webm, mov, .frec или изображение)
properties-media-loop = Зациклить (в конце начинать сначала)
properties-media-hwdecode = Аппаратное декодирование (само откатывается к программному)
properties-media-note = .frec воспроизводится через собственный кодек freally-video — ничего скачивать не нужно. Другие видеоформаты декодируются через компонент FFmpeg по запросу. Звук файла получает собственную полоску микшера; смещение синхронизации полоски точно настраивает совмещение A/V. Клип без звука оставляет свою полоску беззвучной.
properties-color = Цвет
properties-width = Ширина
properties-height = Высота
properties-testtone-note = Непрерывная синусоида 1 кГц на −20 dBFS. Уровень и заглушение — на полосе микшера; больше настраивать нечего.
properties-timer-format = Формат времени (strftime)
properties-timer-format-note = напр. %H:%M:%S (по умолчанию), %I:%M %p, %A %H:%M — неверный шаблон вернётся к %H:%M:%S.
properties-timer-utc = Смещение UTC (минуты)
properties-timer-utc-placeholder = местное время
properties-timer-duration = Длительность (секунды)
properties-timer-target = Отсчёт до (HH:MM)
properties-timer-target-note = Цель по часам идёт сама и повторяется ежедневно; оставьте пустым, чтобы использовать длительность со Старт/Пауза/Сброс.
properties-timer-end = На нуле
properties-timer-end-none = Ничего
properties-timer-end-flash = Мигать таймером
properties-timer-end-switch = Сменить сцену
properties-timer-end-scene = Сцена
properties-timer-size = Размер (px)
properties-timer-start = Старт
properties-timer-pause = Пауза
properties-timer-reset = Сброс
properties-text-file = Читать из файла (путь; пусто = текст выше)
properties-text-binding = Разбирать как
properties-text-binding-whole = Файл целиком
properties-text-binding-csv = Ячейка CSV
properties-text-binding-json = Указатель JSON
properties-text-csv-row = Строка
properties-text-csv-column = Столбец
properties-text-csv-column-placeholder = имя или номер
properties-text-json-pointer = Указатель
properties-text-file-note = Файл перечитывается в течение полусекунды после изменения. Атомарная запись (temp + переименование) переносится: последнее хорошее значение остаётся на экране во время подмены.
avsync-title = Калибровка синхронизации A/V
avsync-intro = Проиграйте встроенный шаблон «вспышка + сигнал» через экран и колонки, снимите его камерой и микрофоном, которые нужно выровнять, — стенд измерит расхождение. Петля проходит через экран и колонки, так что их небольшие задержки тоже учитываются.
avsync-video-label = Камера (источник видео)
avsync-audio-label = Микрофон (источник звука)
avsync-pick = Выберите источник…
avsync-no-video = Сначала добавьте камеру как источник — стенд измеряет источники, а не «сырые» устройства.
avsync-no-audio = Сначала добавьте микрофон как источник звука.
avsync-projector = Программа во весь экран на
avsync-projector-open = Открыть проектор
avsync-projector-window-title = Программа — синхронизация A/V
avsync-start-note = Запуск временно добавляет источник «Шаблон синхронизации A/V» поверх текущей сцены и проигрывает сигнал на устройстве мониторинга. По завершении всё убирается.
avsync-manual = Смещение синхронизации (мс, вручную)
avsync-start = Начать калибровку
avsync-measuring = Измерение около 12 секунд — направьте камеру на мигающую программу и не шумите…
avsync-flash-seen = Камера видит вспышку
avsync-flash-waiting = Ожидание, пока камера увидит вспышку…
avsync-beep-heard = Микрофон слышит сигнал
avsync-beep-waiting = Ожидание, пока микрофон услышит сигнал…
avsync-cancel = Отмена
avsync-result-offset = Видео приходит на { $offset } мс позже звука.
avsync-result-detail = Измерено за { $cycles } циклов, ±{ $jitter } мс.
avsync-negative = Звук и так приходит позже видео. Задержка звука не исправит это направление — если звук этой камеры несёт другая полоса, уменьшите смещение там.
avsync-over-cap = Измеренный разрыв превышает предел смещения { $max } мс. Такой разрыв обычно означает не тот источник — проверьте цепочку и измерьте снова.
avsync-applied = Применено — смещение микрофона теперь { $offset } мс.
avsync-apply = Применить { $offset } мс к микрофону
avsync-again = Измерить снова
avsync-close = Закрыть
avsync-error-noFlash = Камера ни разу не увидела вспышку. Направьте её на мигающую программу (полный экран помогает), убедитесь, что источник работает, и измерьте снова.
avsync-error-noBeep = Микрофон ни разу не услышал сигнал. Убедитесь, что устройство мониторинга слышно и микрофон работает (не заглушён push-to-talk), и измерьте снова.
avsync-error-tooFewCycles = Слишком мало чистых циклов вспышки/сигнала. Держите шаблон хорошо видимым и слышимым весь замер.
avsync-error-notThePattern = Увиденное или услышанное не повторяется в ритме шаблона — вероятно, это свет или шум комнаты, а не тестовый сигнал.
avsync-error-unstable = Циклы слишком расходятся, чтобы доверять одному числу. Закрепите камеру, уменьшите шум и измерьте снова.
hotkey-audit-title = Карта горячих клавиш
hotkey-audit-search = Поиск
hotkey-audit-filter = Функция
hotkey-audit-filter-all = Все функции
hotkey-audit-col-key = Клавиша
hotkey-audit-col-action = Действие
hotkey-audit-col-where = Где
hotkey-audit-col-status = Статус
hotkey-audit-ok = OK
hotkey-audit-shared = Разделяют { $count } привязки
hotkey-audit-unregistered = Не зарегистрирована в ОС (занята другим приложением или недоступна)
hotkey-audit-invalid = Недопустимое сочетание
hotkey-audit-empty = Горячих клавиш пока нет — назначьте их в Настройки → Горячие клавиши или на полосе микшера.
hotkey-audit-export = Экспорт шпаргалки
hotkey-audit-exported = Сохранено в { $path }
hotkey-audit-note = Назначение и изменение клавиш — в Настройки → Горячие клавиши (глобальные действия) и на каждой полосе микшера (push-to-talk / push-to-mute); эта таблица их проверяет и документирует.
hotkey-audit-action-record = Переключить запись
hotkey-audit-action-go-live = Переключить трансляцию
hotkey-audit-action-transition = Выполнить переход
hotkey-audit-action-save-replay = Сохранить повтор
hotkey-audit-action-add-marker = Добавить маркер
hotkey-audit-action-still = Снять стоп-кадр
hotkey-audit-action-panic = Экран паники
hotkey-audit-action-timer-toggle = Старт/пауза всех таймеров
hotkey-audit-action-timer-reset = Сброс всех таймеров
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Запись
hotkey-audit-feature-streaming = Трансляция
hotkey-audit-feature-studio = Режим студии
hotkey-audit-feature-replay = Повтор
hotkey-audit-feature-markers = Маркеры
hotkey-audit-feature-stills = Стоп-кадры
hotkey-audit-feature-panic = Паника
hotkey-audit-feature-timers = Таймеры
hotkey-audit-feature-audio = Звук (по источникам)
properties-text = Текст
properties-font-family = Семейство шрифта (системное; пусто = по умолчанию)
properties-size-px = Размер (пикс.)
properties-text-color = Цвет текста
properties-align = Выравнивание
properties-align-left = по левому краю
properties-align-center = по центру
properties-align-right = по правому краю
properties-line-spacing = Межстрочный интервал
properties-wrap-width = Ширина переноса (пикс.; 0 = выкл.)
properties-force-rtl = Принудительно справа налево
properties-text-note = Рендеринг использует настоящий шейпинг (арабское соединение, лигатуры) и двунаправленный порядок строк. По умолчанию используется встроенное семейство Noto Sans (вкл. арабский/иврит); системные семейства тоже работают. CJK пока использует системные шрифты.
properties-repick-capturing = Захват: { $label }
properties-repick-looking = Поиск источников…
properties-repick-none-displays = Не найдено дисплеев для повторного выбора.
properties-repick-none-windows = Не найдено окон для повторного выбора.
properties-repick-again = Выбрать снова:
properties-device = Устройство
properties-video-current-device = (текущее устройство)
properties-format = Формат
properties-format-auto-loading = Авто (загрузка форматов…)
properties-deinterlace = Деинтерлейсинг
properties-deinterlace-off = Выкл.
properties-deinterlace-discard = Отброс (удвоение строк одного поля)
properties-deinterlace-bob = Боб (поля попеременно)
properties-deinterlace-linear = Линейный (интерполяция)
properties-deinterlace-blend = Смешение (среднее полей)
properties-deinterlace-adaptive = Адаптивный к движению (класс yadif)
properties-field-order = Порядок полей
properties-field-order-top = Сначала верхнее поле
properties-field-order-bottom = Сначала нижнее поле
properties-deinterlace-note = Для чересстрочных сигналов карт захвата. Чистый CPU, одинаково на всех ОС; изменение перезапускает устройство (как смена формата).
camera-controls-title = Управление камерой
camera-controls-refresh = Обновить
camera-controls-reset = Сбросить профиль
camera-controls-empty = Сейчас регулировок нет — устройство должно вести трансляцию (сначала добавьте его в сцену), а некоторые бэкенды не сообщают ничего (особенно macOS). Это честное состояние для каждой ОС.
camera-controls-note = Изменения действуют сразу и сохраняются в профиль устройства; он снова применяется при переподключении и перезапуске.
camera-control-brightness = Яркость
camera-control-contrast = Контраст
camera-control-hue = Оттенок
camera-control-saturation = Насыщенность
camera-control-sharpness = Резкость
camera-control-gamma = Гамма
camera-control-white-balance = Баланс белого
camera-control-backlight = Компенсация контрового света
camera-control-gain = Усиление
camera-control-pan = Панорама
camera-control-tilt = Наклон
camera-control-zoom = Зум
camera-control-exposure = Экспозиция
camera-control-iris = Диафрагма
camera-control-focus = Фокус
properties-format-auto = Авто (максимальное разрешение)
properties-audio-capture-of = Захватить звук
properties-audio-default-output = Устройство вывода по умолчанию (то, что вы слышите)
properties-audio-default-input = Устройство ввода по умолчанию
properties-audio-default-suffix = (по умолчанию)
properties-audio-current-device = (текущее устройство: { $id })

# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Усиление
audiofilters-name-noise-gate = Шумовые ворота
audiofilters-name-compressor = Компрессор
audiofilters-name-limiter = Лимитер
audiofilters-name-eq = 3-полосный эквалайзер
audiofilters-name-denoise = Шумоподавление
audiofilters-name-ducking = Приглушение
audiofilters-title = Аудиофильтры — { $name }
audiofilters-chain-header = Цепочка фильтров (верхний работает первым, до фейдера)
audiofilters-add = + Добавить фильтр
audiofilters-add-menu = Добавить аудиофильтр
audiofilters-empty = Пока нет фильтров — подавите шум микрофона (классический DSP, без ML), закройте комнату воротами, укротите пики компрессором или приглушите музыку под ваш голос.
audiofilters-enable = Включить { $name }
audiofilters-run-earlier = Выполнять раньше
audiofilters-move-up = Переместить { $name } вверх
audiofilters-run-later = Выполнять позже
audiofilters-move-down = Переместить { $name } вниз
audiofilters-remove-title = Удалить фильтр
audiofilters-remove = Удалить { $name }
audiofilters-gain-db = Усиление (dB)
audiofilters-open-db = Открывать при (dB)
audiofilters-close-db = Закрывать при (dB)
audiofilters-attack-ms = Атака (мс)
audiofilters-hold-ms = Удержание (мс)
audiofilters-release-ms = Восстановление (мс)
audiofilters-ratio = Соотношение (:1)
audiofilters-threshold-db = Порог (dB)
audiofilters-output-gain-db = Выходное усиление (dB)
audiofilters-ceiling-db = Потолок (dB)
audiofilters-low-db = Низкие (dB)
audiofilters-mid-db = Средние (dB)
audiofilters-high-db = Высокие (dB)
audiofilters-strength = Сила
audiofilters-denoise-note = Собственное спектральное подавление классического DSP — постоянный шум (вентиляторы, шипение) убирается, а речь проходит. Без ML, без моделей, согласно уставу.
audiofilters-duck-under = Приглушать под
audiofilters-ducking-trigger = Источник-триггер приглушения
audiofilters-pick-trigger = (выберите триггер — напр. ваш микрофон)
audiofilters-trigger-at-db = Срабатывать при (dB)
audiofilters-duck-by-db = Приглушать на (dB)

# --- FiltersDialog.tsx ---
filters-name-chroma-key = Хромакей
filters-name-color-key = Ключ по цвету
filters-name-luma-key = Ключ по яркости
filters-name-render-delay = Задержка рендеринга
filters-name-color-correction = Цветокоррекция
filters-name-lut = Применить LUT
filters-name-blur = Размытие
filters-name-mask = Маска изображения
filters-name-sharpen = Резкость
filters-name-scroll = Прокрутка
filters-name-crop = Обрезка
filters-title = Фильтры — { $name }
filters-blend-mode = Режим наложения
filters-chain-header = Цепочка фильтров (верхний работает первым)
filters-add = + Добавить фильтр
filters-add-menu = Добавить фильтр
filters-empty = Пока нет фильтров — примените хромакей к веб-камере, цветокоррекцию к захвату или прокрутите бегущую строку.
filters-enable = Включить { $name }
filters-run-earlier = Выполнять раньше
filters-move-up = Переместить { $name } вверх
filters-run-later = Выполнять позже
filters-move-down = Переместить { $name } вниз
filters-remove-title = Удалить фильтр
filters-remove = Удалить { $name }
filters-key-color-rgb = Ключевой цвет (любой цвет, расстояние RGB)
filters-similarity = Сходство
filters-smoothness = Плавность
filters-luma-min = Мин. яркость (тёмное убирается)
filters-luma-max = Макс. яркость (светлое убирается)
filters-delay = Задержка (мс — только видео, напр. для синхронизации со звуком; ограничена 500)
filters-key-color = Ключевой цвет
filters-spill = Растекание
filters-gamma = Гамма
filters-brightness = Яркость
filters-contrast = Контраст
filters-saturation = Насыщенность
filters-hue-shift = Сдвиг оттенка
filters-opacity = Непрозрачность
filters-cube-file = Файл .cube
filters-amount = Величина
filters-radius = Радиус
filters-mask-image = Изображение маски
filters-mask-mode = Режим
filters-mask-alpha = альфа
filters-mask-luma = яркость
filters-mask-invert = инвертировать
filters-speed-x = Скорость X (пикс./с)
filters-speed-y = Скорость Y (пикс./с)
filters-crop-left = слева
filters-crop-top = сверху
filters-crop-right = справа
filters-crop-bottom = снизу
filters-crop-aria = обрезка { $side }

# --- PickerShell.tsx ---
pickershell-refresh-aria = Обновить
pickershell-refresh-title = Обновить список
pickershell-close = Закрыть


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Сообщить об ошибке
bugreport-intro = Отчёты анонимны и отправляются по желанию — ничего не отправляется автоматически. Вы просмотрите точный текст ниже, затем отправите его через предзаполненную issue на GitHub или ваше почтовое приложение. Никаких персональных данных (ваш домашний путь и имя пользователя скрыты); ни аккаунта, ни сервера.
bugreport-crash-notice = Freally Capture неожиданно закрылось при предыдущем запуске — анонимные детали сбоя включены ниже. Их отправка поможет быстро всё исправить.
bugreport-description-label = Что вы делали, когда это случилось? (необязательно)
bugreport-description-placeholder = напр. превью зависло, когда я добавил вторую веб-камеру
bugreport-include-crash = Включить анонимные детали сбоя с последнего запуска
bugreport-preview-label = Что именно будет отправлено
bugreport-open-github = Открыть issue на GitHub
bugreport-gmail-title = Открывает окно составления письма Gmail в вашем браузере, предзаполненное. Не вошли? Google сначала покажет экран входа.
bugreport-compose-gmail = Составить в Gmail
bugreport-email-title = Открывает черновик в почтовом приложении, которое этот ПК использует по умолчанию (Outlook, Thunderbird, Mail…)
bugreport-send-email = Отправить email
bugreport-copied = Скопировано ✓
bugreport-copy-report = Копировать отчёт
bugreport-dismiss-crash = Отклонить сбой
bugreport-copy-failed = не удалось скопировать — выделите текст и скопируйте вручную
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = ЧТО ПРОИЗОШЛО
bugreport-preview-no-description = (описание не предоставлено)
bugreport-preview-diagnostics = АНОНИМНАЯ ДИАГНОСТИКА (без персональных данных)
bugreport-preview-from = От: Freally Capture
bugreport-preview-crash-excerpt = --- фрагмент сбоя ---

# --- Updates.tsx ---
updates-title = Обновление ПО
updates-checking = Проверка обновлений…
updates-uptodate = У вас последняя версия.
updates-check-again = Проверить снова
updates-available = Доступна версия { $version }
updates-current-version = (у вас { $current })
updates-release-notes-label = Версия { $version } — Примечания к выпуску
updates-confirm = Обновить сейчас? Загрузка проверяется по встроенному ключу подписи перед применением. Freally Capture закрывается, запускается установщик, и новая версия открывается сама.
updates-yes-update-now = Да, обновить сейчас
updates-no-not-now = Нет, не сейчас
updates-downloading = Загрузка { $version }…
updates-starting = запуск…
updates-installed = Обновление установлено.
updates-restart-now = Перезапустить сейчас
updates-restart-later = Перезапустить позже
updates-try-again = Повторить

# --- Models.tsx ---
models-title = Компоненты
models-ffmpeg-heading = FFmpeg — проводные кодеки
models-badge-third-party = Сторонний · не включён в сборку
models-ffmpeg-desc = Собственный движок Freally Capture записывает без потерь freally-video (.frec) без чего-либо дополнительного. Для записи проводных форматов, которые ожидают платформы и плееры — H.264/AAC (и HEVC/AV1) в mp4/mkv/mov/webm — используется FFmpeg, отдельный инструмент, с которым это приложение никогда не поставляется: эти кодеки обременены патентами, поэтому он остаётся необязательным и чётко помеченным. Он загружается по запросу из закреплённой сборки ниже, проверяется по SHA-256 перед первым использованием, кэшируется для каждого пользователя и запускается как отдельный процесс. Его лицензия (LGPL/GPL) — своя, см. THIRD-PARTY-NOTICES.
models-checking = Проверка…
models-ffmpeg-not-installed = Не установлен. Доступно: FFmpeg { $version } из { $source } (загрузка { $size }).
models-ffmpeg-none-pinned = Для этой платформы пока не закреплена сборка FFmpeg — запись проводных кодеков здесь недоступна. Запись freally-video без потерь не затронута.
models-ffmpeg-download-verify = Загрузить и проверить ({ $size })
models-downloading = Загрузка…
models-download-of = из
models-cancel = Отмена
models-ffmpeg-verifying = Проверка загрузки по закреплённому SHA-256…
models-ffmpeg-extracting = Распаковка…
models-ffmpeg-ready = Установлен и проверен — { $version }
models-remove = Удалить
models-ffmpeg-retry = Повторить загрузку
models-network-note = Загрузка — единственное сетевое действие на этой панели, и оно никогда не начинается само. Неудачная контрольная сумма прерывает установку — приложение отказывается запускать байты, за которые не может поручиться.
models-cef-heading = Среда выполнения браузерного источника — Chromium (CEF)
models-cef-desc = Браузерные источники рендерят веб-страницы (уведомления, виджеты, оверлеи) через Chromium Embedded Framework — среду выполнения ~100 МБ, с которой это приложение никогда не поставляется. Она загружается по запросу из официального индекса сборок CEF, проверяется по SHA-1 этого индекса перед распаковкой и кэшируется для каждого пользователя. Браузерный источник, который через неё рендерит, приходит со своей вехой; это устанавливает нужную ему среду выполнения.
models-cef-download-install = Загрузить и установить
models-cef-unsupported = CEF не публикует сборку для этой платформы — браузерные источники здесь недоступны.
models-cef-resolving = Определение последней стабильной сборки…
models-cef-verifying = Проверка загрузки по SHA-1 индекса…
models-cef-extracting = Распаковка среды выполнения…
models-cef-ready = Установлено — CEF { $version }.
models-cef-retry = Повторить
models-integrations-heading = Необязательные интеграции
models-badge-never-bundled = Никогда не включается в сборку
models-ndi-detected = Обнаружено
models-ndi-not-installed = Не установлено
models-vst-available = Доступно
models-vst-not-available = Недоступно

# --- Recordings.tsx ---
recordings-title = Записи
recordings-loading = Чтение папки…
recordings-empty = Записей ещё нет — «Начать запись» пишет в папку, заданную в «Вывод».
recordings-frec-label = собственный без потерь (freally-video)
recordings-remux-title = Переупаковать как mp4 — копирование потока, без перекодирования, без изменения качества (нужен компонент FFmpeg)
recordings-remuxing = Ремукс…
recordings-remux-to-mp4 = Ремукс в MP4
recordings-export-mp4-title = Декодировать собственный .frec и перекодировать в MP4 (H.264/AAC), чтобы он проигрывался в любом плеере — нужен компонент FFmpeg
recordings-exporting = Экспорт…
recordings-export-mp4 = Экспорт → MP4
recordings-export-mkv-title = Декодировать собственный .frec и перекодировать в MKV, чтобы он проигрывался в любом плеере
recordings-starting = запуск…
recordings-frames = { $done } / { $total } кадров
recordings-cancel = Отмена
recordings-export-cancelled = Экспорт отменён.
recordings-exported-to = Экспортировано в { $path }
recordings-remuxed-to = Ремукс в { $path }

# --- OpenedFrec.tsx ---
openfrec-title = Открыть запись .frec
openfrec-desc = Freally Capture записывает в собственном формате без потерь .frec — но не воспроизводит его. Freally Player будет воспроизводить .frec напрямую после выпуска. Пока экспортируйте его в MP4/MKV, и он проигрывается в любом плеере (VLC, плеере вашей ОС, где угодно).
openfrec-exported-to = Экспортировано в { $path }
openfrec-exporting = Экспорт…
openfrec-starting = запуск…
openfrec-export-mp4 = Экспорт → MP4
openfrec-export-mkv = Экспорт → MKV

# --- VerticalCanvasDialog.tsx ---
vertical-title = Вертикальный холст (9:16)
vertical-enable = Включить второй холст — записывается и транслируется независимо от программы
vertical-scene-label = Сцена, которую составляет этот холст
vertical-width = Ширина
vertical-height = Высота
vertical-preview-alt = Превью вертикального холста
vertical-note = Позиции элементов пиксельно-точны на разных холстах: выберите эту сцену на рейке сцен, чтобы расположить её, пока это превью показывает вертикальный результат. Цели трансляции выбирают этот холст в ⦿ Трансляция…; «Настройки → Вывод» может записывать его вместе с основным файлом.
vertical-close = Закрыть

# --- EulaGate.tsx ---
eula-title = Freally Capture — Лицензионное соглашение
eula-version = v{ $version }
eula-intro = Пожалуйста, прочтите и примите это соглашение, чтобы использовать Freally Capture. Коротко: это нейтральный инструмент, и вы несёте полную ответственность за то, что захватываете, записываете и транслируете — и за наличие прав на это.
eula-thanks = Спасибо за прочтение.
eula-scroll-hint = Прокрутите до конца, чтобы продолжить.
eula-decline = Отклонить и выйти
eula-agree = Я согласен


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Вывод
output-loading = Настройки ещё загружаются…
output-container-frec = freally-video (.frec) — без потерь, собственный, ничего скачивать не нужно
output-container-mkv = MKV — устойчив к сбоям; ремукс в mp4 позже
output-container-mp4 = MP4 — проигрывается везде
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Без потерь
output-preset-lossless-title = Собственный кодек freally-video — бит-в-бит, без загрузки
output-preset-high-label = Высокое качество
output-preset-high-title = MP4, лучший обнаруженный кодировщик, почти без потерь CQ 16, пресет «Качество»
output-preset-balanced-label = Сбалансированный
output-preset-balanced-title = MKV, лучший обнаруженный кодировщик, CQ 23, пресет «Сбалансированный»
output-recording-format = Формат записи
output-ffmpeg-warning = Этому формату нужен компонент FFmpeg (проводные кодеки — не включены в сборку). Для .frec без потерь ничего не нужно.
output-install = Установить…
output-recordings-folder = Папка записей
output-folder-placeholder = Папка «Видео» ОС
output-filename-prefix = Префикс имени файла
output-recording-template = Имя файла записи
output-replay-template = Имя файла повтора
output-still-template = Имя файла стоп-кадра
output-template-tokens = Токены: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Папка повторов
output-still-folder = Папка стоп-кадров
output-same-folder-placeholder = Папка записей
output-frame-rate = Частота кадров
output-fps-option = { $fps } fps
output-split-every = Разбивать каждые (минуты, 0 = выкл.)
output-output-width = Ширина вывода (0 = холст; только проводные форматы)
output-output-height = Высота вывода (0 = холст)
output-record-vertical = Также записывать вертикальный холст (параллельный файл «… (vertical)»; нужен включённый холст 9:16)
output-audio-tracks = Аудиодорожки
output-recorded-tracks-group = Записываемые дорожки
output-track-last-one = Хотя бы одна дорожка должна записываться
output-record-track-on = Запись дорожки { $index }: вкл.
output-record-track-off = Запись дорожки { $index }: выкл.
output-encoder-heading = Кодировщик
output-video-encoder = Видеокодировщик
output-encoder-auto = Авто — лучший обнаруженный (H.264)
output-encoder-unavailable = — здесь недоступен
output-preset = Пресет
output-preset-quality = Качество
output-preset-balanced-option = Сбалансированный
output-preset-performance = Производительность
output-rate-control = Контроль битрейта
output-rc-cqp = CQP (постоянное качество)
output-rc-cbr = CBR (постоянный битрейт)
output-rc-vbr = VBR (переменный битрейт)
output-cq = CQ (0–51, ниже = лучше)
output-bitrate = Битрейт (kbps)
output-keyframe = Интервал ключевых кадров (с)
output-audio-bitrate = Битрейт аудио (kbps / дорожка)
output-presets = Пресеты:

# --- SettingsStream.tsx ---
stream-title = Настройки — Трансляция
stream-target-enabled = Цель { $index } включена
stream-target = Цель { $index }
stream-remove = Удалить
stream-service = Сервис
stream-canvas = Холст
stream-canvas-main = Основной (программа)
stream-canvas-vertical = Вертикальный (9:16 — включите его в студии)
stream-ingest-srt = URL приёма SRT
stream-ingest-whip = URL эндпоинта WHIP
stream-ingest-url = URL приёма
stream-ingest-override = (переопределение — пусто = пресет сервиса)
stream-key-srt = streamid (необязательно — добавляется как ?streamid=…; считается секретом)
stream-key-whip = Bearer-токен (необязательно — отправляется как заголовок Authorization; секрет)
stream-key-custom = Ключ потока (с вашего сервера — считается секретом)
stream-key-service = Ключ потока (из вашей панели автора — считается секретом)
stream-key-aria = Ключ потока { $index }
stream-key-hide = Скрыть
stream-key-show = Показать
stream-encoder = Кодировщик (H.264 — то, что несут RTMP, SRT и WHIP)
stream-encoder-auto = Авто — лучший обнаруженный кодировщик H.264
stream-encoder-unavailable = (здесь недоступен)
stream-video-bitrate = Битрейт видео (kbps, CBR)
stream-audio-bitrate = Битрейт аудио (kbps)
stream-fps = FPS
stream-keyframe = Интервал ключевых кадров (с)
stream-audio-track = Аудиодорожка (1–6)
stream-output-width = Ширина вывода (0 = холст)
stream-output-height = Высота вывода (0 = холст)
stream-add-target = + Добавить цель
stream-go-live-note = «В эфир» публикует на каждую включённую цель одновременно, напрямую на каждую платформу. Цели с одинаковыми настройками кодировщика делят одно кодирование.
stream-auto-record = Начинать запись при выходе в эфир (запись всё равно останавливается независимо)
stream-ffmpeg-note-before = Проводные кодеки трансляции работают через помеченный компонент ffmpeg по запросу —
stream-ffmpeg-note-link = управляйте им здесь
stream-ffmpeg-note-after = . Локальная запись продолжается, что бы ни делала трансляция.
stream-cancel = Отмена
stream-save = Сохранить

# --- SettingsReplay.tsx ---
replay-title = Настройки — Буфер повтора
replay-length-15s = 15 с
replay-length-30s = 30 с
replay-length-1min = 1 мин
replay-length-2min = 2 мин
replay-length-5min = 5 мин
replay-quality-low = Низкое (3 Mbps)
replay-quality-standard = Стандартное (6 Mbps)
replay-quality-high = Высокое (12 Mbps)
replay-length-presets = Пресеты длины
replay-quality-presets = Пресеты качества
replay-length-seconds = Длина (секунды)
replay-video-bitrate = Битрейт видео (kbps)
replay-fps = FPS
replay-audio-track = Аудиодорожка (1–6)
replay-note = Пока включён, буфер выполняет собственное лёгкое кодирование в ограниченное дисковое кольцо — около { $mb } МБ при этих настройках. Сохранение сшивает кольцо без перекодирования и никогда не затрагивает трансляцию или запись. Изменения применяются при следующем включении.
replay-cancel = Отмена
replay-save = Сохранить

# --- SettingsRemote.tsx ---
remote-title = Настройки — Удалённое управление
remote-enable = Включить удалённый WebSocket API
remote-password = Пароль (обязателен — контроллеры аутентифицируются им)
remote-password-placeholder = пароль для ваших контроллеров
remote-password-hide = Скрыть
remote-password-show = Показать
remote-port = Порт
remote-allow-lan = Разрешить подключения по LAN (по умолчанию только эта машина)
remote-note = Выкл. = порт закрыт. Вкл. = защищённый паролем WebSocket на 127.0.0.1 (или в вашей LAN при согласии), который может переключать сцены, запускать переход, начинать/останавливать трансляцию и запись, сохранять повторы и задавать отключение звука/громкость — те же действия, что и в интерфейсе, не более. Он не может читать файлы. Относитесь к паролю как к любым учётным данным; предпочитайте «только эта машина», если только вы специально не управляете с другого устройства.
remote-password-required = Для включения удалённого API требуется пароль.
remote-cancel = Отмена
remote-save = Сохранить

# --- SettingsHotkeys.tsx ---
hotkeys-title = Настройки — Горячие клавиши
hotkeys-record = Начать / остановить запись
hotkeys-record-placeholder = напр. Ctrl+Shift+R
hotkeys-go-live = В эфир / Завершить трансляцию
hotkeys-go-live-placeholder = напр. Ctrl+Shift+L
hotkeys-transition = Переход студийного режима
hotkeys-transition-placeholder = напр. Ctrl+Shift+T или F13
hotkeys-save-replay = Сохранить повтор (последние N секунд)
hotkeys-save-replay-placeholder = напр. Ctrl+Shift+S
hotkeys-add-marker = Поставить метку главы (запись)
hotkeys-add-marker-placeholder = напр. Ctrl+Shift+K
hotkeys-note = Горячие клавиши глобальные — они срабатывают, пока сфокусированы другие приложения. Пусто = не назначено. Клавиши push-to-talk/mute микшера находятся в меню ⋯ каждой полоски. В Linux/Wayland глобальные горячие клавиши могут быть недоступны (ограничение компоновщика) — кнопки продолжают работать.
hotkeys-cancel = Отмена
hotkeys-save = Сохранить

# --- WorkspaceDialog.tsx ---
workspace-title = Профили и коллекции сцен
workspace-profiles = Профили
workspace-profiles-hint = Профиль — это ваши настройки: цель трансляции, вывод, горячие клавиши. Переключайте по шоу или по платформе.
workspace-collections = Коллекции сцен
workspace-collections-hint = Коллекция — это ваши сцены + источники. «Создать» дублирует текущую как отправную точку.
workspace-active = Активно
workspace-switch-to = Переключиться на { $name }
workspace-active-marker = ● активно
workspace-new-name-placeholder = новое имя…
workspace-new-name-label = Имя нового { $title }
workspace-create = Создать

# --- OBS import (CAP-M02) ---
workspace-import-obs = Импорт из OBS…
workspace-import-obs-hint = Загрузите коллекцию сцен OBS (её scenes.json). Текущая коллекция будет сохранена заранее.
workspace-import-busy = Импорт…
workspace-import-title = «{ $name }» импортировано
workspace-import-summary = сцен: { $scenes } · источников: { $sources } · элементов: { $items }
workspace-import-dismiss = Закрыть
workspace-import-clean = Всё импортировано без ошибок.
workspace-import-geometry-caveat = Размеры и позиции подгоняются из макета OBS — проверьте каждую сцену и заново выберите устройства захвата.
workspace-import-notes-title = Импортировано с замечаниями
workspace-import-skipped-title = Не импортировано
import-note-needsReselect = Заново выберите устройство/монитор/окно
import-note-gameCaptureAsWindow = Захват игры → Захват окна
import-note-referencesFile = Проверьте путь к файлу
import-note-filterDropped = Некоторые фильтры не поддерживаются
import-note-geometryApproximated = Позиция/размер приблизительны
import-skip-unsupportedKind = Нет подходящего типа источника
import-skip-group = Группы пока не поддерживаются

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Перепривязать отсутствующие файлы…
doctor-title = Отсутствующие файлы
doctor-scanning = Сканирование…
doctor-all-good = Все указанные файлы на месте. Перепривязывать нечего.
doctor-intro = { $count } указанных файлов не найдено на этом компьютере. Укажите новое расположение каждого — каждая сцена, где он используется, исправится сразу.
doctor-relinked = Перепривязано ссылок: { $count }.
doctor-uses = использован { $count }×
doctor-locate = Найти…
doctor-locate-folder = Искать в папке…
doctor-locate-folder-hint = Выберите папку; каждый отсутствующий файл находится по имени и перепривязывается.
doctor-kind-image = изображение
doctor-kind-media = медиа
doctor-kind-slideshow = слайд-шоу
doctor-kind-font = шрифт
doctor-kind-lut = LUT
doctor-kind-mask = маска
history-relinkFiles = Перепривязать файлы

# --- ScriptsDialog.tsx ---
scripts-title = Скрипты (Lua)
scripts-empty = Скриптов ещё нет — добавьте файл .lua. Смотрите scripts/sample.lua для API: реагируйте на события эфира/сцены/записи и управляйте теми же командами, что и удалённый API.
scripts-enable = Включить { $path }
scripts-remove = Удалить { $path }
scripts-path-label = Путь к скрипту
scripts-add = Добавить
scripts-note = Скрипты работают изолированно — без доступа к файлам или ОС; они могут вызывать только те же команды студии, что и удалённый API (переключать сцены, переход, запись/трансляция/повтор, отключение звука). Ошибка скрипта логируется и локализуется. Изменения применяются в течение секунды.
scripts-error-not-lua = Укажите файл .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Браузерные доки
browser-dock-empty = Доков ещё нет — добавьте всплывающий чат, страницу уведомлений или ваши веб-кнопки Companion.
browser-dock-open = Открыть
browser-dock-remove = Удалить { $name }
browser-dock-name-placeholder = имя (напр. чат Twitch)
browser-dock-name-label = Имя дока
browser-dock-url-label = URL дока
browser-dock-note = Док открывается как отдельное окно, которое можно разместить рядом со студией. Страница не получает доступа к приложению — она просто рендерится. Только URL http(s); доки открываются только по нажатию «Открыть».
browser-dock-error-name = Назовите док (напр. чат Twitch).
browser-dock-error-url = URL дока должен начинаться с http:// или https://.

# --- studio-preview-pane ---
studio-preview-label = Предпросмотр студийного режима
studio-preview-heading = Предпросмотр
studio-preview-hint = нажмите на сцену, чтобы загрузить её сюда
studio-preview-empty = Предпросмотр появится здесь.
studio-preview-mirrors = отражает программу
studio-preview-transition-select = Переход
studio-preview-duration = Длительность перехода (ms)
studio-preview-commit-title = Применить Предпросмотр → Программа через переход (зрители это увидят)
studio-preview-transitioning = Выполняется переход…
studio-preview-transition-button = Переход ⇄
studio-preview-luma-placeholder = изображение вытеснения в оттенках серого (png/jpg)
studio-preview-luma-label = Изображение вытеснения по яркости
studio-preview-browse = Обзор…
studio-preview-filter-images = Изображения
studio-preview-filter-video = Видео
studio-preview-stinger-placeholder = видео стингера (ProRes 4444 .mov сохраняет альфу)
studio-preview-stinger-label = Видеофайл стингера
studio-preview-stinger-cut-label = Точка склейки стингера (ms)
studio-preview-stinger-cut-title = Когда смена сцены происходит под стингером (ms от начала перехода)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Срез
transition-kind-fade = Угасание
transition-kind-slide-left = Слайд ←
transition-kind-slide-right = Слайд →
transition-kind-slide-up = Слайд ↑
transition-kind-slide-down = Слайд ↓
transition-kind-swipe-left = Смахивание ←
transition-kind-swipe-right = Смахивание →
transition-kind-luma-linear = Вытеснение по яркости (линейное)
transition-kind-luma-radial = Вытеснение по яркости (радиальное)
transition-kind-luma-horizontal = Вытеснение по яркости (горизонтальное)
transition-kind-luma-diamond = Вытеснение по яркости (ромбовидное)
transition-kind-luma-clock = Вытеснение по яркости (часовое)
transition-kind-image = Вытеснение изображением (своё)
transition-kind-stinger = Стингер (видео)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Пользовательский (RTMP/RTMPS)
stream-service-srt = SRT (собственный сервер)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = О программе
about-tagline = Записывайте и транслируйте как студия — без аккаунтов, без облака.
about-version = Версия
about-created-by = Автор
about-project-started = Проект начат
about-first-stable = Первый стабильный выпуск
about-first-stable-pending = Пока нет — 1.0.0 в разработке
about-platform = Платформа
about-local-first = Freally Capture работает полностью на вашей машине. Без аккаунтов, без телеметрии, без облака — ваш компьютер покидает лишь та трансляция, которую вы сами решили отправить.
about-website = Веб-сайт
about-issues = Сообщить о проблеме
about-license = Лицензия
about-eula = EULA
about-third-party = Уведомления о стороннем ПО
about-check-updates = Проверить обновления…

# --- unified settings modal (TASK-906) ---
settings-title = Настройки
settings-language-section = Язык
settings-language = Язык интерфейса
settings-language-system = Системный по умолчанию
settings-language-note = Выбранный здесь язык запоминается. «Системный по умолчанию» следует за вашей операционной системой. Непереведённый текст отображается на английском.
settings-appearance-section = Внешний вид
settings-theme = Тема
settings-theme-dark = Тёмная
settings-theme-light = Светлая
settings-theme-custom = Пользовательская
settings-accent = Акцент
settings-general-section = Общие
settings-show-stats-dock = Показать панель статистики
settings-more-section = Дополнительные настройки
settings-open-output = Запись…
settings-open-stream = Трансляция…
settings-open-replay = Повтор…
settings-open-hotkeys = Горячие клавиши…
settings-open-remote = Удалённый API…
settings-open-about = О программе…
controls-settings = ⚙ Настройки…
controls-settings-title = Язык, внешний вид и общие настройки приложения

# --- command palette (TASK-904) ---
palette-title = Палитра команд
palette-search = Поиск сцен, источников и действий
palette-placeholder = Поиск сцен, источников, действий…
palette-no-results = Ничего не соответствует “{ $query }”
palette-hint = ↑ ↓ для перемещения · Enter для запуска · Esc для закрытия
palette-group-scenes = Сцена
palette-group-sources = Источник
palette-group-actions = Действие
palette-transition = Переход: превью → программа
palette-save-replay = Сохранить повтор
palette-add-marker = Поставить метку главы
palette-vertical-canvas = Вертикальный холст (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Добро пожаловать в Freally Capture
wizard-welcome = Два коротких шага: посмотрим, на что способна ваша машина, а затем создадим сцену. Это займёт около тридцати секунд, и всё можно изменить позже.
wizard-local-first = Отсюда ничто не покидает ваш компьютер. У Freally Capture нет аккаунтов, нет телеметрии и нет облака.
wizard-start = Начать
wizard-skip = Пропустить
wizard-hardware-title = На что способна ваша машина
wizard-probing = Проверяем вашу видеокарту и процессор…
wizard-encoder = Кодировщик
wizard-canvas = Холст
wizard-bitrate = Битрейт
wizard-probe-found = Найдено: { $gpus } · { $cores } физических ядер
wizard-no-gpu = нет отдельного GPU
wizard-apply = Использовать эти настройки
wizard-keep-current = Оставить как есть
wizard-template-title = Начните со сцены
wizard-template-screen = Захватить мой экран
wizard-template-screen-note = Добавляет «Захват дисплея» вашего основного монитора. Самое привычное начало.
wizard-template-empty = Начать с пустого
wizard-template-empty-note = Пустая сцена. Источники добавите сами кнопкой «+».
wizard-done = Всё готово.
wizard-done-hint = Нажмите Ctrl+K в любой момент, чтобы искать сцены, источники и действия. Настройки — за кнопкой ⚙.
wizard-close = Начать трансляцию

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Ваша видеокарта умеет кодировать видео сама, оставляя процессор свободным для остальной студии.
autoconfig-reason-software = Подходящий аппаратный кодировщик не найден, поэтому кодировать будет процессор. Это работает, просто нагружает CPU сильнее.
autoconfig-reason-quality-hardware = 1080p при 60 кадрах в секунду и битрейте, который принимает любая крупная платформа.
autoconfig-reason-quality-software = 30 кадров в секунду, потому что программное кодирование при 60 теряет кадры на большинстве процессоров.
autoconfig-reason-quality-low-cores = Пониженный битрейт, потому что у этого процессора мало ядер, а программное кодирование будет бороться за них с компоновщиком.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Запись началась
announce-recording-paused = Запись приостановлена
announce-recording-stopped = Запись остановлена
announce-live-started = Вы в эфире
announce-live-ended = Трансляция завершена
announce-reconnecting = Соединение потеряно, переподключение
announce-stream-failed = Сбой трансляции
announce-frames-dropped = Пропущено кадров: { $count }

# CAP-M01 — undo/redo edit history
palette-undo = Отменить
palette-redo = Повторить
palette-edit-history = История изменений…
history-title = История изменений
history-empty = Пока нечего отменять.
history-current = Текущее состояние
history-close = Закрыть
history-addScene = Добавить сцену
history-renameScene = Переименовать сцену
history-removeScene = Удалить сцену
history-reorderScene = Изменить порядок сцен
history-addSource = Добавить источник
history-removeSource = Удалить источник
history-reorderSource = Изменить порядок источников
history-renameSource = Переименовать источник
history-transformSource = Переместить источник
history-toggleVisibility = Переключить видимость
history-toggleLock = Переключить блокировку
history-setBlendMode = Изменить режим наложения
history-editSourceProperties = Изменить свойства
history-applyLayout = Расставить макет
history-moveToSeat = Переместить на место
history-groupSources = Сгруппировать источники
history-ungroupSources = Разгруппировать источники
history-toggleGroupVisibility = Переключить группу
history-setSceneAudio = Звук сцены
history-setVerticalCanvas = Вертикальный холст
history-addFilter = Добавить фильтр
history-removeFilter = Удалить фильтр
history-reorderFilter = Изменить порядок фильтров
history-editFilter = Изменить фильтр
history-toggleFilter = Переключить фильтр
history-setVolume = Настроить громкость
history-toggleMute = Переключить звук
history-setMonitor = Изменить мониторинг
history-setTracks = Изменить дорожки
history-setSyncOffset = Настроить синхронизацию A/V
history-setAudioHotkeys = Аудиосочетания клавиш

# CAP-M04 — alignment aids
settings-alignment-section = Помощь в выравнивании
settings-smart-guides = Умные направляющие (привязка при перетаскивании)
settings-safe-areas = Наложения безопасной зоны
settings-rulers = Линейки
align-group = Выровнять по холсту
align-left = Выровнять по левому краю
align-hcenter = Центрировать по горизонтали
align-right = Выровнять по правому краю
align-top = Выровнять по верхнему краю
align-vcenter = Центрировать по вертикали
align-bottom = Выровнять по нижнему краю

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Выровнять и распределить выбранное
arrange-left = Выровнять левые края
arrange-hcenter = Центрировать по горизонтали
arrange-right = Выровнять правые края
arrange-top = Выровнять верхние края
arrange-vcenter = Центрировать по вертикали
arrange-bottom = Выровнять нижние края
distribute-h = Распределить по горизонтали
distribute-v = Распределить по вертикали
guides-group = Направляющие
guides-add-v = Добавить вертикальную направляющую
guides-add-h = Добавить горизонтальную направляющую
history-arrangeItems = Упорядочить элементы
history-editGuides = Изменить направляющие

# CAP-M05 — edit transform + copy/paste
transform-title = Изменить трансформацию — { $name }
transform-anchor = Опорная точка
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Поворот
transform-crop = Обрезка
transform-crop-left = Слева
transform-crop-top = Сверху
transform-crop-right = Справа
transform-crop-bottom = Снизу
transform-no-size = Размер и обрезка станут доступны, когда источник сообщит свои размеры.
transform-copy = Копировать трансформацию
transform-paste = Вставить трансформацию
transform-close = Закрыть
filters-copy = Копировать фильтры ({ $count })
filters-paste = Вставить фильтры ({ $count })
palette-edit-transform = Изменить трансформацию…
history-pasteFilters = Вставить фильтры

# CAP-M26 — keying workbench
workbench-title = Мастерская кеинга — { $name }
workbench-mode-keyed = С ключом
workbench-mode-source = Источник
workbench-mode-matte = Матта
workbench-mode-split = Разделение
workbench-eyedropper = Пипетка
workbench-eyedropper-hint = Щёлкните по источнику, чтобы взять цвет ключа.
workbench-loupe = Лупа
workbench-split = Разделение
workbench-preview-alt = Предпросмотр мастерской кеинга
workbench-tune = Настроить
workbench-close = Закрыть

# CAP-M06 — multiview monitor
multiview-title = Мультивью
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Нажмите на сцену, чтобы переключиться на неё.
multiview-hint-stage = Нажмите на сцену, чтобы поставить её в предпросмотр.
palette-multiview = Монитор мультивью

# CAP-M07 — projectors
projector-title = Открыть проектор
projector-source = Источник
projector-target-program = Программа
projector-target-preview = Предпросмотр
projector-target-scene = Сцена…
projector-target-source = Источник…
projector-target-multiview = Мультивью
projector-which-scene = Какая сцена
projector-which-source = Какой источник
projector-none = Нечего показать
projector-display = Дисплей
projector-windowed = Плавающее окно (этот экран)
projector-display-option = Дисплей { $n } — { $w }×{ $h }
projector-primary = (основной)
projector-open = Открыть
projector-cancel = Отмена
projector-exit-hint = Нажмите Esc для выхода
palette-projector = Открыть проектор…

# CAP-M08 — still-frame grab
palette-still = Сделать стоп-кадр…
still-saved-toast = Стоп-кадр сохранён: { $name }
still-failed-toast = Не удалось сделать стоп-кадр: { $error }
hotkeys-still = Сделать стоп-кадр
hotkeys-still-placeholder = напр. Ctrl+Shift+P

# CAP-M13 — source health dashboard
palette-source-health = Состояние источников…
palette-av-sync = Калибровка синхронизации A/V…
palette-hotkey-audit = Карта горячих клавиш…
health-title = Состояние источников
health-col-source = Источник
health-col-state = Состояние
health-col-resolution = Разрешение
health-col-fps = FPS
health-col-last-frame = Последний кадр
health-col-dropped = Пропущено
health-col-retries = Перезапуски
health-col-actions = Действия
health-state-live = В эфире
health-state-waiting = Ожидание
health-state-error = Ошибка
health-state-inactive = Неактивен
health-restart = Перезапустить
health-properties = Свойства
health-empty = В этой коллекции пока нет источников.
health-seconds = { $value } с

# CAP-M23 — quit guard + orderly shutdown
quit-title = Выйти из Freally Capture?
quit-body = При выходе сейчас будет безопасно выполнено по порядку:
quit-consequence-stream = Завершение эфира и отключение от сервиса.
quit-consequence-recording = Остановка записи и финализация её файлов.
quit-consequence-replay = Отключение буфера повтора — несохранённые кадры будут удалены.
quit-confirm = Выйти безопасно
quit-quitting = Завершение…
quit-cancel = Отмена

# CAP-M11 — crash-safe recording salvage
salvage-title = Восстановить прерванные записи?
salvage-body = Последний сеанс завершился неожиданно, пока эти записи ещё писались. Восстановление создаёт воспроизводимую копию рядом с оригиналом — исходный файл никогда не изменяется.
salvage-repair = Восстановить
salvage-repairing = Восстановление…
salvage-done = Восстановлено
salvage-repaired = Восстановлено → { $name }
salvage-failed = Сбой восстановления: { $error }
salvage-dismiss = Не сейчас

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Сбой кодировщика — переключено с { $from } на { $to }. Эфир переподключился и продолжается.
fallback-toast-recording = Сбой кодировщика — переключено с { $from } на { $to }. Запись продолжается в новом файле.
fallback-note = Резервный кодировщик: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Звук программы пропал
alarm-clipping = Звук программы клиппирует
alarm-black = Картинка программы чёрная
alarm-frozen = Картинка программы давно не меняется
alarm-lowDisk = Место на диске: осталось около { $minutes } мин при текущем битрейте
alarm-dismiss = Скрыть тревогу
alarm-cleared = Устранено: { $alarm }

# CAP-M22 — panic button
palette-panic = Паника — переключить на экран приватности
panic-banner-title = Паника
panic-banner-body = Программа показывает экран приватности; весь звук выключен, захваты остановлены. Эфир и запись продолжаются.
panic-restore = Восстановить…
panic-restore-confirm = Восстановить программу?
panic-restore-yes = Восстановить
panic-restore-cancel = Отмена
hotkeys-panic = Паника (экран приватности)
hotkeys-panic-placeholder = напр. Ctrl+Shift+F12
hotkeys-timer-toggle = Старт/пауза всех таймеров
hotkeys-timer-toggle-placeholder = напр. Ctrl+Shift+T
hotkeys-timer-reset = Сброс всех таймеров
hotkeys-timer-reset-placeholder = напр. Ctrl+Shift+0
panic-slate-color = Цвет экрана паники
panic-slate-image = Изображение экрана паники
panic-slate-image-placeholder = Необязательный путь к изображению

# CAP-M24 — redacted diagnostics bundle
diag-title = Диагностический пакет
diag-intro = Экспортирует очищенный .zip (снимок настроек, проба кодировщиков, свежая статистика — секреты, пути и имена никогда не включаются) для ручного прикрепления к issue на GitHub. Ничего никуда не отправляется.
diag-preview = Показать содержимое
diag-hide-preview = Скрыть просмотр
diag-export = Экспортировать .zip
diag-exported = Экспортировано: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Предэфирная проверка
preflight-intro = Каждый блокирующий пункт должен быть зелёным; остальное — честные подсказки.
preflight-item-targets = Цели настроены (ключ/URL)
preflight-item-encoder = Доступен рабочий кодировщик
preflight-item-sources = Все источники в порядке
preflight-item-disk = Место на диске для записи
preflight-item-mic = Уровень микрофона
preflight-item-desktopAudio = Уровень звука рабочего стола
preflight-item-replay = Буфер повтора включён
preflight-targets-detail = { $count } включено
preflight-sources-detail = { $count } источник(ов) с ошибкой
preflight-disk-detail = ~{ $minutes } мин при текущем битрейте
preflight-fix-stream = Настройки эфира…
preflight-fix-components = Компоненты…
preflight-fix-sources = Состояние источников…
preflight-fix-replay = Включить
preflight-optional = необязательно
preflight-hold = Не выходить в эфир, пока всё не зелёное
preflight-cancel = Отмена
preflight-go-anyway = Всё равно в эфир
preflight-go-live = В эфир
