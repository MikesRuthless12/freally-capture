# Freally Capture — tr
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Stüdyo Modu
toggle-on = açık
toggle-off = kapalı
stats = İstatistikler
core-ok = çekirdek Tamam
hide-stats-dock = İstatistik panelini gizle
show-stats-dock = İstatistik panelini göster


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Ayarlar kaydedilemedi — değişiklik yeniden başlatmadan sonra kaybolacak.
studio-mode-leave = Stüdyo Modundan Çık
studio-mode-enter-title = Stüdyo Modu — bir önizleme sahnesini düzenleyin, geçişle programa aktarın
vertical-canvas-title = İkinci (dikey 9:16) çıkış tuvali — bağımsız olarak kaydedilebilir ve yayınlanabilir
app-version = v{ $version }
core-error = çekirdek HATA
core-unreachable = çekirdeğe ulaşılamıyor (tarayıcı modu)
connecting-to-core = çekirdeğe bağlanılıyor…
filters-source-fallback = Kaynak

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Program önizlemesi
preview-program-output = Program çıkışı
preview-canvas-editor = Tuval düzenleyici
preview-px-to-edge-label = Kare kenarlarına piksel
preview-px-to-edge = kenara px Sol { $left } · Üst { $top } · Sağ { $right } · Alt { $bottom }
preview-program-heading = Program
preview-no-gpu = Kullanılabilir bir GPU bağdaştırıcısı bulunamadı — birleştirici bu makinede çalışamaz.
preview-starting-compositor = Birleştirici başlatılıyor…
preview-empty-scene = Bu sahne boş — Kaynaklar'dan bir kaynak ekleyin, sonra tam burada tuval üzerinde sürükleyin, ölçekleyin ve döndürün.
preview-fps = { $fps } fps
preview-dropped = { $dropped } düşürüldü

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Davet bağlantısı alındı
remote-join-with-webcam = Web kamerasıyla katıl
remote-dismiss = Kapat
remote-you-are-guest = Uzak bir konuksunuz
remote-share-view-title = Ekranınızı konuğun uygulamasına paylaşın (görüntünüzü canlı görürler)
remote-stop-sharing-view = Görüntü paylaşımını durdur
remote-share-my-view = Görüntümü paylaş
remote-allow-center-title = Konuğun merkezdeki görüntüyü değiştirmesine izin verin (kontrol sizde kalır, istediğiniz zaman geri alabilirsiniz)
remote-guest-switching = Konuk değiştirme:
remote-stop-screen = Ekranı durdur
remote-share-screen = Ekran paylaş
remote-share-screen-title-guest = Ekranınızı sunucuyla paylaşın (merkeze alabilecekleri bir kaynak olur)
remote-center-request-label = Merkez görünüm isteği
remote-center = Merkez
remote-center-my-cam = Kameram
remote-center-my-screen = Ekranım
remote-center-host-view = Sunucu görünümü
remote-end-session = Oturumu bitir
remote-leave = Ayrıl
remote-host-view-heading = Sunucu görünümü
remote-host-shared-view-label = Sunucunun paylaşılan görünümü
remote-guest-position-label = Konuk konumu
remote-put-guest = Konuğu { $position } koy
remote-remove-title = Konuğu kaldır — aynı bağlantıyla yeniden katılabilirler
remote-remove = Kaldır
remote-ban-title = Konuğu yasakla — onları engeller ve davet bağlantısını geçersiz kılar
remote-ban = Yasakla
remote-guest-self-muted = konuk kendini sessize aldı
remote-unmute-guest = Konuğun sesini aç
remote-mute-guest = Konuğu sessize al
remote-muted-by-host = Sunucu tarafından sessize alındı
remote-unmute-mic = Mikrofonu aç
remote-mute-mic = Mikrofonu sessize al
remote-waiting-for-host = sunucu bekleniyor


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = kaynak
sources-fallback-video = video
sources-fallback-error = hata
sources-kind-unknown = ?
sources-missing-source = (eksik kaynak)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Ekran
sources-badge-window = Pencere
sources-badge-portal = Portal
sources-badge-camera = Kamera
sources-badge-image = Görüntü
sources-badge-media = Medya
sources-badge-guest = Konuk
sources-badge-color = Renk
sources-badge-text = Metin
sources-badge-scene = Sahne
sources-badge-slides = Slaytlar
sources-badge-chat = Sohbet
sources-badge-audio-in = Ses Girişi
sources-badge-audio-out = Ses Çıkışı
sources-badge-app-audio = Uygulama Sesi
sources-badge-test-bars = Çubuklar
sources-badge-test-grid = Izgara
sources-badge-test-sweep = Süpürme
sources-badge-test-tone = Ton
sources-badge-test-sync = Senkron
sources-badge-timer = Zamanlayıcı

# Add-source menu items
sources-add-display = Ekran Yakalama
sources-add-window = Pencere Yakalama
sources-add-game = Oyun Yakalama (önce okuyun)
sources-add-webcam = Video Yakalama Aygıtı
sources-add-image = Görüntü
sources-add-media = Medya (video/görüntü dosyası)
sources-add-remote-guest = Uzak Konuk (P2P denemesi)
sources-add-color = Renk
sources-add-text = Metin
sources-add-timer = Zamanlayıcı / Saat
sources-add-nested-scene = İç İçe Sahne
sources-add-slideshow = Görüntü Slayt Gösterisi
sources-add-chat-overlay = Canlı Sohbet Katmanı
sources-add-test-signal = Test sinyali
sources-add-audio-input = Ses Girişi Yakalama
sources-add-audio-output = Ses Çıkışı Yakalama
sources-add-app-audio = Uygulama Sesi (Windows)
sources-add-existing = Mevcut kaynak…

# Panel header + toolbar buttons
sources-panel-title = Kaynaklar
sources-group-title = Kaynakları grupla — iki veya daha fazla öğe seçin, sonra Grup oluştur; gruplanan öğeler birlikte taşınır ve gösterilir/gizlenir
sources-group-aria = Kaynakları grupla
sources-arrange = Düzenle: ekran + köşeler
sources-add-source = Kaynak ekle
sources-browser-source-note = Tarayıcı Kaynağı kendi isteğe bağlı bileşen aşamasıyla gelir (~180 MB Chromium motoru — asla paketlenmez). Bugün: gerçek bir tarayıcı penceresini Pencere Yakalama + bir kroma/renk anahtarıyla yakalayın ya da sohbeti/uyarıları bir Panel olarak açın (Denetimler → Paneller).

# Empty state
sources-empty = Bu sahnede kaynak yok — “+” ile bir Ekran Yakalama, Pencere, Web kamerası, Görüntü, Renk veya Metin ekleyin. Bunları tuval üzerinde sürükleyin, ölçekleyin ve döndürün; sağ taraftaki düğmeler yığını yeniden sıralar.

# Per-row controls
sources-already-in-group = Zaten { $name } içinde
sources-pick-for-new-group = Yeni grup için seç
sources-pick-item-for-group = Yeni grup için { $name } seç
sources-hide = Gizle
sources-show = Göster
sources-hide-item = { $name } gizle
sources-show-item = { $name } göster
sources-stream-hide = Yayında gizle
sources-stream-show = Yayında göster
sources-stream-hide-item = { $name } yayında gizle
sources-stream-show-item = { $name } yayında göster
sources-record-hide = Kayıtta gizle
sources-record-show = Kayıtta göster
sources-record-hide-item = { $name } kayıtta gizle
sources-record-show-item = { $name } kayıtta göster
sources-unfocus-title = Odağı kaldır — düzeni geri yükle
sources-focus-title = Odakla — tuvali doldur (Konuşmacıyı Öne Çıkar)
sources-unfocus-item = { $name } odağını kaldır
sources-focus-item = { $name } odakla
sources-center-title = Merkeze al — bunu paylaşılan merkez görünümü yap (kameralar raya taşınır)
sources-center-item = { $name } merkeze al
sources-rename-item = { $name } yeniden adlandır
sources-in-group = { $name } grubunda

# Row status + retry
sources-retry-error = Yeniden dene — { $message }
sources-retry-item = { $name } yeniden dene
sources-status-error = durum: hata
sources-open-privacy-title = Bu izin için macOS gizlilik ayarlarını aç
sources-open-privacy-item = { $name } için gizlilik ayarlarını aç
sources-privacy-settings-button = ayarlar
sources-status-starting = başlatılıyor…
sources-status-live = canlı
sources-status-aria = durum: { $state }

# Media row pause/resume
sources-media-resume-title = Videoyu sürdür (yayında canlı)
sources-media-pause-title = Videoyu duraklat — kareyi dondur ve sessize al, yayında canlı
sources-media-resume-item = { $name } sürdür
sources-media-pause-item = { $name } duraklat

# Hover controls
sources-unlock = Kilidi aç
sources-lock = Kilitle
sources-unlock-item = { $name } kilidini aç
sources-lock-item = { $name } kilitle
sources-raise-title = Yığında yükselt
sources-raise-item = { $name } yükselt
sources-lower-title = Yığında alçalt
sources-lower-item = { $name } alçalt
sources-filters-title = Filtreler ve karışım
sources-filters-item = { $name } için filtreler
sources-properties-title = Özellikler
sources-properties-item = { $name } özellikleri
sources-remove-title = Bu sahneden kaldır
sources-remove-item = { $name } kaldır

# Grouping footer
sources-create-group = Grup oluştur ({ $count })
sources-cancel = İptal

# Groups list
sources-groups-aria = Kaynak grupları
sources-hide-group = Grubu gizle
sources-show-group = Grubu göster
sources-item-count = · { $count } öğe
sources-ungroup-title = Grubu çöz — öğeler oldukları yerde kalır
sources-ungroup-item = { $name } grubunu çöz

# Live Chat Overlay picker
sources-chat-title = Canlı Sohbet Katmanı ekle
sources-chat-youtube-label = YouTube — kanal, watch veya live_chat URL'si (anahtar yok, oturum açma yok)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  veya bir watch?v= URL'si
sources-chat-twitch-label = Twitch — kanal adı (anonim okunur, hesap yok)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — kanal etiketi (herkese açık uç nokta, mümkün olduğunca)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Mesajlar, saydam bir arka planda ilerleyen bir s:dd:ss ÖÖ/ÖS zaman damgasıyla görünür (varsayılan sağ üst; istediğiniz yere sürükleyin). Bir sohbet seli yalnızca eski satırları yaşlandırır — yayını veya kaydı asla durduramaz. Facebook sohbeti kendi Graph anahtarınızı gerektirir ve henüz uygulanmadı — asla gerekli değildir ve yukarıdaki platformları asla engellemez.
sources-chat-add = Sohbet katmanı ekle
sources-chat-default-name = Canlı Sohbet

# Image Slideshow picker
sources-slideshow-title = Görüntü Slayt Gösterisi ekle
sources-slideshow-empty = Henüz görüntü yok — Gözat, onları sırayla ekler.
sources-slideshow-remove-slide = { $number } numaralı slaytı kaldır
sources-slideshow-browse = Görüntülere gözat…
sources-slideshow-per-slide-label = Slayt başına (ms)
sources-slideshow-crossfade-label = Çapraz geçiş (ms, 0 = kesme)
sources-slideshow-loop-label = Döngü (kapalı = son slaytı tut)
sources-slideshow-shuffle-label = Her döngüde karıştır
sources-slideshow-note = Çapraz geçiş, eşit boyutlu görüntüleri harmanlar; farklı boyutlar sınırda sert kesme yapar (sessiz yeniden ölçekleme yok).
sources-slideshow-add = Slayt gösterisi ekle ({ $count })

# Nested Scene picker
sources-nested-title = İç İçe Sahne ekle
sources-nested-empty = İçe yerleştirilecek başka sahne yok — önce ikinci bir sahne ekleyin.
sources-nested-scene-name = Sahne: { $name }
sources-nested-note = İç içe sahne, program tuvali boyutunda canlı olarak işlenir ve kendi düzenlemelerini izler; dönüşümler, filtreler ve karışım ona da herhangi bir kaynak gibi uygulanır. Onu gösteren bir sahne programdayken ses kaynakları miksaja katılır.

# Display / Window capture picker
sources-capture-display-title = Ekran Yakalama ekle
sources-capture-window-title = Pencere Yakalama ekle
sources-capture-looking = Kaynaklar aranıyor…
sources-capture-none-displays = Burada yakalanacak bir şey yok — ekran bulunamadı.
sources-capture-none-windows = Burada yakalanacak bir şey yok — pencere bulunamadı.
sources-capture-portal-note = Wayland'de sistem iletişim kutusu ekranı veya pencereyi seçer — uygulamalar orada genel olarak yakalayamaz, dolayısıyla dürüst (ve tek) yol budur.
sources-capture-window-note = Önizlemeler canlı güncellenir. Küçültülmüş bir pencere, siz geri yükleyene kadar son karesini (ya da hiçbirini) gösterir.
sources-thumb-no-preview = önizleme yok
sources-thumb-loading = yükleniyor…

# Video Capture Device picker
sources-webcam-title = Video Yakalama Aygıtı ekle
sources-webcam-looking = Kameralar aranıyor…
sources-webcam-none = Kamera veya yakalama kartı bulunamadı.
sources-webcam-format-label = Biçim
sources-webcam-format-auto-loading = Otomatik (biçimler yükleniyor…)
sources-webcam-format-auto = Otomatik (en yüksek çözünürlük)
sources-webcam-card-presets-label = Kart ön ayarları:
sources-webcam-preset-title = Bu kartın belirttiği { $label } modunu seçin
sources-webcam-add = Kamera ekle

# Audio Input / Output capture picker
sources-audio-output-title = Ses Çıkışı Yakalama ekle
sources-audio-input-title = Ses Girişi Yakalama ekle
sources-audio-default-output = Varsayılan çıkış (duyduğunuz)
sources-audio-default-input = Varsayılan giriş
sources-audio-looking = Ses aygıtları aranıyor…
sources-audio-none-output = Burada masaüstü sesi yakalama aygıtı bulunamadı.
sources-audio-none-input = Mikrofon veya hat girişi bulunamadı.
sources-audio-input-note = Mikser şeritleri bir VU göstergesi, fader, sessize alma, izleme, filtreler (gürültü azaltma, kapı, kompresör…) ve iz ataması alır. Her şey bu makinede kalır.

# Application Audio picker
sources-appaudio-title = Uygulama Sesi ekle
sources-appaudio-looking = Ses çıkaran uygulamalar aranıyor…
sources-appaudio-none = Şu anda hiçbir uygulama ses çıkarmıyor — uygulamada oynatmayı başlatın, sonra yenileyin.
sources-appaudio-refresh = ⟳ Yenile
sources-appaudio-note = Tam olarak o uygulamanın sesini yakalar — kendi VU'su, fader'ı, sessize alması, filtreleri ve izi.

# Game Capture picker
sources-game-title = Oyun Yakalama
sources-game-checking = Kontrol ediliyor…
sources-game-use-portal = Ekran Yakalama (Portal) kullan
sources-game-use-window = Bunun yerine Pencere Yakalama kullan

# Image picker
sources-image-title = Görüntü ekle
sources-image-file-label = Görüntü dosyası (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Görüntü ekle

# Path field
sources-browse = Gözat…

# Media picker
sources-media-title = Medya ekle
sources-media-file-label = Medya dosyası (mp4, mkv, webm, mov, .frec veya bir görüntü)
sources-media-loop-label = Döngü (sonunda baştan başlat)
sources-media-note = .frec, sahip olunan freally-video codec'i üzerinden oynatılır — indirilecek bir şey yok. Wire biçimleri (mp4/mkv/webm/…) isteğe bağlı FFmpeg bileşeni üzerinden çözülür; sesi miksere kendi şeridi olarak düşer.
sources-media-add = Medya ekle

# Invite expiry options
sources-ttl-15min = 15 dk
sources-ttl-30min = 30 dk
sources-ttl-1hour = 1 saat
sources-ttl-1day = 1 gün

# Remote Guest form
sources-remote-copy-failed = kopyalanamadı — bağlantıyı seçip elle kopyalayın
sources-remote-join-failed = katılım başarısız: { $error }
sources-remote-title = Uzak Konuk (P2P denemesi)
sources-remote-host-heading = Sunucu — bir konuk davet edin
sources-remote-start-hosting = Barındırmaya başla
sources-remote-expires-label = Sona erme
sources-remote-invite-expiry-aria = Davet süresi
sources-remote-invite-link-aria = Davet bağlantısı
sources-remote-copied = Kopyalandı ✓
sources-remote-copy = Kopyala
sources-remote-share-note = Bu bağlantıyı paylaşın (Discord / mesaj / e-posta). Oturumunuzu taşır ve ayarlandığı gibi sona erer. Konuk bunu açar ve web kamerasıyla katılır.
sources-remote-qr-note = Doğrudan tarayıcıdan katılmak için bir telefonda tarayın — kamera + mikrofon, kurulum yok. Yukarıdaki kopyalanabilir freally:// bağlantısı, sahip olan bir makinede Freally Capture'ı açar.
sources-remote-guest-heading = Konuk — davetle katılın
sources-remote-paste-placeholder = davet bağlantısını yapıştırın
sources-remote-invite-input-aria = Davet bağlantısı veya oturum kimliği
sources-remote-join = Web kamerasıyla katıl
sources-remote-session-note = Canlı oturum denetimleri (sessize alma, bitirme) ana pencerenin üstündeki çubukta kalır — bu iletişim kutusunu kapatabilirsiniz.
sources-remote-stop-session = Oturumu durdur

# Invite QR
sources-invite-qr-aria = Davet bağlantısı QR kodu

# Remote device pickers
sources-devices-output-unavailable = çıkış yönlendirme kullanılamıyor — varsayılan aygıtta oynatılıyor
sources-devices-mic-test-failed = mikrofon testi başarısız: { $error }
sources-devices-heading = Oturum ses aygıtları
sources-devices-microphone-label = Mikrofon
sources-devices-microphone-aria = Oturum mikrofonu
sources-devices-system-default = Sistem varsayılanı
sources-devices-output-label = Çıkış
sources-devices-output-aria = Oturum ses çıkışı
sources-devices-stop-test = Testi durdur
sources-devices-test = Test — kendinizi duyun
sources-devices-testing-note = mikrofona konuşun — seçili aygıtları canlı duyuyorsunuz
sources-devices-idle-note = mikrofonunuzu çıkışa döndürür (kulaklık geri beslemeyi önler)

# TURN relay section
sources-turn-save-failed = kaydedilemedi: { $error }
sources-turn-summary = Ağ — isteğe bağlı TURN aktarımı (gelişmiş)
sources-turn-note-1 = Oturumlar doğrudan (P2P) bağlanır — ücretsiz, aktarım gerekmez. HER İKİ taraf da katı NAT'ların arkasındaysa doğrudan yol başarısız olabilir; o zaman kendi çalıştırdığınız bir TURN aktarımı medyayı taşır. Bunu atlamak sorun değil — çoğu bağlantı yalnızca doğrudan çalışır.
sources-turn-note-2 = Ücretsiz seçenek: Oracle Cloud "Always Free", coturn'ü ücretsiz çalıştırır (not: Oracle kayıtta kredi kartı ister ama Always-Free biçimi ücretsiz kalır). Adımlar: 1) ücretsiz VM oluşturun, 2) coturn kurun, 3) UDP 3478'i açın, 4) bir kullanıcı/parola ayarlayın, 5) buraya turn:vm-ip-adresiniz:3478 + kimlik bilgilerini girin. Kimlik bilgileriniz yerel ayarlar dosyanızda kalır ve asla günlüğe kaydedilmez.
sources-turn-url-label = TURN URL'si
sources-turn-url-placeholder = turn:host:3478 (boş = yalnızca doğrudan)
sources-turn-url-aria = TURN URL'si
sources-turn-username-label = Kullanıcı adı
sources-turn-username-aria = TURN kullanıcı adı
sources-turn-credential-label = Kimlik bilgisi
sources-turn-credential-aria = TURN kimlik bilgisi
sources-turn-note-3 = Aktarım, üç alanın tümü ayarlandığında devreye girer (bir TURN sunucusu kimlik bilgilerini gerektirir) ve başlattığınız veya katıldığınız bir sonraki oturuma uygulanır. Kendi iki makineniz arasında yalnızca aktarımlı bir test çağrısıyla doğrulayın.
sources-turn-settings-unavailable = ayarlar kullanılamıyor (tarayıcı modu)

