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
sources-add-nested-scene = مشهد متداخل
sources-add-slideshow = عرض شرائح صور
sources-add-chat-overlay = تراكب الدردشة المباشرة
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
controls-pause-title-resume = استئناف — يستمر الملف كخط زمني واحد متصل
controls-pause-title-pause = إيقاف مؤقت — لا تُكتَب إطارات؛ الاستئناف يُكمل الملف نفسه القابل للتشغيل
controls-resume-recording = ▶ استئناف التسجيل
controls-pause-recording = ⏸ إيقاف التسجيل مؤقتًا
controls-reactions-label = التفاعلات (مدمجة في البرنامج)
controls-reactions-title = أطلِق تفاعلًا فوق البرنامج — مُسجَّل ومَبثوث، فتُظهر الإعادة اللحظة بدقة. المشاهدون في الدردشة يُطلقونها أيضًا (تطفو إيموجي تفاعلهم تلقائيًا)؛ الفيضان يحدّ فقط ما يظهر على الشاشة.
controls-react = تفاعل { $emoji }
controls-virtual-camera-title = تحتاج الكاميرا الافتراضية إلى مكوّن مُشغّل موقّع خاص بها لكل نظام (Win11 MFCreateVirtualCamera / Win10 DirectShow / امتداد macOS CoreMediaIO / Linux v4l2loopback) — تأتي كمرحلة مستقلة. نموذج التغذية جاهز لها: البرنامج أو اللوحة العمودية أو مصدر واحد، مع ميكروفون افتراضي مقترن على Windows/Linux (لا يوجد في macOS واجهة برمجية لميكروفون افتراضي — نقولها بصراحة).
controls-virtual-camera = ⌁ بدء الكاميرا الافتراضية
controls-files-title = التسجيلات المكتملة + إجراء إعادة الحزم إلى mp4
controls-files = ▤ الملفات…
controls-output-title = تنسيق التسجيل والمُرمِّز والمجلد والمسارات والتقسيم
controls-output = ⚙ الإخراج…
controls-stream-title = هدف البث المباشر: الخدمة، مفتاح البث، المُرمِّز، معدل البت
controls-stream = ⦿ البث…
controls-codecs-title = مكوّن كودكات الشبكة ffmpeg عند الطلب (موسوم بوضوح، لا يُضمَّن أبدًا)
controls-codecs = ⬡ الكودكات…
controls-replay-title = طول مخزن الإعادة + إعدادات الجودة المسبقة
controls-replay = ⟲ الإعادة…
controls-keys-title = اختصارات عامة: التسجيل، البث المباشر، الانتقال، حفظ الإعادة
controls-keys = ⌨ المفاتيح…
controls-scripts-title = نصوص Lua في بيئة معزولة: تتفاعل مع أحداث البث المباشر/المشهد/التسجيل، وتقود الاستوديو
controls-scripts = ⚡ النصوص…
controls-docks-title = أرصفة المتصفح: افتح نافذة دردشة منبثقة أو صفحة تنبيهات أو أزرار Companion كنافذة بجانب الاستوديو
controls-docks = ⧉ الأرصفة…
controls-remote-title = واجهة برمجية عن بُعد عبر WebSocket لوحدات تحكم Stream Deck / Companion (مُعطَّلة افتراضيًا)
controls-remote = ⌁ عن بُعد…
controls-profiles-title = ملفات التعريف (الإعدادات) + مجموعات المشاهد — لقطات قابلة للتبديل
controls-profiles = ▣ ملفات التعريف…
controls-bug-title = أبلِغ عن خطأ — مجهول، اختياري (لا يُرسَل شيء تلقائيًا)
controls-bug = 🐞 أبلِغ عن خطأ…
controls-updates-title = تحقّق من التحديثات — موقّعة، مُتحقَّق منها، لا شيء يُنزَّل بلا نقرة
controls-updates = ⭳ تحقّق من التحديثات…
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

# --- StatsDock.tsx ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = الذاكرة
stats-dropped = مُسقَطة
stats-render = التصيير
stats-gpu = GPU
stats-gpu-compositing = يُركِّب
stats-gpu-idle = خامل
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
audiofilters-title = فلاتر الصوت — { $name }
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
hotkeys-record-placeholder = مثال Ctrl+Shift+R
hotkeys-go-live = بث مباشر / إنهاء البث
hotkeys-go-live-placeholder = مثال Ctrl+Shift+L
hotkeys-transition = انتقال وضع الاستوديو
hotkeys-transition-placeholder = مثال Ctrl+Shift+T أو F13
hotkeys-save-replay = حفظ الإعادة (آخر N ثانية)
hotkeys-save-replay-placeholder = مثال Ctrl+Shift+S
hotkeys-add-marker = وضع علامة فصل (التسجيل)
hotkeys-add-marker-placeholder = مثال Ctrl+Shift+K
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
settings-more-section = إعدادات إضافية
settings-open-output = التسجيل…
settings-open-stream = البث…
settings-open-replay = الإعادة…
settings-open-hotkeys = الاختصارات…
settings-open-remote = الواجهة البرمجية عن بُعد…
settings-open-about = حول…
controls-settings = ⚙ الإعدادات…
controls-settings-title = اللغة والمظهر والتفضيلات على مستوى التطبيق

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
