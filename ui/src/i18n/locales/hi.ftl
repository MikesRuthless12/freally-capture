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
remote-center-my-cam = मेरा कैम
remote-center-my-screen = मेरी स्क्रीन
remote-center-host-view = होस्ट व्यू
remote-end-session = सत्र समाप्त करें
remote-leave = छोड़ें
remote-host-view-heading = होस्ट व्यू
remote-host-shared-view-label = होस्ट का साझा व्यू
remote-guest-position-label = गेस्ट स्थिति
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
sources-badge-test-bars = बार्स
sources-badge-test-grid = ग्रिड
sources-badge-test-sweep = स्वीप
sources-badge-test-tone = टोन
sources-badge-test-sync = सिंक
sources-badge-timer = टाइमर

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
sources-add-timer = टाइमर / घड़ी
sources-add-nested-scene = नेस्टेड सीन
sources-add-slideshow = इमेज स्लाइडशो
sources-add-chat-overlay = लाइव चैट ओवरले
sources-add-test-signal = टेस्ट सिग्नल
sources-add-audio-input = ऑडियो इनपुट कैप्चर
sources-add-audio-output = ऑडियो आउटपुट कैप्चर
sources-add-app-audio = एप्लिकेशन ऑडियो (Windows)
sources-add-background-music = पृष्ठभूमि संगीत…
sources-background-music-note = लूप होता है, प्रीव्यू में सुनाई देता है और रिकॉर्डिंग/स्ट्रीम में कैप्चर होता है। वीडियो की आवाज़ के बजाय संगीत के लिए, मिक्सर में वीडियो की Backdrop स्ट्रिप म्यूट करें।
sources-background-music-count = { $n } ट्रैक
sources-background-music-shuffle = शफ़ल
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
sources-stream-hide = स्ट्रीम में छिपाएँ
sources-stream-show = स्ट्रीम में दिखाएँ
sources-stream-hide-item = { $name } को स्ट्रीम में छिपाएँ
sources-stream-show-item = { $name } को स्ट्रीम में दिखाएँ
sources-record-hide = रिकॉर्डिंग में छिपाएँ
sources-record-show = रिकॉर्डिंग में दिखाएँ
sources-record-hide-item = { $name } को रिकॉर्डिंग में छिपाएँ
sources-record-show-item = { $name } को रिकॉर्डिंग में दिखाएँ
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
sources-testsignal-title = टेस्ट सिग्नल जोड़ें
sources-testsignal-pattern-label = पैटर्न
sources-testsignal-bars = SMPTE कलर बार्स
sources-testsignal-grid = कैलिब्रेशन ग्रिड
sources-testsignal-sweep = मोशन स्वीप
sources-testsignal-tone = 1 kHz टोन (−20 dBFS)
sources-testsignal-flash-beep = A/V सिंक फ़्लैश + बीप
sources-testsignal-note = बिना कैमरा जोड़े दृश्य, एन्कोडर, प्रोजेक्टर और स्ट्रीम लक्ष्य जाँचें। फ़्लैश + बीप पैटर्न A/V सिंक वर्कबेंच को चलाता है।
sources-testsignal-add = टेस्ट सिग्नल जोड़ें
sources-timer-title = टाइमर जोड़ें
sources-timer-mode-label = मोड
sources-timer-wall-clock = दीवार घड़ी
sources-timer-countdown = काउंटडाउन
sources-timer-stopwatch = स्टॉपवॉच
sources-timer-since-live = लाइव होने से समय
sources-timer-since-recording = रिकॉर्डिंग से समय
sources-timer-note = अवधि, फ़ॉर्मैट, स्टाइल और काउंटडाउन-समाप्ति क्रियाएँ स्रोत की प्रॉपर्टीज़ में हैं।
sources-timer-add = टाइमर जोड़ें

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
controls-iso-lanes = प्रोग्राम के साथ रिकॉर्ड हो रही ISO लेनें: { $count }
controls-pause-title-resume = फिर से शुरू करें — फ़ाइल एक ही सतत टाइमलाइन के रूप में जारी रहती है
controls-pause-title-pause = रोकें — कोई फ़्रेम नहीं लिखा जाता; फिर से शुरू करने पर वही प्ले करने योग्य फ़ाइल जारी रहती है
controls-resume-recording = ▶ रिकॉर्डिंग फिर से शुरू करें
controls-pause-recording = ⏸ रिकॉर्डिंग रोकें
controls-reactions-label = रिएक्शन (प्रोग्राम में बेक किए गए)
controls-reactions-title = प्रोग्राम के ऊपर एक रिएक्शन तैराएँ — रिकॉर्ड और स्ट्रीम दोनों, ताकि रीप्ले सही पल दिखाए। चैट में दर्शक भी इन्हें ट्रिगर करते हैं (उनका रिएक्शन इमोजी अपने आप तैरता है); एक बाढ़ केवल स्क्रीन पर मौजूद संख्या को सीमित करती है।
controls-react = रिएक्ट { $emoji }
controls-virtual-camera-title = वर्चुअल कैमरे को प्रति OS अपना साइन किया हुआ ड्राइवर कंपोनेंट चाहिए (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO extension / Linux v4l2loopback) — यह अपने माइलस्टोन के रूप में आता है। फ़ीड मॉडल इसके लिए तैयार है: प्रोग्राम, वर्टिकल कैनवास, या एक अकेला सोर्स, Windows/Linux पर एक जोड़े वर्चुअल माइक के साथ (macOS में कोई वर्चुअल-माइक API नहीं है — ईमानदारी से कहा)।
controls-virtual-camera = ⌁ वर्चुअल कैमरा शुरू करें
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
mixer-routing = रूटिंग
mixer-routing-title = ऑडियो आउटपुट रूटिंग

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = ऑडियो रूटिंग
routing-intro = स्ट्रिप्स को ट्रैक बसों में असाइन करें, फिर किसी भी बस को किसी फिज़िकल आउटपुट पर भेजें — किसी हार्डवेयर रिकॉर्डर की फ़ीड, दूसरे कमरे के स्पीकर, या किसी खाली ट्रैक पर हेडफ़ोन क्यू। मॉनिटर अपना खुद का डिवाइस रखता है; ये रूट उसके ऊपर जोड़े जाते हैं, इसलिए कोई सेट न होने पर मिक्स अपरिवर्तित रहता है।
routing-sends-title = ट्रैक सेंड
routing-no-strips = इस सीन में कोई ऑडियो स्रोत नहीं है।
routing-source = स्रोत
routing-track = ट्रैक { $n }
routing-send-aria = { $source } को ट्रैक { $n } पर भेजें
routing-outputs-title = फिज़िकल आउटपुट
routing-master = मास्टर
routing-off = बंद
routing-default-output = डिफ़ॉल्ट आउटपुट
routing-device-aria = { $bus } के लिए आउटपुट डिवाइस
routing-trim-aria = { $bus } के लिए आउटपुट ट्रिम
routing-trim-db = { $db } dB
routing-muted = म्यूट
routing-device-error = डिवाइस उपलब्ध नहीं

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = डकिंग
mixer-ducking-title = डकिंग मैट्रिक्स
ducking-title = डकिंग मैट्रिक्स
ducking-intro = कोई भी स्रोत किसी भी अन्य को डक कर सकता है। जब भी ट्रिगर (पंक्ति) बोलता है, सेल लक्ष्य (कॉलम) को दबा देती है — गहराई, थ्रेशोल्ड और टाइमिंग सेट करने के लिए कोई सेल चुनें। हर जोड़ी अपनी अलग डक होती है, इसलिए एक स्ट्रिप को एक साथ कई ट्रिगर द्वारा डक किया जा सकता है।
ducking-need-two = उनके बीच डक करने के लिए कम से कम दो ऑडियो स्रोत जोड़ें।
ducking-trigger-target = ट्रिगर ↓ / लक्ष्य →
ducking-cell-aria = { $trigger } { $target } को डक करता है
ducking-pair = { $trigger } → { $target }
ducking-remove = हटाएँ
ducking-amount = मात्रा
ducking-threshold = थ्रेशोल्ड
ducking-attack = अटैक
ducking-release = रिलीज़
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = लाउडनेस सामान्यीकरण
loudness-intro = प्रोग्राम को पीक सीलिंग के साथ लाउडनेस लक्ष्य की ओर धीरे-धीरे ले जाता है, ताकि आपकी स्ट्रीम और रिकॉर्डिंग एक समान स्तर पर पहुँचें। धीमा और सौम्य — यह दिशा देता है, कभी पंप नहीं करता।
loudness-enable = प्रोग्राम को लक्ष्य तक ले जाएँ
loudness-target = लक्ष्य
loudness-target-option = { $target } LUFS
loudness-ceiling = पीक सीलिंग (dBFS)
loudness-note = −14 LUFS YouTube-शैली प्लेबैक के लिए उपयुक्त है; −16 एक सामान्य स्ट्रीमिंग लक्ष्य है; −23 EBU R128 प्रसारण है। यही लक्ष्य रिकॉर्ड के बाद वाली सामान्यीकरण क्रिया द्वारा उपयोग किया जाता है।
ltc-badge = LTC
ltc-title = SMPTE टाइमकोड (LTC)
ltc-intro = किसी ट्रैक पर SMPTE रैखिक टाइमकोड बनाएँ, और किसी भी ऑडियो इनपुट से आने वाला LTC पढ़ें — पोस्ट में बाहरी रिकॉर्डर और कैमरों को सिंक करने का क्लासिक ऑडियो टाइमकोड। पूर्णतः ऑफ़लाइन।
ltc-generate = किसी ट्रैक पर LTC बनाएँ
ltc-track = टाइमकोड ट्रैक
ltc-track-option = ट्रैक { $track }
ltc-fps = फ़्रेम दर
ltc-read = LTC यहाँ से पढ़ें
ltc-read-off = बंद
ltc-decoded = आने वाला टाइमकोड
ltc-no-lock = कोई संकेत नहीं
ltc-note = जनरेटर दिन के समय से सिंक होता है, नॉन-ड्रॉप। इसका ट्रैक रिकॉर्ड करें (आउटपुट सेटिंग्स में असाइन करें) या बाहरी उपकरण को फीड करने हेतु किसी आउटपुट पर रूट करें। रीडर स्टैट्स-ओवरले टाइमकोड लाइन चलाता है और चैप्टर मार्कर पर मुहर लगाता है।
loudness-on = LUFS { $target }
loudness-off = सामान्यीकरण बंद

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = साउंडबोर्ड
mixer-soundboard-title = साउंडबोर्ड
soundboard-title = साउंडबोर्ड
soundboard-add-pad = + पैड
soundboard-stop-all = सभी रोकें
soundboard-edit = संपादित करें
soundboard-empty = अभी कोई पैड नहीं — एक जोड़ें और उसे एक स्थानीय ऑडियो क्लिप असाइन करें।
soundboard-new-pad = नया पैड
soundboard-no-clip = कोई क्लिप नहीं
soundboard-audio-files = ऑडियो फ़ाइलें
soundboard-name = नाम
soundboard-choose-clip = क्लिप चुनें…
soundboard-gain = गेन
soundboard-choke = चोक
soundboard-choke-none = कोई नहीं
soundboard-loop = लूप
soundboard-auto-duck = ऑटो-डकिंग
soundboard-tracks = ट्रैक
soundboard-hotkey = हॉटकी
soundboard-hotkey-placeholder = जैसे Ctrl+Shift+1
soundboard-remove = हटाएँ

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = प्लगइन
mixer-plugins-title = ऑडियो प्लगइन (CLAP / VST3)
plugins-title = ऑडियो प्लगइन
plugins-scanning = स्कैन हो रहा है…
plugins-none = मानक फ़ोल्डरों में कोई CLAP या VST3 प्लगइन नहीं मिला।

