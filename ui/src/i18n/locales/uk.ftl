# Freally Capture — uk
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Студійний режим
toggle-on = увімк.
toggle-off = вимк.
stats = Статистика
core-ok = ядро OK
hide-stats-dock = Приховати панель статистики
show-stats-dock = Показати панель статистики


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Не вдалося зберегти налаштування — зміна не збережеться після перезапуску.
studio-mode-leave = Вийти зі студійного режиму
studio-mode-enter-title = Студійний режим — редагуйте сцену в попередньому перегляді та виводьте її в програму через перехід
vertical-canvas-title = Друге (вертикальне 9:16) вихідне полотно — записується та транслюється незалежно
app-version = v{ $version }
core-error = ядро ПОМИЛКА
core-unreachable = ядро недоступне (режим браузера)
connecting-to-core = підключення до ядра…
filters-source-fallback = Джерело

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Попередній перегляд програми
preview-program-output = Вивід програми
preview-canvas-editor = Редактор полотна
preview-px-to-edge-label = Пікселів до країв кадру
preview-px-to-edge = пікс. до краю L { $left } · T { $top } · R { $right } · B { $bottom }
preview-program-heading = Програма
preview-no-gpu = Не знайдено придатного GPU-адаптера — композитор не може працювати на цій машині.
preview-starting-compositor = Запуск композитора…
preview-empty-scene = Ця сцена порожня — додайте джерело в розділі «Джерела», а потім перетягуйте, масштабуйте й обертайте його прямо тут, на полотні.
preview-fps = { $fps } fps
preview-dropped = { $dropped } втрачено

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Отримано запрошення
remote-join-with-webcam = Приєднатися з вебкамерою
remote-dismiss = Відхилити
remote-hosting-guest = Приймаєте віддаленого гостя
remote-you-are-guest = Ви — віддалений гість
remote-share-view-title = Поділіться екраном із застосунком гостя (він бачить ваш вигляд у реальному часі)
remote-stop-sharing-view = Припинити показ вигляду
remote-share-my-view = Показати мій вигляд
remote-allow-center-title = Дозволити гостю перемикати, який вигляд у центрі (ви лишаєтесь керувати й можете повернути будь-коли)
remote-guest-switching = Перемикання гостем:
remote-stop-screen = Зупинити екран
remote-share-screen = Показати екран
remote-share-screen-title-guest = Поділіться екраном із ведучим (він стане джерелом, яке можна вивести в центр)
remote-center-request-label = Запит на вигляд у центрі
remote-center = У центр
remote-center-cam-title = Попросити ведучого вивести вашу камеру в центр
remote-center-my-cam = Моя камера
remote-center-screen-title = Попросити ведучого вивести ваш екран у центр
remote-center-my-screen = Мій екран
remote-center-host-title = Повернути центр вигляду ведучого
remote-center-host-view = Вигляд ведучого
remote-end-session = Завершити сеанс
remote-leave = Вийти
remote-host-view-heading = Вигляд ведучого
remote-host-shared-view-label = Спільний вигляд ведучого
remote-guest-position-label = Позиція гостя
remote-guest-label = Гість
remote-put-guest = Розмістити гостя { $position }
remote-remove-title = Видалити гостя — він зможе приєднатися знову за тим самим посиланням
remote-remove = Видалити
remote-ban-title = Заблокувати гостя — блокує його та робить запрошення недійсним
remote-ban = Заблокувати
remote-guest-self-muted = гість заглушив себе
remote-unmute-guest = Увімкнути звук гостя
remote-mute-guest = Заглушити гостя
remote-muted-by-host = Заглушено ведучим
remote-unmute-mic = Увімкнути мікрофон
remote-mute-mic = Заглушити мікрофон
remote-waiting-for-host = очікування ведучого


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = джерело
sources-fallback-video = відео
sources-fallback-error = помилка
sources-kind-unknown = ?
sources-missing-source = (джерело відсутнє)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Дисплей
sources-badge-window = Вікно
sources-badge-portal = Portal
sources-badge-camera = Камера
sources-badge-image = Зображення
sources-badge-media = Медіа
sources-badge-guest = Гість
sources-badge-color = Колір
sources-badge-text = Текст
sources-badge-scene = Сцена
sources-badge-slides = Слайди
sources-badge-chat = Чат
sources-badge-audio-in = Аудіовхід
sources-badge-audio-out = Аудіовихід
sources-badge-app-audio = Звук застосунку

# Add-source menu items
sources-add-display = Захоплення екрана
sources-add-window = Захоплення вікна
sources-add-game = Захоплення гри (спершу прочитайте)
sources-add-webcam = Пристрій захоплення відео
sources-add-image = Зображення
sources-add-media = Медіа (відео/зображення)
sources-add-remote-guest = Віддалений гість (P2P-прототип)
sources-add-color = Колір
sources-add-text = Текст
sources-add-nested-scene = Вкладена сцена
sources-add-slideshow = Слайдшоу зображень
sources-add-chat-overlay = Оверлей живого чату
sources-add-audio-input = Захоплення аудіовходу
sources-add-audio-output = Захоплення аудіовиходу
sources-add-app-audio = Звук застосунку (Windows)
sources-add-existing = Наявне джерело…

# Panel header + toolbar buttons
sources-panel-title = Джерела
sources-group-title = Згрупувати джерела — виберіть два чи більше елементи, потім «Створити групу»; згруповані елементи рухаються та ховаються/показуються разом
sources-group-aria = Згрупувати джерела
sources-arrange = Розташувати: екран + кути
sources-add-source = Додати джерело
sources-browser-source-note = Джерело браузера постачається як окремий компонент, що завантажується за потреби (рушій Chromium ~180 МБ — ніколи не включається в комплект). Наразі: захопіть справжнє вікно браузера через «Захоплення вікна» + хромакей/кольоровий ключ або відкрийте чат/сповіщення як док (Керування → Доки).

# Empty state
sources-empty = У цій сцені немає джерел — додайте «Захоплення екрана», «Вікно», «Вебкамеру», «Зображення», «Колір» чи «Текст» кнопкою «+». Перетягуйте, масштабуйте й обертайте їх на полотні; кнопки праворуч змінюють порядок у стеку.

# Per-row controls
sources-already-in-group = Уже в { $name }
sources-pick-for-new-group = Вибрати для нової групи
sources-pick-item-for-group = Вибрати { $name } для нової групи
sources-hide = Приховати
sources-show = Показати
sources-hide-item = Приховати { $name }
sources-show-item = Показати { $name }
sources-unfocus-title = Прибрати фокус — відновити компонування
sources-focus-title = Фокус — заповнити полотно (виділити доповідача)
sources-unfocus-item = Прибрати фокус з { $name }
sources-focus-item = Сфокусувати { $name }
sources-center-title = У центр — зробити це спільним центральним виглядом (камери переходять на панель)
sources-center-item = Вивести { $name } в центр
sources-rename-item = Перейменувати { $name }
sources-in-group = У групі { $name }

# Row status + retry
sources-retry-error = Повторити — { $message }
sources-retry-item = Повторити { $name }
sources-status-error = статус: помилка
sources-open-privacy-title = Відкрити налаштування конфіденційності macOS для цього дозволу
sources-open-privacy-item = Відкрити налаштування конфіденційності для { $name }
sources-privacy-settings-button = налаштування
sources-status-starting = запуск…
sources-status-live = наживо
sources-status-aria = статус: { $state }

# Media row pause/resume
sources-media-resume-title = Відновити відео (наживо в трансляції)
sources-media-pause-title = Призупинити відео — утримати кадр + без звуку, наживо в трансляції
sources-media-resume-item = Відновити { $name }
sources-media-pause-item = Призупинити { $name }