# Color picker
sources-color-title = Renk ekle
sources-color-label = Renk
sources-color-width-label = Genişlik
sources-color-height-label = Yükseklik
sources-color-add = Renk ekle
sources-testsignal-title = Test sinyali ekle
sources-testsignal-pattern-label = Desen
sources-testsignal-bars = SMPTE renk çubukları
sources-testsignal-grid = Kalibrasyon ızgarası
sources-testsignal-sweep = Hareket süpürmesi
sources-testsignal-tone = 1 kHz ton (−20 dBFS)
sources-testsignal-flash-beep = A/V senkron flaş + bip
sources-testsignal-note = Kamera bağlamadan sahneleri, kodlayıcıları, projektörleri ve yayın hedeflerini doğrulayın. Flaş + bip deseni A/V senkron tezgâhını besler.
sources-testsignal-add = Test sinyali ekle
sources-timer-title = Zamanlayıcı ekle
sources-timer-mode-label = Mod
sources-timer-wall-clock = Duvar saati
sources-timer-countdown = Geri sayım
sources-timer-stopwatch = Kronometre
sources-timer-since-live = Yayından bu yana
sources-timer-since-recording = Kayıttan bu yana
sources-timer-note = Süre, biçim, stil ve geri sayım bitiş eylemleri kaynağın Özellikler'indedir.
sources-timer-add = Zamanlayıcı ekle

# Text picker
sources-text-title = Metin ekle
sources-text-label = Metin
sources-text-default = Metin
sources-text-color-label = Renk
sources-text-color-aria = Metin rengi
sources-text-size-label = Boyut (px)
sources-text-note = Yazı tipi ailesi, hizalama, sarma ve RTL, kaynağın Özellikler'inde bulunur. Paketlenmiş Noto Sans (Arapça/İbranice dahil) varsayılandır — her makinede aynı.
sources-text-add = Metin ekle

# Existing source picker
sources-existing-title = Mevcut bir kaynak ekle
sources-existing-empty = Henüz kaynak yok — önce herhangi bir sahneye bir tane ekleyin. Mevcut kaynaklar paylaşılır: birini yeniden adlandırmak veya yeniden yapılandırmak, onu gösteren her sahneyi günceller.

# Screen + corners layout
sources-slot-off = Kapalı
sources-slot-center = Merkez (ekran)
sources-slot-top-left = Sol Üst
sources-slot-top-right = Sağ Üst
sources-slot-bottom-left = Sol Alt
sources-slot-bottom-right = Sağ Alt
sources-layout-title = Düzenle: Ekran + köşeler
sources-layout-empty = Önce bu sahneye bir ekran yakalama ve bir veya daha fazla kamera ekleyin, sonra bunları burada düzenleyin.
sources-layout-note = Merkeze bir ekran ve köşelere en fazla dört kamera koyun — açıklayıcı / podcast düzeniniz. Her köşe bir web kamerası, yakalanan bir çağrı penceresi veya bir medya klibi tutar. Sonrasında herhangi birini tuval üzerinde sürükleyebilirsiniz.
sources-layout-slot-aria = { $name } için yuva
sources-layout-apply = Düzeni uygula


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Denetimler
controls-start-stop-title-stop = Kaydı durdur ve sonlandır
controls-start-stop-title-start = Program akışını Ayarlar → Çıkış yapılandırmasıyla kaydet
controls-finalizing = ◌ Sonlandırılıyor…
controls-stop-recording = ■ Kaydı Durdur
controls-start-recording = ● Kaydı Başlat
controls-marker-title = Bu anda bir bölüm işaretçisi bırak — KAYDA düşer (mkv bölümleri veya bir yardımcı dosya). Platform tarafındaki yayın işaretçileri, bu uygulamanın asla istemediği platform hesaplarını gerektirir.
controls-marker = ◈ İşaretçi
controls-iso-lanes = Programın yanında kaydedilen ISO şeritleri: { $count }
controls-pause-title-resume = Sürdür — dosya tek bir kesintisiz zaman çizelgesi olarak devam eder
controls-pause-title-pause = Duraklat — hiçbir kare yazılmaz; sürdürmek aynı oynatılabilir dosyaya devam eder
controls-resume-recording = ▶ Kaydı Sürdür
controls-pause-recording = ⏸ Kaydı Duraklat
controls-reactions-label = Tepkiler (programa gömülü)
controls-reactions-title = Programın üzerinde bir tepki uçur — kaydedilir VE yayınlanır, böylece tekrar tam anı gösterir. Sohbetteki izleyiciler de bunları tetikler (tepki emojileri otomatik uçar); bir sel yalnızca ekrandakini sınırlar.
controls-react = { $emoji } tepki ver
controls-virtual-camera-title = Sanal kamera her işletim sistemi için kendi imzalı sürücü bileşenini gerektirir (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO uzantısı / Linux v4l2loopback) — kendi aşaması olarak gelir. Akış modeli hazır: program, dikey tuval veya tek bir kaynak, Windows/Linux'ta eşli bir sanal mikrofonla (macOS'te sanal mikrofon API'si yok — dürüstçe söylenirse).
controls-virtual-camera = ⌁ Sanal Kamerayı Başlat
controls-saved = Kaydedildi: { $path }