# --- StatsDock.tsx ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = मेमोरी
stats-dropped = ड्रॉप हुए
stats-render = रेंडर
stats-gpu = GPU
stats-gpu-compositing = कम्पोज़िटिंग
stats-gpu-idle = निष्क्रिय
stats-disk = डिस्क
stats-disk-free = खाली
stats-disk-left = रिकॉर्ड शेष
stats-disk-rate = ≈ { $rate } MB/s रिकॉर्डिंग
stats-vertical-fps = 9:16 FPS
stats-targets-label = स्ट्रीम लक्ष्य
stats-rehearsal-note = रिहर्सल — लक्ष्य केवल लोकल सिंक पर प्रकाशित करते हैं
stats-timeline-open = टाइमलाइन
timeline-title = सत्र टाइमलाइन
timeline-empty = अभी कुछ रिकॉर्ड नहीं — स्ट्रीम या रिकॉर्डिंग के दौरान टाइमलाइन दर्ज होती है।
timeline-live = LIVE — अभी रिकॉर्ड हो रही है
timeline-fit = फ़िट करें
timeline-legend-fps = fps
timeline-legend-behind = एन्कोडर कतार (पीछे फ़्रेम)
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
scenes-recent-title = हाल की
scenes-recent-clear = साफ़ करें
scenes-recent-clear-confirm = हाल के दृश्यों की सूची साफ़ करें?


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
channelstrip-solo-title = सोलो (PFL) — मॉनिटर केवल सोलो स्ट्रिप सुनता है; प्रोग्राम मिक्स अछूता रहता है
channelstrip-solo-source = { $name } सोलो (PFL)
channelstrip-pan-label = बैलेंस (डबल-क्लिक से रीसेट)
channelstrip-pan-aria = { $name } का बैलेंस
channelstrip-mono-label = मोनो में डाउनमिक्स
channelstrip-automix-label = ऑटो-मिक्स (गेन-शेयरिंग)
channelstrip-automix-note = गेन-शेयरिंग: मिक्सर सभी ऑटो-मिक्स स्ट्रिप के संयुक्त स्तर को स्थिर रखता है और उसे उसी को सौंपता है जो बोल रहा हो — मल्टी-माइक पैनल और पॉडकास्ट के लिए आदर्श। जब तक आप कोई स्ट्रिप न जोड़ें, तब तक बंद।
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = इस सोर्स के लिए एक इको-मुक्त रिटर्न बनाता है — प्रोग्राम में सभी, सिवाय इसी सोर्स के। इसे किसी रिमोट गेस्ट के लिए इस्तेमाल करें ताकि उन्हें अपनी ही विलंबित आवाज़ न सुनाई दे।
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
livebutton-rehearse = रिहर्सल
livebutton-rehearse-record-confirm = क्या इस रिहर्सल को वीडियो फ़ाइल में भी रिकॉर्ड करें?
livebutton-rehearse-title = पूरा शो एक लोकल सिंक पर चलाएँ — कुछ भी भेजा नहीं जाता
livebutton-end-rehearsal = रिहर्सल समाप्त करें
livebutton-title-rehearsing = रिहर्सल चल रही है — इस मशीन से कुछ भी बाहर नहीं जाता
livebutton-badge-rehearsal = रिहर्सल
livebutton-aria-rehearsal = रिहर्सल जारी है
livebutton-rehearsal-banner = रिहर्सल — इस मशीन से कुछ भी बाहर नहीं जाता

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
properties-testtone-note = −20 dBFS पर लगातार 1 kHz साइन टोन। स्तर और म्यूट इसकी मिक्सर स्ट्रिप पर हैं; और कुछ सेट करने को नहीं है।
properties-timer-format = समय फ़ॉर्मैट (strftime)
properties-timer-format-note = जैसे %H:%M:%S (डिफ़ॉल्ट), %I:%M %p, %A %H:%M — अमान्य पैटर्न %H:%M:%S पर लौट आता है।
properties-timer-utc = UTC ऑफ़सेट (मिनट)
properties-timer-utc-placeholder = स्थानीय समय
properties-timer-duration = अवधि (सेकंड)
properties-timer-clock = किसी घड़ी समय तक गिनती
properties-timer-end = शून्य पर
properties-timer-end-none = कुछ न करें
properties-timer-end-flash = टाइमर चमकाएँ
properties-timer-end-switch = दृश्य बदलें
properties-timer-end-scene = दृश्य
properties-timer-size = आकार (px)
properties-timer-start = शुरू
properties-timer-pause = रोकें
properties-timer-reset = रीसेट
properties-text-file = फ़ाइल से पढ़ें (पथ; खाली = ऊपर का टेक्स्ट)
properties-text-binding = इस रूप में पार्स करें
properties-text-binding-whole = पूरी फ़ाइल
properties-text-binding-csv = CSV सेल
properties-text-binding-json = JSON पॉइंटर
properties-text-csv-row = पंक्ति
properties-text-csv-column = कॉलम
properties-text-csv-column-placeholder = नाम या क्रमांक
properties-text-json-pointer = पॉइंटर
properties-text-file-note = बदलाव के आधे सेकंड में फ़ाइल दोबारा पढ़ी जाती है। एटॉमिक लेखन (टेम्प + रीनेम) सहा जाता है: बदलाव के दौरान आख़िरी सही मान स्क्रीन पर बना रहता है।
avsync-title = A/V सिंक कैलिब्रेशन
avsync-intro = अंतर्निर्मित फ़्लैश + बीप पैटर्न को अपनी स्क्रीन और स्पीकर से चलाएँ, जिन कैमरा-माइक को संरेखित करना है उनसे उसे कैप्चर करें — वर्कबेंच दोनों के बीच का अंतर मापता है। लूप स्क्रीन और स्पीकर से होकर जाता है, इसलिए उनकी छोटी लेटेंसी भी शामिल रहती है।
avsync-video-label = कैमरा (वीडियो स्रोत)
avsync-audio-label = माइक्रोफ़ोन (ऑडियो स्रोत)
avsync-pick = स्रोत चुनें…
avsync-no-video = पहले कैमरे को स्रोत के रूप में जोड़ें — वर्कबेंच स्रोत मापता है, सीधे डिवाइस नहीं।
avsync-no-audio = पहले माइक्रोफ़ोन को ऑडियो स्रोत के रूप में जोड़ें।
avsync-projector = प्रोग्राम को फ़ुलस्क्रीन दिखाएँ:
avsync-projector-open = प्रोजेक्टर खोलें
avsync-projector-window-title = प्रोग्राम — A/V सिंक
avsync-start-note = शुरू करने पर वर्तमान दृश्य के ऊपर अस्थायी "A/V सिंक पैटर्न" स्रोत जुड़ता है और बीप मॉनिटर डिवाइस पर बजती है। रन खत्म होते ही सब हट जाता है।
avsync-manual = सिंक ऑफ़सेट (ms, मैनुअल)
avsync-start = कैलिब्रेशन शुरू करें
avsync-measuring = लगभग 12 सेकंड माप हो रहा है — कैमरे को चमकते प्रोग्राम की ओर रखें और कमरे को स्थिर रखें…
avsync-flash-seen = कैमरा फ़्लैश देख रहा है
avsync-flash-waiting = कैमरे के फ़्लैश देखने की प्रतीक्षा…
avsync-beep-heard = माइक्रोफ़ोन बीप सुन रहा है
avsync-beep-waiting = माइक्रोफ़ोन के बीप सुनने की प्रतीक्षा…
avsync-cancel = रद्द करें
avsync-result-offset = वीडियो ऑडियो के { $offset } ms बाद पहुँचता है।
avsync-result-detail = { $cycles } चक्रों में मापा गया, ±{ $jitter } ms।
avsync-negative = ऑडियो पहले से ही वीडियो के बाद पहुँचता है। ऑडियो में देरी इस दिशा को ठीक नहीं कर सकती — अगर इस कैमरे की आवाज़ किसी और स्ट्रिप पर है तो वहाँ का ऑफ़सेट घटाएँ।
avsync-over-cap = मापा गया अंतर { $max } ms की सीमा से बाहर है। इतना बड़ा अंतर प्रायः गलत स्रोत चुनने से आता है — चेन जाँचें और फिर मापें।
avsync-applied = लागू — माइक्रोफ़ोन का सिंक ऑफ़सेट अब { $offset } ms है।
avsync-apply = माइक्रोफ़ोन पर { $offset } ms लागू करें
avsync-again = फिर मापें
avsync-close = बंद करें
avsync-error-noFlash = कैमरे ने फ़्लैश कभी नहीं देखा। उसे चमकते प्रोग्राम की ओर करें (फ़ुलस्क्रीन मदद करता है), स्रोत लाइव है यह पक्का करें, फिर मापें।
avsync-error-noBeep = माइक्रोफ़ोन ने बीप कभी नहीं सुनी। मॉनिटर डिवाइस सुनाई दे रहा है और माइक लाइव है (पुश-टू-टॉक से बंद नहीं) यह पक्का करें, फिर मापें।
avsync-error-tooFewCycles = पर्याप्त साफ़ फ़्लैश/बीप चक्र नहीं मिले। पूरे रन में पैटर्न स्पष्ट दिखता-सुनता रहे।
avsync-error-notThePattern = जो दिखा/सुना गया वह पैटर्न की लय पर नहीं दोहराता — शायद कमरे की रोशनी या शोर है, परीक्षण संकेत नहीं।
avsync-error-unstable = चक्र आपस में इतने अलग हैं कि एक संख्या पर भरोसा नहीं हो सकता। कैमरा स्थिर करें, शोर घटाएँ, फिर मापें।
hotkey-audit-title = हॉटकी मैप
hotkey-audit-search = खोजें
hotkey-audit-filter = फ़ीचर
hotkey-audit-filter-all = सभी फ़ीचर
hotkey-audit-col-key = कुंजी
hotkey-audit-col-action = क्रिया
hotkey-audit-col-where = कहाँ
hotkey-audit-col-status = स्थिति
hotkey-audit-ok = ठीक
hotkey-audit-shared = { $count } बाइंडिंग साझा करती हैं
hotkey-audit-unregistered = OS में पंजीकृत नहीं (कहीं और कब्ज़े में या अनुपलब्ध)
hotkey-audit-invalid = मान्य शॉर्टकट नहीं
hotkey-audit-empty = अभी कोई हॉटकी नहीं — सेटिंग्स → हॉटकीज़ या मिक्सर स्ट्रिप पर बाँधें।
hotkey-audit-export = चीट शीट निर्यात करें
hotkey-audit-exported = { $path } में सहेजा गया
hotkey-audit-note = कुंजियाँ सेटिंग्स → हॉटकीज़ (वैश्विक क्रियाएँ) और हर मिक्सर स्ट्रिप (पुश-टू-टॉक / पुश-टू-म्यूट) में बाँधें-बदलें; यह तालिका उनका ऑडिट और दस्तावेज़ करती है।
hotkey-audit-action-record = रिकॉर्डिंग टॉगल
hotkey-audit-action-go-live = स्ट्रीमिंग टॉगल
hotkey-audit-action-transition = ट्रांज़िशन लागू करें
hotkey-audit-action-save-replay = रीप्ले सहेजें
hotkey-audit-action-add-marker = मार्कर जोड़ें
hotkey-audit-action-still = स्टिल कैप्चर करें
hotkey-audit-action-panic = पैनिक स्लेट
hotkey-audit-action-timer-toggle = सभी टाइमर शुरू/रोकें
hotkey-audit-action-timer-reset = सभी टाइमर रीसेट
hotkey-audit-action-ptt = पुश-टू-टॉक
hotkey-audit-action-ptm = पुश-टू-म्यूट
hotkey-audit-feature-recording = रिकॉर्डिंग
hotkey-audit-feature-streaming = स्ट्रीमिंग
hotkey-audit-feature-studio = स्टूडियो मोड
hotkey-audit-feature-replay = रीप्ले
hotkey-audit-feature-markers = मार्कर
hotkey-audit-feature-stills = स्टिल
hotkey-audit-feature-panic = पैनिक
hotkey-audit-feature-timers = टाइमर
hotkey-audit-feature-audio = ऑडियो (प्रति स्रोत)
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
properties-deinterlace = डी-इंटरलेसिंग
properties-deinterlace-off = बंद
properties-deinterlace-discard = डिस्कार्ड (एक फ़ील्ड की लाइनें दोहराएँ)
properties-deinterlace-bob = बॉब (फ़ील्ड बारी-बारी)
properties-deinterlace-linear = रैखिक (इंटरपोलेट)
properties-deinterlace-blend = ब्लेंड (फ़ील्ड औसत)
properties-deinterlace-adaptive = मोशन-अनुकूली (yadif-श्रेणी)
properties-field-order = फ़ील्ड क्रम
properties-field-order-top = पहले ऊपरी फ़ील्ड
properties-field-order-bottom = पहले निचला फ़ील्ड
properties-deinterlace-note = इंटरलेस्ड कैप्चर-कार्ड फ़ीड के लिए। शुद्ध CPU, हर OS पर एक जैसा; बदलने पर डिवाइस पुनरारंभ होता है (फ़ॉर्मैट बदलने जैसा)।
camera-controls-title = कैमरा नियंत्रण
camera-controls-refresh = रीफ़्रेश
camera-controls-reset = प्रोफ़ाइल रीसेट
camera-controls-empty = अभी कोई नियंत्रण नहीं — डिवाइस स्ट्रीमिंग में होना चाहिए (पहले इसे दृश्य में जोड़ें), और कुछ बैकएंड कोई नियंत्रण नहीं देते (खासकर macOS)। यही हर OS की ईमानदार स्थिति है।
camera-controls-note = बदलाव तुरंत लागू होते हैं और डिवाइस की प्रोफ़ाइल में सहेजे जाते हैं, जो दोबारा जोड़ने और रीस्टार्ट पर फिर लागू होती है।
camera-control-brightness = चमक
camera-control-contrast = कंट्रास्ट
camera-control-hue = रंगत
camera-control-saturation = संतृप्ति
camera-control-sharpness = तीक्ष्णता
camera-control-gamma = गामा
camera-control-white-balance = व्हाइट बैलेंस
camera-control-backlight = बैकलाइट क्षतिपूर्ति
camera-control-gain = गेन
camera-control-pan = पैन
camera-control-tilt = टिल्ट
camera-control-zoom = ज़ूम
camera-control-exposure = एक्सपोज़र
camera-control-iris = आइरिस
camera-control-focus = फ़ोकस
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
audiofilters-name-parametric-eq = पैरामीट्रिक EQ
audiofilters-name-de-esser = डी-एसर
audiofilters-name-rumble-guard = रंबल गार्ड
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = प्रीसेट
audiofilters-voice-preset-pick = वॉइस प्रीसेट…
audiofilters-voice-broadcast = ब्रॉडकास्ट वॉइस
audiofilters-voice-podcast = पॉडकास्ट वॉइस
audiofilters-voice-clean = क्लीन वॉइस
audiofilters-voice-none = चेन साफ़ करें
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = सिबिलेंस फ़्रीक्वेंसी (Hz)
audiofilters-deesser-amount = अधिकतम कटौती (dB)
audiofilters-rumble-freq = लो-कट (Hz)
audiofilters-title = ऑडियो फ़िल्टर — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = लाइव स्पेक्ट्रम के साथ पैरामीट्रिक EQ रिस्पॉन्स कर्व
eq-band-type = प्रकार
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + बैंड
eq-remove-band = बैंड हटाएं
eq-type-bell = बेल
eq-type-lowShelf = लो शेल्फ
eq-type-highShelf = हाई शेल्फ
eq-type-notch = नॉच
eq-type-highPass = हाई-पास
eq-type-lowPass = लो-पास
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
filters-name-perspective = परिप्रेक्ष्य
filters-name-fade-loop = फ़ेड लूप
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
filters-name-shader = शेडर (WGSL)
filters-shader-gallery = गैलरी
filters-shader-gallery-pick = प्रीसेट लोड करें…
filters-shader-gallery-grayscale = ग्रेस्केल
filters-shader-gallery-invert = इनवर्ट
filters-shader-gallery-scanlines = स्कैनलाइन
filters-shader-gallery-vignette = विनेट
filters-shader-source = शेडर स्रोत (WGSL)
filters-shader-hint = WGSL में एक effect(uv, color, p, texel, time) लिखें जो vec4 लौटाए। स्लाइडर के लिए पैरामीटर को // @param name min max default से एनोटेट करें। अमान्य शेडर अनदेखा किया जाता है — जब तक यह कंपाइल न हो, स्रोत बिना फ़िल्टर के रेंडर होता है।
filters-name-bezier-mask = बेज़ियर मास्क
filters-mask-editor-hint = किसी बिंदु को खींचकर हिलाएँ, जोड़ने के लिए डबल-क्लिक करें, हटाने के लिए बिंदु पर राइट-क्लिक करें।
filters-mask-shape = आकार
filters-mask-shape-pick = प्रीसेट…
filters-mask-shape-rectangle = आयत
filters-mask-shape-diamond = हीरा
filters-mask-shape-hexagon = षट्भुज
filters-mask-shape-circle = वृत्त
filters-mask-feather = फ़ेदर
filters-mask-export-wipe = वाइप के रूप में निर्यात करें…
filters-mask-image = मास्क इमेज
filters-mask-mode = मोड
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = उलटें
filters-speed-x = गति X (px/s)
filters-speed-y = गति Y (px/s)
filters-tilt = झुकाव
filters-far-fade = दूर किनारे का फ़ेड
filters-fade-in-s = फ़ेड इन (से)
filters-visible-s = दृश्य (से)
filters-fade-out-s = फ़ेड आउट (से)
filters-hidden-s = छिपा (से)
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
recordings-trim = ट्रिम
recordings-trim-title = इस रिकॉर्डिंग से एक क्लिप काटें — कीफ़्रेम-संरेखित कट बिना री-एनकोड के निर्यात होते हैं
recordings-verify = जाँचें
recordings-verify-title = फ़ाइल की अखंडता जाँचें — कंटेनर संरचना, निरंतरता, A/V इंटरलीव, अवधि
recordings-verifying = जाँच हो रही है…
verify-dismiss = बंद करें
verify-verdict-pass = { $name } — अखंडता ठीक है
verify-verdict-warn = { $name } — चेतावनियों के साथ जाँचा गया
verify-verdict-fail = { $name } — समस्याएँ मिलीं
verify-container = कंटेनर
verify-video-continuity = वीडियो निरंतरता
verify-audio-continuity = ऑडियो निरंतरता
verify-av-interleave = A/V इंटरलीव
verify-duration = अवधि
recordings-alpha-label = अल्फ़ा
recordings-prores-title = अल्फ़ा सहेजने वाला ProRes 4444 .mov मास्टर निर्यात करें (संपादन हेतु)
recordings-qtrle-title = अल्फ़ा सहेजने वाला QuickTime Animation .mov निर्यात करें (अधिकतम संगतता)
trim-title = ट्रिम — { $name }
trim-loading = फ़ाइल पढ़ी जा रही है…
trim-preview-alt = पूर्वावलोकन फ़्रेम
trim-position = प्लेबैक स्थिति
trim-step-second-back = एक सेकंड पीछे
trim-step-frame-back = एक फ़्रेम पीछे
trim-step-frame-forward = एक फ़्रेम आगे
trim-step-second-forward = एक सेकंड आगे
trim-snap = कीफ़्रेम
trim-snap-title = निकटतम कीफ़्रेम पर स्नैप करें — वहाँ कट बिना री-एनकोड के निर्यात होता है
trim-set-in = इन-पॉइंट
trim-set-out = आउट-पॉइंट
trim-range-invalid = आउट-पॉइंट इन-पॉइंट के बाद होना चाहिए।
trim-copy-badge = ✓ बिना री-एनकोड निर्यात — इन-पॉइंट कीफ़्रेम पर है।
trim-reencode-badge = री-एनकोड होगा: इन-पॉइंट कीफ़्रेमों के बीच है (हानिरहित कट के लिए "कीफ़्रेम" से स्नैप करें)।
trim-export = क्लिप निर्यात करें
trim-export-916 = 9:16
trim-export-916-title = रीफ़्रेम किया वर्टिकल निर्यात (वर्टिकल कैनवास आकार पर केंद्रित क्रॉप) — हमेशा री-एनकोड होता है
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
recordings-normalize = सामान्यीकृत करें
recordings-normalizing = सामान्यीकृत किया जा रहा है…
recordings-normalize-title = लाउडनेस को लक्ष्य तक सामान्यीकृत करें (एक प्रति लिखता है)
recordings-normalized-to = { $path } में सामान्यीकृत किया गया