# Hover controls
sources-unlock = Розблокувати
sources-lock = Заблокувати
sources-unlock-item = Розблокувати { $name }
sources-lock-item = Заблокувати { $name }
sources-raise-title = Підняти у стеку
sources-raise-item = Підняти { $name }
sources-lower-title = Опустити у стеку
sources-lower-item = Опустити { $name }
sources-filters-title = Фільтри та змішування
sources-filters-item = Фільтри для { $name }
sources-properties-title = Властивості
sources-properties-item = Властивості { $name }
sources-remove-title = Видалити з цієї сцени
sources-remove-item = Видалити { $name }

# Grouping footer
sources-create-group = Створити групу ({ $count })
sources-cancel = Скасувати

# Groups list
sources-groups-aria = Групи джерел
sources-hide-group = Приховати групу
sources-show-group = Показати групу
sources-item-count = · { $count } елементів
sources-ungroup-title = Розгрупувати — елементи лишаються на місці
sources-ungroup-item = Розгрупувати { $name }

# Live Chat Overlay picker
sources-chat-title = Додати оверлей живого чату
sources-chat-youtube-label = YouTube — URL каналу, watch або live_chat (без ключа, без входу)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  or a watch?v= URL
sources-chat-twitch-label = Twitch — назва каналу (анонімне читання, без облікового запису)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — слаг каналу (публічна кінцева точка, за можливості)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Повідомлення з'являються з міткою часу h:mm:ss AM/PM на прозорому фоні (за замовчуванням угорі праворуч; перетягніть куди завгодно). Потік чату лише витісняє старі рядки — він ніколи не зупинить трансляцію чи запис. Чат Facebook потребує вашого власного токена Graph і поки не реалізований — він ніколи не потрібен і не блокує платформи вище.
sources-chat-add = Додати оверлей чату
sources-chat-default-name = Живий чат

# Image Slideshow picker
sources-slideshow-title = Додати слайдшоу зображень
sources-slideshow-empty = Ще немає зображень — «Огляд» додає їх по порядку.
sources-slideshow-remove-slide = Видалити слайд { $number }
sources-slideshow-browse = Огляд зображень…
sources-slideshow-per-slide-label = На слайд (мс)
sources-slideshow-crossfade-label = Перехресне згасання (мс, 0 = різко)
sources-slideshow-loop-label = Цикл (вимк. = утримати останній слайд)
sources-slideshow-shuffle-label = Перемішувати щоцикл
sources-slideshow-note = Перехресне згасання змішує зображення однакового розміру; різні розміри різко переключаються на межі (без тихого масштабування).
sources-slideshow-add = Додати слайдшоу ({ $count })

# Nested Scene picker
sources-nested-title = Додати вкладену сцену
sources-nested-empty = Немає іншої сцени для вкладення — спершу додайте другу сцену.
sources-nested-scene-name = Сцена: { $name }
sources-nested-note = Вкладена сцена відображається наживо в розмірі полотна програми та слідує власним правкам; трансформації, фільтри та змішування застосовуються до неї, як до будь-якого джерела. Її аудіоджерела приєднуються до мікса, поки сцена, що її показує, є програмою.

# Display / Window capture picker
sources-capture-display-title = Додати захоплення екрана
sources-capture-window-title = Додати захоплення вікна
sources-capture-looking = Пошук джерел…
sources-capture-none-displays = Нічого захоплювати — дисплеїв не знайдено.
sources-capture-none-windows = Нічого захоплювати — вікон не знайдено.
sources-capture-portal-note = У Wayland екран чи вікно вибирає системний діалог — застосунки не можуть захоплювати глобально, тож це чесний (і єдиний) шлях.
sources-capture-window-note = Попередні перегляди оновлюються наживо. Згорнуте вікно показує свій останній кадр (або нічого), доки ви його не розгорнете.
sources-thumb-no-preview = немає перегляду
sources-thumb-loading = завантаження…

# Video Capture Device picker
sources-webcam-title = Додати пристрій захоплення відео
sources-webcam-looking = Пошук камер…
sources-webcam-none = Камер чи карт захоплення не знайдено.
sources-webcam-format-label = Формат
sources-webcam-format-auto-loading = Авто (завантаження форматів…)
sources-webcam-format-auto = Авто (найвища роздільність)
sources-webcam-card-presets-label = Пресети карти:
sources-webcam-preset-title = Виберіть режим { $label }, який пропонує ця карта
sources-webcam-add = Додати камеру

# Audio Input / Output capture picker
sources-audio-output-title = Додати захоплення аудіовиходу
sources-audio-input-title = Додати захоплення аудіовходу
sources-audio-default-output = Вихід за замовчуванням (те, що ви чуєте)
sources-audio-default-input = Вхід за замовчуванням
sources-audio-looking = Пошук аудіопристроїв…
sources-audio-none-output = Пристрою захоплення звуку робочого столу не знайдено.
sources-audio-none-input = Мікрофонів чи лінійних входів не знайдено.
sources-audio-input-note = Смуги мікшера отримують VU-індикатор, фейдер, вимкнення звуку, моніторинг, фільтри (шумозаглушення, гейт, компресор…) і призначення доріжок. Усе лишається на цій машині.

# Application Audio picker
sources-appaudio-title = Додати звук застосунку
sources-appaudio-looking = Пошук застосунків, що відтворюють звук…
sources-appaudio-none = Зараз жоден застосунок не відтворює звук — увімкніть відтворення в застосунку, потім оновіть.
sources-appaudio-refresh = ⟳ Оновити
sources-appaudio-note = Захоплює звук саме цього застосунку — власний VU, фейдер, вимкнення, фільтри та доріжку.

# Game Capture picker
sources-game-title = Захоплення гри
sources-game-checking = Перевірка…
sources-game-use-portal = Використати захоплення екрана (Portal)
sources-game-use-window = Використати натомість захоплення вікна

