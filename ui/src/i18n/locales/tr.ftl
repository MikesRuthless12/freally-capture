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
remote-hosting-guest = Uzak bir konuk barındırılıyor
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
remote-center-cam-title = Sunucudan kameranızı merkeze almasını isteyin
remote-center-my-cam = Kameram
remote-center-screen-title = Sunucudan paylaşılan ekranınızı merkeze almasını isteyin
remote-center-my-screen = Ekranım
remote-center-host-title = Merkezi sunucunun görüntüsüne geri verin
remote-center-host-view = Sunucu görünümü
remote-end-session = Oturumu bitir
remote-leave = Ayrıl
remote-host-view-heading = Sunucu görünümü
remote-host-shared-view-label = Sunucunun paylaşılan görünümü
remote-guest-position-label = Konuk konumu
remote-guest-label = Konuk
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
sources-add-nested-scene = İç İçe Sahne
sources-add-slideshow = Görüntü Slayt Gösterisi
sources-add-chat-overlay = Canlı Sohbet Katmanı
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
controls-pause-title-resume = Sürdür — dosya tek bir kesintisiz zaman çizelgesi olarak devam eder
controls-pause-title-pause = Duraklat — hiçbir kare yazılmaz; sürdürmek aynı oynatılabilir dosyaya devam eder
controls-resume-recording = ▶ Kaydı Sürdür
controls-pause-recording = ⏸ Kaydı Duraklat
controls-reactions-label = Tepkiler (programa gömülü)
controls-reactions-title = Programın üzerinde bir tepki uçur — kaydedilir VE yayınlanır, böylece tekrar tam anı gösterir. Sohbetteki izleyiciler de bunları tetikler (tepki emojileri otomatik uçar); bir sel yalnızca ekrandakini sınırlar.
controls-react = { $emoji } tepki ver
controls-virtual-camera-title = Sanal kamera her işletim sistemi için kendi imzalı sürücü bileşenini gerektirir (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO uzantısı / Linux v4l2loopback) — kendi aşaması olarak gelir. Akış modeli hazır: program, dikey tuval veya tek bir kaynak, Windows/Linux'ta eşli bir sanal mikrofonla (macOS'te sanal mikrofon API'si yok — dürüstçe söylenirse).
controls-virtual-camera = ⌁ Sanal Kamerayı Başlat
controls-files-title = Tamamlanmış kayıtlar + mp4'e yeniden paketleme eylemi
controls-files = ▤ Dosyalar…
controls-output-title = Kayıt biçimi, kodlayıcı, klasör, izler ve bölme
controls-output = ⚙ Çıkış…
controls-stream-title = Yayına Başla hedefi: hizmet, yayın anahtarı, kodlayıcı, bit hızı
controls-stream = ⦿ Yayın…
controls-codecs-title = İsteğe bağlı ffmpeg wire-codec bileşeni (açıkça etiketli, asla paketlenmez)
controls-codecs = ⬡ Codec'ler…
controls-replay-title = Tekrar arabelleği uzunluğu + kalite ön ayarları
controls-replay = ⟲ Tekrar…
controls-keys-title = Genel kısayollar: kayıt, Yayına Başla, geçiş, tekrarı kaydet
controls-keys = ⌨ Tuşlar…
controls-scripts-title = Yalıtılmış Lua betikleri: yayına-geçme/sahne/kayıt olaylarına tepki ver, stüdyoyu yönet
controls-scripts = ⚡ Betikler…
controls-docks-title = Tarayıcı panelleri: bir sohbet açılır penceresi, uyarılar sayfası veya Companion düğmelerini stüdyonun yanında bir pencere olarak aç
controls-docks = ⧉ Paneller…
controls-remote-title = Stream Deck / Companion denetleyicileri için WebSocket uzak API'si (varsayılan olarak kapalı)
controls-remote = ⌁ Uzaktan…
controls-profiles-title = Profiller (ayarlar) + sahne koleksiyonları — değiştirilebilir anlık görüntüler
controls-profiles = ▣ Profiller…
controls-bug-title = Bir hata bildir — anonim, isteğe bağlı (hiçbir şey otomatik gönderilmez)
controls-bug = 🐞 Hata bildir…
controls-updates-title = Güncellemeleri denetle — imzalı, doğrulanmış, tıklama olmadan hiçbir şey indirilmez
controls-updates = ⭳ Güncellemeleri denetle…
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

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Bellek
stats-dropped = Düşürülen
stats-render = İşleme
stats-gpu = GPU
stats-gpu-compositing = birleştiriyor
stats-gpu-idle = boşta
stats-vertical-fps = 9:16 FPS
stats-targets-label = Yayın hedefleri
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
audiofilters-title = Ses filtreleri — { $name }
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
filters-mask-image = Maske görüntüsü
filters-mask-mode = Mod
filters-mask-alpha = alfa
filters-mask-luma = luma
filters-mask-invert = tersine çevir
filters-speed-x = Hız X (px/s)
filters-speed-y = Hız Y (px/s)
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
hotkeys-record-placeholder = örn. Ctrl+Shift+R
hotkeys-go-live = Yayına Başla / Yayını Bitir
hotkeys-go-live-placeholder = örn. Ctrl+Shift+L
hotkeys-transition = Stüdyo-Modu Geçişi
hotkeys-transition-placeholder = örn. Ctrl+Shift+T veya F13
hotkeys-save-replay = Tekrarı Kaydet (son N saniye)
hotkeys-save-replay-placeholder = örn. Ctrl+Shift+S
hotkeys-add-marker = Bölüm işaretçisi bırak (kayıt)
hotkeys-add-marker-placeholder = örn. Ctrl+Shift+K
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

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Özel (RTMP/RTMPS)
stream-service-srt = SRT (kendi barındırılan)
stream-service-whip = WHIP (WebRTC)