# --- Audio-only recording (CAP-N38) ---
audiorec-title = केवल ऑडियो
audiorec-format = ऑडियो रिकॉर्डिंग प्रारूप
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = ऑडियो रिकॉर्ड करें
audiorec-stop = रोकें
audiorec-pause = ठहराएं
audiorec-resume = फिर से शुरू करें
audiorec-recording = REC { $sec }s
audiorec-saved = { $count } ट्रैक फ़ाइल सहेजी गईं

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
output-recording-template = रिकॉर्डिंग फ़ाइलनाम
output-replay-template = रीप्ले फ़ाइलनाम
output-still-template = स्थिर फ़्रेम फ़ाइलनाम
output-template-tokens = टोकन: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = रीप्ले फ़ोल्डर
output-still-folder = स्थिर फ़्रेम फ़ोल्डर
output-same-folder-placeholder = रिकॉर्डिंग फ़ोल्डर
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
bench-open = एनकोडर बेंचमार्क चलाएँ…
bench-title = एनकोडर बेंचमार्क
bench-intro = इस मशीन पर छोटी मापी गई एन्कोड सीढ़ियाँ चलाता है (हर पहचाना एनकोडर × प्रीसेट × रिज़ॉल्यूशन) — लगभग एक मिनट, पूरी तरह ऑफ़लाइन, कुछ भी कंप्यूटर से बाहर नहीं जाता। विफलताएँ सूचीबद्ध होती हैं, छिपाई नहीं जातीं। पहले स्ट्रीम या रिकॉर्डिंग रोकें।
bench-start = बेंचमार्क शुरू करें
bench-rerun = फिर चलाएँ
bench-running = माप जारी… { $done } / { $total }
bench-cancel = रद्द करें
bench-col-encoder = एनकोडर
bench-col-preset = प्रीसेट
bench-col-rung = पायदान
bench-col-achieved = fps
bench-col-headroom = मार्जिन
bench-failed = विफल
bench-rec-title = सिफ़ारिश (मापी गई)
bench-rec-body = { $encoder }, { $preset } पर, { $width }×{ $height } @ { $fps } fps — मापा गया { $headroom }× रीयल-टाइम। सुझाया स्ट्रीम बिटरेट: { $bitrate } kbps।
bench-rec-none = इस मशीन पर कुछ भी रीयल-टाइम नहीं टिकता — कैनवास रिज़ॉल्यूशन या fps घटाकर फिर मापें।
bench-apply = रिकॉर्डिंग सेटिंग्स पर लागू करें
bench-applied = लागू ✓
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
output-iso-heading = ISO रिकॉर्डिंग
output-iso-explainer = चुने गए स्रोतों को साफ़ रिकॉर्ड करें, हर एक प्रोग्राम के साथ अपनी अलग फ़ाइल में — कंपोज़िट से पहले, कैनवास के आकार और फ़्रेम दर पर, ताकि हर फ़ाइल संपादन टाइमलाइन पर संरेखित होकर बैठे। मिड-रेंज GPU पर दो लेन सहज चलती हैं; हर अतिरिक्त लेन एक और रेंडर और एनकोड की लागत जोड़ती है।
output-iso-none = संग्रह में अभी कोई स्रोत नहीं है।
output-iso-source-on = "{ $name }" अपनी अलग ISO फ़ाइल में रिकॉर्ड हो रहा है — रोकने के लिए क्लिक करें
output-iso-source-off = "{ $name }" को अपनी अलग ISO फ़ाइल में रिकॉर्ड करें
output-iso-post-filter = स्रोत के फ़िल्टरों के साथ रिकॉर्ड करें (पोस्ट-फ़िल्टर); अनचेक करने पर कच्चा स्रोत रिकॉर्ड होगा
output-iso-format = ISO फ़ॉर्मैट
output-iso-encoder = ISO वीडियो एनकोडर
output-alpha-frec = पारदर्शिता (अल्फ़ा) के साथ रिकॉर्ड करें — पारदर्शी पृष्ठभूमि पर प्रोग्राम
output-auto-export = मास्टर के साथ एक साझा-योग्य MP4 भी सहेजें
output-auto-export-title = जब .frec रिकॉर्डिंग पूर्ण होती है, पृष्ठभूमि निर्यात उसके साथ डबल-क्लिक से चलने वाली MP4 प्रति लिखता है (पृष्ठभूमि कतार में दिखती है)। लॉसलेस मास्टर ही मास्टर रहता है।
output-alpha-title = रिकॉर्डर को अपना पारदर्शी रेंडर मिलता है; पूर्वावलोकन और स्ट्रीम सामान्य रहते हैं। रिकॉर्डिंग सूची से ProRes 4444 या QTRLE में निर्यात करें — MP4/MKV अल्फ़ा समतल कर देते हैं।
output-split-events = इन पर भी नई फ़ाइल शुरू करें… (हर भाग ठीक घटना पर शुरू होता है; न्यूनतम लंबाई 1 से.)
output-split-on-scene = दृश्य बदलने पर
output-split-on-marker = मार्कर पर
output-split-on-rundown = रनडाउन चरण पर
output-auto-markers = स्टूडियो घटनाओं पर स्वतः अध्याय मार्कर डालें (दृश्य बदलना, रीप्ले सहेजना, पुनः कनेक्शन, गिरे फ़्रेम, अलार्म, नियम)
output-auto-markers-title = टाइप किए मार्कर रिकॉर्डिंग के अध्यायों (mkv) या .chapters.txt साइडकार में, मैनुअल मार्कर हॉटकी के साथ दर्ज होते हैं
output-pipeline-heading = रिकॉर्डिंग-पश्चात पाइपलाइन
output-pipeline-explainer = रिकॉर्डिंग पूर्ण होने पर ये चरण मुख्य फ़ाइल पर क्रम से पृष्ठभूमि में चलते हैं। एक बंद क्रिया-समुच्चय — जान-बूझकर कोई "कमांड चलाएँ" चरण नहीं है। पहली विफलता पर शृंखला रुक जाती है।
output-pipeline-enabled = हर रिकॉर्डिंग के बाद पाइपलाइन चलाएँ
output-pipeline-add = चरण जोड़ें…
output-pipeline-up = ऊपर ले जाएँ
output-pipeline-down = नीचे ले जाएँ
output-pipeline-remove = चरण हटाएँ
output-pipeline-template = नाम बदलने का टेम्पलेट (CAP-M25 टोकन)
output-pipeline-folder = फ़ोल्डर
pipeline-queue = रिकॉर्डिंग-पश्चात पाइपलाइन
pipeline-verify = जाँचें
pipeline-remux = MP4 में रीमक्स करें
pipeline-normalize = लाउडनेस सामान्य करें
pipeline-rename = नाम बदलें
pipeline-move = फ़ोल्डर में ले जाएँ
pipeline-copy = फ़ोल्डर में कॉपी करें
pipeline-reveal = फ़ाइल प्रबंधक में दिखाएँ
pipeline-luaEvent = Lua स्क्रिप्ट्स को सूचित करें
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
stream-session-report = सत्र समाप्त होने पर रिकॉर्डिंग के पास सत्र रिपोर्ट (HTML + Markdown) लिखें
stream-simulator-title = नेटवर्क सिम्युलेटर (रिहर्सल)
stream-simulator-note = केवल रिहर्सल के लोकल सिंक को आकार देता है — रीकनेक्ट और कमज़ोर अपलिंक का अभ्यास। असली Go Live कभी प्रभावित नहीं होता।
stream-simulator-profile = प्रोफ़ाइल
stream-simulator-off = बंद
stream-simulator-hotel-wifi = होटल वाई-फ़ाई
stream-simulator-mobile-hotspot = मोबाइल हॉटस्पॉट
stream-simulator-custom = कस्टम
stream-simulator-bandwidth = बैंडविड्थ (kbps, 0 = असीमित)
stream-simulator-latency = विलंबता (ms)
stream-simulator-jitter = जिटर (± ms)
stream-simulator-outage-every = हर (से) में कटौती (0 = कभी नहीं)
stream-simulator-outage-len = कटौती की अवधि (से)
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
hotkeys-go-live = लाइव जाएँ / स्ट्रीम समाप्त करें
hotkeys-transition = स्टूडियो-मोड ट्रांज़िशन
hotkeys-save-replay = रीप्ले सहेजें (आखिरी N सेकंड)
hotkeys-add-marker = एक चैप्टर मार्कर डालें (रिकॉर्डिंग)
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

# --- OBS import (CAP-M02) ---
workspace-import-obs = OBS से आयात करें…
workspace-import-obs-hint = कोई OBS सीन संग्रह (उसकी scenes.json) लाएँ। आपका मौजूदा संग्रह पहले सहेजा जाता है।
workspace-import-busy = आयात हो रहा है…
workspace-pack-title = सीन-संग्रह पैक
workspace-pack-export = पैक निर्यात करें…
workspace-pack-exporting = निर्यात हो रहा है…
workspace-pack-import = पैक आयात करें…
workspace-pack-importing = आयात हो रहा है…
workspace-pack-hint = एक .fcappack सक्रिय संग्रह और उसकी स्थानीय एसेट को एक पोर्टेबल फ़ाइल में बंडल करता है — पूरा लेआउट साझा करें, कोई सर्वर नहीं। आयात इसे नए संग्रह के रूप में लाता है और उस पर स्विच कर देता है।
workspace-pack-exported = "{ $name }" निर्यात किया गया — { $bundled } एसेट बंडल की गईं, { $external } बाहरी।
workspace-pack-imported = "{ $name }" आयात किया गया — { $scenes } सीन, { $relinked } एसेट फिर से लिंक की गईं, { $external } बाहरी।
workspace-compare = तुलना करें और मर्ज करें…
compare-title = तुलना करें और मर्ज करें
compare-intro = सक्रिय कलेक्शन की दूसरे से तुलना करें और केवल वही बदलाव लाएँ जो आप चुनते हैं। कुछ भी इस मशीन से बाहर नहीं जाता।
compare-target-label = इससे तुलना करें
compare-target-none = कोई कलेक्शन चुनें…
compare-target-pack = किसी पैक फ़ाइल से…
compare-against = बनाम { $name }
compare-comparing = तुलना हो रही है…
compare-no-diff = कोई अंतर नहीं — कलेक्शन मेल खाते हैं।
compare-select-all = सभी चुनें
compare-clear = साफ़ करें
compare-merge-hint = कोई सीन चुनने पर उसके द्वारा उपयोग किए गए सोर्स भी आ जाते हैं।
compare-sources-title = सोर्स
compare-scenes-title = सीन
compare-kind-added = जोड़ा गया
compare-kind-removed = हटाया गया
compare-kind-modified = बदला गया
compare-renamed = पहले "{ $from }" था
compare-reordered = पुनः क्रमित
compare-apply = { $count } लागू करें
compare-apply-none = लागू करने के लिए बदलाव चुनें
compare-applying = लागू हो रहा है…
compare-aspect-source = सोर्स
compare-aspect-transform = स्थिति
compare-aspect-visibility = दृश्यता
compare-aspect-outputs = आउटपुट
compare-aspect-filters = फ़िल्टर
compare-aspect-lock = लॉक
compare-aspect-blend = ब्लेंड
compare-aspect-scaling = स्केलिंग
compare-aspect-backdrop = पृष्ठभूमि
compare-aspect-reveal = रिवील
compare-aspect-name = नाम
compare-aspect-settings = सेटिंग्स
compare-aspect-audio = ऑडियो
workspace-snapshots = स्नैपशॉट…
snapshots-title = स्नैपशॉट
snapshots-intro = इस कलेक्शन के नामित चेकपॉइंट जिन पर आप लौट सकते हैं — जानबूझकर रखे गए, स्वतः सहेजने और पूर्ववत से परे। स्थानीय रूप से संग्रहीत।
snapshots-name-placeholder = इस चेकपॉइंट को नाम दें…
snapshots-create = स्नैपशॉट सहेजें
snapshots-empty = अभी कोई स्नैपशॉट नहीं — एक सहेजें ताकि एक नामित चेकपॉइंट बना रहे जिस पर आप लौट सकें।
snapshots-usage = { $cap } में से { $count } रखे गए · { $size }
snapshots-scenes = { $scenes } सीन · { $sources } स्रोत
snapshots-restore = पुनर्स्थापित करें
snapshots-compare = तुलना करें
snapshots-delete = स्नैपशॉट हटाएँ
snapshots-delete-aria = स्नैपशॉट "{ $name }" हटाएँ
snapshots-restore-confirm = इस स्नैपशॉट को पुनर्स्थापित करें? आपका मौजूदा लेआउट पहले एक स्नैपशॉट के रूप में सहेजा जाता है।

