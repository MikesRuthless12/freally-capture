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
sources-badge-test-bars = Смуги
sources-badge-test-grid = Сітка
sources-badge-test-sweep = Розгортка
sources-badge-test-tone = Тон
sources-badge-test-sync = Синхро
sources-badge-timer = Таймер

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
sources-add-timer = Таймер / Годинник
sources-add-nested-scene = Вкладена сцена
sources-add-slideshow = Слайдшоу зображень
sources-add-chat-overlay = Оверлей живого чату
sources-add-test-signal = Тестовий сигнал
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
sources-testsignal-title = Додати тестовий сигнал
sources-testsignal-pattern-label = Шаблон
sources-testsignal-bars = Кольорові смуги SMPTE
sources-testsignal-grid = Калібрувальна сітка
sources-testsignal-sweep = Розгортка руху
sources-testsignal-tone = Тон 1 кГц (−20 dBFS)
sources-testsignal-flash-beep = Спалах + сигнал синхронізації A/V
sources-testsignal-note = Перевіряйте сцени, кодувальники, проєктори та цілі трансляції без під'єднаної камери. Шаблон «спалах + сигнал» живить стенд синхронізації A/V.
sources-testsignal-add = Додати тестовий сигнал
sources-timer-title = Додати таймер
sources-timer-mode-label = Режим
sources-timer-wall-clock = Настінний годинник
sources-timer-countdown = Зворотний відлік
sources-timer-stopwatch = Секундомір
sources-timer-since-live = Час від початку етеру
sources-timer-since-recording = Час від початку запису
sources-timer-note = Тривалість, формат, оформлення та дії після відліку — у властивостях джерела.
sources-timer-add = Додати таймер

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
controls-iso-lanes = ISO-доріжки, що записуються разом із програмою: { $count }
controls-pause-title-resume = Відновити — файл продовжується як єдина суцільна шкала часу
controls-pause-title-pause = Призупинити — кадри не записуються; відновлення продовжує той самий відтворюваний файл
controls-resume-recording = ▶ Відновити запис
controls-pause-recording = ⏸ Призупинити запис
controls-reactions-label = Реакції (вбудовані в програму)
controls-reactions-title = Запустити реакцію поверх програми — записується І транслюється, тож повтор показує точний момент. Глядачі в чаті теж їх викликають (їхні емодзі-реакції спливають автоматично); потік лише обмежує кількість на екрані.
controls-react = Реакція { $emoji }
controls-virtual-camera-title = Віртуальна камера потребує власного підписаного драйвера для кожної ОС (Win11 MFCreateVirtualCamera / Win10 DirectShow / розширення macOS CoreMediaIO / Linux v4l2loopback) — постачається як окремий етап. Модель сигналу для неї готова: програма, вертикальне полотно чи одне джерело, з парним віртуальним мікрофоном на Windows/Linux (у macOS немає API віртуального мікрофона — чесно кажучи).
controls-virtual-camera = ⌁ Запустити віртуальну камеру
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
mixer-routing = Маршрутизація
mixer-routing-title = Маршрутизація аудіовиходу

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Маршрутизація звуку
routing-intro = Призначте смуги на шини доріжок, потім надішліть будь-яку шину на фізичний вихід — сигнал на апаратний рекордер, колонки в іншій кімнаті або контроль у навушниках на вільній доріжці. Монітор зберігає власний пристрій; ці маршрути додаються згори, тож якщо нічого не задано, мікс лишається незмінним.
routing-sends-title = Посили на доріжки
routing-no-strips = У цій сцені немає джерел звуку.
routing-source = Джерело
routing-track = Доріжка { $n }
routing-send-aria = Надіслати { $source } на доріжку { $n }
routing-outputs-title = Фізичні виходи
routing-master = Майстер
routing-off = Вимк.
routing-default-output = Вихід за замовчуванням
routing-device-aria = Пристрій виводу для { $bus }
routing-trim-aria = Триммер виходу для { $bus }
routing-trim-db = { $db } dB
routing-muted = Заглушено
routing-device-error = Пристрій недоступний

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Дакінг
mixer-ducking-title = Матриця дакінгу
ducking-title = Матриця дакінгу
ducking-intro = Будь-яке джерело може приглушувати будь-які інші. Комірка знижує ціль (стовпець), щойно заговорить тригер (рядок) — виберіть комірку, щоб задати її глибину, поріг і таймінг. Кожна пара — це окремий дакінг, тож один канал можуть приглушувати кілька тригерів одночасно.
ducking-need-two = Додайте принаймні два аудіоджерела, щоб приглушувати їх одне одним.
ducking-trigger-target = Тригер ↓ / Ціль →
ducking-cell-aria = { $trigger } приглушує { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Видалити
ducking-amount = Величина
ducking-threshold = Поріг
ducking-attack = Атака
ducking-release = Відпускання
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Нормалізація гучності
loudness-intro = Плавно веде програму до цільової гучності з обмеженням піків, щоб трансляція та записи виходили на сталий рівень. Повільно й м'яко — скеровує, але ніколи не пампить.
loudness-enable = Вести програму до цілі
loudness-target = Ціль
loudness-target-option = { $target } LUFS
loudness-ceiling = Стеля піків (dBFS)
loudness-note = −14 LUFS підходить для відтворення у стилі YouTube; −16 — поширена ціль для стримінгу; −23 — мовлення EBU R128. Та сама ціль використовується дією «Нормалізувати» після запису.
ltc-badge = LTC
ltc-title = Таймкод SMPTE (LTC)
ltc-intro = Генеруйте лінійний таймкод SMPTE на доріжці та читайте вхідний LTC з будь-якого аудіовходу — класичний аудіотаймкод для синхронізації зовнішніх рекордерів і камер у постпродакшені. Повністю офлайн.
ltc-generate = Генерувати LTC на доріжку
ltc-track = Доріжка таймкоду
ltc-track-option = Доріжка { $track }
ltc-fps = Частота кадрів
ltc-read = Читати LTC з
ltc-read-off = Вимк.
ltc-decoded = Вхідний таймкод
ltc-no-lock = немає сигналу
ltc-note = Генератор синхронізується з часом доби, без пропуску кадрів. Запишіть його доріжку (призначте в налаштуваннях виходу) або направте на вихід для зовнішнього обладнання. Читач живить рядок таймкоду в оверлеї статистики та штампує маркери розділів.
loudness-on = LUFS { $target }
loudness-off = Норм. вимк.

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Саундборд
mixer-soundboard-title = Саундборд
soundboard-title = Саундборд
soundboard-add-pad = + Пед
soundboard-stop-all = Зупинити все
soundboard-edit = Редагувати
soundboard-empty = Ще немає педів — додайте один і призначте локальний аудіокліп.
soundboard-new-pad = Новий пед
soundboard-no-clip = Немає кліпу
soundboard-audio-files = Аудіофайли
soundboard-name = Назва
soundboard-choose-clip = Вибрати кліп…
soundboard-gain = Підсилення
soundboard-choke = Чоук
soundboard-choke-none = Немає
soundboard-loop = Повтор
soundboard-auto-duck = Авто-дакінг
soundboard-tracks = Доріжки
soundboard-hotkey = Гаряча клавіша
soundboard-hotkey-placeholder = напр. Ctrl+Shift+1
soundboard-remove = Видалити

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Плагіни
mixer-plugins-title = Аудіоплагіни (CLAP / VST3)
plugins-title = Аудіоплагіни
plugins-scanning = Сканування…
plugins-none = Плагінів CLAP або VST3 у стандартних теках не знайдено.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Пам'ять
stats-dropped = Втрачено
stats-render = Рендер
stats-gpu = GPU
stats-gpu-compositing = композитинг
stats-gpu-idle = простій
stats-disk = Диск
stats-disk-free = вільно
stats-disk-left = Залиш. запису
stats-disk-rate = ≈ { $rate } МБ/с запис
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
channelstrip-solo-title = Соло (PFL) — у моніторі чутно лише соло-смуги; програмний мікс не змінюється
channelstrip-solo-source = Соло { $name } (PFL)
channelstrip-pan-label = Баланс (подвійний клац скидає)
channelstrip-pan-aria = Баланс { $name }
channelstrip-mono-label = Звести в моно
channelstrip-automix-label = Автомікс (розподіл підсилення)
channelstrip-automix-note = Розподіл підсилення: мікшер утримує сумарний рівень усіх смуг в автоміксі стабільним і передає його тому, хто говорить — ідеально для панелей із кількома мікрофонами та подкастів. Вимкнено, доки ви не додасте смугу.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Створює повернення без відлуння для цього джерела — усі в програмі, крім самого цього джерела. Використовуйте його для віддаленого гостя, щоб він не чув власний запізнілий голос.
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
properties-testtone-note = Неперервна синусоїда 1 кГц на −20 dBFS. Рівень і вимкнення звуку — на смузі мікшера; більше нічого налаштовувати.
properties-timer-format = Формат часу (strftime)
properties-timer-format-note = напр. %H:%M:%S (типово), %I:%M %p, %A %H:%M — хибний шаблон повертається до %H:%M:%S.
properties-timer-utc = Зсув UTC (хвилини)
properties-timer-utc-placeholder = місцевий час
properties-timer-duration = Тривалість (секунди)
properties-timer-target = Відлік до (HH:MM)
properties-timer-target-note = Ціль за годинником іде сама й повторюється щодня; залиште порожнім, щоб використовувати тривалість зі Старт/Пауза/Скинути.
properties-timer-end = На нулі
properties-timer-end-none = Нічого
properties-timer-end-flash = Блимати таймером
properties-timer-end-switch = Змінити сцену
properties-timer-end-scene = Сцена
properties-timer-size = Розмір (px)
properties-timer-start = Старт
properties-timer-pause = Пауза
properties-timer-reset = Скинути
properties-text-file = Читати з файлу (шлях; порожньо = текст вище)
properties-text-binding = Розбирати як
properties-text-binding-whole = Увесь файл
properties-text-binding-csv = Комірка CSV
properties-text-binding-json = Вказівник JSON
properties-text-csv-row = Рядок
properties-text-csv-column = Стовпець
properties-text-csv-column-placeholder = назва або номер
properties-text-json-pointer = Вказівник
properties-text-file-note = Файл перечитується протягом пів секунди після зміни. Атомарний запис (temp + перейменування) переноситься: останнє добре значення лишається на екрані під час підміни.
avsync-title = Калібрування синхронізації A/V
avsync-intro = Відтворіть вбудований шаблон «спалах + сигнал» через екран і колонки, зніміть його камерою та мікрофоном, які треба вирівняти, — стенд виміряє розбіжність. Петля проходить через екран і колонки, тож їхні невеликі затримки теж враховано.
avsync-video-label = Камера (джерело відео)
avsync-audio-label = Мікрофон (джерело звуку)
avsync-pick = Виберіть джерело…
avsync-no-video = Спершу додайте камеру як джерело — стенд вимірює джерела, а не «сирі» пристрої.
avsync-no-audio = Спершу додайте мікрофон як джерело звуку.
avsync-projector = Програма на весь екран на
avsync-projector-open = Відкрити проєктор
avsync-projector-window-title = Програма — синхронізація A/V
avsync-start-note = Запуск тимчасово додає джерело «Шаблон синхронізації A/V» поверх поточної сцени та відтворює сигнал на пристрої моніторингу. Після завершення все прибирається.
avsync-manual = Зсув синхронізації (мс, вручну)
avsync-start = Почати калібрування
avsync-measuring = Вимірювання близько 12 секунд — спрямуйте камеру на блимаючу програму й не шуміть…
avsync-flash-seen = Камера бачить спалах
avsync-flash-waiting = Очікування, поки камера побачить спалах…
avsync-beep-heard = Мікрофон чує сигнал
avsync-beep-waiting = Очікування, поки мікрофон почує сигнал…
avsync-cancel = Скасувати
avsync-result-offset = Відео надходить на { $offset } мс пізніше за звук.
avsync-result-detail = Виміряно за { $cycles } циклів, ±{ $jitter } мс.
avsync-negative = Звук і так надходить пізніше за відео. Затримка звуку не виправить цей напрям — якщо звук цієї камери несе інша смуга, зменшіть зсув там.
avsync-over-cap = Виміряний розрив перевищує межу зсуву { $max } мс. Такий розрив зазвичай означає не те джерело — перевірте ланцюг і виміряйте знову.
avsync-applied = Застосовано — зсув мікрофона тепер { $offset } мс.
avsync-apply = Застосувати { $offset } мс до мікрофона
avsync-again = Виміряти знову
avsync-close = Закрити
avsync-error-noFlash = Камера жодного разу не побачила спалах. Спрямуйте її на блимаючу програму (повний екран допомагає), переконайтеся, що джерело працює, і виміряйте знову.
avsync-error-noBeep = Мікрофон жодного разу не почув сигнал. Переконайтеся, що пристрій моніторингу чутно й мікрофон працює (не заглушений push-to-talk), і виміряйте знову.
avsync-error-tooFewCycles = Замало чистих циклів спалаху/сигналу. Тримайте шаблон добре видимим і чутним упродовж усього виміру.
avsync-error-notThePattern = Побачене чи почуте не повторюється в ритмі шаблону — імовірно, це світло чи шум кімнати, а не тестовий сигнал.
avsync-error-unstable = Цикли надто розходяться, щоб довіряти одному числу. Закріпіть камеру, зменшіть шум і виміряйте знову.
hotkey-audit-title = Мапа гарячих клавіш
hotkey-audit-search = Пошук
hotkey-audit-filter = Функція
hotkey-audit-filter-all = Усі функції
hotkey-audit-col-key = Клавіша
hotkey-audit-col-action = Дія
hotkey-audit-col-where = Де
hotkey-audit-col-status = Стан
hotkey-audit-ok = OK
hotkey-audit-shared = Спільна для { $count } прив'язок
hotkey-audit-unregistered = Не зареєстрована в ОС (зайнята деінде або недоступна)
hotkey-audit-invalid = Недійсне сполучення
hotkey-audit-empty = Гарячих клавіш ще немає — призначте їх у Налаштування → Гарячі клавіші або на смузі мікшера.
hotkey-audit-export = Експортувати шпаргалку
hotkey-audit-exported = Збережено в { $path }
hotkey-audit-note = Призначення і зміна клавіш — у Налаштування → Гарячі клавіші (глобальні дії) та на кожній смузі мікшера (push-to-talk / push-to-mute); ця таблиця їх перевіряє й документує.
hotkey-audit-action-record = Перемкнути запис
hotkey-audit-action-go-live = Перемкнути трансляцію
hotkey-audit-action-transition = Виконати перехід
hotkey-audit-action-save-replay = Зберегти повтор
hotkey-audit-action-add-marker = Додати маркер
hotkey-audit-action-still = Зняти стоп-кадр
hotkey-audit-action-panic = Екран паніки
hotkey-audit-action-timer-toggle = Старт/пауза всіх таймерів
hotkey-audit-action-timer-reset = Скинути всі таймери
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Запис
hotkey-audit-feature-streaming = Трансляція
hotkey-audit-feature-studio = Режим студії
hotkey-audit-feature-replay = Повтор
hotkey-audit-feature-markers = Маркери
hotkey-audit-feature-stills = Стоп-кадри
hotkey-audit-feature-panic = Паніка
hotkey-audit-feature-timers = Таймери
hotkey-audit-feature-audio = Звук (за джерелом)
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
properties-deinterlace = Деінтерлейсинг
properties-deinterlace-off = Вимк.
properties-deinterlace-discard = Відкидання (подвоєння рядків одного поля)
properties-deinterlace-bob = Боб (поля почергово)
properties-deinterlace-linear = Лінійний (інтерполяція)
properties-deinterlace-blend = Змішування (середнє полів)
properties-deinterlace-adaptive = Адаптивний до руху (клас yadif)
properties-field-order = Порядок полів
properties-field-order-top = Спершу верхнє поле
properties-field-order-bottom = Спершу нижнє поле
properties-deinterlace-note = Для черезрядкових сигналів карт захоплення. Чистий CPU, однаково на всіх ОС; зміна перезапускає пристрій (як зміна формату).
camera-controls-title = Керування камерою
camera-controls-refresh = Оновити
camera-controls-reset = Скинути профіль
camera-controls-empty = Зараз регулювань немає — пристрій має транслювати (спершу додайте його до сцени), а деякі бекенди не повідомляють нічого (особливо macOS). Це чесний стан для кожної ОС.
camera-controls-note = Зміни діють одразу й зберігаються в профіль пристрою; він знову застосовується після перепідключення та перезапуску.
camera-control-brightness = Яскравість
camera-control-contrast = Контраст
camera-control-hue = Відтінок
camera-control-saturation = Насиченість
camera-control-sharpness = Різкість
camera-control-gamma = Гамма
camera-control-white-balance = Баланс білого
camera-control-backlight = Компенсація контрового світла
camera-control-gain = Підсилення
camera-control-pan = Панорама
camera-control-tilt = Нахил
camera-control-zoom = Зум
camera-control-exposure = Експозиція
camera-control-iris = Діафрагма
camera-control-focus = Фокус
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
audiofilters-name-parametric-eq = Параметричний еквалайзер
audiofilters-name-de-esser = Де-есер
audiofilters-name-rumble-guard = Фільтр гулу
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Пресет
audiofilters-voice-preset-pick = Пресет голосу…
audiofilters-voice-broadcast = Ефірний голос
audiofilters-voice-podcast = Подкаст-голос
audiofilters-voice-clean = Чистий голос
audiofilters-voice-none = Очистити ланцюг
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Частота сибілянтів (Hz)
audiofilters-deesser-amount = Макс. зниження (dB)
audiofilters-rumble-freq = Зріз низьких (Hz)
audiofilters-title = Аудіофільтри — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Крива АЧХ параметричного еквалайзера з живим спектром
eq-band-type = Тип
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Смуга
eq-remove-band = Видалити смугу
eq-type-bell = Дзвін
eq-type-lowShelf = Нижня полиця
eq-type-highShelf = Верхня полиця
eq-type-notch = Режекторний
eq-type-highPass = Верхніх частот
eq-type-lowPass = Нижніх частот
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
filters-name-shader = Шейдер (WGSL)
filters-shader-gallery = Галерея
filters-shader-gallery-pick = Завантажити пресет…
filters-shader-gallery-grayscale = Відтінки сірого
filters-shader-gallery-invert = Інвертувати
filters-shader-gallery-scanlines = Рядки розгортки
filters-shader-gallery-vignette = Віньєтка
filters-shader-source = Вихідний код шейдера (WGSL)
filters-shader-hint = Напишіть на WGSL функцію effect(uv, color, p, texel, time), що повертає vec4. Позначте параметри через // @param name min max default для повзунків. Некоректний шейдер ігнорується — джерело показується без фільтра, доки не скомпілюється.
filters-name-bezier-mask = Маска Безьє
filters-mask-editor-hint = Перетягніть точку, щоб перемістити її, двічі клацніть, щоб додати, клацніть правою кнопкою по точці, щоб видалити її.
filters-mask-shape = Форма
filters-mask-shape-pick = Пресет…
filters-mask-shape-rectangle = Прямокутник
filters-mask-shape-diamond = Ромб
filters-mask-shape-hexagon = Шестикутник
filters-mask-shape-circle = Коло
filters-mask-feather = Розтушування
filters-mask-export-wipe = Експортувати як витирання…
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
recordings-trim = Обрізати
recordings-trim-title = Виріжте кліп із цього запису — зрізи по ключових кадрах експортуються без перекодування
recordings-verify = Перевірити
recordings-verify-title = Перевірка цілісності файлу — структура контейнера, безперервність, чергування A/V, тривалість
recordings-verifying = Перевірка…
verify-dismiss = Закрити
verify-verdict-pass = { $name } — цілісність гаразд
verify-verdict-warn = { $name } — перевірено з попередженнями
verify-verdict-fail = { $name } — знайдено проблеми
verify-container = Контейнер
verify-video-continuity = Безперервність відео
verify-audio-continuity = Безперервність аудіо
verify-av-interleave = Чергування A/V
verify-duration = Тривалість
recordings-alpha-label = альфа
recordings-prores-title = Експортувати майстер .mov ProRes 4444 зі збереженням альфи (для монтажу)
recordings-qtrle-title = Експортувати .mov QuickTime Animation зі збереженням альфи (максимальна сумісність)
trim-title = Обрізання — { $name }
trim-loading = Читання файлу…
trim-preview-alt = Кадр попереднього перегляду
trim-position = Позиція відтворення
trim-step-second-back = Секунда назад
trim-step-frame-back = Кадр назад
trim-step-frame-forward = Кадр уперед
trim-step-second-forward = Секунда вперед
trim-snap = Ключовий кадр
trim-snap-title = Прив'язати до найближчого ключового кадру — зріз там експортується без перекодування
trim-set-in = Точка входу
trim-set-out = Точка виходу
trim-range-invalid = Точка виходу має бути після точки входу.
trim-copy-badge = ✓ Експорт без перекодування — точка входу лежить на ключовому кадрі.
trim-reencode-badge = Буде перекодовано: точка входу між ключовими кадрами (натисніть «Ключовий кадр» для зрізу без втрат).
trim-export = Експортувати кліп
trim-export-916 = 9:16
trim-export-916-title = Експорт у вертикальному форматі (центроване обрізання під розмір вертикального полотна) — завжди перекодовує
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
recordings-normalize = Нормалізувати
recordings-normalizing = Нормалізація…
recordings-normalize-title = Нормалізувати гучність до цілі (записує копію)
recordings-normalized-to = Нормалізовано до { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Лише звук
audiorec-format = Формат аудіозапису
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Записати звук
audiorec-stop = Зупинити
audiorec-pause = Пауза
audiorec-resume = Відновити
audiorec-recording = REC { $sec } с
audiorec-saved = Збережено { $count } файл(ів) доріжок

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
output-recording-template = Ім'я файлу запису
output-replay-template = Ім'я файлу повтору
output-still-template = Ім'я файлу стоп-кадру
output-template-tokens = Токени: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Папка повторів
output-still-folder = Папка стоп-кадрів
output-same-folder-placeholder = Папка записів
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
output-iso-heading = ISO-запис
output-iso-explainer = Записуйте вибрані джерела в чистому вигляді, кожне у власний файл поруч із програмою — до композиції, у розмірі та частоті кадрів полотна, щоб кожен файл лягав на монтажну шкалу вирівняним. Дві доріжки комфортні на GPU середнього класу; кожна додаткова — ще один рендер і кодування.
output-iso-none = У колекції поки немає джерел.
output-iso-source-on = «{ $name }» записується у власний ISO-файл — натисніть, щоб зупинити
output-iso-source-off = Записувати «{ $name }» у власний ISO-файл
output-iso-post-filter = Записувати з фільтрами джерела (пост-фільтр); без позначки записується необроблене джерело
output-iso-format = Формат ISO
output-iso-encoder = Відеокодувальник ISO
output-alpha-frec = Записувати з прозорістю (альфа) — програма на прозорому тлі
output-alpha-title = Рекордер отримує власний прозорий рендер; попередній перегляд і стрим лишаються звичайними. Експортуйте в ProRes 4444 чи QTRLE зі списку записів — MP4/MKV сплющують альфу.
output-split-events = Також починати новий файл при… (кожна частина починається точно на події; мінімальна довжина 1 с)
output-split-on-scene = зміні сцени
output-split-on-marker = маркері
output-split-on-rundown = кроці сценарію
output-auto-markers = Автоматично ставити маркери розділів на подіях студії (зміна сцени, збереження повтору, перепідключення, втрачені кадри, тривоги, правила)
output-auto-markers-title = Типізовані маркери потрапляють у розділи запису (mkv) або файл .chapters.txt, поруч із ручною гарячою клавішею
output-pipeline-heading = Конвеєр після запису
output-pipeline-explainer = Після фіналізації запису ці кроки виконуються над основним файлом по черзі у фоні. Закритий набір дій — кроку «виконати команду» немає навмисно. Ланцюжок зупиняється на першій помилці.
output-pipeline-enabled = Запускати конвеєр після кожного запису
output-pipeline-add = Додати крок…
output-pipeline-up = Вгору
output-pipeline-down = Вниз
output-pipeline-remove = Видалити крок
output-pipeline-template = Шаблон перейменування (токени CAP-M25)
output-pipeline-folder = Тека
pipeline-queue = Конвеєр після запису
pipeline-verify = Перевірити
pipeline-remux = Ремукс у MP4
pipeline-normalize = Нормалізувати гучність
pipeline-rename = Перейменувати
pipeline-move = Перемістити до теки
pipeline-copy = Скопіювати до теки
pipeline-reveal = Показати у файловому менеджері
pipeline-luaEvent = Сповістити Lua-скрипти
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
hotkeys-go-live = В ефір / Завершити трансляцію
hotkeys-transition = Перехід студійного режиму
hotkeys-save-replay = Зберегти повтор (останні N секунд)
hotkeys-add-marker = Поставити маркер розділу (запис)
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

# --- OBS import (CAP-M02) ---
workspace-import-obs = Імпорт з OBS…
workspace-import-obs-hint = Завантажте колекцію сцен OBS (її scenes.json). Поточну колекцію буде збережено заздалегідь.
workspace-import-busy = Імпортування…
workspace-import-title = «{ $name }» імпортовано
workspace-import-summary = сцен: { $scenes } · джерел: { $sources } · елементів: { $items }
workspace-import-dismiss = Закрити
workspace-import-clean = Усе імпортовано без помилок.
workspace-import-geometry-caveat = Розміри й позиції підганяються з макета OBS — перевірте кожну сцену та повторно виберіть пристрої захоплення.
workspace-import-notes-title = Імпортовано із зауваженнями
workspace-import-skipped-title = Не імпортовано
import-note-needsReselect = Повторно виберіть пристрій/монітор/вікно
import-note-gameCaptureAsWindow = Захоплення гри → Захоплення вікна
import-note-referencesFile = Перевірте шлях до файлу
import-note-filterDropped = Деякі фільтри не підтримуються
import-note-geometryApproximated = Позиція/розмір приблизні
import-skip-unsupportedKind = Немає відповідного типу джерела
import-skip-group = Групи ще не підтримуються

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Повторно прив'язати відсутні файли…
doctor-title = Відсутні файли
doctor-scanning = Сканування…
doctor-all-good = Усі згадані файли на місці. Немає що прив'язувати.
doctor-intro = { $count } згаданих файлів не знайдено на цьому комп'ютері. Вкажіть нове розташування кожного — кожну сцену, що його використовує, буде виправлено одразу.
doctor-relinked = Повторно прив'язано посилань: { $count }.
doctor-uses = використано { $count }×
doctor-locate = Знайти…
doctor-locate-folder = Шукати в теці…
doctor-locate-folder-hint = Виберіть теку; кожен відсутній файл знаходиться за іменем і прив'язується повторно.
doctor-kind-image = зображення
doctor-kind-media = медіа
doctor-kind-slideshow = слайд-шоу
doctor-kind-font = шрифт
doctor-kind-lut = LUT
doctor-kind-mask = маска
history-relinkFiles = Повторно прив'язати файли

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
studio-preview-stinger-matte-label = Трек-матт
studio-preview-stinger-matte-title = Як стінгер із трек-маттом пакує прозорість: заливка та її матт поруч (горизонтально) або один над одним (вертикально)
studio-preview-stinger-duck-label = Приглушувати програму
studio-preview-stinger-duck-title = Приглушувати звук програми під власним звуком стінгера під час його відтворення (0 = вимк.)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = Переміщення (морфінг)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Немає
stinger-matte-horizontal = Поруч
stinger-matte-vertical = Один над одним

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
settings-open-about = Про застосунок…

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

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Ласкаво просимо до Freally Capture
wizard-welcome = Два швидкі кроки: перевіримо, на що здатен ваш комп'ютер, а потім створимо сцену. Це займе близько тридцяти секунд, і згодом усе можна змінити.
wizard-local-first = Нічого звідси не залишає ваш комп'ютер. У Freally Capture немає облікових записів, телеметрії чи хмари.
wizard-start = Почати
wizard-skip = Пропустити
wizard-hardware-title = На що здатен ваш комп'ютер
wizard-probing = Перевіряємо вашу відеокарту та процесор…
wizard-encoder = Кодувальник
wizard-canvas = Полотно
wizard-bitrate = Бітрейт
wizard-probe-found = Знайдено: { $gpus } · { $cores } фізичних ядер
wizard-no-gpu = немає окремого GPU
wizard-apply = Використати ці налаштування
wizard-keep-current = Залишити як є
wizard-template-title = Почніть зі сцени
wizard-template-screen = Захопити мій екран
wizard-template-screen-note = Додає «Захоплення екрана» вашого основного монітора. Найпоширеніший спосіб почати.
wizard-template-empty = Почати з порожньої
wizard-template-empty-note = Порожня сцена. Додавайте джерела самі кнопкою +.
wizard-done = Усе готово.
wizard-done-hint = Натисніть Ctrl+K будь-коли, щоб шукати сцени, джерела та дії. Налаштування — за кнопкою ⚙.
wizard-close = Почати трансляцію

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Ваша відеокарта може кодувати відео самостійно, тож процесор лишається вільним для решти студії.
autoconfig-reason-software = Придатного апаратного кодувальника не знайдено, тож кодуватиме процесор. Це працює, лише навантажує CPU більше.
autoconfig-reason-quality-hardware = 1080p за 60 кадрів на секунду, з бітрейтом, який приймає кожна велика платформа.
autoconfig-reason-quality-software = 30 кадрів на секунду, бо програмне кодування на 60 втрачає кадри на більшості процесорів.
autoconfig-reason-quality-low-cores = Нижчий бітрейт, бо цей процесор має мало ядер, і програмне кодування конкуруватиме за них із композитором.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Запис розпочато
announce-recording-paused = Запис призупинено
announce-recording-stopped = Запис зупинено
announce-live-started = Ви в ефірі
announce-live-ended = Трансляцію завершено
announce-reconnecting = Зв'язок втрачено, повторне підключення
announce-stream-failed = Помилка трансляції
announce-frames-dropped = Пропущено кадрів { $count }

# CAP-M01 — undo/redo edit history
palette-undo = Скасувати
palette-redo = Повторити
palette-edit-history = Історія змін…
history-title = Історія змін
history-empty = Поки немає що скасовувати.
history-current = Поточний стан
history-close = Закрити
history-addScene = Додати сцену
history-renameScene = Перейменувати сцену
history-removeScene = Видалити сцену
history-reorderScene = Змінити порядок сцен
history-addSource = Додати джерело
history-removeSource = Видалити джерело
history-reorderSource = Змінити порядок джерел
history-renameSource = Перейменувати джерело
history-transformSource = Перемістити джерело
history-toggleVisibility = Перемкнути видимість
history-toggleLock = Перемкнути блокування
history-setBlendMode = Змінити режим змішування
history-editSourceProperties = Змінити властивості
history-applyLayout = Розташувати макет
history-moveToSeat = Перемістити на місце
history-groupSources = Згрупувати джерела
history-ungroupSources = Розгрупувати джерела
history-toggleGroupVisibility = Перемкнути групу
history-setSceneAudio = Звук сцени
history-setVerticalCanvas = Вертикальне полотно
history-addFilter = Додати фільтр
history-removeFilter = Видалити фільтр
history-reorderFilter = Змінити порядок фільтрів
history-editFilter = Змінити фільтр
history-toggleFilter = Перемкнути фільтр
history-setVolume = Налаштувати гучність
history-toggleMute = Перемкнути звук
history-setMonitor = Змінити моніторинг
history-setTracks = Змінити доріжки
history-setSyncOffset = Налаштувати синхронізацію A/V
history-setAudioHotkeys = Аудіоскорочення

# CAP-M04 — alignment aids
settings-alignment-section = Помічники вирівнювання
settings-smart-guides = Розумні напрямні (прилипання під час перетягування)
settings-safe-areas = Накладки безпечної зони
settings-rulers = Лінійки
align-group = Вирівняти за полотном
align-left = Вирівняти ліворуч
align-hcenter = Центрувати горизонтально
align-right = Вирівняти праворуч
align-top = Вирівняти вгору
align-vcenter = Центрувати вертикально
align-bottom = Вирівняти вниз

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Вирівняти й розподілити вибране
arrange-left = Вирівняти ліві краї
arrange-hcenter = Центрувати горизонтально
arrange-right = Вирівняти праві краї
arrange-top = Вирівняти верхні краї
arrange-vcenter = Центрувати вертикально
arrange-bottom = Вирівняти нижні краї
distribute-h = Розподілити горизонтально
distribute-v = Розподілити вертикально
guides-group = Напрямні
guides-add-v = Додати вертикальну напрямну
guides-add-h = Додати горизонтальну напрямну
guides-clear = Прибрати всі напрямні
history-arrangeItems = Упорядкувати елементи
history-editGuides = Редагувати напрямні

# CAP-M05 — edit transform + copy/paste
transform-title = Змінити трансформацію — { $name }
transform-anchor = Опорна точка
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Обертання
transform-crop = Обрізання
transform-crop-left = Ліворуч
transform-crop-top = Зверху
transform-crop-right = Праворуч
transform-crop-bottom = Знизу
transform-no-size = Розмір та обрізання стануть доступні, коли джерело повідомить свої розміри.
transform-copy = Копіювати трансформацію
transform-paste = Вставити трансформацію
transform-close = Закрити
filters-copy = Копіювати фільтри ({ $count })
filters-paste = Вставити фільтри ({ $count })
palette-edit-transform = Змінити трансформацію…
history-pasteFilters = Вставити фільтри

# CAP-M26 — keying workbench
workbench-title = Майстерня кеїнгу — { $name }
workbench-mode-keyed = З ключем
workbench-mode-source = Джерело
workbench-mode-matte = Матт
workbench-mode-split = Розділення
workbench-eyedropper = Піпетка
workbench-eyedropper-hint = Клацніть джерело, щоб узяти колір ключа.
workbench-loupe = Лупа
workbench-split = Розділення
workbench-preview-alt = Перегляд майстерні кеїнгу
workbench-tune = Налаштувати
workbench-close = Закрити

# CAP-M06 — multiview monitor
multiview-title = Мультив’ю
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Клацніть сцену, щоб перемкнутися на неї.
multiview-hint-stage = Клацніть сцену, щоб підготувати її в попередньому перегляді.
palette-multiview = Монітор мультив’ю

# CAP-M07 — projectors
projector-title = Відкрити проектор
projector-source = Джерело
projector-target-program = Програма
projector-target-preview = Попередній перегляд
projector-target-scene = Сцена…
projector-target-source = Джерело…
projector-target-multiview = Мультивʼю
projector-which-scene = Яка сцена
projector-which-source = Яке джерело
projector-none = Немає що показати
projector-display = Дисплей
projector-windowed = Плаваюче вікно (цей екран)
projector-display-option = Дисплей { $n } — { $w }×{ $h }
projector-primary = (основний)
projector-open = Відкрити
projector-cancel = Скасувати
projector-exit-hint = Натисніть Esc для виходу
palette-projector = Відкрити проектор…

# CAP-M08 — still-frame grab
palette-still = Зробити стоп-кадр…
still-saved-toast = Стоп-кадр збережено: { $name }
still-failed-toast = Не вдалося зробити стоп-кадр: { $error }
hotkeys-still = Зробити стоп-кадр

# CAP-M13 — source health dashboard
palette-source-health = Стан джерел…
palette-av-sync = Калібрування синхронізації A/V…
palette-hotkey-audit = Мапа гарячих клавіш…
health-title = Стан джерел
health-col-source = Джерело
health-col-state = Стан
health-col-resolution = Роздільна здатність
health-col-fps = FPS
health-col-last-frame = Останній кадр
health-col-dropped = Пропущено
health-col-retries = Перезапуски
health-col-actions = Дії
health-state-live = В ефірі
health-state-waiting = Очікування
health-state-error = Помилка
health-state-inactive = Неактивне
health-restart = Перезапустити
health-properties = Властивості
health-empty = У цій колекції ще немає джерел.
health-seconds = { $value } с

# CAP-M23 — quit guard + orderly shutdown
quit-title = Вийти з Freally Capture?
quit-body = Під час виходу зараз буде безпечно виконано по черзі:
quit-consequence-stream = Завершення прямого ефіру та відключення від сервісу.
quit-consequence-recording = Зупинка запису та фіналізація його файлів.
quit-consequence-replay = Вимкнення буфера повтору — незбережені кадри буде відкинуто.
quit-confirm = Вийти безпечно
quit-quitting = Завершення…
quit-cancel = Скасувати

# CAP-M11 — crash-safe recording salvage
salvage-title = Відновити перервані записи?
salvage-body = Останній сеанс завершився несподівано, поки ці записи ще писалися. Відновлення створює відтворювану копію поруч з оригіналом — початковий файл ніколи не змінюється.
salvage-repair = Відновити
salvage-repairing = Відновлення…
salvage-done = Відновлено
salvage-repaired = Відновлено → { $name }
salvage-failed = Помилка відновлення: { $error }
salvage-dismiss = Не зараз

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Збій кодувальника — перемкнено з { $from } на { $to }. Ефір перепідключився і триває.
fallback-toast-recording = Збій кодувальника — перемкнено з { $from } на { $to }. Запис триває в новому файлі.
fallback-note = Резервний кодувальник: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Звук програми зник
alarm-clipping = Звук програми кліпує
alarm-black = Зображення програми чорне
alarm-frozen = Зображення програми давно не змінюється
alarm-lowDisk = Місце на диску: залишилось близько { $minutes } хв за поточного бітрейту
alarm-dismiss = Закрити тривогу
alarm-cleared = Вирішено: { $alarm }

# CAP-M22 — panic button
palette-panic = Паніка — перемкнути на екран приватності
panic-banner-title = Паніка
panic-banner-body = Програма показує екран приватності; увесь звук вимкнено, захоплення зупинено. Ефір і запис тривають.
panic-restore = Відновити…
panic-restore-confirm = Відновити програму?
panic-restore-yes = Відновити
panic-restore-cancel = Скасувати
hotkeys-panic = Паніка (екран приватності)
hotkeys-timer-toggle = Старт/пауза всіх таймерів
hotkeys-timer-reset = Скинути всі таймери
panic-slate-color = Колір екрана паніки
panic-slate-image = Зображення екрана паніки
panic-slate-image-placeholder = Необов'язковий шлях до зображення

# CAP-M24 — redacted diagnostics bundle
diag-title = Діагностичний пакет
diag-intro = Експортує очищений .zip (знімок налаштувань, проба кодувальників, свіжа статистика — секрети, шляхи та імена ніколи не включаються) для ручного долучення до issue на GitHub. Нічого нікуди не надсилається.
diag-preview = Показати вміст
diag-hide-preview = Сховати перегляд
diag-export = Експортувати .zip
diag-exported = Експортовано: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Передефірна перевірка
preflight-intro = Кожен блокуючий пункт має бути зеленим; решта — чесні підказки.
preflight-item-targets = Цілі налаштовано (ключ/URL)
preflight-item-encoder = Доступний робочий кодувальник
preflight-item-sources = Усі джерела справні
preflight-item-disk = Місце на диску для запису
preflight-item-mic = Рівень мікрофона
preflight-item-desktopAudio = Рівень звуку робочого столу
preflight-item-replay = Буфер повтору увімкнено
preflight-targets-detail = { $count } увімкнено
preflight-sources-detail = { $count } джерел(а) з помилкою
preflight-disk-detail = ~{ $minutes } хв за поточного бітрейту
preflight-fix-stream = Налаштування ефіру…
preflight-fix-components = Компоненти…
preflight-fix-sources = Стан джерел…
preflight-fix-replay = Увімкнути
preflight-optional = необов'язково
preflight-hold = Не виходити в ефір, доки все не зелене
preflight-cancel = Скасувати
preflight-go-anyway = Все одно в ефір
preflight-go-live = В ефір


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Тло
scenes-backdrop-aria = Тло сцени { $name }
backdrop-title = Тло — { $name }
backdrop-hint = Шпалери, закріплені позаду всього в цій сцені: зображення, анімований GIF або зациклене відео. Захоплення завжди зверху; прокручуйте над полотном, щоб масштабувати.
backdrop-choose = Вибрати зображення або відео…
backdrop-remove = Прибрати тло
backdrop-none = Тло не задано.
backdrop-position = Розташування
backdrop-split-full = Усе полотно
backdrop-split-left = Ліва половина
backdrop-split-right = Права половина
backdrop-split-top = Верхня половина
backdrop-split-bottom = Нижня половина
backdrop-sync = Запускати відтворення разом із записом
backdrop-sync-hint = Тримає перший кадр до початку запису; кожен дубль запускає відео спочатку.
backdrop-preview-play = Переглянути відтворення
backdrop-preview-pause = Призупинити перегляд
backdrop-filter-all = Тла (зображення та відео)
backdrop-filter-images = Зображення
backdrop-filter-media = Відео та GIF
sources-backdrop-badge = Шпалери тла (закріплені внизу)
sources-backdrop-pinned = Тло лишається закріпленим у самому низу
filters-name-flip = Віддзеркалення
filters-flip-horizontal = По горизонталі
filters-flip-vertical = По вертикалі
history-setSceneBackdrop = Задати тло
history-setBackdropSplit = Перемістити тло
history-setBackdropSync = Синхронізація тла із записом
backdrop-scrub = Позиція відтворення
backdrop-loop = Цикл
backdrop-reverse = Відтворювати задом наперед
backdrop-reverse-hint = Реверс один раз створює обернену копію (відео потребує компонента ffmpeg; GIF обертається миттєво) — перше перемикання може тривати довго на великих файлах.
filters-scaling = Масштабування
filters-scaling-hint = Піксель-точні режими для ретро/піксельного вмісту; «Ціле» додатково прив'язує намальований розмір до цілих кратних (маркери показують логічний розмір).
filters-scaling-auto = Плавне
filters-scaling-nearest = Найближчий сусід
filters-scaling-integer = Ціле (цілі ×)
filters-scaling-sharp = Різке білінійне
history-setScaling = Змінити масштабування
hotkeys-zoom-100 = Зум: скинути (100%)
hotkeys-zoom-150 = Зум: наблизити до 150%
hotkeys-zoom-200 = Зум: наблизити 2×
sources-follow-title = Слідувати за курсором під час зуму (Windows; прокручуйте над полотном для зуму)
sources-follow-item = Перемкнути стеження за курсором для { $name }
filters-autocrop = ✂ Обрізати чорні смуги
filters-autocrop-title = Сканує наступний кадр на смуги леттербокса/піларбокса та обрізає їх (можна скасувати). Темні сцени ніколи не обрізаються.
filters-autocrop-follow = Перевіряти знову при зміні роздільності
history-autoCrop = Автообрізання чорних смуг
sources-link-audio = Також захоплювати звук цього застосунку (пов'язано: приховання глушить, видалення вікна видаляє)
history-addLinkedWindow = Додати вікно + пов'язаний звук
sources-hdr-title = Цей дисплей HDR — відкрийте тонмапінг (полотно лишається SDR)
sources-hdr-item = HDR-тонмапінг для { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Цей дисплей виводить HDR. Без тонмапінгу світла зрізаються, і захоплення виглядає блякло на SDR-полотні. Зміни діють із наступного кадру.
sources-hdr-enable-suggested = Увімкнути рекомендоване (maxRGB, 200 ніт)
sources-hdr-operator = Оператор
sources-hdr-op-clip = Зріз (вимк.)
sources-hdr-op-maxrgb = maxRGB (зберігає відтінок)
sources-hdr-op-reinhard = Рейнгард
sources-hdr-op-bt2408 = Коліно BT.2408 (SDR точно)
sources-hdr-paper-white = Паперовий білий
sources-hdr-nits = ніт
projector-target-passthrough = Наскрізний монітор (низька затримка)
projector-which-device = Пристрій
projector-passthrough-none = Спершу додайте дисплей, вікно або пристрій захоплення.
projector-passthrough-about = Сирі кадри пристрою — без сцен, фільтрів і композитора. Показує виміряну затримку; звук і далі моніториться через канал мікшера.
projector-passthrough-hint = Наскрізний — Esc закриває
projector-latency = { $ms } мс
projector-latency-measuring = вимірювання…
automation-title = Автоматизація — правила, макроси та змінні
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Правила
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Увімк.
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Коли
automation-then-run = тоді запустити
automation-no-macro = (no macro)
automation-macros = Макроси
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Запустити
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Змінні студії
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
rundown-title = Сценарій ефіру
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Старт
rundown-next = Далі ▸
rundown-stop = Стоп
rundown-idle = Не запущено
rundown-next-up = Далі: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Крок
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
automation-layer = Шар
automation-layer-hint = Спрацьовує лише за активного шару (порожньо = всі). Шари липкі: клавіша шару перемикає й лишається (API ОС не дає шарів «по утриманню»).
automation-chord-hint = Звичайна клавіша (Ctrl+Shift+M) або акорд із двох натискань (Ctrl+K, 3). Друга клавіша зайнята лише поки акорд очікується.
panel-title = LAN-панель і tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Запустити панель
panel-port = Порт
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Пароль
panel-show = Показати
panel-hide = Сховати
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Зберегти
osc-title = Панель керування OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Приймати OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ-камери
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Камера
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Адреса
ptz-port = Порт
ptz-speed = Швидкість
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
ptz-presets = Пресети
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Викликати
ptz-store = Зберегти
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI-панель керування
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Вхід
midi-output = Вихід (зворотний зв'язок)
midi-none = (none)
midi-learn = Навчити
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Дія
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
panel-lan-warning = ⚠ Трафік LAN не шифрується — пароль передається в URL через HTTP. Використовуйте лише в довіреній мережі.
osc-lan-warning = ⚠ OSC не має пароля — будь-який пристрій у мережі може надіслати ці команди. Режим LAN — лише в довіреній мережі.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Стат.
sources-add-system-stats = Статистика продуктивності (HUD)
sources-stats-title = Додати HUD продуктивності
sources-stats-note = Показує глядачам у програмі справжні виміряні показники студії — fps, CPU, пам'ять, час рендера, втрачені кадри та поточний бітрейт. Які рядки показувати, розмір і колір — у «Властивостях» джерела. Завантаження GPU не показується, бо воно не вимірюється.
sources-stats-add = Додати HUD статистики
properties-stats-show-fps = Показувати FPS
properties-stats-show-cpu = Показувати CPU
properties-stats-show-memory = Показувати пам'ять
properties-stats-show-render = Показувати час рендера
properties-stats-show-dropped = Показувати втрачені кадри
properties-stats-show-bitrate = Показувати бітрейт
properties-stats-show-timecode = Показувати таймкод (LTC)
properties-stats-size = Розмір (px)
properties-stats-note = HUD малює компактні універсальні підписи (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) просто в програму; коли трансляції немає, рядок бітрейту показує «—».

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Візуалізатор
sources-add-visualizer = Аудіовізуалізатор
sources-visualizer-title = Додати аудіовізуалізатор
sources-visualizer-style-label = Стиль
sources-visualizer-style-bars = Смуги спектра
sources-visualizer-style-scope = Осцилограф
sources-visualizer-style-vu = VU-індикатори
sources-visualizer-target-label = Слухає
sources-visualizer-target-master = Майстер-мікс
sources-visualizer-target-track = Доріжка { $n }
sources-visualizer-note = Малює сигнал, який справді потрапляє в мікс (після фейдера) — заглушене джерело лишається пласким, точно як звучить. Розмір, колір, кількість смуг і швидкість спаду — у «Властивостях» джерела.
sources-visualizer-add = Додати візуалізатор
properties-vis-bands = Смуги
properties-vis-decay = Швидкість спаду (dB/s)
properties-vis-peak-hold = Позначки піків
properties-vis-missing-source = (джерело відсутнє)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Спліти
sources-add-split-timer = Спідран-таймер сплітів
sources-splits-title = Додати таймер сплітів
sources-splits-file-label = Файл .lss (LiveSplit)
sources-splits-comparison-label = Порівнювати з
sources-splits-comparison-pb = Особистий рекорд
sources-splits-comparison-best = Найкращі сегменти
sources-splits-comparison-average = Середнє
sources-splits-note = Файл імпортується лише для читання — до нього нічого не записується. Призначте глобальні клавіші Split / Undo / Skip / Reset у Налаштування → Гарячі клавіші. Авто-сплітери через пам'ять процесу навмисно не підтримуються.
sources-splits-add = Додати таймер сплітів
properties-splits-size = Розмір (px)
properties-splits-ahead = Випередження
properties-splits-behind = Відставання
properties-splits-gold = Золото
properties-splits-split = Спліт
properties-splits-undo = Скасувати
properties-splits-skip = Пропустити
properties-splits-reset = Скинути
properties-splits-note = Кнопки керують запущеним таймером (глобальні клавіші роблять те саме з будь-якого застосунку). Забіг ніколи не записується у файл .lss.
hotkeys-split-split = Таймер сплітів: старт / спліт
hotkeys-split-undo = Таймер сплітів: скасувати спліт
hotkeys-split-skip = Таймер сплітів: пропустити сегмент
hotkeys-split-reset = Таймер сплітів: скинути
hotkey-audit-action-split-split = Спліт (таймер сплітів)
hotkey-audit-action-split-undo = Скасувати спліт
hotkey-audit-action-split-skip = Пропустити сегмент
hotkey-audit-action-split-reset = Скинути таймер сплітів
hotkey-audit-feature-split-timer = Таймер сплітів

# Media playlist source (CAP-N17)
sources-badge-playlist = Плейлист
sources-add-playlist = Медіаплейлист (без пауз)
sources-playlist-title = Додати медіаплейлист
sources-playlist-files-label = Файли (по одному в рядку, грають згори донизу)
sources-playlist-browse = Огляд…
sources-playlist-loop = Повтор
sources-playlist-shuffle = Перемішати (один розіграш на запуск; у повторі порядок той самий)
sources-playlist-hold-last = Тримати останній кадр наприкінці
sources-playlist-note = Відтворює весь обрізаний список без пауз через позначений компонент ffmpeg (лише wire-формати — .frec і зображення через Медіа/Слайдшоу). Елементи або всі відео, або всі аудіо, ніколи впереміш. Обрізання, cue-точки та змінна «now playing» — у «Властивостях».
sources-playlist-add = Додати плейлист
properties-playlist-items = Елементи (згори донизу)
properties-playlist-up = Вгору
properties-playlist-down = Вниз
properties-playlist-remove = Прибрати елемент
properties-playlist-in = З (с)
properties-playlist-out = До (с)
properties-playlist-cues = Cue (с, через кому)
properties-playlist-add-item = + Додати елемент
properties-playlist-loop = Повтор
properties-playlist-shuffle = Перемішати
properties-playlist-hold-last = Тримати останній кадр
properties-playlist-hw = Апаратне декодування
properties-playlist-variable = Змінна «now playing» (порожньо = вимк.)
properties-playlist-previous = ⏮ Назад
properties-playlist-next = ⏭ Далі
properties-playlist-note = Кнопки cue та Далі/Назад керують ЖИВИМ плейлистом; правки елементів набирають чинності після «Застосувати» (плейлист перезапускається). Вставте {"{{"}yourVariable{"}}"} у джерело «Текст», щоб показувати поточний елемент.
hotkeys-playlist-next = Плейлист: наступний елемент
hotkeys-playlist-previous = Плейлист: попередній елемент
hotkey-audit-action-playlist-next = Плейлист: далі
hotkey-audit-action-playlist-previous = Плейлист: назад
hotkey-audit-feature-playlist = Плейлист

# Instant replay source (CAP-N10)
sources-badge-replay = Повтор
sources-add-replay = Миттєвий повтор
sources-replay-title = Додати миттєвий повтор
sources-replay-seconds-label = Довжина ролика (секунди)
sources-replay-speed-label = Швидкість
sources-replay-speed-full = 100% (зі звуком)
sources-replay-speed-half = 50% сповільнення (без звуку)
sources-replay-speed-quarter = 25% сповільнення (без звуку)
sources-replay-note = Лишається прозорим, доки ви не запустите повтор. Увімкніть буфер повторів (Керування) і призначте клавішу Roll — повтор вирізає останні миті буфера, програє їх у програму й знову стає прозорим.
sources-replay-add = Додати повтор
properties-replay-roll = ⏵ Запустити повтор
properties-replay-note = Roll вирізає УВІМКНЕНИЙ буфер у кліп і програє його на обраній швидкості — ретаймінг, жодної інтерполяції. Сповільнення навмисно беззвучне. Перемотування й пауза працюють під час відтворення; наприкінці джерело знову прозоре.
hotkeys-replay-roll = Миттєвий повтор: запуск
hotkey-audit-action-replay-roll = Запустити миттєвий повтор

# Input overlay source (CAP-N13)
sources-badge-input = Ввід
sources-add-input-overlay = Оверлей вводу (клавіші/геймпад)
sources-input-title = Додати оверлей вводу
sources-input-layout-label = Розкладка
sources-input-layout-wasd = WASD + миша
sources-input-layout-keyboard = Компактна клавіатура + миша
sources-input-layout-gamepad = Геймпад (два стіки)
sources-input-layout-fightstick = Файтстик
sources-input-color-label = Клавіші
sources-input-accent-label = Натиснуто
sources-input-privacy-note = Приватність: ввід зчитується лише поки це джерело в ефірі у сцені, і опитуються тільки фіксовані клавіші розкладки — миттєва перевірка «чи натиснута зараз», жодних хуків. Нічого не журналюється, не зберігається й нікуди не надсилається; набраний текст ніколи не перехоплюється.
sources-input-os-note = Стан клавіатури та миші сьогодні зчитується лише у Windows — інші системи малюють клавіші ненатиснутими (сказано чесно, без імітації). Геймпади працюють усюди через бібліотеку gilrs; малюється перший підключений контролер, а без нього розкладка лишається ненатиснутою.
sources-input-add = Додати оверлей вводу

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Ефекти курсора
filters-cursorfx-hint = У Windows (де застосунок сам малює курсор) вони наносяться просто в захоплення й потрапляють у записи та трансляції. У macOS і Linux курсор накладає система, тож ці ефекти доступні лише в Windows. Зміни застосовуються одразу.
filters-cursorfx-halo = Ореол курсора
filters-cursorfx-halo-color = Колір
filters-cursorfx-halo-radius = Радіус (px)
filters-cursorfx-ripples = Хвилі від клацань
filters-cursorfx-left-color = Ліва кнопка
filters-cursorfx-right-color = Права кнопка
filters-cursorfx-keystrokes = Показ клавіш
filters-cursorfx-keystrokes-hint = Показує фіксований набір клавіш (літери, цифри, модифікатори, стрілки) біля курсора, доки їх утримують. Клавіші зчитуються лише коли це ввімкнено, малюються просто в кадр і ніколи не зберігаються та не потрапляють у журнал.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Титр
sources-add-title = Титри / Табло
sources-title-title = Додати титр
sources-title-template-label = Почати з
sources-title-template-lower-third = Нижня плашка (смуга + ім'я + підпис)
sources-title-template-scoreboard = Табло (плашка + 4 комірки)
sources-title-template-blank = Порожнє полотно
sources-title-width-label = Ширина полотна
sources-title-height-label = Висота полотна
sources-title-template-name = Ім'я
sources-title-template-subtitle = Титул
sources-title-template-home = ГОСПОДАРІ
sources-title-template-away = ГОСТІ
sources-title-note = Багатошарові титри (текст / зображення / плашка) з анімацією входу/виходу, складаються локально — без браузерного джерела. Шари, прив'язки до файлів і {"{{"}змінних{"}}"} та живе керування — у Властивостях джерела.
sources-title-add = Додати титр
properties-title-layers = Шари (малюються по черзі — пізніші рядки зверху)
properties-title-kind-text = Текст
properties-title-kind-image = Зображення
properties-title-kind-rect = Плашка
properties-title-x = X
properties-title-y = Y
properties-title-outline = Обведення (px)
properties-title-outline-color = Обведення
properties-title-shadow = Тінь
properties-title-animation = Анімація входу/виходу
properties-title-anim-none = Немає (стик)
properties-title-anim-fade = Розчинення
properties-title-anim-slide-left = Зсув ліворуч
properties-title-anim-slide-up = Зсув угору
properties-title-anim-wipe = Шторка
properties-title-duration = Тривалість (мс)
properties-title-fire-in = ▶ Запустити вхід
properties-title-fire-out = ◼ Запустити вихід
properties-title-set-live = В ефір
properties-title-set-live-note = Одразу надсилає цей текст у ЖИВИЙ титр — без «Застосувати», без перезапуску
properties-title-up = Шар вище
properties-title-down = Шар нижче
properties-title-remove = Видалити шар
properties-title-add-text = + Текст
properties-title-add-image = + Зображення
properties-title-add-rect = + Плашка
properties-title-note = Вхід/вихід і «В ефір» керують ПРАЦЮЮЧИМ титром; зміни шарів набирають чинності після «Застосувати» (титр перезапускається і знову входить). Текстові комірки можуть прив'язуватися до відстежуваного файлу (комірка CSV / значення JSON / весь файл) і підставляти {"{{"}змінні{"}}"} — «В ефір» перемагає обидва.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN-інджест (слухач SRT/RTMP)
sources-lan-title = Додати слухач LAN-інджесту
sources-lan-protocol-label = Протокол
sources-lan-protocol-srt = SRT (з шифруванням — рекомендовано)
sources-lan-protocol-rtmp = RTMP (без автентифікації)
sources-lan-port-label = Порт (1024–65535)
sources-lan-passphrase-label = Парольна фраза (порожньо = відкрито)
sources-lan-passphrase-hint = Парольні фрази SRT — від 10 до 79 символів; відправник має використати ту саму.
sources-lan-open-warning = Без парольної фрази: будь-хто в цій мережі може живити це джерело, без шифрування. Задайте її, якщо мережа не лише ваша.
sources-lan-rtmp-warning = В RTMP немає автентифікації — будь-хто в цій мережі може надсилати на цей порт. Краще SRT з парольною фразою.
sources-lan-url-label = Спрямуйте застосунок відправника на
sources-lan-qr-aria = QR-код адреси інджесту
sources-lan-note = Лише LAN: слухає локальну адресу цієї машини, лише поки джерело існує, і ніколи не торкається інтернету — ніщо не залишає машину, доки відправник у вашій мережі не надішле першим. Декодування йде через чітко позначений компонент ffmpeg. Полотно показує цю адресу, доки відправник не під'єднається.
sources-lan-add = Почати слухати
properties-lan-note = Застосування зміни протоколу, порту чи парольної фрази перезапускає слухач — відправник має під'єднатися знову. Потік вписується в полотно 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Лінк
sources-add-freally-link = Freally Link (інший екземпляр)
sources-link-title = Додати Freally Link
sources-link-about = Приймає програму іншого екземпляра Freally Capture — відео та майстер-звук — через вашу власну мережу. Спершу ввімкніть «Вихід Freally Link» на передавальному екземплярі. v1 передає motion-JPEG по TCP: чудово в дротовій LAN чи хорошому Wi-Fi, чесно щодо смуги на слабких з'єднаннях.
sources-link-scan = Сканувати LAN
sources-link-scanning = Сканування…
sources-link-none = Виходів Freally Link не знайдено. Увімкніть «Вихід Freally Link» на іншому екземплярі (Керування → LAN-панель) або введіть його адресу нижче.
sources-link-host = Адреса
sources-link-port = Порт
sources-link-key = Ключ сполучення
sources-link-key-hint = Ключ із налаштувань «Вихід Freally Link» відправника — без нього відправник не віддасть жодного кадру.
sources-link-add = Додати лінк
properties-link-note = Без з'єднання джерело показує заставку «підключення» і саме повторює спроби зі зростаючою паузою — воно ніколи не застигає на старому кадрі. Один приймач на відправника; зайнятого відправника ввічливо пробують знову.
link-title = Вихід Freally Link
link-about = Поділіться програмою цього екземпляра — відео та майстер-звуком — з ОДНИМ іншим Freally Capture у вашій власній мережі; там вона з'явиться як джерело «Freally Link» (стримінг із двох ПК, додаткові монітори). Типово вимкнено; ніщо не оголошується й не слухає, доки не ввімкнете. v1 передає motion-JPEG + нестиснений звук по TCP — для дротової LAN чи хорошого Wi-Fi, ніколи для інтернету.
link-enable = Ділитися програмою в моїй мережі
link-name = Назва екземпляра
link-key = Ключ сполучення
link-key-hint = Щонайменше 8 символів — приймачі мають ввести цей ключ, перш ніж буде передано бодай один кадр.
link-lan-warning = ⚠ Приймачі мають пред'явити ключ сполучення, перш ніж щось буде передано, але сам потік у v1 не шифрується — використовуйте лише в довіреній мережі.
link-serving = Приймачі знайдуть цей екземпляр через «Сканувати LAN» або додадуть вручну за адресою:
link-off-hint = Увімкніть спільний доступ, щоб відкрити порт і оголошувати цей екземпляр під час сканування LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Меню застосунку
menu-file = Файл
menu-edit = Редагування
menu-view = Вигляд
menu-docks = Доки
menu-profile = Профіль
menu-collection = Колекція сцен
menu-tools = Інструменти
menu-help = Довідка
menu-rename = Перейменувати
menu-remove = Вилучити
menu-import = Імпорт
menu-export = Експорт
menu-file-show-recordings = Показати записи
menu-file-remux = Перепакувати в MP4…
menu-file-settings = Налаштування…
menu-file-show-settings-folder = Показати теку налаштувань
menu-file-exit = Вихід
menu-edit-undo = Скасувати
menu-edit-redo = Повторити
menu-edit-history = Історія змін…
menu-edit-copy-transform = Копіювати трансформацію
menu-edit-paste-transform = Вставити трансформацію
menu-edit-copy-filters = Копіювати фільтри
menu-edit-paste-filters = Вставити фільтри
menu-edit-transform = Трансформація…
menu-edit-lock-preview = Заблокувати попередній перегляд
menu-view-fullscreen = Повноекранний інтерфейс
menu-stats-dock = Панель статистики
menu-view-multiview = Монітор мультив’ю…
menu-view-projectors = Проектори…
menu-view-source-health = Стан джерел…
menu-view-still = Зробити стоп-кадр
menu-docks-browser = Браузерні доки…
menu-docks-lock = Заблокувати доки
menu-docks-reset = Скинути розташування доків
menu-profile-manage = Керування профілями…
menu-collection-manage = Керування колекціями сцен…
menu-collection-import-obs = Імпорт з OBS…
menu-collection-missing = Перевірити відсутні файли…
menu-tools-wizard = Запустити майстер налаштування
menu-tools-wizard-title = Майстер налаштування запускається під час першого запуску; повторний запуск поки недоступний.
menu-tools-automation = Правила автоматизації та макроси…
menu-tools-rundown = Показати сценарій ефіру…
menu-tools-hotkeys = Мапа гарячих клавіш…
menu-tools-av-sync = Калібрування синхронізації A/V…
menu-tools-scripts = Скрипти Lua…
menu-tools-components = Компоненти…
menu-tools-midi = MIDI-керування…
menu-tools-ptz = PTZ-камери…
menu-tools-remote = API віддаленого керування…
menu-tools-panel = LAN-панель і tally…
menu-help-portal = Портал довідки
menu-help-website = Відвідати вебсайт
menu-help-discord = Приєднатися до сервера Discord
menu-help-bug = Повідомити про ваду…
menu-help-updates = Перевірити оновлення…
menu-help-whats-new = Що нового
menu-help-about = Про програму…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Категорії налаштувань
settings-cat-general = Загальні
settings-cat-appearance = Вигляд
settings-cat-streaming = Трансляція
settings-cat-output = Вивід
settings-cat-replay = Повтор
settings-cat-hotkeys = Гарячі клавіші
settings-cat-network = Мережа
settings-cat-accessibility = Доступність
settings-cat-about = Про застосунок
settings-ok = Гаразд
settings-cancel = Скасувати
settings-apply = Застосувати
settings-save = Зберегти
settings-loading = Завантаження налаштувань…
settings-hotkeys-filter = Фільтр гарячих клавіш
settings-hotkeys-filter-placeholder = Введіть текст, щоб відфільтрувати дії або клавіші…
settings-hotkeys-no-match = Жодна гаряча клавіша не відповідає “{ $query }”.
settings-hotkey-none = Немає
settings-hotkey-group-ctrl = Ctrl + клавіша
settings-hotkey-group-ctrl-shift = Ctrl + Shift + клавіша
settings-hotkey-group-ctrl-alt = Ctrl + Alt + клавіша
settings-hotkey-group-function = Функціональні клавіші
settings-hotkey-group-numpad = Цифрова клавіатура
settings-panic-section = Екран паніки
settings-meter-section = Індикатори рівня мікшера
settings-meter-note = Кольори, якими проходять індикатори рівня аудіомікшера — від тиші до перевантаження. Пресет для людей із дальтонізмом використовує градієнт від синього до помаранчевого, помітний за червоно-зеленої колірної сліпоти.
settings-meter-preset = Кольори індикатора
settings-meter-preset-default = Зелений / жовтий / червоний
settings-meter-preset-colorblind = Для дальтоніків (синій / помаранчевий)
settings-meter-preset-custom = Власні
settings-meter-low = Звичайний
settings-meter-mid = Гучний
settings-meter-high = Перевантаження
settings-meter-preview = Попередній перегляд

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Що нового
whats-new-loading = Завантаження приміток до випуску…
whats-new-version = Що нового у версії { $version }
whats-new-empty = Немає приміток для цього випуску.
filters-name-directional-blur = Спрямоване розмиття
filters-name-radial-blur = Радіальне розмиття
filters-name-zoom-blur = Розмиття масштабуванням
filters-name-pixelate = Пікселізація
filters-angle = Кут (°)
filters-center-x = Центр X
filters-center-y = Центр Y
filters-block-size = Розмір блоку (px)
filters-name-freeze = Заморозити
filters-freeze-hint = Коли ввімкнено, це джерело утримує останній кадр — програма, попередній перегляд, запис і трансляція завмирають разом. Перемкніть цей фільтр, щоб заморозити або розморозити.
transform-3d = 3D-нахил
transform-rotation-x = Нахил X (°)
transform-rotation-y = Нахил Y (°)
transform-perspective = Перспектива
transform-reveal = Показати/сховати
transform-reveal-ms = Поява (мс)
sources-clone-title = Клон (те саме джерело, власні фільтри)
sources-clone-item = Клонувати { $name }
menu-tools-downstream = Вихідні кеї…
menu-tools-transition-rules = Правила переходів…
dsk-title = Вихідні кеї
dsk-hint = Накладання, скомпоновані на виході програми — над кожною сценою, і вони лишаються на місці під час перемикання сцен (логотип, значок В ЕФІРІ, нижня плашка). Верх списку малюється спереду.
dsk-empty = Кеїв поки немає — додайте джерело, щоб накласти його на кожну сцену.
dsk-enable = Увімкнути цей кей
dsk-move-up = Вгору (наверх)
dsk-move-down = Вниз
dsk-remove = Видалити кей
dsk-opacity = Непрозорість
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Масштаб
dsk-add = + Додати кей
transition-rules-title = Правила переходів
transition-rules-hint = Задайте парі сцен власний перехід. Коли ви переходите з першої сцени до другої, замість типових значень використовуються цей тип і тривалість (правило Стінгер/Зображення все одно використовує файл, заданий в елементах керування переходом).
transition-rules-empty = Ще немає правил — кожна пара сцен використовує стандартний перехід.
transition-rules-from = З
transition-rules-to = До
transition-rules-kind = Перехід
transition-rules-duration = Тривалість (мс)
transition-rules-add = Додати правило
transition-rules-remove = Видалити правило