# Image picker
sources-image-title = Додати зображення
sources-image-file-label = Файл зображення (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Додати зображення

# Path field
sources-browse = Огляд…

# Media picker
sources-media-title = Додати медіа
sources-media-file-label = Медіафайл (mp4, mkv, webm, mov, .frec або зображення)
sources-media-loop-label = Цикл (перезапуск спочатку в кінці)
sources-media-note = .frec відтворюється через власний кодек freally-video — нічого завантажувати. Транспортні формати (mp4/mkv/webm/…) декодуються через компонент FFmpeg, що завантажується за потреби; їхній звук потрапляє в мікшер як окрема смуга.
sources-media-add = Додати медіа

# Invite expiry options
sources-ttl-15min = 15 хв
sources-ttl-30min = 30 хв
sources-ttl-1hour = 1 година
sources-ttl-1day = 1 день

# Remote Guest form
sources-remote-copy-failed = не вдалося скопіювати — виділіть посилання та скопіюйте вручну
sources-remote-join-failed = не вдалося приєднатися: { $error }
sources-remote-title = Віддалений гість (P2P-прототип)
sources-remote-host-heading = Ведучий — запросити гостя
sources-remote-start-hosting = Почати приймати
sources-remote-expires-label = Термін дії
sources-remote-invite-expiry-aria = Термін дії запрошення
sources-remote-invite-link-aria = Посилання запрошення
sources-remote-copied = Скопійовано ✓
sources-remote-copy = Копіювати
sources-remote-share-note = Поділіться цим посиланням (Discord / повідомлення / email). Воно містить ваш сеанс і має заданий термін дії. Гість відкриває його та приєднується зі своєю вебкамерою.
sources-remote-qr-note = Скануйте телефоном, щоб приєднатися прямо з браузера — камера + мікрофон, без встановлення. Посилання freally:// вище відкривається у Freally Capture на машині, де він встановлений.
sources-remote-guest-heading = Гість — приєднатися за запрошенням
sources-remote-paste-placeholder = вставте посилання запрошення
sources-remote-invite-input-aria = Посилання запрошення або ідентифікатор сеансу
sources-remote-join = Приєднатися з вебкамерою
sources-remote-session-note = Керування живим сеансом (вимкнення звуку, завершення) лишається на панелі вгорі головного вікна — це вікно можна закрити.
sources-remote-stop-session = Зупинити сеанс

# Invite QR
sources-invite-qr-aria = QR-код посилання запрошення

# Remote device pickers
sources-devices-output-unavailable = маршрутизація виходу недоступна — відтворення на пристрої за замовчуванням
sources-devices-mic-test-failed = тест мікрофона не вдався: { $error }
sources-devices-heading = Аудіопристрої сеансу
sources-devices-microphone-label = Мікрофон
sources-devices-microphone-aria = Мікрофон сеансу
sources-devices-system-default = Системний за замовчуванням
sources-devices-output-label = Вихід
sources-devices-output-aria = Аудіовихід сеансу
sources-devices-stop-test = Зупинити тест
sources-devices-test = Тест — почуйте себе
sources-devices-testing-note = говоріть у мікрофон — ви чуєте вибрані пристрої наживо
sources-devices-idle-note = зациклює ваш мікрофон на вихід (навушники уникають зворотного зв'язку)

# TURN relay section
sources-turn-save-failed = не вдалося зберегти: { $error }
sources-turn-summary = Мережа — необов'язковий TURN-ретранслятор (розширене)
sources-turn-note-1 = Сеанси з'єднуються напряму (P2P) — безкоштовно, ретранслятор не потрібен. Якщо ОБИДВА боки за суворими NAT, прямий шлях може не спрацювати; тоді медіа несе TURN-ретранслятор, який ви запускаєте самі. Це можна пропустити — більшість з'єднань працюють лише напряму.
sources-turn-note-2 = Безкоштовний варіант: Oracle Cloud «Always Free» запускає coturn безкоштовно (зверніть увагу: Oracle просить кредитну картку під час реєстрації, але конфігурація Always-Free лишається безкоштовною). Кроки: 1) створіть безкоштовну ВМ, 2) встановіть coturn, 3) відкрийте UDP 3478, 4) задайте користувача/пароль, 5) введіть тут turn:your-vm-ip:3478 + облікові дані. Ваші облікові дані лишаються в локальному файлі налаштувань і ніколи не логуються.
sources-turn-url-label = TURN URL
sources-turn-url-placeholder = turn:host:3478 (порожньо = лише напряму)
sources-turn-url-aria = TURN URL
sources-turn-username-label = Ім'я користувача
sources-turn-username-aria = Ім'я користувача TURN
sources-turn-credential-label = Обліковий ключ
sources-turn-credential-aria = Обліковий ключ TURN
sources-turn-note-3 = Ретранслятор задіюється, щойно заповнені всі три поля (TURN-серверу потрібні облікові дані), і застосовується до наступного сеансу, який ви почнете чи до якого приєднаєтесь. Перевірте його тестовим викликом лише через ретранслятор між двома вашими машинами.
sources-turn-settings-unavailable = налаштування недоступні (режим браузера)

# Color picker
sources-color-title = Додати колір
sources-color-label = Колір
sources-color-width-label = Ширина
sources-color-height-label = Висота
sources-color-add = Додати колір

# Text picker
sources-text-title = Додати текст
sources-text-label = Текст
sources-text-default = Текст
sources-text-color-label = Колір
sources-text-color-aria = Колір тексту
sources-text-size-label = Розмір (px)
sources-text-note = Сімейство шрифтів, вирівнювання, перенесення та RTL — у властивостях джерела. Вбудований Noto Sans (зокрема арабська/іврит) за замовчуванням — однаковий на кожній машині.
sources-text-add = Додати текст

# Existing source picker
sources-existing-title = Додати наявне джерело
sources-existing-empty = Джерел ще немає — спершу додайте хоч одне в будь-яку сцену. Наявні джерела спільні: перейменування чи переналаштування одного оновлює кожну сцену, що його показує.

# Screen + corners layout
sources-slot-off = Вимк.
sources-slot-center = Центр (екран)
sources-slot-top-left = Угорі ліворуч
sources-slot-top-right = Угорі праворуч
sources-slot-bottom-left = Внизу ліворуч
sources-slot-bottom-right = Внизу праворуч
sources-layout-title = Розташувати: екран + кути
sources-layout-empty = Спершу додайте в цю сцену захоплення екрана та одну чи більше камер, потім розташуйте їх тут.
sources-layout-note = Розмістіть екран у центрі та до чотирьох камер по кутах — компонування для пояснень / подкасту. У кожному куті — вебкамера, захоплене вікно дзвінка чи медіакліп. Після цього будь-яке з них можна перетягнути на полотні.
sources-layout-slot-aria = Слот для { $name }
sources-layout-apply = Застосувати компонування


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Керування
controls-start-stop-title-stop = Зупинити та завершити запис
controls-start-stop-title-start = Записати сигнал програми з конфігурацією «Налаштування → Вивід»
controls-finalizing = ◌ Завершення…
controls-stop-recording = ■ Зупинити запис
controls-start-recording = ● Почати запис
controls-marker-title = Поставити маркер розділу в цей момент — він потрапляє в ЗАПИС (розділи mkv або допоміжний файл). Маркери трансляції на боці платформи потребують облікових записів платформ, яких цей застосунок ніколи не запитує.
controls-marker = ◈ Маркер
controls-pause-title-resume = Відновити — файл продовжується як єдина суцільна шкала часу
controls-pause-title-pause = Призупинити — кадри не записуються; відновлення продовжує той самий відтворюваний файл
controls-resume-recording = ▶ Відновити запис
controls-pause-recording = ⏸ Призупинити запис
controls-reactions-label = Реакції (вбудовані в програму)
controls-reactions-title = Запустити реакцію поверх програми — записується І транслюється, тож повтор показує точний момент. Глядачі в чаті теж їх викликають (їхні емодзі-реакції спливають автоматично); потік лише обмежує кількість на екрані.
controls-react = Реакція { $emoji }
controls-virtual-camera-title = Віртуальна камера потребує власного підписаного драйвера для кожної ОС (Win11 MFCreateVirtualCamera / Win10 DirectShow / розширення macOS CoreMediaIO / Linux v4l2loopback) — постачається як окремий етап. Модель сигналу для неї готова: програма, вертикальне полотно чи одне джерело, з парним віртуальним мікрофоном на Windows/Linux (у macOS немає API віртуального мікрофона — чесно кажучи).
controls-virtual-camera = ⌁ Запустити віртуальну камеру
controls-files-title = Завершені записи + дія перепакування в mp4
controls-files = ▤ Файли…
controls-output-title = Формат запису, кодувальник, папка, доріжки та розбиття
controls-output = ⚙ Вивід…
controls-stream-title = Ціль трансляції: сервіс, ключ трансляції, кодувальник, бітрейт
controls-stream = ⦿ Трансляція…
controls-codecs-title = Компонент транспортних кодеків ffmpeg, що завантажується за потреби (чітко позначений, ніколи не в комплекті)
controls-codecs = ⬡ Кодеки…
controls-replay-title = Довжина буфера повтору + пресети якості
controls-replay = ⟲ Повтор…
controls-keys-title = Глобальні гарячі клавіші: запис, трансляція, перехід, збереження повтору
controls-keys = ⌨ Клавіші…
controls-scripts-title = Ізольовані Lua-скрипти: реагують на події трансляції/сцени/запису, керують студією
controls-scripts = ⚡ Скрипти…
controls-docks-title = Браузерні доки: відкрийте спливний чат, сторінку сповіщень чи кнопки Companion як вікно поруч зі студією
controls-docks = ⧉ Доки…
controls-remote-title = Віддалений WebSocket API для контролерів Stream Deck / Companion (вимкнено за замовчуванням)
controls-remote = ⌁ Віддалене керування…
controls-profiles-title = Профілі (налаштування) + колекції сцен — перемикні знімки
controls-profiles = ▣ Профілі…
controls-bug-title = Повідомити про ваду — анонімно, за згодою (нічого не надсилається автоматично)
controls-bug = 🐞 Повідомити про ваду…
controls-updates-title = Перевірити оновлення — підписані, перевірені, нічого не завантажується без кліку
controls-updates = ⭳ Перевірити оновлення…
controls-saved = Збережено: { $path }