# --- Phase 8 (backup / theme editor / dock presets / settings search / quick actions) ---
workspace-backup = बैकअप और पुनर्स्थापना…
backup-title = बैकअप और पुनर्स्थापना
backup-intro = अपने पूरे स्टूडियो — सेटिंग्स, प्रोफ़ाइल, सीन कलेक्शन — को एक फ़ाइल में सहेजें, और इसे यहाँ या किसी नई मशीन पर पुनर्स्थापित करें।
backup-export = बैकअप निर्यात करें…
backup-exporting = निर्यात हो रहा है…
backup-exported = { $collections } कलेक्शन और { $profiles } प्रोफ़ाइल का बैकअप लिया गया ({ $size })।
backup-restore = बैकअप से पुनर्स्थापित करें…
backup-secrets-note = स्ट्रीम कीज़ और पासवर्ड कभी बैकअप में शामिल नहीं होते — पुनर्स्थापना के बाद उन्हें फिर से दर्ज करें।
backup-restore-from = { $created } को बनाया गया बैकअप (v{ $version })
backup-restore-settings = सेटिंग्स
backup-restore-collections = सीन कलेक्शन ({ $count })
backup-restore-profiles = प्रोफ़ाइल ({ $count })
backup-restore-apply = चयनित पुनर्स्थापित करें
backup-restoring = पुनर्स्थापित हो रहा है…
backup-restored = पुनर्स्थापित किया गया।
backup-restart-note = पुनर्स्थापित कलेक्शन और प्रोफ़ाइल लोड करने के लिए Freally Capture को पुनः आरंभ करें।
backup-cancel = रद्द करें
theme-editor-title = कस्टम पैलेट
theme-color-bg = पृष्ठभूमि
theme-color-panel = पैनल
theme-color-text = टेक्स्ट
theme-color-muted = मंद
theme-color-accent = एक्सेंट
theme-color-accent2 = एक्सेंट 2
theme-contrast-text-bg = पृष्ठभूमि पर टेक्स्ट
theme-contrast-text-panel = पैनल पर टेक्स्ट
theme-contrast-muted-bg = पृष्ठभूमि पर मंद
theme-contrast-accent-bg = पृष्ठभूमि पर एक्सेंट
theme-contrast-low = (लक्ष्य ≥ { $min }:1)
theme-reset = डार्क पर रीसेट करें
theme-export = .fctheme निर्यात करें…
theme-import = .fctheme आयात करें…
workspace-dock-presets = डॉक लेआउट
workspace-dock-preset-placeholder = वर्तमान लेआउट को इस रूप में सहेजें…
workspace-dock-preset-save = सहेजें
workspace-dock-preset-apply = “{ $name }” पर स्विच करें
workspace-dock-preset-delete = लेआउट “{ $name }” हटाएँ
workspace-dock-presets-hint = एक लेआउट स्टैट्स डॉक और मिक्सर के अभिविन्यास को याद रखता है। स्विच करने के लिए किसी एक पर क्लिक करें।
settings-search-placeholder = सेटिंग्स खोजें…
settings-search-none = कोई सेटिंग मेल नहीं खाती।
settings-changed = खोलने के बाद बदला गया
menu-tools-quick-actions = त्वरित क्रियाएँ…
menu-tools-featured-chat = विशेष चैट संदेश…
quick-actions-title = त्वरित क्रियाएँ
quick-actions-empty = अभी कोई बटन नहीं — जोड़ने के लिए संपादित करें पर क्लिक करें।
quick-actions-edit = संपादित करें
quick-actions-done = पूर्ण
quick-actions-add-page = पेज
quick-actions-label = बटन लेबल
quick-actions-color = बटन का रंग
quick-actions-add = बटन जोड़ें
quick-actions-remove = “{ $name }” हटाएँ
quick-actions-delete-page = “{ $name }” हटाएँ
quick-actions-hint = बटन वही स्टूडियो कमांड चलाते हैं जो हॉटकी और LAN पैनल चलाते हैं — सीन, मैक्रो और साउंडबोर्ड पैड भी। कमांड बटन LAN पैनल पर भी दिखते हैं।
quick-actions-cmd-transition = ट्रांज़िशन
quick-actions-cmd-startStream = लाइव जाएँ
quick-actions-cmd-stopStream = स्ट्रीम समाप्त करें
quick-actions-cmd-startRecording = रिकॉर्डिंग शुरू करें
quick-actions-cmd-stopRecording = रिकॉर्डिंग रोकें
quick-actions-cmd-pauseRecording = रिकॉर्डिंग पॉज़ करें
quick-actions-cmd-addMarker = मार्कर जोड़ें
quick-actions-cmd-armReplay = रीप्ले सक्रिय करें
quick-actions-cmd-saveReplay = रीप्ले सहेजें
quick-actions-cmd-setStudioMode = स्टूडियो मोड टॉगल करें
quick-actions-cmd-scene = सीन पर स्विच करें…
quick-actions-cmd-macro = मैक्रो चलाएँ…
quick-actions-cmd-soundboard = साउंडबोर्ड पैड…
quick-actions-value-scene = सीन नाम
quick-actions-value-macro = मैक्रो नाम
quick-actions-value-soundboard = पैड आईडी
teleprompter-stop = रोकें
about-portable-on = पोर्टेबल मोड: चालू — सभी ऐप डेटा ऐप के बगल में संग्रहीत होता है।
about-portable-off = पोर्टेबल मोड: बंद। ऐप को पूरी तरह स्वयं-निहित रूप से चलाने के लिए ऐप के बगल में एक "{ $marker }" फ़ाइल जोड़ें (USB-स्टिक मोड)।
about-portable-data = डेटा फ़ोल्डर: { $path }
workspace-import-title = "{ $name }" आयात किया गया
workspace-import-summary = { $scenes } सीन · { $sources } स्रोत · { $items } आइटम
workspace-import-dismiss = बंद करें
workspace-import-clean = सब कुछ ठीक से आयात हो गया।
workspace-import-geometry-caveat = आकार और स्थिति OBS लेआउट से समायोजित की जाती हैं — हर सीन जाँचें और कैप्चर डिवाइस फिर से चुनें।
workspace-import-notes-title = टिप्पणियों के साथ आयात
workspace-import-skipped-title = आयात नहीं हुआ
import-note-needsReselect = डिवाइस/मॉनिटर/विंडो फिर से चुनें
import-note-gameCaptureAsWindow = गेम कैप्चर → विंडो कैप्चर
import-note-referencesFile = फ़ाइल पथ जाँचें
import-note-filterDropped = कुछ फ़िल्टर असमर्थित
import-note-geometryApproximated = स्थिति/आकार अनुमानित
import-skip-unsupportedKind = कोई समकक्ष स्रोत प्रकार नहीं
import-skip-group = समूह अभी समर्थित नहीं

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = अनुपलब्ध फ़ाइलें फिर से जोड़ें…
doctor-title = अनुपलब्ध फ़ाइलें
doctor-scanning = स्कैन हो रहा है…
doctor-all-good = सभी संदर्भित फ़ाइलें मौजूद हैं। फिर से जोड़ने के लिए कुछ नहीं।
doctor-intro = इस कंप्यूटर पर { $count } संदर्भित फ़ाइलें नहीं मिलीं। हर एक का नया स्थान बताएं — उसका उपयोग करने वाला हर सीन एक साथ ठीक हो जाएगा।
doctor-relinked = { $count } संदर्भ फिर से जोड़े गए।
doctor-uses = { $count }× उपयोग
doctor-locate = ढूँढें…
doctor-locate-folder = फ़ोल्डर में ढूँढें…
doctor-locate-folder-hint = एक फ़ोल्डर चुनें; हर अनुपलब्ध फ़ाइल नाम से मिलाकर फिर से जोड़ी जाती है।
doctor-kind-image = छवि
doctor-kind-media = मीडिया
doctor-kind-slideshow = स्लाइडशो
doctor-kind-font = फ़ॉन्ट
doctor-kind-lut = LUT
doctor-kind-mask = मास्क
history-relinkFiles = फ़ाइलें फिर से जोड़ें

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
studio-preview-stinger-matte-label = ट्रैक मैट
studio-preview-stinger-matte-title = ट्रैक-मैट स्टिंगर पारदर्शिता को कैसे पैक करता है: फ़िल और उसका मैट साथ-साथ (क्षैतिज) या ऊपर-नीचे (लंबवत)
studio-preview-stinger-duck-label = प्रोग्राम डक करें
studio-preview-stinger-duck-title = चलते समय स्टिंगर के अपने ऑडियो के नीचे प्रोग्राम ऑडियो को डक करें (0 = बंद)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = मूव (मॉर्फ)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = कोई नहीं
stinger-matte-horizontal = साथ-साथ
stinger-matte-vertical = ऊपर-नीचे

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
settings-open-about = परिचय…

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

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Freally Capture में आपका स्वागत है
wizard-welcome = दो आसान कदम: पहले देखें कि आपकी मशीन क्या कर सकती है, फिर एक सीन शुरू करें। इसमें लगभग तीस सेकंड लगते हैं, और बाद में आप सब कुछ बदल सकते हैं।
wizard-local-first = यहाँ कुछ भी आपके कंप्यूटर से बाहर नहीं जाता। Freally Capture में कोई अकाउंट नहीं, कोई टेलीमेट्री नहीं, और कोई क्लाउड नहीं है।
wizard-start = शुरू करें
wizard-skip = छोड़ें
wizard-hardware-title = आपकी मशीन क्या कर सकती है
wizard-probing = आपका ग्राफ़िक्स कार्ड और प्रोसेसर जाँचा जा रहा है…
wizard-encoder = एन्कोडर
wizard-canvas = कैनवास
wizard-bitrate = बिटरेट
wizard-probe-found = मिला: { $gpus } · { $cores } फ़िज़िकल कोर
wizard-no-gpu = कोई समर्पित GPU नहीं
wizard-apply = ये सेटिंग्स उपयोग करें
wizard-keep-current = जो मेरे पास है वही रखें
wizard-template-title = एक सीन से शुरू करें
wizard-template-screen = मेरी स्क्रीन कैप्चर करें
wizard-template-screen-note = आपके मुख्य मॉनिटर की एक डिस्प्ले कैप्चर जोड़ता है। शुरू करने की सबसे आम जगह।
wizard-template-empty = खाली शुरू करें
wizard-template-empty-note = एक खाली सीन। + बटन से सोर्स खुद जोड़ें।
wizard-done = आप तैयार हैं।
wizard-done-hint = सीन, सोर्स और क्रियाएँ खोजने के लिए किसी भी समय Ctrl+K दबाएँ। सेटिंग्स ⚙ बटन के पीछे रहती हैं।
wizard-close = स्ट्रीमिंग शुरू करें

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = आपका ग्राफ़िक्स कार्ड खुद वीडियो एन्कोड कर सकता है, जिससे प्रोसेसर बाकी स्टूडियो के लिए मुक्त रहता है।
autoconfig-reason-software = कोई उपयोगी हार्डवेयर एन्कोडर नहीं मिला, इसलिए प्रोसेसर एन्कोड करेगा। यह काम करता है, बस इसमें ज़्यादा CPU लगता है।
autoconfig-reason-quality-hardware = 60 फ़्रेम प्रति सेकंड पर 1080p, ऐसे बिटरेट पर जिसे हर बड़ा प्लेटफ़ॉर्म स्वीकार करता है।
autoconfig-reason-quality-software = 30 फ़्रेम प्रति सेकंड, क्योंकि 60 पर सॉफ़्टवेयर एन्कोडिंग अधिकांश प्रोसेसर पर फ़्रेम गिरा देती है।
autoconfig-reason-quality-low-cores = एक कम बिटरेट, क्योंकि इस प्रोसेसर में कम कोर हैं और सॉफ़्टवेयर एन्कोडिंग उनके लिए कम्पोज़िटर से होड़ करेगी।

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = रिकॉर्डिंग शुरू हो गई
announce-recording-paused = रिकॉर्डिंग रोकी गई
announce-recording-stopped = रिकॉर्डिंग बंद हो गई
announce-live-started = आप अब लाइव हैं
announce-live-ended = स्ट्रीम समाप्त हो गई
announce-reconnecting = कनेक्शन टूट गया, फिर से जुड़ रहे हैं
announce-stream-failed = स्ट्रीम विफल हो गई
announce-frames-dropped = { $count } फ़्रेम ड्रॉप हुए

# CAP-M01 — undo/redo edit history
palette-undo = पूर्ववत करें
palette-redo = फिर से करें
palette-edit-history = संपादन इतिहास…
history-title = संपादन इतिहास
history-empty = अभी पूर्ववत करने के लिए कुछ नहीं है।
history-current = वर्तमान स्थिति
history-close = बंद करें
history-addScene = दृश्य जोड़ें
history-renameScene = दृश्य का नाम बदलें
history-removeScene = दृश्य हटाएँ
history-reorderScene = दृश्य पुनः क्रमित करें
history-addSource = स्रोत जोड़ें
history-removeSource = स्रोत हटाएँ
history-reorderSource = स्रोत पुनः क्रमित करें
history-renameSource = स्रोत का नाम बदलें
history-transformSource = स्रोत ले जाएँ
history-toggleVisibility = दृश्यता टॉगल करें
history-toggleOutputVisibility = आउटपुट दृश्यता टॉगल करें
history-toggleLock = लॉक टॉगल करें
history-setBlendMode = ब्लेंड मोड बदलें
history-editSourceProperties = गुण संपादित करें
history-applyLayout = लेआउट व्यवस्थित करें
history-moveToSeat = स्थान पर ले जाएँ
history-groupSources = स्रोत समूहित करें
history-ungroupSources = समूह अलग करें
history-toggleGroupVisibility = समूह टॉगल करें
history-setSceneAudio = दृश्य ऑडियो
history-setVerticalCanvas = लंबवत कैनवास
history-addFilter = फ़िल्टर जोड़ें
history-removeFilter = फ़िल्टर हटाएँ
history-reorderFilter = फ़िल्टर पुनः क्रमित करें
history-editFilter = फ़िल्टर संपादित करें
history-toggleFilter = फ़िल्टर टॉगल करें
history-setVolume = वॉल्यूम समायोजित करें
history-toggleMute = म्यूट टॉगल करें
history-setMonitor = मॉनिटरिंग बदलें
history-setTracks = ट्रैक बदलें
history-setSyncOffset = A/V सिंक समायोजित करें
history-setAudioHotkeys = ऑडियो शॉर्टकट

