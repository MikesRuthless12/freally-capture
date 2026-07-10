# Freally Capture — hi
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = स्टूडियो मोड
toggle-on = चालू
toggle-off = बंद
stats = आँकड़े
core-ok = कोर ठीक
hide-stats-dock = आँकड़े डॉक छिपाएँ
show-stats-dock = आँकड़े डॉक दिखाएँ


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = सेटिंग्स सहेजी नहीं जा सकीं — रीस्टार्ट के बाद यह बदलाव नहीं बचेगा।
studio-mode-leave = स्टूडियो मोड छोड़ें
studio-mode-enter-title = स्टूडियो मोड — एक प्रीव्यू सीन एडिट करें, ट्रांज़िशन के साथ इसे प्रोग्राम में कमिट करें
vertical-canvas-title = दूसरा (वर्टिकल 9:16) आउटपुट कैनवास — स्वतंत्र रूप से रिकॉर्ड और स्ट्रीम किया जा सकता है
app-version = v{ $version }
core-error = कोर त्रुटि
core-unreachable = कोर तक नहीं पहुँच सका (ब्राउज़र मोड)
connecting-to-core = कोर से कनेक्ट हो रहा है…
filters-source-fallback = सोर्स

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = प्रोग्राम प्रीव्यू
preview-program-output = प्रोग्राम आउटपुट
preview-canvas-editor = कैनवास एडिटर
preview-px-to-edge-label = फ़्रेम किनारों तक पिक्सेल
preview-px-to-edge = किनारे तक px — L { $left } · T { $top } · R { $right } · B { $bottom }
preview-program-heading = प्रोग्राम
preview-no-gpu = कोई उपयोगी GPU एडाप्टर नहीं मिला — इस मशीन पर कम्पोज़िटर नहीं चल सकता।
preview-starting-compositor = कम्पोज़िटर शुरू हो रहा है…
preview-empty-scene = यह सीन खाली है — Sources में एक सोर्स जोड़ें, फिर उसे यहीं कैनवास पर ड्रैग, स्केल और रोटेट करें।
preview-fps = { $fps } fps
preview-dropped = { $dropped } ड्रॉप हुए

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = इनवाइट लिंक मिला
remote-join-with-webcam = वेबकैम के साथ जुड़ें
remote-dismiss = खारिज करें
remote-hosting-guest = एक रिमोट गेस्ट होस्ट कर रहे हैं
remote-you-are-guest = आप एक रिमोट गेस्ट हैं
remote-share-view-title = गेस्ट के ऐप में अपनी स्क्रीन साझा करें (वे आपका व्यू लाइव देखते हैं)
remote-stop-sharing-view = व्यू साझा करना रोकें
remote-share-my-view = मेरा व्यू साझा करें
remote-allow-center-title = गेस्ट को यह बदलने दें कि कौन-सा व्यू केंद्र में रहे (नियंत्रण आपके पास रहता है और आप कभी भी वापस बदल सकते हैं)
remote-guest-switching = गेस्ट स्विचिंग:
remote-stop-screen = स्क्रीन रोकें
remote-share-screen = स्क्रीन साझा करें
remote-share-screen-title-guest = होस्ट के साथ अपनी स्क्रीन साझा करें (यह एक सोर्स बन जाता है जिसे वे केंद्र में रख सकते हैं)
remote-center-request-label = केंद्र व्यू अनुरोध
remote-center = केंद्र
remote-center-cam-title = होस्ट से अपना कैमरा केंद्र में रखने के लिए कहें
remote-center-my-cam = मेरा कैम
remote-center-screen-title = होस्ट से अपनी साझा स्क्रीन केंद्र में रखने के लिए कहें
remote-center-my-screen = मेरी स्क्रीन
remote-center-host-title = केंद्र होस्ट के व्यू को वापस दें
remote-center-host-view = होस्ट व्यू
remote-end-session = सत्र समाप्त करें
remote-leave = छोड़ें
remote-host-view-heading = होस्ट व्यू
remote-host-shared-view-label = होस्ट का साझा व्यू
remote-guest-position-label = गेस्ट स्थिति
remote-guest-label = गेस्ट
remote-put-guest = गेस्ट को { $position } रखें
remote-remove-title = गेस्ट हटाएँ — वे उसी लिंक से फिर से जुड़ सकते हैं
remote-remove = हटाएँ
remote-ban-title = गेस्ट बैन करें — उन्हें ब्लॉक करता है और इनवाइट लिंक अमान्य कर देता है
remote-ban = बैन करें
remote-guest-self-muted = गेस्ट ने खुद को म्यूट किया
remote-unmute-guest = गेस्ट अनम्यूट करें
remote-mute-guest = गेस्ट म्यूट करें
remote-muted-by-host = होस्ट द्वारा म्यूट किया गया
remote-unmute-mic = माइक अनम्यूट करें
remote-mute-mic = माइक म्यूट करें
remote-waiting-for-host = होस्ट की प्रतीक्षा है


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = सोर्स
sources-fallback-video = वीडियो
sources-fallback-error = त्रुटि
sources-kind-unknown = ?
sources-missing-source = (सोर्स नहीं मिला)

# Kind badges
sources-badge-display = डिस्प्ले
sources-badge-window = विंडो
sources-badge-portal = पोर्टल
sources-badge-camera = कैमरा
sources-badge-image = इमेज
sources-badge-media = मीडिया
sources-badge-guest = गेस्ट
sources-badge-color = रंग
sources-badge-text = टेक्स्ट
sources-badge-scene = सीन
sources-badge-slides = स्लाइड्स
sources-badge-chat = चैट
sources-badge-audio-in = ऑडियो इन
sources-badge-audio-out = ऑडियो आउट
sources-badge-app-audio = ऐप ऑडियो

# Add-source menu items
sources-add-display = डिस्प्ले कैप्चर
sources-add-window = विंडो कैप्चर
sources-add-game = गेम कैप्चर (पहले पढ़ें)
sources-add-webcam = वीडियो कैप्चर डिवाइस
sources-add-image = इमेज
sources-add-media = मीडिया (वीडियो/इमेज फ़ाइल)
sources-add-remote-guest = रिमोट गेस्ट (P2P स्पाइक)
sources-add-color = रंग
sources-add-text = टेक्स्ट
sources-add-nested-scene = नेस्टेड सीन
sources-add-slideshow = इमेज स्लाइडशो
sources-add-chat-overlay = लाइव चैट ओवरले
sources-add-audio-input = ऑडियो इनपुट कैप्चर
sources-add-audio-output = ऑडियो आउटपुट कैप्चर
sources-add-app-audio = एप्लिकेशन ऑडियो (Windows)
sources-add-existing = मौजूदा सोर्स…

# Panel header + toolbar buttons
sources-panel-title = सोर्स
sources-group-title = सोर्स ग्रुप करें — दो या अधिक आइटम चुनें, फिर Create group; ग्रुप किए गए आइटम एक साथ मूव होते और दिखते/छिपते हैं
sources-group-aria = सोर्स ग्रुप करें
sources-arrange = व्यवस्थित करें: स्क्रीन + कोने
sources-add-source = एक सोर्स जोड़ें
sources-browser-source-note = Browser Source अपने स्वयं के ऑन-डिमांड कंपोनेंट माइलस्टोन के रूप में आता है (एक ~180 MB Chromium इंजन — कभी बंडल नहीं होता)। आज: Window Capture + एक क्रोमा/कलर की के साथ एक असली ब्राउज़र विंडो कैप्चर करें, या चैट/अलर्ट को एक डॉक के रूप में खोलें (Controls → Docks)।

# Empty state
sources-empty = इस सीन में कोई सोर्स नहीं — “+” से एक Display Capture, Window, Webcam, Image, Color या Text जोड़ें। उन्हें कैनवास पर ड्रैग, स्केल और रोटेट करें; दाईं ओर के बटन स्टैक को फिर से क्रमबद्ध करते हैं।

# Per-row controls
sources-already-in-group = पहले से { $name } में
sources-pick-for-new-group = नए ग्रुप के लिए चुनें
sources-pick-item-for-group = नए ग्रुप के लिए { $name } चुनें
sources-hide = छिपाएँ
sources-show = दिखाएँ
sources-hide-item = { $name } छिपाएँ
sources-show-item = { $name } दिखाएँ
sources-unfocus-title = अनफ़ोकस — लेआउट पुनर्स्थापित करें
sources-focus-title = फ़ोकस — कैनवास भरें (Highlight Speaker)
sources-unfocus-item = { $name } अनफ़ोकस करें
sources-focus-item = { $name } फ़ोकस करें
sources-center-title = केंद्र — इसे साझा केंद्र व्यू बनाएँ (कैम रेल पर चले जाते हैं)
sources-center-item = { $name } केंद्र में रखें
sources-rename-item = { $name } का नाम बदलें
sources-in-group = ग्रुप { $name } में

# Row status + retry
sources-retry-error = पुनः प्रयास — { $message }
sources-retry-item = { $name } पुनः प्रयास करें
sources-status-error = स्थिति: त्रुटि
sources-open-privacy-title = इस अनुमति के लिए macOS गोपनीयता सेटिंग्स खोलें
sources-open-privacy-item = { $name } के लिए गोपनीयता सेटिंग्स खोलें
sources-privacy-settings-button = सेटिंग्स
sources-status-starting = शुरू हो रहा है…
sources-status-live = लाइव
sources-status-aria = स्थिति: { $state }

# Media row pause/resume
sources-media-resume-title = वीडियो फिर से चलाएँ (स्ट्रीम पर लाइव)
sources-media-pause-title = वीडियो रोकें — फ़्रेम रोकें और मौन हो जाएँ, स्ट्रीम पर लाइव
sources-media-resume-item = { $name } फिर से चलाएँ
sources-media-pause-item = { $name } रोकें

# Hover controls
sources-unlock = अनलॉक करें
sources-lock = लॉक करें
sources-unlock-item = { $name } अनलॉक करें
sources-lock-item = { $name } लॉक करें
sources-raise-title = स्टैक में ऊपर उठाएँ
sources-raise-item = { $name } ऊपर उठाएँ
sources-lower-title = स्टैक में नीचे करें
sources-lower-item = { $name } नीचे करें
sources-filters-title = फ़िल्टर और ब्लेंड
sources-filters-item = { $name } के लिए फ़िल्टर
sources-properties-title = प्रॉपर्टीज़
sources-properties-item = { $name } की प्रॉपर्टीज़
sources-remove-title = इस सीन से हटाएँ
sources-remove-item = { $name } हटाएँ