# --- MixerDock.tsx ---
mixer-title = Ses Mikseri
mixer-monitor-error = izleme: { $error }
mixer-switch-to-horizontal = Yatay şeritlere geç
mixer-switch-to-vertical = Dikey şeritlere geç
mixer-layout-aria-vertical = Mikser düzeni: dikey — yataya geç
mixer-layout-aria-horizontal = Mikser düzeni: yatay — dikeye geç
mixer-empty = Bu sahnede ses kaynağı yok — Kaynaklar'da “+” ile bir Ses Girişi Yakalama (mikrofon) veya Ses Çıkışı Yakalama (masaüstü sesi) ekleyin. Şeritler bir VU göstergesi, fader, sessize alma, izleme, filtreler ve iz ataması alır.
mixer-advanced-title = Ses — { $name }
mixer-loudness-label = Program ses düzeyi (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Anlık ses düzeyi (400 ms)
mixer-short-term-title = Kısa süreli ses düzeyi (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = İzleme
mixer-monitor-device-aria = İzleme çıkış aygıtı
mixer-default-output = Varsayılan çıkış
mixer-routing = Yönlendirme
mixer-routing-title = Ses çıkışı yönlendirme

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Ses yönlendirme
routing-intro = Şeritleri iz bus'larına atayın, ardından herhangi bir bus'ı fiziksel bir çıkışa gönderin — bir donanım kaydediciye besleme, başka bir odadaki hoparlörler ya da boş bir izde kulaklık cue'su. İzleme kendi aygıtını korur; bu yönlendirmeler bunun üzerine eklenir, dolayısıyla hiçbiri ayarlanmazsa miks değişmez.
routing-sends-title = İz göndermeleri
routing-no-strips = Bu sahnede ses kaynağı yok.
routing-source = Kaynak
routing-track = İz { $n }
routing-send-aria = { $source } kaynağını İz { $n }'e gönder
routing-outputs-title = Fiziksel çıkışlar
routing-master = Master
routing-off = Kapalı
routing-default-output = Varsayılan çıkış
routing-device-aria = { $bus } için çıkış aygıtı
routing-trim-aria = { $bus } için çıkış trim
routing-trim-db = { $db } dB
routing-muted = Sessiz
routing-device-error = Aygıt kullanılamıyor

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Ducking
mixer-ducking-title = Ducking matrisi
ducking-title = Ducking matrisi
ducking-intro = Herhangi bir kaynak diğerlerini duck edebilir. Tetik (satır) konuştuğunda bir hücre hedefi (sütun) kısar — derinliğini, eşiğini ve zamanlamasını ayarlamak için bir hücre seçin. Her çift kendi ducking'idir, böylece bir kanal aynı anda birden fazla tetik tarafından duck edilebilir.
ducking-need-two = Aralarında ducking yapmak için en az iki ses kaynağı ekleyin.
ducking-trigger-target = Tetik ↓ / Hedef →
ducking-cell-aria = { $trigger }, { $target } sesini kısıyor
ducking-pair = { $trigger } → { $target }
ducking-remove = Kaldır
ducking-amount = Miktar
ducking-threshold = Eşik
ducking-attack = Atak
ducking-release = Bırakma
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Ses düzeyi normalleştirme
loudness-intro = Programı bir tepe tavanıyla birlikte ses düzeyi hedefine doğru yavaşça yönlendirir; böylece yayınınız ve kayıtlarınız tutarlı bir seviyeye oturur. Yavaş ve nazik — yönlendirir, asla pompalamaz.
loudness-enable = Programı hedefe yönlendir
loudness-target = Hedef
loudness-target-option = { $target } LUFS
loudness-ceiling = Tepe tavanı (dBFS)
loudness-note = −14 LUFS YouTube tarzı oynatmaya uygundur; −16 yaygın bir yayın hedefidir; −23 ise EBU R128 yayıncılığıdır. Kayıt sonrası Normalleştir eyleminde de aynı hedef kullanılır.
ltc-badge = LTC
ltc-title = SMPTE Zaman Kodu (LTC)
ltc-intro = Bir kanala SMPTE doğrusal zaman kodu üret ve herhangi bir ses girişinden gelen LTC'yi oku — harici kaydediciler ve kameraları postta senkronlamak için klasik ses zaman kodu. Tümüyle çevrimdışı.
ltc-generate = Bir kanala LTC üret
ltc-track = Zaman kodu kanalı
ltc-track-option = Kanal { $track }
ltc-fps = Kare hızı
ltc-read = LTC okunacak kaynak
ltc-read-off = Kapalı
ltc-decoded = Gelen zaman kodu
ltc-no-lock = sinyal yok
ltc-note = Üretici günün saatine jam-sync olur, non-drop. Kanalını kaydet (Çıktı ayarlarında ata) ya da harici cihaza beslemek için bir çıkışa yönlendir. Okuyucu, istatistik yer paylaşımının zaman kodu satırını besler ve bölüm işaretçilerini damgalar.
loudness-on = LUFS { $target }
loudness-off = Norm. kapalı

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Ses panosu
mixer-soundboard-title = Ses panosu
soundboard-title = Ses panosu
soundboard-add-pad = + Pad
soundboard-stop-all = Tümünü durdur
soundboard-edit = Düzenle
soundboard-empty = Henüz pad yok — bir tane ekleyin ve yerel bir ses klibi atayın.
soundboard-new-pad = Yeni pad
soundboard-no-clip = Klip yok
soundboard-audio-files = Ses dosyaları
soundboard-name = Ad
soundboard-choose-clip = Klip seç…
soundboard-gain = Kazanç
soundboard-choke = Choke
soundboard-choke-none = Yok
soundboard-loop = Döngü
soundboard-auto-duck = Otomatik ducking
soundboard-tracks = İzler
soundboard-hotkey = Kısayol
soundboard-hotkey-placeholder = örn. Ctrl+Shift+1
soundboard-remove = Kaldır

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Eklentiler
mixer-plugins-title = Ses eklentileri (CLAP / VST3)
plugins-title = Ses eklentileri
plugins-scanning = Taranıyor…
plugins-none = Standart klasörlerde CLAP veya VST3 eklentisi bulunamadı.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Bellek
stats-dropped = Düşürülen
stats-render = İşleme
stats-gpu = GPU
stats-gpu-compositing = birleştiriyor
stats-gpu-idle = boşta
stats-disk = Disk
stats-disk-free = boş
stats-disk-left = Kayıt kaldı
stats-disk-rate = ≈ { $rate } MB/s kayıt
stats-vertical-fps = 9:16 FPS
stats-targets-label = Yayın hedefleri
stats-rehearsal-note = Prova — hedefler yalnızca yerel bir alıcıya yayınlar
stats-timeline-open = Zaman çizelgesi
timeline-title = Oturum zaman çizelgesi
timeline-empty = Henüz kayıt yok — çizelge yayın veya kayıt sırasında tutulur.
timeline-live = CANLI — hâlâ kaydediyor
timeline-fit = Sığdır
timeline-legend-fps = fps
timeline-legend-behind = kodlayıcı kuyruğu (geride kare)
stats-shared-encode = · paylaşılan kodlama
stats-starting = Birleştirici başlatılıyor…

# --- ScenesRail.tsx ---
scenes-title = Sahneler
scenes-new-scene-name = Sahne
scenes-add = Sahne ekle
scenes-empty = Stüdyo çekirdeğine bağlanılıyor…
scenes-rename = { $name } yeniden adlandır
scenes-on-program = Programda
scenes-preview = { $name } önizle
scenes-switch-to = { $name } geç
scenes-move-up = Yukarı taşı
scenes-move-up-aria = { $name } yukarı taşı
scenes-move-down = Aşağı taşı
scenes-move-down-aria = { $name } aşağı taşı
scenes-last-stays = Son sahne kalır
scenes-remove = Bu sahneyi kaldır
scenes-remove-aria = { $name } kaldır


# =============================================================
# --- components ---
# =============================================================
# components
# Extracted user-visible strings from ui/src/components/*.tsx:
#   ChannelStrip, LiveButton, RecDot, ReplayControls,
#   PropertiesDialog, AudioFiltersDialog, FiltersDialog, PickerShell.
# (Panel.tsx and NumberField.tsx render only caller-supplied props — no literals.)
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- ChannelStrip.tsx ---
channelstrip-level = Seviye
channelstrip-monitor-off = İzleme kapalı
channelstrip-monitor-only = Yalnızca izleme (miksajda değil)
channelstrip-monitor-and-output = İzleme ve çıkış
channelstrip-status-error = hata
channelstrip-status-live = canlı
channelstrip-status-waiting-audio = ses bekleniyor
channelstrip-status = durum: { $state }
channelstrip-status-waiting = bekliyor
channelstrip-mute = Sessize al
channelstrip-unmute = Sesi aç
channelstrip-mute-source = { $name } sessize al
channelstrip-unmute-source = { $name } sesini aç
channelstrip-scene-mix-on = Sahne başına miksaj AÇIK — bu şerit bu sahne için genel miksajı geçersiz kılar (genel miksajı yeniden izlemek için tıklayın)
channelstrip-scene-mix-off = Sahne başına miksaj — bu şeride mevcut sahne için kendi fader'ını/sessize almasını verin
channelstrip-scene-mix-label = { $name } için sahne başına miksaj
channelstrip-monitor-cycle = { $mode } — döngü için tıklayın
channelstrip-monitor-mode = { $name } izleme modu: { $mode }
channelstrip-audio-filters-title = Ses filtreleri (gürültü azaltma, kapı, kompresör…)
channelstrip-audio-filters-label = { $name } için ses filtreleri
channelstrip-advanced-title = Senkronizasyon kaydırması ve bas-konuş kısayolları
channelstrip-advanced-label = { $name } için gelişmiş ses ayarları
channelstrip-track-assignment = İz ataması
channelstrip-track = İz { $n }
channelstrip-track-assigned = İz { $n } (atandı)
channelstrip-track-label = { $name } için İz { $n }
channelstrip-device-error = aygıt hatası
channelstrip-audio-device-error = ses aygıtı hatası
channelstrip-volume-label = { $name } ses düzeyi (desibel)
channelstrip-ptt-hold = Bas-konuş: { $key } basılı tut
channelstrip-sync-offset = Senkronizasyon kaydırması (ms, 0–{ $max } — bu sesi geciktirir)
channelstrip-solo-title = Solo (PFL) — monitör yalnız solodaki şeritleri duyar; program miksi değişmez
channelstrip-solo-source = { $name } solo (PFL)
channelstrip-pan-label = Denge (çift tıklama sıfırlar)
channelstrip-pan-aria = { $name } dengesi
channelstrip-mono-label = Monoya indir
channelstrip-automix-label = Oto-miks (kazanç paylaşımı)
channelstrip-automix-note = Kazanç paylaşımı: mikser tüm oto-miks şeritlerinin birleşik seviyesini sabit tutar ve onu o an konuşana devreder — çok mikrofonlu paneller ve podcast'ler için idealdir. Bir şerit ekleyene kadar kapalı.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Bu kaynak için yankısız bir dönüş üretir — kaynağın kendisi dışında programdaki herkes. Uzak bir konuk için kullanın, böylece kendi gecikmeli sesini duymaz.
channelstrip-ptt-hotkey = Bas-konuş kısayolu (basılı tutulmadıkça sessiz)
channelstrip-ptt-placeholder = örn. Ctrl+Shift+T veya F13
channelstrip-ptt-aria = Bas-konuş kısayolu
channelstrip-ptm-hotkey = Bas-sustur kısayolu (basılı tutulurken sessiz)
channelstrip-ptm-placeholder = örn. Ctrl+Shift+M
channelstrip-ptm-aria = Bas-sustur kısayolu
channelstrip-hotkeys-note = Kısayollar, diğer uygulamalar odaktayken çalışır. Linux/Wayland'de genel kısayollar kullanılamayabilir — bu bir birleştirici sınırıdır, dürüstçe söylenirse.
channelstrip-apply = Uygula


# --- LiveButton.tsx ---
livebutton-failure-ended = yayın sona erdi
livebutton-title-live = Yayını bitir — her hedef (çalışan bir kayıt devam eder)
livebutton-title-offline = Etkin her Ayarlar → Yayın hedefine yayına geç
livebutton-end-stream = ■ Yayını Bitir
livebutton-aria-reconnecting = Yeniden bağlanıyor
livebutton-aria-live = Canlı
livebutton-badge-retry = yeniden dene { $n }
livebutton-badge-live = canlı
livebutton-go-live = ⦿ Yayına Başla
livebutton-rehearse = Prova
livebutton-rehearse-title = Tüm yayını yerel bir alıcıya karşı çalıştır — hiçbir şey gönderilmez
livebutton-end-rehearsal = Provayı bitir
livebutton-title-rehearsing = Bir prova sürüyor — bu makineden hiçbir şey çıkmaz
livebutton-badge-rehearsal = PROVA
livebutton-aria-rehearsal = Prova sürüyor
livebutton-rehearsal-banner = Prova — bu makineden hiçbir şey çıkmaz


# --- RecDot.tsx ---
recdot-paused-aria = Kayıt duraklatıldı
recdot-recording-aria = Kaydediliyor
recdot-tracks-one = { $count } ses izi kaydediliyor
recdot-tracks-other = { $count } ses izi kaydediliyor
recdot-paused = duraklatıldı


# --- ReplayControls.tsx ---
replaycontrols-saved = Tekrar kaydedildi — { $name }
replaycontrols-failure-stopped = arabellek durdu
replaycontrols-title-disarm = Tekrar arabelleğini devre dışı bırak (kaydedilmemiş geçmişi düşürür)
replaycontrols-title-arm = Dönen tekrar arabelleğini etkinleştir — kaydetmeye hazır son N saniyeyi tutar (kendi hafif kodlaması; yayın ve kayıt etkilenmez)
replaycontrols-replay-seconds = ⟲ Tekrar { $seconds }s
replaycontrols-arm = ⟲ Tekrar Arabelleğini Etkinleştir
replaycontrols-save-title = Son N saniyeyi kayıtlar klasörüne kaydet (Tekrarı-Kaydet kısayolunda da)
replaycontrols-save = ⤓ Kaydet


# --- PropertiesDialog.tsx ---
properties-title = Özellikler — { $name }
properties-name = Ad
properties-cancel = İptal
properties-apply = Uygula
properties-youtube = YouTube — kanal / watch / live_chat URL'si (anahtar yok, oturum açma yok, hiç)
properties-twitch = Twitch — kanal adı (anonim)
properties-kick = Kick — kanal etiketi (herkese açık uç nokta)
properties-width-px = Genişlik (px)
properties-lines = Satırlar
properties-font-px = Yazı tipi (px)
properties-images = Görüntü dosyaları (satır başına bir yol, sırayla gösterilir)
properties-per-slide = Slayt başına (ms)
properties-crossfade = Çapraz geçiş (ms, 0 = kesme)
properties-loop-slideshow = Döngü (kapalı = son slaytı tut)
properties-shuffle = Her döngüde karıştır
properties-nested-scene = Bu kaynağın oluşturduğu sahne (bunu zaten içeren bir sahne reddedilir)
properties-portal-note = Wayland ScreenCast portalı, bu kaynak her başladığında sistem iletişim kutusunda ekranı veya pencereyi seçer — tasarım gereği burada yapılandırılacak bir şey yok.
properties-appaudio-capturing = { $exe } uygulamasından ses yakalanıyor
properties-appaudio-exe-fallback = bir uygulama
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Farklı bir uygulamayı hedeflemek için kaynağı yeniden ekleyin (uygulama yeniden başladığında işlem kimliği değişir).
properties-image-file = Görüntü dosyası
properties-media-file = Medya dosyası (mp4, mkv, webm, mov, .frec veya bir görüntü)
properties-media-loop = Döngü (sonunda baştan başlat)
properties-media-hwdecode = Donanım çözme (kendiliğinden yazılıma geri döner)
properties-media-note = .frec, sahip olunan freally-video codec'i üzerinden oynatılır — indirilecek bir şey yok. Diğer video biçimleri isteğe bağlı FFmpeg bileşeni üzerinden çözülür. Dosyanın sesi kendi mikser şeridini alır; şeridin senkronizasyon kaydırması A/V hizasını ince ayarlar. Sesi olmayan bir klip şeridini sessiz bırakır.
properties-color = Renk
properties-width = Genişlik
properties-height = Yükseklik
properties-testtone-note = −20 dBFS'te sürekli 1 kHz sinüs. Seviye ve sessize alma mikser şeridindedir; ayarlanacak başka bir şey yok.
properties-timer-format = Saat biçimi (strftime)
properties-timer-format-note = örn. %H:%M:%S (varsayılan), %I:%M %p, %A %H:%M — geçersiz desen %H:%M:%S'e döner.
properties-timer-utc = UTC farkı (dakika)
properties-timer-utc-placeholder = yerel saat
properties-timer-duration = Süre (saniye)
properties-timer-target = Şu saate geri say (HH:MM)
properties-timer-target-note = Duvar saati hedefi kendi başına çalışır ve her gün yinelenir; süreyi Başlat/Duraklat/Sıfırla ile kullanmak için boş bırakın.
properties-timer-end = Sıfırda
properties-timer-end-none = Hiçbir şey yapma
properties-timer-end-flash = Zamanlayıcıyı yakıp söndür
properties-timer-end-switch = Sahne değiştir
properties-timer-end-scene = Sahne
properties-timer-size = Boyut (px)
properties-timer-start = Başlat
properties-timer-pause = Duraklat
properties-timer-reset = Sıfırla
properties-text-file = Dosyadan oku (yol; boş = yukarıdaki metin)
properties-text-binding = Şu şekilde çözümle
properties-text-binding-whole = Tüm dosya
properties-text-binding-csv = CSV hücresi
properties-text-binding-json = JSON işaretçisi
properties-text-csv-row = Satır
properties-text-csv-column = Sütun
properties-text-csv-column-placeholder = ad veya numara
properties-text-json-pointer = İşaretçi
properties-text-file-note = Dosya, değişiklikten yarım saniye içinde yeniden okunur. Atomik yazımlar (geçici + yeniden adlandırma) tolere edilir: takas sırasında son iyi değer ekranda kalır.
avsync-title = A/V Senkron Kalibrasyonu
avsync-intro = Yerleşik flaş + bip desenini ekranınız ve hoparlörlerinizden oynatın, hizalamak istediğiniz kamera ve mikrofonla geri yakalayın — tezgâh aradaki farkı ölçer. Döngü ekran ve hoparlörlerden geçtiği için onların küçük gecikmeleri de dahildir.
avsync-video-label = Kamera (video kaynağı)
avsync-audio-label = Mikrofon (ses kaynağı)
avsync-pick = Kaynak seçin…
avsync-no-video = Önce kamerayı kaynak olarak ekleyin — tezgâh ham cihazları değil kaynakları ölçer.
avsync-no-audio = Önce mikrofonu ses kaynağı olarak ekleyin.
avsync-projector = Programı tam ekran göster:
avsync-projector-open = Projektörü aç
avsync-projector-window-title = Program — A/V senkron
avsync-start-note = Başlatmak, geçerli sahnenin üstüne geçici bir "A/V Senkron Deseni" kaynağı ekler ve bipi monitör cihazında çalar. Bittiğinde her şey kaldırılır.
avsync-manual = Senkron ofseti (ms, elle)
avsync-start = Kalibrasyonu başlat
avsync-measuring = Yaklaşık 12 saniye ölçülüyor — kamerayı yanıp sönen programa çevirin ve odayı sakin tutun…
avsync-flash-seen = Kamera flaşı görüyor
avsync-flash-waiting = Kameranın flaşı görmesi bekleniyor…
avsync-beep-heard = Mikrofon bipi duyuyor
avsync-beep-waiting = Mikrofonun bipi duyması bekleniyor…
avsync-cancel = İptal
avsync-result-offset = Video, sesten { $offset } ms sonra geliyor.
avsync-result-detail = { $cycles } döngüde ölçüldü, ±{ $jitter } ms.
avsync-negative = Ses zaten videodan geç geliyor. Sesi geciktirmek bu yönü düzeltemez — bu kameranın sesini başka bir şerit taşıyorsa oradaki ofseti düşürün.
avsync-over-cap = Ölçülen fark { $max } ms ofset sınırının üzerinde. Bu kadar büyük fark çoğunlukla yanlış kaynak demektir — zinciri kontrol edip yeniden ölçün.
avsync-applied = Uygulandı — mikrofonun senkron ofseti artık { $offset } ms.
avsync-apply = Mikrofona { $offset } ms uygula
avsync-again = Yeniden ölç
avsync-close = Kapat
avsync-error-noFlash = Kamera flaşı hiç görmedi. Yanıp sönen programa çevirin (tam ekran yardımcı olur), kaynağın canlı olduğundan emin olun ve yeniden ölçün.
avsync-error-noBeep = Mikrofon bipi hiç duymadı. Monitör cihazının duyulur ve mikrofonun canlı olduğundan (bas-konuş ile kapalı değil) emin olun, sonra yeniden ölçün.
avsync-error-tooFewCycles = Yeterince temiz flaş/bip döngüsü yakalanmadı. Deseni ölçüm boyunca net görünür ve duyulur tutun.
avsync-error-notThePattern = Görülen ya da duyulan, desenin ritminde tekrarlanmıyor — büyük olasılıkla oda ışığı veya gürültü, test sinyali değil.
avsync-error-unstable = Döngüler tek bir sayıya güvenilemeyecek kadar uyumsuz. Kamerayı sabitleyin, gürültüyü azaltın ve yeniden ölçün.
hotkey-audit-title = Kısayol Haritası
hotkey-audit-search = Ara
hotkey-audit-filter = Özellik
hotkey-audit-filter-all = Tüm özellikler
hotkey-audit-col-key = Tuş
hotkey-audit-col-action = Eylem
hotkey-audit-col-where = Nerede
hotkey-audit-col-status = Durum
hotkey-audit-ok = Tamam
hotkey-audit-shared = { $count } atama paylaşıyor
hotkey-audit-unregistered = İşletim sistemine kayıtlı değil (başka yerde kapılmış ya da kullanılamaz)
hotkey-audit-invalid = Geçersiz kısayol
hotkey-audit-empty = Henüz kısayol yok — Ayarlar → Kısayollar'da veya bir mikser şeridinde atayın.
hotkey-audit-export = Kopya kâğıdını dışa aktar
hotkey-audit-exported = { $path } konumuna kaydedildi
hotkey-audit-note = Tuşlar Ayarlar → Kısayollar'da (genel eylemler) ve her mikser şeridinde (bas-konuş / bas-sustur) atanır ve değiştirilir; bu tablo denetler ve belgeler.
hotkey-audit-action-record = Kaydı aç/kapat
hotkey-audit-action-go-live = Yayını aç/kapat
hotkey-audit-action-transition = Geçişi uygula
hotkey-audit-action-save-replay = Tekrarı kaydet
hotkey-audit-action-add-marker = İşaret ekle
hotkey-audit-action-still = Kare yakala
hotkey-audit-action-panic = Panik ekranı
hotkey-audit-action-timer-toggle = Tüm zamanlayıcıları başlat/duraklat
hotkey-audit-action-timer-reset = Tüm zamanlayıcıları sıfırla
hotkey-audit-action-ptt = Bas-konuş
hotkey-audit-action-ptm = Bas-sustur
hotkey-audit-feature-recording = Kayıt
hotkey-audit-feature-streaming = Yayın
hotkey-audit-feature-studio = Stüdyo modu
hotkey-audit-feature-replay = Tekrar
hotkey-audit-feature-markers = İşaretler
hotkey-audit-feature-stills = Kareler
hotkey-audit-feature-panic = Panik
hotkey-audit-feature-timers = Zamanlayıcılar
hotkey-audit-feature-audio = Ses (kaynak başına)
properties-text = Metin
properties-font-family = Yazı tipi ailesi (sistem; boş = varsayılan)
properties-size-px = Boyut (px)
properties-text-color = Metin rengi
properties-align = Hizala
properties-align-left = sol
properties-align-center = orta
properties-align-right = sağ
properties-line-spacing = Satır aralığı
properties-wrap-width = Sarma genişliği (px; 0 = kapalı)
properties-force-rtl = Sağdan sola zorla
properties-text-note = İşleme, gerçek şekillendirme (Arapça birleştirme, ligatürler) ve çift yönlü satır sıralaması kullanır. Paketlenmiş Noto Sans ailesi (Arapça/İbranice dahil) varsayılandır; sistem aileleri de çalışır. CJK şimdilik sistem yazı tiplerini kullanır.
properties-repick-capturing = Yakalanıyor: { $label }
properties-repick-looking = Kaynaklar aranıyor…
properties-repick-none-displays = Yeniden seçilecek ekran bulunamadı.
properties-repick-none-windows = Yeniden seçilecek pencere bulunamadı.
properties-repick-again = Yeniden seç:
properties-device = Aygıt
properties-video-current-device = (mevcut aygıt)
properties-format = Biçim
properties-format-auto-loading = Otomatik (biçimler yükleniyor…)
properties-deinterlace = Deinterlacing
properties-deinterlace-off = Kapalı
properties-deinterlace-discard = At (tek alanın satırlarını çiftle)
properties-deinterlace-bob = Bob (alanlar dönüşümlü)
properties-deinterlace-linear = Doğrusal (ara değerle)
properties-deinterlace-blend = Karıştır (alan ortalaması)
properties-deinterlace-adaptive = Harekete uyarlanır (yadif sınıfı)
properties-field-order = Alan sırası
properties-field-order-top = Önce üst alan
properties-field-order-bottom = Önce alt alan
properties-deinterlace-note = Taramalı yakalama kartı sinyalleri için. Saf CPU, her işletim sisteminde aynı; değiştirmek aygıtı yeniden başlatır (biçim değişikliği gibi).
camera-controls-title = Kamera denetimleri
camera-controls-refresh = Yenile
camera-controls-reset = Profili sıfırla
camera-controls-empty = Şu an denetim yok — aygıt yayında olmalı (önce bir sahneye ekleyin) ve bazı arka uçlar hiç bildirmez (özellikle macOS). İşletim sistemi başına dürüst durum bu.
camera-controls-note = Değişiklikler anında uygulanır ve aygıtın profiline kaydedilir; yeniden takmada ve yeniden başlatmada tekrar uygulanır.
camera-control-brightness = Parlaklık
camera-control-contrast = Karşıtlık
camera-control-hue = Ton
camera-control-saturation = Doygunluk
camera-control-sharpness = Keskinlik
camera-control-gamma = Gama
camera-control-white-balance = Beyaz dengesi
camera-control-backlight = Arka ışık telafisi
camera-control-gain = Kazanç
camera-control-pan = Pan
camera-control-tilt = Eğim
camera-control-zoom = Yakınlaştırma
camera-control-exposure = Pozlama
camera-control-iris = İris
camera-control-focus = Odak
properties-format-auto = Otomatik (en yüksek çözünürlük)
properties-audio-capture-of = Sesini yakala
properties-audio-default-output = Varsayılan çıkış (duyduğunuz)
properties-audio-default-input = Varsayılan giriş
properties-audio-default-suffix = (varsayılan)
properties-audio-current-device = (mevcut aygıt: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Kazanç
audiofilters-name-noise-gate = Gürültü Kapısı
audiofilters-name-compressor = Kompresör
audiofilters-name-limiter = Limitleyici
audiofilters-name-eq = 3-Bant EQ
audiofilters-name-denoise = Gürültü Azaltma
audiofilters-name-ducking = Ducking
audiofilters-name-parametric-eq = Parametrik EQ
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Gümbürtü filtresi
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Ön ayar
audiofilters-voice-preset-pick = Ses ön ayarı…
audiofilters-voice-broadcast = Yayın sesi
audiofilters-voice-podcast = Podcast sesi
audiofilters-voice-clean = Temiz ses
audiofilters-voice-none = Zinciri temizle
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Tıslama frekansı (Hz)
audiofilters-deesser-amount = Maks. azaltma (dB)
audiofilters-rumble-freq = Düşük kesim (Hz)
audiofilters-title = Ses filtreleri — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Canlı spektrumlu parametrik EQ tepki eğrisi
eq-band-type = Tür
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Bant
eq-remove-band = Bandı kaldır
eq-type-bell = Bell
eq-type-lowShelf = Low shelf
eq-type-highShelf = High shelf
eq-type-notch = Notch
eq-type-highPass = Yüksek geçiren
eq-type-lowPass = Alçak geçiren
audiofilters-chain-header = Filtre zinciri (üst önce çalışır, fader'dan önce)
audiofilters-add = + Filtre ekle
audiofilters-add-menu = Bir ses filtresi ekle
audiofilters-empty = Henüz filtre yok — bir mikrofonu gürültüden arındır (klasik DSP, ML yok), odayı kapıla, tepeleri kompresörle yumuşat ya da müziği sesinin altına duck et.
audiofilters-enable = { $name } etkinleştir
audiofilters-run-earlier = Daha önce çalıştır
audiofilters-move-up = { $name } yukarı taşı
audiofilters-run-later = Daha sonra çalıştır
audiofilters-move-down = { $name } aşağı taşı
audiofilters-remove-title = Filtreyi kaldır
audiofilters-remove = { $name } kaldır
audiofilters-gain-db = Kazanç (dB)
audiofilters-open-db = Açılma (dB)
audiofilters-close-db = Kapanma (dB)
audiofilters-attack-ms = Atak (ms)
audiofilters-hold-ms = Tutma (ms)
audiofilters-release-ms = Bırakma (ms)
audiofilters-ratio = Oran (:1)
audiofilters-threshold-db = Eşik (dB)
audiofilters-output-gain-db = Çıkış kazancı (dB)
audiofilters-ceiling-db = Tavan (dB)
audiofilters-low-db = Alçak (dB)
audiofilters-mid-db = Orta (dB)
audiofilters-high-db = Yüksek (dB)
audiofilters-strength = Güç
audiofilters-denoise-note = Sahip olunan klasik-DSP spektral bastırma — sabit gürültü (fanlar, tıslama) düşerken konuşma geçer. ML yok, model yok, sözleşme gereği.
audiofilters-duck-under = Şunun altına duck et
audiofilters-ducking-trigger = Ducking tetik kaynağı
audiofilters-pick-trigger = (bir tetik seçin — örn. mikrofonunuz)
audiofilters-trigger-at-db = Tetikleme (dB)
audiofilters-duck-by-db = Duck miktarı (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Kroma Anahtarı
filters-name-color-key = Renk Anahtarı
filters-name-luma-key = Luma Anahtarı
filters-name-render-delay = İşleme Gecikmesi
filters-name-color-correction = Renk Düzeltme
filters-name-lut = LUT Uygula
filters-name-blur = Bulanıklaştır
filters-name-mask = Görüntü Maskesi
filters-name-sharpen = Keskinleştir
filters-name-scroll = Kaydır
filters-name-perspective = Perspektif
filters-name-fade-loop = Solma döngüsü
filters-name-crop = Kırp
filters-title = Filtreler — { $name }
filters-blend-mode = Karışım modu
filters-chain-header = Filtre zinciri (üst önce çalışır)
filters-add = + Filtre ekle
filters-add-menu = Bir filtre ekle
filters-empty = Henüz filtre yok — bir web kamerasına kroma anahtarı uygula, bir yakalamayı renk düzelt ya da bir kayan yazı kaydır.
filters-enable = { $name } etkinleştir
filters-run-earlier = Daha önce çalıştır
filters-move-up = { $name } yukarı taşı
filters-run-later = Daha sonra çalıştır
filters-move-down = { $name } aşağı taşı
filters-remove-title = Filtreyi kaldır
filters-remove = { $name } kaldır
filters-key-color-rgb = Anahtar renk (herhangi bir renk, RGB uzaklığı)
filters-similarity = Benzerlik
filters-smoothness = Yumuşaklık
filters-luma-min = Luma min (daha koyu pikseller çıkar)
filters-luma-max = Luma maks (daha parlak pikseller çıkar)
filters-delay = Gecikme (ms — yalnızca video, örn. sesle senkronize etmek için; 500 ile sınırlı)
filters-key-color = Anahtar renk
filters-spill = Taşma
filters-gamma = Gama
filters-brightness = Parlaklık
filters-contrast = Kontrast
filters-saturation = Doygunluk
filters-hue-shift = Ton kaydırma
filters-opacity = Opaklık
filters-cube-file = .cube dosyası
filters-amount = Miktar
filters-radius = Yarıçap
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galeri
filters-shader-gallery-pick = Bir ön ayar yükle…
filters-shader-gallery-grayscale = Gri tonlama
filters-shader-gallery-invert = Tersine çevir
filters-shader-gallery-scanlines = Tarama çizgileri
filters-shader-gallery-vignette = Vinyet
filters-shader-source = Shader kaynağı (WGSL)
filters-shader-hint = vec4 döndüren bir WGSL effect(uv, color, p, texel, time) yazın. Kaydırıcılar için parametreleri // @param name min max default ile işaretleyin. Geçersiz bir shader yok sayılır — kaynak, derlenene kadar filtresiz görüntülenir.
filters-name-bezier-mask = Bézier maskesi
filters-mask-editor-hint = Taşımak için bir noktayı sürükleyin, eklemek için çift tıklayın, kaldırmak için bir noktaya sağ tıklayın.
filters-mask-shape = Şekil
filters-mask-shape-pick = Ön ayar…
filters-mask-shape-rectangle = Dikdörtgen
filters-mask-shape-diamond = Elmas
filters-mask-shape-hexagon = Altıgen
filters-mask-shape-circle = Daire
filters-mask-feather = Yumuşatma
filters-mask-export-wipe = Silme olarak dışa aktar…
filters-mask-image = Maske görüntüsü
filters-mask-mode = Mod
filters-mask-alpha = alfa
filters-mask-luma = luma
filters-mask-invert = tersine çevir
filters-speed-x = Hız X (px/s)
filters-speed-y = Hız Y (px/s)
filters-tilt = Eğim
filters-far-fade = Uzak kenar solması
filters-fade-in-s = Belirme (sn)
filters-visible-s = Görünür (sn)
filters-fade-out-s = Solma (sn)
filters-hidden-s = Gizli (sn)
filters-crop-left = sol
filters-crop-top = üst
filters-crop-right = sağ
filters-crop-bottom = alt
filters-crop-aria = kırp { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Yenile
pickershell-refresh-title = Listeyi yenile
pickershell-close = Kapat


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Hata bildir
bugreport-intro = Raporlar anonimdir ve isteğe bağlıdır — hiçbir şey otomatik gönderilmez. Aşağıdaki tam metni gözden geçirir, sonra önceden doldurulmuş bir GitHub sorunu veya e-posta uygulamanız aracılığıyla gönderirsiniz. Kişisel veri yok (ev yolunuz ve kullanıcı adınız gizlenir); hesap yok, sunucu yok.
bugreport-crash-notice = Freally Capture önceki bir çalıştırmada beklenmedik şekilde kapandı — anonim çökme ayrıntıları aşağıda yer alır. Bunları bildirmek hızlı düzeltmeye yardımcı olur.
bugreport-description-label = Bu olduğunda ne yapıyordunuz? (isteğe bağlı)
bugreport-description-placeholder = örn. ikinci bir web kamerası eklediğimde önizleme dondu
bugreport-include-crash = Son çalıştırmadan anonim çökme ayrıntılarını dahil et
bugreport-preview-label = Tam olarak ne gönderilecek
bugreport-open-github = GitHub sorunu aç
bugreport-gmail-title = Gmail'in oluşturma penceresini tarayıcınızda önceden doldurulmuş olarak açar. Oturum kapalı mı? Google önce giriş ekranını gösterir.
bugreport-compose-gmail = Gmail'de oluştur
bugreport-email-title = Bu bilgisayarın varsayılan olarak kullandığı posta uygulamasında bir taslak açar (Outlook, Thunderbird, Mail…)
bugreport-send-email = E-posta gönder
bugreport-copied = Kopyalandı ✓
bugreport-copy-report = Raporu kopyala
bugreport-dismiss-crash = Çökmeyi kapat
bugreport-copy-failed = kopyalanamadı — metni seçip elle kopyalayın
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = NE OLDU
bugreport-preview-no-description = (açıklama sağlanmadı)
bugreport-preview-diagnostics = ANONİM TANILAMA (kişisel veri yok)
bugreport-preview-from = Kimden: Freally Capture
bugreport-preview-crash-excerpt = --- çökme alıntısı ---


# --- Updates.tsx ---
updates-title = Yazılım güncellemesi
updates-checking = Güncellemeler denetleniyor…
updates-uptodate = En son sürümü kullanıyorsunuz.
updates-check-again = Yeniden denetle
updates-available = Sürüm { $version } kullanılabilir
updates-current-version = ({ $current } sürümündesiniz)
updates-release-notes-label = Sürüm { $version } — Sürüm notları
updates-confirm = Şimdi güncellemek ister misiniz? İndirilen, uygulanmadan önce paketlenmiş imzalama anahtarına karşı doğrulanır. Freally Capture kapanır, yükleyici çalışır ve yeni sürüm kendiliğinden yeniden açılır.
updates-yes-update-now = Evet, şimdi güncelle
updates-no-not-now = Hayır, şimdi değil
updates-downloading = { $version } indiriliyor…
updates-starting = başlatılıyor…
updates-installed = Güncelleme yüklendi.
updates-restart-now = Şimdi yeniden başlat
updates-restart-later = Sonra yeniden başlat
updates-try-again = Yeniden dene


# --- Models.tsx ---
models-title = Bileşenler
models-ffmpeg-heading = FFmpeg — wire codec'leri
models-badge-third-party = Üçüncü taraf · paketlenmez
models-ffmpeg-desc = Freally Capture'ın kendi motoru, kayıpsız freally-video (.frec) kaydını fazladan hiçbir şey olmadan yapar. Platformların ve oynatıcıların beklediği wire biçimlerini kaydetmek — mp4/mkv/mov/webm içinde H.264/AAC (ve HEVC/AV1) — bu uygulamanın asla birlikte gelmediği ayrı bir araç olan FFmpeg'i kullanır: bu codec'ler patentle yüklüdür, bu yüzden isteğe bağlı ve açıkça etiketli kalır. Aşağıdaki sabitlenmiş yapıdan istek üzerine indirilir, ilk kullanımdan önce SHA-256 ile doğrulanır, kullanıcı başına önbelleğe alınır ve ayrı bir işlem olarak çalıştırılır. Lisansı (LGPL/GPL) kendisine aittir — bkz. THIRD-PARTY-NOTICES.
models-checking = Kontrol ediliyor…
models-ffmpeg-not-installed = Yüklü değil. Mevcut: { $source } kaynağından FFmpeg { $version } ({ $size } indirme).
models-ffmpeg-none-pinned = Bu platform için henüz sabitlenmiş bir FFmpeg yapısı yok — wire-codec kaydı burada kullanılamaz. Kayıpsız freally-video kaydı etkilenmez.
models-ffmpeg-download-verify = İndir ve doğrula ({ $size })
models-downloading = İndiriliyor…
models-download-of = /
models-cancel = İptal
models-ffmpeg-verifying = İndirilen, sabitlenmiş SHA-256'ya karşı doğrulanıyor…
models-ffmpeg-extracting = Açılıyor…
models-ffmpeg-ready = Yüklendi ve doğrulandı — { $version }
models-remove = Kaldır
models-ffmpeg-retry = İndirmeyi yeniden dene
models-network-note = İndirme, bu paneldeki tek ağ işlemidir ve asla kendiliğinden başlamaz. Başarısız bir sağlama toplamı yüklemeyi iptal eder — uygulama, garanti edemediği baytları çalıştırmayı reddeder.
models-cef-heading = Tarayıcı Kaynağı çalışma zamanı — Chromium (CEF)
models-cef-desc = Tarayıcı kaynakları web sayfalarını (uyarılar, widget'lar, katmanlar) Chromium Embedded Framework üzerinden işler — bu uygulamanın asla birlikte gelmediği ~100 MB'lık bir çalışma zamanı. İstek üzerine resmi CEF yapı dizininden indirilir, herhangi bir şey açılmadan önce o dizinin SHA-1'ine karşı doğrulanır ve kullanıcı başına önbelleğe alınır. Onunla işlenen tarayıcı kaynağı kendi aşamasıyla gelir; bu, ihtiyaç duyduğu çalışma zamanını yükler.
models-cef-download-install = İndir ve yükle
models-cef-unsupported = CEF bu platform için yapı yayımlamıyor — tarayıcı kaynakları burada kullanılamaz.
models-cef-resolving = En son kararlı yapı çözümleniyor…
models-cef-verifying = İndirilen, dizin SHA-1'ine karşı doğrulanıyor…
models-cef-extracting = Çalışma zamanı açılıyor…
models-cef-ready = Yüklendi — CEF { $version }.
models-cef-retry = Yeniden dene
models-integrations-heading = İsteğe bağlı entegrasyonlar
models-badge-never-bundled = Asla paketlenmez
models-ndi-detected = Algılandı
models-ndi-not-installed = Yüklü değil
models-vst-available = Kullanılabilir
models-vst-not-available = Kullanılamıyor


# --- Recordings.tsx ---
recordings-title = Kayıtlar
recordings-loading = Klasör okunuyor…
recordings-empty = Henüz kayıt yok — Kaydı Başlat, Çıkış'ta ayarlanan klasöre yazar.
recordings-frec-label = sahip olunan kayıpsız (freally-video)
recordings-remux-title = mp4 olarak yeniden paketle — akış kopyası, yeniden kodlama yok, kalite değişikliği yok (FFmpeg bileşenini gerektirir)
recordings-trim = Kırp
recordings-trim-title = Bu kayıttan bir klip kesin — anahtar kareye hizalı kesimler yeniden kodlanmadan dışa aktarılır
recordings-verify = Doğrula
recordings-verify-title = Dosyanın bütünlüğünü denetle — kap yapısı, süreklilik, A/V serpiştirme, süre
recordings-verifying = Doğrulanıyor…
verify-dismiss = Kapat
verify-verdict-pass = { $name } — bütünlük sağlam
verify-verdict-warn = { $name } — uyarılarla doğrulandı
verify-verdict-fail = { $name } — sorunlar bulundu
verify-container = Kap
verify-video-continuity = Video sürekliliği
verify-audio-continuity = Ses sürekliliği
verify-av-interleave = A/V serpiştirme
verify-duration = Süre
recordings-alpha-label = alfa
recordings-prores-title = Alfayı koruyan ProRes 4444 .mov master dışa aktar (kurgu için)
recordings-qtrle-title = Alfayı koruyan QuickTime Animation .mov dışa aktar (azami uyumluluk)
trim-title = Kırp — { $name }
trim-loading = Dosya okunuyor…
trim-preview-alt = Önizleme karesi
trim-position = Oynatma konumu
trim-step-second-back = Bir saniye geri
trim-step-frame-back = Bir kare geri
trim-step-frame-forward = Bir kare ileri
trim-step-second-forward = Bir saniye ileri
trim-snap = Anahtar kare
trim-snap-title = En yakın anahtar kareye hizala — oradaki kesim yeniden kodlanmadan dışa aktarılır
trim-set-in = Giriş noktası
trim-set-out = Çıkış noktası
trim-range-invalid = Çıkış noktası giriş noktasından sonra olmalı.
trim-copy-badge = ✓ Yeniden kodlanmadan dışa aktarılır — giriş noktası bir anahtar karede.
trim-reencode-badge = Yeniden kodlanacak: giriş noktası anahtar kareler arasında ("Anahtar kare" ile kayıpsız kesime hizalayın).
trim-export = Klibi dışa aktar
trim-export-916 = 9:16
trim-export-916-title = Yeniden çerçevelenmiş dikey dışa aktarım (dikey tuval boyutunda ortalanmış kırpma) — her zaman yeniden kodlar
recordings-remuxing = Yeniden paketleniyor…
recordings-remux-to-mp4 = MP4'e yeniden paketle
recordings-export-mp4-title = Sahip olunan .frec'i çöz ve MP4'e (H.264/AAC) yeniden kodla, böylece her oynatıcıda oynar — FFmpeg bileşenini gerektirir
recordings-exporting = Dışa aktarılıyor…
recordings-export-mp4 = Dışa aktar → MP4
recordings-export-mkv-title = Sahip olunan .frec'i çöz ve MKV'ye yeniden kodla, böylece her oynatıcıda oynar
recordings-starting = başlatılıyor…
recordings-frames = { $done } / { $total } kare
recordings-cancel = İptal
recordings-export-cancelled = Dışa aktarma iptal edildi.
recordings-exported-to = { $path } konumuna dışa aktarıldı
recordings-remuxed-to = { $path } konumuna yeniden paketlendi
recordings-normalize = Normalleştir
recordings-normalizing = Normalleştiriliyor…
recordings-normalize-title = Ses düzeyini hedefe normalleştir (bir kopya yazar)
recordings-normalized-to = { $path } konumuna normalleştirildi

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Yalnızca ses
audiorec-format = Ses kaydı biçimi
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Sesi kaydet
audiorec-stop = Durdur
audiorec-pause = Duraklat
audiorec-resume = Sürdür
audiorec-recording = REC { $sec }s
audiorec-saved = { $count } iz dosyası kaydedildi


# --- OpenedFrec.tsx ---
openfrec-title = .frec kaydını aç
openfrec-desc = Freally Capture, sahip olunan kayıpsız .frec biçimini kaydeder — onu oynatmaz. Freally Player, yayımlandığında .frec'i doğrudan oynatacak. Şimdilik onu MP4/MKV'ye dışa aktarın, her oynatıcıda (VLC, işletim sistemi oynatıcınız, herhangi biri) oynar.
openfrec-exported-to = { $path } konumuna dışa aktarıldı
openfrec-exporting = Dışa aktarılıyor…
openfrec-starting = başlatılıyor…
openfrec-export-mp4 = Dışa aktar → MP4
openfrec-export-mkv = Dışa aktar → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Dikey tuval (9:16)
vertical-enable = İkinci tuvali etkinleştir — programdan bağımsız olarak kaydedilebilir ve yayınlanabilir
vertical-scene-label = Bu tuvalin oluşturduğu sahne
vertical-width = Genişlik
vertical-height = Yükseklik
vertical-preview-alt = Dikey tuval önizlemesi
vertical-note = Öğe konumları tuvaller arasında piksel-doğru: bu önizleme dikey sonucu gösterirken düzenlemek için Sahneler rayında bu sahneyi seçin. Yayın hedefleri bu tuvali ⦿ Yayın…'da seçer; Ayarlar → Çıkış onu ana dosyanın yanında kaydedebilir.
vertical-close = Kapat


# --- EulaGate.tsx ---
eula-title = Freally Capture — Lisans Sözleşmesi
eula-version = v{ $version }
eula-intro = Freally Capture'ı kullanmak için lütfen bu sözleşmeyi okuyup kabul edin. Kısacası: bu tarafsız bir araçtır ve yakaladığınız, kaydettiğiniz ve yayınladığınız şeylerden — ve bunlara ilişkin haklara sahip olmaktan — yalnızca siz sorumlusunuz.
eula-thanks = Okuduğunuz için teşekkürler.
eula-scroll-hint = Devam etmek için sona kaydırın.
eula-decline = Reddet ve Çık
eula-agree = Kabul Ediyorum


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Çıkış
output-loading = Ayarlar hâlâ yükleniyor…
output-container-frec = freally-video (.frec) — kayıpsız, sahip olunan, indirilecek bir şey yok
output-container-mkv = MKV — çökmeye dayanıklı; sonra mp4'e yeniden paketle
output-container-mp4 = MP4 — her yerde oynar
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Kayıpsız
output-preset-lossless-title = Sahip olunan freally-video codec'i — bit-tam, indirme yok
output-preset-high-label = Yüksek kalite
output-preset-high-title = MP4, en iyi algılanan kodlayıcı, neredeyse kayıpsız CQ 16, Kalite ön ayarı
output-preset-balanced-label = Dengeli
output-preset-balanced-title = MKV, en iyi algılanan kodlayıcı, CQ 23, Dengeli ön ayar
output-recording-format = Kayıt biçimi
output-ffmpeg-warning = Bu biçim FFmpeg bileşenini gerektirir (wire codec'leri — paketlenmez). Kayıpsız .frec hiçbir şey gerektirmez.
output-install = Yükle…
output-recordings-folder = Kayıtlar klasörü
output-folder-placeholder = İşletim sistemi Videolar klasörü
output-filename-prefix = Dosya adı öneki
output-recording-template = Kayıt dosya adı
output-replay-template = Tekrar dosya adı
output-still-template = Kare dosya adı
output-template-tokens = Belirteçler: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Tekrar klasörü
output-still-folder = Kare klasörü
output-same-folder-placeholder = Kayıtlar klasörü
output-frame-rate = Kare hızı
output-fps-option = { $fps } fps
output-split-every = Şu sürede böl (dakika, 0 = kapalı)
output-output-width = Çıkış genişliği (0 = tuval; yalnızca wire biçimleri)
output-output-height = Çıkış yüksekliği (0 = tuval)
output-record-vertical = Dikey tuvali de kaydet (paralel bir “… (vertical)” dosyası; 9:16 tuvalinin etkin olmasını gerektirir)
output-audio-tracks = Ses izleri
output-recorded-tracks-group = Kaydedilen izler
output-track-last-one = En az bir iz kaydedilmelidir
output-record-track-on = { $index } numaralı izi kaydet: açık
output-record-track-off = { $index } numaralı izi kaydet: kapalı
output-encoder-heading = Kodlayıcı
output-video-encoder = Video kodlayıcı
output-encoder-auto = Otomatik — en iyi algılanan (H.264)
output-encoder-unavailable = — burada kullanılamıyor
output-preset = Ön ayar
output-preset-quality = Kalite
bench-open = Kodlayıcı benchmark'ını çalıştır…
bench-title = Kodlayıcı benchmark'ı
bench-intro = Bu makinede kısa ölçümlü kodlama merdivenleri çalıştırır (algılanan her kodlayıcı × ön ayar × çözünürlük) — yaklaşık bir dakika, tamamen çevrimdışı, hiçbir şey bilgisayarından çıkmaz. Hatalar listelenir, asla gizlenmez. Önce yayını veya kaydı durdur.
bench-start = Benchmark'ı başlat
bench-rerun = Yeniden çalıştır
bench-running = Ölçülüyor… { $done } / { $total }
bench-cancel = İptal
bench-col-encoder = Kodlayıcı
bench-col-preset = Ön ayar
bench-col-rung = Basamak
bench-col-achieved = fps
bench-col-headroom = Pay
bench-failed = başarısız
bench-rec-title = Öneri (ölçülmüş)
bench-rec-body = { $encoder }, { $preset } ile { $width }×{ $height } @ { $fps } fps — ölçülen { $headroom }× gerçek zaman. Önerilen yayın bitrate'i: { $bitrate } kbps.
bench-rec-none = Bu makinede hiçbir ayar gerçek zamanı sürdüremiyor — tuval çözünürlüğünü veya fps'i düşürüp yeniden ölç.
bench-apply = Kayıt ayarlarına uygula
bench-applied = Uygulandı ✓
output-preset-balanced-option = Dengeli
output-preset-performance = Performans
output-rate-control = Hız denetimi
output-rc-cqp = CQP (sabit kalite)
output-rc-cbr = CBR (sabit bit hızı)
output-rc-vbr = VBR (değişken bit hızı)
output-cq = CQ (0–51, düşük = daha iyi)
output-bitrate = Bit hızı (kbps)
output-keyframe = Anahtar kare aralığı (s)
output-audio-bitrate = Ses bit hızı (kbps / iz)
output-iso-heading = ISO kaydı
output-iso-explainer = Seçili kaynakları temiz olarak, her birini programın yanında kendi dosyasına kaydedin — birleştirme öncesinde, tuval boyutu ve kare hızında; böylece her dosya kurgu zaman çizelgesine hizalı düşer. Orta sınıf bir GPU'da iki şerit rahattır; her ek şerit bir render ve bir kodlama daha demektir.
output-iso-none = Koleksiyonda henüz kaynak yok.
output-iso-source-on = "{ $name }" kendi ISO dosyasına kaydediliyor — durdurmak için tıklayın
output-iso-source-off = "{ $name }" kaynağını kendi ISO dosyasına kaydet
output-iso-post-filter = Kaynağın filtreleriyle kaydet (filtre sonrası); işaretlenmezse ham kaynak kaydedilir
output-iso-format = ISO biçimi
output-iso-encoder = ISO video kodlayıcı
output-alpha-frec = Saydamlıkla kaydet (alfa) — program saydam bir arka plan üzerinde
output-alpha-title = Kaydedici kendi saydam render'ını alır; önizleme ve yayın normal kalır. Kayıt listesinden ProRes 4444 veya QTRLE'ye aktarın — MP4/MKV alfayı düzleştirir.
output-split-events = Şunlarda da yeni dosya başlat… (her bölüm tam olayda başlar; asgari uzunluk 1 sn)
output-split-on-scene = sahne geçişi
output-split-on-marker = işaretçi
output-split-on-rundown = akış adımı
output-auto-markers = Stüdyo olaylarında bölüm işaretçilerini otomatik bırak (sahne geçişi, tekrar kaydı, yeniden bağlanma, düşen kareler, alarmlar, kurallar)
output-auto-markers-title = Türlü işaretçiler kaydın bölümlerine (mkv) veya .chapters.txt yan dosyasına, elle işaretçi kısayolunun yanına yazılır
output-pipeline-heading = Kayıt sonrası hattı
output-pipeline-explainer = Bir kayıt tamamlanınca bu adımlar ana dosya üzerinde sırayla, arka planda çalışır. Kapalı bir eylem kümesi — bilerek "komut çalıştır" adımı yoktur. Zincir ilk hatada durur.
output-pipeline-enabled = Her kayıttan sonra hattı çalıştır
output-pipeline-add = Adım ekle…
output-pipeline-up = Yukarı taşı
output-pipeline-down = Aşağı taşı
output-pipeline-remove = Adımı kaldır
output-pipeline-template = Yeniden adlandırma şablonu (CAP-M25 belirteçleri)
output-pipeline-folder = Klasör
pipeline-queue = Kayıt sonrası hattı
pipeline-verify = Doğrula
pipeline-remux = MP4'e remux
pipeline-normalize = Ses düzeyini normalleştir
pipeline-rename = Yeniden adlandır
pipeline-move = Klasöre taşı
pipeline-copy = Klasöre kopyala
pipeline-reveal = Dosya yöneticisinde göster
pipeline-luaEvent = Lua betiklerine bildir
output-presets = Ön ayarlar:

# --- SettingsStream.tsx ---
stream-title = Ayarlar — Yayın
stream-target-enabled = Hedef { $index } etkin
stream-target = Hedef { $index }
stream-remove = Kaldır
stream-service = Hizmet
stream-canvas = Tuval
stream-canvas-main = Ana (program)
stream-canvas-vertical = Dikey (9:16 — stüdyoda etkinleştirin)
stream-ingest-srt = SRT alım URL'si
stream-ingest-whip = WHIP uç nokta URL'si
stream-ingest-url = Alım URL'si
stream-ingest-override = (geçersiz kıl — boş = hizmet ön ayarı)
stream-key-srt = streamid (isteğe bağlı — ?streamid=… olarak eklenir; gizli olarak ele alınır)
stream-key-whip = Bearer belirteci (isteğe bağlı — Authorization başlığı olarak gönderilir; bir gizli)
stream-key-custom = Yayın anahtarı (sunucunuzdan — gizli olarak ele alınır)
stream-key-service = Yayın anahtarı (içerik üretici panonuzdan — gizli olarak ele alınır)
stream-key-aria = Yayın anahtarı { $index }
stream-key-hide = Gizle
stream-key-show = Göster
stream-encoder = Kodlayıcı (H.264 — RTMP, SRT ve WHIP'in hepsinin taşıdığı)
stream-encoder-auto = Otomatik — en iyi algılanan H.264 kodlayıcı
stream-encoder-unavailable = (burada kullanılamıyor)
stream-video-bitrate = Video bit hızı (kbps, CBR)
stream-audio-bitrate = Ses bit hızı (kbps)
stream-fps = FPS
stream-keyframe = Anahtar kare aralığı (s)
stream-audio-track = Ses izi (1–6)
stream-output-width = Çıkış genişliği (0 = tuval)
stream-output-height = Çıkış yüksekliği (0 = tuval)
stream-add-target = + Hedef ekle
stream-go-live-note = Yayına Başla, etkin her hedefe aynı anda, doğrudan her platforma yayınlar. Aynı kodlayıcı ayarlarına sahip hedefler tek bir kodlamayı paylaşır.
stream-auto-record = Yayına geçtiğimde kaydı başlat (kayıt yine de bağımsız olarak durur)
stream-session-report = Oturum bitince kaydın yanına bir oturum raporu (HTML + Markdown) yaz
stream-simulator-title = Ağ simülatörü (provalar)
stream-simulator-note = Yalnızca provanın yerel alıcılarını şekillendirir — yeniden bağlanmaları ve zayıf hatları çalıştırır. Gerçek bir yayın asla etkilenmez.
stream-simulator-profile = Profil
stream-simulator-off = Kapalı
stream-simulator-hotel-wifi = Otel Wi-Fi
stream-simulator-mobile-hotspot = Mobil erişim noktası
stream-simulator-custom = Özel
stream-simulator-bandwidth = Bant genişliği (kbps, 0 = sınırsız)
stream-simulator-latency = Gecikme (ms)
stream-simulator-jitter = Titreşim (± ms)
stream-simulator-outage-every = Kesinti aralığı (sn, 0 = asla)
stream-simulator-outage-len = Kesinti süresi (sn)
stream-ffmpeg-note-before = Yayın wire codec'leri, etiketli isteğe bağlı ffmpeg bileşeni üzerinden çalışır —
stream-ffmpeg-note-link = buradan yönetin
stream-ffmpeg-note-after = . Yerel kayıt, yayın ne yaparsa yapsın çalışmaya devam eder.
stream-cancel = İptal
stream-save = Kaydet

# --- SettingsReplay.tsx ---
replay-title = Ayarlar — Tekrar Arabelleği
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 dk
replay-length-2min = 2 dk
replay-length-5min = 5 dk
replay-quality-low = Düşük (3 Mbps)
replay-quality-standard = Standart (6 Mbps)
replay-quality-high = Yüksek (12 Mbps)
replay-length-presets = Uzunluk ön ayarları
replay-quality-presets = Kalite ön ayarları
replay-length-seconds = Uzunluk (saniye)
replay-video-bitrate = Video bit hızı (kbps)
replay-fps = FPS
replay-audio-track = Ses izi (1–6)
replay-note = Etkinken, arabellek kendi hafif kodlamasını sınırlı bir disk üstü halka içine çalıştırır — bu ayarlarda yaklaşık { $mb } MB. Kaydetme, halkayı yeniden kodlamadan birleştirir ve yayına veya kayda asla dokunmaz. Değişiklikler bir sonraki etkinleştirmede uygulanır.
replay-cancel = İptal
replay-save = Kaydet

# --- SettingsRemote.tsx ---
remote-title = Ayarlar — Uzaktan Denetim
remote-enable = WebSocket uzak API'sini etkinleştir
remote-password = Parola (gerekli — denetleyiciler bununla kimlik doğrular)
remote-password-placeholder = denetleyicileriniz için bir parola
remote-password-hide = Gizle
remote-password-show = Göster
remote-port = Bağlantı noktası
remote-allow-lan = LAN bağlantılarına izin ver (varsayılan yalnızca bu makine)
remote-note = Kapalı = bağlantı noktası kapalıdır. Açık = 127.0.0.1 üzerinde (ya da etkinleştirdiğinizde LAN'ınızda) parola korumalı bir WebSocket; sahne değiştirebilir, geçişi çalıştırabilir, yayını ve kaydı başlatıp durdurabilir, tekrarları kaydedebilir ve sessize alma/ses düzeylerini ayarlayabilir — arayüzle aynı eylemler, fazlası değil. Dosyaları okuyamaz. Parolayı herhangi bir kimlik bilgisi gibi ele alın; başka bir aygıttan özellikle denetlemiyorsanız yalnızca-bu-makine tercih edin.
remote-password-required = Uzak API'yi etkinleştirmek için bir parola gerekir.
remote-cancel = İptal
remote-save = Kaydet

# --- SettingsHotkeys.tsx ---
hotkeys-title = Ayarlar — Kısayollar
hotkeys-record = Kaydı başlat / durdur
hotkeys-go-live = Yayına Başla / Yayını Bitir
hotkeys-transition = Stüdyo-Modu Geçişi
hotkeys-save-replay = Tekrarı Kaydet (son N saniye)
hotkeys-add-marker = Bölüm işaretçisi bırak (kayıt)
hotkeys-note = Kısayollar geneldir — diğer uygulamalar odaktayken çalışır. Boş = bağlanmamış. Mikser bas-konuş/sustur tuşları her şeridin ⋯ menüsündedir. Linux/Wayland'de genel kısayollar kullanılamayabilir (bir birleştirici sınırı) — düğmeler çalışmaya devam eder.
hotkeys-cancel = İptal
hotkeys-save = Kaydet

# --- WorkspaceDialog.tsx ---
workspace-title = Profiller ve Sahne Koleksiyonları
workspace-profiles = Profiller
workspace-profiles-hint = Bir profil sizin ayarlarınızdır — yayın hedefi, çıkış, kısayollar. Gösteri veya platform başına değiştirin.
workspace-collections = Sahne koleksiyonları
workspace-collections-hint = Bir koleksiyon sizin sahneleriniz + kaynaklarınızdır. Oluştur, mevcut olanı başlangıç noktası olarak çoğaltır.
workspace-active = Etkin
workspace-switch-to = { $name } geç
workspace-active-marker = ● etkin
workspace-new-name-placeholder = yeni ad…
workspace-new-name-label = Yeni { $title } adı
workspace-create = Oluştur

# --- OBS import (CAP-M02) ---
workspace-import-obs = OBS'den içe aktar…
workspace-import-obs-hint = Bir OBS sahne koleksiyonunu (scenes.json dosyasını) içe aktarın. Mevcut koleksiyonunuz önce kaydedilir.
workspace-import-busy = İçe aktarılıyor…
workspace-import-title = "{ $name }" içe aktarıldı
workspace-import-summary = { $scenes } sahne · { $sources } kaynak · { $items } öğe
workspace-import-dismiss = Kapat
workspace-import-clean = Her şey sorunsuz aktarıldı.
workspace-import-geometry-caveat = Boyutlar ve konumlar OBS düzeninden uyarlanır — her sahneyi gözden geçirin ve yakalama aygıtlarını yeniden seçin.
workspace-import-notes-title = Notlarla içe aktarıldı
workspace-import-skipped-title = İçe aktarılmadı
import-note-needsReselect = Aygıtı/monitörü/pencereyi yeniden seçin
import-note-gameCaptureAsWindow = Oyun Yakalama → Pencere Yakalama
import-note-referencesFile = Dosya yolunu kontrol edin
import-note-filterDropped = Bazı filtreler desteklenmiyor
import-note-geometryApproximated = Konum/boyut yaklaşık
import-skip-unsupportedKind = Eşdeğer kaynak türü yok
import-skip-group = Gruplar henüz desteklenmiyor

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Eksik dosyaları yeniden bağla…
doctor-title = Eksik dosyalar
doctor-scanning = Taranıyor…
doctor-all-good = Başvurulan tüm dosyalar mevcut. Yeniden bağlanacak bir şey yok.
doctor-intro = Başvurulan { $count } dosya bu bilgisayarda bulunamadı. Her birine yeni konumunu gösterin — onu kullanan her sahne aynı anda düzeltilir.
doctor-relinked = { $count } başvuru yeniden bağlandı.
doctor-uses = { $count }× kullanıldı
doctor-locate = Bul…
doctor-locate-folder = Klasörde ara…
doctor-locate-folder-hint = Bir klasör seçin; her eksik dosya ada göre bulunup yeniden bağlanır.
doctor-kind-image = görüntü
doctor-kind-media = ortam
doctor-kind-slideshow = slayt gösterisi
doctor-kind-font = yazı tipi
doctor-kind-lut = LUT
doctor-kind-mask = maske
history-relinkFiles = Dosyaları yeniden bağla

# --- ScriptsDialog.tsx ---
scripts-title = Betikler (Lua)
scripts-empty = Henüz betik yok — bir .lua dosyası ekleyin. API için scripts/sample.lua dosyasına bakın: yayına-geçme/sahne/kayıt olaylarına tepki verin ve uzak API ile aynı komutları çalıştırın.
scripts-enable = { $path } etkinleştir
scripts-remove = { $path } kaldır
scripts-path-label = Betik yolu
scripts-add = Ekle
scripts-note = Betikler yalıtılmış çalışır — dosya veya işletim sistemi erişimi yok; yalnızca uzak API ile aynı stüdyo komutlarını çağırabilirler (sahne değiştirme, geçiş, kayıt/yayın/tekrar, sessize almalar). Bir betik hatası günlüğe kaydedilir ve sınırlanır. Değişiklikler bir saniye içinde uygulanır.
scripts-error-not-lua = Bir .lua dosyasını gösterin.

# --- BrowserDock.tsx ---
browser-dock-title = Tarayıcı panelleri
browser-dock-empty = Henüz panel yok — bir sohbet açılır penceresi, bir uyarılar sayfası veya Companion web düğmelerinizi ekleyin.
browser-dock-open = Aç
browser-dock-remove = { $name } kaldır
browser-dock-name-placeholder = ad (örn. Twitch Sohbeti)
browser-dock-name-label = Panel adı
browser-dock-url-label = Panel URL'si
browser-dock-note = Bir panel, stüdyonun yanına yerleştirebileceğiniz kendi penceresi olarak açılır. Sayfanın uygulamaya erişimi yoktur — yalnızca işler. Yalnızca http(s) URL'leri; paneller yalnızca Aç'a tıkladığınızda açılır.
browser-dock-error-name = Panele bir ad verin (örn. Twitch Sohbeti).
browser-dock-error-url = Bir panel URL'si http:// veya https:// ile başlamalıdır.

# --- studio-preview-pane ---
studio-preview-label = Stüdyo Modu önizlemesi
studio-preview-heading = Önizleme
studio-preview-hint = buraya yüklemek için bir sahneye tıklayın
studio-preview-empty = Önizleme burada görünecek.
studio-preview-mirrors = programı yansıtır
studio-preview-transition-select = Geçiş
studio-preview-duration = Geçiş süresi (ms)
studio-preview-commit-title = Önizleme → Program geçişini uygula (izleyiciler görür)
studio-preview-transitioning = Geçiş yapılıyor…
studio-preview-transition-button = Geçiş ⇄
studio-preview-luma-placeholder = gri tonlamalı silme görüntüsü (png/jpg)
studio-preview-luma-label = Luma silme görüntüsü
studio-preview-browse = Gözat…
studio-preview-filter-images = Görüntüler
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = stinger videosu (ProRes 4444 .mov alfa kanalını korur)
studio-preview-stinger-label = Stinger video dosyası
studio-preview-stinger-cut-label = Stinger kesme noktası (ms)
studio-preview-stinger-cut-title = Sahne değişiminin stinger altında gerçekleştiği an (geçişin başından itibaren ms)
studio-preview-stinger-matte-label = Track matte
studio-preview-stinger-matte-title = Track matte'li bir stinger şeffaflığı nasıl paketler: dolgu ve matte'i yan yana (yatay) veya üst üste (dikey)
studio-preview-stinger-duck-label = Programı duck et
studio-preview-stinger-duck-title = Çalarken program sesini stinger'ın kendi sesinin altına duck et (0 = kapalı)
studio-preview-stinger-duck-unit = dB

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Kesme
transition-kind-fade = Solma
transition-kind-slide-left = Kaydırma ←
transition-kind-slide-right = Kaydırma →
transition-kind-slide-up = Kaydırma ↑
transition-kind-slide-down = Kaydırma ↓
transition-kind-swipe-left = Süpürme ←
transition-kind-swipe-right = Süpürme →
transition-kind-luma-linear = Luma silme (doğrusal)
transition-kind-luma-radial = Luma silme (ışınsal)
transition-kind-luma-horizontal = Luma silme (yatay)
transition-kind-luma-diamond = Luma silme (elmas)
transition-kind-luma-clock = Luma silme (saat)
transition-kind-image = Görüntü silme (özel)
transition-kind-stinger = Stinger (video)
transition-kind-move = Taşı (morph)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Yok
stinger-matte-horizontal = Yan yana
stinger-matte-vertical = Üst üste

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Özel (RTMP/RTMPS)
stream-service-srt = SRT (kendi barındırılan)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Hakkında
about-tagline = Bir stüdyo gibi kaydedin ve yayınlayın — hesap yok, bulut yok.
about-version = Sürüm
about-created-by = Oluşturan
about-project-started = Proje başlangıcı
about-first-stable = İlk kararlı sürüm
about-first-stable-pending = Henüz değil — 1.0.0 üzerinde çalışılıyor
about-platform = Platform
about-local-first = Freally Capture tamamen sizin makinenizde çalışır. Hesap yok, telemetri yok, bulut yok — bilgisayarınızdan çıkan tek şey göndermeyi seçtiğiniz yayındır.
about-website = Web sitesi
about-issues = Sorun bildir
about-license = Lisans
about-eula = EULA
about-third-party = Üçüncü taraf bildirimleri
about-check-updates = Güncellemeleri denetle…

# --- unified settings modal (TASK-906) ---
settings-title = Ayarlar
settings-language-section = Dil
settings-language = Arayüz dili
settings-language-system = Sistem varsayılanı
settings-language-note = Burada seçtiğiniz dil hatırlanır. “Sistem varsayılanı” işletim sisteminizi izler. Çevrilmemiş metin İngilizceye geri döner.
settings-appearance-section = Görünüm
settings-theme = Tema
settings-theme-dark = Koyu
settings-theme-light = Açık
settings-theme-custom = Özel
settings-accent = Vurgu
settings-general-section = Genel
settings-show-stats-dock = İstatistik panelini göster
settings-open-about = Hakkında…

# --- command palette (TASK-904) ---
palette-title = Komut paleti
palette-search = Sahneleri, kaynakları ve eylemleri ara
palette-placeholder = Sahneleri, kaynakları, eylemleri ara…
palette-no-results = “{ $query }” ile eşleşen bir şey yok
palette-hint = ↑ ↓ taşımak için · Enter çalıştırmak için · Esc kapatmak için
palette-group-scenes = Sahne
palette-group-sources = Kaynak
palette-group-actions = Eylem
palette-transition = Önizleme → Program geçişi
palette-save-replay = Tekrarı kaydet
palette-add-marker = Bölüm işaretçisi bırak
palette-vertical-canvas = Dikey (9:16) tuval…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Freally Capture'a hoş geldiniz
wizard-welcome = İki hızlı adım: makinenizin neler yapabileceğine bir bakın, sonra bir sahne başlatın. Yaklaşık otuz saniye sürer ve her şeyi sonradan değiştirebilirsiniz.
wizard-local-first = Burada hiçbir şey bilgisayarınızdan çıkmaz. Freally Capture'da hesap, telemetri ve bulut yoktur.
wizard-start = Başlayalım
wizard-skip = Atla
wizard-hardware-title = Makineniz neler yapabilir
wizard-probing = Ekran kartınız ve işlemciniz kontrol ediliyor…
wizard-encoder = Kodlayıcı
wizard-canvas = Tuval
wizard-bitrate = Bit hızı
wizard-probe-found = Bulundu: { $gpus } · { $cores } fiziksel çekirdek
wizard-no-gpu = ayrı GPU yok
wizard-apply = Bu ayarları kullan
wizard-keep-current = Olduğu gibi bırak
wizard-template-title = Bir sahneyle başlayın
wizard-template-screen = Ekranımı yakala
wizard-template-screen-note = Ana monitörünüzün bir Ekran Yakalama'sını ekler. Başlamak için en yaygın yer.
wizard-template-empty = Boş başla
wizard-template-empty-note = Boş bir sahne. Kaynakları + düğmesiyle kendiniz ekleyin.
wizard-done = Her şey hazır.
wizard-done-hint = Sahneleri, kaynakları ve eylemleri aramak için istediğiniz zaman Ctrl+K'ye basın. Ayarlar ⚙ düğmesinin arkasındadır.
wizard-close = Yayına başla

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Ekran kartınız videoyu kendi başına kodlayabiliyor, bu da işlemciyi stüdyonun geri kalanı için serbest bırakıyor.
autoconfig-reason-software = Kullanılabilir bir donanım kodlayıcı bulunamadı, bu yüzden kodlamayı işlemci yapacak — çalışır, yalnızca daha fazla CPU harcar.
autoconfig-reason-quality-hardware = Her büyük platformun kabul ettiği bir bit hızında, saniyede 60 karede 1080p.
autoconfig-reason-quality-software = Saniyede 30 kare, çünkü 60 karede yazılım kodlaması çoğu işlemcide kare düşürür.
autoconfig-reason-quality-low-cores = Daha düşük bir bit hızı, çünkü bu işlemcinin çekirdeği az ve yazılım kodlaması bunlar için birleştiriciyle yarışacak.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Kayıt başladı
announce-recording-paused = Kayıt duraklatıldı
announce-recording-stopped = Kayıt durduruldu
announce-live-started = Yayındasınız
announce-live-ended = Yayın sona erdi
announce-reconnecting = Bağlantı koptu, yeniden bağlanılıyor
announce-stream-failed = Yayın başarısız oldu
announce-frames-dropped = { $count } kare düştü

# CAP-M01 — undo/redo edit history
palette-undo = Geri al
palette-redo = Yinele
palette-edit-history = Düzenleme geçmişi…
history-title = Düzenleme geçmişi
history-empty = Henüz geri alınacak bir şey yok.
history-current = Geçerli durum
history-close = Kapat
history-addScene = Sahne ekle
history-renameScene = Sahneyi yeniden adlandır
history-removeScene = Sahneyi kaldır
history-reorderScene = Sahneleri yeniden sırala
history-addSource = Kaynak ekle
history-removeSource = Kaynağı kaldır
history-reorderSource = Kaynakları yeniden sırala
history-renameSource = Kaynağı yeniden adlandır
history-transformSource = Kaynağı taşı
history-toggleVisibility = Görünürlüğü değiştir
history-toggleOutputVisibility = Çıkış görünürlüğünü değiştir
history-toggleLock = Kilidi değiştir
history-setBlendMode = Karışım modunu değiştir
history-editSourceProperties = Özellikleri düzenle
history-applyLayout = Düzeni yerleştir
history-moveToSeat = Yerine taşı
history-groupSources = Kaynakları grupla
history-ungroupSources = Grubu çöz
history-toggleGroupVisibility = Grubu değiştir
history-setSceneAudio = Sahne sesi
history-setVerticalCanvas = Dikey tuval
history-addFilter = Filtre ekle
history-removeFilter = Filtreyi kaldır
history-reorderFilter = Filtreleri yeniden sırala
history-editFilter = Filtreyi düzenle
history-toggleFilter = Filtreyi değiştir
history-setVolume = Sesi ayarla
history-toggleMute = Sessizi değiştir
history-setMonitor = İzlemeyi değiştir
history-setTracks = Parçaları değiştir
history-setSyncOffset = A/V senkronunu ayarla
history-setAudioHotkeys = Ses kısayolları

# CAP-M04 — alignment aids
settings-alignment-section = Hizalama yardımcıları
settings-smart-guides = Akıllı kılavuzlar (sürüklerken yapış)
settings-safe-areas = Güvenli alan katmanları
settings-rulers = Cetveller
align-group = Tuvale hizala
align-left = Sola hizala
align-hcenter = Yatayda ortala
align-right = Sağa hizala
align-top = Üste hizala
align-vcenter = Dikeyde ortala
align-bottom = Alta hizala

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Seçimi hizala ve dağıt
arrange-left = Sol kenarları hizala
arrange-hcenter = Yatayda ortala
arrange-right = Sağ kenarları hizala
arrange-top = Üst kenarları hizala
arrange-vcenter = Dikeyde ortala
arrange-bottom = Alt kenarları hizala
distribute-h = Yatay dağıt
distribute-v = Dikey dağıt
guides-group = Kılavuzlar
guides-add-v = Dikey kılavuz ekle
guides-add-h = Yatay kılavuz ekle
guides-clear = Tüm kılavuzları temizle
history-arrangeItems = Öğeleri düzenle
history-editGuides = Kılavuzları düzenle

# CAP-M05 — edit transform + copy/paste
transform-title = Dönüşümü düzenle — { $name }
transform-anchor = Sabit nokta
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Döndürme
transform-crop = Kırpma
transform-crop-left = Sol
transform-crop-top = Üst
transform-crop-right = Sağ
transform-crop-bottom = Alt
transform-no-size = Boyut ve kırpma, kaynak boyutlarını bildirdiğinde kullanılabilir olur.
transform-copy = Dönüşümü kopyala
transform-paste = Dönüşümü yapıştır
transform-close = Kapat
filters-copy = Filtreleri kopyala ({ $count })
filters-paste = Filtreleri yapıştır ({ $count })
palette-edit-transform = Dönüşümü düzenle…
history-pasteFilters = Filtreleri yapıştır

# CAP-M26 — keying workbench
workbench-title = Anahtarlama tezgahı — { $name }
workbench-mode-keyed = Anahtarlı
workbench-mode-source = Kaynak
workbench-mode-matte = Mat
workbench-mode-split = Bölünmüş
workbench-eyedropper = Damlalık
workbench-eyedropper-hint = Anahtar rengini örneklemek için kaynağa tıklayın.
workbench-loupe = Büyüteç
workbench-split = Bölme
workbench-preview-alt = Anahtarlama tezgahı önizlemesi
workbench-tune = Ayarla
workbench-close = Kapat

# CAP-M06 — multiview monitor
multiview-title = Çoklu görünüm
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Bir sahneye geçmek için ona tıklayın.
multiview-hint-stage = Bir sahneyi önizlemede hazırlamak için ona tıklayın.
palette-multiview = Çoklu görünüm monitörü

# CAP-M07 — projectors
projector-title = Projektör aç
projector-source = Kaynak
projector-target-program = Program
projector-target-preview = Önizleme
projector-target-scene = Sahne…
projector-target-source = Kaynak…
projector-target-multiview = Çoklu görünüm
projector-which-scene = Hangi sahne
projector-which-source = Hangi kaynak
projector-none = Gösterilecek bir şey yok
projector-display = Ekran
projector-windowed = Yüzen pencere (bu ekran)
projector-display-option = Ekran { $n } — { $w }×{ $h }
projector-primary = (birincil)
projector-open = Aç
projector-cancel = İptal
projector-exit-hint = Çıkmak için Esc'e basın
palette-projector = Projektör aç…

# CAP-M08 — still-frame grab
palette-still = Kare yakala…
still-saved-toast = Kare kaydedildi: { $name }
still-failed-toast = Kare yakalama başarısız: { $error }
hotkeys-still = Kare yakala

# CAP-M13 — source health dashboard
palette-source-health = Kaynak sağlığı…
palette-av-sync = A/V senkron kalibrasyonu…
palette-hotkey-audit = Kısayol haritası…
health-title = Kaynak Sağlığı
health-col-source = Kaynak
health-col-state = Durum
health-col-resolution = Çözünürlük
health-col-fps = FPS
health-col-last-frame = Son kare
health-col-dropped = Bırakılan
health-col-retries = Yeniden başlatmalar
health-col-actions = Eylemler
health-state-live = Canlı
health-state-waiting = Bekliyor
health-state-error = Hata
health-state-inactive = Devre dışı
health-restart = Yeniden başlat
health-properties = Özellikler
health-empty = Bu koleksiyonda henüz kaynak yok.
health-seconds = { $value } sn

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture'dan çıkılsın mı?
quit-body = Şimdi çıkmak sırasıyla şunları güvenle yapar:
quit-consequence-stream = Canlı yayını sonlandırır ve hizmet bağlantısını keser.
quit-consequence-recording = Kaydı durdurur ve dosyalarını sonlandırır.
quit-consequence-replay = Tekrar arabelleğini kapatır — kaydedilmemiş görüntüler atılır.
quit-confirm = Güvenle çık
quit-quitting = Kapatılıyor…
quit-cancel = İptal

# CAP-M11 — crash-safe recording salvage
salvage-title = Kesintiye uğrayan kayıtlar kurtarılsın mı?
salvage-body = Son oturum, bu kayıtlar hâlâ yazılırken beklenmedik şekilde sona erdi. Onarım, orijinalin yanına oynatılabilir bir kopya yazar — orijinal dosya asla değiştirilmez.
salvage-repair = Onar
salvage-repairing = Onarılıyor…
salvage-done = Onarıldı
salvage-repaired = Onarıldı → { $name }
salvage-failed = Onarım başarısız: { $error }
salvage-dismiss = Şimdi değil

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Kodlayıcı arızası — { $from } yerine { $to } kullanılıyor. Yayın yeniden bağlandı ve sürüyor.
fallback-toast-recording = Kodlayıcı arızası — { $from } yerine { $to } kullanılıyor. Kayıt yeni bir dosyada devam ediyor.
fallback-note = Yedek kodlayıcı: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Program sesi sessizleşti
alarm-clipping = Program sesi kırpılıyor
alarm-black = Program görüntüsü siyah
alarm-frozen = Program görüntüsü bir süredir değişmedi
alarm-lowDisk = Disk alanı: mevcut bit hızında yaklaşık { $minutes } dk kaldı
alarm-dismiss = Alarmı kapat
alarm-cleared = Çözüldü: { $alarm }

# CAP-M22 — panic button
palette-panic = Panik — gizlilik ekranına geç
panic-banner-title = Panik
panic-banner-body = Program gizlilik ekranını gösteriyor; tüm ses kapalı ve yakalamalar durdu. Yayın ve kayıt sürüyor.
panic-restore = Geri yükle…
panic-restore-confirm = Program geri yüklensin mi?
panic-restore-yes = Geri yükle
panic-restore-cancel = İptal
hotkeys-panic = Panik (gizlilik ekranı)
hotkeys-timer-toggle = Tüm zamanlayıcıları başlat/duraklat
hotkeys-timer-reset = Tüm zamanlayıcıları sıfırla
panic-slate-color = Panik ekranı rengi
panic-slate-image = Panik ekranı görseli
panic-slate-image-placeholder = İsteğe bağlı görsel yolu

# CAP-M24 — redacted diagnostics bundle
diag-title = Tanılama paketi
diag-intro = GitHub issue'suna elle eklemek için ayıklanmış bir .zip (yapılandırma anlık görüntüsü, kodlayıcı sondası, son istatistikler — gizli bilgiler, yollar ve adlar asla dahil edilmez) dışa aktarır. Hiçbir şey gönderilmez.
diag-preview = İçeriği gör
diag-hide-preview = Önizlemeyi gizle
diag-export = .zip dışa aktar
diag-exported = Dışa aktarıldı: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Yayın öncesi kontrol
preflight-intro = Engelleyen her madde yeşil olmalı; kalanlar dürüst hatırlatmadır.
preflight-item-targets = Yayın hedefleri ayarlı (anahtar/URL)
preflight-item-encoder = Kullanılabilir kodlayıcı var
preflight-item-sources = Tüm kaynaklar sağlıklı
preflight-item-disk = Kayıt için disk alanı
preflight-item-mic = Mikrofon ölçümü
preflight-item-desktopAudio = Masaüstü ses ölçümü
preflight-item-replay = Tekrar arabelleği hazır
preflight-targets-detail = { $count } etkin
preflight-sources-detail = { $count } kaynak hatalı
preflight-disk-detail = Mevcut bit hızında ~{ $minutes } dk
preflight-fix-stream = Yayın ayarları…
preflight-fix-components = Bileşenler…
preflight-fix-sources = Kaynak sağlığı…
preflight-fix-replay = Hazırla
preflight-optional = isteğe bağlı
preflight-hold = Hepsi yeşil olana dek Go Live'ı beklet
preflight-cancel = İptal
preflight-go-anyway = Yine de yayına geç
preflight-go-live = Yayına geç


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Arka plan
scenes-backdrop-aria = { $name } arka planı
backdrop-title = Arka plan — { $name }
backdrop-hint = Bu sahnede her şeyin arkasına sabitlenen bir duvar kâğıdı — bir görsel, hareketli bir GIF veya döngüde bir video. Yakalamanız her zaman üsttedir; yakınlaştırmak için tuval üzerinde kaydırın.
backdrop-choose = Görsel veya video seç…
backdrop-remove = Arka planı kaldır
backdrop-none = Arka plan ayarlanmadı.
backdrop-position = Konum
backdrop-split-full = Tüm tuval
backdrop-split-left = Sol yarı
backdrop-split-right = Sağ yarı
backdrop-split-top = Üst yarı
backdrop-split-bottom = Alt yarı
backdrop-sync = Kayıt başlayınca oynatmayı başlat
backdrop-sync-hint = Kayda başlayana dek ilk karede bekler; her çekim videoyu baştan başlatır.
backdrop-preview-play = Önizlemeyi oynat
backdrop-preview-pause = Önizlemeyi duraklat
backdrop-filter-all = Arka planlar (görseller ve video)
backdrop-filter-images = Görseller
backdrop-filter-media = Video ve GIF
sources-backdrop-badge = Arka plan duvar kâğıdı (en alta sabit)
sources-backdrop-pinned = Arka plan en altta sabit kalır
filters-name-flip = Çevir
filters-flip-horizontal = Yatay
filters-flip-vertical = Dikey
history-setSceneBackdrop = Arka planı ayarla
history-setBackdropSplit = Arka planı taşı
history-setBackdropSync = Arka plan kayıt eşitlemesi
backdrop-scrub = Oynatma konumu
backdrop-loop = Döngü
backdrop-reverse = Tersten oynat
backdrop-reverse-hint = Ters oynatma bir kez ters bir kopya oluşturur (videolar ffmpeg bileşenini gerektirir; GIF'ler anında tersine döner) — uzun dosyalarda ilk geçiş zaman alabilir.
filters-scaling = Ölçekleme
filters-scaling-hint = Retro/piksel içerik için piksel hassasiyetli modlar; Tamsayı ayrıca çizilen boyutu tam katlara oturtur (tutamaçlar mantıksal boyutu gösterir).
filters-scaling-auto = Yumuşak
filters-scaling-nearest = En yakın komşu
filters-scaling-integer = Tamsayı (tam ×)
filters-scaling-sharp = Keskin çift doğrusal
history-setScaling = Ölçeklemeyi değiştir
hotkeys-zoom-100 = Yakınlaştırma: sıfırla (%100)
hotkeys-zoom-150 = Yakınlaştırma: %150'ye yaklaş
hotkeys-zoom-200 = Yakınlaştırma: 2× yaklaş
sources-follow-title = Yakınken imleci izle (Windows; yakınlaştırmak için tuval üzerinde kaydırın)
sources-follow-item = { $name } için imleç izlemeyi aç/kapat
filters-autocrop = ✂ Siyah bantları otomatik kırp
filters-autocrop-title = Sonraki kareyi letterbox/pillarbox bantları için tarar ve kırpar (geri alınabilir). Karanlık sahneler asla kırpılmaz.
filters-autocrop-follow = Çözünürlük değişince yeniden denetle
history-autoCrop = Siyah bantları otomatik kırp
sources-link-audio = Bu uygulamanın sesini de yakala (bağlı: gizlemek susturur, pencereyi kaldırmak onu da kaldırır)
history-addLinkedWindow = Pencere + bağlı ses ekle
sources-hdr-title = Bu ekran HDR — ton eşlemeyi aç (tuval SDR kalır)
sources-hdr-item = { $name } için HDR ton eşleme
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Bu ekran HDR veriyor. Ton eşleme olmadan parlak alanlar kırpılır ve yakalama SDR tuvalde soluk görünür. Değişiklikler sonraki karede uygulanır.
sources-hdr-enable-suggested = Önerileni etkinleştir (maxRGB, 200 nit)
sources-hdr-operator = Operatör
sources-hdr-op-clip = Kırpma (kapalı)
sources-hdr-op-maxrgb = maxRGB (tonu korur)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 dizi (SDR birebir)
sources-hdr-paper-white = Kağıt beyazı
sources-hdr-nits = nit
projector-target-passthrough = Geçiş monitörü (düşük gecikme)
projector-which-device = Aygıt
projector-passthrough-none = Önce bir ekran, pencere veya yakalama aygıtı ekleyin.
projector-passthrough-about = Ham aygıt kareleri — sahne yok, filtre yok, birleştirici yok. Ölçülen gecikmeyi gösterir; ses yine mikser kanalından dinlenir.
projector-passthrough-hint = Geçiş — Esc kapatır
projector-latency = { $ms } ms
projector-latency-measuring = ölçülüyor…
automation-title = Otomasyon — kurallar, makrolar ve değişkenler
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Kurallar
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Açık
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Şu olduğunda
automation-then-run = şunu çalıştır
automation-no-macro = (no macro)
automation-macros = Makrolar
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Çalıştır
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Stüdyo değişkenleri
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
rundown-title = Yayın akışı
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Başlat
rundown-next = Sonraki ▸
rundown-stop = Durdur
rundown-idle = Çalışmıyor
rundown-next-up = Sırada: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Adım
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
automation-layer = Katman
automation-layer-hint = Yalnızca bu katman etkinken çalışır (boş = tüm katmanlar). Katmanlar yapışkandır: katman tuşu değiştirir ve öyle kalır (işletim sistemi API'si basılı-tut katmanı sunmaz).
automation-chord-hint = Düz bir tuş (Ctrl+Shift+M) veya iki vuruşlu akor (Ctrl+K, 3). Akorun ikinci tuşu yalnızca akor beklerken ayrılır.
panel-title = LAN paneli ve tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Paneli yayınla
panel-port = Port
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Parola
panel-show = Göster
panel-hide = Gizle
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Kaydet
osc-title = OSC kontrol yüzeyi
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = OSC dinle
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ kameralar
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Kamera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Adres
ptz-port = Bağlantı noktası
ptz-speed = Hız
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
ptz-presets = Ön ayarlar
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Çağır
ptz-store = Kaydet
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI kontrol yüzeyi
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Giriş
midi-output = Çıkış (geri bildirim)
midi-none = (none)
midi-learn = Öğren
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Eylem
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
panel-lan-warning = ⚠ LAN trafiği şifreli değil — parola URL’de düz HTTP ile gider. Yalnızca güvendiğiniz ağda kullanın.
osc-lan-warning = ⚠ OSC’nin parolası yok — ağdaki her cihaz bu komutları gönderebilir. LAN modunu yalnızca güvendiğiniz ağda kullanın.

# System-stats HUD source (CAP-N14)
sources-badge-stats = İstat.
sources-add-system-stats = Performans İstatistikleri (HUD)
sources-stats-title = Performans HUD'u ekle
sources-stats-note = Stüdyonun gerçekten ölçülen sayılarını izleyicileriniz için programda gösterir — fps, CPU, bellek, render süresi, düşen kareler ve canlı bit hızı. Hangi satırların görüneceği, boyut ve renk kaynağın Özellikler'indedir. GPU kullanımı ölçülmediği için gösterilmez.
sources-stats-add = İstatistik HUD'u ekle
properties-stats-show-fps = FPS'yi göster
properties-stats-show-cpu = CPU'yu göster
properties-stats-show-memory = Belleği göster
properties-stats-show-render = Render süresini göster
properties-stats-show-dropped = Düşen kareleri göster
properties-stats-show-bitrate = Bit hızını göster
properties-stats-show-timecode = Zaman kodunu göster (LTC)
properties-stats-size = Boyut (px)
properties-stats-note = HUD, kompakt evrensel etiketleri (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) doğrudan programa çizer; yayın yokken bit hızı satırı “—” gösterir.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Görselleştirici
sources-add-visualizer = Ses Görselleştirici
sources-visualizer-title = Ses görselleştirici ekle
sources-visualizer-style-label = Stil
sources-visualizer-style-bars = Spektrum çubukları
sources-visualizer-style-scope = Osiloskop
sources-visualizer-style-vu = VU metreler
sources-visualizer-target-label = Dinlediği
sources-visualizer-target-master = Ana miks
sources-visualizer-target-track = Kanal { $n }
sources-visualizer-note = Gerçekten mikslenen sinyali çizer (fader sonrası) — susturulmuş bir kaynak, tıpkı duyulduğu gibi düz görünür. Boyut, renk, çubuk sayısı ve düşme hızı kaynağın Özellikler'indedir.
sources-visualizer-add = Görselleştirici ekle
properties-vis-bands = Çubuklar
properties-vis-decay = Düşme hızı (dB/s)
properties-vis-peak-hold = Tepe tutma işaretleri
properties-vis-missing-source = (kaynak eksik)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splitler
sources-add-split-timer = Speedrun Split Zamanlayıcısı
sources-splits-title = Split zamanlayıcısı ekle
sources-splits-file-label = LiveSplit .lss dosyası
sources-splits-comparison-label = Karşılaştır
sources-splits-comparison-pb = Kişisel rekor
sources-splits-comparison-best = En iyi bölümler
sources-splits-comparison-average = Ortalama
sources-splits-note = Dosya salt okunur içe aktarılır — asla geri yazılmaz. Ayarlar → Kısayollar'dan genel Split / Undo / Skip / Reset tuşlarını atayın. İşlem belleği üzerinden otomatik split'leyiciler bilerek desteklenmez.
sources-splits-add = Split zamanlayıcısı ekle
properties-splits-size = Boyut (px)
properties-splits-ahead = Önde
properties-splits-behind = Geride
properties-splits-gold = Altın
properties-splits-split = Split
properties-splits-undo = Geri al
properties-splits-skip = Atla
properties-splits-reset = Sıfırla
properties-splits-note = Düğmeler canlı zamanlayıcıyı sürer (genel kısayollar her uygulamadan aynısını yapar). Koşu asla .lss dosyasına yazılmaz.
hotkeys-split-split = Split zamanlayıcı: başlat / split
hotkeys-split-undo = Split zamanlayıcı: split'i geri al
hotkeys-split-skip = Split zamanlayıcı: bölümü atla
hotkeys-split-reset = Split zamanlayıcı: sıfırla
hotkey-audit-action-split-split = Split (split zamanlayıcı)
hotkey-audit-action-split-undo = Split'i geri al
hotkey-audit-action-split-skip = Bölümü atla
hotkey-audit-action-split-reset = Split zamanlayıcıyı sıfırla
hotkey-audit-feature-split-timer = Split zamanlayıcı

# Media playlist source (CAP-N17)
sources-badge-playlist = Çalma listesi
sources-add-playlist = Medya Çalma Listesi (kesintisiz)
sources-playlist-title = Medya çalma listesi ekle
sources-playlist-files-label = Dosyalar (satır başına bir, yukarıdan aşağı çalınır)
sources-playlist-browse = Göz at…
sources-playlist-loop = Döngü
sources-playlist-shuffle = Karıştır (her başlangıçta bir çekiliş; döngüde aynı sıra tekrarlanır)
sources-playlist-hold-last = Sonda son kareyi tut
sources-playlist-note = Kırpılmış listenin tamamını etiketli ffmpeg bileşeniyle kesintisiz çalar (yalnızca wire biçimleri — .frec ve görseller Medya/Slayt gösterisiyle). Öğeler ya hep video ya hep ses olur, asla karışmaz. Kırpmalar, cue noktaları ve «now playing» değişkeni Özellikler'dedir.
sources-playlist-add = Çalma listesi ekle
properties-playlist-items = Öğeler (yukarıdan aşağı)
properties-playlist-up = Yukarı
properties-playlist-down = Aşağı
properties-playlist-remove = Öğeyi kaldır
properties-playlist-in = Başlangıç (sn)
properties-playlist-out = Bitiş (sn)
properties-playlist-cues = Cue (sn, virgülle)
properties-playlist-add-item = + Öğe ekle
properties-playlist-loop = Döngü
properties-playlist-shuffle = Karıştır
properties-playlist-hold-last = Son kareyi tut
properties-playlist-hw = Donanımsal çözme
properties-playlist-variable = «Now playing» değişkeni (boş = kapalı)
properties-playlist-previous = ⏮ Önceki
properties-playlist-next = ⏭ Sonraki
properties-playlist-note = Cue düğmeleri ve Sonraki/Önceki, ÇALAN listeyi sürer; öğe değişiklikleri Uygula ile geçerli olur (liste yeniden başlar). Çalan öğenin adını göstermek için bir Metin kaynağına {"{{"}yourVariable{"}}"} koyun.
hotkeys-playlist-next = Çalma listesi: sonraki öğe
hotkeys-playlist-previous = Çalma listesi: önceki öğe
hotkey-audit-action-playlist-next = Çalma listesi sonraki
hotkey-audit-action-playlist-previous = Çalma listesi önceki
hotkey-audit-feature-playlist = Çalma listesi

# Instant replay source (CAP-N10)
sources-badge-replay = Tekrar
sources-add-replay = Anında Tekrar
sources-replay-title = Anında tekrar ekle
sources-replay-seconds-label = Roll uzunluğu (saniye)
sources-replay-speed-label = Hız
sources-replay-speed-full = %100 (sesli)
sources-replay-speed-half = %50 ağır çekim (sessiz)
sources-replay-speed-quarter = %25 ağır çekim (sessiz)
sources-replay-note = Roll edene kadar saydam kalır. Tekrar arabelleğini kur (Kontroller) ve Roll tuşunu ata — roll, arabelleğin son anlarını keser, programda oynatır ve sonra saydamlığa döner.
sources-replay-add = Anında tekrar ekle
properties-replay-roll = ⏵ Tekrarı başlat
properties-replay-note = Roll, KURULU arabelleği bir klibe keser ve seçilen hızda oynatır — yeniden zamanlanır, asla ara kare üretilmez. Ağır çekim bilerek sessizdir. Oynatma sırasında sarma ve duraklatma çalışır; sonunda kaynak saydamlığa döner.
hotkeys-replay-roll = Anında tekrar: başlat
hotkey-audit-action-replay-roll = Anında tekrarı başlat

# Input overlay source (CAP-N13)
sources-badge-input = Giriş
sources-add-input-overlay = Giriş katmanı (tuşlar/oyun kolu)
sources-input-title = Giriş katmanı ekle
sources-input-layout-label = Düzen
sources-input-layout-wasd = WASD + fare
sources-input-layout-keyboard = Kompakt klavye + fare
sources-input-layout-gamepad = Oyun kolu (çift analog)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Tuşlar
sources-input-accent-label = Basılı
sources-input-privacy-note = Gizlilik: giriş yalnızca bu kaynak bir sahnede canlıyken okunur ve yalnızca düzenin sabit tuşları sorgulanır — anlık bir "şu an basılı mı?" bakışı, asla bir hook değil. Hiçbir şey günlüğe yazılmaz, saklanmaz veya bir yere gönderilmez; yazılan metin asla yakalanmaz.
sources-input-os-note = Klavye ve fare durumu bugün yalnızca Windows'ta okunur — diğer sistemler tuşları basılmamış çizer (dürüstçe söylenir, asla taklit edilmez). Oyun kolları gilrs kitaplığıyla her yerde çalışır; bağlı ilk kumanda çizilir, kumanda yoksa düzen basılmamış kalır.
sources-input-add = Giriş katmanı ekle

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = İmleç efektleri
filters-cursorfx-hint = Windows'ta (imleci uygulamanın kendisi çizer) doğrudan yakalamanın içine çizilir; böylece kayıtlarda ve yayınlarda görünür. macOS ve Linux imleci sistem tarafında birleştirir, bu yüzden bu efektler yalnızca Windows içindir. Değişiklikler anında uygulanır.
filters-cursorfx-halo = İmleç halesi
filters-cursorfx-halo-color = Renk
filters-cursorfx-halo-radius = Yarıçap (px)
filters-cursorfx-ripples = Tıklama halkaları
filters-cursorfx-left-color = Sol tık
filters-cursorfx-right-color = Sağ tık
filters-cursorfx-keystrokes = Tuş gösterimi
filters-cursorfx-keystrokes-hint = Basılı tutuldukça sabit bir tuş kümesini (harfler, rakamlar, değiştiriciler, oklar) imlecin yanında gösterir. Tuşlar yalnızca bu açıkken okunur, doğrudan kareye çizilir ve asla saklanmaz ya da günlüğe yazılmaz.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Başlık
sources-add-title = Başlık / Skor tabelası
sources-title-title = Başlık ekle
sources-title-template-label = Şununla başla
sources-title-template-lower-third = Alt bant (çubuk + ad + alt yazı)
sources-title-template-scoreboard = Skor tabelası (plaka + 4 hücre)
sources-title-template-blank = Boş tuval
sources-title-width-label = Tuval genişliği
sources-title-height-label = Tuval yüksekliği
sources-title-template-name = Ad
sources-title-template-subtitle = Unvan
sources-title-template-home = EV SAHİBİ
sources-title-template-away = KONUK
sources-title-note = Giriş/çıkış animasyonlu katmanlı başlıklar (metin / görsel / kutu), yerelde birleştirilir — tarayıcı kaynağı yok. Katmanlar, dosya bağlantıları ve {"{{"}değişkenler{"}}"} ile canlı kontroller kaynağın Özellikler'inde.
sources-title-add = Başlık ekle
properties-title-layers = Katmanlar (sırayla çizilir — sonraki satırlar üstte)
properties-title-kind-text = Metin
properties-title-kind-image = Görsel
properties-title-kind-rect = Kutu
properties-title-x = X
properties-title-y = Y
properties-title-outline = Kontur (px)
properties-title-outline-color = Kontur
properties-title-shadow = Gölge
properties-title-animation = Giriş/çıkış animasyonu
properties-title-anim-none = Yok (kesme)
properties-title-anim-fade = Solma
properties-title-anim-slide-left = Sola kaydır
properties-title-anim-slide-up = Yukarı kaydır
properties-title-anim-wipe = Silme
properties-title-duration = Süre (ms)
properties-title-fire-in = ▶ Girişi başlat
properties-title-fire-out = ◼ Çıkışı başlat
properties-title-set-live = Canlıya yaz
properties-title-set-live-note = Bu metni ŞİMDİ canlı başlığa gönderir — Uygula yok, yeniden başlatma yok
properties-title-up = Katmanı yukarı taşı
properties-title-down = Katmanı aşağı taşı
properties-title-remove = Katmanı kaldır
properties-title-add-text = + Metin
properties-title-add-image = + Görsel
properties-title-add-rect = + Kutu
properties-title-note = Giriş/çıkış ve "Canlıya yaz" ÇALIŞAN başlığı yönetir; katman düzenlemeleri Uygula ile geçerli olur (başlık yeniden başlar ve tekrar girer). Metin hücreleri izlenen bir dosyaya bağlanabilir (CSV hücresi / JSON değeri / tüm dosya) ve {"{{"}değişkenleri{"}}"} açar — "Canlıya yaz" ikisinden de önce gelir.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN alımı (SRT/RTMP dinleyici)
sources-lan-title = LAN alım dinleyicisi ekle
sources-lan-protocol-label = Protokol
sources-lan-protocol-srt = SRT (şifrelenebilir — önerilir)
sources-lan-protocol-rtmp = RTMP (kimlik doğrulaması yok)
sources-lan-port-label = Bağlantı noktası (1024–65535)
sources-lan-passphrase-label = Parola (boş = açık)
sources-lan-passphrase-hint = SRT parolaları 10–79 karakterdir; gönderen aynısını kullanmalı.
sources-lan-open-warning = Parola yok: bu ağdaki herkes bu kaynağı şifresiz besleyebilir. Ağ yalnızca sizin değilse bir parola belirleyin.
sources-lan-rtmp-warning = RTMP'de kimlik doğrulaması yoktur — bu ağdaki herkes bu bağlantı noktasına gönderebilir. Parolalı SRT'yi tercih edin.
sources-lan-url-label = Gönderenin uygulamasını şuna yöneltin
sources-lan-qr-aria = Alım URL'sinin QR kodu
sources-lan-note = Yalnızca LAN: bu makinenin yerel adresinde, yalnızca kaynak var olduğu sürece dinler ve internete asla dokunmaz — ağınızdaki bir gönderen önce göndermedikçe makineden hiçbir şey çıkmaz. Kod çözme, açıkça etiketlenmiş ffmpeg bileşeni üzerinden yürür. Bir gönderen bağlanana dek tuval bu URL'yi gösterir.
sources-lan-add = Dinlemeye başla
properties-lan-note = Protokol, bağlantı noktası veya parola değişikliğini uygulamak dinleyiciyi yeniden başlatır — gönderen yeniden bağlanmalıdır. Akış 1920×1080 tuvale sığdırılır.

# Freally Link source & output (CAP-N12)
sources-badge-link = Bağlantı
sources-add-freally-link = Freally Link (başka bir kopya)
sources-link-title = Freally Link ekle
sources-link-about = Başka bir Freally Capture kopyasının programını — video ve master ses — kendi ağınız üzerinden alır. Önce gönderen kopyada "Freally Link çıkışı"nı açın. v1, TCP üzerinden motion-JPEG aktarır: kablolu LAN veya iyi Wi-Fi'de harika, zayıf bağlantılarda bant genişliği konusunda dürüst.
sources-link-scan = LAN'ı tara
sources-link-scanning = Taranıyor…
sources-link-none = Freally Link çıkışı bulunamadı. Diğer kopyada "Freally Link çıkışı"nı açın (Kontroller → LAN paneli) veya adresini aşağıya yazın.
sources-link-host = Adres
sources-link-port = Bağlantı noktası
sources-link-key = Eşleştirme anahtarı
sources-link-key-hint = Gönderenin "Freally Link çıkışı" ayarlarındaki anahtar — anahtar olmadan gönderen tek bir kare bile sunmaz.
sources-link-add = Bağlantı ekle
properties-link-note = Bağlantı yokken kaynak bir "bağlanıyor" yüzü gösterir ve artan bekleme ile kendi kendine yeniden dener — asla eski bir karede donmaz. Gönderici başına bir alıcı; meşgul bir gönderici kibarca yeniden denenir.
link-title = Freally Link çıkışı
link-about = Bu kopyanın programını — video ve master sesi — kendi ağınızdaki TEK bir başka Freally Capture ile paylaşın; orada "Freally Link" kaynağı olarak görünür (iki PC ile yayın, ek monitörler). Varsayılan olarak kapalı; siz açana dek hiçbir şey duyurulmaz veya dinlenmez. v1, TCP üzerinden motion-JPEG + sıkıştırılmamış ses aktarır — kablolu LAN veya iyi Wi-Fi için, asla internet için değil.
link-enable = Programı ağımda paylaş
link-name = Kopya adı
link-key = Eşleştirme anahtarı
link-key-hint = En az 8 karakter — alıcılar bu anahtarı girmeden tek bir kare bile sunulmaz.
link-lan-warning = ⚠ Alıcılar herhangi bir şey almadan önce eşleştirme anahtarını sunmalıdır, ancak akışın kendisi v1'de şifrelenmez — yalnızca güvendiğiniz bir ağda kullanın.
link-serving = Alıcılar bu kopyayı "LAN'ı tara" ile bulabilir veya elle şu adresten ekleyebilir:
link-off-hint = Bağlantı noktasını açmak ve bu kopyayı LAN taramalarına duyurmak için paylaşımı etkinleştirin.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Uygulama menüsü
menu-file = Dosya
menu-edit = Düzen
menu-view = Görünüm
menu-docks = Paneller
menu-profile = Profil
menu-collection = Sahne Koleksiyonu
menu-tools = Araçlar
menu-help = Yardım
menu-rename = Yeniden adlandır
menu-remove = Kaldır
menu-import = İçe aktar
menu-export = Dışa aktar
menu-file-show-recordings = Kayıtları göster
menu-file-remux = MP4'e yeniden paketle…
menu-file-settings = Ayarlar…
menu-file-show-settings-folder = Ayarlar klasörünü göster
menu-file-exit = Çıkış
menu-edit-undo = Geri al
menu-edit-redo = Yinele
menu-edit-history = Düzenleme geçmişi…
menu-edit-copy-transform = Dönüşümü kopyala
menu-edit-paste-transform = Dönüşümü yapıştır
menu-edit-copy-filters = Filtreleri kopyala
menu-edit-paste-filters = Filtreleri yapıştır
menu-edit-transform = Dönüşüm…
menu-edit-lock-preview = Önizlemeyi kilitle
menu-view-fullscreen = Tam ekran arayüz
menu-stats-dock = İstatistik paneli
menu-view-multiview = Çoklu görünüm monitörü…
menu-view-projectors = Projektörler…
menu-view-source-health = Kaynak sağlığı…
menu-view-still = Kare yakala
menu-docks-browser = Tarayıcı panelleri…
menu-docks-lock = Panelleri kilitle
menu-docks-reset = Panel düzenini sıfırla
menu-profile-manage = Profilleri yönet…
menu-collection-manage = Sahne koleksiyonlarını yönet…
menu-collection-import-obs = OBS'den içe aktar…
menu-collection-missing = Eksik dosyaları denetle…
menu-tools-wizard = Kurulum sihirbazını çalıştır
menu-tools-wizard-title = Kurulum sihirbazı ilk açılışta çalışır; yeniden çalıştırma yolu henüz yok.
menu-tools-automation = Otomasyon kuralları ve makrolar…
menu-tools-rundown = Yayın akışını göster…
menu-tools-hotkeys = Kısayol haritası…
menu-tools-av-sync = A/V senkron kalibrasyonu…
menu-tools-scripts = Lua betikleri…
menu-tools-components = Bileşenler…
menu-tools-midi = MIDI denetimi…
menu-tools-ptz = PTZ kameralar…
menu-tools-remote = Uzaktan denetim API'si…
menu-tools-panel = LAN paneli ve tally…
menu-help-portal = Yardım portalı
menu-help-website = Web sitesini ziyaret et
menu-help-discord = Discord sunucusuna katıl
menu-help-bug = Hata bildir…
menu-help-updates = Güncellemeleri denetle…
menu-help-whats-new = Yenilikler
menu-help-about = Hakkında…
menu-help-more-apps = Diğer Freally uygulamaları…
moreapps-title = Diğer Freally uygulamaları

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Ayar kategorileri
settings-cat-general = Genel
settings-cat-appearance = Görünüm
settings-cat-streaming = Yayın
settings-cat-output = Çıkış
settings-cat-replay = Tekrar
settings-cat-hotkeys = Kısayollar
settings-cat-network = Ağ
settings-cat-accessibility = Erişilebilirlik
settings-cat-about = Hakkında
settings-ok = Tamam
settings-cancel = İptal
settings-apply = Uygula
settings-save = Kaydet
settings-loading = Ayarlar yükleniyor…
settings-hotkeys-filter = Kısayolları filtrele
settings-hotkeys-filter-placeholder = Eylemleri veya tuşları filtrelemek için yazın…
settings-hotkeys-no-match = “{ $query }” ile eşleşen kısayol yok.
settings-hotkey-none = Yok
settings-hotkey-group-ctrl = Ctrl + tuş
settings-hotkey-group-ctrl-shift = Ctrl + Shift + tuş
settings-hotkey-group-ctrl-alt = Ctrl + Alt + tuş
settings-hotkey-group-function = İşlev tuşları
settings-hotkey-group-numpad = Sayısal tuş takımı
settings-panic-section = Panik ekranı
settings-meter-section = Mikser seviye göstergeleri
settings-meter-note = Ses mikserinin seviye göstergelerinin sessizden kırpılmaya kadar geçtiği renkler. Renk körlüğüne uygun ön ayar, kırmızı-yeşil renk körlüğünde de okunabilen mavi → turuncu bir geçiş kullanır.
settings-meter-preset = Gösterge renkleri
settings-meter-preset-default = Yeşil / sarı / kırmızı
settings-meter-preset-colorblind = Renk körlüğüne uygun (mavi / turuncu)
settings-meter-preset-custom = Özel
settings-meter-low = Normal
settings-meter-mid = Yüksek
settings-meter-high = Kırpılma
settings-meter-preview = Önizleme

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Yenilikler
whats-new-loading = Sürüm notları yükleniyor…
whats-new-version = { $version } sürümündeki yenilikler
whats-new-empty = Bu sürüm için sürüm notu yok.
filters-name-directional-blur = Yönlü Bulanıklık
filters-name-radial-blur = Radyal Bulanıklık
filters-name-zoom-blur = Yakınlaştırma Bulanıklığı
filters-name-pixelate = Pikselleştir
filters-angle = Açı (°)
filters-center-x = Merkez X
filters-center-y = Merkez Y
filters-block-size = Blok boyutu (px)
filters-name-freeze = Dondur
filters-freeze-hint = Etkinleştirildiğinde bu kaynak son karesini tutar — program, önizleme, kayıt ve yayın hep birlikte donar. Dondurmak veya çözmek için bu filtreyi açıp kapatın.
transform-3d = 3B eğim
transform-rotation-x = Eğim X (°)
transform-rotation-y = Eğim Y (°)
transform-perspective = Perspektif
transform-reveal = Göster/gizle
transform-reveal-ms = Belirme (ms)
sources-clone-title = Kopyala (aynı kaynak, kendi filtreleri)
sources-clone-item = { $name } kopyala
menu-tools-downstream = Çıkış Keyer'ları…
menu-tools-transition-rules = Geçiş Kuralları…
dsk-title = Çıkış Keyer'ları
dsk-hint = Program çıkışına bindirilen katmanlar — her sahnenin üzerinde ve sahne değiştirdiğinizde yerinde kalırlar (logo, CANLI rozeti, alt bant). Listenin en üstü en önde çizilir.
dsk-empty = Henüz keyer yok — her sahneye bindirmek için bir kaynak ekleyin.
dsk-enable = Bu keyer'ı etkinleştir
dsk-move-up = Yukarı taşı (en öne)
dsk-move-down = Aşağı taşı
dsk-remove = Keyer'ı kaldır
dsk-opacity = Opaklık
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Ölçek
dsk-add = + Keyer ekle
transition-rules-title = Geçiş kuralları
transition-rules-hint = Bir sahne çiftine kendi geçişini verin. İlk sahneden ikincisine geçtiğinizde, varsayılan yerine bu tür ve süre kullanılır (bir Stinger/Görüntü kuralı yine de geçiş denetimlerinde ayarlanan dosyayı kullanır).
transition-rules-empty = Henüz kural yok — her sahne çifti varsayılan geçişi kullanır.
transition-rules-from = Nereden
transition-rules-to = Nereye
transition-rules-kind = Geçiş
transition-rules-duration = Süre (ms)
transition-rules-add = Kural ekle
transition-rules-remove = Kuralı kaldır

# --- Telestrator (CAP-N57) ---
telestrator-group = Telestrator
telestrator-draw = Çiz
telestrator-tool-pen = Kalem
telestrator-tool-highlight = Fosforlu kalem
telestrator-tool-arrow = Ok
telestrator-tool-ellipse = Elips
telestrator-color = Renk
telestrator-width = Kalınlık
telestrator-whiteboard = Beyaz tahta modu
telestrator-persist = Koru
telestrator-fade = Soldur
telestrator-undo = Son işareti geri al
telestrator-clear = İşaretleri temizle
hotkeys-telestrator-clear = Telestrator: işaretleri temizle

# --- Teleprompter (CAP-N58) ---
teleprompter-title = Teleprompter
teleprompter-loading = Yükleniyor…
teleprompter-empty = Henüz metin yok — teleprompter panelinden bir tane ekleyin.
teleprompter-script = Metin
teleprompter-script-placeholder = Metninizi yazın veya yapıştırın (düz metin ya da Markdown)…
teleprompter-top = Başa
teleprompter-slower = Daha yavaş
teleprompter-play = Oynat
teleprompter-pause = Duraklat
teleprompter-faster = Daha hızlı
teleprompter-speed = Hız (satır/sn)
teleprompter-font = Yazı boyutu
teleprompter-mirror = Ayna (cam)
teleprompter-open-projector = Projektörü aç
teleprompter-preview = Önizleme
teleprompter-remote-hint = Bu kapalıyken kaydırmayı kısayol, MIDI veya LAN paneliyle kontrol edin.
menu-tools-teleprompter = Teleprompter
hotkeys-teleprompter-toggle = Teleprompter: oynat / duraklat

# --- Remote guests: green room / cues / QoS / auto-grid (CAP-N54–N56, N59) ---
remote-hosting-count = { $count } konuk barındırılıyor
remote-green-room-default = Bekleme odası
remote-green-room-default-title = Yeni konuklar, siz yerleştirene kadar bekleme odasında bekler
remote-auto-grid = Otomatik ızgara
remote-auto-grid-title = Konuklar girip çıktıkça onları ızgarada tut
remote-arrange-grid = Izgarayı düzenle
remote-arrange-grid-title = Yayındaki konukları şimdi ızgaraya diz
remote-green-room-monitor = Bekleme odası önizlemesi
remote-tech-cam-ok = Kamera ✓
remote-tech-cam-no = Kamera yok
remote-tech-mic-ok = Mikrofon ✓
remote-tech-mic-no = Mikrofon yok
remote-seat-on-air = Yayına al
remote-cues-label = İşaretler
remote-cue = İşaret
remote-cue-thirty = 30 saniye
remote-cue-wrap = Toparla
remote-cue-next = Sıradaki sensin
remote-cue-speak = Daha yüksek konuş
remote-qos-good = Bağlantı iyi
remote-qos-fair = Bağlantı orta
remote-qos-poor = Bağlantı kötü
remote-green-room-guest = Bekleme odasındasın — sunucu birazdan seni yayına alacak.

# --- Hotkey audit — Phase 7 actions ---
hotkey-audit-action-telestrator-clear = Telestrator: işaretleri temizle
hotkey-audit-action-teleprompter-toggle = Teleprompter: oynat / duraklat
hotkey-audit-feature-telestrator = Telestrator
hotkey-audit-feature-teleprompter = Teleprompter