# CAP-M04 — alignment aids
settings-alignment-section = संरेखण सहायक
settings-smart-guides = स्मार्ट गाइड (खींचते समय स्नैप)
settings-safe-areas = सुरक्षित-क्षेत्र ओवरले
settings-rulers = रूलर
align-group = कैनवास पर संरेखित करें
align-left = बाएँ संरेखित करें
align-hcenter = क्षैतिज केंद्र
align-right = दाएँ संरेखित करें
align-top = ऊपर संरेखित करें
align-vcenter = लंबवत केंद्र
align-bottom = नीचे संरेखित करें

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = चयन संरेखित करें और वितरित करें
arrange-left = बाएँ किनारे संरेखित करें
arrange-hcenter = क्षैतिज रूप से केंद्रित करें
arrange-right = दाएँ किनारे संरेखित करें
arrange-top = ऊपरी किनारे संरेखित करें
arrange-vcenter = लंबवत रूप से केंद्रित करें
arrange-bottom = निचले किनारे संरेखित करें
distribute-h = क्षैतिज रूप से वितरित करें
distribute-v = लंबवत रूप से वितरित करें
guides-group = गाइड
guides-add-v = लंबवत गाइड जोड़ें
guides-add-h = क्षैतिज गाइड जोड़ें
guides-clear = सभी गाइड हटाएँ
history-arrangeItems = आइटम व्यवस्थित करें
history-editGuides = गाइड संपादित करें

# CAP-M05 — edit transform + copy/paste
transform-title = ट्रांसफ़ॉर्म संपादित करें — { $name }
transform-anchor = एंकर
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = घुमाव
transform-crop = क्रॉप
transform-crop-left = बाएँ
transform-crop-top = ऊपर
transform-crop-right = दाएँ
transform-crop-bottom = नीचे
transform-no-size = स्रोत द्वारा अपने आयाम बताए जाने पर आकार और क्रॉप उपलब्ध होंगे।
transform-copy = ट्रांसफ़ॉर्म कॉपी करें
transform-paste = ट्रांसफ़ॉर्म पेस्ट करें
transform-close = बंद करें
filters-copy = फ़िल्टर कॉपी करें ({ $count })
filters-paste = फ़िल्टर पेस्ट करें ({ $count })
palette-edit-transform = ट्रांसफ़ॉर्म संपादित करें…
history-pasteFilters = फ़िल्टर पेस्ट करें

# CAP-M26 — keying workbench
workbench-title = कीइंग वर्कबेंच — { $name }
workbench-mode-keyed = कीड
workbench-mode-source = स्रोत
workbench-mode-matte = मैट
workbench-mode-split = विभाजित
workbench-eyedropper = आईड्रॉपर
workbench-eyedropper-hint = की रंग नमूना लेने के लिए स्रोत पर क्लिक करें।
workbench-loupe = लूप
workbench-split = विभाजन
workbench-preview-alt = कीइंग वर्कबेंच पूर्वावलोकन
workbench-tune = समायोजित करें
workbench-close = बंद करें

# CAP-M06 — multiview monitor
multiview-title = मल्टीव्यू
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = किसी दृश्य पर स्विच करने के लिए उस पर क्लिक करें।
multiview-hint-stage = किसी दृश्य को पूर्वावलोकन में तैयार करने के लिए उस पर क्लिक करें।
palette-multiview = मल्टीव्यू मॉनिटर

# CAP-M07 — projectors
projector-title = प्रोजेक्टर खोलें
projector-source = स्रोत
projector-target-program = प्रोग्राम
projector-target-preview = पूर्वावलोकन
projector-target-scene = सीन…
projector-target-source = स्रोत…
projector-target-multiview = मल्टीव्यू
projector-which-scene = कौन-सा सीन
projector-which-source = कौन-सा स्रोत
projector-none = दिखाने के लिए कुछ नहीं
projector-display = डिस्प्ले
projector-windowed = तैरती हुई विंडो (यह स्क्रीन)
projector-display-option = डिस्प्ले { $n } — { $w }×{ $h }
projector-primary = (प्राथमिक)
projector-open = खोलें
projector-cancel = रद्द करें
projector-exit-hint = बाहर निकलने के लिए Esc दबाएँ
palette-projector = प्रोजेक्टर खोलें…

# CAP-M08 — still-frame grab
palette-still = स्थिर फ़्रेम कैप्चर करें…
still-saved-toast = फ़्रेम सहेजा गया: { $name }
still-failed-toast = फ़्रेम कैप्चर विफल: { $error }
hotkeys-still = स्थिर फ़्रेम कैप्चर करें

# CAP-M13 — source health dashboard
palette-source-health = स्रोत स्वास्थ्य…
palette-av-sync = A/V सिंक कैलिब्रेशन…
palette-hotkey-audit = हॉटकी मैप…
palette-featured-chat = विशेष चैट संदेश…
featured-chat-title = विशेष चैट संदेश
featured-chat-intro = किसी भी हाल की चैट पंक्ति को प्रोग्राम में नीचे बैनर के रूप में पिन करें — स्ट्रीम और रिकॉर्डिंग के दर्शक इसे देखते हैं। पुनरारंभ पर पिन हट जाता है; रंग बने रहते हैं।
featured-chat-empty = अभी चैट नहीं — एक चैट ओवरले स्रोत जोड़ें और उसे अपने चैनल से जोड़ें; पंक्तियाँ आते ही यहाँ दिखेंगी।
featured-chat-pinned = प्रोग्राम पर:
featured-chat-pin = पिन करें
featured-chat-clear = हटाएँ
featured-chat-bg = बैनर रंग
featured-chat-text = पाठ रंग
featured-chat-note = बैनर नीचे-केंद्र में हर सीन के ऊपर रहता है — इसे हटाने के लिए साफ़ करें।
health-title = स्रोत स्वास्थ्य
health-col-source = स्रोत
health-col-state = स्थिति
health-col-resolution = रिज़ॉल्यूशन
health-col-fps = FPS
health-col-last-frame = अंतिम फ़्रेम
health-col-dropped = छोड़े गए
health-col-retries = पुनरारंभ
health-col-actions = क्रियाएँ
health-state-live = लाइव
health-state-waiting = प्रतीक्षारत
health-state-error = त्रुटि
health-state-inactive = निष्क्रिय
health-restart = पुनरारंभ करें
health-properties = गुण
health-empty = इस संग्रह में अभी कोई स्रोत नहीं है।
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture छोड़ें?
quit-body = अभी छोड़ने पर क्रम से निम्नलिखित सुरक्षित रूप से होगा:
quit-consequence-stream = लाइव स्ट्रीम समाप्त करें और सेवा से डिस्कनेक्ट करें।
quit-consequence-recording = रिकॉर्डिंग रोकें और उसकी फ़ाइलें अंतिम रूप दें।
quit-consequence-replay = रीप्ले बफ़र बंद करें — बिना सहेजा रीप्ले फ़ुटेज हट जाएगा।
quit-confirm = सुरक्षित रूप से बाहर निकलें
quit-quitting = बंद हो रहा है…
quit-cancel = रद्द करें

# CAP-M11 — crash-safe recording salvage
salvage-title = बाधित रिकॉर्डिंग पुनर्प्राप्त करें?
salvage-body = पिछला सत्र अप्रत्याशित रूप से समाप्त हो गया, जबकि ये रिकॉर्डिंग अभी लिखी जा रही थीं। मरम्मत मूल के बगल में चलने योग्य प्रति बनाती है — मूल फ़ाइल कभी नहीं बदली जाती।
salvage-repair = मरम्मत करें
salvage-repairing = मरम्मत हो रही है…
salvage-done = मरम्मत हो गई
salvage-repaired = मरम्मत हो गई → { $name }
salvage-failed = मरम्मत विफल: { $error }
salvage-dismiss = अभी नहीं

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = एन्कोडर विफलता — { $from } से { $to } पर स्विच किया गया। स्ट्रीम पुनः कनेक्ट होकर चालू है।
fallback-toast-recording = एन्कोडर विफलता — { $from } से { $to } पर स्विच किया गया। रिकॉर्डिंग नई फ़ाइल में जारी है।
fallback-note = एन्कोडर फ़ॉलबैक: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = प्रोग्राम ऑडियो मौन हो गया है
alarm-clipping = प्रोग्राम ऑडियो क्लिप हो रहा है
alarm-black = प्रोग्राम चित्र काला है
alarm-frozen = प्रोग्राम चित्र कुछ समय से नहीं बदला है
alarm-lowDisk = डिस्क स्थान: वर्तमान बिटरेट पर लगभग { $minutes } मिनट शेष
alarm-dismiss = अलार्म हटाएँ
alarm-cleared = हल हुआ: { $alarm }

# CAP-M22 — panic button
palette-panic = पैनिक — गोपनीयता स्लेट पर कट करें
panic-banner-title = पैनिक
panic-banner-body = प्रोग्राम गोपनीयता स्लेट दिखा रहा है; सारा ऑडियो म्यूट है और कैप्चर रुके हैं। स्ट्रीम और रिकॉर्डिंग चालू रहती हैं।
panic-restore = पुनर्स्थापित करें…
panic-restore-confirm = प्रोग्राम पुनर्स्थापित करें?
panic-restore-yes = पुनर्स्थापित करें
panic-restore-cancel = रद्द करें
hotkeys-panic = पैनिक (गोपनीयता स्लेट)
hotkeys-timer-toggle = सभी टाइमर शुरू/रोकें
hotkeys-timer-reset = सभी टाइमर रीसेट करें
panic-slate-color = पैनिक स्लेट रंग
panic-slate-image = पैनिक स्लेट छवि
panic-slate-image-placeholder = वैकल्पिक छवि पथ