# Grouping footer
sources-create-group = ग्रुप बनाएँ ({ $count })
sources-cancel = रद्द करें

# Groups list
sources-groups-aria = सोर्स ग्रुप
sources-hide-group = ग्रुप छिपाएँ
sources-show-group = ग्रुप दिखाएँ
sources-item-count = · { $count } आइटम
sources-ungroup-title = अनग्रुप — आइटम जहाँ हैं वहीं रहते हैं
sources-ungroup-item = { $name } अनग्रुप करें

# Live Chat Overlay picker
sources-chat-title = एक लाइव चैट ओवरले जोड़ें
sources-chat-youtube-label = YouTube — चैनल, watch, या live_chat URL (कोई key नहीं, कोई साइन-इन नहीं)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  या एक watch?v= URL
sources-chat-twitch-label = Twitch — चैनल नाम (गुमनाम रूप से पढ़ा जाता है, कोई अकाउंट नहीं)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — चैनल slug (सार्वजनिक endpoint, best-effort)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = संदेश एक पारदर्शी बैकग्राउंड पर चलती h:mm:ss AM/PM टाइमस्टैम्प के साथ दिखते हैं (डिफ़ॉल्ट रूप से ऊपर-दाएँ; इसे कहीं भी ड्रैग करें)। एक चैट बाढ़ केवल पुरानी पंक्तियों को हटाती है — यह स्ट्रीम या रिकॉर्डिंग को कभी रोक नहीं सकती। Facebook चैट के लिए आपका अपना Graph token चाहिए और यह अभी लागू नहीं है — यह कभी आवश्यक नहीं है और ऊपर के प्लेटफ़ॉर्म को कभी नहीं रोकता।
sources-chat-add = चैट ओवरले जोड़ें
sources-chat-default-name = लाइव चैट

# Image Slideshow picker
sources-slideshow-title = एक इमेज स्लाइडशो जोड़ें
sources-slideshow-empty = अभी कोई इमेज नहीं — Browse उन्हें क्रम में जोड़ता है।
sources-slideshow-remove-slide = स्लाइड { $number } हटाएँ
sources-slideshow-browse = इमेज ब्राउज़ करें…
sources-slideshow-per-slide-label = प्रति स्लाइड (ms)
sources-slideshow-crossfade-label = क्रॉसफ़ेड (ms, 0 = कट)
sources-slideshow-loop-label = लूप (बंद = आखिरी स्लाइड पर रुकें)
sources-slideshow-shuffle-label = हर चक्र में शफ़ल करें
sources-slideshow-note = क्रॉसफ़ेड समान आकार की इमेज को ब्लेंड करता है; अलग आकार सीमा पर हार्ड-कट होते हैं (कोई मौन री-स्केल नहीं)।
sources-slideshow-add = स्लाइडशो जोड़ें ({ $count })

# Nested Scene picker
sources-nested-title = एक नेस्टेड सीन जोड़ें
sources-nested-empty = नेस्ट करने के लिए कोई अन्य सीन नहीं — पहले एक दूसरा सीन जोड़ें।
sources-nested-scene-name = सीन: { $name }
sources-nested-note = नेस्टेड सीन प्रोग्राम कैनवास आकार पर लाइव रेंडर होता है और अपने खुद के एडिट का अनुसरण करता है; ट्रांसफ़ॉर्म, फ़िल्टर और ब्लेंड इस पर किसी भी सोर्स की तरह लागू होते हैं। जब इसे दिखाने वाला सीन प्रोग्राम पर होता है तो इसके ऑडियो सोर्स मिक्स में शामिल हो जाते हैं।

# Display / Window capture picker
sources-capture-display-title = एक डिस्प्ले कैप्चर जोड़ें
sources-capture-window-title = एक विंडो कैप्चर जोड़ें
sources-capture-looking = सोर्स खोजे जा रहे हैं…
sources-capture-none-displays = यहाँ कैप्चर करने के लिए कुछ नहीं — कोई डिस्प्ले नहीं मिला।
sources-capture-none-windows = यहाँ कैप्चर करने के लिए कुछ नहीं — कोई विंडो नहीं मिली।
sources-capture-portal-note = Wayland पर, सिस्टम डायलॉग स्क्रीन या विंडो चुनता है — ऐप वहाँ वैश्विक रूप से कैप्चर नहीं कर सकते, इसलिए यही ईमानदार (और एकमात्र) रास्ता है।
sources-capture-window-note = प्रीव्यू लाइव अपडेट होते हैं। एक मिनिमाइज़्ड विंडो अपना आखिरी फ़्रेम (या कोई नहीं) तब तक दिखाती है जब तक आप उसे रीस्टोर नहीं करते।
sources-thumb-no-preview = कोई प्रीव्यू नहीं
sources-thumb-loading = लोड हो रहा है…

# Video Capture Device picker
sources-webcam-title = एक वीडियो कैप्चर डिवाइस जोड़ें
sources-webcam-looking = कैमरे खोजे जा रहे हैं…
sources-webcam-none = कोई कैमरा या कैप्चर कार्ड नहीं मिला।
sources-webcam-format-label = फ़ॉर्मैट
sources-webcam-format-auto-loading = ऑटो (फ़ॉर्मैट लोड हो रहे हैं…)
sources-webcam-format-auto = ऑटो (उच्चतम रिज़ॉल्यूशन)
sources-webcam-card-presets-label = कार्ड प्रीसेट:
sources-webcam-preset-title = यह कार्ड जो { $label } मोड विज्ञापित करता है उसे चुनें
sources-webcam-add = कैमरा जोड़ें

# Audio Input / Output capture picker
sources-audio-output-title = एक ऑडियो आउटपुट कैप्चर जोड़ें
sources-audio-input-title = एक ऑडियो इनपुट कैप्चर जोड़ें
sources-audio-default-output = डिफ़ॉल्ट आउटपुट (जो आप सुनते हैं)
sources-audio-default-input = डिफ़ॉल्ट इनपुट
sources-audio-looking = ऑडियो डिवाइस खोजे जा रहे हैं…
sources-audio-none-output = यहाँ कोई डेस्कटॉप-ऑडियो कैप्चर डिवाइस नहीं मिला।
sources-audio-none-input = कोई माइक्रोफ़ोन या लाइन-इन नहीं मिला।
sources-audio-input-note = मिक्सर स्ट्रिप को एक VU मीटर, फ़ेडर, म्यूट, मॉनिटरिंग, फ़िल्टर (डीनॉइज़, गेट, कंप्रेसर…), और ट्रैक असाइनमेंट मिलते हैं। सब कुछ इसी मशीन पर रहता है।

# Application Audio picker
sources-appaudio-title = एप्लिकेशन ऑडियो जोड़ें
sources-appaudio-looking = आवाज़ करने वाले ऐप खोजे जा रहे हैं…
sources-appaudio-none = अभी कोई ऐप आवाज़ नहीं कर रहा — ऐप में प्लेबैक शुरू करें, फिर रिफ़्रेश करें।
sources-appaudio-refresh = ⟳ रिफ़्रेश
sources-appaudio-note = बिल्कुल उसी ऐप का ऑडियो कैप्चर करता है — इसका अपना VU, फ़ेडर, म्यूट, फ़िल्टर और ट्रैक।

# Game Capture picker
sources-game-title = गेम कैप्चर
sources-game-checking = जाँच हो रही है…
sources-game-use-portal = Screen Capture (Portal) का उपयोग करें
sources-game-use-window = इसके बजाय Window Capture का उपयोग करें