# --- MixerDock.tsx ---
mixer-title = Аудіомікшер
mixer-monitor-error = моніторинг: { $error }
mixer-switch-to-horizontal = Перемкнути на горизонтальні смуги
mixer-switch-to-vertical = Перемкнути на вертикальні смуги
mixer-layout-aria-vertical = Компонування мікшера: вертикальне — перемкнути на горизонтальне
mixer-layout-aria-horizontal = Компонування мікшера: горизонтальне — перемкнути на вертикальне
mixer-empty = У цій сцені немає аудіоджерел — додайте «Захоплення аудіовходу» (мікрофон) чи «Захоплення аудіовиходу» (звук робочого столу) кнопкою «+» у «Джерелах». Смуги отримують VU-індикатор, фейдер, вимкнення звуку, моніторинг, фільтри та призначення доріжок.
mixer-advanced-title = Аудіо — { $name }
mixer-loudness-label = Гучність програми (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Миттєва гучність (400 мс)
mixer-short-term-title = Короткочасна гучність (3 с)
mixer-lufs-short = S { $value }
mixer-monitor-label = Моніторинг
mixer-monitor-device-aria = Пристрій виводу моніторингу
mixer-default-output = Вихід за замовчуванням

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Пам'ять
stats-dropped = Втрачено
stats-render = Рендер
stats-gpu = GPU
stats-gpu-compositing = композитинг
stats-gpu-idle = простій
stats-vertical-fps = 9:16 FPS
stats-targets-label = Цілі трансляції
stats-shared-encode = · спільне кодування
stats-starting = Запуск композитора…

# --- ScenesRail.tsx ---
scenes-title = Сцени
scenes-new-scene-name = Сцена
scenes-add = Додати сцену
scenes-empty = Підключення до ядра студії…
scenes-rename = Перейменувати { $name }
scenes-on-program = У програмі
scenes-preview = Перегляд { $name }
scenes-switch-to = Перемкнути на { $name }
scenes-move-up = Перемістити вгору
scenes-move-up-aria = Перемістити { $name } вгору
scenes-move-down = Перемістити вниз
scenes-move-down-aria = Перемістити { $name } вниз
scenes-last-stays = Остання сцена лишається
scenes-remove = Видалити цю сцену
scenes-remove-aria = Видалити { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Рівень
channelstrip-monitor-off = Моніторинг вимкнено
channelstrip-monitor-only = Лише моніторинг (не в міксі)
channelstrip-monitor-and-output = Моніторинг і вивід
channelstrip-status-error = помилка
channelstrip-status-live = наживо
channelstrip-status-waiting-audio = очікування звуку
channelstrip-status = статус: { $state }
channelstrip-status-waiting = очікування
channelstrip-mute = Заглушити
channelstrip-unmute = Увімкнути звук
channelstrip-mute-source = Заглушити { $name }
channelstrip-unmute-source = Увімкнути звук { $name }
channelstrip-scene-mix-on = Мікс для сцени УВІМК. — ця смуга перевизначає глобальний мікс для цієї сцени (клацніть, щоб знову слідувати глобальному міксу)
channelstrip-scene-mix-off = Мікс для сцени — надати цій смузі власний фейдер/вимкнення для поточної сцени
channelstrip-scene-mix-label = Мікс для сцени для { $name }
channelstrip-monitor-cycle = { $mode } — клацніть для перемикання
channelstrip-monitor-mode = Режим моніторингу { $name }: { $mode }
channelstrip-audio-filters-title = Аудіофільтри (шумозаглушення, гейт, компресор…)
channelstrip-audio-filters-label = Аудіофільтри для { $name }
channelstrip-advanced-title = Зсув синхронізації та гарячі клавіші push-to-talk
channelstrip-advanced-label = Розширені налаштування звуку для { $name }
channelstrip-track-assignment = Призначення доріжок
channelstrip-track = Доріжка { $n }
channelstrip-track-assigned = Доріжка { $n } (призначено)
channelstrip-track-label = Доріжка { $n } для { $name }
channelstrip-device-error = помилка пристрою
channelstrip-audio-device-error = помилка аудіопристрою
channelstrip-volume-label = Гучність { $name } у децибелах
channelstrip-ptt-hold = Push-to-talk: утримуйте { $key }
channelstrip-sync-offset = Зсув синхронізації (мс, 0–{ $max } — затримує цей звук)
channelstrip-ptt-hotkey = Гаряча клавіша push-to-talk (тиша, доки не утримується)
channelstrip-ptt-placeholder = напр. Ctrl+Shift+T або F13
channelstrip-ptt-aria = Гаряча клавіша push-to-talk
channelstrip-ptm-hotkey = Гаряча клавіша push-to-mute (тиша, поки утримується)
channelstrip-ptm-placeholder = напр. Ctrl+Shift+M
channelstrip-ptm-aria = Гаряча клавіша push-to-mute
channelstrip-hotkeys-note = Гарячі клавіші працюють, коли активні інші застосунки. У Linux/Wayland глобальні гарячі клавіші можуть бути недоступні — це обмеження композитора, чесно кажучи.
channelstrip-apply = Застосувати

# --- LiveButton.tsx ---
livebutton-failure-ended = трансляцію завершено
livebutton-title-live = Завершити трансляцію — усі цілі (активний запис продовжується)
livebutton-title-offline = Вийти в ефір на всі увімкнені цілі «Налаштування → Трансляція»
livebutton-end-stream = ■ Завершити трансляцію
livebutton-aria-reconnecting = Повторне підключення
livebutton-aria-live = Наживо
livebutton-badge-retry = спроба { $n }
livebutton-badge-live = наживо
livebutton-go-live = ⦿ В ефір

# --- RecDot.tsx ---
recdot-paused-aria = Запис призупинено
recdot-recording-aria = Запис
recdot-tracks-one = запис { $count } аудіодоріжки
recdot-tracks-other = запис { $count } аудіодоріжок
recdot-paused = призупинено

# --- ReplayControls.tsx ---
replaycontrols-saved = Повтор збережено — { $name }
replaycontrols-failure-stopped = буфер зупинено
replaycontrols-title-disarm = Вимкнути буфер повтору (відкидає незбережену історію)
replaycontrols-title-arm = Увімкнути ковзний буфер повтору — тримає останні N секунд готовими до збереження (власне легке кодування; трансляція та запис не зачіпаються)
replaycontrols-replay-seconds = ⟲ Повтор { $seconds }с
replaycontrols-arm = ⟲ Увімкнути буфер повтору
replaycontrols-save-title = Зберегти останні N секунд у папку записів (також на гарячій клавіші «Зберегти повтор»)
replaycontrols-save = ⤓ Зберегти

# --- PropertiesDialog.tsx ---
properties-title = Властивості — { $name }
properties-name = Назва
properties-cancel = Скасувати
properties-apply = Застосувати
properties-youtube = YouTube — URL каналу / watch / live_chat (без ключа, без входу, ніколи)
properties-twitch = Twitch — назва каналу (анонімно)
properties-kick = Kick — слаг каналу (публічна кінцева точка)
properties-width-px = Ширина (px)
properties-lines = Рядки
properties-font-px = Шрифт (px)
properties-images = Файли зображень (один шлях на рядок, показуються по порядку)
properties-per-slide = На слайд (мс)
properties-crossfade = Перехресне згасання (мс, 0 = різко)
properties-loop-slideshow = Цикл (вимк. = утримати останній слайд)
properties-shuffle = Перемішувати щоцикл
properties-nested-scene = Сцена, яку компонує це джерело (сцена, що вже містить цю, відхиляється)
properties-portal-note = Portal Wayland ScreenCast вибирає екран чи вікно в системному діалозі щоразу, коли це джерело запускається — тут нема чого налаштовувати, за задумом.
properties-appaudio-capturing = Захоплення звуку з { $exe }
properties-appaudio-exe-fallback = застосунок
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Додайте джерело заново, щоб націлитись на інший застосунок (ідентифікатор процесу змінюється при перезапуску застосунку).
properties-image-file = Файл зображення
properties-media-file = Медіафайл (mp4, mkv, webm, mov, .frec або зображення)
properties-media-loop = Цикл (перезапуск спочатку в кінці)
properties-media-hwdecode = Апаратне декодування (самостійно переходить на програмне)
properties-media-note = .frec відтворюється через власний кодек freally-video — нічого завантажувати. Інші відеоформати декодуються через компонент FFmpeg, що завантажується за потреби. Звук файлу отримує власну смугу мікшера; зсув синхронізації смуги точно налаштовує A/V-вирівнювання. Кліп без звуку лишає свою смугу беззвучною.
properties-color = Колір
properties-width = Ширина
properties-height = Висота
properties-text = Текст
properties-font-family = Сімейство шрифтів (системне; порожньо = за замовчуванням)
properties-size-px = Розмір (px)
properties-text-color = Колір тексту
properties-align = Вирівнювання
properties-align-left = ліворуч
properties-align-center = по центру
properties-align-right = праворуч
properties-line-spacing = Міжрядковий інтервал
properties-wrap-width = Ширина перенесення (px; 0 = вимк.)
properties-force-rtl = Примусово справа наліво
properties-text-note = Рендеринг використовує справжнє формування (арабське з'єднання, лігатури) та двонапрямний порядок рядків. Вбудоване сімейство Noto Sans (зокрема арабська/іврит) за замовчуванням; системні сімейства теж працюють. CJK наразі використовує системні шрифти.
properties-repick-capturing = Захоплення: { $label }
properties-repick-looking = Пошук джерел…
properties-repick-none-displays = Не знайдено дисплеїв для повторного вибору.
properties-repick-none-windows = Не знайдено вікон для повторного вибору.
properties-repick-again = Вибрати знову:
properties-device = Пристрій
properties-video-current-device = (поточний пристрій)
properties-format = Формат
properties-format-auto-loading = Авто (завантаження форматів…)
properties-format-auto = Авто (найвища роздільність)
properties-audio-capture-of = Захопити звук
properties-audio-default-output = Вихід за замовчуванням (те, що ви чуєте)
properties-audio-default-input = Вхід за замовчуванням
properties-audio-default-suffix = (за замовчуванням)
properties-audio-current-device = (поточний пристрій: { $id })

# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Підсилення
audiofilters-name-noise-gate = Шумовий гейт
audiofilters-name-compressor = Компресор
audiofilters-name-limiter = Лімітер
audiofilters-name-eq = 3-смуговий еквалайзер
audiofilters-name-denoise = Шумозаглушення
audiofilters-name-ducking = Дакінг
audiofilters-title = Аудіофільтри — { $name }
audiofilters-chain-header = Ланцюг фільтрів (верхній працює першим, до фейдера)
audiofilters-add = + Додати фільтр
audiofilters-add-menu = Додати аудіофільтр
audiofilters-empty = Ще немає фільтрів — придушіть шум мікрофона (класичний DSP, без ML), закрийте гейтом кімнату, приборкайте піки компресором чи приглушіть музику під ваш голос.
audiofilters-enable = Увімкнути { $name }
audiofilters-run-earlier = Виконати раніше
audiofilters-move-up = Перемістити { $name } вгору
audiofilters-run-later = Виконати пізніше
audiofilters-move-down = Перемістити { $name } вниз
audiofilters-remove-title = Видалити фільтр
audiofilters-remove = Видалити { $name }
audiofilters-gain-db = Підсилення (dB)
audiofilters-open-db = Відкриття на (dB)
audiofilters-close-db = Закриття на (dB)
audiofilters-attack-ms = Атака (мс)
audiofilters-hold-ms = Утримання (мс)
audiofilters-release-ms = Відпускання (мс)
audiofilters-ratio = Співвідношення (:1)
audiofilters-threshold-db = Поріг (dB)
audiofilters-output-gain-db = Вихідне підсилення (dB)
audiofilters-ceiling-db = Стеля (dB)
audiofilters-low-db = Низькі (dB)
audiofilters-mid-db = Середні (dB)
audiofilters-high-db = Високі (dB)
audiofilters-strength = Сила
audiofilters-denoise-note = Власне спектральне придушення на класичному DSP — постійний шум (вентилятори, шипіння) зникає, а мовлення проходить. Без ML, без моделей, згідно зі статутом.
audiofilters-duck-under = Приглушувати під
audiofilters-ducking-trigger = Джерело-тригер дакінгу
audiofilters-pick-trigger = (виберіть тригер — напр. ваш мікрофон)
audiofilters-trigger-at-db = Спрацювання на (dB)
audiofilters-duck-by-db = Приглушувати на (dB)

# --- FiltersDialog.tsx ---
filters-name-chroma-key = Хромакей
filters-name-color-key = Кольоровий ключ
filters-name-luma-key = Яскравісний ключ
filters-name-render-delay = Затримка рендеру
filters-name-color-correction = Корекція кольору
filters-name-lut = Застосувати LUT
filters-name-blur = Розмиття
filters-name-mask = Маска зображення
filters-name-sharpen = Різкість
filters-name-scroll = Прокручування
filters-name-crop = Обрізка
filters-title = Фільтри — { $name }
filters-blend-mode = Режим змішування
filters-chain-header = Ланцюг фільтрів (верхній працює першим)
filters-add = + Додати фільтр
filters-add-menu = Додати фільтр
filters-empty = Ще немає фільтрів — застосуйте хромакей до вебкамери, скоригуйте колір захоплення чи прокрутіть рядок.
filters-enable = Увімкнути { $name }
filters-run-earlier = Виконати раніше
filters-move-up = Перемістити { $name } вгору
filters-run-later = Виконати пізніше
filters-move-down = Перемістити { $name } вниз
filters-remove-title = Видалити фільтр
filters-remove = Видалити { $name }
filters-key-color-rgb = Ключовий колір (будь-який колір, відстань RGB)
filters-similarity = Схожість
filters-smoothness = Плавність
filters-luma-min = Мін. яскравість (темніше вирізається)
filters-luma-max = Макс. яскравість (світліше вирізається)
filters-delay = Затримка (мс — лише відео, напр. для синхронізації зі звуком; максимум 500)
filters-key-color = Ключовий колір
filters-spill = Розтікання
filters-gamma = Гамма
filters-brightness = Яскравість
filters-contrast = Контраст
filters-saturation = Насиченість
filters-hue-shift = Зсув відтінку
filters-opacity = Непрозорість
filters-cube-file = Файл .cube
filters-amount = Величина
filters-radius = Радіус
filters-mask-image = Зображення маски
filters-mask-mode = Режим
filters-mask-alpha = альфа
filters-mask-luma = яскравість
filters-mask-invert = інвертувати
filters-speed-x = Швидкість X (px/с)
filters-speed-y = Швидкість Y (px/с)
filters-crop-left = ліворуч
filters-crop-top = згори
filters-crop-right = праворуч
filters-crop-bottom = знизу
filters-crop-aria = обрізка { $side }

# --- PickerShell.tsx ---
pickershell-refresh-aria = Оновити
pickershell-refresh-title = Оновити список
pickershell-close = Закрити


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Повідомити про ваду
bugreport-intro = Звіти анонімні та за згодою — нічого не надсилається автоматично. Ви переглянете точний текст нижче, потім надішлете його через попередньо заповнену issue на GitHub чи ваш поштовий застосунок. Без особистих даних (ваш домашній шлях та ім'я користувача приховані); без облікового запису, без сервера.
bugreport-crash-notice = Freally Capture несподівано закрився під час попереднього запуску — анонімні деталі збою наведено нижче. Повідомлення про них допомагає швидко це виправити.
bugreport-description-label = Що ви робили, коли це сталося? (необов'язково)
bugreport-description-placeholder = напр. попередній перегляд завис, коли я додав другу вебкамеру
bugreport-include-crash = Включити анонімні деталі збою з останнього запуску
bugreport-preview-label = Що саме буде надіслано
bugreport-open-github = Відкрити issue на GitHub
bugreport-gmail-title = Відкриває вікно створення листа Gmail у вашому браузері, попередньо заповнене. Не увійшли? Google спершу покаже екран входу.
bugreport-compose-gmail = Створити в Gmail
bugreport-email-title = Відкриває чернетку в поштовому застосунку, який цей ПК використовує за замовчуванням (Outlook, Thunderbird, Mail…)
bugreport-send-email = Надіслати email
bugreport-copied = Скопійовано ✓
bugreport-copy-report = Копіювати звіт
bugreport-dismiss-crash = Відхилити збій
bugreport-copy-failed = не вдалося скопіювати — виділіть текст і скопіюйте вручну
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = ЩО СТАЛОСЯ
bugreport-preview-no-description = (опис не надано)
bugreport-preview-diagnostics = АНОНІМНА ДІАГНОСТИКА (без особистих даних)
bugreport-preview-from = Від: Freally Capture
bugreport-preview-crash-excerpt = --- фрагмент збою ---

# --- Updates.tsx ---
updates-title = Оновлення програми
updates-checking = Перевірка оновлень…
updates-uptodate = У вас найновіша версія.
updates-check-again = Перевірити знову
updates-available = Доступна версія { $version }
updates-current-version = (у вас { $current })
updates-release-notes-label = Версія { $version } — примітки до випуску
updates-confirm = Оновити зараз? Завантаження перевіряється за вбудованим ключем підпису перед застосуванням. Freally Capture закривається, запускається інсталятор, і нова версія відкривається сама.
updates-yes-update-now = Так, оновити зараз
updates-no-not-now = Ні, не зараз
updates-downloading = Завантаження { $version }…
updates-starting = запуск…
updates-installed = Оновлення встановлено.
updates-restart-now = Перезапустити зараз
updates-restart-later = Перезапустити пізніше
updates-try-again = Спробувати знову

# --- Models.tsx ---
models-title = Компоненти
models-ffmpeg-heading = FFmpeg — транспортні кодеки
models-badge-third-party = Стороннє · не в комплекті
models-ffmpeg-desc = Власний рушій Freally Capture записує без втрат freally-video (.frec) без нічого додаткового. Запис транспортних форматів, яких очікують платформи та плеєри — H.264/AAC (і HEVC/AV1) у mp4/mkv/mov/webm — використовує FFmpeg, окремий інструмент, з яким цей застосунок ніколи не постачається: ці кодеки обтяжені патентами, тож він лишається необов'язковим і чітко позначеним. Він завантажується за потреби із закріпленої збірки нижче, перевіряється SHA-256 перед першим використанням, кешується для кожного користувача та керується як окремий процес. Його ліцензія (LGPL/GPL) — власна; див. THIRD-PARTY-NOTICES.
models-checking = Перевірка…
models-ffmpeg-not-installed = Не встановлено. Доступно: FFmpeg { $version } із { $source } (завантаження { $size }).
models-ffmpeg-none-pinned = Для цієї платформи ще не закріплено збірку FFmpeg — запис у транспортні кодеки тут недоступний. Запис без втрат freally-video це не зачіпає.
models-ffmpeg-download-verify = Завантажити й перевірити ({ $size })
models-downloading = Завантаження…
models-download-of = з
models-cancel = Скасувати
models-ffmpeg-verifying = Перевірка завантаження за закріпленим SHA-256…
models-ffmpeg-extracting = Розпакування…
models-ffmpeg-ready = Встановлено й перевірено — { $version }
models-remove = Видалити
models-ffmpeg-retry = Повторити завантаження
models-network-note = Завантаження — єдина мережева дія на цій панелі, і воно ніколи не починається саме. Невдала контрольна сума перериває встановлення — застосунок відмовляється виконувати байти, за які не може поручитися.
models-cef-heading = Середовище джерела браузера — Chromium (CEF)
models-cef-desc = Джерела браузера відображають вебсторінки (сповіщення, віджети, оверлеї) через Chromium Embedded Framework — середовище ~100 МБ, з яким цей застосунок ніколи не постачається. Воно завантажується за потреби з офіційного індексу збірок CEF, перевіряється за SHA-1 цього індексу перед розпакуванням і кешується для кожного користувача. Джерело браузера, що відображається через нього, з'явиться окремим етапом; це встановлює потрібне йому середовище.
models-cef-download-install = Завантажити й встановити
models-cef-unsupported = CEF не публікує збірку для цієї платформи — джерела браузера тут недоступні.
models-cef-resolving = Визначення найновішої стабільної збірки…
models-cef-verifying = Перевірка завантаження за SHA-1 індексу…
models-cef-extracting = Розпакування середовища…
models-cef-ready = Встановлено — CEF { $version }.
models-cef-retry = Повторити
models-integrations-heading = Необов'язкові інтеграції
models-badge-never-bundled = Ніколи не в комплекті
models-ndi-detected = Виявлено
models-ndi-not-installed = Не встановлено
models-vst-available = Доступно
models-vst-not-available = Недоступно

# --- Recordings.tsx ---
recordings-title = Записи
recordings-loading = Читання папки…
recordings-empty = Записів ще немає — «Почати запис» пише в папку, задану у «Виводі».
recordings-frec-label = власний без втрат (freally-video)
recordings-remux-title = Перепакувати в mp4 — копіювання потоку, без перекодування, без зміни якості (потрібен компонент FFmpeg)
recordings-remuxing = Перепакування…
recordings-remux-to-mp4 = Перепакувати в MP4
recordings-export-mp4-title = Декодувати власний .frec і перекодувати в MP4 (H.264/AAC), щоб відтворювався в будь-якому плеєрі — потрібен компонент FFmpeg
recordings-exporting = Експорт…
recordings-export-mp4 = Експорт → MP4
recordings-export-mkv-title = Декодувати власний .frec і перекодувати в MKV, щоб відтворювався в будь-якому плеєрі
recordings-starting = запуск…
recordings-frames = { $done } / { $total } кадрів
recordings-cancel = Скасувати
recordings-export-cancelled = Експорт скасовано.
recordings-exported-to = Експортовано до { $path }
recordings-remuxed-to = Перепаковано до { $path }

# --- OpenedFrec.tsx ---
openfrec-title = Відкрити запис .frec
openfrec-desc = Freally Capture записує власний формат без втрат .frec — але не відтворює його. Freally Player відтворюватиме .frec напряму після випуску. Наразі експортуйте його в MP4/MKV, і він відтворюватиметься в будь-якому плеєрі (VLC, плеєр вашої ОС, будь-що).
openfrec-exported-to = Експортовано до { $path }
openfrec-exporting = Експорт…
openfrec-starting = запуск…
openfrec-export-mp4 = Експорт → MP4
openfrec-export-mkv = Експорт → MKV

# --- VerticalCanvasDialog.tsx ---
vertical-title = Вертикальне полотно (9:16)
vertical-enable = Увімкнути друге полотно — записується та транслюється незалежно від програми
vertical-scene-label = Сцена, яку компонує це полотно
vertical-width = Ширина
vertical-height = Висота
vertical-preview-alt = Попередній перегляд вертикального полотна
vertical-note = Позиції елементів піксельно точні на всіх полотнах: виберіть цю сцену на панелі «Сцени», щоб розташувати її, поки цей перегляд показує вертикальний результат. Цілі трансляції вибирають це полотно в ⦿ Трансляція…; «Налаштування → Вивід» може записувати його поряд з основним файлом.
vertical-close = Закрити

# --- EulaGate.tsx ---
eula-title = Freally Capture — Ліцензійна угода
eula-version = v{ $version }
eula-intro = Будь ласка, прочитайте та прийміть цю угоду, щоб користуватися Freally Capture. Коротко: це нейтральний інструмент, і ви несете повну відповідальність за те, що захоплюєте, записуєте та транслюєте — і за наявність прав на це.
eula-thanks = Дякуємо за прочитання.
eula-scroll-hint = Прокрутіть до кінця, щоб продовжити.
eula-decline = Відхилити й вийти
eula-agree = Я погоджуюсь


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Налаштування — Вивід
output-loading = Налаштування ще завантажуються…
output-container-frec = freally-video (.frec) — без втрат, власний, нічого завантажувати
output-container-mkv = MKV — стійкий до збоїв; пізніше перепакуйте в mp4
output-container-mp4 = MP4 — відтворюється всюди
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Без втрат
output-preset-lossless-title = Власний кодек freally-video — біт-у-біт, без завантаження
output-preset-high-label = Висока якість
output-preset-high-title = MP4, найкращий виявлений кодувальник, майже без втрат CQ 16, пресет «Якість»
output-preset-balanced-label = Збалансовано
output-preset-balanced-title = MKV, найкращий виявлений кодувальник, CQ 23, пресет «Збалансовано»
output-recording-format = Формат запису
output-ffmpeg-warning = Цей формат потребує компонента FFmpeg (транспортні кодеки — не в комплекті). Без втрат .frec не потребує нічого.
output-install = Встановити…
output-recordings-folder = Папка записів
output-folder-placeholder = Папка «Відео» ОС
output-filename-prefix = Префікс імені файлу
output-frame-rate = Частота кадрів
output-fps-option = { $fps } fps
output-split-every = Розбивати кожні (хвилин, 0 = вимк.)
output-output-width = Ширина виводу (0 = полотно; лише транспортні формати)
output-output-height = Висота виводу (0 = полотно)
output-record-vertical = Також записувати вертикальне полотно (паралельний файл «… (vertical)»; потрібне ввімкнене полотно 9:16)
output-audio-tracks = Аудіодоріжки
output-recorded-tracks-group = Записувані доріжки
output-track-last-one = Принаймні одна доріжка має записуватися
output-record-track-on = Запис доріжки { $index }: увімк.
output-record-track-off = Запис доріжки { $index }: вимк.
output-encoder-heading = Кодувальник
output-video-encoder = Відеокодувальник
output-encoder-auto = Авто — найкращий виявлений (H.264)
output-encoder-unavailable = — недоступний тут
output-preset = Пресет
output-preset-quality = Якість
output-preset-balanced-option = Збалансовано
output-preset-performance = Продуктивність
output-rate-control = Керування бітрейтом
output-rc-cqp = CQP (постійна якість)
output-rc-cbr = CBR (постійний бітрейт)
output-rc-vbr = VBR (змінний бітрейт)
output-cq = CQ (0–51, менше = краще)
output-bitrate = Бітрейт (kbps)
output-keyframe = Інтервал ключових кадрів (с)
output-audio-bitrate = Аудіобітрейт (kbps / доріжка)
output-presets = Пресети:

# --- SettingsStream.tsx ---
stream-title = Налаштування — Трансляція
stream-target-enabled = Ціль { $index } увімкнено
stream-target = Ціль { $index }
stream-remove = Видалити
stream-service = Сервіс
stream-canvas = Полотно
stream-canvas-main = Основне (програма)
stream-canvas-vertical = Вертикальне (9:16 — увімкніть у студії)
stream-ingest-srt = URL прийому SRT
stream-ingest-whip = URL кінцевої точки WHIP
stream-ingest-url = URL прийому
stream-ingest-override = (перевизначення — порожньо = пресет сервісу)
stream-key-srt = streamid (необов'язково — додається як ?streamid=…; вважається секретом)
stream-key-whip = Токен Bearer (необов'язково — надсилається як заголовок Authorization; секрет)
stream-key-custom = Ключ трансляції (з вашого сервера — вважається секретом)
stream-key-service = Ключ трансляції (з вашої панелі автора — вважається секретом)
stream-key-aria = Ключ трансляції { $index }
stream-key-hide = Приховати
stream-key-show = Показати
stream-encoder = Кодувальник (H.264 — те, що несуть RTMP, SRT і WHIP)
stream-encoder-auto = Авто — найкращий виявлений кодувальник H.264
stream-encoder-unavailable = (недоступний тут)
stream-video-bitrate = Відеобітрейт (kbps, CBR)
stream-audio-bitrate = Аудіобітрейт (kbps)
stream-fps = FPS
stream-keyframe = Інтервал ключових кадрів (с)
stream-audio-track = Аудіодоріжка (1–6)
stream-output-width = Ширина виводу (0 = полотно)
stream-output-height = Висота виводу (0 = полотно)
stream-add-target = + Додати ціль
stream-go-live-note = «В ефір» публікує на всі увімкнені цілі одразу, напряму до кожної платформи. Цілі з однаковими налаштуваннями кодувальника ділять єдине кодування.
stream-auto-record = Починати запис при виході в ефір (запис усе одно зупиняється незалежно)
stream-ffmpeg-note-before = Транспортні кодеки трансляції працюють через позначений компонент ffmpeg, що завантажується за потреби —
stream-ffmpeg-note-link = керуйте ним тут
stream-ffmpeg-note-after = . Локальний запис продовжується незалежно від того, що відбувається з трансляцією.
stream-cancel = Скасувати
stream-save = Зберегти

# --- SettingsReplay.tsx ---
replay-title = Налаштування — Буфер повтору
replay-length-15s = 15 с
replay-length-30s = 30 с
replay-length-1min = 1 хв
replay-length-2min = 2 хв
replay-length-5min = 5 хв
replay-quality-low = Низька (3 Mbps)
replay-quality-standard = Стандартна (6 Mbps)
replay-quality-high = Висока (12 Mbps)
replay-length-presets = Пресети довжини
replay-quality-presets = Пресети якості
replay-length-seconds = Довжина (секунди)
replay-video-bitrate = Відеобітрейт (kbps)
replay-fps = FPS
replay-audio-track = Аудіодоріжка (1–6)
replay-note = Коли увімкнено, буфер виконує власне легке кодування в обмежене кільце на диску — близько { $mb } МБ за цих налаштувань. Збереження зшиває кільце без перекодування й ніколи не зачіпає трансляцію чи запис. Зміни застосовуються під час наступного ввімкнення.
replay-cancel = Скасувати
replay-save = Зберегти

# --- SettingsRemote.tsx ---
remote-title = Налаштування — Віддалене керування
remote-enable = Увімкнути віддалений WebSocket API
remote-password = Пароль (обов'язковий — контролери проходять з ним автентифікацію)
remote-password-placeholder = пароль для ваших контролерів
remote-password-hide = Приховати
remote-password-show = Показати
remote-port = Порт
remote-allow-lan = Дозволити з'єднання по LAN (за замовчуванням лише ця машина)
remote-note = Вимк. = порт закрито. Увімк. = захищений паролем WebSocket на 127.0.0.1 (або вашій LAN за згодою), який може перемикати сцени, запускати перехід, починати/зупиняти трансляцію та запис, зберігати повтори й задавати вимкнення/гучність — ті самі дії, що й інтерфейс, і не більше. Він не може читати файли. Ставтеся до пароля як до будь-яких облікових даних; надавайте перевагу лише-цій-машині, якщо ви не керуєте з іншого пристрою навмисно.
remote-password-required = Для ввімкнення віддаленого API потрібен пароль.
remote-cancel = Скасувати
remote-save = Зберегти

# --- SettingsHotkeys.tsx ---
hotkeys-title = Налаштування — Гарячі клавіші
hotkeys-record = Почати / зупинити запис
hotkeys-record-placeholder = напр. Ctrl+Shift+R
hotkeys-go-live = В ефір / Завершити трансляцію
hotkeys-go-live-placeholder = напр. Ctrl+Shift+L
hotkeys-transition = Перехід студійного режиму
hotkeys-transition-placeholder = напр. Ctrl+Shift+T або F13
hotkeys-save-replay = Зберегти повтор (останні N секунд)
hotkeys-save-replay-placeholder = напр. Ctrl+Shift+S
hotkeys-add-marker = Поставити маркер розділу (запис)
hotkeys-add-marker-placeholder = напр. Ctrl+Shift+K
hotkeys-note = Гарячі клавіші глобальні — вони спрацьовують, коли активні інші застосунки. Порожньо = не призначено. Клавіші push-to-talk/вимкнення мікшера — у меню ⋯ кожної смуги. У Linux/Wayland глобальні гарячі клавіші можуть бути недоступні (обмеження композитора) — кнопки продовжують працювати.
hotkeys-cancel = Скасувати
hotkeys-save = Зберегти

# --- WorkspaceDialog.tsx ---
workspace-title = Профілі та колекції сцен
workspace-profiles = Профілі
workspace-profiles-hint = Профіль — це ваші налаштування: ціль трансляції, вивід, гарячі клавіші. Перемикайте для кожного шоу чи платформи.
workspace-collections = Колекції сцен
workspace-collections-hint = Колекція — це ваші сцени + джерела. «Створити» дублює поточну як відправну точку.
workspace-active = Активна
workspace-switch-to = Перемкнути на { $name }
workspace-active-marker = ● активна
workspace-new-name-placeholder = нова назва…
workspace-new-name-label = Нова назва { $title }
workspace-create = Створити

# --- ScriptsDialog.tsx ---
scripts-title = Скрипти (Lua)
scripts-empty = Ще немає скриптів — додайте файл .lua. Див. scripts/sample.lua щодо API: реагуйте на події трансляції/сцени/запису та керуйте тими самими командами, що й віддалений API.
scripts-enable = Увімкнути { $path }
scripts-remove = Видалити { $path }
scripts-path-label = Шлях до скрипта
scripts-add = Додати
scripts-note = Скрипти виконуються ізольовано — без доступу до файлів чи ОС; вони можуть викликати лише ті самі команди студії, що й віддалений API (перемикання сцен, перехід, запис/трансляція/повтор, вимкнення звуку). Помилка скрипта логується та стримується. Зміни застосовуються протягом секунди.
scripts-error-not-lua = Вкажіть файл .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Браузерні доки
browser-dock-empty = Ще немає доків — додайте спливний чат, сторінку сповіщень чи вебкнопки Companion.
browser-dock-open = Відкрити
browser-dock-remove = Видалити { $name }
browser-dock-name-placeholder = назва (напр. Twitch Chat)
browser-dock-name-label = Назва дока
browser-dock-url-label = URL дока
browser-dock-note = Док відкривається як власне вікно, яке можна розмістити поруч зі студією. Сторінка не отримує доступу до застосунку — вона просто відображається. Лише URL http(s); доки відкриваються лише коли ви клацаєте «Відкрити».
browser-dock-error-name = Назвіть док (напр. Twitch Chat).
browser-dock-error-url = URL дока має починатися з http:// або https://.

# --- studio-preview-pane ---
studio-preview-label = Попередній перегляд студійного режиму
studio-preview-heading = Попередній перегляд
studio-preview-hint = клацніть сцену, щоб завантажити її сюди
studio-preview-empty = Тут з'явиться попередній перегляд.
studio-preview-mirrors = віддзеркалює програму
studio-preview-transition-select = Перехід
studio-preview-duration = Тривалість переходу (ms)
studio-preview-commit-title = Застосувати «Попередній перегляд» → «Програма» через перехід (глядачі це бачать)
studio-preview-transitioning = Виконується перехід…
studio-preview-transition-button = Перехід ⇄
studio-preview-luma-placeholder = зображення витирання у відтінках сірого (png/jpg)
studio-preview-luma-label = Зображення витирання за яскравістю
studio-preview-browse = Огляд…
studio-preview-filter-images = Зображення
studio-preview-filter-video = Відео
studio-preview-stinger-placeholder = відео стінгера (ProRes 4444 .mov зберігає альфа-канал)
studio-preview-stinger-label = Відеофайл стінгера
studio-preview-stinger-cut-label = Точка зрізу стінгера (ms)
studio-preview-stinger-cut-title = Коли зміна сцени відбувається під стінгером (ms від початку переходу)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Зріз
transition-kind-fade = Згасання
transition-kind-slide-left = Ковзання ←
transition-kind-slide-right = Ковзання →
transition-kind-slide-up = Ковзання ↑
transition-kind-slide-down = Ковзання ↓
transition-kind-swipe-left = Змах ←
transition-kind-swipe-right = Змах →
transition-kind-luma-linear = Витирання за яскравістю (лінійне)
transition-kind-luma-radial = Витирання за яскравістю (радіальне)
transition-kind-luma-horizontal = Витирання за яскравістю (горизонтальне)
transition-kind-luma-diamond = Витирання за яскравістю (ромб)
transition-kind-luma-clock = Витирання за яскравістю (годинникове)
transition-kind-image = Витирання зображенням (власне)
transition-kind-stinger = Стінгер (відео)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Власний (RTMP/RTMPS)
stream-service-srt = SRT (власний хостинг)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Про застосунок
about-tagline = Записуйте та транслюйте як у студії — без облікових записів, без хмари.
about-version = Версія
about-created-by = Автор
about-project-started = Проєкт розпочато
about-first-stable = Перший стабільний випуск
about-first-stable-pending = Ще ні — 1.0.0 у розробці
about-platform = Платформа
about-local-first = Freally Capture працює повністю на вашій машині. Без облікових записів, без телеметрії, без хмари — з вашого комп'ютера виходить лише та трансляція, яку ви обрали надіслати.
about-website = Вебсайт
about-issues = Повідомити про проблему
about-license = Ліцензія
about-eula = EULA
about-third-party = Повідомлення третіх сторін
about-check-updates = Перевірити оновлення…

# --- unified settings modal (TASK-906) ---
settings-title = Налаштування
settings-language-section = Мова
settings-language = Мова інтерфейсу
settings-language-system = Системна за замовчуванням
settings-language-note = Вибрана тут мова запам'ятовується. «Системна за замовчуванням» слідує за вашою операційною системою. Неперекладений текст повертається до англійської.
settings-appearance-section = Вигляд
settings-theme = Тема
settings-theme-dark = Темна
settings-theme-light = Світла
settings-theme-custom = Власна
settings-accent = Акцент
settings-general-section = Загальні
settings-show-stats-dock = Показати панель статистики
settings-more-section = Додаткові налаштування
settings-open-output = Запис…
settings-open-stream = Трансляція…
settings-open-replay = Повтор…
settings-open-hotkeys = Гарячі клавіші…
settings-open-remote = Віддалений API…
settings-open-about = Про застосунок…
controls-settings = ⚙ Налаштування…
controls-settings-title = Мова, вигляд та загальні налаштування застосунку

# --- command palette (TASK-904) ---
palette-title = Палітра команд
palette-search = Пошук сцен, джерел і дій
palette-placeholder = Пошук сцен, джерел, дій…
palette-no-results = Нічого не відповідає “{ $query }”
palette-hint = ↑ ↓ для переміщення · Enter для запуску · Esc для закриття
palette-group-scenes = Сцена
palette-group-sources = Джерело
palette-group-actions = Дія
palette-transition = Перехід «Попередній перегляд» → «Програма»
palette-save-replay = Зберегти повтор
palette-add-marker = Поставити маркер розділу
palette-vertical-canvas = Вертикальне полотно (9:16)…