# CAP-M24 — redacted diagnostics bundle
diag-title = निदान बंडल
diag-intro = GitHub इश्यू में हाथ से जोड़ने के लिए एक संपादित .zip निर्यात करें (कॉन्फ़िग स्नैपशॉट, एन्कोडर जांच, हाल के आँकड़े — रहस्य, पथ और नाम कभी शामिल नहीं)। कुछ भी कहीं नहीं भेजा जाता।
diag-preview = सामग्री देखें
diag-hide-preview = पूर्वावलोकन छिपाएँ
diag-export = .zip निर्यात करें
diag-exported = निर्यात किया गया: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = गो-लाइव पूर्व-जांच
preflight-intro = हर अवरोधक मद हरी होनी चाहिए; बाकी ईमानदार संकेत हैं।
preflight-item-targets = स्ट्रीम लक्ष्य कॉन्फ़िगर (कुंजी/URL सेट)
preflight-item-encoder = उपयोगी एन्कोडर उपलब्ध
preflight-item-sources = सभी स्रोत स्वस्थ
preflight-item-disk = रिकॉर्डिंग के लिए डिस्क स्थान
preflight-item-mic = माइक्रोफ़ोन मीटरिंग
preflight-item-desktopAudio = डेस्कटॉप ऑडियो मीटरिंग
preflight-item-replay = रीप्ले बफ़र सक्रिय
preflight-targets-detail = { $count } सक्षम
preflight-sources-detail = { $count } स्रोत त्रुटि में
preflight-disk-detail = वर्तमान बिटरेट पर ~{ $minutes } मिनट
preflight-fix-stream = स्ट्रीम सेटिंग्स…
preflight-fix-components = घटक…
preflight-fix-sources = स्रोत स्वास्थ्य…
preflight-fix-replay = सक्रिय करें
preflight-optional = वैकल्पिक
preflight-hold = सब हरा होने तक गो लाइव रोकें
preflight-cancel = रद्द करें
preflight-go-anyway = फिर भी लाइव जाएँ
preflight-go-live = लाइव जाएँ


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = पृष्ठभूमि
scenes-backdrop-aria = { $name } की पृष्ठभूमि
backdrop-title = पृष्ठभूमि — { $name }
backdrop-hint = इस दृश्य में हर चीज़ के पीछे टिका वॉलपेपर — कोई छवि, एनिमेटेड GIF या लूप में चलता वीडियो। आपका कैप्चर हमेशा ऊपर रहता है; ज़ूम के लिए कैनवास पर स्क्रॉल करें।
backdrop-choose = छवि या वीडियो चुनें…
backdrop-remove = पृष्ठभूमि हटाएँ
backdrop-none = कोई पृष्ठभूमि नहीं।
backdrop-position = स्थिति
backdrop-split-full = पूरा कैनवास
backdrop-split-fit = फ़िट
backdrop-split-left = बायाँ आधा
backdrop-split-right = दायाँ आधा
backdrop-split-top = ऊपरी आधा
backdrop-split-bottom = निचला आधा
backdrop-sync = रिकॉर्डिंग शुरू होते ही प्लेबैक शुरू करें
backdrop-sync-hint = रिकॉर्ड करने तक पहले फ़्रेम पर रुका रहता है; हर टेक में वीडियो शुरुआत से चलता है।
backdrop-preview-play = पूर्वावलोकन चलाएँ
backdrop-preview-pause = पूर्वावलोकन रोकें
backdrop-filter-all = पृष्ठभूमियाँ (छवियाँ और वीडियो)
backdrop-filter-images = छवियाँ
backdrop-filter-media = वीडियो और GIF
sources-backdrop-badge = पृष्ठभूमि वॉलपेपर (सबसे नीचे स्थिर)
sources-backdrop-pinned = पृष्ठभूमि सबसे नीचे स्थिर रहती है
filters-name-flip = पलटें
filters-flip-horizontal = क्षैतिज
filters-flip-vertical = ऊर्ध्वाधर
history-setSceneBackdrop = पृष्ठभूमि सेट करें
history-setBackdropSplit = पृष्ठभूमि खिसकाएँ
history-setBackdropSync = पृष्ठभूमि रिकॉर्डिंग सिंक
backdrop-scrub = प्लेबैक स्थिति
backdrop-loop = लूप
backdrop-reverse = उल्टा चलाएँ
backdrop-reverse-hint = रिवर्स एक बार उल्टी प्रति बनाता है (वीडियो के लिए ffmpeg कॉम्पोनेन्ट चाहिए; GIF तुरंत उल्टे चलते हैं) — लंबी फ़ाइलों में पहली बार समय लग सकता है।
filters-scaling = स्केलिंग
filters-scaling-hint = रेट्रो/पिक्सेल सामग्री के लिए पिक्सेल-परफ़ेक्ट मोड; पूर्णांक मोड बनाए गए आकार को पूर्ण गुणकों पर भी टिकाता है (हैंडल तार्किक आकार दिखाते हैं)।
filters-scaling-auto = स्मूद
filters-scaling-nearest = निकटतम पड़ोसी
filters-scaling-integer = पूर्णांक (पूर्ण ×)
filters-scaling-sharp = शार्प बाइलिनियर
history-setScaling = स्केलिंग बदलें
hotkeys-zoom-100 = ज़ूम: रीसेट (100%)
hotkeys-zoom-150 = ज़ूम: 150% तक बढ़ाएँ
hotkeys-zoom-200 = ज़ूम: 2× बढ़ाएँ
sources-follow-title = ज़ूम के दौरान कर्सर का पीछा करें (Windows; ज़ूम के लिए कैनवास पर स्क्रॉल करें)
sources-follow-item = { $name } के लिए कर्सर-फ़ॉलो टॉगल करें
filters-autocrop = ✂ काली पट्टियाँ स्वतः काटें
filters-autocrop-title = अगले फ़्रेम में लेटरबॉक्स/पिलरबॉक्स पट्टियाँ खोजकर काटता है (पूर्ववत किया जा सकता है)। अंधेरे दृश्य कभी नहीं कटते।
filters-autocrop-follow = रिज़ॉल्यूशन बदलने पर दोबारा जाँचें
history-autoCrop = काली पट्टियाँ स्वतः कटीं
sources-link-audio = इस ऐप की आवाज़ भी कैप्चर करें (लिंक्ड: छिपाने पर म्यूट, विंडो हटाने पर साथ हटेगा)
history-addLinkedWindow = विंडो + लिंक्ड ऑडियो जोड़ें
sources-hdr-title = यह डिस्प्ले HDR है — टोन-मैप खोलें (कैनवास SDR ही रहता है)
sources-hdr-item = { $name } का HDR टोन-मैप
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = यह डिस्प्ले HDR देता है। टोन-मैप के बिना हाइलाइट कट जाते हैं और SDR कैनवास पर कैप्चर फीका दिखता है। बदलाव अगले फ़्रेम से लागू होते हैं।
sources-hdr-enable-suggested = सुझाया चालू करें (maxRGB, 200 निट)
sources-hdr-operator = ऑपरेटर
sources-hdr-op-clip = क्लिप (बंद)
sources-hdr-op-maxrgb = maxRGB (रंगत बनाए रखे)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 घुटना (SDR सटीक)
sources-hdr-paper-white = पेपर व्हाइट
sources-hdr-nits = निट
projector-target-passthrough = पासथ्रू मॉनिटर (कम विलंब)
projector-which-device = डिवाइस
projector-passthrough-none = पहले कोई डिस्प्ले, विंडो या कैप्चर डिवाइस जोड़ें।
projector-passthrough-about = डिवाइस के कच्चे फ़्रेम — कोई सीन नहीं, कोई फ़िल्टर नहीं, कोई कंपोज़िटर नहीं। मापी गई विलंबता दिखाता है; ऑडियो अब भी मिक्सर स्ट्रिप से मॉनिटर होता है।
projector-passthrough-hint = पासथ्रू — Esc से बंद
projector-latency = { $ms } ms
projector-latency-measuring = माप रहे हैं…
automation-title = स्वचालन — नियम, मैक्रो और वेरिएबल
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = नियम
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = चालू
automation-rule-name = Rule name
automation-remove = Remove
automation-when = जब
automation-then-run = तब चलाएँ
automation-no-macro = (no macro)
automation-macros = मैक्रो
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = चलाएँ
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = स्टूडियो वेरिएबल
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
rundown-title = शो रनडाउन
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = शुरू
rundown-next = अगला ▸
rundown-stop = रोकें
rundown-idle = नहीं चल रहा
rundown-next-up = अगला: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + चरण
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
automation-layer = लेयर
automation-layer-hint = यह लेयर सक्रिय होने पर ही चलता है (खाली = सभी लेयर)। लेयर स्टिकी हैं: लेयर कुंजी बदलकर वहीं रहती है (OS ग्लोबल हॉटकी API में होल्ड-लेयर संभव नहीं)।
automation-chord-hint = सादा कुंजी (Ctrl+Shift+M) या दो-स्ट्रोक कॉर्ड (Ctrl+K, 3)। कॉर्ड की दूसरी कुंजी केवल कॉर्ड लंबित रहने तक आरक्षित होती है।
panel-title = LAN पैनल और टैली
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = पैनल चालू करें
panel-port = पोर्ट
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = पासवर्ड
panel-show = दिखाएँ
panel-hide = छिपाएँ
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = सहेजें
osc-title = OSC कंट्रोल सरफ़ेस
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = OSC सुनें
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ कैमरे
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = कैमरा
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = पता
ptz-port = पोर्ट
ptz-speed = गति
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
ptz-presets = प्रीसेट
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = बुलाएँ
ptz-store = सहेजें
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI कंट्रोल सरफ़ेस
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = इनपुट
midi-output = आउटपुट (फ़ीडबैक)
midi-none = (none)
midi-learn = लर्न
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = क्रिया
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
panel-lan-warning = ⚠ LAN ट्रैफ़िक एन्क्रिप्टेड नहीं है — पासवर्ड URL में सादे HTTP से जाता है। केवल भरोसेमंद नेटवर्क पर उपयोग करें।
osc-lan-warning = ⚠ OSC में पासवर्ड नहीं है — नेटवर्क का कोई भी डिवाइस ये कमांड भेज सकता है। LAN मोड केवल भरोसेमंद नेटवर्क पर।

# System-stats HUD source (CAP-N14)
sources-badge-stats = आँकड़े
sources-add-system-stats = प्रदर्शन आँकड़े (HUD)
sources-stats-title = प्रदर्शन HUD जोड़ें
sources-stats-note = आपके दर्शकों के लिए प्रोग्राम में स्टूडियो के असली मापे गए आँकड़े दिखाता है — fps, CPU, मेमोरी, रेंडर समय, गिरे हुए फ़्रेम और लाइव बिटरेट। कौन-सी पंक्तियाँ दिखें, आकार और रंग स्रोत की Properties में हैं। GPU उपयोग नहीं दिखाया जाता क्योंकि उसे मापा नहीं जाता।
sources-stats-add = आँकड़े HUD जोड़ें
properties-stats-show-fps = FPS दिखाएँ
properties-stats-show-cpu = CPU दिखाएँ
properties-stats-show-memory = मेमोरी दिखाएँ
properties-stats-show-render = रेंडर समय दिखाएँ
properties-stats-show-dropped = गिरे फ़्रेम दिखाएँ
properties-stats-show-bitrate = बिटरेट दिखाएँ
properties-stats-show-timecode = टाइमकोड दिखाएँ (LTC)
properties-stats-size = आकार (px)
properties-stats-note = HUD संक्षिप्त सार्वभौमिक लेबल (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) सीधे प्रोग्राम पर बनाता है; जब कोई स्ट्रीम नहीं चल रही हो तो बिटरेट पंक्ति “—” दिखाती है।

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = विज़ुअलाइज़र
sources-add-visualizer = ऑडियो विज़ुअलाइज़र
sources-visualizer-title = ऑडियो विज़ुअलाइज़र जोड़ें
sources-visualizer-style-label = शैली
sources-visualizer-style-bars = स्पेक्ट्रम बार
sources-visualizer-style-scope = ऑसिलोस्कोप
sources-visualizer-style-vu = VU मीटर
sources-visualizer-target-label = सुनता है
sources-visualizer-target-master = मास्टर मिक्स
sources-visualizer-target-track = ट्रैक { $n }
sources-visualizer-note = वही सिग्नल बनाता है जो वास्तव में मिक्स होता है (पोस्ट-फ़ेडर) — म्यूट किया स्रोत सपाट दिखता है, ठीक वैसा जैसा वह सुनाई देता है। आकार, रंग, बार संख्या और गिरावट दर स्रोत की Properties में हैं।
sources-visualizer-classic = क्लासिक रंग
sources-visualizer-add = विज़ुअलाइज़र जोड़ें
properties-vis-bands = बार
properties-vis-decay = गिरावट दर (dB/s)
properties-vis-peak-hold = पीक-होल्ड चिह्न
properties-vis-missing-source = (स्रोत अनुपलब्ध)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = स्प्लिट्स
sources-add-split-timer = स्पीडरन स्प्लिट टाइमर
sources-splits-title = स्प्लिट टाइमर जोड़ें
sources-splits-file-label = LiveSplit .lss फ़ाइल
sources-splits-comparison-label = तुलना करें
sources-splits-comparison-pb = निजी सर्वश्रेष्ठ
sources-splits-comparison-best = सर्वश्रेष्ठ सेगमेंट
sources-splits-comparison-average = औसत
sources-splits-note = फ़ाइल केवल पढ़ने के लिए आयात होती है — उसमें कभी कुछ नहीं लिखा जाता। Settings → Hotkeys में वैश्विक Split / Undo / Skip / Reset कुंजियाँ बाँधें। प्रोसेस-मेमोरी ऑटो-स्प्लिटर जान-बूझकर समर्थित नहीं हैं।
sources-splits-add = स्प्लिट टाइमर जोड़ें
properties-splits-size = आकार (px)
properties-splits-ahead = आगे
properties-splits-behind = पीछे
properties-splits-gold = गोल्ड
properties-splits-split = स्प्लिट
properties-splits-undo = पूर्ववत
properties-splits-skip = छोड़ें
properties-splits-reset = रीसेट
properties-splits-note = बटन लाइव टाइमर चलाते हैं (वैश्विक हॉटकी किसी भी ऐप से यही करती हैं)। रन कभी .lss फ़ाइल में सहेजा नहीं जाता।
hotkeys-split-split = स्प्लिट टाइमर: शुरू / स्प्लिट
hotkeys-split-undo = स्प्लिट टाइमर: स्प्लिट पूर्ववत
hotkeys-split-skip = स्प्लिट टाइमर: सेगमेंट छोड़ें
hotkeys-split-reset = स्प्लिट टाइमर: रीसेट
hotkey-audit-action-split-split = स्प्लिट (स्प्लिट टाइमर)
hotkey-audit-action-split-undo = स्प्लिट पूर्ववत
hotkey-audit-action-split-skip = सेगमेंट छोड़ें
hotkey-audit-action-split-reset = स्प्लिट टाइमर रीसेट
hotkey-audit-feature-split-timer = स्प्लिट टाइमर

# Media playlist source (CAP-N17)
sources-badge-playlist = प्लेलिस्ट
sources-add-playlist = मीडिया प्लेलिस्ट (बिना अंतराल)
sources-playlist-title = मीडिया प्लेलिस्ट जोड़ें
sources-playlist-files-label = फ़ाइलें (प्रति पंक्ति एक, ऊपर से नीचे चलती हैं)
sources-playlist-browse = ब्राउज़ करें…
sources-playlist-loop = लूप
sources-playlist-shuffle = शफ़ल (हर शुरुआत पर एक ड्रॉ; लूप में वही क्रम दोहराता है)
sources-playlist-hold-last = अंत में आख़िरी फ़्रेम रोके रखें
sources-playlist-note = पूरी ट्रिम की गई सूची को लेबल किए गए ffmpeg कॉम्पोनेन्ट से बिना अंतराल चलाता है (केवल wire फ़ॉर्मैट — .frec और चित्र Media/Slideshow से)। आइटम सब वीडियो या सब ऑडियो होते हैं, कभी मिश्रित नहीं। ट्रिम, क्यू पॉइंट और «now playing» वेरिएबल Properties में हैं।
sources-playlist-add = प्लेलिस्ट जोड़ें
properties-playlist-items = आइटम (ऊपर से नीचे)
properties-playlist-up = ऊपर करें
properties-playlist-down = नीचे करें
properties-playlist-remove = आइटम हटाएँ
properties-playlist-in = से (से.)
properties-playlist-out = तक (से.)
properties-playlist-cues = क्यू (से., अल्पविराम से अलग)
properties-playlist-add-item = + आइटम जोड़ें
properties-playlist-loop = लूप
properties-playlist-shuffle = शफ़ल
properties-playlist-hold-last = आख़िरी फ़्रेम रोकें
properties-playlist-hw = हार्डवेयर डिकोड
properties-playlist-variable = «Now playing» वेरिएबल (खाली = बंद)
properties-playlist-previous = ⏮ पिछला
properties-playlist-next = ⏭ अगला
properties-playlist-note = क्यू बटन और अगला/पिछला चल रही प्लेलिस्ट को चलाते हैं; आइटम बदलाव Apply पर लागू होते हैं (प्लेलिस्ट फिर शुरू होती है)। चल रहे आइटम का नाम दिखाने के लिए किसी Text स्रोत में {"{{"}yourVariable{"}}"} रखें।
hotkeys-playlist-next = प्लेलिस्ट: अगला आइटम
hotkeys-playlist-previous = प्लेलिस्ट: पिछला आइटम
hotkey-audit-action-playlist-next = प्लेलिस्ट अगला
hotkey-audit-action-playlist-previous = प्लेलिस्ट पिछला
hotkey-audit-feature-playlist = प्लेलिस्ट

# Instant replay source (CAP-N10)
sources-badge-replay = रीप्ले
sources-add-replay = इंस्टेंट रीप्ले
sources-replay-title = इंस्टेंट रीप्ले जोड़ें
sources-replay-seconds-label = रोल लंबाई (सेकंड)
sources-replay-speed-label = गति
sources-replay-speed-full = 100% (ऑडियो सहित)
sources-replay-speed-half = 50% स्लो-मो (मूक)
sources-replay-speed-quarter = 25% स्लो-मो (मूक)
sources-replay-note = रोल करने तक पारदर्शी रहता है। रीप्ले बफ़र आर्म करें (कंट्रोल्स) और रोल हॉटकी बाँधें — रोल बफ़र के आख़िरी पल लेकर प्रोग्राम में चलाता है, फिर पारदर्शी हो जाता है।
sources-replay-add = इंस्टेंट रीप्ले जोड़ें
properties-replay-roll = ⏵ रीप्ले रोल करें
properties-replay-note = रोल आर्म्ड बफ़र को क्लिप में लेकर चुनी गति पर चलाता है — री-टाइम्ड, कभी इंटरपोलेटेड नहीं। स्लो-मो जान-बूझकर मूक है। चलते समय स्क्रब/पॉज़ काम करते हैं; अंत में स्रोत फिर पारदर्शी हो जाता है।
hotkeys-replay-roll = इंस्टेंट रीप्ले: रोल
hotkey-audit-action-replay-roll = इंस्टेंट रीप्ले रोल