# Image picker
sources-image-title = एक इमेज जोड़ें
sources-image-file-label = इमेज फ़ाइल (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = इमेज जोड़ें

# Path field
sources-browse = ब्राउज़ करें…

# Media picker
sources-media-title = मीडिया जोड़ें
sources-media-file-label = मीडिया फ़ाइल (mp4, mkv, webm, mov, .frec, या एक इमेज)
sources-media-loop-label = लूप (अंत में शुरू से फिर से शुरू करें)
sources-media-note = .frec स्वामित्व वाले freally-video कोडेक के माध्यम से चलता है — डाउनलोड करने के लिए कुछ नहीं। वायर फ़ॉर्मैट (mp4/mkv/webm/…) ऑन-डिमांड FFmpeg कंपोनेंट के माध्यम से डिकोड होते हैं; इसका ऑडियो मिक्सर में अपनी स्ट्रिप के रूप में आता है।
sources-media-add = मीडिया जोड़ें

# Invite expiry options
sources-ttl-15min = 15 मिनट
sources-ttl-30min = 30 मिनट
sources-ttl-1hour = 1 घंटा
sources-ttl-1day = 1 दिन

# Remote Guest form
sources-remote-copy-failed = कॉपी नहीं हो सका — लिंक चुनें और मैन्युअल रूप से कॉपी करें
sources-remote-join-failed = जुड़ना विफल: { $error }
sources-remote-title = रिमोट गेस्ट (P2P स्पाइक)
sources-remote-host-heading = होस्ट — एक गेस्ट को आमंत्रित करें
sources-remote-start-hosting = होस्टिंग शुरू करें
sources-remote-expires-label = समाप्त होता है
sources-remote-invite-expiry-aria = इनवाइट समाप्ति
sources-remote-invite-link-aria = इनवाइट लिंक
sources-remote-copied = कॉपी हो गया ✓
sources-remote-copy = कॉपी करें
sources-remote-share-note = यह लिंक साझा करें (Discord / टेक्स्ट / ईमेल)। यह आपका सत्र ले जाता है और निर्धारित समय पर समाप्त होता है। गेस्ट इसे खोलता है और अपने वेबकैम के साथ जुड़ता है।
sources-remote-qr-note = ब्राउज़र से सीधे जुड़ने के लिए एक फ़ोन पर स्कैन करें — कैमरा + माइक, कोई इंस्टॉल नहीं। ऊपर का कॉपी करने योग्य freally:// लिंक उस मशीन पर Freally Capture में खुलता है जिस पर यह इंस्टॉल है।
sources-remote-guest-heading = गेस्ट — एक इनवाइट के साथ जुड़ें
sources-remote-paste-placeholder = इनवाइट लिंक पेस्ट करें
sources-remote-invite-input-aria = इनवाइट लिंक या सत्र id
sources-remote-join = वेबकैम के साथ जुड़ें
sources-remote-session-note = लाइव सत्र नियंत्रण (म्यूट, समाप्त) मुख्य विंडो के शीर्ष पर बार पर रहते हैं — आप इस डायलॉग को बंद कर सकते हैं।
sources-remote-stop-session = सत्र रोकें

# Invite QR
sources-invite-qr-aria = इनवाइट लिंक QR कोड

# Remote device pickers
sources-devices-output-unavailable = आउटपुट रूटिंग उपलब्ध नहीं — डिफ़ॉल्ट डिवाइस पर चल रहा है
sources-devices-mic-test-failed = माइक टेस्ट विफल: { $error }
sources-devices-heading = सत्र ऑडियो डिवाइस
sources-devices-microphone-label = माइक्रोफ़ोन
sources-devices-microphone-aria = सत्र माइक्रोफ़ोन
sources-devices-system-default = सिस्टम डिफ़ॉल्ट
sources-devices-output-label = आउटपुट
sources-devices-output-aria = सत्र ऑडियो आउटपुट
sources-devices-stop-test = टेस्ट रोकें
sources-devices-test = टेस्ट — खुद को सुनें
sources-devices-testing-note = माइक में बोलें — आप चयनित डिवाइस लाइव सुन रहे हैं
sources-devices-idle-note = आपके माइक को आउटपुट पर लूप करता है (हेडफ़ोन फ़ीडबैक से बचाते हैं)

# TURN relay section
sources-turn-save-failed = सहेजा नहीं जा सका: { $error }
sources-turn-summary = नेटवर्क — वैकल्पिक TURN रिले (उन्नत)
sources-turn-note-1 = सत्र सीधे (P2P) कनेक्ट होते हैं — मुफ़्त, कोई रिले नहीं चाहिए। यदि दोनों पक्ष सख्त NAT के पीछे हैं तो सीधा रास्ता विफल हो सकता है; तब आपके द्वारा चलाया गया एक TURN रिले मीडिया ले जाता है। इसे छोड़ना ठीक है — अधिकांश कनेक्शन केवल-सीधे काम करते हैं।
sources-turn-note-2 = मुफ़्त विकल्प: Oracle Cloud "Always Free" बिना लागत के coturn चलाता है (नोट: Oracle साइनअप पर एक क्रेडिट कार्ड माँगता है, लेकिन Always-Free शेप मुफ़्त रहता है)। चरण: 1) मुफ़्त VM बनाएँ, 2) coturn इंस्टॉल करें, 3) UDP 3478 खोलें, 4) एक user/password सेट करें, 5) यहाँ turn:your-vm-ip:3478 + क्रेडेंशियल दर्ज करें। आपका क्रेडेंशियल आपकी स्थानीय सेटिंग्स फ़ाइल में रहता है और कभी लॉग नहीं होता।
sources-turn-url-label = TURN URL
sources-turn-url-placeholder = turn:host:3478 (खाली = केवल सीधा)
sources-turn-url-aria = TURN URL
sources-turn-username-label = Username
sources-turn-username-aria = TURN username
sources-turn-credential-label = क्रेडेंशियल
sources-turn-credential-aria = TURN क्रेडेंशियल
sources-turn-note-3 = तीनों फ़ील्ड सेट होने पर रिले सक्रिय होता है (एक TURN सर्वर को क्रेडेंशियल की आवश्यकता होती है) और यह अगले सत्र पर लागू होता है जिसे आप शुरू या जॉइन करते हैं। इसे अपनी दो मशीनों के बीच एक रिले-only टेस्ट कॉल से सत्यापित करें।
sources-turn-settings-unavailable = सेटिंग्स उपलब्ध नहीं (ब्राउज़र मोड)

# Color picker
sources-color-title = एक रंग जोड़ें
sources-color-label = रंग
sources-color-width-label = चौड़ाई
sources-color-height-label = ऊँचाई
sources-color-add = रंग जोड़ें

# Text picker
sources-text-title = टेक्स्ट जोड़ें
sources-text-label = टेक्स्ट
sources-text-default = टेक्स्ट
sources-text-color-label = रंग
sources-text-color-aria = टेक्स्ट रंग
sources-text-size-label = आकार (px)
sources-text-note = फ़ॉन्ट फ़ैमिली, संरेखण, रैपिंग और RTL सोर्स की प्रॉपर्टीज़ में रहते हैं। बंडल किया गया Noto Sans (अरबी/हिब्रू सहित) डिफ़ॉल्ट है — हर मशीन पर एक जैसा।
sources-text-add = टेक्स्ट जोड़ें

# Existing source picker
sources-existing-title = एक मौजूदा सोर्स जोड़ें
sources-existing-empty = अभी कोई सोर्स मौजूद नहीं — पहले किसी सीन में एक जोड़ें। मौजूदा सोर्स साझा होते हैं: किसी एक का नाम बदलने या पुनः कॉन्फ़िगर करने से हर उस सीन पर असर पड़ता है जो इसे दिखाता है।

# Screen + corners layout
sources-slot-off = बंद
sources-slot-center = केंद्र (स्क्रीन)
sources-slot-top-left = ऊपर-बाएँ
sources-slot-top-right = ऊपर-दाएँ
sources-slot-bottom-left = नीचे-बाएँ
sources-slot-bottom-right = नीचे-दाएँ
sources-layout-title = व्यवस्थित करें: स्क्रीन + कोने
sources-layout-empty = पहले इस सीन में एक स्क्रीन कैप्चर और एक या अधिक कैमरे जोड़ें, फिर उन्हें यहाँ व्यवस्थित करें।
sources-layout-note = बीच में एक स्क्रीन और कोनों में चार तक कैमरे रखें — आपका explainer / podcast लेआउट। हर कोना एक वेबकैम, एक कैप्चर की गई कॉल विंडो, या एक मीडिया क्लिप रखता है। आप बाद में इनमें से किसी को भी कैनवास पर ड्रैग कर सकते हैं।
sources-layout-slot-aria = { $name } के लिए स्लॉट
sources-layout-apply = लेआउट लागू करें


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = नियंत्रण
controls-start-stop-title-stop = रिकॉर्डिंग रोकें और अंतिम रूप दें
controls-start-stop-title-start = Settings → Output कॉन्फ़िगरेशन के साथ प्रोग्राम फ़ीड रिकॉर्ड करें
controls-finalizing = ◌ अंतिम रूप दिया जा रहा है…
controls-stop-recording = ■ रिकॉर्डिंग रोकें
controls-start-recording = ● रिकॉर्डिंग शुरू करें
controls-marker-title = इस पल पर एक चैप्टर मार्कर डालें — यह रिकॉर्डिंग में आता है (mkv चैप्टर, या एक sidecar फ़ाइल)। प्लेटफ़ॉर्म-साइड स्ट्रीम मार्कर के लिए प्लेटफ़ॉर्म अकाउंट चाहिए, जो यह ऐप कभी नहीं माँगता।
controls-marker = ◈ मार्कर
controls-pause-title-resume = फिर से शुरू करें — फ़ाइल एक ही सतत टाइमलाइन के रूप में जारी रहती है
controls-pause-title-pause = रोकें — कोई फ़्रेम नहीं लिखा जाता; फिर से शुरू करने पर वही प्ले करने योग्य फ़ाइल जारी रहती है
controls-resume-recording = ▶ रिकॉर्डिंग फिर से शुरू करें
controls-pause-recording = ⏸ रिकॉर्डिंग रोकें
controls-reactions-label = रिएक्शन (प्रोग्राम में बेक किए गए)
controls-reactions-title = प्रोग्राम के ऊपर एक रिएक्शन तैराएँ — रिकॉर्ड और स्ट्रीम दोनों, ताकि रीप्ले सही पल दिखाए। चैट में दर्शक भी इन्हें ट्रिगर करते हैं (उनका रिएक्शन इमोजी अपने आप तैरता है); एक बाढ़ केवल स्क्रीन पर मौजूद संख्या को सीमित करती है।
controls-react = रिएक्ट { $emoji }
controls-virtual-camera-title = वर्चुअल कैमरे को प्रति OS अपना साइन किया हुआ ड्राइवर कंपोनेंट चाहिए (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO extension / Linux v4l2loopback) — यह अपने माइलस्टोन के रूप में आता है। फ़ीड मॉडल इसके लिए तैयार है: प्रोग्राम, वर्टिकल कैनवास, या एक अकेला सोर्स, Windows/Linux पर एक जोड़े वर्चुअल माइक के साथ (macOS में कोई वर्चुअल-माइक API नहीं है — ईमानदारी से कहा)।
controls-virtual-camera = ⌁ वर्चुअल कैमरा शुरू करें
controls-files-title = पूर्ण की गई रिकॉर्डिंग + remux-to-mp4 क्रिया
controls-files = ▤ फ़ाइलें…
controls-output-title = रिकॉर्डिंग फ़ॉर्मैट, एन्कोडर, फ़ोल्डर, ट्रैक और स्प्लिटिंग
controls-output = ⚙ आउटपुट…
controls-stream-title = लाइव जाने का लक्ष्य: सेवा, स्ट्रीम की, एन्कोडर, बिटरेट
controls-stream = ⦿ स्ट्रीम…
controls-codecs-title = ऑन-डिमांड ffmpeg वायर-कोडेक कंपोनेंट (स्पष्ट रूप से लेबल किया गया, कभी बंडल नहीं)
controls-codecs = ⬡ कोडेक…
controls-replay-title = रीप्ले बफ़र लंबाई + गुणवत्ता प्रीसेट
controls-replay = ⟲ रीप्ले…
controls-keys-title = वैश्विक हॉटकी: रिकॉर्ड, लाइव जाएँ, ट्रांज़िशन, रीप्ले सहेजें
controls-keys = ⌨ कीज़…
controls-scripts-title = सैंडबॉक्स्ड Lua स्क्रिप्ट: go-live/scene/recording घटनाओं पर प्रतिक्रिया दें, स्टूडियो चलाएँ
controls-scripts = ⚡ स्क्रिप्ट…
controls-docks-title = ब्राउज़र डॉक: स्टूडियो के बगल में एक विंडो के रूप में एक चैट पॉपआउट, अलर्ट पेज, या Companion बटन खोलें
controls-docks = ⧉ डॉक…
controls-remote-title = Stream Deck / Companion कंट्रोलर के लिए WebSocket रिमोट API (डिफ़ॉल्ट रूप से बंद)
controls-remote = ⌁ रिमोट…
controls-profiles-title = प्रोफ़ाइल (सेटिंग्स) + सीन कलेक्शन — स्विच करने योग्य स्नैपशॉट
controls-profiles = ▣ प्रोफ़ाइल…
controls-bug-title = एक बग रिपोर्ट करें — गुमनाम, ऑप्ट-इन (कुछ भी स्वचालित रूप से नहीं भेजा जाता)
controls-bug = 🐞 एक बग रिपोर्ट करें…
controls-updates-title = अपडेट जाँचें — साइन किया गया, सत्यापित, बिना क्लिक के कुछ भी डाउनलोड नहीं होता
controls-updates = ⭳ अपडेट जाँचें…
controls-saved = सहेजा गया: { $path }

