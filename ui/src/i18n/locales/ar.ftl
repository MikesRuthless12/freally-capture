# Freally Capture — ar
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = وضع الاستوديو
toggle-on = تشغيل
toggle-off = إيقاف
stats = الإحصائيات
core-ok = النواة سليمة
hide-stats-dock = إخفاء لوحة الإحصائيات
show-stats-dock = إظهار لوحة الإحصائيات


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = تعذّر حفظ الإعدادات — لن يبقى التغيير بعد إعادة التشغيل.
studio-mode-leave = مغادرة وضع الاستوديو
studio-mode-enter-title = وضع الاستوديو — حرّر مشهد المعاينة ثم ادفعه إلى البرنامج بانتقال
vertical-canvas-title = لوحة الإخراج الثانية (عمودية 9:16) — قابلة للتسجيل والبث بشكل مستقل
app-version = v{ $version }
core-error = خطأ في النواة
core-unreachable = تعذّر الوصول إلى النواة (وضع المتصفح)
connecting-to-core = جارٍ الاتصال بالنواة…
filters-source-fallback = مصدر

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = معاينة البرنامج
preview-program-output = مخرج البرنامج
preview-canvas-editor = محرّر اللوحة
preview-px-to-edge-label = البكسلات إلى حواف الإطار
preview-px-to-edge = بكسل إلى الحافة يسار { $left } · أعلى { $top } · يمين { $right } · أسفل { $bottom }
preview-program-heading = البرنامج
preview-no-gpu = لم يُعثر على مُحوّل GPU صالح للاستخدام — لا يمكن تشغيل المُركِّب على هذا الجهاز.
preview-starting-compositor = جارٍ بدء المُركِّب…
preview-empty-scene = هذا المشهد فارغ — أضف مصدرًا في المصادر، ثم اسحبه وحجّمه ودوّره هنا على اللوحة مباشرة.
preview-fps = { $fps } fps
preview-dropped = { $dropped } مُسقَطة

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = تم استلام رابط الدعوة
remote-join-with-webcam = انضمّ بكاميرا الويب
remote-dismiss = تجاهل
remote-hosting-guest = استضافة ضيف عن بُعد
remote-you-are-guest = أنت ضيف عن بُعد
remote-share-view-title = شارك شاشتك مع تطبيق الضيف (يرى معاينتك مباشرة)
remote-stop-sharing-view = إيقاف مشاركة العرض
remote-share-my-view = شارك عرضي
remote-allow-center-title = اسمح للضيف بتبديل العرض الذي يحتل المركز (تبقى المتحكّم ويمكنك التبديل في أي وقت)
remote-guest-switching = تبديل الضيف:
remote-stop-screen = إيقاف الشاشة
remote-share-screen = مشاركة الشاشة
remote-share-screen-title-guest = شارك شاشتك مع المضيف (تصبح مصدرًا يمكنه توسيطه)
remote-center-request-label = طلب توسيط العرض
remote-center = توسيط
remote-center-cam-title = اطلب من المضيف توسيط كاميرتك
remote-center-my-cam = كاميرتي
remote-center-screen-title = اطلب من المضيف توسيط شاشتك المُشارَكة
remote-center-my-screen = شاشتي
remote-center-host-title = أعِد المركز إلى عرض المضيف
remote-center-host-view = عرض المضيف
remote-end-session = إنهاء الجلسة
remote-leave = مغادرة
remote-host-view-heading = عرض المضيف
remote-host-shared-view-label = العرض المُشارَك للمضيف
remote-guest-position-label = موضع الضيف
remote-guest-label = ضيف
remote-put-guest = ضع الضيف { $position }
remote-remove-title = أزِل الضيف — يمكنه إعادة الانضمام بالرابط نفسه
remote-remove = إزالة
remote-ban-title = احظر الضيف — يمنعه ويُبطل رابط الدعوة
remote-ban = حظر
remote-guest-self-muted = الضيف كتم نفسه
remote-unmute-guest = إلغاء كتم الضيف
remote-mute-guest = كتم الضيف
remote-muted-by-host = مكتوم من المضيف
remote-unmute-mic = إلغاء كتم الميكروفون
remote-mute-mic = كتم الميكروفون
remote-waiting-for-host = بانتظار المضيف


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = مصدر
sources-fallback-video = فيديو
sources-fallback-error = خطأ
sources-kind-unknown = ؟
sources-missing-source = (مصدر مفقود)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = شاشة
sources-badge-window = نافذة
sources-badge-portal = بوابة
sources-badge-camera = كاميرا
sources-badge-image = صورة
sources-badge-media = وسائط
sources-badge-guest = ضيف
sources-badge-color = لون
sources-badge-text = نص
sources-badge-scene = مشهد
sources-badge-slides = شرائح
sources-badge-chat = دردشة
sources-badge-audio-in = دخل صوت
sources-badge-audio-out = خرج صوت
sources-badge-app-audio = صوت تطبيق
sources-badge-test-bars = أشرطة
sources-badge-test-grid = شبكة
sources-badge-test-sweep = مسح
sources-badge-test-tone = نغمة
sources-badge-test-sync = تزامن
sources-badge-timer = مؤقّت

# Add-source menu items
sources-add-display = التقاط الشاشة
sources-add-window = التقاط نافذة
sources-add-game = التقاط لعبة (اقرأ أولًا)
sources-add-webcam = جهاز التقاط فيديو
sources-add-image = صورة
sources-add-media = وسائط (ملف فيديو/صورة)
sources-add-remote-guest = ضيف عن بُعد (تجربة P2P)
sources-add-color = لون
sources-add-text = نص
sources-add-timer = مؤقّت / ساعة
sources-add-nested-scene = مشهد متداخل
sources-add-slideshow = عرض شرائح صور
sources-add-chat-overlay = تراكب الدردشة المباشرة
sources-add-test-signal = إشارة اختبار
sources-add-audio-input = التقاط دخل الصوت
sources-add-audio-output = التقاط خرج الصوت
sources-add-app-audio = صوت التطبيق (Windows)
sources-add-existing = مصدر موجود…

# Panel header + toolbar buttons
sources-panel-title = المصادر
sources-group-title = جمّع المصادر — اختر عنصرين أو أكثر، ثم أنشئ مجموعة؛ العناصر المُجمَّعة تتحرك وتظهر/تختفي معًا
sources-group-aria = تجميع المصادر
sources-arrange = ترتيب: شاشة + زوايا
sources-add-source = إضافة مصدر
sources-browser-source-note = يأتي مصدر المتصفح كمكوّن مستقل عند الطلب (محرك Chromium بحجم ~180 ميجابايت — لا يُضمَّن أبدًا). حاليًا: التقط نافذة متصفح حقيقية عبر التقاط نافذة + مفتاح كروما/لون، أو افتح الدردشة/التنبيهات كرصيف (عناصر التحكم ← الأرصفة).

# Empty state
sources-empty = لا مصادر في هذا المشهد — أضف التقاط شاشة أو نافذة أو كاميرا ويب أو صورة أو لون أو نص عبر "+". اسحبها وحجّمها ودوّرها على اللوحة؛ الأزرار الجانبية تعيد ترتيب المكدس.

# Per-row controls
sources-already-in-group = موجود بالفعل في { $name }
sources-pick-for-new-group = اختر للمجموعة الجديدة
sources-pick-item-for-group = اختر { $name } للمجموعة الجديدة
sources-hide = إخفاء
sources-show = إظهار
sources-hide-item = إخفاء { $name }
sources-show-item = إظهار { $name }
sources-unfocus-title = إلغاء التركيز — استعادة التخطيط
sources-focus-title = تركيز — ملء اللوحة (إبراز المتحدث)
sources-unfocus-item = إلغاء تركيز { $name }
sources-focus-item = تركيز { $name }
sources-center-title = توسيط — اجعل هذا هو العرض المركزي المُشارَك (تنتقل الكاميرات إلى الشريط)
sources-center-item = توسيط { $name }
sources-rename-item = إعادة تسمية { $name }
sources-in-group = في المجموعة { $name }

# Row status + retry
sources-retry-error = إعادة المحاولة — { $message }
sources-retry-item = إعادة محاولة { $name }
sources-status-error = الحالة: خطأ
sources-open-privacy-title = افتح إعدادات خصوصية macOS لهذا الإذن
sources-open-privacy-item = افتح إعدادات الخصوصية لـ { $name }
sources-privacy-settings-button = الإعدادات
sources-status-starting = جارٍ البدء…
sources-status-live = مباشر
sources-status-aria = الحالة: { $state }

# Media row pause/resume
sources-media-resume-title = استئناف الفيديو (مباشر على البث)
sources-media-pause-title = إيقاف الفيديو مؤقتًا — تجميد الإطار مع كتم الصوت، مباشر على البث
sources-media-resume-item = استئناف { $name }
sources-media-pause-item = إيقاف { $name } مؤقتًا

# Hover controls
sources-unlock = فتح القفل
sources-lock = قفل
sources-unlock-item = فتح قفل { $name }
sources-lock-item = قفل { $name }
sources-raise-title = رفع في المكدس
sources-raise-item = رفع { $name }
sources-lower-title = خفض في المكدس
sources-lower-item = خفض { $name }
sources-filters-title = الفلاتر والمزج
sources-filters-item = فلاتر { $name }
sources-properties-title = الخصائص
sources-properties-item = خصائص { $name }
sources-remove-title = إزالة من هذا المشهد
sources-remove-item = إزالة { $name }

# Grouping footer
sources-create-group = إنشاء مجموعة ({ $count })
sources-cancel = إلغاء

# Groups list
sources-groups-aria = مجموعات المصادر
sources-hide-group = إخفاء المجموعة
sources-show-group = إظهار المجموعة
sources-item-count = · { $count } عناصر
sources-ungroup-title = فك التجميع — تبقى العناصر في أماكنها
sources-ungroup-item = فك تجميع { $name }

# Live Chat Overlay picker
sources-chat-title = إضافة تراكب دردشة مباشرة
sources-chat-youtube-label = YouTube — رابط القناة أو المشاهدة أو live_chat (بلا مفتاح، بلا تسجيل دخول)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  أو رابط watch?v=
sources-chat-twitch-label = Twitch — اسم القناة (قراءة مجهولة، بلا حساب)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — مُعرّف القناة (نقطة نهاية عامة، أفضل جهد)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = تظهر الرسائل مع طابع زمني h:mm:ss ص/م متحرك على خلفية شفافة (افتراضيًا أعلى اليمين؛ اسحبها إلى أي مكان). فيضان الدردشة يُقادِم الأسطر القديمة فقط — لا يمكنه أبدًا تعطيل البث أو التسجيل. تحتاج دردشة Facebook إلى رمز Graph الخاص بك ولم تُنفَّذ بعد — ليست مطلوبة أبدًا ولا تعيق المنصات أعلاه.
sources-chat-add = إضافة تراكب الدردشة
sources-chat-default-name = دردشة مباشرة

# Image Slideshow picker
sources-slideshow-title = إضافة عرض شرائح صور
sources-slideshow-empty = لا صور بعد — يضيفها التصفّح بالترتيب.
sources-slideshow-remove-slide = إزالة الشريحة { $number }
sources-slideshow-browse = تصفّح الصور…
sources-slideshow-per-slide-label = لكل شريحة (مللي ثانية)
sources-slideshow-crossfade-label = تلاشٍ متقاطع (مللي ثانية، 0 = قطع)
sources-slideshow-loop-label = تكرار (إيقاف = تثبيت الشريحة الأخيرة)
sources-slideshow-shuffle-label = خلط كل دورة
sources-slideshow-note = يمزج التلاشي المتقاطع الصور المتساوية الحجم؛ الأحجام المختلفة تُقطَع بحدّة عند الحدود (بلا إعادة تحجيم صامتة).
sources-slideshow-add = إضافة عرض الشرائح ({ $count })

# Nested Scene picker
sources-nested-title = إضافة مشهد متداخل
sources-nested-empty = لا مشهد آخر للتداخل — أضف مشهدًا ثانيًا أولًا.
sources-nested-scene-name = مشهد: { $name }
sources-nested-note = يُعرَض المشهد المتداخل مباشرةً بحجم لوحة البرنامج ويتبع تعديلاته الخاصة؛ التحويلات والفلاتر والمزج تُطبَّق عليه كأي مصدر. تنضم مصادره الصوتية إلى المزيج عندما يكون مشهد يعرضه هو البرنامج.

# Display / Window capture picker
sources-capture-display-title = إضافة التقاط شاشة
sources-capture-window-title = إضافة التقاط نافذة
sources-capture-looking = جارٍ البحث عن المصادر…
sources-capture-none-displays = لا شيء لالتقاطه هنا — لم يُعثر على شاشات.
sources-capture-none-windows = لا شيء لالتقاطه هنا — لم يُعثر على نوافذ.
sources-capture-portal-note = على Wayland، يختار مربع حوار النظام الشاشة أو النافذة — لا يمكن للتطبيقات الالتقاط عالميًا هناك، فهذا هو المسار الصادق (والوحيد).
sources-capture-window-note = تتحدّث المعاينات مباشرة. النافذة المُصغَّرة تُظهر آخر إطار لها (أو لا شيء) حتى تستعيدها.
sources-thumb-no-preview = لا معاينة
sources-thumb-loading = جارٍ التحميل…

# Video Capture Device picker
sources-webcam-title = إضافة جهاز التقاط فيديو
sources-webcam-looking = جارٍ البحث عن الكاميرات…
sources-webcam-none = لم يُعثر على كاميرات أو بطاقات التقاط.
sources-webcam-format-label = التنسيق
sources-webcam-format-auto-loading = تلقائي (جارٍ تحميل التنسيقات…)
sources-webcam-format-auto = تلقائي (أعلى دقة)
sources-webcam-card-presets-label = إعدادات البطاقة المسبقة:
sources-webcam-preset-title = اختر وضع { $label } الذي تعلن عنه هذه البطاقة
sources-webcam-add = إضافة كاميرا

# Audio Input / Output capture picker
sources-audio-output-title = إضافة التقاط خرج الصوت
sources-audio-input-title = إضافة التقاط دخل الصوت
sources-audio-default-output = الخرج الافتراضي (ما تسمعه)
sources-audio-default-input = الدخل الافتراضي
sources-audio-looking = جارٍ البحث عن أجهزة الصوت…
sources-audio-none-output = لم يُعثر على جهاز التقاط صوت سطح المكتب هنا.
sources-audio-none-input = لم يُعثر على ميكروفونات أو مداخل خطية.
sources-audio-input-note = تحصل شرائح المازج على مقياس صوت ومُنزلق وكتم ومراقبة وفلاتر (إزالة ضوضاء، بوابة، ضاغط…) وتعيين مسار. كل شيء يبقى على هذا الجهاز.

# Application Audio picker
sources-appaudio-title = إضافة صوت تطبيق
sources-appaudio-looking = جارٍ البحث عن التطبيقات التي تُصدر صوتًا…
sources-appaudio-none = لا تطبيقات تُصدر صوتًا الآن — ابدأ التشغيل في التطبيق ثم حدّث.
sources-appaudio-refresh = ⟳ تحديث
sources-appaudio-note = يلتقط صوت ذلك التطبيق بالضبط — مع مقياسه ومُنزلقه وكتمه وفلاتره ومساره الخاص.

# Game Capture picker
sources-game-title = التقاط لعبة
sources-game-checking = جارٍ الفحص…
sources-game-use-portal = استخدم التقاط الشاشة (البوابة)
sources-game-use-window = استخدم التقاط النافذة بدلًا من ذلك

# Image picker
sources-image-title = إضافة صورة
sources-image-file-label = ملف صورة (PNG، JPEG، BMP، GIF، WebP…)
sources-image-add = إضافة صورة

# Path field
sources-browse = تصفّح…

# Media picker
sources-media-title = إضافة وسائط
sources-media-file-label = ملف وسائط (mp4، mkv، webm، mov، أو .frec، أو صورة)
sources-media-loop-label = تكرار (إعادة التشغيل من البداية عند النهاية)
sources-media-note = يُشغَّل .frec عبر كودك freally-video المملوك — لا شيء للتنزيل. تُفكَّك تنسيقات الشبكة (mp4/mkv/webm/…) عبر مكوّن FFmpeg عند الطلب؛ يصل صوتها إلى المازج كشريحة خاصة به.
sources-media-add = إضافة وسائط

# Invite expiry options
sources-ttl-15min = 15 دقيقة
sources-ttl-30min = 30 دقيقة
sources-ttl-1hour = ساعة واحدة
sources-ttl-1day = يوم واحد

# Remote Guest form
sources-remote-copy-failed = تعذّر النسخ — حدّد الرابط وانسخه يدويًا
sources-remote-join-failed = فشل الانضمام: { $error }
sources-remote-title = ضيف عن بُعد (تجربة P2P)
sources-remote-host-heading = مضيف — ادعُ ضيفًا
sources-remote-start-hosting = بدء الاستضافة
sources-remote-expires-label = ينتهي
sources-remote-invite-expiry-aria = انتهاء صلاحية الدعوة
sources-remote-invite-link-aria = رابط الدعوة
sources-remote-copied = تم النسخ ✓
sources-remote-copy = نسخ
sources-remote-share-note = شارك هذا الرابط (Discord / رسالة نصية / بريد إلكتروني). يحمل جلستك وتنتهي صلاحيته كما هو محدّد. يفتحه الضيف وينضم بكاميرا الويب الخاصة به.
sources-remote-qr-note = امسح على الهاتف للانضمام مباشرةً من المتصفح — كاميرا + ميكروفون، بلا تثبيت. رابط freally:// القابل للنسخ أعلاه يُفتح في Freally Capture على جهاز مثبَّت عليه.
sources-remote-guest-heading = ضيف — انضمّ بدعوة
sources-remote-paste-placeholder = الصق رابط الدعوة
sources-remote-invite-input-aria = رابط الدعوة أو مُعرّف الجلسة
sources-remote-join = انضمّ بكاميرا الويب
sources-remote-session-note = تبقى عناصر التحكم بالجلسة المباشرة (كتم، إنهاء) على الشريط أعلى النافذة الرئيسية — يمكنك إغلاق هذا المربع الحواري.
sources-remote-stop-session = إيقاف الجلسة