# Input overlay source (CAP-N13)
sources-badge-input = इनपुट
sources-add-input-overlay = इनपुट ओवरले (कुंजियाँ/गेमपैड)
sources-input-title = इनपुट ओवरले जोड़ें
sources-input-layout-label = लेआउट
sources-input-layout-wasd = WASD + माउस
sources-input-layout-keyboard = कॉम्पैक्ट कीबोर्ड + माउस
sources-input-layout-gamepad = गेमपैड (दो स्टिक)
sources-input-layout-fightstick = फ़ाइट स्टिक
sources-input-color-label = कुंजियाँ
sources-input-accent-label = दबाई गई
sources-input-privacy-note = गोपनीयता: इनपुट केवल तभी पढ़ा जाता है जब यह स्रोत किसी दृश्य में लाइव हो, और केवल लेआउट की तय कुंजियाँ ही पूछी जाती हैं — बस एक क्षणिक «क्या यह अभी दबी है?» झलक, कभी कोई हुक नहीं। कुछ भी लॉग, संग्रहीत या कहीं भेजा नहीं जाता; टाइप किया गया टेक्स्ट कभी कैप्चर नहीं होता।
sources-input-os-note = कीबोर्ड और माउस की स्थिति आज केवल Windows पर पढ़ी जाती है — अन्य सिस्टम कुंजियों को बिना दबा दिखाते हैं (ईमानदारी से कहा गया, कभी नकली नहीं)। गेमपैड gilrs लाइब्रेरी के ज़रिए हर जगह काम करते हैं; पहला जुड़ा कंट्रोलर दिखाया जाता है, और कोई न होने पर लेआउट बिना दबा रहता है।
sources-input-add = इनपुट ओवरले जोड़ें

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = कर्सर प्रभाव
filters-cursorfx-hint = Windows पर (जहाँ ऐप कर्सर खुद बनाता है) ये सीधे कैप्चर में ही बनाए जाते हैं, इसलिए रिकॉर्डिंग और स्ट्रीम में दिखते हैं। macOS और Linux कर्सर को सिस्टम स्तर पर जोड़ते हैं, इसलिए ये प्रभाव केवल Windows के लिए हैं। बदलाव तुरंत लागू होते हैं।
filters-cursorfx-halo = कर्सर हेलो
filters-cursorfx-halo-color = रंग
filters-cursorfx-halo-radius = त्रिज्या (px)
filters-cursorfx-ripples = क्लिक तरंगें
filters-cursorfx-left-color = बायाँ क्लिक
filters-cursorfx-right-color = दायाँ क्लिक
filters-cursorfx-keystrokes = कुंजी संकेत
filters-cursorfx-keystrokes-hint = दबाए रहने तक कुंजियों का एक निश्चित सेट (अक्षर, अंक, मॉडिफ़ायर, तीर) कर्सर के पास दिखाता है। कुंजियाँ केवल इसे चालू रखने पर पढ़ी जाती हैं, सीधे फ़्रेम में बनाई जाती हैं, और कभी सहेजी या लॉग नहीं होतीं।

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = टाइटल
sources-add-title = टाइटल / स्कोरबोर्ड
sources-title-title = टाइटल जोड़ें
sources-title-template-label = इससे शुरू करें
sources-title-template-lower-third = लोअर थर्ड (पट्टी + नाम + उपशीर्षक)
sources-title-template-scoreboard = स्कोरबोर्ड (प्लेट + 4 सेल)
sources-title-template-blank = खाली कैनवास
sources-title-width-label = कैनवास चौड़ाई
sources-title-height-label = कैनवास ऊँचाई
sources-title-template-name = नाम
sources-title-template-subtitle = पद
sources-title-template-home = होम
sources-title-template-away = अवे
sources-title-note = परत-दर-परत टाइटल (टेक्स्ट / छवि / बॉक्स) प्रवेश/निकास एनीमेशन के साथ, स्थानीय रूप से संयोजित — कोई ब्राउज़र स्रोत नहीं। परतें, फ़ाइल बाइंडिंग और {"{{"}चर{"}}"} तथा लाइव नियंत्रण स्रोत की Properties में हैं।
sources-title-add = टाइटल जोड़ें
properties-title-layers = परतें (क्रम से बनती हैं — बाद की पंक्तियाँ ऊपर)
properties-title-kind-text = टेक्स्ट
properties-title-kind-image = छवि
properties-title-kind-rect = बॉक्स
properties-title-x = X
properties-title-y = Y
properties-title-outline = आउटलाइन (px)
properties-title-outline-color = आउटलाइन
properties-title-shadow = छाया
properties-title-animation = प्रवेश/निकास एनीमेशन
properties-title-anim-none = कोई नहीं (कट)
properties-title-anim-fade = फ़ेड
properties-title-anim-slide-left = बाएँ स्लाइड
properties-title-anim-slide-up = ऊपर स्लाइड
properties-title-anim-wipe = वाइप
properties-title-duration = अवधि (ms)
properties-title-fire-in = ▶ प्रवेश चलाएँ
properties-title-fire-out = ◼ निकास चलाएँ
properties-title-set-live = लाइव सेट करें
properties-title-set-live-note = यह टेक्स्ट अभी लाइव टाइटल में भेजता है — बिना Apply, बिना रीस्टार्ट
properties-title-up = परत ऊपर ले जाएँ
properties-title-down = परत नीचे ले जाएँ
properties-title-remove = परत हटाएँ
properties-title-add-text = + टेक्स्ट
properties-title-add-image = + छवि
properties-title-add-rect = + बॉक्स
properties-title-note = प्रवेश/निकास और "लाइव सेट करें" चल रहे टाइटल को चलाते हैं; परत बदलाव Apply पर लागू होते हैं (टाइटल फिर शुरू होकर दोबारा प्रवेश करता है)। टेक्स्ट सेल किसी निगरानी वाली फ़ाइल से जुड़ सकते हैं (CSV सेल / JSON मान / पूरी फ़ाइल) और {"{{"}चर{"}}"} भरते हैं — "लाइव सेट करें" दोनों पर भारी है।

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN इनजेस्ट (SRT/RTMP लिसनर)
sources-lan-title = LAN इनजेस्ट लिसनर जोड़ें
sources-lan-protocol-label = प्रोटोकॉल
sources-lan-protocol-srt = SRT (एन्क्रिप्ट हो सकता है — अनुशंसित)
sources-lan-protocol-rtmp = RTMP (कोई प्रमाणीकरण नहीं)
sources-lan-port-label = पोर्ट (1024–65535)
sources-lan-passphrase-label = पासफ़्रेज़ (खाली = खुला)
sources-lan-passphrase-hint = SRT पासफ़्रेज़ 10–79 अक्षरों के होते हैं; भेजने वाले को वही इस्तेमाल करना होगा।
sources-lan-open-warning = कोई पासफ़्रेज़ नहीं: इस नेटवर्क पर कोई भी बिना एन्क्रिप्शन इस स्रोत में भेज सकता है। जब तक नेटवर्क सिर्फ़ आपका न हो, एक सेट करें।
sources-lan-rtmp-warning = RTMP में प्रमाणीकरण नहीं है — इस नेटवर्क पर कोई भी इस पोर्ट पर भेज सकता है। पासफ़्रेज़ के साथ SRT चुनें।
sources-lan-url-label = भेजने वाले का ऐप इस पते पर लगाएँ
sources-lan-qr-aria = इनजेस्ट URL का QR कोड
sources-lan-note = केवल LAN: इस मशीन के लोकल पते पर, सिर्फ़ तब तक सुनता है जब तक स्रोत मौजूद है, और इंटरनेट को कभी नहीं छूता — जब तक आपके नेटवर्क का कोई भेजने वाला पहले न भेजे, मशीन से कुछ बाहर नहीं जाता। डिकोडिंग साफ़ लेबल वाले ffmpeg कॉम्पोनेन्ट से होती है। भेजने वाले के जुड़ने तक कैनवास यही URL दिखाता है।
sources-lan-add = सुनना शुरू करें
properties-lan-note = प्रोटोकॉल, पोर्ट या पासफ़्रेज़ बदलाव लागू करने पर लिसनर फिर से शुरू होता है — भेजने वाले को दोबारा जुड़ना होगा। स्ट्रीम 1920×1080 कैनवास में बैठाई जाती है।

# Freally Link source & output (CAP-N12)
sources-badge-link = लिंक
sources-add-freally-link = Freally Link (दूसरा इंस्टेंस)
sources-link-title = Freally Link जोड़ें
sources-link-about = आपके अपने नेटवर्क पर दूसरे Freally Capture इंस्टेंस का प्रोग्राम — वीडियो और मास्टर ऑडियो — प्राप्त करता है। पहले भेजने वाले इंस्टेंस पर "Freally Link आउटपुट" चालू करें। v1 TCP पर motion-JPEG भेजता है: वायर्ड LAN या अच्छे Wi-Fi पर बढ़िया, कमज़ोर कनेक्शनों पर बैंडविड्थ के बारे में ईमानदार।
sources-link-scan = LAN स्कैन करें
sources-link-scanning = स्कैन हो रहा है…
sources-link-none = कोई Freally Link आउटपुट नहीं मिला। दूसरे इंस्टेंस पर "Freally Link आउटपुट" चालू करें (कंट्रोल्स → LAN पैनल) या नीचे उसका पता लिखें।
sources-link-host = पता
sources-link-port = पोर्ट
sources-link-key = पेयरिंग कुंजी
sources-link-key-hint = भेजने वाले की "Freally Link आउटपुट" सेटिंग्स की कुंजी — इसके बिना भेजने वाला एक भी फ़्रेम नहीं भेजता।
sources-link-add = लिंक जोड़ें
properties-link-note = कनेक्शन न होने पर स्रोत "कनेक्ट हो रहा है" चेहरा दिखाता है और बढ़ते अंतराल के साथ खुद दोबारा कोशिश करता है — यह कभी पुराने फ़्रेम पर नहीं जमता। प्रति भेजने वाला एक ही रिसीवर; व्यस्त भेजने वाले को विनम्रता से दोबारा आज़माया जाता है।
link-title = Freally Link आउटपुट
link-about = इस इंस्टेंस का प्रोग्राम — वीडियो और मास्टर ऑडियो — अपने नेटवर्क पर सिर्फ़ एक और Freally Capture के साथ साझा करें; वहाँ यह "Freally Link" स्रोत के रूप में दिखता है (दो-PC स्ट्रीमिंग, अतिरिक्त मॉनिटर)। डिफ़ॉल्ट रूप से बंद; चालू करने तक कुछ भी घोषित या सुनता नहीं। v1 TCP पर motion-JPEG + असम्पीड़ित ऑडियो भेजता है — वायर्ड LAN या अच्छे Wi-Fi के लिए, इंटरनेट के लिए कभी नहीं।
link-enable = मेरे नेटवर्क पर प्रोग्राम साझा करें
link-name = इंस्टेंस का नाम
link-key = पेयरिंग कुंजी
link-key-hint = कम से कम 8 अक्षर — रिसीवर को यह कुंजी दर्ज करनी होगी, तभी कोई फ़्रेम भेजा जाएगा।
link-lan-warning = ⚠ कुछ भी भेजे जाने से पहले रिसीवर को पेयरिंग कुंजी प्रस्तुत करनी होगी, लेकिन v1 में स्ट्रीम खुद एन्क्रिप्टेड नहीं है — इसे सिर्फ़ भरोसेमंद नेटवर्क पर इस्तेमाल करें।
link-serving = रिसीवर इस इंस्टेंस को "LAN स्कैन करें" से खोज सकते हैं या इसे मैन्युअल रूप से यहाँ जोड़ सकते हैं:
link-off-hint = पोर्ट खोलने और LAN स्कैन में इस इंस्टेंस की घोषणा करने के लिए साझा करना चालू करें।