# --- MixerDock.tsx ---
mixer-title = ऑडियो मिक्सर
mixer-monitor-error = मॉनिटर: { $error }
mixer-switch-to-horizontal = क्षैतिज स्ट्रिप पर स्विच करें
mixer-switch-to-vertical = लंबवत स्ट्रिप पर स्विच करें
mixer-layout-aria-vertical = मिक्सर लेआउट: लंबवत — क्षैतिज पर स्विच करें
mixer-layout-aria-horizontal = मिक्सर लेआउट: क्षैतिज — लंबवत पर स्विच करें
mixer-empty = इस सीन में कोई ऑडियो सोर्स नहीं — Sources में “+” से एक Audio Input Capture (माइक) या Audio Output Capture (डेस्कटॉप ऑडियो) जोड़ें। स्ट्रिप को एक VU मीटर, फ़ेडर, म्यूट, मॉनिटरिंग, फ़िल्टर और ट्रैक असाइनमेंट मिलते हैं।
mixer-advanced-title = ऑडियो — { $name }
mixer-loudness-label = प्रोग्राम लाउडनेस (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = तात्कालिक लाउडनेस (400 ms)
mixer-short-term-title = शॉर्ट-टर्म लाउडनेस (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = मॉनिटर
mixer-monitor-device-aria = मॉनिटर आउटपुट डिवाइस
mixer-default-output = डिफ़ॉल्ट आउटपुट

# --- StatsDock.tsx ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = मेमोरी
stats-dropped = ड्रॉप हुए
stats-render = रेंडर
stats-gpu = GPU
stats-gpu-compositing = कम्पोज़िटिंग
stats-gpu-idle = निष्क्रिय
stats-vertical-fps = 9:16 FPS
stats-targets-label = स्ट्रीम लक्ष्य
stats-shared-encode = · साझा एन्कोड
stats-starting = कम्पोज़िटर शुरू हो रहा है…

# --- ScenesRail.tsx ---
scenes-title = सीन
scenes-new-scene-name = सीन
scenes-add = एक सीन जोड़ें
scenes-empty = स्टूडियो कोर से कनेक्ट हो रहा है…
scenes-rename = { $name } का नाम बदलें
scenes-on-program = प्रोग्राम पर
scenes-preview = { $name } प्रीव्यू करें
scenes-switch-to = { $name } पर स्विच करें
scenes-move-up = ऊपर ले जाएँ
scenes-move-up-aria = { $name } ऊपर ले जाएँ
scenes-move-down = नीचे ले जाएँ
scenes-move-down-aria = { $name } नीचे ले जाएँ
scenes-last-stays = आखिरी सीन बना रहता है
scenes-remove = इस सीन को हटाएँ
scenes-remove-aria = { $name } हटाएँ


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = स्तर
channelstrip-monitor-off = मॉनिटर बंद
channelstrip-monitor-only = केवल मॉनिटर (मिक्स में नहीं)
channelstrip-monitor-and-output = मॉनिटर और आउटपुट
channelstrip-status-error = त्रुटि
channelstrip-status-live = लाइव
channelstrip-status-waiting-audio = ऑडियो की प्रतीक्षा है
channelstrip-status = स्थिति: { $state }
channelstrip-status-waiting = प्रतीक्षा में
channelstrip-mute = म्यूट
channelstrip-unmute = अनम्यूट
channelstrip-mute-source = { $name } म्यूट करें
channelstrip-unmute-source = { $name } अनम्यूट करें
channelstrip-scene-mix-on = प्रति-सीन मिक्स चालू — यह स्ट्रिप इस सीन के लिए वैश्विक मिक्स को ओवरराइड करती है (वैश्विक मिक्स का फिर से अनुसरण करने के लिए क्लिक करें)
channelstrip-scene-mix-off = प्रति-सीन मिक्स — इस स्ट्रिप को वर्तमान सीन के लिए अपना खुद का फ़ेडर/म्यूट दें
channelstrip-scene-mix-label = { $name } के लिए प्रति-सीन मिक्स
channelstrip-monitor-cycle = { $mode } — साइकल करने के लिए क्लिक करें
channelstrip-monitor-mode = { $name } का मॉनिटर मोड: { $mode }
channelstrip-audio-filters-title = ऑडियो फ़िल्टर (डीनॉइज़, गेट, कंप्रेसर…)
channelstrip-audio-filters-label = { $name } के लिए ऑडियो फ़िल्टर
channelstrip-advanced-title = सिंक ऑफ़सेट और push-to-talk हॉटकी
channelstrip-advanced-label = { $name } के लिए उन्नत ऑडियो सेटिंग्स
channelstrip-track-assignment = ट्रैक असाइनमेंट
channelstrip-track = ट्रैक { $n }
channelstrip-track-assigned = ट्रैक { $n } (असाइन किया गया)
channelstrip-track-label = { $name } के लिए ट्रैक { $n }
channelstrip-device-error = डिवाइस त्रुटि
channelstrip-audio-device-error = ऑडियो डिवाइस त्रुटि
channelstrip-volume-label = डेसिबल में { $name } का वॉल्यूम
channelstrip-ptt-hold = Push-to-talk: { $key } दबाए रखें
channelstrip-sync-offset = सिंक ऑफ़सेट (ms, 0–{ $max } — इस ऑडियो को विलंबित करता है)
channelstrip-ptt-hotkey = Push-to-talk हॉटकी (दबाए रखने तक मौन)
channelstrip-ptt-placeholder = जैसे Ctrl+Shift+T या F13
channelstrip-ptt-aria = Push-to-talk हॉटकी
channelstrip-ptm-hotkey = Push-to-mute हॉटकी (दबाए रखने पर मौन)
channelstrip-ptm-placeholder = जैसे Ctrl+Shift+M
channelstrip-ptm-aria = Push-to-mute हॉटकी
channelstrip-hotkeys-note = हॉटकी तब भी काम करती हैं जब अन्य ऐप फ़ोकस में हों। Linux/Wayland पर, वैश्विक हॉटकी उपलब्ध नहीं हो सकतीं — यह एक कम्पोज़िटर सीमा है, ईमानदारी से कहा।
channelstrip-apply = लागू करें

# --- LiveButton.tsx ---
livebutton-failure-ended = स्ट्रीम समाप्त हो गई
livebutton-title-live = स्ट्रीम समाप्त करें — हर लक्ष्य (एक चल रही रिकॉर्डिंग जारी रहती है)
livebutton-title-offline = हर सक्षम Settings → Stream लक्ष्य पर लाइव जाएँ
livebutton-end-stream = ■ स्ट्रीम समाप्त करें
livebutton-aria-reconnecting = फिर से कनेक्ट हो रहा है
livebutton-aria-live = लाइव
livebutton-badge-retry = पुनः प्रयास { $n }
livebutton-badge-live = लाइव
livebutton-go-live = ⦿ लाइव जाएँ

# --- RecDot.tsx ---
recdot-paused-aria = रिकॉर्डिंग रुकी हुई
recdot-recording-aria = रिकॉर्डिंग
recdot-tracks-one = { $count } ऑडियो ट्रैक रिकॉर्ड हो रहा है
recdot-tracks-other = { $count } ऑडियो ट्रैक रिकॉर्ड हो रहे हैं
recdot-paused = रुका हुआ

# --- ReplayControls.tsx ---
replaycontrols-saved = रीप्ले सहेजा गया — { $name }
replaycontrols-failure-stopped = बफ़र रुक गया
replaycontrols-title-disarm = रीप्ले बफ़र निष्क्रिय करें (बिना सहेजा इतिहास हटा देता है)
replaycontrols-title-arm = रोलिंग रीप्ले बफ़र सक्रिय करें — आखिरी N सेकंड सहेजने के लिए तैयार रखता है (इसका अपना हल्का एन्कोड; स्ट्रीम और रिकॉर्डिंग अछूती रहती हैं)
replaycontrols-replay-seconds = ⟲ रीप्ले { $seconds }s
replaycontrols-arm = ⟲ रीप्ले बफ़र सक्रिय करें
replaycontrols-save-title = आखिरी N सेकंड रिकॉर्डिंग फ़ोल्डर में सहेजें (Save-Replay हॉटकी पर भी)
replaycontrols-save = ⤓ सहेजें

# --- PropertiesDialog.tsx ---
properties-title = प्रॉपर्टीज़ — { $name }
properties-name = नाम
properties-cancel = रद्द करें
properties-apply = लागू करें
properties-youtube = YouTube — चैनल / watch / live_chat URL (कभी कोई key नहीं, कोई साइन-इन नहीं)
properties-twitch = Twitch — चैनल नाम (गुमनाम)
properties-kick = Kick — चैनल slug (सार्वजनिक endpoint)
properties-width-px = चौड़ाई (px)
properties-lines = पंक्तियाँ
properties-font-px = फ़ॉन्ट (px)
properties-images = इमेज फ़ाइलें (प्रति पंक्ति एक path, क्रम में दिखाई गई)
properties-per-slide = प्रति स्लाइड (ms)
properties-crossfade = क्रॉसफ़ेड (ms, 0 = कट)
properties-loop-slideshow = लूप (बंद = आखिरी स्लाइड पर रुकें)
properties-shuffle = हर चक्र में शफ़ल करें
properties-nested-scene = यह सोर्स जो सीन बनाता है (एक सीन जिसमें यह पहले से है, अस्वीकृत है)
properties-portal-note = Wayland ScreenCast portal हर बार इस सोर्स के शुरू होने पर सिस्टम डायलॉग में स्क्रीन या विंडो चुनता है — डिज़ाइन के अनुसार यहाँ कॉन्फ़िगर करने के लिए कुछ नहीं है।
properties-appaudio-capturing = { $exe } से ऑडियो कैप्चर हो रहा है
properties-appaudio-exe-fallback = एक एप्लिकेशन
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = किसी अलग ऐप को लक्ष्य बनाने के लिए सोर्स फिर से जोड़ें (ऐप के रीस्टार्ट होने पर एक process id बदल जाता है)।
properties-image-file = इमेज फ़ाइल
properties-media-file = मीडिया फ़ाइल (mp4, mkv, webm, mov, .frec, या एक इमेज)
properties-media-loop = लूप (अंत में शुरू से फिर से शुरू करें)
properties-media-hwdecode = हार्डवेयर डिकोड (अपने आप सॉफ़्टवेयर पर वापस आ जाता है)
properties-media-note = .frec स्वामित्व वाले freally-video कोडेक के माध्यम से चलता है — डाउनलोड करने के लिए कुछ नहीं। अन्य वीडियो फ़ॉर्मैट ऑन-डिमांड FFmpeg कंपोनेंट के माध्यम से डिकोड होते हैं। फ़ाइल के ऑडियो को अपनी मिक्सर स्ट्रिप मिलती है; स्ट्रिप का सिंक ऑफ़सेट A/V संरेखण को ठीक करता है। बिना ऑडियो वाली एक क्लिप अपनी स्ट्रिप को मौन छोड़ देती है।
properties-color = रंग
properties-width = चौड़ाई
properties-height = ऊँचाई
properties-text = टेक्स्ट
properties-font-family = फ़ॉन्ट फ़ैमिली (सिस्टम; खाली = डिफ़ॉल्ट)
properties-size-px = आकार (px)
properties-text-color = टेक्स्ट रंग
properties-align = संरेखण
properties-align-left = बाएँ
properties-align-center = केंद्र
properties-align-right = दाएँ
properties-line-spacing = पंक्ति अंतराल
properties-wrap-width = रैप चौड़ाई (px; 0 = बंद)
properties-force-rtl = दाएँ-से-बाएँ मजबूर करें
properties-text-note = रेंडरिंग असली शेपिंग (अरबी जोड़, ligatures) और bidi पंक्ति क्रम का उपयोग करती है। बंडल किया गया Noto Sans फ़ैमिली (अरबी/हिब्रू सहित) डिफ़ॉल्ट है; सिस्टम फ़ैमिली भी काम करती हैं। CJK अभी सिस्टम फ़ॉन्ट का उपयोग करता है।
properties-repick-capturing = कैप्चर हो रहा है: { $label }
properties-repick-looking = सोर्स खोजे जा रहे हैं…
properties-repick-none-displays = फिर से चुनने के लिए कोई डिस्प्ले नहीं मिला।
properties-repick-none-windows = फिर से चुनने के लिए कोई विंडो नहीं मिली।
properties-repick-again = फिर से चुनें:
properties-device = डिवाइस
properties-video-current-device = (वर्तमान डिवाइस)
properties-format = फ़ॉर्मैट
properties-format-auto-loading = ऑटो (फ़ॉर्मैट लोड हो रहे हैं…)
properties-format-auto = ऑटो (उच्चतम रिज़ॉल्यूशन)
properties-audio-capture-of = इसका ऑडियो कैप्चर करें
properties-audio-default-output = डिफ़ॉल्ट आउटपुट (जो आप सुनते हैं)
properties-audio-default-input = डिफ़ॉल्ट इनपुट
properties-audio-default-suffix = (डिफ़ॉल्ट)
properties-audio-current-device = (वर्तमान डिवाइस: { $id })

# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = गेन
audiofilters-name-noise-gate = नॉइज़ गेट
audiofilters-name-compressor = कंप्रेसर
audiofilters-name-limiter = लिमिटर
audiofilters-name-eq = 3-बैंड EQ
audiofilters-name-denoise = डीनॉइज़
audiofilters-name-ducking = डकिंग
audiofilters-title = ऑडियो फ़िल्टर — { $name }
audiofilters-chain-header = फ़िल्टर चेन (ऊपर वाला पहले चलता है, फ़ेडर से पहले)
audiofilters-add = + फ़िल्टर जोड़ें
audiofilters-add-menu = एक ऑडियो फ़िल्टर जोड़ें
audiofilters-empty = अभी कोई फ़िल्टर नहीं — एक माइक को डीनॉइज़ करें (क्लासिक DSP, कोई ML नहीं), कमरे को गेट करें, कंप्रेसर से पीक को काबू करें, या संगीत को अपनी आवाज़ के नीचे डक करें।
audiofilters-enable = { $name } सक्षम करें
audiofilters-run-earlier = पहले चलाएँ
audiofilters-move-up = { $name } ऊपर ले जाएँ
audiofilters-run-later = बाद में चलाएँ
audiofilters-move-down = { $name } नीचे ले जाएँ
audiofilters-remove-title = फ़िल्टर हटाएँ
audiofilters-remove = { $name } हटाएँ
audiofilters-gain-db = गेन (dB)
audiofilters-open-db = इस पर खुलें (dB)
audiofilters-close-db = इस पर बंद हों (dB)
audiofilters-attack-ms = अटैक (ms)
audiofilters-hold-ms = होल्ड (ms)
audiofilters-release-ms = रिलीज़ (ms)
audiofilters-ratio = अनुपात (:1)
audiofilters-threshold-db = थ्रेशोल्ड (dB)
audiofilters-output-gain-db = आउटपुट गेन (dB)
audiofilters-ceiling-db = सीलिंग (dB)
audiofilters-low-db = लो (dB)
audiofilters-mid-db = मिड (dB)
audiofilters-high-db = हाई (dB)
audiofilters-strength = ताकत
audiofilters-denoise-note = स्वामित्व वाला क्लासिक-DSP स्पेक्ट्रल दमन — स्थिर शोर (पंखे, हिस) घटता है जबकि भाषण गुज़रता है। चार्टर के अनुसार, कोई ML नहीं, कोई मॉडल नहीं।
audiofilters-duck-under = इसके नीचे डक करें
audiofilters-ducking-trigger = डकिंग ट्रिगर सोर्स
audiofilters-pick-trigger = (एक ट्रिगर चुनें — जैसे आपका माइक)
audiofilters-trigger-at-db = इस पर ट्रिगर करें (dB)
audiofilters-duck-by-db = इतना डक करें (dB)

# --- FiltersDialog.tsx ---
filters-name-chroma-key = क्रोमा की
filters-name-color-key = कलर की
filters-name-luma-key = लुमा की
filters-name-render-delay = रेंडर डिले
filters-name-color-correction = कलर करेक्शन
filters-name-lut = LUT लागू करें
filters-name-blur = ब्लर
filters-name-mask = इमेज मास्क
filters-name-sharpen = शार्पन
filters-name-scroll = स्क्रॉल
filters-name-crop = क्रॉप
filters-title = फ़िल्टर — { $name }
filters-blend-mode = ब्लेंड मोड
filters-chain-header = फ़िल्टर चेन (ऊपर वाला पहले चलता है)
filters-add = + फ़िल्टर जोड़ें
filters-add-menu = एक फ़िल्टर जोड़ें
filters-empty = अभी कोई फ़िल्टर नहीं — एक वेबकैम को क्रोमा की करें, एक कैप्चर को कलर-करेक्ट करें, या एक टिकर स्क्रॉल करें।
filters-enable = { $name } सक्षम करें
filters-run-earlier = पहले चलाएँ
filters-move-up = { $name } ऊपर ले जाएँ
filters-run-later = बाद में चलाएँ
filters-move-down = { $name } नीचे ले जाएँ
filters-remove-title = फ़िल्टर हटाएँ
filters-remove = { $name } हटाएँ
filters-key-color-rgb = की रंग (कोई भी रंग, RGB दूरी)
filters-similarity = समानता
filters-smoothness = चिकनाई
filters-luma-min = लुमा न्यूनतम (गहरे हिस्से हटाता है)
filters-luma-max = लुमा अधिकतम (चमकीले हिस्से हटाता है)
filters-delay = डिले (ms — केवल वीडियो, जैसे ऑडियो के साथ सिंक करने के लिए; अधिकतम 500)
filters-key-color = की रंग
filters-spill = स्पिल
filters-gamma = गामा
filters-brightness = चमक
filters-contrast = कंट्रास्ट
filters-saturation = संतृप्ति
filters-hue-shift = ह्यू शिफ़्ट
filters-opacity = अपारदर्शिता
filters-cube-file = .cube फ़ाइल
filters-amount = मात्रा
filters-radius = त्रिज्या
filters-mask-image = मास्क इमेज
filters-mask-mode = मोड
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = उलटें
filters-speed-x = गति X (px/s)
filters-speed-y = गति Y (px/s)
filters-crop-left = बाएँ
filters-crop-top = ऊपर
filters-crop-right = दाएँ
filters-crop-bottom = नीचे
filters-crop-aria = क्रॉप { $side }

# --- PickerShell.tsx ---
pickershell-refresh-aria = रिफ़्रेश
pickershell-refresh-title = सूची रिफ़्रेश करें
pickershell-close = बंद करें


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = एक बग रिपोर्ट करें
bugreport-intro = रिपोर्ट गुमनाम और ऑप्ट-इन होती हैं — कुछ भी स्वचालित रूप से नहीं भेजा जाता। आप नीचे दिए गए सटीक टेक्स्ट की समीक्षा करेंगे, फिर इसे एक पहले से भरे GitHub issue या अपने ईमेल ऐप के माध्यम से सबमिट करेंगे। कोई व्यक्तिगत डेटा नहीं (आपका home path और username छिपा दिया जाता है); कोई अकाउंट नहीं, कोई सर्वर नहीं।
bugreport-crash-notice = Freally Capture पिछली बार अप्रत्याशित रूप से बंद हो गया — गुमनाम क्रैश विवरण नीचे शामिल हैं। इन्हें रिपोर्ट करने से इसे जल्दी ठीक करने में मदद मिलती है।
bugreport-description-label = जब यह हुआ तब आप क्या कर रहे थे? (वैकल्पिक)
bugreport-description-placeholder = जैसे दूसरा वेबकैम जोड़ने पर प्रीव्यू फ़्रीज़ हो गया
bugreport-include-crash = पिछले रन के गुमनाम क्रैश विवरण शामिल करें
bugreport-preview-label = बिल्कुल क्या भेजा जाएगा
bugreport-open-github = GitHub issue खोलें
bugreport-gmail-title = आपके ब्राउज़र में Gmail की compose विंडो खोलता है, पहले से भरी हुई। साइन आउट हैं? Google पहले अपनी लॉगिन स्क्रीन दिखाता है।
bugreport-compose-gmail = Gmail में लिखें
bugreport-email-title = इस PC के डिफ़ॉल्ट मेल ऐप (Outlook, Thunderbird, Mail…) में एक ड्राफ़्ट खोलता है
bugreport-send-email = ईमेल भेजें
bugreport-copied = कॉपी हो गया ✓
bugreport-copy-report = रिपोर्ट कॉपी करें
bugreport-dismiss-crash = क्रैश खारिज करें
bugreport-copy-failed = कॉपी नहीं हो सका — टेक्स्ट चुनें और मैन्युअल रूप से कॉपी करें
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = क्या हुआ
bugreport-preview-no-description = (कोई विवरण नहीं दिया गया)
bugreport-preview-diagnostics = गुमनाम डायग्नोस्टिक्स (कोई व्यक्तिगत डेटा नहीं)
bugreport-preview-from = प्रेषक: Freally Capture
bugreport-preview-crash-excerpt = --- क्रैश अंश ---

# --- Updates.tsx ---
updates-title = सॉफ़्टवेयर अपडेट
updates-checking = अपडेट जाँचे जा रहे हैं…
updates-uptodate = आप नवीनतम संस्करण पर हैं।
updates-check-again = फिर से जाँचें
updates-available = संस्करण { $version } उपलब्ध है
updates-current-version = (आपके पास { $current } है)
updates-release-notes-label = संस्करण { $version } — रिलीज़ नोट्स
updates-confirm = क्या आप अभी अपडेट करना चाहते हैं? लागू होने से पहले डाउनलोड को बंडल किए गए साइनिंग की के विरुद्ध सत्यापित किया जाता है। Freally Capture बंद होता है, इंस्टॉलर चलता है, और नया संस्करण अपने आप फिर से खुलता है।
updates-yes-update-now = हाँ, अभी अपडेट करें
updates-no-not-now = नहीं, अभी नहीं
updates-downloading = { $version } डाउनलोड हो रहा है…
updates-starting = शुरू हो रहा है…
updates-installed = अपडेट इंस्टॉल हो गया।
updates-restart-now = अभी रीस्टार्ट करें
updates-restart-later = बाद में रीस्टार्ट करें
updates-try-again = फिर से प्रयास करें

# --- Models.tsx ---
models-title = कंपोनेंट
models-ffmpeg-heading = FFmpeg — वायर कोडेक
models-badge-third-party = तृतीय-पक्ष · बंडल नहीं
models-ffmpeg-desc = Freally Capture का अपना इंजन बिना किसी अतिरिक्त चीज़ के लॉसलेस freally-video (.frec) रिकॉर्ड करता है। प्लेटफ़ॉर्म और प्लेयर जिन वायर फ़ॉर्मैट की अपेक्षा करते हैं — mp4/mkv/mov/webm में H.264/AAC (और HEVC/AV1) — उन्हें रिकॉर्ड करने के लिए FFmpeg का उपयोग होता है, एक अलग टूल जिसके साथ यह ऐप कभी नहीं आता: वे कोडेक पेटेंट-बाधित हैं, इसलिए यह वैकल्पिक और स्पष्ट रूप से लेबल किया हुआ रहता है। यह नीचे दिए गए पिन किए गए बिल्ड से ऑन-डिमांड डाउनलोड होता है, पहले उपयोग से पहले SHA-256-सत्यापित होता है, प्रति-उपयोगकर्ता कैश होता है, और एक अलग प्रोसेस के रूप में चलाया जाता है। इसका लाइसेंस (LGPL/GPL) इसका अपना है — THIRD-PARTY-NOTICES देखें।
models-checking = जाँच हो रही है…
models-ffmpeg-not-installed = इंस्टॉल नहीं है। उपलब्ध: { $source } से FFmpeg { $version } ({ $size } डाउनलोड)।
models-ffmpeg-none-pinned = इस प्लेटफ़ॉर्म के लिए अभी कोई FFmpeg बिल्ड पिन नहीं है — वायर-कोडेक रिकॉर्डिंग यहाँ उपलब्ध नहीं है। लॉसलेस freally-video रिकॉर्डिंग अप्रभावित है।
models-ffmpeg-download-verify = डाउनलोड और सत्यापित करें ({ $size })
models-downloading = डाउनलोड हो रहा है…
models-download-of = में से
models-cancel = रद्द करें
models-ffmpeg-verifying = डाउनलोड को पिन किए गए SHA-256 के विरुद्ध सत्यापित किया जा रहा है…
models-ffmpeg-extracting = अनपैक हो रहा है…
models-ffmpeg-ready = इंस्टॉल और सत्यापित — { $version }
models-remove = हटाएँ
models-ffmpeg-retry = डाउनलोड पुनः प्रयास करें
models-network-note = डाउनलोड इस पैनल पर एकमात्र नेटवर्क क्रिया है और कभी अपने आप शुरू नहीं होती। एक विफल checksum इंस्टॉल को रद्द कर देता है — ऐप ऐसे bytes चलाने से मना करता है जिनकी वह पुष्टि नहीं कर सकता।
models-cef-heading = Browser Source रनटाइम — Chromium (CEF)
models-cef-desc = Browser sources वेब पेज (अलर्ट, विजेट, ओवरले) को Chromium Embedded Framework के माध्यम से रेंडर करते हैं — एक ~100 MB रनटाइम जिसके साथ यह ऐप कभी नहीं आता। यह आधिकारिक CEF बिल्ड इंडेक्स से ऑन-डिमांड डाउनलोड होता है, कुछ भी अनपैक होने से पहले उस इंडेक्स के SHA-1 के विरुद्ध सत्यापित होता है, और प्रति-उपयोगकर्ता कैश होता है। इसके माध्यम से रेंडर होने वाला browser source अपने माइलस्टोन के साथ आता है; यह उसके लिए आवश्यक रनटाइम इंस्टॉल करता है।
models-cef-download-install = डाउनलोड और इंस्टॉल करें
models-cef-unsupported = CEF इस प्लेटफ़ॉर्म के लिए कोई बिल्ड प्रकाशित नहीं करता — browser sources यहाँ उपलब्ध नहीं हैं।
models-cef-resolving = नवीनतम स्थिर बिल्ड का पता लगाया जा रहा है…
models-cef-verifying = डाउनलोड को इंडेक्स SHA-1 के विरुद्ध सत्यापित किया जा रहा है…
models-cef-extracting = रनटाइम अनपैक हो रहा है…
models-cef-ready = इंस्टॉल हो गया — CEF { $version }।
models-cef-retry = पुनः प्रयास करें
models-integrations-heading = वैकल्पिक इंटीग्रेशन
models-badge-never-bundled = कभी बंडल नहीं
models-ndi-detected = पता चला
models-ndi-not-installed = इंस्टॉल नहीं है
models-vst-available = उपलब्ध
models-vst-not-available = उपलब्ध नहीं

# --- Recordings.tsx ---
recordings-title = रिकॉर्डिंग
recordings-loading = फ़ोल्डर पढ़ा जा रहा है…
recordings-empty = अभी कोई रिकॉर्डिंग नहीं — Start Recording, Output में सेट फ़ोल्डर में लिखता है।
recordings-frec-label = स्वामित्व वाला लॉसलेस (freally-video)
recordings-remux-title = mp4 के रूप में फिर से रैप करें — stream copy, कोई री-एन्कोड नहीं, कोई गुणवत्ता बदलाव नहीं (FFmpeg कंपोनेंट चाहिए)
recordings-remuxing = Remux हो रहा है…
recordings-remux-to-mp4 = MP4 में Remux करें
recordings-export-mp4-title = स्वामित्व वाले .frec को डिकोड करें और MP4 (H.264/AAC) में री-एन्कोड करें ताकि यह किसी भी प्लेयर में चले — FFmpeg कंपोनेंट चाहिए
recordings-exporting = एक्सपोर्ट हो रहा है…
recordings-export-mp4 = एक्सपोर्ट → MP4
recordings-export-mkv-title = स्वामित्व वाले .frec को डिकोड करें और MKV में री-एन्कोड करें ताकि यह किसी भी प्लेयर में चले
recordings-starting = शुरू हो रहा है…
recordings-frames = { $done } / { $total } फ़्रेम
recordings-cancel = रद्द करें
recordings-export-cancelled = एक्सपोर्ट रद्द किया गया।
recordings-exported-to = { $path } में एक्सपोर्ट किया गया
recordings-remuxed-to = { $path } में remux किया गया

# --- OpenedFrec.tsx ---
openfrec-title = .frec रिकॉर्डिंग खोलें
openfrec-desc = Freally Capture स्वामित्व वाले लॉसलेस .frec फ़ॉर्मैट में रिकॉर्ड करता है — यह इसे प्ले नहीं करता। Freally Player रिलीज़ होने पर .frec को सीधे प्ले करेगा। अभी के लिए, इसे MP4/MKV में एक्सपोर्ट करें और यह किसी भी प्लेयर (VLC, आपका OS प्लेयर, कुछ भी) में चलता है।
openfrec-exported-to = { $path } में एक्सपोर्ट किया गया
openfrec-exporting = एक्सपोर्ट हो रहा है…
openfrec-starting = शुरू हो रहा है…
openfrec-export-mp4 = एक्सपोर्ट → MP4
openfrec-export-mkv = एक्सपोर्ट → MKV

# --- VerticalCanvasDialog.tsx ---
vertical-title = वर्टिकल कैनवास (9:16)
vertical-enable = दूसरा कैनवास सक्षम करें — प्रोग्राम से स्वतंत्र रूप से रिकॉर्ड और स्ट्रीम किया जा सकता है
vertical-scene-label = यह कैनवास जो सीन बनाता है
vertical-width = चौड़ाई
vertical-height = ऊँचाई
vertical-preview-alt = वर्टिकल कैनवास प्रीव्यू
vertical-note = आइटम स्थितियाँ कैनवासों में पिक्सेल-सटीक होती हैं: इस प्रीव्यू के वर्टिकल परिणाम दिखाते हुए इसे व्यवस्थित करने के लिए Scenes रेल में यह सीन चुनें। स्ट्रीम लक्ष्य ⦿ Stream… में यह कैनवास चुनते हैं; Settings → Output इसे मुख्य फ़ाइल के साथ रिकॉर्ड कर सकता है।
vertical-close = बंद करें

# --- EulaGate.tsx ---
eula-title = Freally Capture — लाइसेंस अनुबंध
eula-version = v{ $version }
eula-intro = Freally Capture का उपयोग करने के लिए कृपया इस अनुबंध को पढ़ें और स्वीकार करें। संक्षेप में: यह एक तटस्थ टूल है, और आप जो कैप्चर, रिकॉर्ड और प्रसारित करते हैं उसके लिए — और उस पर अधिकार रखने के लिए — पूरी तरह आप ज़िम्मेदार हैं।
eula-thanks = पढ़ने के लिए धन्यवाद।
eula-scroll-hint = जारी रखने के लिए अंत तक स्क्रॉल करें।
eula-decline = अस्वीकार करें और बाहर निकलें
eula-agree = मैं सहमत हूँ


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = आउटपुट
output-loading = सेटिंग्स अभी भी लोड हो रही हैं…
output-container-frec = freally-video (.frec) — लॉसलेस, स्वामित्व वाला, डाउनलोड करने के लिए कुछ नहीं
output-container-mkv = MKV — क्रैश-सहिष्णु; बाद में mp4 में remux करें
output-container-mp4 = MP4 — हर जगह चलता है
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = लॉसलेस
output-preset-lossless-title = स्वामित्व वाला freally-video कोडेक — bit-exact, कोई डाउनलोड नहीं
output-preset-high-label = उच्च गुणवत्ता
output-preset-high-title = MP4, सर्वोत्तम-पहचाना एन्कोडर, near-lossless CQ 16, Quality प्रीसेट
output-preset-balanced-label = संतुलित
output-preset-balanced-title = MKV, सर्वोत्तम-पहचाना एन्कोडर, CQ 23, Balanced प्रीसेट
output-recording-format = रिकॉर्डिंग फ़ॉर्मैट
output-ffmpeg-warning = इस फ़ॉर्मैट के लिए FFmpeg कंपोनेंट चाहिए (वायर कोडेक — बंडल नहीं)। लॉसलेस .frec को कुछ नहीं चाहिए।
output-install = इंस्टॉल करें…
output-recordings-folder = रिकॉर्डिंग फ़ोल्डर
output-folder-placeholder = OS Videos फ़ोल्डर
output-filename-prefix = फ़ाइलनाम उपसर्ग
output-frame-rate = फ़्रेम दर
output-fps-option = { $fps } fps
output-split-every = हर इतने पर स्प्लिट करें (मिनट, 0 = बंद)
output-output-width = आउटपुट चौड़ाई (0 = कैनवास; केवल वायर फ़ॉर्मैट)
output-output-height = आउटपुट ऊँचाई (0 = कैनवास)
output-record-vertical = वर्टिकल कैनवास भी रिकॉर्ड करें (एक समानांतर “… (vertical)” फ़ाइल; 9:16 कैनवास सक्षम होना चाहिए)
output-audio-tracks = ऑडियो ट्रैक
output-recorded-tracks-group = रिकॉर्ड किए गए ट्रैक
output-track-last-one = कम से कम एक ट्रैक रिकॉर्ड होना चाहिए
output-record-track-on = ट्रैक { $index } रिकॉर्ड करें: चालू
output-record-track-off = ट्रैक { $index } रिकॉर्ड करें: बंद
output-encoder-heading = एन्कोडर
output-video-encoder = वीडियो एन्कोडर
output-encoder-auto = ऑटो — सर्वोत्तम पहचाना (H.264)
output-encoder-unavailable = — यहाँ उपलब्ध नहीं
output-preset = प्रीसेट
output-preset-quality = गुणवत्ता
output-preset-balanced-option = संतुलित
output-preset-performance = प्रदर्शन
output-rate-control = रेट कंट्रोल
output-rc-cqp = CQP (स्थिर गुणवत्ता)
output-rc-cbr = CBR (स्थिर बिटरेट)
output-rc-vbr = VBR (परिवर्तनीय बिटरेट)
output-cq = CQ (0–51, कम = बेहतर)
output-bitrate = बिटरेट (kbps)
output-keyframe = कीफ़्रेम अंतराल (s)
output-audio-bitrate = ऑडियो बिटरेट (kbps / ट्रैक)
output-presets = प्रीसेट:

# --- SettingsStream.tsx ---
stream-title = सेटिंग्स — स्ट्रीम
stream-target-enabled = लक्ष्य { $index } सक्षम
stream-target = लक्ष्य { $index }
stream-remove = हटाएँ
stream-service = सेवा
stream-canvas = कैनवास
stream-canvas-main = मुख्य (प्रोग्राम)
stream-canvas-vertical = वर्टिकल (9:16 — इसे स्टूडियो में सक्षम करें)
stream-ingest-srt = SRT ingest URL
stream-ingest-whip = WHIP endpoint URL
stream-ingest-url = Ingest URL
stream-ingest-override = (override — खाली = सेवा प्रीसेट)
stream-key-srt = streamid (वैकल्पिक — ?streamid=… के रूप में जोड़ा जाता है; एक secret माना जाता है)
stream-key-whip = Bearer token (वैकल्पिक — Authorization header के रूप में भेजा जाता है; एक secret)
stream-key-custom = स्ट्रीम की (आपके सर्वर से — एक secret माना जाता है)
stream-key-service = स्ट्रीम की (आपके creator dashboard से — एक secret माना जाता है)
stream-key-aria = स्ट्रीम की { $index }
stream-key-hide = छिपाएँ
stream-key-show = दिखाएँ
stream-encoder = एन्कोडर (H.264 — जो RTMP, SRT और WHIP सभी ले जाते हैं)
stream-encoder-auto = ऑटो — सर्वोत्तम पहचाना H.264 एन्कोडर
stream-encoder-unavailable = (यहाँ उपलब्ध नहीं)
stream-video-bitrate = वीडियो बिटरेट (kbps, CBR)
stream-audio-bitrate = ऑडियो बिटरेट (kbps)
stream-fps = FPS
stream-keyframe = कीफ़्रेम अंतराल (s)
stream-audio-track = ऑडियो ट्रैक (1–6)
stream-output-width = आउटपुट चौड़ाई (0 = कैनवास)
stream-output-height = आउटपुट ऊँचाई (0 = कैनवास)
stream-add-target = + लक्ष्य जोड़ें
stream-go-live-note = लाइव जाएँ हर सक्षम लक्ष्य पर एक साथ प्रकाशित करता है, सीधे हर प्लेटफ़ॉर्म पर। समान एन्कोडर सेटिंग्स वाले लक्ष्य एक ही एन्कोड साझा करते हैं।
stream-auto-record = लाइव जाने पर रिकॉर्डिंग शुरू करें (रिकॉर्डिंग फिर भी स्वतंत्र रूप से रुकती है)
stream-ffmpeg-note-before = स्ट्रीमिंग वायर कोडेक लेबल किए गए ऑन-डिमांड ffmpeg कंपोनेंट के माध्यम से चलते हैं —
stream-ffmpeg-note-link = इसे यहाँ प्रबंधित करें
stream-ffmpeg-note-after = । स्ट्रीम जो भी करे, स्थानीय रिकॉर्डिंग चलती रहती है।
stream-cancel = रद्द करें
stream-save = सहेजें

# --- SettingsReplay.tsx ---
replay-title = सेटिंग्स — रीप्ले बफ़र
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 मिनट
replay-length-2min = 2 मिनट
replay-length-5min = 5 मिनट
replay-quality-low = लो (3 Mbps)
replay-quality-standard = स्टैंडर्ड (6 Mbps)
replay-quality-high = हाई (12 Mbps)
replay-length-presets = लंबाई प्रीसेट
replay-quality-presets = गुणवत्ता प्रीसेट
replay-length-seconds = लंबाई (सेकंड)
replay-video-bitrate = वीडियो बिटरेट (kbps)
replay-fps = FPS
replay-audio-track = ऑडियो ट्रैक (1–6)
replay-note = सक्रिय रहने पर, बफ़र अपना खुद का हल्का एन्कोड एक सीमित ऑन-डिस्क रिंग में चलाता है — इन सेटिंग्स पर लगभग { $mb } MB। सहेजना रिंग को बिना री-एन्कोड किए जोड़ता है और स्ट्रीम या रिकॉर्डिंग को कभी नहीं छूता। बदलाव अगली बार सक्रिय करने पर लागू होते हैं।
replay-cancel = रद्द करें
replay-save = सहेजें

# --- SettingsRemote.tsx ---
remote-title = सेटिंग्स — रिमोट कंट्रोल
remote-enable = WebSocket रिमोट API सक्षम करें
remote-password = पासवर्ड (आवश्यक — कंट्रोलर इससे प्रमाणित होते हैं)
remote-password-placeholder = आपके कंट्रोलर के लिए एक पासवर्ड
remote-password-hide = छिपाएँ
remote-password-show = दिखाएँ
remote-port = पोर्ट
remote-allow-lan = LAN कनेक्शन की अनुमति दें (डिफ़ॉल्ट केवल यह मशीन)
remote-note = बंद = पोर्ट बंद है। चालू = 127.0.0.1 पर (या ऑप्ट-इन करने पर आपके LAN पर) एक पासवर्ड-सुरक्षित WebSocket जो सीन स्विच कर सकता है, ट्रांज़िशन चला सकता है, स्ट्रीम और रिकॉर्डिंग शुरू/बंद कर सकता है, रीप्ले सहेज सकता है, और म्यूट/वॉल्यूम सेट कर सकता है — UI जैसी ही क्रियाएँ, इससे अधिक कुछ नहीं। यह फ़ाइलें नहीं पढ़ सकता। पासवर्ड को किसी भी क्रेडेंशियल की तरह समझें; जब तक आप विशेष रूप से किसी अन्य डिवाइस से नियंत्रण न करें, केवल-यह-मशीन को प्राथमिकता दें।
remote-password-required = रिमोट API सक्षम करने के लिए एक पासवर्ड आवश्यक है।
remote-cancel = रद्द करें
remote-save = सहेजें

# --- SettingsHotkeys.tsx ---
hotkeys-title = सेटिंग्स — हॉटकी
hotkeys-record = रिकॉर्डिंग शुरू / बंद करें
hotkeys-record-placeholder = जैसे Ctrl+Shift+R
hotkeys-go-live = लाइव जाएँ / स्ट्रीम समाप्त करें
hotkeys-go-live-placeholder = जैसे Ctrl+Shift+L
hotkeys-transition = स्टूडियो-मोड ट्रांज़िशन
hotkeys-transition-placeholder = जैसे Ctrl+Shift+T या F13
hotkeys-save-replay = रीप्ले सहेजें (आखिरी N सेकंड)
hotkeys-save-replay-placeholder = जैसे Ctrl+Shift+S
hotkeys-add-marker = एक चैप्टर मार्कर डालें (रिकॉर्डिंग)
hotkeys-add-marker-placeholder = जैसे Ctrl+Shift+K
hotkeys-note = हॉटकी वैश्विक हैं — वे तब भी चलती हैं जब अन्य ऐप फ़ोकस में हों। खाली = अनबाउंड। मिक्सर push-to-talk/mute कीज़ हर स्ट्रिप के ⋯ मेन्यू पर रहती हैं। Linux/Wayland पर, वैश्विक हॉटकी उपलब्ध नहीं हो सकतीं (एक कम्पोज़िटर सीमा) — बटन काम करते रहते हैं।
hotkeys-cancel = रद्द करें
hotkeys-save = सहेजें

# --- WorkspaceDialog.tsx ---
workspace-title = प्रोफ़ाइल और सीन कलेक्शन
workspace-profiles = प्रोफ़ाइल
workspace-profiles-hint = एक प्रोफ़ाइल आपकी सेटिंग्स है — स्ट्रीम लक्ष्य, आउटपुट, हॉटकी। प्रति शो या प्रति प्लेटफ़ॉर्म स्विच करें।
workspace-collections = सीन कलेक्शन
workspace-collections-hint = एक कलेक्शन आपके सीन + सोर्स है। Create वर्तमान को एक शुरुआती बिंदु के रूप में डुप्लिकेट करता है।
workspace-active = सक्रिय
workspace-switch-to = { $name } पर स्विच करें
workspace-active-marker = ● सक्रिय
workspace-new-name-placeholder = नया नाम…
workspace-new-name-label = नया { $title } नाम
workspace-create = बनाएँ

# --- ScriptsDialog.tsx ---
scripts-title = स्क्रिप्ट (Lua)
scripts-empty = अभी कोई स्क्रिप्ट नहीं — एक .lua फ़ाइल जोड़ें। API के लिए scripts/sample.lua देखें: go-live/scene/recording घटनाओं पर प्रतिक्रिया दें और रिमोट API जैसी ही कमांड चलाएँ।
scripts-enable = { $path } सक्षम करें
scripts-remove = { $path } हटाएँ
scripts-path-label = स्क्रिप्ट path
scripts-add = जोड़ें
scripts-note = स्क्रिप्ट सैंडबॉक्स में चलती हैं — कोई फ़ाइल या OS एक्सेस नहीं; वे केवल रिमोट API जैसी ही स्टूडियो कमांड कॉल कर सकती हैं (सीन स्विच, ट्रांज़िशन, रिकॉर्ड/स्ट्रीम/रीप्ले, म्यूट)। एक स्क्रिप्ट त्रुटि लॉग होती है और सीमित रहती है। बदलाव एक सेकंड के भीतर लागू होते हैं।
scripts-error-not-lua = एक .lua फ़ाइल पर इंगित करें।

# --- BrowserDock.tsx ---
browser-dock-title = ब्राउज़र डॉक
browser-dock-empty = अभी कोई डॉक नहीं — एक चैट पॉपआउट, एक अलर्ट पेज, या अपने Companion वेब बटन जोड़ें।
browser-dock-open = खोलें
browser-dock-remove = { $name } हटाएँ
browser-dock-name-placeholder = नाम (जैसे Twitch Chat)
browser-dock-name-label = डॉक नाम
browser-dock-url-label = डॉक URL
browser-dock-note = एक डॉक अपनी विंडो के रूप में खुलता है जिसे आप स्टूडियो के बगल में रख सकते हैं। पेज को ऐप तक कोई पहुँच नहीं मिलती — यह बस रेंडर करता है। केवल http(s) URL; डॉक तभी खुलते हैं जब आप Open क्लिक करते हैं।
browser-dock-error-name = डॉक को नाम दें (जैसे Twitch Chat)।
browser-dock-error-url = एक डॉक URL http:// या https:// से शुरू होना चाहिए।

# --- studio-preview-pane ---
studio-preview-label = स्टूडियो मोड प्रीव्यू
studio-preview-heading = प्रीव्यू
studio-preview-hint = इसे यहाँ लोड करने के लिए किसी सीन पर क्लिक करें
studio-preview-empty = प्रीव्यू यहाँ दिखाई देगा।
studio-preview-mirrors = प्रोग्राम को मिरर करता है
studio-preview-transition-select = ट्रांज़िशन
studio-preview-duration = ट्रांज़िशन अवधि (ms)
studio-preview-commit-title = ट्रांज़िशन के माध्यम से प्रीव्यू → प्रोग्राम में कमिट करें (दर्शक इसे देखते हैं)
studio-preview-transitioning = ट्रांज़िशन हो रहा है…
studio-preview-transition-button = ट्रांज़िशन ⇄
studio-preview-luma-placeholder = ग्रेस्केल वाइप छवि (png/jpg)
studio-preview-luma-label = ल्यूमा वाइप छवि
studio-preview-browse = ब्राउज़ करें…
studio-preview-filter-images = छवियाँ
studio-preview-filter-video = वीडियो
studio-preview-stinger-placeholder = स्टिंगर वीडियो (ProRes 4444 .mov अपना अल्फा बनाए रखता है)
studio-preview-stinger-label = स्टिंगर वीडियो फ़ाइल
studio-preview-stinger-cut-label = स्टिंगर कट पॉइंट (ms)
studio-preview-stinger-cut-title = जब सीन स्वैप स्टिंगर के नीचे आता है (ट्रांज़िशन के भीतर ms)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = कट
transition-kind-fade = फ़ेड
transition-kind-slide-left = स्लाइड ←
transition-kind-slide-right = स्लाइड →
transition-kind-slide-up = स्लाइड ↑
transition-kind-slide-down = स्लाइड ↓
transition-kind-swipe-left = स्वाइप ←
transition-kind-swipe-right = स्वाइप →
transition-kind-luma-linear = ल्यूमा वाइप (रैखिक)
transition-kind-luma-radial = ल्यूमा वाइप (रेडियल)
transition-kind-luma-horizontal = ल्यूमा वाइप (क्षैतिज)
transition-kind-luma-diamond = ल्यूमा वाइप (डायमंड)
transition-kind-luma-clock = ल्यूमा वाइप (घड़ी)
transition-kind-image = छवि वाइप (कस्टम)
transition-kind-stinger = स्टिंगर (वीडियो)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = कस्टम (RTMP/RTMPS)
stream-service-srt = SRT (स्व-होस्टेड)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = परिचय
about-tagline = स्टूडियो की तरह रिकॉर्ड और स्ट्रीम करें — कोई अकाउंट नहीं, कोई क्लाउड नहीं।
about-version = संस्करण
about-created-by = निर्माता
about-project-started = प्रोजेक्ट शुरू हुआ
about-first-stable = पहली स्थिर रिलीज़
about-first-stable-pending = अभी नहीं — 1.0.0 प्रगति पर है
about-platform = प्लेटफ़ॉर्म
about-local-first = Freally Capture पूरी तरह आपकी मशीन पर चलता है। कोई अकाउंट नहीं, कोई टेलीमेट्री नहीं, कोई क्लाउड नहीं — आपके कंप्यूटर से केवल वही चीज़ बाहर जाती है जो स्ट्रीम आपने भेजने के लिए चुनी है।
about-website = वेबसाइट
about-issues = समस्या रिपोर्ट करें
about-license = लाइसेंस
about-eula = EULA
about-third-party = तृतीय-पक्ष सूचनाएँ
about-check-updates = अपडेट जाँचें…

# --- unified settings modal (TASK-906) ---
settings-title = सेटिंग्स
settings-language-section = भाषा
settings-language = इंटरफ़ेस भाषा
settings-language-system = सिस्टम डिफ़ॉल्ट
settings-language-note = यहाँ चुनी गई भाषा याद रखी जाती है। “सिस्टम डिफ़ॉल्ट” आपके ऑपरेटिंग सिस्टम का अनुसरण करता है। अनुवादित न किया गया टेक्स्ट अंग्रेज़ी पर वापस आ जाता है।
settings-appearance-section = दिखावट
settings-theme = थीम
settings-theme-dark = गहरा
settings-theme-light = हल्का
settings-theme-custom = कस्टम
settings-accent = एक्सेंट
settings-general-section = सामान्य
settings-show-stats-dock = आँकड़े डॉक दिखाएँ
settings-more-section = अधिक सेटिंग्स
settings-open-output = रिकॉर्डिंग…
settings-open-stream = स्ट्रीमिंग…
settings-open-replay = रीप्ले…
settings-open-hotkeys = हॉटकी…
settings-open-remote = रिमोट API…
settings-open-about = परिचय…
controls-settings = ⚙ सेटिंग्स…
controls-settings-title = भाषा, दिखावट और ऐप-व्यापी प्राथमिकताएँ

# --- command palette (TASK-904) ---
palette-title = कमांड पैलेट
palette-search = सीन, सोर्स और क्रियाएँ खोजें
palette-placeholder = सीन, सोर्स, क्रियाएँ खोजें…
palette-no-results = “{ $query }” से कुछ भी मेल नहीं खाता
palette-hint = ↑ ↓ ले जाने के लिए · Enter चलाने के लिए · Esc बंद करने के लिए
palette-group-scenes = सीन
palette-group-sources = सोर्स
palette-group-actions = क्रिया
palette-transition = ट्रांज़िशन प्रीव्यू → प्रोग्राम
palette-save-replay = रीप्ले सहेजें
palette-add-marker = एक चैप्टर मार्कर डालें
palette-vertical-canvas = वर्टिकल (9:16) कैनवास…