# Invite QR
sources-invite-qr-aria = رمز QR لرابط الدعوة

# Remote device pickers
sources-devices-output-unavailable = توجيه الخرج غير متاح — يُشغَّل على الجهاز الافتراضي
sources-devices-mic-test-failed = فشل اختبار الميكروفون: { $error }
sources-devices-heading = أجهزة صوت الجلسة
sources-devices-microphone-label = الميكروفون
sources-devices-microphone-aria = ميكروفون الجلسة
sources-devices-system-default = افتراضي النظام
sources-devices-output-label = الخرج
sources-devices-output-aria = خرج صوت الجلسة
sources-devices-stop-test = إيقاف الاختبار
sources-devices-test = اختبار — اسمع نفسك
sources-devices-testing-note = تحدّث في الميكروفون — أنت تسمع الأجهزة المحددة مباشرة
sources-devices-idle-note = يُعيد ميكروفونك إلى الخرج (السماعات تتجنّب التغذية الراجعة)

# TURN relay section
sources-turn-save-failed = تعذّر الحفظ: { $error }
sources-turn-summary = الشبكة — مُرحِّل TURN اختياري (متقدّم)
sources-turn-note-1 = تتصل الجلسات مباشرةً (P2P) — مجانًا، بلا حاجة لمُرحِّل. إذا كان الطرفان خلف NAT صارم فقد يفشل المسار المباشر؛ عندها يحمل الوسائط مُرحِّل TURN تُشغِّله بنفسك. تخطّي هذا لا بأس به — معظم الاتصالات تعمل مباشرةً فقط.
sources-turn-note-2 = خيار مجاني: يشغّل Oracle Cloud "Always Free" خادم coturn بلا تكلفة (ملاحظة: يطلب Oracle بطاقة ائتمان عند التسجيل، لكن هيئة Always-Free تبقى مجانية). الخطوات: 1) أنشئ الجهاز الافتراضي المجاني، 2) ثبّت coturn، 3) افتح UDP 3478، 4) عيّن مستخدمًا/كلمة مرور، 5) أدخل turn:your-vm-ip:3478 + بيانات الاعتماد هنا. تبقى بيانات اعتمادك في ملف إعداداتك المحلي ولا تُسجَّل أبدًا.
sources-turn-url-label = عنوان TURN
sources-turn-url-placeholder = turn:host:3478 (فارغ = مباشر فقط)
sources-turn-url-aria = عنوان TURN
sources-turn-username-label = اسم المستخدم
sources-turn-username-aria = اسم مستخدم TURN
sources-turn-credential-label = بيانات الاعتماد
sources-turn-credential-aria = بيانات اعتماد TURN
sources-turn-note-3 = يعمل المُرحِّل بمجرد تعيين الحقول الثلاثة (يتطلب خادم TURN بيانات الاعتماد) ويُطبَّق على الجلسة التالية التي تبدأها أو تنضم إليها. تحقّق منه باستدعاء اختباري عبر المُرحِّل فقط بين جهازيك.
sources-turn-settings-unavailable = الإعدادات غير متاحة (وضع المتصفح)

# Color picker
sources-color-title = إضافة لون
sources-color-label = اللون
sources-color-width-label = العرض
sources-color-height-label = الارتفاع
sources-color-add = إضافة لون
sources-testsignal-title = إضافة إشارة اختبار
sources-testsignal-pattern-label = النمط
sources-testsignal-bars = أشرطة ألوان SMPTE
sources-testsignal-grid = شبكة معايرة
sources-testsignal-sweep = مسح حركي
sources-testsignal-tone = نغمة 1 كيلوهرتز (−20 dBFS)
sources-testsignal-flash-beep = وميض + صافرة لتزامن الصوت والصورة
sources-testsignal-note = تحقّق من المشاهد والمُرمِّزات وأجهزة العرض ووجهات البث دون توصيل كاميرا. نمط الوميض + الصافرة يشغّل طاولة معايرة تزامن الصوت والصورة.
sources-testsignal-add = إضافة إشارة الاختبار
sources-timer-title = إضافة مؤقّت
sources-timer-mode-label = الوضع
sources-timer-wall-clock = ساعة حائط
sources-timer-countdown = عدّ تنازلي
sources-timer-stopwatch = ساعة إيقاف
sources-timer-since-live = الوقت منذ البث
sources-timer-since-recording = الوقت منذ التسجيل
sources-timer-note = المدة والتنسيق والتنسيق البصري وإجراءات نهاية العد كلها في خصائص المصدر.
sources-timer-add = إضافة المؤقّت

# Text picker
sources-text-title = إضافة نص
sources-text-label = النص
sources-text-default = نص
sources-text-color-label = اللون
sources-text-color-aria = لون النص
sources-text-size-label = الحجم (بكسل)
sources-text-note = عائلة الخط والمحاذاة والالتفاف والكتابة من اليمين لليسار في خصائص المصدر. الخط المُضمَّن Noto Sans (بما فيه العربية/العبرية) هو الافتراضي — مطابق على كل جهاز.
sources-text-add = إضافة نص

# Existing source picker
sources-existing-title = إضافة مصدر موجود
sources-existing-empty = لا توجد مصادر بعد — أضف واحدًا إلى أي مشهد أولًا. المصادر الموجودة مُشترَكة: إعادة تسمية أو إعادة ضبط أحدها يُحدّث كل مشهد يعرضه.

# Screen + corners layout
sources-slot-off = إيقاف
sources-slot-center = المركز (الشاشة)
sources-slot-top-left = أعلى اليسار
sources-slot-top-right = أعلى اليمين
sources-slot-bottom-left = أسفل اليسار
sources-slot-bottom-right = أسفل اليمين
sources-layout-title = ترتيب: شاشة + زوايا
sources-layout-empty = أضف التقاط شاشة وكاميرا واحدة أو أكثر إلى هذا المشهد أولًا، ثم رتّبها هنا.
sources-layout-note = ضع شاشة في المركز وحتى أربع كاميرات في الزوايا — تخطيط الشرح / البودكاست. كل زاوية تحمل كاميرا ويب أو نافذة مكالمة مُلتقَطة أو مقطع وسائط. يمكنك سحب أيٍّ منها على اللوحة لاحقًا.
sources-layout-slot-aria = خانة لـ { $name }
sources-layout-apply = تطبيق التخطيط


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = عناصر التحكم
controls-start-stop-title-stop = أوقِف التسجيل وأنهِه
controls-start-stop-title-start = سجّل تغذية البرنامج بإعداد الإعدادات ← الإخراج
controls-finalizing = ◌ جارٍ الإنهاء…
controls-stop-recording = ■ إيقاف التسجيل
controls-start-recording = ● بدء التسجيل
controls-marker-title = ضع علامة فصل في هذه اللحظة — تظهر في التسجيل (فصول mkv، أو ملف جانبي). تحتاج علامات البث على المنصة إلى حسابات المنصة، وهو ما لا يطلبه هذا التطبيق أبدًا.
controls-marker = ◈ علامة
controls-iso-lanes = مسارات ISO تُسجَّل إلى جانب البرنامج: { $count }
controls-pause-title-resume = استئناف — يستمر الملف كخط زمني واحد متصل
controls-pause-title-pause = إيقاف مؤقت — لا تُكتَب إطارات؛ الاستئناف يُكمل الملف نفسه القابل للتشغيل
controls-resume-recording = ▶ استئناف التسجيل
controls-pause-recording = ⏸ إيقاف التسجيل مؤقتًا
controls-reactions-label = التفاعلات (مدمجة في البرنامج)
controls-reactions-title = أطلِق تفاعلًا فوق البرنامج — مُسجَّل ومَبثوث، فتُظهر الإعادة اللحظة بدقة. المشاهدون في الدردشة يُطلقونها أيضًا (تطفو إيموجي تفاعلهم تلقائيًا)؛ الفيضان يحدّ فقط ما يظهر على الشاشة.
controls-react = تفاعل { $emoji }
controls-virtual-camera-title = تحتاج الكاميرا الافتراضية إلى مكوّن مُشغّل موقّع خاص بها لكل نظام (Win11 MFCreateVirtualCamera / Win10 DirectShow / امتداد macOS CoreMediaIO / Linux v4l2loopback) — تأتي كمرحلة مستقلة. نموذج التغذية جاهز لها: البرنامج أو اللوحة العمودية أو مصدر واحد، مع ميكروفون افتراضي مقترن على Windows/Linux (لا يوجد في macOS واجهة برمجية لميكروفون افتراضي — نقولها بصراحة).
controls-virtual-camera = ⌁ بدء الكاميرا الافتراضية
controls-saved = محفوظ: { $path }