# In-app menu bar (OBS-style chrome)
menu-bar-label = एप्लिकेशन मेनू
menu-file = फ़ाइल
menu-edit = संपादन
menu-view = दृश्य
menu-docks = डॉक
menu-profile = प्रोफ़ाइल
menu-collection = सीन कलेक्शन
menu-tools = उपकरण
menu-help = सहायता
menu-rename = नाम बदलें
menu-remove = हटाएँ
menu-import = आयात करें
menu-export = निर्यात करें
menu-file-show-recordings = रिकॉर्डिंग दिखाएँ
menu-file-remux = MP4 में Remux करें…
menu-file-settings = सेटिंग्स…
menu-file-show-settings-folder = सेटिंग्स फ़ोल्डर दिखाएँ
menu-file-exit = बाहर निकलें
menu-edit-undo = पूर्ववत करें
menu-edit-redo = फिर से करें
menu-edit-history = संपादन इतिहास…
menu-edit-copy-transform = ट्रांसफ़ॉर्म कॉपी करें
menu-edit-paste-transform = ट्रांसफ़ॉर्म पेस्ट करें
menu-edit-copy-filters = फ़िल्टर कॉपी करें
menu-edit-paste-filters = फ़िल्टर पेस्ट करें
menu-edit-transform = ट्रांसफ़ॉर्म…
menu-edit-lock-preview = पूर्वावलोकन लॉक करें
menu-view-fullscreen = फ़ुलस्क्रीन इंटरफ़ेस
menu-stats-dock = आँकड़े डॉक
menu-view-multiview = मल्टीव्यू मॉनिटर…
menu-view-projectors = प्रोजेक्टर…
menu-view-source-health = स्रोत स्वास्थ्य…
menu-view-still = स्थिर फ़्रेम कैप्चर करें
menu-docks-browser = ब्राउज़र डॉक…
menu-docks-lock = डॉक लॉक करें
menu-docks-reset = डॉक लेआउट रीसेट करें
menu-profile-manage = प्रोफ़ाइल प्रबंधित करें…
menu-collection-manage = सीन कलेक्शन प्रबंधित करें…
menu-collection-import-obs = OBS से आयात करें…
menu-collection-missing = अनुपलब्ध फ़ाइलें जाँचें…
menu-collection-clear = इतिहास साफ़ करें…
menu-collection-clear-confirm = इन दृश्य संग्रहों और उनकी फ़ाइलों को स्थायी रूप से हटाएँ? { $names } — सक्रिय संग्रह "{ $active }" रखा जाएगा। इसे पूर्ववत नहीं किया जा सकता।
menu-tools-wizard = सेटअप विज़ार्ड चलाएँ
menu-tools-wizard-title = सेटअप विज़ार्ड पहली बार चलाने पर चलता है; इसे दोबारा चलाने का तरीका अभी उपलब्ध नहीं है।
menu-tools-automation = स्वचालन नियम और मैक्रो…
menu-tools-rundown = शो रनडाउन दिखाएँ…
menu-tools-hotkeys = हॉटकी मैप…
menu-tools-av-sync = A/V सिंक कैलिब्रेशन…
menu-tools-scripts = Lua स्क्रिप्ट…
menu-tools-components = कंपोनेंट…
menu-tools-midi = MIDI नियंत्रण…
menu-tools-ptz = PTZ कैमरे…
menu-tools-remote = रिमोट कंट्रोल API…
menu-tools-panel = LAN पैनल और टैली…
menu-help-portal = सहायता पोर्टल
menu-help-website = वेबसाइट देखें
menu-help-discord = Discord सर्वर से जुड़ें
menu-help-bug = एक बग रिपोर्ट करें…
menu-help-updates = अपडेट जाँचें…
menu-help-whats-new = नया क्या है
menu-help-about = ऐप के बारे में…
menu-help-more-apps = और Freally ऐप्स…
moreapps-title = और Freally ऐप्स

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = सेटिंग्स श्रेणियाँ
settings-cat-general = सामान्य
settings-cat-appearance = दिखावट
settings-cat-streaming = स्ट्रीमिंग
settings-cat-output = आउटपुट
settings-cat-replay = रीप्ले
settings-cat-hotkeys = हॉटकी
settings-cat-network = नेटवर्क
settings-cat-accessibility = सुगम्यता
settings-cat-about = परिचय
settings-ok = ठीक है
settings-cancel = रद्द करें
settings-apply = लागू करें
settings-save = सहेजें
settings-loading = सेटिंग्स लोड हो रही हैं…
settings-hotkeys-filter = हॉटकी फ़िल्टर करें
settings-hotkeys-filter-placeholder = क्रियाएँ या कुंजियाँ फ़िल्टर करने के लिए टाइप करें…
settings-hotkeys-no-match = कोई हॉटकी “{ $query }” से मेल नहीं खाती।
settings-hotkey-none = कोई नहीं
settings-hotkey-group-ctrl = Ctrl + कुंजी
settings-hotkey-group-ctrl-shift = Ctrl + Shift + कुंजी
settings-hotkey-group-ctrl-alt = Ctrl + Alt + कुंजी
settings-hotkey-group-function = फ़ंक्शन कुंजियाँ
settings-hotkey-group-numpad = नमपैड
settings-panic-section = पैनिक स्लेट
settings-meter-section = मिक्सर स्तर मीटर
settings-meter-note = वे रंग जिनसे ऑडियो मिक्सर के स्तर मीटर शांत से क्लिपिंग तक गुज़रते हैं। वर्णांधता-सुरक्षित प्रीसेट नीले से नारंगी का ऐसा क्रम इस्तेमाल करता है जो लाल-हरे रंग की पहचान में कठिनाई होने पर भी पढ़ा जा सके।
settings-meter-preset = मीटर रंग
settings-meter-preset-default = हरा / पीला / लाल
settings-meter-preset-colorblind = वर्णांधता-सुरक्षित (नीला / नारंगी)
settings-meter-preset-custom = कस्टम
settings-meter-low = सामान्य
settings-meter-mid = तेज़
settings-meter-high = क्लिपिंग
settings-meter-preview = पूर्वावलोकन

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = नया क्या है
whats-new-loading = रिलीज़ नोट्स लोड हो रहे हैं…
whats-new-version = संस्करण { $version } में नया क्या है
whats-new-empty = इस संस्करण के लिए कोई रिलीज़ नोट्स नहीं हैं।
filters-name-directional-blur = दिशात्मक धुंधलापन
filters-name-radial-blur = रेडियल धुंधलापन
filters-name-zoom-blur = ज़ूम धुंधलापन
filters-name-pixelate = पिक्सेलेट
filters-angle = कोण (°)
filters-center-x = केंद्र X
filters-center-y = केंद्र Y
filters-block-size = ब्लॉक आकार (px)
filters-name-freeze = फ़्रीज़
filters-freeze-hint = सक्षम होने पर, यह स्रोत अपना अंतिम फ़्रेम रोक लेता है — प्रोग्राम, पूर्वावलोकन, रिकॉर्डिंग और स्ट्रीम सभी एक साथ फ़्रीज़ हो जाते हैं। फ़्रीज़ या अनफ़्रीज़ करने के लिए इस फ़िल्टर को टॉगल करें।
transform-3d = 3D झुकाव
transform-rotation-x = झुकाव X (°)
transform-rotation-y = झुकाव Y (°)
transform-perspective = परिप्रेक्ष्य
transform-reveal = दिखाएँ/छिपाएँ
transform-reveal-ms = फ़ेड-इन (ms)
sources-clone-title = क्लोन (वही फ़ीड, अपने फ़िल्टर)
sources-clone-item = { $name } क्लोन करें
menu-tools-downstream = डाउनस्ट्रीम कीयर…
menu-tools-transition-rules = ट्रांज़िशन नियम…
dsk-title = डाउनस्ट्रीम कीयर
dsk-hint = प्रोग्राम आउटपुट पर संयोजित ओवरले — हर दृश्य के ऊपर, और दृश्य बदलने पर वे यथावत रहते हैं (एक लोगो, एक LIVE बैज, एक लोअर-थर्ड)। सूची का शीर्ष सबसे ऊपर बनता है।
dsk-empty = अभी तक कोई कीयर नहीं — हर दृश्य पर ओवरले करने के लिए एक स्रोत जोड़ें।
dsk-enable = यह कीयर सक्षम करें
dsk-move-up = ऊपर ले जाएँ (सबसे ऊपर)
dsk-move-down = नीचे ले जाएँ
dsk-remove = कीयर हटाएँ
dsk-opacity = अपारदर्शिता
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = स्केल
dsk-add = + कीयर जोड़ें
transition-rules-title = ट्रांज़िशन नियम
transition-rules-hint = किसी सीन जोड़ी को उसका अपना ट्रांज़िशन दें। जब आप पहले सीन से दूसरे पर कमिट करते हैं, तो डिफ़ॉल्ट के बजाय यह प्रकार और अवधि उपयोग होती है (स्टिंगर/इमेज नियम अब भी ट्रांज़िशन नियंत्रण में सेट फ़ाइल का उपयोग करता है)।
transition-rules-empty = अभी कोई नियम नहीं — हर सीन जोड़ी डिफ़ॉल्ट ट्रांज़िशन का उपयोग करती है।
transition-rules-from = से
transition-rules-to = तक
transition-rules-kind = ट्रांज़िशन
transition-rules-duration = अवधि (ms)
transition-rules-add = नियम जोड़ें
transition-rules-remove = नियम हटाएँ

# --- Telestrator (CAP-N57) ---
telestrator-group = टेलीस्ट्रेटर
telestrator-draw = बनाएँ
telestrator-tool-pen = पेन
telestrator-tool-highlight = हाइलाइटर
telestrator-tool-arrow = तीर
telestrator-tool-ellipse = दीर्घवृत्त
telestrator-color = रंग
telestrator-width = चौड़ाई
telestrator-whiteboard = व्हाइटबोर्ड मोड
telestrator-persist = रखें
telestrator-fade = फ़ेड
telestrator-undo = अंतिम निशान पूर्ववत करें
telestrator-clear = निशान साफ़ करें
hotkeys-telestrator-clear = टेलीस्ट्रेटर: निशान साफ़ करें

# --- Teleprompter (CAP-N58) ---
teleprompter-title = टेलीप्रॉम्प्टर
teleprompter-loading = लोड हो रहा है…
teleprompter-empty = अभी कोई स्क्रिप्ट नहीं — टेलीप्रॉम्प्टर पैनल में एक जोड़ें।
teleprompter-script = स्क्रिप्ट
teleprompter-script-placeholder = अपनी स्क्रिप्ट टाइप करें या पेस्ट करें…
teleprompter-top = शीर्ष
teleprompter-slower = धीमा
teleprompter-play = चलाएँ
teleprompter-pause = रोकें
teleprompter-faster = तेज़
teleprompter-step-back = पीछे जाएँ
teleprompter-step-forward = आगे जाएँ
teleprompter-est-time = पढ़ने का समय
teleprompter-caesura-hint = रुकने के लिए -- टाइप करें
teleprompter-caesura-pause = विराम — रुकने के लिए -- टाइप करें
teleprompter-countdown = काउंटडाउन शुरू करें
teleprompter-countdown-secs = काउंटडाउन (सेकंड)
teleprompter-speed = गति (वर्ण/से)
teleprompter-speed-bpm = गति (BPM)
teleprompter-bpm-mode = BPM मोड (गायन)
teleprompter-font = फ़ॉन्ट आकार
teleprompter-mirror = दर्पण (ग्लास)
teleprompter-open-projector = प्रोजेक्टर खोलें
teleprompter-preview = पूर्वावलोकन
teleprompter-remote-hint = यह बंद होने पर हॉटकी, MIDI या LAN पैनल से स्क्रॉल नियंत्रित करें।
teleprompter-read-aloud = प्रति-OS वाक् संश्लेषण से ज़ोर से पढ़ें
menu-tools-teleprompter = टेलीप्रॉम्प्टर
hotkeys-teleprompter-toggle = टेलीप्रॉम्प्टर: चलाएँ / रोकें

# --- Remote guests: green room / cues / QoS / auto-grid (CAP-N54–N56, N59) ---
remote-hosting-count = { $count } अतिथि होस्ट हो रहे हैं
remote-green-room-default = ग्रीन रूम
remote-green-room-default-title = नए अतिथि तब तक ग्रीन रूम में रुकते हैं जब तक आप उन्हें बैठाते नहीं
remote-auto-grid = ऑटो-ग्रिड
remote-auto-grid-title = अतिथियों के आने-जाने पर उन्हें ग्रिड में रखें
remote-arrange-grid = ग्रिड लगाएँ
remote-arrange-grid-title = बैठे हुए अतिथियों को अभी ग्रिड में लगाएँ
remote-green-room-monitor = ग्रीन रूम पूर्वावलोकन
remote-tech-cam-ok = कैम ✓
remote-tech-cam-no = कैम नहीं
remote-tech-mic-ok = माइक ✓
remote-tech-mic-no = माइक नहीं
remote-seat-on-air = ऑन-एयर बैठाएँ
remote-cues-label = संकेत
remote-cue = संकेत
remote-cue-thirty = 30 सेकंड
remote-cue-wrap = समेटें
remote-cue-next = अगली बारी आपकी
remote-cue-speak = ज़ोर से बोलें
remote-qos-good = कनेक्शन अच्छा
remote-qos-fair = कनेक्शन ठीक-ठाक
remote-qos-poor = कनेक्शन खराब
remote-green-room-guest = आप ग्रीन रूम में हैं — होस्ट आपको जल्द ही ऑन-एयर करेगा।

# --- Hotkey audit — Phase 7 actions ---
hotkey-audit-action-telestrator-clear = टेलीस्ट्रेटर: निशान साफ़ करें
hotkey-audit-action-teleprompter-toggle = टेलीप्रॉम्प्टर: चलाएँ / रोकें
hotkey-audit-feature-telestrator = टेलीस्ट्रेटर
hotkey-audit-feature-teleprompter = टेलीप्रॉम्प्टर

# V1-C — Starting Soon countdown slate
sources-add-starting-soon = जल्द शुरू…
sources-starting-soon-title = जल्द शुरू
sources-starting-soon-default = जल्द शुरू
sources-starting-soon-message = संदेश
sources-starting-soon-minutes = मिनट
sources-starting-soon-hours = घंटे
sources-starting-soon-start-at = इस समय शुरू करें
sources-starting-soon-music = पृष्ठभूमि संगीत…
sources-starting-soon-music-name = पृष्ठभूमि संगीत
sources-starting-soon-background = पृष्ठभूमि
sources-starting-soon-note = एक पूर्ण-स्क्रीन काउंटडाउन जो शून्य पर लाल रंग में झपकता है और रुक जाता है — तैयार होने पर इसे हटाएँ और स्वयं अपने लाइव दृश्य पर स्विच करें।
sources-starting-soon-add = जोड़ें
sources-slate-solid = ठोस रंग
sources-slate-gradient = ग्रेडिएंट
sources-slate-transparent = पारदर्शी
sources-slate-image = छवि
sources-slate-file = छवि / वीडियो / GIF
sources-slate-browse = ब्राउज़ करें…
sources-starting-soon-choose-image = छवि चुनें…
sources-slate-media-note = स्थिर छवियाँ यहाँ एम्बेड होती हैं। लूपिंग वीडियो या GIF पृष्ठभूमि के लिए 'पारदर्शी' चुनें और दृश्य की पृष्ठभूमि (दृश्य पट्टी में 🖼) सेट करें — काउंटडाउन ऊपर रहता है।
properties-timer-slate = 'जल्द शुरू' स्लेट
properties-timer-slate-off = बंद (इनलाइन टेक्स्ट)
properties-timer-message = संदेश
properties-timer-slate-color = स्लेट रंग
properties-timer-slate-from = से
properties-timer-slate-to = तक
properties-timer-slate-note = पारदर्शी स्लेट के नीचे रखी छवि, वीडियो या दृश्य पृष्ठभूमि को दिखने देता है।

# V1-D: Social & channels bar source
sources-badge-social = Social
sources-add-social-bar = Social & Channels Bar
sources-social-title = Add a Social Bar
sources-social-add = Add social bar
sources-social-default-header = Follow me
sources-social-header = Header (blank = none)
sources-social-header-placeholder = Follow me
sources-social-size = Size (px)
sources-social-text-color = Text
sources-social-bg-color = Panel
sources-social-opacity = Panel opacity
sources-social-font = Font family (blank = default)
sources-social-font-placeholder = e.g. Inter, Arial
sources-social-accounts = Accounts (one row each — blank handles are skipped)
sources-social-empty = No accounts yet — add one below.
sources-social-platform = Platform
sources-social-custom = Custom
sources-social-handle = Handle
sources-social-handle-placeholder = @yourhandle
sources-social-custom-label = Label (e.g. Website)
sources-social-custom-color = Colour
sources-social-add-row = + Account
sources-social-up = Move account up
sources-social-down = Move account down
sources-social-remove = Remove account
sources-social-note = A generated panel drawn locally — no logos are fetched or embedded (a coloured badge marks each platform), nothing is read off disk, and nothing touches the network. Rows with a blank handle are skipped.

# CAP-N77: Browser source
sources-badge-browser = ब्राउज़र
sources-add-browser = ब्राउज़र (वेब पेज)
sources-browser-title = ब्राउज़र स्रोत जोड़ें
sources-browser-add = ब्राउज़र स्रोत जोड़ें
sources-browser-url = पेज URL (http:// या https://)
sources-browser-url-invalid = कोई http:// या https:// URL दर्ज करें — लोकल फ़ाइलें मीडिया या छवि स्रोतों से चलती हैं।
sources-browser-width = चौड़ाई (px)
sources-browser-height = ऊँचाई (px)
sources-browser-fps = FPS
sources-browser-transparent = पारदर्शी पेज बैकग्राउंड (अल्फ़ा बना रहता है)
sources-browser-note = http/https पेजों को ऑफ़स्क्रीन रेंडर करता है, ऑन-डिमांड ब्राउज़र रनटाइम कॉम्पोनेन्ट के ज़रिए — कभी बंडल नहीं, कभी प्रोसेस के भीतर नहीं। लोकल फ़ाइलें मीडिया/छवि से चलती हैं।
sources-browser-component-missing = ब्राउज़र रनटाइम कॉम्पोनेन्ट चाहिए — इसे उपकरण → कंपोनेंट से इंस्टॉल करें।