# --- MixerDock.tsx ---
mixer-title = مازج الصوت
mixer-monitor-error = المراقبة: { $error }
mixer-switch-to-horizontal = التبديل إلى شرائح أفقية
mixer-switch-to-vertical = التبديل إلى شرائح عمودية
mixer-layout-aria-vertical = تخطيط المازج: عمودي — التبديل إلى أفقي
mixer-layout-aria-horizontal = تخطيط المازج: أفقي — التبديل إلى عمودي
mixer-empty = لا مصادر صوت في هذا المشهد — أضف التقاط دخل صوت (ميكروفون) أو التقاط خرج صوت (صوت سطح المكتب) عبر "+" في المصادر. تحصل الشرائح على مقياس صوت ومُنزلق وكتم ومراقبة وفلاتر وتعيين مسار.
mixer-advanced-title = الصوت — { $name }
mixer-loudness-label = جهارة البرنامج (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = الجهارة اللحظية (400 مللي ثانية)
mixer-short-term-title = الجهارة قصيرة المدى (3 ثوانٍ)
mixer-lufs-short = S { $value }
mixer-monitor-label = المراقبة
mixer-monitor-device-aria = جهاز خرج المراقبة
mixer-default-output = الخرج الافتراضي
mixer-routing = التوجيه
mixer-routing-title = توجيه خرج الصوت

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = توجيه الصوت
routing-intro = عيّن الشرائط إلى باصات المسارات، ثم أرسِل أي باص إلى مخرج فيزيائي — تغذية لمسجّل عتادي، أو سماعات في غرفة أخرى، أو استماع بسماعة الرأس على مسار احتياطي. تحتفظ المراقبة بجهازها الخاص؛ وتُضاف هذه المسارات فوق ذلك، لذا لا يتغيّر المزيج عند عدم ضبط أي منها.
routing-sends-title = الإرسال إلى المسارات
routing-no-strips = لا توجد مصادر صوت في هذا المشهد.
routing-source = المصدر
routing-track = مسار { $n }
routing-send-aria = إرسال { $source } إلى المسار { $n }
routing-outputs-title = المخارج الفيزيائية
routing-master = الرئيسي
routing-off = إيقاف
routing-default-output = الخرج الافتراضي
routing-device-aria = جهاز خرج لـ { $bus }
routing-trim-aria = تريم خرج { $bus }
routing-trim-db = { $db } dB
routing-muted = مكتوم
routing-device-error = الجهاز غير متاح

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = الخفض التلقائي
mixer-ducking-title = مصفوفة الخفض
ducking-title = مصفوفة الخفض
ducking-intro = يمكن لأي مصدر أن يخفض أي مصادر أخرى. تخفض الخلية الهدف (العمود) كلما تحدّث المُشغِّل (الصف) — اختر خلية لضبط عمقها وعتبتها وتوقيتها. كل زوج هو خفض مستقل بذاته، لذا يمكن خفض قناة واحدة بواسطة عدة مُشغِّلات في آنٍ واحد.
ducking-need-two = أضِف مصدرَي صوت على الأقل لتطبيق الخفض بينهما.
ducking-trigger-target = المُشغِّل ↓ / الهدف →
ducking-cell-aria = { $trigger } يخفض { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = إزالة
ducking-amount = المقدار
ducking-threshold = العتبة
ducking-attack = الهجوم
ducking-release = التحرير
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = تطبيع الجهارة
loudness-intro = يوجّه البرنامج تدريجيًّا نحو هدف جهارة مع سقف للذروة، حتى يصل بثّك وتسجيلاتك إلى مستوى ثابت. بطيء ولطيف — يوجّه ولا ينبض أبدًا.
loudness-enable = توجيه البرنامج نحو الهدف
loudness-target = الهدف
loudness-target-option = { $target } LUFS
loudness-ceiling = سقف الذروة (dBFS)
loudness-note = −14 LUFS تناسب التشغيل بنمط YouTube؛ −16 هدف بثّ شائع؛ −23 هي بثّ EBU R128. ويُستخدَم الهدف نفسه في إجراء التطبيع بعد التسجيل.
ltc-badge = LTC
ltc-title = رمز التوقيت SMPTE (LTC)
ltc-intro = ولّد رمز توقيت SMPTE الخطي على مسار، واقرأ LTC الوارد من أي دخل صوتي — رمز توقيت صوتي كلاسيكي لمزامنة المسجلات والكاميرات الخارجية في المونتاج. يعمل دون اتصال تمامًا.
ltc-generate = توليد LTC على مسار
ltc-track = مسار رمز التوقيت
ltc-track-option = المسار { $track }
ltc-fps = معدل الإطارات
ltc-read = قراءة LTC من
ltc-read-off = إيقاف
ltc-decoded = رمز التوقيت الوارد
ltc-no-lock = لا إشارة
ltc-note = يتزامن المولّد مع وقت اليوم، دون إسقاط إطارات. سجّل مساره (خصّصه في إعدادات الإخراج) أو وجّهه إلى مخرج لتغذية العتاد الخارجي. القارئ يقود سطر رمز التوقيت في تراكب الإحصائيات ويختم علامات الفصول.
loudness-on = LUFS { $target }
loudness-off = التطبيع متوقف

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = لوحة الأصوات
mixer-soundboard-title = لوحة الأصوات
soundboard-title = لوحة الأصوات
soundboard-add-pad = + باد
soundboard-stop-all = إيقاف الكل
soundboard-edit = تحرير
soundboard-empty = لا توجد بادات بعد — أضف واحدة وعيّن مقطعًا صوتيًا محليًا.
soundboard-new-pad = باد جديد
soundboard-no-clip = لا يوجد مقطع
soundboard-audio-files = ملفات صوتية
soundboard-name = الاسم
soundboard-choose-clip = اختر مقطعًا…
soundboard-gain = كسب
soundboard-choke = تشوك
soundboard-choke-none = بلا
soundboard-loop = تكرار
soundboard-auto-duck = خفض تلقائي
soundboard-tracks = المسارات
soundboard-hotkey = اختصار
soundboard-hotkey-placeholder = مثال Ctrl+Shift+1
soundboard-remove = إزالة

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = الإضافات
mixer-plugins-title = إضافات الصوت (CLAP / VST3)
plugins-title = إضافات الصوت
plugins-scanning = جارٍ الفحص…
plugins-none = لم يتم العثور على إضافات CLAP أو VST3 في المجلدات القياسية.

# --- StatsDock.tsx ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = الذاكرة
stats-dropped = مُسقَطة
stats-render = التصيير
stats-gpu = GPU
stats-gpu-compositing = يُركِّب
stats-gpu-idle = خامل
stats-disk = القرص
stats-disk-free = متاح
stats-disk-left = المتبقي للتسجيل
stats-disk-rate = ≈ { $rate } م.بايت/ث للتسجيل
stats-vertical-fps = 9:16 FPS
stats-targets-label = أهداف البث
stats-shared-encode = · ترميز مُشترَك
stats-starting = جارٍ بدء المُركِّب…

# --- ScenesRail.tsx ---
scenes-title = المشاهد
scenes-new-scene-name = مشهد
scenes-add = إضافة مشهد
scenes-empty = جارٍ الاتصال بنواة الاستوديو…
scenes-rename = إعادة تسمية { $name }
scenes-on-program = على البرنامج
scenes-preview = معاينة { $name }
scenes-switch-to = التبديل إلى { $name }
scenes-move-up = تحريك لأعلى
scenes-move-up-aria = تحريك { $name } لأعلى
scenes-move-down = تحريك لأسفل
scenes-move-down-aria = تحريك { $name } لأسفل
scenes-last-stays = يبقى المشهد الأخير
scenes-remove = إزالة هذا المشهد
scenes-remove-aria = إزالة { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = المستوى
channelstrip-monitor-off = المراقبة مُطفأة
channelstrip-monitor-only = مراقبة فقط (ليست في المزيج)
channelstrip-monitor-and-output = مراقبة وخرج
channelstrip-status-error = خطأ
channelstrip-status-live = مباشر
channelstrip-status-waiting-audio = بانتظار الصوت
channelstrip-status = الحالة: { $state }
channelstrip-status-waiting = بالانتظار
channelstrip-mute = كتم
channelstrip-unmute = إلغاء الكتم
channelstrip-mute-source = كتم { $name }
channelstrip-unmute-source = إلغاء كتم { $name }
channelstrip-scene-mix-on = مزيج لكل مشهد مُفعَّل — تتجاوز هذه الشريحة المزيج العام لهذا المشهد (انقر للعودة لاتّباع المزيج العام)
channelstrip-scene-mix-off = مزيج لكل مشهد — امنح هذه الشريحة مُنزلقها/كتمها الخاص للمشهد الحالي
channelstrip-scene-mix-label = مزيج لكل مشهد لـ { $name }
channelstrip-monitor-cycle = { $mode } — انقر للتبديل
channelstrip-monitor-mode = وضع مراقبة { $name }: { $mode }
channelstrip-audio-filters-title = فلاتر الصوت (إزالة ضوضاء، بوابة، ضاغط…)
channelstrip-audio-filters-label = فلاتر صوت { $name }
channelstrip-advanced-title = إزاحة المزامنة واختصارات الضغط للتحدث
channelstrip-advanced-label = إعدادات صوت متقدّمة لـ { $name }
channelstrip-track-assignment = تعيين المسار
channelstrip-track = مسار { $n }
channelstrip-track-assigned = مسار { $n } (مُعيَّن)
channelstrip-track-label = مسار { $n } لـ { $name }
channelstrip-device-error = خطأ في الجهاز
channelstrip-audio-device-error = خطأ في جهاز الصوت
channelstrip-volume-label = مستوى صوت { $name } بالديسيبل
channelstrip-ptt-hold = اضغط للتحدث: اضغط مع الاستمرار { $key }
channelstrip-sync-offset = إزاحة المزامنة (مللي ثانية، 0–{ $max } — تؤخّر هذا الصوت)
channelstrip-solo-title = سولو (PFL) — تسمع المراقبة الشرائط المعزولة فقط؛ مزيج البرنامج لا يتغير
channelstrip-solo-source = سولو { $name } (PFL)
channelstrip-pan-label = التوازن (نقرة مزدوجة للتصفير)
channelstrip-pan-aria = توازن { $name }
channelstrip-mono-label = دمج إلى أحادي
channelstrip-automix-label = مزج تلقائي (مشاركة الكسب)
channelstrip-automix-note = مشاركة الكسب: يُبقي المازج المستوى الإجمالي لجميع شرائح المزج التلقائي ثابتًا ويمرّره إلى من يتحدث — مثالي للجلسات متعددة الميكروفونات والبودكاست. مُعطّل حتى تُضيف شريحة.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = يُنتج عودة خالية من الصدى لهذا المصدر — الجميع في البرنامج باستثناء هذا المصدر نفسه. استخدمه لضيف عن بُعد كي لا يسمع صوته المتأخّر.
channelstrip-ptt-hotkey = اختصار الضغط للتحدث (صامت ما لم يُضغَط)
channelstrip-ptt-placeholder = مثال Ctrl+Shift+T أو F13
channelstrip-ptt-aria = اختصار الضغط للتحدث
channelstrip-ptm-hotkey = اختصار الضغط للكتم (صامت أثناء الضغط)
channelstrip-ptm-placeholder = مثال Ctrl+Shift+M
channelstrip-ptm-aria = اختصار الضغط للكتم
channelstrip-hotkeys-note = تعمل الاختصارات أثناء تركيز تطبيقات أخرى. على Linux/Wayland، قد تكون الاختصارات العامة غير متاحة — هذا قيد من المُركِّب، نقوله بصراحة.
channelstrip-apply = تطبيق

# --- LiveButton.tsx ---
livebutton-failure-ended = انتهى البث
livebutton-title-live = أنهِ البث — كل هدف (يستمر التسجيل الجاري)
livebutton-title-offline = ابدأ البث المباشر لكل هدف مُفعَّل في الإعدادات ← البث
livebutton-end-stream = ■ إنهاء البث
livebutton-aria-reconnecting = جارٍ إعادة الاتصال
livebutton-aria-live = مباشر
livebutton-badge-retry = محاولة { $n }
livebutton-badge-live = مباشر
livebutton-go-live = ⦿ بث مباشر

# --- RecDot.tsx ---
recdot-paused-aria = التسجيل متوقّف مؤقتًا
recdot-recording-aria = جارٍ التسجيل
recdot-tracks-one = تسجيل { $count } مسار صوتي
recdot-tracks-other = تسجيل { $count } مسارات صوتية
recdot-paused = متوقّف مؤقتًا

# --- ReplayControls.tsx ---
replaycontrols-saved = تم حفظ الإعادة — { $name }
replaycontrols-failure-stopped = توقّف المخزن
replaycontrols-title-disarm = إلغاء تسليح مخزن الإعادة (يُسقِط السجل غير المحفوظ)
replaycontrols-title-arm = سلّح مخزن الإعادة المتدحرج — يُبقي آخر N ثانية جاهزة للحفظ (ترميزه الخفيف الخاص؛ البث والتسجيل لا يتأثّران)
replaycontrols-replay-seconds = ⟲ إعادة { $seconds }ث
replaycontrols-arm = ⟲ تسليح مخزن الإعادة
replaycontrols-save-title = احفظ آخر N ثانية إلى مجلد التسجيلات (متاح أيضًا على اختصار حفظ الإعادة)
replaycontrols-save = ⤓ حفظ

# --- PropertiesDialog.tsx ---
properties-title = الخصائص — { $name }
properties-name = الاسم
properties-cancel = إلغاء
properties-apply = تطبيق
properties-youtube = YouTube — رابط القناة / المشاهدة / live_chat (بلا مفتاح، بلا تسجيل دخول، أبدًا)
properties-twitch = Twitch — اسم القناة (مجهول)
properties-kick = Kick — مُعرّف القناة (نقطة نهاية عامة)
properties-width-px = العرض (بكسل)
properties-lines = الأسطر
properties-font-px = الخط (بكسل)
properties-images = ملفات الصور (مسار واحد لكل سطر، تُعرَض بالترتيب)
properties-per-slide = لكل شريحة (مللي ثانية)
properties-crossfade = تلاشٍ متقاطع (مللي ثانية، 0 = قطع)
properties-loop-slideshow = تكرار (إيقاف = تثبيت الشريحة الأخيرة)
properties-shuffle = خلط كل دورة
properties-nested-scene = المشهد الذي يُركِّبه هذا المصدر (يُرفَض مشهد يحتوي هذا بالفعل)
properties-portal-note = تختار بوابة ScreenCast في Wayland الشاشة أو النافذة في مربع حوار النظام كل مرة يبدأ فيها هذا المصدر — لا شيء لضبطه هنا، عن قصد.
properties-appaudio-capturing = التقاط الصوت من { $exe }
properties-appaudio-exe-fallback = تطبيق
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = أعِد إضافة المصدر لاستهداف تطبيق مختلف (يتغيّر مُعرّف العملية عند إعادة تشغيل التطبيق).
properties-image-file = ملف صورة
properties-media-file = ملف وسائط (mp4، mkv، webm، mov، أو .frec، أو صورة)
properties-media-loop = تكرار (إعادة التشغيل من البداية عند النهاية)
properties-media-hwdecode = فك تشفير بالعتاد (يعود إلى البرمجيات تلقائيًا)
properties-media-note = يُشغَّل .frec عبر كودك freally-video المملوك — لا شيء للتنزيل. تُفكَّك تنسيقات الفيديو الأخرى عبر مكوّن FFmpeg عند الطلب. يحصل صوت الملف على شريحة مازج خاصة به؛ تضبط إزاحة مزامنة الشريحة محاذاة الصوت/الصورة بدقة. المقطع بلا صوت يترك شريحته صامتة.
properties-color = اللون
properties-width = العرض
properties-height = الارتفاع
properties-testtone-note = موجة جيبية مستمرة بتردد 1 كيلوهرتز عند −20 dBFS. يتحكم شريط المازج في مستواها وكتمها؛ لا يوجد شيء آخر للضبط.
properties-timer-format = تنسيق الوقت (strftime)
properties-timer-format-note = مثل %H:%M:%S (الافتراضي)، %I:%M %p، %A %H:%M — النمط غير الصالح يعود إلى %H:%M:%S.
properties-timer-utc = إزاحة UTC (دقائق)
properties-timer-utc-placeholder = التوقيت المحلي
properties-timer-duration = المدة (ثوانٍ)
properties-timer-target = عدّ تنازلي حتى (HH:MM)
properties-timer-target-note = هدف ساعة الحائط يعمل بنفسه ويتكرر يوميًا؛ اتركه فارغًا لاستخدام المدة مع بدء/إيقاف مؤقت/تصفير.
properties-timer-end = عند الصفر
properties-timer-end-none = لا شيء
properties-timer-end-flash = وميض المؤقّت
properties-timer-end-switch = تبديل المشهد
properties-timer-end-scene = المشهد
properties-timer-size = الحجم (بكسل)
properties-timer-start = بدء
properties-timer-pause = إيقاف مؤقت
properties-timer-reset = تصفير
properties-text-file = القراءة من ملف (المسار؛ فارغ = استخدام النص أعلاه)
properties-text-binding = التحليل كـ
properties-text-binding-whole = الملف كاملًا
properties-text-binding-csv = خلية CSV
properties-text-binding-json = مؤشر JSON
properties-text-csv-row = الصف
properties-text-csv-column = العمود
properties-text-csv-column-placeholder = اسم أو رقم
properties-text-json-pointer = المؤشر
properties-text-file-note = يُعاد قراءة الملف خلال نصف ثانية من التغيير. الكتابة الذرّية (ملف مؤقت + إعادة تسمية) مسموحة: تبقى آخر قيمة صالحة على الشاشة أثناء التبديل.
avsync-title = معايرة تزامن الصوت والصورة
avsync-intro = شغّل نمط الوميض + الصافرة المدمج عبر شاشتك ومكبرات صوتك، والتقطه بالكاميرا والميكروفون اللذين تريد محاذاتهما، وسيقيس المشغل الفجوة بينهما. تمر الحلقة عبر الشاشة والسماعات، لذا تُحتسب تأخيراتهما الصغيرة أيضًا.
avsync-video-label = الكاميرا (مصدر الفيديو)
avsync-audio-label = الميكروفون (مصدر الصوت)
avsync-pick = اختر مصدرًا…
avsync-no-video = أضِف الكاميرا كمصدر أولًا — يقيس المشغل المصادر لا الأجهزة مباشرة.
avsync-no-audio = أضِف الميكروفون كمصدر صوت أولًا.
avsync-projector = عرض البرنامج بملء الشاشة على
avsync-projector-open = فتح جهاز العرض
avsync-projector-window-title = البرنامج — تزامن الصوت والصورة
avsync-start-note = يضيف البدء مصدر «نمط تزامن A/V» مؤقتًا فوق المشهد الحالي ويشغّل الصافرة على جهاز المراقبة. يُزال كل شيء عند انتهاء التشغيل.
avsync-manual = إزاحة التزامن (مللي ثانية، يدوي)
avsync-start = بدء المعايرة
avsync-measuring = جارٍ القياس لنحو 12 ثانية — وجّه الكاميرا نحو البرنامج الوامض وحافظ على هدوء الغرفة…
avsync-flash-seen = الكاميرا ترى الوميض
avsync-flash-waiting = بانتظار أن ترى الكاميرا الوميض…
avsync-beep-heard = الميكروفون يسمع الصافرة
avsync-beep-waiting = بانتظار أن يسمع الميكروفون الصافرة…
avsync-cancel = إلغاء
avsync-result-offset = يصل الفيديو بعد الصوت بمقدار { $offset } مللي ثانية.
avsync-result-detail = قيس عبر { $cycles } دورات، ±{ $jitter } مللي ثانية.
avsync-negative = الصوت يصل أصلًا بعد الفيديو. تأخير الصوت لا يصلح هذا الاتجاه — إن كان شريط آخر يحمل صوت هذه الكاميرا فاخفض إزاحته بدلًا من ذلك.
avsync-over-cap = الفجوة المقيسة تتجاوز حد إزاحة التزامن { $max } مللي ثانية. فجوة بهذا الحجم تعني غالبًا اختيار مصدر خاطئ — تحقق من السلسلة وأعد القياس.
avsync-applied = طُبّقت — إزاحة تزامن الميكروفون الآن { $offset } مللي ثانية.
avsync-apply = تطبيق { $offset } مللي ثانية على الميكروفون
avsync-again = إعادة القياس
avsync-close = إغلاق
avsync-error-noFlash = لم ترَ الكاميرا الوميض قط. وجّهها نحو البرنامج الوامض (ملء الشاشة يساعد) وتأكد أن المصدر يعمل، ثم أعد القياس.
avsync-error-noBeep = لم يسمع الميكروفون الصافرة قط. تأكد أن جهاز المراقبة مسموع وأن الميكروفون يعمل (وليس مقيدًا بمفتاح اضغط-للتحدث)، ثم أعد القياس.
avsync-error-tooFewCycles = لم تُلتقط دورات وميض/صافرة نظيفة كافية. أبقِ النمط مرئيًا ومسموعًا بوضوح طوال التشغيل.
avsync-error-notThePattern = ما رُصد لا يتكرر بإيقاع النمط — الأرجح أنه ضوء أو ضجيج في الغرفة، وليس إشارة الاختبار.
avsync-error-unstable = تتعارض الدورات كثيرًا لاعتماد رقم واحد. ثبّت الكاميرا وقلل ضجيج الغرفة ثم أعد القياس.
hotkey-audit-title = خريطة الاختصارات
hotkey-audit-search = بحث
hotkey-audit-filter = الميزة
hotkey-audit-filter-all = كل الميزات
hotkey-audit-col-key = المفتاح
hotkey-audit-col-action = الإجراء
hotkey-audit-col-where = الموضع
hotkey-audit-col-status = الحالة
hotkey-audit-ok = سليم
hotkey-audit-shared = مشترك بين { $count } ارتباطات
hotkey-audit-unregistered = غير مسجّل لدى النظام (محجوز في مكان آخر أو غير متاح)
hotkey-audit-invalid = ليس اختصارًا صالحًا
hotkey-audit-empty = لا اختصارات بعد — اربطها من الإعدادات ← الاختصارات أو على شريط المازج.
hotkey-audit-export = تصدير ورقة مرجعية
hotkey-audit-exported = حُفظت في { $path }
hotkey-audit-note = اربط المفاتيح وغيّرها من الإعدادات ← الاختصارات (الإجراءات العامة) وعلى كل شريط مازج (اضغط-للتحدث / اضغط-للكتم)؛ هذا الجدول يدقق ويوثق.
hotkey-audit-action-record = تبديل التسجيل
hotkey-audit-action-go-live = تبديل البث
hotkey-audit-action-transition = تنفيذ الانتقال
hotkey-audit-action-save-replay = حفظ الإعادة
hotkey-audit-action-add-marker = إضافة علامة
hotkey-audit-action-still = التقاط صورة ثابتة
hotkey-audit-action-panic = لوحة الطوارئ
hotkey-audit-action-timer-toggle = بدء/إيقاف كل المؤقّتات
hotkey-audit-action-timer-reset = تصفير كل المؤقّتات
hotkey-audit-action-ptt = اضغط للتحدث
hotkey-audit-action-ptm = اضغط للكتم
hotkey-audit-feature-recording = التسجيل
hotkey-audit-feature-streaming = البث
hotkey-audit-feature-studio = وضع الاستوديو
hotkey-audit-feature-replay = الإعادة
hotkey-audit-feature-markers = العلامات
hotkey-audit-feature-stills = اللقطات
hotkey-audit-feature-panic = الطوارئ
hotkey-audit-feature-timers = المؤقّتات
hotkey-audit-feature-audio = الصوت (لكل مصدر)
properties-text = النص
properties-font-family = عائلة الخط (النظام؛ فارغ = افتراضي)
properties-size-px = الحجم (بكسل)
properties-text-color = لون النص
properties-align = المحاذاة
properties-align-left = يسار
properties-align-center = وسط
properties-align-right = يمين
properties-line-spacing = تباعد الأسطر
properties-wrap-width = عرض الالتفاف (بكسل؛ 0 = إيقاف)
properties-force-rtl = فرض من اليمين لليسار
properties-text-note = يستخدم التصيير تشكيلًا حقيقيًا (ربط الحروف العربية، الروابط) وترتيب أسطر ثنائي الاتجاه. عائلة Noto Sans المُضمَّنة (بما فيها العربية/العبرية) هي الافتراضية؛ عائلات النظام تعمل أيضًا. تستخدم CJK خطوط النظام حاليًا.
properties-repick-capturing = يلتقط: { $label }
properties-repick-looking = جارٍ البحث عن المصادر…
properties-repick-none-displays = لم يُعثر على شاشات لإعادة الاختيار.
properties-repick-none-windows = لم يُعثر على نوافذ لإعادة الاختيار.
properties-repick-again = اختر مجددًا:
properties-device = الجهاز
properties-video-current-device = (الجهاز الحالي)
properties-format = التنسيق
properties-format-auto-loading = تلقائي (جارٍ تحميل التنسيقات…)
properties-deinterlace = إزالة التشابك
properties-deinterlace-off = إيقاف
properties-deinterlace-discard = إسقاط (مضاعفة أسطر حقل واحد)
properties-deinterlace-bob = بوب (تناوب الحقول)
properties-deinterlace-linear = خطي (استيفاء)
properties-deinterlace-blend = مزج (متوسط الحقلين)
properties-deinterlace-adaptive = متكيّف مع الحركة (فئة yadif)
properties-field-order = ترتيب الحقول
properties-field-order-top = الحقل العلوي أولًا
properties-field-order-bottom = الحقل السفلي أولًا
properties-deinterlace-note = لتغذيات بطاقات الالتقاط المتشابكة. معالجة CPU خالصة ومتطابقة على كل الأنظمة؛ تغييره يعيد تشغيل الجهاز (كتغيير الصيغة).
camera-controls-title = عناصر تحكم الكاميرا
camera-controls-refresh = تحديث
camera-controls-reset = إعادة ضبط الملف
camera-controls-empty = لا عناصر تحكم الآن — يجب أن يكون الجهاز يبث (أضِفه إلى مشهد أولًا)، وبعض الأنظمة لا تبلّغ عن أي عناصر (خاصة macOS). هذه هي الحالة الصادقة لكل نظام.
camera-controls-note = تُطبق التغييرات فورًا وتُحفظ في ملف تعريف هذا الجهاز، ويعاد تطبيقها عند إعادة التوصيل وإعادة التشغيل.
camera-control-brightness = السطوع
camera-control-contrast = التباين
camera-control-hue = تدرج اللون
camera-control-saturation = التشبع
camera-control-sharpness = الحدة
camera-control-gamma = غاما
camera-control-white-balance = توازن الأبيض
camera-control-backlight = تعويض الإضاءة الخلفية
camera-control-gain = الكسب
camera-control-pan = التحريك الأفقي
camera-control-tilt = الإمالة
camera-control-zoom = التقريب
camera-control-exposure = التعريض
camera-control-iris = القزحية
camera-control-focus = التركيز
properties-format-auto = تلقائي (أعلى دقة)
properties-audio-capture-of = التقاط صوت
properties-audio-default-output = الخرج الافتراضي (ما تسمعه)
properties-audio-default-input = الدخل الافتراضي
properties-audio-default-suffix = (افتراضي)
properties-audio-current-device = (الجهاز الحالي: { $id })

# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = كسب
audiofilters-name-noise-gate = بوابة ضوضاء
audiofilters-name-compressor = ضاغط
audiofilters-name-limiter = مُحدِّد
audiofilters-name-eq = مُعادِل ثلاثي النطاقات
audiofilters-name-denoise = إزالة الضوضاء
audiofilters-name-ducking = خفض تلقائي
audiofilters-name-parametric-eq = مُعادِل بارامتري
audiofilters-name-de-esser = مزيل الصفير
audiofilters-name-rumble-guard = حاجز الهدير
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = إعداد مسبق
audiofilters-voice-preset-pick = إعداد صوت مسبق…
audiofilters-voice-broadcast = صوت البث
audiofilters-voice-podcast = صوت البودكاست
audiofilters-voice-clean = صوت نظيف
audiofilters-voice-none = مسح السلسلة
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = تردد الصفير (Hz)
audiofilters-deesser-amount = أقصى خفض (dB)
audiofilters-rumble-freq = قطع الترددات المنخفضة (Hz)
audiofilters-title = فلاتر الصوت — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = منحنى استجابة المُعادِل البارامتري مع طيف مباشر
eq-band-type = النوع
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + نطاق
eq-remove-band = إزالة النطاق
eq-type-bell = جرس
eq-type-lowShelf = رف منخفض
eq-type-highShelf = رف مرتفع
eq-type-notch = نوتش
eq-type-highPass = تمرير عالٍ
eq-type-lowPass = تمرير منخفض
audiofilters-chain-header = سلسلة الفلاتر (الأعلى يعمل أولًا، قبل المُنزلق)
audiofilters-add = + إضافة فلتر
audiofilters-add-menu = إضافة فلتر صوت
audiofilters-empty = لا فلاتر بعد — أزِل ضوضاء ميكروفون (DSP كلاسيكي، بلا تعلّم آلي)، أغلق بوابة الغرفة، روّض الذُّرى بالضاغط، أو اخفض الموسيقى تحت صوتك.
audiofilters-enable = تفعيل { $name }
audiofilters-run-earlier = تشغيل أبكر
audiofilters-move-up = تحريك { $name } لأعلى
audiofilters-run-later = تشغيل لاحقًا
audiofilters-move-down = تحريك { $name } لأسفل
audiofilters-remove-title = إزالة الفلتر
audiofilters-remove = إزالة { $name }
audiofilters-gain-db = الكسب (dB)
audiofilters-open-db = يفتح عند (dB)
audiofilters-close-db = يغلق عند (dB)
audiofilters-attack-ms = الهجوم (مللي ثانية)
audiofilters-hold-ms = الإمساك (مللي ثانية)
audiofilters-release-ms = التحرير (مللي ثانية)
audiofilters-ratio = النسبة (:1)
audiofilters-threshold-db = العتبة (dB)
audiofilters-output-gain-db = كسب الخرج (dB)
audiofilters-ceiling-db = السقف (dB)
audiofilters-low-db = منخفض (dB)
audiofilters-mid-db = متوسط (dB)
audiofilters-high-db = مرتفع (dB)
audiofilters-strength = القوة
audiofilters-denoise-note = كبت طيفي كلاسيكي DSP مملوك — الضوضاء الثابتة (المراوح، الهسهسة) تنخفض بينما يمرّ الكلام. بلا تعلّم آلي، بلا نماذج، وفق الميثاق.
audiofilters-duck-under = اخفض تحت
audiofilters-ducking-trigger = مصدر تشغيل الخفض
audiofilters-pick-trigger = (اختر مُشغِّلًا — مثل ميكروفونك)
audiofilters-trigger-at-db = التشغيل عند (dB)
audiofilters-duck-by-db = الخفض بمقدار (dB)

# --- FiltersDialog.tsx ---
filters-name-chroma-key = مفتاح الكروما
filters-name-color-key = مفتاح اللون
filters-name-luma-key = مفتاح اللمعان
filters-name-render-delay = تأخير التصيير
filters-name-color-correction = تصحيح الألوان
filters-name-lut = تطبيق LUT
filters-name-blur = تمويه
filters-name-mask = قناع صورة
filters-name-sharpen = زيادة الحدّة
filters-name-scroll = تمرير
filters-name-crop = اقتصاص
filters-title = الفلاتر — { $name }
filters-blend-mode = وضع المزج
filters-chain-header = سلسلة الفلاتر (الأعلى يعمل أولًا)
filters-add = + إضافة فلتر
filters-add-menu = إضافة فلتر
filters-empty = لا فلاتر بعد — طبّق مفتاح كروما على كاميرا ويب، صحّح ألوان التقاط، أو مرّر شريطًا إخباريًا.
filters-enable = تفعيل { $name }
filters-run-earlier = تشغيل أبكر
filters-move-up = تحريك { $name } لأعلى
filters-run-later = تشغيل لاحقًا
filters-move-down = تحريك { $name } لأسفل
filters-remove-title = إزالة الفلتر
filters-remove = إزالة { $name }
filters-key-color-rgb = لون المفتاح (أي لون، مسافة RGB)
filters-similarity = التشابه
filters-smoothness = النعومة
filters-luma-min = أدنى لمعان (يُزيل المفاتيح الأغمق)
filters-luma-max = أقصى لمعان (يُزيل المفاتيح الأفتح)
filters-delay = التأخير (مللي ثانية — فيديو فقط، مثل المزامنة مع الصوت؛ بحدّ أقصى 500)
filters-key-color = لون المفتاح
filters-spill = التسرّب
filters-gamma = جاما
filters-brightness = السطوع
filters-contrast = التباين
filters-saturation = التشبّع
filters-hue-shift = إزاحة التدرّج
filters-opacity = العتامة
filters-cube-file = ملف .cube
filters-amount = المقدار
filters-radius = نصف القطر
filters-name-shader = شيدر (WGSL)
filters-shader-gallery = المعرض
filters-shader-gallery-pick = تحميل إعداد مسبق…
filters-shader-gallery-grayscale = تدرّج رمادي
filters-shader-gallery-invert = عكس
filters-shader-gallery-scanlines = خطوط المسح
filters-shader-gallery-vignette = تظليل الحواف
filters-shader-source = مصدر الشيدر (WGSL)
filters-shader-hint = اكتب دالة effect(uv, color, p, texel, time) بلغة WGSL تُعيد vec4. علّق على المعاملات باستخدام // @param name min max default لإنشاء المنزلقات. يُتجاهَل الشيدر غير الصالح — يُعرض المصدر دون تصفية حتى يُترجَم بنجاح.
filters-name-bezier-mask = قناع بيزيه
filters-mask-editor-hint = اسحب نقطة لتحريكها، وانقر نقرًا مزدوجًا لإضافة نقطة، وانقر بزر الفأرة الأيمن على نقطة لإزالتها.
filters-mask-shape = الشكل
filters-mask-shape-pick = إعداد مسبق…
filters-mask-shape-rectangle = مستطيل
filters-mask-shape-diamond = معيّن
filters-mask-shape-hexagon = سداسي
filters-mask-shape-circle = دائرة
filters-mask-feather = تنعيم الحواف
filters-mask-export-wipe = تصدير كمسح…
filters-mask-image = صورة القناع
filters-mask-mode = الوضع
filters-mask-alpha = ألفا
filters-mask-luma = لمعان
filters-mask-invert = عكس
filters-speed-x = السرعة س (بكسل/ث)
filters-speed-y = السرعة ص (بكسل/ث)
filters-crop-left = يسار
filters-crop-top = أعلى
filters-crop-right = يمين
filters-crop-bottom = أسفل
filters-crop-aria = اقتصاص { $side }

# --- PickerShell.tsx ---
pickershell-refresh-aria = تحديث
pickershell-refresh-title = تحديث القائمة
pickershell-close = إغلاق


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = الإبلاغ عن خطأ
bugreport-intro = التقارير مجهولة واختيارية — لا يُرسَل شيء تلقائيًا. ستراجع النص الدقيق أدناه، ثم تُرسله عبر مشكلة GitHub مُعبّأة مسبقًا أو تطبيق بريدك. لا بيانات شخصية (يُحجَب مسار منزلك واسم المستخدم)؛ لا حساب، لا خادم.
bugreport-crash-notice = أُغلق Freally Capture بشكل غير متوقّع في تشغيل سابق — تفاصيل التعطّل المجهولة مُضمَّنة أدناه. الإبلاغ عنها يساعد في إصلاحه بسرعة.
bugreport-description-label = ماذا كنت تفعل حين حدث؟ (اختياري)
bugreport-description-placeholder = مثال: تجمّدت المعاينة عندما أضفت كاميرا ويب ثانية
bugreport-include-crash = تضمين تفاصيل التعطّل المجهولة من التشغيل الأخير
bugreport-preview-label = ما سيُرسَل بالضبط
bugreport-open-github = فتح مشكلة GitHub
bugreport-gmail-title = يفتح نافذة إنشاء رسالة Gmail في متصفحك، مُعبّأة مسبقًا. غير مسجّل الدخول؟ يعرض Google شاشة تسجيل الدخول أولًا.
bugreport-compose-gmail = إنشاء في Gmail
bugreport-email-title = يفتح مسودة في أي تطبيق بريد يستخدمه هذا الجهاز افتراضيًا (Outlook، Thunderbird، Mail…)
bugreport-send-email = إرسال بريد إلكتروني
bugreport-copied = تم النسخ ✓
bugreport-copy-report = نسخ التقرير
bugreport-dismiss-crash = تجاهل التعطّل
bugreport-copy-failed = تعذّر النسخ — حدّد النص وانسخه يدويًا
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = ما الذي حدث
bugreport-preview-no-description = (لم يُقدَّم وصف)
bugreport-preview-diagnostics = تشخيصات مجهولة (بلا بيانات شخصية)
bugreport-preview-from = من: Freally Capture
bugreport-preview-crash-excerpt = --- مقتطف التعطّل ---

# --- Updates.tsx ---
updates-title = تحديث البرنامج
updates-checking = جارٍ التحقّق من التحديثات…
updates-uptodate = أنت على أحدث إصدار.
updates-check-again = تحقّق مجددًا
updates-available = الإصدار { $version } متاح
updates-current-version = (لديك { $current })
updates-release-notes-label = الإصدار { $version } — ملاحظات الإصدار
updates-confirm = هل تريد التحديث الآن؟ يُتحقَّق من التنزيل مقابل مفتاح التوقيع المُضمَّن قبل تطبيقه. يُغلق Freally Capture، ويعمل المُثبِّت، ويُعاد فتح الإصدار الجديد من تلقاء نفسه.
updates-yes-update-now = نعم، حدّث الآن
updates-no-not-now = لا، ليس الآن
updates-downloading = جارٍ تنزيل { $version }…
updates-starting = جارٍ البدء…
updates-installed = تم تثبيت التحديث.
updates-restart-now = أعِد التشغيل الآن
updates-restart-later = أعِد التشغيل لاحقًا
updates-try-again = حاول مجددًا

# --- Models.tsx ---
models-title = المكوّنات
models-ffmpeg-heading = FFmpeg — كودكات الشبكة
models-badge-third-party = طرف ثالث · غير مُضمَّن
models-ffmpeg-desc = يُسجِّل محرك Freally Capture الخاص freally-video (.frec) بلا فقدان ودون أي إضافات. تسجيل تنسيقات الشبكة التي تتوقّعها المنصات والمشغّلات — H.264/AAC (و HEVC/AV1) في mp4/mkv/mov/webm — يستخدم FFmpeg، أداة منفصلة لا يأتي بها هذا التطبيق أبدًا: تلك الكودكات مُقيَّدة ببراءات اختراع، لذا تبقى اختيارية وموسومة بوضوح. تُنزَّل عند الطلب من البناء المثبَّت أدناه، ويُتحقَّق منها بـ SHA-256 قبل أول استخدام، وتُخزَّن مؤقتًا لكل مستخدم، وتُدار كعملية منفصلة. ترخيصها (LGPL/GPL) خاص بها — راجع THIRD-PARTY-NOTICES.
models-checking = جارٍ الفحص…
models-ffmpeg-not-installed = غير مُثبَّت. متاح: FFmpeg { $version } من { $source } (تنزيل { $size }).
models-ffmpeg-none-pinned = لا يوجد بناء FFmpeg مثبَّت لهذه المنصة بعد — تسجيل كودكات الشبكة غير متاح هنا. تسجيل freally-video بلا فقدان لا يتأثّر.
models-ffmpeg-download-verify = تنزيل وتحقّق ({ $size })
models-downloading = جارٍ التنزيل…
models-download-of = من
models-cancel = إلغاء
models-ffmpeg-verifying = جارٍ التحقّق من التنزيل مقابل SHA-256 المثبَّت…
models-ffmpeg-extracting = جارٍ فك الحزم…
models-ffmpeg-ready = مُثبَّت ومُتحقَّق منه — { $version }
models-remove = إزالة
models-ffmpeg-retry = إعادة محاولة التنزيل
models-network-note = التنزيل هو إجراء الشبكة الوحيد في هذه اللوحة ولا يبدأ من تلقاء نفسه أبدًا. المجموع الاختباري الفاشل يُجهض التثبيت — يرفض التطبيق تشغيل بايتات لا يمكنه ضمانها.
models-cef-heading = بيئة تشغيل مصدر المتصفح — Chromium (CEF)
models-cef-desc = تُصيّر مصادر المتصفح صفحات الويب (التنبيهات، الأدوات، التراكبات) عبر Chromium Embedded Framework — بيئة تشغيل بحجم ~100 ميجابايت لا يأتي بها هذا التطبيق أبدًا. تُنزَّل عند الطلب من فهرس بناء CEF الرسمي، ويُتحقَّق منها مقابل SHA-1 لذلك الفهرس قبل فك أي شيء، وتُخزَّن مؤقتًا لكل مستخدم. مصدر المتصفح الذي يُصيَّر عبرها يأتي بمرحلته الخاصة؛ هذا يُثبِّت بيئة التشغيل التي يحتاجها.
models-cef-download-install = تنزيل وتثبيت
models-cef-unsupported = لا ينشر CEF بناءً لهذه المنصة — مصادر المتصفح غير متاحة هنا.
models-cef-resolving = جارٍ تحديد أحدث بناء مستقر…
models-cef-verifying = جارٍ التحقّق من التنزيل مقابل SHA-1 الفهرس…
models-cef-extracting = جارٍ فك حزم بيئة التشغيل…
models-cef-ready = مُثبَّت — CEF { $version }.
models-cef-retry = إعادة المحاولة
models-integrations-heading = تكاملات اختيارية
models-badge-never-bundled = لا يُضمَّن أبدًا
models-ndi-detected = مُكتشَف
models-ndi-not-installed = غير مُثبَّت
models-vst-available = متاح
models-vst-not-available = غير متاح

# --- Recordings.tsx ---
recordings-title = التسجيلات
recordings-loading = جارٍ قراءة المجلد…
recordings-empty = لا تسجيلات بعد — بدء التسجيل يكتب في المجلد المحدّد في الإخراج.
recordings-frec-label = مملوك بلا فقدان (freally-video)
recordings-remux-title = أعِد التغليف كـ mp4 — نسخ التدفق، بلا إعادة ترميز، بلا تغيير في الجودة (يحتاج مكوّن FFmpeg)
recordings-trim = قصّ
recordings-trim-title = اقتطع مقطعًا من هذا التسجيل — القصّ المحاذي للإطارات المفتاحية يُصدَّر دون إعادة ترميز
recordings-verify = تحقّق
recordings-verify-title = افحص سلامة الملف — بنية الحاوية، الاستمرارية، تداخل الصوت/الصورة، المدة
recordings-verifying = جارٍ التحقق…
verify-dismiss = إغلاق
verify-verdict-pass = { $name } — السلامة سليمة
verify-verdict-warn = { $name } — تم التحقق مع تحذيرات
verify-verdict-fail = { $name } — عُثر على مشاكل
verify-container = الحاوية
verify-video-continuity = استمرارية الفيديو
verify-audio-continuity = استمرارية الصوت
verify-av-interleave = تداخل الصوت/الصورة
verify-duration = المدة
recordings-alpha-label = ألفا
recordings-prores-title = تصدير ماستر ‎.mov بترميز ProRes 4444 يحفظ الألفا (للمونتاج)
recordings-qtrle-title = تصدير ‎.mov بترميز QuickTime Animation يحفظ الألفا (توافق أقصى)
trim-title = قصّ — { $name }
trim-loading = جارٍ قراءة الملف…
trim-preview-alt = إطار المعاينة
trim-position = موضع التشغيل
trim-step-second-back = ثانية للخلف
trim-step-frame-back = إطار للخلف
trim-step-frame-forward = إطار للأمام
trim-step-second-forward = ثانية للأمام
trim-snap = إطار مفتاحي
trim-snap-title = الالتقاط إلى أقرب إطار مفتاحي — القص هناك يُصدَّر دون إعادة ترميز
trim-set-in = نقطة البداية
trim-set-out = نقطة النهاية
trim-range-invalid = يجب أن تأتي نقطة النهاية بعد نقطة البداية.
trim-copy-badge = ✓ يُصدَّر دون إعادة ترميز — نقطة البداية على إطار مفتاحي.
trim-reencode-badge = ستُعاد الترميز: نقطة البداية بين إطارين مفتاحيين (استخدم «إطار مفتاحي» للالتقاط لقصّ دون فقد).
trim-export = تصدير المقطع
trim-export-916 = 9:16
trim-export-916-title = تصدير عمودي مُعاد التأطير (قصّ مركزي بحجم اللوحة العمودية) — يعيد الترميز دائمًا
recordings-remuxing = جارٍ إعادة التغليف…
recordings-remux-to-mp4 = إعادة تغليف إلى MP4
recordings-export-mp4-title = فُكّ .frec المملوك وأعِد ترميزه إلى MP4 (H.264/AAC) ليعمل في أي مشغّل — يحتاج مكوّن FFmpeg
recordings-exporting = جارٍ التصدير…
recordings-export-mp4 = تصدير ← MP4
recordings-export-mkv-title = فُكّ .frec المملوك وأعِد ترميزه إلى MKV ليعمل في أي مشغّل
recordings-starting = جارٍ البدء…
recordings-frames = { $done } / { $total } إطار
recordings-cancel = إلغاء
recordings-export-cancelled = أُلغي التصدير.
recordings-exported-to = صُدِّر إلى { $path }
recordings-remuxed-to = أُعيد تغليفه إلى { $path }
recordings-normalize = تطبيع
recordings-normalizing = جارٍ التطبيع…
recordings-normalize-title = تطبيع الجهارة إلى الهدف (يكتب نسخة)
recordings-normalized-to = تم التطبيع إلى { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = الصوت فقط
audiorec-format = تنسيق تسجيل الصوت
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = تسجيل الصوت
audiorec-stop = إيقاف
audiorec-pause = إيقاف مؤقت
audiorec-resume = استئناف
audiorec-recording = REC { $sec }s
audiorec-saved = تم حفظ { $count } ملف مسار

# --- OpenedFrec.tsx ---
openfrec-title = فتح تسجيل .frec
openfrec-desc = يُسجِّل Freally Capture تنسيق .frec المملوك بلا فقدان — لكنه لا يُشغِّله. سيُشغِّل Freally Player تنسيق .frec مباشرةً عند إصداره. حاليًا، صدّره إلى MP4/MKV ليعمل في أي مشغّل (VLC، مشغّل نظامك، أي شيء).
openfrec-exported-to = صُدِّر إلى { $path }
openfrec-exporting = جارٍ التصدير…
openfrec-starting = جارٍ البدء…
openfrec-export-mp4 = تصدير ← MP4
openfrec-export-mkv = تصدير ← MKV

# --- VerticalCanvasDialog.tsx ---
vertical-title = اللوحة العمودية (9:16)
vertical-enable = فعّل اللوحة الثانية — قابلة للتسجيل والبث بشكل مستقل عن البرنامج
vertical-scene-label = المشهد الذي تُركِّبه هذه اللوحة
vertical-width = العرض
vertical-height = الارتفاع
vertical-preview-alt = معاينة اللوحة العمودية
vertical-note = مواضع العناصر مطابقة بالبكسل عبر اللوحات: اختر هذا المشهد في شريط المشاهد لترتيبه بينما تعرض هذه المعاينة النتيجة العمودية. أهداف البث تختار هذه اللوحة في ⦿ البث…؛ يمكن للإعدادات ← الإخراج تسجيلها بجانب الملف الرئيسي.
vertical-close = إغلاق

# --- EulaGate.tsx ---
eula-title = Freally Capture — اتفاقية الترخيص
eula-version = v{ $version }
eula-intro = يُرجى قراءة هذه الاتفاقية وقبولها لاستخدام Freally Capture. باختصار: إنها أداة محايدة، وأنت وحدك المسؤول عمّا تلتقطه وتسجّله وتبثّه — وعن امتلاك الحقوق له.
eula-thanks = شكرًا لقراءتك.
eula-scroll-hint = مرّر إلى النهاية للمتابعة.
eula-decline = رفض وخروج
eula-agree = أوافق


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = الإخراج
output-loading = لا تزال الإعدادات قيد التحميل…
output-container-frec = freally-video (.frec) — بلا فقدان، مملوك، لا شيء للتنزيل
output-container-mkv = MKV — مقاوم للأعطال؛ أعِد التغليف إلى mp4 لاحقًا
output-container-mp4 = MP4 — يعمل في كل مكان
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = بلا فقدان
output-preset-lossless-title = كودك freally-video المملوك — مطابق للبت، بلا تنزيل
output-preset-high-label = جودة عالية
output-preset-high-title = MP4، أفضل مُرمِّز مُكتشَف، شبه بلا فقدان CQ 16، إعداد الجودة المسبق
output-preset-balanced-label = متوازن
output-preset-balanced-title = MKV، أفضل مُرمِّز مُكتشَف، CQ 23، الإعداد المتوازن المسبق
output-recording-format = تنسيق التسجيل
output-ffmpeg-warning = يحتاج هذا التنسيق مكوّن FFmpeg (كودكات الشبكة — غير مُضمَّن). أما .frec بلا فقدان فلا يحتاج شيئًا.
output-install = تثبيت…
output-recordings-folder = مجلد التسجيلات
output-folder-placeholder = مجلد فيديوهات النظام
output-filename-prefix = بادئة اسم الملف
output-recording-template = اسم ملف التسجيل
output-replay-template = اسم ملف الإعادة
output-still-template = اسم ملف الإطار الثابت
output-template-tokens = الرموز: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = مجلد الإعادة
output-still-folder = مجلد الإطارات الثابتة
output-same-folder-placeholder = مجلد التسجيلات
output-frame-rate = معدل الإطارات
output-fps-option = { $fps } fps
output-split-every = تقسيم كل (دقائق، 0 = إيقاف)
output-output-width = عرض الإخراج (0 = اللوحة؛ تنسيقات الشبكة فقط)
output-output-height = ارتفاع الإخراج (0 = اللوحة)
output-record-vertical = سجّل اللوحة العمودية أيضًا (ملف موازٍ "… (عمودي)"؛ يحتاج تفعيل لوحة 9:16)
output-audio-tracks = مسارات الصوت
output-recorded-tracks-group = المسارات المُسجَّلة
output-track-last-one = يجب أن يُسجِّل مسار واحد على الأقل
output-record-track-on = تسجيل المسار { $index }: تشغيل
output-record-track-off = تسجيل المسار { $index }: إيقاف
output-encoder-heading = المُرمِّز
output-video-encoder = مُرمِّز الفيديو
output-encoder-auto = تلقائي — أفضل مُكتشَف (H.264)
output-encoder-unavailable = — غير متاح هنا
output-preset = الإعداد المسبق
output-preset-quality = الجودة
output-preset-balanced-option = متوازن
output-preset-performance = الأداء
output-rate-control = التحكم بالمعدل
output-rc-cqp = CQP (جودة ثابتة)
output-rc-cbr = CBR (معدل بت ثابت)
output-rc-vbr = VBR (معدل بت متغيّر)
output-cq = CQ (0–51، أقل = أفضل)
output-bitrate = معدل البت (kbps)
output-keyframe = فاصل الإطار المفتاحي (ث)
output-audio-bitrate = معدل بت الصوت (kbps / مسار)
output-iso-heading = تسجيل ISO
output-iso-explainer = سجّل المصادر المحددة نظيفة، كل مصدر في ملفه الخاص إلى جانب البرنامج — قبل التركيب، وبحجم اللوحة ومعدل إطاراتها، لكي يستقر كل ملف محاذيًا على خط زمن المونتاج. مساران مريحان على بطاقة رسوميات متوسطة؛ وكل مسار إضافي يكلف تصييرًا وترميزًا إضافيين.
output-iso-none = لا توجد مصادر في المجموعة بعد.
output-iso-source-on = يجري تسجيل "{ $name }" في ملف ISO خاص به — انقر للإيقاف
output-iso-source-off = تسجيل "{ $name }" في ملف ISO خاص به
output-iso-post-filter = التسجيل مع مرشحات المصدر (بعد الترشيح)؛ وبدون تحديد يُسجَّل المصدر الخام
output-iso-format = صيغة ISO
output-iso-encoder = مرمّز فيديو ISO
output-alpha-frec = التسجيل مع الشفافية (ألفا) — البرنامج فوق خلفية شفافة
output-alpha-title = يحصل المسجل على تصيير شفاف خاص به؛ تبقى المعاينة والبث كما هما. صدّر إلى ProRes 4444 أو QTRLE من قائمة التسجيلات — MP4/MKV تسطّح الألفا.
output-split-events = ابدأ ملفًا جديدًا أيضًا عند… (كل جزء يبدأ على الحدث تمامًا؛ أدنى طول للجزء ثانية واحدة)
output-split-on-scene = تبديل المشهد
output-split-on-marker = علامة
output-split-on-rundown = خطوة سير العرض
output-auto-markers = إسقاط علامات الفصول تلقائيًا عند أحداث الاستوديو (تبديل المشهد، حفظ الإعادة، إعادة الاتصال، الإطارات المفقودة، الإنذارات، القواعد)
output-auto-markers-title = تُكتب العلامات المصنّفة في فصول التسجيل (mkv) أو في الملف الجانبي ‎.chapters.txt، إلى جانب مفتاح العلامة اليدوي
output-pipeline-heading = خط أنابيب ما بعد التسجيل
output-pipeline-explainer = بعد اكتمال التسجيل، تُنفَّذ هذه الخطوات على الملف الرئيسي بالترتيب في الخلفية. مجموعة إجراءات مغلقة — لا توجد خطوة «تشغيل أمر» عمدًا. تتوقف السلسلة عند أول فشل.
output-pipeline-enabled = تشغيل خط الأنابيب بعد كل تسجيل
output-pipeline-add = أضف خطوة…
output-pipeline-up = تحريك لأعلى
output-pipeline-down = تحريك لأسفل
output-pipeline-remove = إزالة الخطوة
output-pipeline-template = قالب إعادة التسمية (رموز CAP-M25)
output-pipeline-folder = المجلد
pipeline-queue = خط أنابيب ما بعد التسجيل
pipeline-verify = تحقّق
pipeline-remux = إعادة تغليف إلى MP4
pipeline-normalize = معايرة الصوت
pipeline-rename = إعادة تسمية
pipeline-move = نقل إلى مجلد
pipeline-copy = نسخ إلى مجلد
pipeline-reveal = إظهار في مدير الملفات
pipeline-luaEvent = إخطار سكربتات Lua
output-presets = إعدادات مسبقة:

# --- SettingsStream.tsx ---
stream-title = الإعدادات — البث
stream-target-enabled = الهدف { $index } مُفعَّل
stream-target = الهدف { $index }
stream-remove = إزالة
stream-service = الخدمة
stream-canvas = اللوحة
stream-canvas-main = الرئيسية (البرنامج)
stream-canvas-vertical = عمودية (9:16 — فعّلها في الاستوديو)
stream-ingest-srt = عنوان استقبال SRT
stream-ingest-whip = عنوان نقطة نهاية WHIP
stream-ingest-url = عنوان الاستقبال
stream-ingest-override = (تجاوز — فارغ = إعداد الخدمة المسبق)
stream-key-srt = streamid (اختياري — يُلحَق كـ ?streamid=…؛ يُعامَل كسرّ)
stream-key-whip = رمز Bearer (اختياري — يُرسَل كترويسة Authorization؛ سرّ)
stream-key-custom = مفتاح البث (من خادمك — يُعامَل كسرّ)
stream-key-service = مفتاح البث (من لوحة تحكم المُنشئ — يُعامَل كسرّ)
stream-key-aria = مفتاح البث { $index }
stream-key-hide = إخفاء
stream-key-show = إظهار
stream-encoder = المُرمِّز (H.264 — ما تحمله RTMP و SRT و WHIP جميعًا)
stream-encoder-auto = تلقائي — أفضل مُرمِّز H.264 مُكتشَف
stream-encoder-unavailable = (غير متاح هنا)
stream-video-bitrate = معدل بت الفيديو (kbps، CBR)
stream-audio-bitrate = معدل بت الصوت (kbps)
stream-fps = FPS
stream-keyframe = فاصل الإطار المفتاحي (ث)
stream-audio-track = مسار الصوت (1–6)
stream-output-width = عرض الإخراج (0 = اللوحة)
stream-output-height = ارتفاع الإخراج (0 = اللوحة)
stream-add-target = + إضافة هدف
stream-go-live-note = يَنشر البث المباشر إلى كل هدف مُفعَّل دفعةً واحدة، مباشرةً لكل منصة. الأهداف ذات إعدادات المُرمِّز المتطابقة تتشارك ترميزًا واحدًا.
stream-auto-record = ابدأ التسجيل عند البث المباشر (يظل التسجيل يتوقّف بشكل مستقل)
stream-ffmpeg-note-before = تعمل كودكات الشبكة للبث عبر مكوّن ffmpeg الموسوم عند الطلب —
stream-ffmpeg-note-link = أدِرْه هنا
stream-ffmpeg-note-after = . يستمر التسجيل المحلي مهما فعل البث.
stream-cancel = إلغاء
stream-save = حفظ

# --- SettingsReplay.tsx ---
replay-title = الإعدادات — مخزن الإعادة
replay-length-15s = 15 ث
replay-length-30s = 30 ث
replay-length-1min = دقيقة واحدة
replay-length-2min = دقيقتان
replay-length-5min = 5 دقائق
replay-quality-low = منخفضة (3 Mbps)
replay-quality-standard = قياسية (6 Mbps)
replay-quality-high = عالية (12 Mbps)
replay-length-presets = إعدادات الطول المسبقة
replay-quality-presets = إعدادات الجودة المسبقة
replay-length-seconds = الطول (ثوانٍ)
replay-video-bitrate = معدل بت الفيديو (kbps)
replay-fps = FPS
replay-audio-track = مسار الصوت (1–6)
replay-note = أثناء التسليح، يُشغِّل المخزن ترميزه الخفيف الخاص إلى حلقة محدودة على القرص — نحو { $mb } ميجابايت بهذه الإعدادات. الحفظ يخيط الحلقة دون إعادة ترميز ولا يمسّ البث أو التسجيل أبدًا. تُطبَّق التغييرات في المرة التالية التي تُسلِّح فيها.
replay-cancel = إلغاء
replay-save = حفظ

# --- SettingsRemote.tsx ---
remote-title = الإعدادات — التحكم عن بُعد
remote-enable = فعّل الواجهة البرمجية عن بُعد عبر WebSocket
remote-password = كلمة المرور (مطلوبة — تُصادق بها وحدات التحكم)
remote-password-placeholder = كلمة مرور لوحدات تحكمك
remote-password-hide = إخفاء
remote-password-show = إظهار
remote-port = المنفذ
remote-allow-lan = السماح باتصالات LAN (الافتراضي هذا الجهاز فقط)
remote-note = إيقاف = المنفذ مغلق. تشغيل = WebSocket محمي بكلمة مرور على 127.0.0.1 (أو شبكتك المحلية عند التفعيل) يمكنه تبديل المشاهد، وتشغيل الانتقال، وبدء/إيقاف البث والتسجيل، وحفظ الإعادات، وضبط الكتم/مستويات الصوت — الإجراءات نفسها كالواجهة، لا أكثر. لا يمكنه قراءة الملفات. عامِل كلمة المرور كأي بيانات اعتماد؛ فضّل هذا الجهاز فقط ما لم تتحكم تحديدًا من جهاز آخر.
remote-password-required = كلمة المرور مطلوبة لتفعيل الواجهة البرمجية عن بُعد.
remote-cancel = إلغاء
remote-save = حفظ

# --- SettingsHotkeys.tsx ---
hotkeys-title = الإعدادات — الاختصارات
hotkeys-record = بدء / إيقاف التسجيل
hotkeys-go-live = بث مباشر / إنهاء البث
hotkeys-transition = انتقال وضع الاستوديو
hotkeys-save-replay = حفظ الإعادة (آخر N ثانية)
hotkeys-add-marker = وضع علامة فصل (التسجيل)
hotkeys-note = الاختصارات عامة — تعمل أثناء تركيز تطبيقات أخرى. فارغ = غير مربوط. مفاتيح الضغط للتحدث/الكتم في المازج على قائمة ⋯ لكل شريحة. على Linux/Wayland، قد تكون الاختصارات العامة غير متاحة (قيد من المُركِّب) — تظل الأزرار تعمل.
hotkeys-cancel = إلغاء
hotkeys-save = حفظ

# --- WorkspaceDialog.tsx ---
workspace-title = ملفات التعريف ومجموعات المشاهد
workspace-profiles = ملفات التعريف
workspace-profiles-hint = ملف التعريف هو إعداداتك — هدف البث، الإخراج، الاختصارات. بدّل لكل عرض أو لكل منصة.
workspace-collections = مجموعات المشاهد
workspace-collections-hint = المجموعة هي مشاهدك + مصادرك. الإنشاء يُكرّر الحالية كنقطة بداية.
workspace-active = نشط
workspace-switch-to = التبديل إلى { $name }
workspace-active-marker = ● نشط
workspace-new-name-placeholder = اسم جديد…
workspace-new-name-label = اسم { $title } الجديد
workspace-create = إنشاء

# --- OBS import (CAP-M02) ---
workspace-import-obs = استيراد من OBS…
workspace-import-obs-hint = استورد مجموعة مشاهد OBS (ملف scenes.json الخاص بها). تُحفظ مجموعتك الحالية أولاً.
workspace-import-busy = جارٍ الاستيراد…
workspace-import-title = تم استيراد «{ $name }»
workspace-import-summary = { $scenes } مشهد · { $sources } مصدر · { $items } عنصر
workspace-import-dismiss = إغلاق
workspace-import-clean = تم استيراد كل شيء بنجاح.
workspace-import-geometry-caveat = تُضبط الأحجام والمواضع من تخطيط OBS — راجع كل مشهد وأعد اختيار أجهزة الالتقاط.
workspace-import-notes-title = مستورد مع ملاحظات
workspace-import-skipped-title = لم يُستورد
import-note-needsReselect = أعد اختيار الجهاز/الشاشة/النافذة
import-note-gameCaptureAsWindow = التقاط اللعبة → التقاط النافذة
import-note-referencesFile = تحقق من مسار الملف
import-note-filterDropped = بعض المرشحات غير مدعومة
import-note-geometryApproximated = الموضع/الحجم تقريبي
import-skip-unsupportedKind = لا يوجد نوع مصدر مكافئ
import-skip-group = المجموعات غير مدعومة بعد

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = إعادة ربط الملفات المفقودة…
doctor-title = ملفات مفقودة
doctor-scanning = جارٍ الفحص…
doctor-all-good = جميع الملفات المشار إليها موجودة. لا شيء لإعادة ربطه.
doctor-intro = تعذّر العثور على { $count } من الملفات المشار إليها على هذا الحاسوب. حدّد الموقع الجديد لكلٍّ منها — يُصلَح كل مشهد يستخدمه دفعة واحدة.
doctor-relinked = تمت إعادة ربط { $count } مرجعًا.
doctor-uses = استُخدم { $count }×
doctor-locate = تحديد الموقع…
doctor-locate-folder = البحث في مجلد…
doctor-locate-folder-hint = اختر مجلدًا؛ يُطابَق كل ملف مفقود بالاسم ويُعاد ربطه.
doctor-kind-image = صورة
doctor-kind-media = وسائط
doctor-kind-slideshow = عرض شرائح
doctor-kind-font = خط
doctor-kind-lut = LUT
doctor-kind-mask = قناع
history-relinkFiles = إعادة ربط الملفات

# --- ScriptsDialog.tsx ---
scripts-title = النصوص (Lua)
scripts-empty = لا نصوص بعد — أضف ملف .lua. راجع scripts/sample.lua للواجهة البرمجية: تفاعل مع أحداث البث المباشر/المشهد/التسجيل وقُد الأوامر نفسها كالواجهة عن بُعد.
scripts-enable = تفعيل { $path }
scripts-remove = إزالة { $path }
scripts-path-label = مسار النص
scripts-add = إضافة
scripts-note = تعمل النصوص في بيئة معزولة — بلا وصول للملفات أو نظام التشغيل؛ يمكنها فقط استدعاء أوامر الاستوديو نفسها كالواجهة عن بُعد (تبديل المشاهد، الانتقال، التسجيل/البث/الإعادة، الكتم). خطأ النص يُسجَّل ويُحتوى. تُطبَّق التغييرات خلال ثانية.
scripts-error-not-lua = أشِر إلى ملف .lua.

# --- BrowserDock.tsx ---
browser-dock-title = أرصفة المتصفح
browser-dock-empty = لا أرصفة بعد — أضف نافذة دردشة منبثقة أو صفحة تنبيهات أو أزرار Companion على الويب.
browser-dock-open = فتح
browser-dock-remove = إزالة { $name }
browser-dock-name-placeholder = الاسم (مثل دردشة Twitch)
browser-dock-name-label = اسم الرصيف
browser-dock-url-label = عنوان الرصيف
browser-dock-note = يفتح الرصيف كنافذته الخاصة يمكنك وضعها بجانب الاستوديو. لا تحصل الصفحة على أي وصول للتطبيق — تُصيَّر فقط. عناوين http(s) فقط؛ تُفتح الأرصفة فقط عند نقر فتح.
browser-dock-error-name = سمِّ الرصيف (مثل دردشة Twitch).
browser-dock-error-url = يجب أن يبدأ عنوان الرصيف بـ http:// أو https://.

# --- studio-preview-pane ---
studio-preview-label = معاينة وضع الاستوديو
studio-preview-heading = المعاينة
studio-preview-hint = انقر مشهدًا لتحميله هنا
studio-preview-empty = ستظهر المعاينة هنا.
studio-preview-mirrors = يعكس البرنامج
studio-preview-transition-select = الانتقال
studio-preview-duration = مدة الانتقال (ms)
studio-preview-commit-title = تثبيت المعاينة → البرنامج عبر الانتقال (يراه الجمهور)
studio-preview-transitioning = جارٍ الانتقال…
studio-preview-transition-button = انتقال ⇄
studio-preview-luma-placeholder = صورة مسح بتدرج رمادي (png/jpg)
studio-preview-luma-label = صورة مسح لوما
studio-preview-browse = استعراض…
studio-preview-filter-images = الصور
studio-preview-filter-video = الفيديو
studio-preview-stinger-placeholder = فيديو ستينجر (ProRes 4444 .mov يحتفظ بقناة ألفا)
studio-preview-stinger-label = ملف فيديو الستينجر
studio-preview-stinger-cut-label = نقطة قطع الستينجر (ms)
studio-preview-stinger-cut-title = متى يقع تبديل المشهد تحت الستينجر (ms داخل الانتقال)
studio-preview-stinger-matte-label = قناع المسار
studio-preview-stinger-matte-title = كيف يحزم ستينجر قناع المسار الشفافية: التعبئة وقناعها جنبًا إلى جنب (أفقي) أو مكدّسين (رأسي)
studio-preview-stinger-duck-label = خفض البرنامج
studio-preview-stinger-duck-title = اخفض صوت البرنامج تحت صوت الستينجر نفسه أثناء تشغيله (0 = إيقاف)
studio-preview-stinger-duck-unit = dB

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = قطع
transition-kind-fade = تلاشٍ
transition-kind-slide-left = انزلاق ←
transition-kind-slide-right = انزلاق →
transition-kind-slide-up = انزلاق ↑
transition-kind-slide-down = انزلاق ↓
transition-kind-swipe-left = تمرير ←
transition-kind-swipe-right = تمرير →
transition-kind-luma-linear = مسح لوما (خطي)
transition-kind-luma-radial = مسح لوما (شعاعي)
transition-kind-luma-horizontal = مسح لوما (أفقي)
transition-kind-luma-diamond = مسح لوما (معيّن)
transition-kind-luma-clock = مسح لوما (ساعة)
transition-kind-image = مسح بصورة (مخصص)
transition-kind-stinger = ستينجر (فيديو)
transition-kind-move = تحريك (تحوّل)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = بلا
stinger-matte-horizontal = جنبًا إلى جنب
stinger-matte-vertical = مكدّس

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = مخصص (RTMP/RTMPS)
stream-service-srt = SRT (مستضاف ذاتيًا)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = حول
about-tagline = سجّل وابثّ كاستوديو — بلا حسابات، بلا سحابة.
about-version = الإصدار
about-created-by = من إنشاء
about-project-started = بدء المشروع
about-first-stable = أول إصدار مستقر
about-first-stable-pending = ليس بعد — 1.0.0 قيد التنفيذ
about-platform = المنصة
about-local-first = يعمل Freally Capture بالكامل على جهازك. بلا حسابات، بلا قياس عن بُعد، بلا سحابة — الشيء الوحيد الذي يغادر حاسوبك هو البث الذي اخترت إرساله.
about-website = الموقع الإلكتروني
about-issues = الإبلاغ عن مشكلة
about-license = الترخيص
about-eula = EULA
about-third-party = إشعارات الأطراف الثالثة
about-check-updates = تحقّق من التحديثات…

# --- unified settings modal (TASK-906) ---
settings-title = الإعدادات
settings-language-section = اللغة
settings-language = لغة الواجهة
settings-language-system = افتراضي النظام
settings-language-note = تُحفَظ اللغة التي تختارها هنا. "افتراضي النظام" يتبع نظام تشغيلك. النص غير المُترجَم يعود إلى الإنجليزية.
settings-appearance-section = المظهر
settings-theme = السمة
settings-theme-dark = داكن
settings-theme-light = فاتح
settings-theme-custom = مخصص
settings-accent = لون التمييز
settings-general-section = عام
settings-show-stats-dock = إظهار لوحة الإحصائيات
settings-open-about = حول…

# --- command palette (TASK-904) ---
palette-title = لوحة الأوامر
palette-search = البحث في المشاهد والمصادر والإجراءات
palette-placeholder = ابحث في المشاهد والمصادر والإجراءات…
palette-no-results = لا شيء يطابق “{ $query }”
palette-hint = ↑ ↓ للتنقّل · Enter للتشغيل · Esc للإغلاق
palette-group-scenes = مشهد
palette-group-sources = مصدر
palette-group-actions = إجراء
palette-transition = انتقال المعاينة → البرنامج
palette-save-replay = حفظ الإعادة
palette-add-marker = وضع علامة فصل
palette-vertical-canvas = اللوحة العمودية (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = مرحبًا بك في Freally Capture
wizard-welcome = خطوتان سريعتان: نتحقّق ممّا يستطيع جهازك فعله، ثم نبدأ مشهدًا. يستغرق الأمر نحو ثلاثين ثانية، ويمكنك تغيير كل شيء لاحقًا.
wizard-local-first = لا شيء هنا يغادر حاسوبك. لا يملك Freally Capture حسابات ولا قياسًا عن بُعد ولا سحابة.
wizard-start = لنبدأ
wizard-skip = تخطّي
wizard-hardware-title = ما يستطيع جهازك فعله
wizard-probing = نتحقّق من بطاقة الرسوميات والمعالج لديك…
wizard-encoder = المُرمِّز
wizard-canvas = اللوحة
wizard-bitrate = معدل البت
wizard-probe-found = وُجد: { $gpus } · { $cores } نواة فعلية
wizard-no-gpu = لا GPU مخصّص
wizard-apply = استخدم هذه الإعدادات
wizard-keep-current = أبقِ ما لديّ
wizard-template-title = ابدأ بمشهد
wizard-template-screen = التقط شاشتي
wizard-template-screen-note = يضيف التقاط الشاشة لشاشتك الرئيسية. أكثر نقطة بداية شيوعًا.
wizard-template-empty = ابدأ فارغًا
wizard-template-empty-note = مشهد فارغ. أضف المصادر بنفسك بزر +.
wizard-done = أصبح كل شيء جاهزًا.
wizard-done-hint = اضغط Ctrl+K في أي وقت للبحث في المشاهد والمصادر والإجراءات. الإعدادات خلف زر ⚙.
wizard-close = ابدأ البث

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = تستطيع بطاقة الرسوميات ترميز الفيديو بنفسها، فيبقى المعالج متفرّغًا لبقية الاستوديو.
autoconfig-reason-software = لم يُعثر على مُرمِّز عتادي صالح، لذا سيتولّى المعالج الترميز. هذا يعمل، لكنه يستهلك المزيد من CPU.
autoconfig-reason-quality-hardware = 1080p عند 60 إطارًا في الثانية، بمعدل بت تقبله كل منصة كبرى.
autoconfig-reason-quality-software = 30 إطارًا في الثانية، لأن الترميز البرمجي عند 60 يُسقِط إطارات على معظم المعالجات.
autoconfig-reason-quality-low-cores = معدل بت أقل، لأن هذا المعالج قليل الأنوية وسينافس الترميز البرمجي المُركِّب عليها.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = بدأ التسجيل
announce-recording-paused = توقف التسجيل مؤقتا
announce-recording-stopped = توقف التسجيل
announce-live-started = أنت الآن على الهواء مباشرة
announce-live-ended = انتهى البث المباشر
announce-reconnecting = انقطع الاتصال، جارٍ إعادة الاتصال
announce-stream-failed = فشل البث المباشر
announce-frames-dropped = أسقط { $count } من الإطارات

# CAP-M01 — undo/redo edit history
palette-undo = تراجع
palette-redo = إعادة
palette-edit-history = سجل التعديلات…
history-title = سجل التعديلات
history-empty = لا يوجد ما يمكن التراجع عنه بعد.
history-current = الحالة الحالية
history-close = إغلاق
history-addScene = إضافة مشهد
history-renameScene = إعادة تسمية المشهد
history-removeScene = إزالة المشهد
history-reorderScene = إعادة ترتيب المشاهد
history-addSource = إضافة مصدر
history-removeSource = إزالة المصدر
history-reorderSource = إعادة ترتيب المصادر
history-renameSource = إعادة تسمية المصدر
history-transformSource = نقل المصدر
history-toggleVisibility = تبديل الظهور
history-toggleLock = تبديل القفل
history-setBlendMode = تغيير وضع المزج
history-editSourceProperties = تعديل الخصائص
history-applyLayout = ترتيب التخطيط
history-moveToSeat = النقل إلى الموضع
history-groupSources = تجميع المصادر
history-ungroupSources = فك تجميع المصادر
history-toggleGroupVisibility = تبديل المجموعة
history-setSceneAudio = صوت المشهد
history-setVerticalCanvas = لوحة عمودية
history-addFilter = إضافة مرشِّح
history-removeFilter = إزالة المرشِّح
history-reorderFilter = إعادة ترتيب المرشِّحات
history-editFilter = تعديل المرشِّح
history-toggleFilter = تبديل المرشِّح
history-setVolume = ضبط مستوى الصوت
history-toggleMute = تبديل الكتم
history-setMonitor = تغيير المراقبة
history-setTracks = تغيير المسارات
history-setSyncOffset = ضبط تزامن الصوت/الصورة
history-setAudioHotkeys = اختصارات الصوت

# CAP-M04 — alignment aids
settings-alignment-section = أدوات المحاذاة
settings-smart-guides = خطوط إرشادية ذكية (محاذاة أثناء السحب)
settings-safe-areas = تراكبات المنطقة الآمنة
settings-rulers = المساطر
align-group = محاذاة إلى اللوحة
align-left = محاذاة لليسار
align-hcenter = توسيط أفقي
align-right = محاذاة لليمين
align-top = محاذاة للأعلى
align-vcenter = توسيط رأسي
align-bottom = محاذاة للأسفل

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = محاذاة وتوزيع المحدد
arrange-left = محاذاة الحواف اليسرى
arrange-hcenter = توسيط أفقيًا
arrange-right = محاذاة الحواف اليمنى
arrange-top = محاذاة الحواف العلوية
arrange-vcenter = توسيط رأسيًا
arrange-bottom = محاذاة الحواف السفلية
distribute-h = توزيع أفقيًا
distribute-v = توزيع رأسيًا
guides-group = الأدلة
guides-add-v = إضافة دليل رأسي
guides-add-h = إضافة دليل أفقي
guides-clear = إزالة كل الأدلة
history-arrangeItems = ترتيب العناصر
history-editGuides = تحرير الأدلة

# CAP-M05 — edit transform + copy/paste
transform-title = تعديل التحويل — { $name }
transform-anchor = نقطة الارتكاز
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = التدوير
transform-crop = الاقتصاص
transform-crop-left = يسار
transform-crop-top = أعلى
transform-crop-right = يمين
transform-crop-bottom = أسفل
transform-no-size = يتوفر الحجم والاقتصاص بمجرد أن يبلغ المصدر عن أبعاده.
transform-copy = نسخ التحويل
transform-paste = لصق التحويل
transform-close = إغلاق
filters-copy = نسخ المرشِّحات ({ $count })
filters-paste = لصق المرشِّحات ({ $count })
palette-edit-transform = تعديل التحويل…
history-pasteFilters = لصق المرشِّحات

# CAP-M26 — keying workbench
workbench-title = طاولة إزالة الخلفية — { $name }
workbench-mode-keyed = بعد الإزالة
workbench-mode-source = المصدر
workbench-mode-matte = القناع
workbench-mode-split = مقسَّم
workbench-eyedropper = قطارة
workbench-eyedropper-hint = انقر على المصدر لأخذ عينة من لون المفتاح.
workbench-loupe = عدسة مكبِّرة
workbench-split = التقسيم
workbench-preview-alt = معاينة طاولة إزالة الخلفية
workbench-tune = ضبط
workbench-close = إغلاق

# CAP-M06 — multiview monitor
multiview-title = العرض المتعدد
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = انقر على مشهد للانتقال إليه.
multiview-hint-stage = انقر على مشهد لتحضيره في المعاينة.
palette-multiview = شاشة العرض المتعدد

# CAP-M07 — projectors
projector-title = فتح جهاز العرض
projector-source = المصدر
projector-target-program = البرنامج
projector-target-preview = المعاينة
projector-target-scene = مشهد…
projector-target-source = مصدر…
projector-target-multiview = العرض المتعدد
projector-which-scene = أي مشهد
projector-which-source = أي مصدر
projector-none = لا شيء لعرضه
projector-display = الشاشة
projector-windowed = نافذة عائمة (هذه الشاشة)
projector-display-option = الشاشة { $n } — { $w }×{ $h }
projector-primary = (رئيسية)
projector-open = فتح
projector-cancel = إلغاء
projector-exit-hint = اضغط Esc للخروج
palette-projector = فتح جهاز العرض…

# CAP-M08 — still-frame grab
palette-still = التقاط إطار ثابت…
still-saved-toast = تم حفظ الإطار: { $name }
still-failed-toast = فشل التقاط الإطار: { $error }
hotkeys-still = التقاط إطار ثابت

# CAP-M13 — source health dashboard
palette-source-health = صحة المصادر…
palette-av-sync = معايرة تزامن الصوت والصورة…
palette-hotkey-audit = خريطة الاختصارات…
health-title = صحة المصادر
health-col-source = المصدر
health-col-state = الحالة
health-col-resolution = الدقة
health-col-fps = FPS
health-col-last-frame = آخر إطار
health-col-dropped = مُسقَطة
health-col-retries = عمليات إعادة التشغيل
health-col-actions = إجراءات
health-state-live = مباشر
health-state-waiting = في الانتظار
health-state-error = خطأ
health-state-inactive = غير نشط
health-restart = إعادة تشغيل
health-properties = خصائص
health-empty = لا تحتوي هذه المجموعة على مصادر بعد.
health-seconds = { $value } ث

# CAP-M23 — quit guard + orderly shutdown
quit-title = إنهاء Freally Capture؟
quit-body = الإنهاء الآن سينفّذ ما يلي بأمان وبالترتيب:
quit-consequence-stream = إنهاء البث المباشر وقطع الاتصال بالخدمة.
quit-consequence-recording = إيقاف التسجيل وإتمام ملفاته.
quit-consequence-replay = إيقاف مخزن الإعادة — تُتجاهل لقطات الإعادة غير المحفوظة.
quit-confirm = إنهاء بأمان
quit-quitting = جارٍ الإغلاق…
quit-cancel = إلغاء

# CAP-M11 — crash-safe recording salvage
salvage-title = استعادة التسجيلات المتقطعة؟
salvage-body = انتهت الجلسة الأخيرة بشكل غير متوقع أثناء كتابة هذه التسجيلات. يُنشئ الإصلاح نسخة قابلة للتشغيل بجوار الأصل — لا يُعدَّل الملف الأصلي أبدًا.
salvage-repair = إصلاح
salvage-repairing = جارٍ الإصلاح…
salvage-done = تم الإصلاح
salvage-repaired = تم الإصلاح ← { $name }
salvage-failed = فشل الإصلاح: { $error }
salvage-dismiss = ليس الآن

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = عطل في المُرمِّز — تم التبديل من { $from } إلى { $to }. أعاد البث الاتصال ويستمر.
fallback-toast-recording = عطل في المُرمِّز — تم التبديل من { $from } إلى { $to }. يستمر التسجيل في ملف جديد.
fallback-note = مُرمِّز احتياطي: { $from } ← { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = صمت صوت البرنامج
alarm-clipping = صوت البرنامج يتشبّع
alarm-black = صورة البرنامج سوداء
alarm-frozen = صورة البرنامج لم تتغير منذ فترة
alarm-lowDisk = مساحة القرص: يتبقى نحو { $minutes } دقيقة بمعدل البت الحالي
alarm-dismiss = إغلاق التنبيه
alarm-cleared = تم الحل: { $alarm }

# CAP-M22 — panic button
palette-panic = الطوارئ — القطع إلى شاشة الخصوصية
panic-banner-title = الطوارئ
panic-banner-body = يعرض البرنامج شاشة الخصوصية؛ كل الصوت مكتوم والالتقاط متوقف. يستمر البث والتسجيل.
panic-restore = استعادة…
panic-restore-confirm = استعادة البرنامج؟
panic-restore-yes = استعادة
panic-restore-cancel = إلغاء
hotkeys-panic = الطوارئ (شاشة الخصوصية)
hotkeys-timer-toggle = بدء/إيقاف كل المؤقّتات
hotkeys-timer-reset = تصفير كل المؤقّتات
panic-slate-color = لون شاشة الطوارئ
panic-slate-image = صورة شاشة الطوارئ
panic-slate-image-placeholder = مسار صورة اختياري

# CAP-M24 — redacted diagnostics bundle
diag-title = حزمة التشخيص
diag-intro = تصدير ملف .zip منقّح (لقطة الإعدادات، فحص المُرمِّزات، إحصاءات حديثة — لا تُضمَّن الأسرار والمسارات والأسماء أبدًا) لإرفاقه يدويًا بمشكلة على GitHub. لا يُرسَل شيء إلى أي مكان.
diag-preview = عرض المحتوى
diag-hide-preview = إخفاء المعاينة
diag-export = تصدير .zip
diag-exported = تم التصدير: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = فحص ما قبل البث
preflight-intro = يجب أن يكون كل بند مانع أخضر؛ والبقية تنبيهات صادقة.
preflight-item-targets = وجهات البث مُهيأة (مفتاح/رابط)
preflight-item-encoder = يتوفر مُرمِّز صالح
preflight-item-sources = كل المصادر سليمة
preflight-item-disk = مساحة قرص للتسجيل
preflight-item-mic = قياس الميكروفون
preflight-item-desktopAudio = قياس صوت سطح المكتب
preflight-item-replay = مخزن الإعادة جاهز
preflight-targets-detail = { $count } مفعّلة
preflight-sources-detail = { $count } مصدر/مصادر بخطأ
preflight-disk-detail = ~{ $minutes } دقيقة بمعدل البت الحالي
preflight-fix-stream = إعدادات البث…
preflight-fix-components = المكوّنات…
preflight-fix-sources = صحة المصادر…
preflight-fix-replay = تجهيز
preflight-optional = اختياري
preflight-hold = منع البث حتى يصبح كل شيء أخضر
preflight-cancel = إلغاء
preflight-go-anyway = ابدأ البث رغم ذلك
preflight-go-live = ابدأ البث


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = الخلفية
scenes-backdrop-aria = خلفية { $name }
backdrop-title = الخلفية — { $name }
backdrop-hint = خلفية مثبّتة خلف كل شيء في هذا المشهد — صورة أو GIF متحرك أو فيديو متكرر. يبقى الالتقاط دائمًا في الأعلى؛ مرّر فوق اللوحة للتكبير.
backdrop-choose = اختر صورة أو فيديو…
backdrop-remove = إزالة الخلفية
backdrop-none = لا توجد خلفية.
backdrop-position = الموضع
backdrop-split-full = اللوحة كاملة
backdrop-split-left = النصف الأيسر
backdrop-split-right = النصف الأيمن
backdrop-split-top = النصف العلوي
backdrop-split-bottom = النصف السفلي
backdrop-sync = بدء التشغيل مع بدء التسجيل
backdrop-sync-hint = يتوقف عند أول إطار حتى تبدأ التسجيل؛ وتبدأ كل لقطة الفيديو من البداية.
backdrop-preview-play = تشغيل المعاينة
backdrop-preview-pause = إيقاف المعاينة مؤقتًا
backdrop-filter-all = خلفيات (صور وفيديو)
backdrop-filter-images = صور
backdrop-filter-media = فيديو وGIF
sources-backdrop-badge = خلفية مثبّتة في الأسفل
sources-backdrop-pinned = تبقى الخلفية مثبّتة في الأسفل
filters-name-flip = قلب
filters-flip-horizontal = أفقي
filters-flip-vertical = رأسي
history-setSceneBackdrop = تعيين الخلفية
history-setBackdropSplit = نقل الخلفية
history-setBackdropSync = مزامنة الخلفية مع التسجيل
backdrop-scrub = موضع التشغيل
backdrop-loop = تكرار
backdrop-reverse = تشغيل بالعكس
backdrop-reverse-hint = يُنشئ العكس نسخة معكوسة مرة واحدة (تتطلب مقاطع الفيديو مكوّن ffmpeg؛ أما GIF فينعكس فورًا) — قد يستغرق أول تبديل وقتًا مع الملفات الطويلة.
filters-scaling = التحجيم
filters-scaling-hint = أوضاع دقيقة البكسل لمحتوى الريترو/البكسل؛ «صحيح» يثبّت أيضًا الحجم المرسوم على مضاعفات كاملة (تعرض المقابض الحجم المنطقي).
filters-scaling-auto = ناعم
filters-scaling-nearest = أقرب جار
filters-scaling-integer = صحيح (مضاعفات كاملة)
filters-scaling-sharp = ثنائي خطي حاد
history-setScaling = تغيير التحجيم
hotkeys-zoom-100 = التكبير: إعادة تعيين (100%)
hotkeys-zoom-150 = التكبير: اقترب إلى 150%
hotkeys-zoom-200 = التكبير: اقترب 2×
sources-follow-title = تتبّع المؤشر أثناء التكبير (ويندوز؛ مرّر فوق اللوحة للتكبير)
sources-follow-item = تبديل تتبّع المؤشر لـ { $name }
filters-autocrop = ✂ قصّ الأشرطة السوداء تلقائيًا
filters-autocrop-title = يفحص الإطار التالي بحثًا عن أشرطة سوداء ويقصّها (قابل للتراجع). المشاهد الداكنة لا تُقصّ أبدًا.
filters-autocrop-follow = إعادة الفحص عند تغيّر الدقة
history-autoCrop = قصّ الأشرطة السوداء تلقائيًا
sources-link-audio = التقط أيضًا صوت هذا التطبيق (مرتبط: الإخفاء يكتمه، وإزالة النافذة تزيله)
history-addLinkedWindow = إضافة نافذة + صوت مرتبط
sources-hdr-title = هذه الشاشة HDR — افتح مخطط الدرجات (تبقى اللوحة SDR)
sources-hdr-item = مخطط درجات HDR لـ { $name }
sources-hdr-dialog-title = HDR ← SDR — { $name }
sources-hdr-hint = هذه الشاشة تُخرج HDR. بدون مخطط الدرجات تُقصّ الإضاءات ويبدو الالتقاط باهتًا على لوحة SDR. تسري التغييرات من الإطار التالي.
sources-hdr-enable-suggested = تفعيل المقترح (maxRGB، 200 شمعة)
sources-hdr-operator = المُعامل
sources-hdr-op-clip = قصّ (إيقاف)
sources-hdr-op-maxrgb = maxRGB (يحافظ على اللون)
sources-hdr-op-reinhard = راينهارد
sources-hdr-op-bt2408 = ركبة BT.2408 (يُبقي SDR كما هو)
sources-hdr-paper-white = أبيض الورق
sources-hdr-nits = شمعة/م²
projector-target-passthrough = شاشة تمرير مباشر (زمن استجابة منخفض)
projector-which-device = الجهاز
projector-passthrough-none = أضف أولًا شاشة أو نافذة أو جهاز التقاط.
projector-passthrough-about = إطارات الجهاز الخام — بلا مشاهد ولا فلاتر ولا مُركِّب. يعرض زمن استجابة مقيسًا؛ ويظل الصوت يُراقَب عبر قناة الخلّاط.
projector-passthrough-hint = تمرير مباشر — Esc للإغلاق
projector-latency = { $ms } مللي ثانية
projector-latency-measuring = جارٍ القياس…
automation-title = الأتمتة — القواعد والماكرو والمتغيرات
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = القواعد
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = مفعّلة
automation-rule-name = Rule name
automation-remove = Remove
automation-when = عندما
automation-then-run = عندئذٍ شغّل
automation-no-macro = (no macro)
automation-macros = الماكرو
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = تشغيل
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = متغيرات الاستوديو
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
rundown-title = جدول البرنامج
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = ابدأ
rundown-next = التالي ▸
rundown-stop = إيقاف
rundown-idle = غير مُشغَّل
rundown-next-up = التالي: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + خطوة
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
automation-layer = طبقة
automation-layer-hint = يعمل فقط أثناء تفعيل هذه الطبقة (فارغ = كل الطبقات). الطبقات ثابتة: مفتاح الطبقة يبدّل ويبقى (واجهة اختصارات النظام لا تدعم طبقة بالضغط المستمر).
automation-chord-hint = مفتاح عادي (Ctrl+Shift+M) أو تركيبة من ضغطتين (Ctrl+K, 3). لا يُحجز المفتاح الثاني إلا أثناء انتظار التركيبة.
panel-title = لوحة الشبكة وتالي
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = تشغيل اللوحة
panel-port = المنفذ
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = كلمة المرور
panel-show = إظهار
panel-hide = إخفاء
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = حفظ
osc-title = سطح تحكم OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = الاستماع إلى OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = كاميرات PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = الكاميرا
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = العنوان
ptz-port = المنفذ
ptz-speed = السرعة
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
ptz-presets = الإعدادات المسبقة
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = استدعاء
ptz-store = تخزين
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = سطح تحكم MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = الإدخال
midi-output = الإخراج (تغذية راجعة)
midi-none = (none)
midi-learn = تعلّم
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = الإجراء
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
panel-lan-warning = ⚠ حركة الشبكة غير مشفَّرة — كلمة المرور تظهر في الرابط عبر HTTP. استخدمها فقط على شبكة موثوقة.
osc-lan-warning = ⚠ لا كلمة مرور لـ OSC — أي جهاز على الشبكة يمكنه إرسال هذه الأوامر. استخدم وضع الشبكة على شبكة موثوقة فقط.

# System-stats HUD source (CAP-N14)
sources-badge-stats = إحصاء
sources-add-system-stats = إحصاءات الأداء (HUD)
sources-stats-title = إضافة شاشة أداء (HUD)
sources-stats-note = يعرض للمشاهدين في البرنامج أرقام الاستوديو المقيسة فعليًا — الإطارات في الثانية وCPU والذاكرة وزمن التصيير والإطارات المفقودة ومعدل البث المباشر. اختيار الأسطر والحجم واللون في خصائص المصدر. لا يُعرض استخدام GPU لأنه لا يُقاس.
sources-stats-add = إضافة HUD الإحصاءات
properties-stats-show-fps = إظهار FPS
properties-stats-show-cpu = إظهار CPU
properties-stats-show-memory = إظهار الذاكرة
properties-stats-show-render = إظهار زمن التصيير
properties-stats-show-dropped = إظهار الإطارات المفقودة
properties-stats-show-bitrate = إظهار معدل البث
properties-stats-show-timecode = عرض رمز التوقيت (LTC)
properties-stats-size = الحجم (px)
properties-stats-note = يرسم الـ HUD تسميات موجزة عالمية (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) مباشرة في البرنامج؛ وعندما لا يوجد بث يعرض سطر معدل البث «—».

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = مُصوِّر
sources-add-visualizer = مُصوِّر الصوت
sources-visualizer-title = إضافة مُصوِّر صوت
sources-visualizer-style-label = النمط
sources-visualizer-style-bars = أعمدة الطيف
sources-visualizer-style-scope = راسم الذبذبات
sources-visualizer-style-vu = مقاييس VU
sources-visualizer-target-label = يستمع إلى
sources-visualizer-target-master = المزيج الرئيسي
sources-visualizer-target-track = المسار { $n }
sources-visualizer-note = يرسم الإشارة التي تُمزج فعلًا (بعد المُنزلِق) — المصدر المكتوم يُرسم مسطحًا كما يُسمع تمامًا. الحجم واللون وعدد الأعمدة وسرعة الهبوط في خصائص المصدر.
sources-visualizer-add = إضافة المُصوِّر
properties-vis-bands = الأعمدة
properties-vis-decay = سرعة الهبوط (dB/s)
properties-vis-peak-hold = علامات ذروة ثابتة
properties-vis-missing-source = (مصدر مفقود)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = تقسيمات
sources-add-split-timer = مؤقّت تقسيمات السرعة
sources-splits-title = إضافة مؤقّت تقسيمات
sources-splits-file-label = ملف LiveSplit ‏.lss
sources-splits-comparison-label = المقارنة مع
sources-splits-comparison-pb = أفضل زمن شخصي
sources-splits-comparison-best = أفضل المقاطع
sources-splits-comparison-average = المتوسط
sources-splits-note = يستورد الملف للقراءة فقط — لا يُكتب إليه شيء أبدًا. اربط مفاتيح Split / Undo / Skip / Reset العامة من الإعدادات ← الاختصارات. لا تُدعم أدوات التقسيم التلقائي عبر ذاكرة العمليات عمدًا.
sources-splits-add = إضافة مؤقّت التقسيمات
properties-splits-size = الحجم (px)
properties-splits-ahead = متقدّم
properties-splits-behind = متأخّر
properties-splits-gold = ذهبي
properties-splits-split = تقسيم
properties-splits-undo = تراجع
properties-splits-skip = تخطٍّ
properties-splits-reset = إعادة ضبط
properties-splits-note = الأزرار تتحكم بالمؤقّت الحي (والاختصارات العامة تفعل الشيء نفسه من أي تطبيق). لا يُحفَظ السباق في ملف ‎.lss أبدًا.
hotkeys-split-split = مؤقّت التقسيمات: بدء / تقسيم
hotkeys-split-undo = مؤقّت التقسيمات: تراجع عن التقسيم
hotkeys-split-skip = مؤقّت التقسيمات: تخطّي المقطع
hotkeys-split-reset = مؤقّت التقسيمات: إعادة ضبط
hotkey-audit-action-split-split = تقسيم (مؤقّت التقسيمات)
hotkey-audit-action-split-undo = تراجع عن التقسيم
hotkey-audit-action-split-skip = تخطّي المقطع
hotkey-audit-action-split-reset = إعادة ضبط مؤقّت التقسيمات
hotkey-audit-feature-split-timer = مؤقّت التقسيمات

# Media playlist source (CAP-N17)
sources-badge-playlist = قائمة تشغيل
sources-add-playlist = قائمة تشغيل وسائط (بدون فجوات)
sources-playlist-title = إضافة قائمة تشغيل وسائط
sources-playlist-files-label = الملفات (ملف في كل سطر، تُشغَّل من الأعلى إلى الأسفل)
sources-playlist-browse = استعراض…
sources-playlist-loop = تكرار
sources-playlist-shuffle = خلط (سحب واحد عند كل تشغيل؛ التكرار مع الخلط يعيد الترتيب نفسه)
sources-playlist-hold-last = تثبيت آخر إطار عند النهاية
sources-playlist-note = تُشغِّل القائمة المقتصّة كاملة بلا فجوات عبر مكوّن ffmpeg المُعلَن (صيغ wire فقط — ‎.frec والصور عبر الوسائط/عرض الشرائح). العناصر كلها فيديو أو كلها صوت، لا خلط. الاقتصاص ونقاط الإشارة ومتغيّر «قيد التشغيل» في خصائص المصدر.
sources-playlist-add = إضافة قائمة التشغيل
properties-playlist-items = العناصر (تُشغَّل من الأعلى إلى الأسفل)
properties-playlist-up = تحريك لأعلى
properties-playlist-down = تحريك لأسفل
properties-playlist-remove = إزالة العنصر
properties-playlist-in = من (ث)
properties-playlist-out = إلى (ث)
properties-playlist-cues = نقاط الإشارة (ث، مفصولة بفواصل)
properties-playlist-add-item = + إضافة عنصر
properties-playlist-loop = تكرار
properties-playlist-shuffle = خلط
properties-playlist-hold-last = تثبيت آخر إطار
properties-playlist-hw = فك ترميز عتادي
properties-playlist-variable = متغيّر «قيد التشغيل» (فارغ = إيقاف)
properties-playlist-previous = ⏮ السابق
properties-playlist-next = ⏭ التالي
properties-playlist-note = أزرار الإشارة والتالي/السابق تتحكم بالقائمة المُشغَّلة؛ تعديلات العناصر تُطبَّق عند «تطبيق» (تُعاد القائمة). ضع {"{{"}yourVariable{"}}"} في مصدر نص لعرض اسم العنصر قيد التشغيل.
hotkeys-playlist-next = قائمة التشغيل: العنصر التالي
hotkeys-playlist-previous = قائمة التشغيل: العنصر السابق
hotkey-audit-action-playlist-next = التالي في قائمة التشغيل
hotkey-audit-action-playlist-previous = السابق في قائمة التشغيل
hotkey-audit-feature-playlist = قائمة التشغيل

# Instant replay source (CAP-N10)
sources-badge-replay = إعادة
sources-add-replay = إعادة فورية
sources-replay-title = إضافة إعادة فورية
sources-replay-seconds-label = طول اللقطة (ثوانٍ)
sources-replay-speed-label = السرعة
sources-replay-speed-full = 100% (مع الصوت)
sources-replay-speed-half = حركة بطيئة 50% (صامتة)
sources-replay-speed-quarter = حركة بطيئة 25% (صامتة)
sources-replay-note = يبقى شفافًا حتى تُطلق الإعادة. فعِّل مخزن الإعادة (لوحة التحكم) واربط مفتاح الإطلاق — الإطلاق يلتقط آخر لحظات المخزن ويعرضها في البرنامج ثم يعود شفافًا.
sources-replay-add = إضافة الإعادة الفورية
properties-replay-roll = ⏵ إطلاق الإعادة
properties-replay-note = الإطلاق يلتقط مخزن الإعادة المُفعَّل في مقطع ويعرضه بالسرعة المختارة — إعادة توقيت، لا استيفاء إطارات. الحركة البطيئة صامتة عمدًا. يعمل التمرير والإيقاف أثناء العرض؛ وفي النهاية يعود المصدر شفافًا.
hotkeys-replay-roll = الإعادة الفورية: إطلاق
hotkey-audit-action-replay-roll = إطلاق الإعادة الفورية

# Input overlay source (CAP-N13)
sources-badge-input = إدخال
sources-add-input-overlay = طبقة الإدخال (مفاتيح/ذراع)
sources-input-title = إضافة طبقة إدخال
sources-input-layout-label = التخطيط
sources-input-layout-wasd = WASD + فأرة
sources-input-layout-keyboard = لوحة مفاتيح مدمجة + فأرة
sources-input-layout-gamepad = ذراع تحكم (عصاتان)
sources-input-layout-fightstick = عصا قتال
sources-input-color-label = المفاتيح
sources-input-accent-label = مضغوط
sources-input-privacy-note = الخصوصية: تُقرأ المدخلات فقط أثناء بث هذا المصدر في مشهد، وتُستطلع مفاتيح التخطيط الثابتة فقط — نظرة لحظية «هل هو مضغوط الآن؟» وليست أداة اعتراض أبدًا. لا يُسجَّل شيء ولا يُخزَّن ولا يُرسل إلى أي مكان؛ ولا يُلتقط النص المكتوب أبدًا.
sources-input-os-note = حالة لوحة المفاتيح والفأرة تُقرأ اليوم على Windows فقط — الأنظمة الأخرى ترسم المفاتيح غير مضغوطة (يُقال ذلك بصدق، لا تزييف). تعمل أذرع التحكم في كل مكان عبر مكتبة gilrs؛ يُرسم أول جهاز متصل، وبدون جهاز يبقى التخطيط غير مضغوط.
sources-input-add = إضافة طبقة الإدخال

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = تأثيرات المؤشر
filters-cursorfx-hint = في ويندوز (حيث يرسم التطبيق المؤشر بنفسه) تُرسم مباشرة داخل الالتقاط فتظهر في التسجيلات والبث. أما macOS ولينكس فيركّبان المؤشر على مستوى النظام، لذا هذه التأثيرات متاحة في ويندوز فقط. تسري التغييرات فورًا.
filters-cursorfx-halo = هالة المؤشر
filters-cursorfx-halo-color = اللون
filters-cursorfx-halo-radius = نصف القطر (px)
filters-cursorfx-ripples = موجات النقر
filters-cursorfx-left-color = النقر الأيسر
filters-cursorfx-right-color = النقر الأيمن
filters-cursorfx-keystrokes = عرض ضغطات المفاتيح
filters-cursorfx-keystrokes-hint = يعرض مجموعة مفاتيح ثابتة (حروف وأرقام ومفاتيح تعديل وأسهم) بجوار المؤشر ما دامت مضغوطة. لا تُقرأ المفاتيح إلا أثناء تفعيل هذا الخيار، وتُرسم مباشرة في الإطار، ولا تُخزَّن أو تُسجَّل أبدًا.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = عنوان
sources-add-title = عنوان / لوحة نتائج
sources-title-title = إضافة عنوان
sources-title-template-label = ابدأ من
sources-title-template-lower-third = شريط سفلي (شريط + اسم + سطر فرعي)
sources-title-template-scoreboard = لوحة نتائج (لوحة + 4 خلايا)
sources-title-template-blank = لوحة فارغة
sources-title-width-label = عرض اللوحة
sources-title-height-label = ارتفاع اللوحة
sources-title-template-name = الاسم
sources-title-template-subtitle = الصفة
sources-title-template-home = المضيف
sources-title-template-away = الضيف
sources-title-note = عناوين من طبقات (نص / صورة / صندوق) مع حركة دخول/خروج، تُركَّب محليًا — بلا مصدر متصفح. الطبقات وروابط الملفات و{"{{"}المتغيرات{"}}"} وأدوات التحكم المباشر كلها في خصائص المصدر.
sources-title-add = إضافة العنوان
properties-title-layers = الطبقات (تُرسم بالترتيب — الصفوف اللاحقة فوق)
properties-title-kind-text = نص
properties-title-kind-image = صورة
properties-title-kind-rect = صندوق
properties-title-x = X
properties-title-y = Y
properties-title-outline = حدود (px)
properties-title-outline-color = الحدود
properties-title-shadow = ظل
properties-title-animation = حركة الدخول/الخروج
properties-title-anim-none = بلا (قطع)
properties-title-anim-fade = تلاشٍ
properties-title-anim-slide-left = انزلاق لليسار
properties-title-anim-slide-up = انزلاق لأعلى
properties-title-anim-wipe = مسح
properties-title-duration = المدة (م.ث)
properties-title-fire-in = ▶ تشغيل الدخول
properties-title-fire-out = ◼ تشغيل الخروج
properties-title-set-live = تعيين مباشرة
properties-title-set-live-note = يدفع هذا النص إلى العنوان المباشر فورًا — دون تطبيق ودون إعادة تشغيل
properties-title-up = رفع الطبقة
properties-title-down = خفض الطبقة
properties-title-remove = إزالة الطبقة
properties-title-add-text = + نص
properties-title-add-image = + صورة
properties-title-add-rect = + صندوق
properties-title-note = تشغيل الدخول/الخروج و«تعيين مباشرة» يتحكمان في العنوان الجاري؛ تعديلات الطبقات تسري عند «تطبيق» (يُعاد تشغيل العنوان ويدخل من جديد). يمكن ربط خلايا النص بملف مراقب (خلية CSV / قيمة JSON / الملف كاملًا) وتفسير {"{{"}المتغيرات{"}}"} — و«تعيين مباشرة» يتقدم على كليهما.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = استقبال عبر الشبكة المحلية (مستمع SRT/RTMP)
sources-lan-title = إضافة مستمع استقبال عبر الشبكة المحلية
sources-lan-protocol-label = البروتوكول
sources-lan-protocol-srt = ‏SRT (قابل للتشفير — موصى به)
sources-lan-protocol-rtmp = ‏RTMP (بدون مصادقة)
sources-lan-port-label = المنفذ (1024–65535)
sources-lan-passphrase-label = عبارة المرور (فارغة = مفتوح)
sources-lan-passphrase-hint = عبارات مرور SRT من 10 إلى 79 حرفًا؛ ويجب أن يستخدم المرسل العبارة نفسها.
sources-lan-open-warning = بدون عبارة مرور: يمكن لأي شخص على هذه الشبكة تغذية هذا المصدر دون تشفير. عيّن واحدة ما لم تكن الشبكة لك وحدك.
sources-lan-rtmp-warning = ‏RTMP بلا مصادقة — يمكن لأي شخص على هذه الشبكة الإرسال إلى هذا المنفذ. فضّل SRT مع عبارة مرور.
sources-lan-url-label = وجّه تطبيق المرسل إلى
sources-lan-qr-aria = رمز QR لعنوان الاستقبال
sources-lan-note = شبكة محلية فقط: يستمع على العنوان المحلي لهذا الجهاز، وفقط ما دام المصدر موجودًا، ولا يلمس الإنترنت أبدًا — لا يغادر أي شيء الجهاز حتى يرسل مرسل على شبكتك أولًا. يمر فك الترميز عبر مكوّن ffmpeg المُسمّى بوضوح. تعرض اللوحة هذا العنوان حتى يتصل مرسل.
sources-lan-add = بدء الاستماع
properties-lan-note = تطبيق تغيير البروتوكول أو المنفذ أو عبارة المرور يعيد تشغيل المستمع — وعلى المرسل إعادة الاتصال. يُلائم البث داخل لوحة 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = وصلة
sources-add-freally-link = Freally Link (نسخة أخرى)
sources-link-title = إضافة Freally Link
sources-link-about = يستقبل برنامج نسخة أخرى من Freally Capture — الفيديو وصوت الماستر — عبر شبكتك الخاصة. فعّل أولًا «مخرج Freally Link» على النسخة المرسلة. الإصدار v1 يبث motion-JPEG عبر TCP: ممتاز على شبكة سلكية أو Wi-Fi جيد، وصريح بشأن عرض النطاق على الوصلات الضعيفة.
sources-link-scan = فحص الشبكة المحلية
sources-link-scanning = جارٍ الفحص…
sources-link-none = لم يُعثر على مخارج Freally Link. فعّل «مخرج Freally Link» على النسخة الأخرى (التحكم ← لوحة LAN) أو اكتب عنوانها أدناه.
sources-link-host = العنوان
sources-link-port = المنفذ
sources-link-key = مفتاح الاقتران
sources-link-key-hint = المفتاح من إعدادات «مخرج Freally Link» لدى المرسل — بدونه لا يرسل المرسل أي إطار.
sources-link-add = إضافة الوصلة
properties-link-note = دون اتصال يعرض المصدر واجهة «جارٍ الاتصال» ويعيد المحاولة تلقائيًا بمهلة متزايدة — ولا يتجمد أبدًا على إطار قديم. مستقبل واحد لكل مرسل؛ ويُعاد بلطف الاتصال بالمرسل المشغول.
link-title = مخرج Freally Link
link-about = شارك برنامج هذه النسخة — الفيديو وصوت الماستر — مع نسخة واحدة فقط من Freally Capture على شبكتك الخاصة؛ يظهر هناك كمصدر «Freally Link» (بث بجهازين، شاشات إضافية). معطّل افتراضيًا؛ لا شيء يُعلن أو يستمع حتى تفعيله. الإصدار v1 يبث motion-JPEG + صوتًا غير مضغوط عبر TCP — مصمم لشبكة سلكية أو Wi-Fi جيد، وليس للإنترنت أبدًا.
link-enable = مشاركة البرنامج على شبكتي
link-name = اسم النسخة
link-key = مفتاح الاقتران
link-key-hint = 8 أحرف على الأقل — يجب أن يُدخل المستقبِلون هذا المفتاح قبل إرسال أي إطار.
link-lan-warning = ⚠ يجب أن يقدّم المستقبِلون مفتاح الاقتران قبل إرسال أي شيء، لكن البث نفسه غير مشفّر في v1 — استخدمه فقط على شبكة تثق بها.
link-serving = يجد المستقبلون هذه النسخة عبر «فحص الشبكة المحلية» أو يضيفونها يدويًا على:
link-off-hint = فعّل المشاركة لفتح المنفذ والإعلان عن هذه النسخة لعمليات فحص LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = قائمة التطبيق
menu-file = ملف
menu-edit = تحرير
menu-view = عرض
menu-docks = الأرصفة
menu-profile = ملف التعريف
menu-collection = مجموعة المشاهد
menu-tools = أدوات
menu-help = مساعدة
menu-rename = إعادة تسمية
menu-remove = إزالة
menu-import = استيراد
menu-export = تصدير
menu-file-show-recordings = إظهار التسجيلات
menu-file-remux = إعادة تغليف إلى MP4…
menu-file-settings = الإعدادات…
menu-file-show-settings-folder = إظهار مجلد الإعدادات
menu-file-exit = خروج
menu-edit-undo = تراجع
menu-edit-redo = إعادة
menu-edit-history = سجل التعديلات…
menu-edit-copy-transform = نسخ التحويل
menu-edit-paste-transform = لصق التحويل
menu-edit-copy-filters = نسخ المرشِّحات
menu-edit-paste-filters = لصق المرشِّحات
menu-edit-transform = التحويل…
menu-edit-lock-preview = قفل المعاينة
menu-view-fullscreen = واجهة ملء الشاشة
menu-stats-dock = لوحة الإحصائيات
menu-view-multiview = شاشة العرض المتعدد…
menu-view-projectors = أجهزة العرض…
menu-view-source-health = صحة المصادر…
menu-view-still = التقاط إطار ثابت
menu-docks-browser = أرصفة المتصفح…
menu-docks-lock = قفل الأرصفة
menu-docks-reset = إعادة تعيين تخطيط الأرصفة
menu-profile-manage = إدارة ملفات التعريف…
menu-collection-manage = إدارة مجموعات المشاهد…
menu-collection-import-obs = استيراد من OBS…
menu-collection-missing = التحقق من الملفات المفقودة…
menu-tools-wizard = تشغيل معالج الإعداد
menu-tools-wizard-title = يعمل معالج الإعداد عند أول تشغيل؛ لا توجد بعد طريقة لإعادة تشغيله.
menu-tools-automation = قواعد الأتمتة والماكرو…
menu-tools-rundown = إظهار جدول البرنامج…
menu-tools-hotkeys = خريطة الاختصارات…
menu-tools-av-sync = معايرة تزامن الصوت والصورة…
menu-tools-scripts = نصوص Lua…
menu-tools-components = المكوّنات…
menu-tools-midi = تحكم MIDI…
menu-tools-ptz = كاميرات PTZ…
menu-tools-remote = واجهة برمجة التحكم عن بُعد…
menu-tools-panel = لوحة الشبكة وتالي…
menu-help-portal = بوابة المساعدة
menu-help-website = زيارة الموقع الإلكتروني
menu-help-discord = الانضمام إلى خادم Discord
menu-help-bug = أبلِغ عن خطأ…
menu-help-updates = تحقّق من التحديثات…
menu-help-whats-new = ما الجديد
menu-help-about = حول…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = فئات الإعدادات
settings-cat-general = عام
settings-cat-appearance = المظهر
settings-cat-streaming = البث
settings-cat-output = الإخراج
settings-cat-replay = الإعادة
settings-cat-hotkeys = الاختصارات
settings-cat-network = الشبكة
settings-cat-accessibility = إمكانية الوصول
settings-cat-about = حول
settings-ok = موافق
settings-cancel = إلغاء
settings-apply = تطبيق
settings-save = حفظ
settings-loading = جارٍ تحميل الإعدادات…
settings-hotkeys-filter = تصفية الاختصارات
settings-hotkeys-filter-placeholder = اكتب لتصفية الإجراءات أو المفاتيح…
settings-hotkeys-no-match = لا يوجد اختصار يطابق “{ $query }”.
settings-hotkey-none = بدون
settings-hotkey-group-ctrl = Ctrl + مفتاح
settings-hotkey-group-ctrl-shift = Ctrl + Shift + مفتاح
settings-hotkey-group-ctrl-alt = Ctrl + Alt + مفتاح
settings-hotkey-group-function = مفاتيح الوظائف
settings-hotkey-group-numpad = لوحة الأرقام
settings-panic-section = شاشة الطوارئ
settings-meter-section = مقاييس مستوى المازج
settings-meter-note = الألوان التي تتدرج عبرها مقاييس المستوى في مازج الصوت، من الهدوء إلى التشبع. الإعداد الآمن لعمى الألوان يستخدم تدرجًا من الأزرق إلى البرتقالي يبقى مقروءًا مع ضعف تمييز الأحمر والأخضر.
settings-meter-preset = ألوان المقياس
settings-meter-preset-default = أخضر / أصفر / أحمر
settings-meter-preset-colorblind = آمن لعمى الألوان (أزرق / برتقالي)
settings-meter-preset-custom = مخصص
settings-meter-low = عادي
settings-meter-mid = مرتفع
settings-meter-high = تشبع
settings-meter-preview = معاينة

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = ما الجديد
whats-new-loading = جارٍ تحميل ملاحظات الإصدار…
whats-new-version = ما الجديد في الإصدار { $version }
whats-new-empty = لا توجد ملاحظات إصدار لهذا الإصدار.
filters-name-directional-blur = ضبابية اتجاهية
filters-name-radial-blur = ضبابية شعاعية
filters-name-zoom-blur = ضبابية التكبير
filters-name-pixelate = تحويل إلى بكسل
filters-angle = الزاوية (°)
filters-center-x = المركز X
filters-center-y = المركز Y
filters-block-size = حجم الكتلة (بكسل)
filters-name-freeze = تجميد
filters-freeze-hint = عند التفعيل، يحتفظ هذا المصدر بإطاره الأخير — البرنامج والمعاينة والتسجيل والبث تتجمد جميعها معًا. بدّل هذا الفلتر للتجميد أو إلغاء التجميد.
transform-3d = إمالة ثلاثية الأبعاد
transform-rotation-x = إمالة X (°)
transform-rotation-y = إمالة Y (°)
transform-perspective = المنظور
transform-reveal = إظهار/إخفاء
transform-reveal-ms = ظهور تدريجي (م.ث)
sources-clone-title = استنساخ (نفس التغذية، فلاتر خاصة)
sources-clone-item = استنساخ { $name }
menu-tools-downstream = مفاتيح الإخراج النهائي…
menu-tools-transition-rules = قواعد الانتقال…
dsk-title = مفاتيح الإخراج النهائي
dsk-hint = تراكبات مركّبة على خرج البرنامج — فوق كل مشهد، وتبقى ثابتة عند تبديل المشاهد (شعار، شارة مباشر، شريط سفلي). أعلى القائمة يُرسم في المقدمة.
dsk-empty = لا توجد مفاتيح بعد — أضف مصدرًا لتراكبه على كل مشهد.
dsk-enable = تفعيل هذا المفتاح
dsk-move-up = تحريك للأعلى (في المقدمة)
dsk-move-down = تحريك للأسفل
dsk-remove = إزالة المفتاح
dsk-opacity = التعتيم
dsk-x = X (بكسل)
dsk-y = Y (بكسل)
dsk-scale = المقياس
dsk-add = + إضافة مفتاح
transition-rules-title = قواعد الانتقال
transition-rules-hint = امنح زوجًا من المشاهد انتقالاً خاصًا به. عند الانتقال من المشهد الأول إلى الثاني، يُستخدم هذا النوع وهذه المدة بدلاً من الإعداد الافتراضي (تظل قاعدة ستينجر/صورة تستخدم الملف المحدد في عناصر التحكم بالانتقال).
transition-rules-empty = لا توجد قواعد بعد — كل زوج من المشاهد يستخدم الانتقال الافتراضي.
transition-rules-from = من
transition-rules-to = إلى
transition-rules-kind = الانتقال
transition-rules-duration = المدة (م.ث)
transition-rules-add = إضافة قاعدة
transition-rules-remove = إزالة القاعدة
