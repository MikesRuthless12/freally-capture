# Freally Capture — id
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Mode Studio
toggle-on = aktif
toggle-off = nonaktif
stats = Statistik
core-ok = inti OK
hide-stats-dock = Sembunyikan dok statistik
show-stats-dock = Tampilkan dok statistik


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Tidak dapat menyimpan pengaturan — perubahan tidak akan bertahan setelah restart.
studio-mode-leave = Keluar dari Mode Studio
studio-mode-enter-title = Mode Studio — edit scene pratinjau, lalu tayangkan ke program dengan transisi
vertical-canvas-title = Kanvas keluaran kedua (vertikal 9:16) — dapat direkam dan di-stream secara terpisah
app-version = v{ $version }
core-error = inti ERROR
core-unreachable = inti tidak dapat dijangkau (mode browser)
connecting-to-core = menghubungkan ke inti…
filters-source-fallback = Sumber

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Pratinjau program
preview-program-output = Keluaran program
preview-canvas-editor = Editor kanvas
preview-px-to-edge-label = Piksel ke tepi bingkai
preview-px-to-edge = px ke tepi L { $left } · T { $top } · R { $right } · B { $bottom }
preview-program-heading = Program
preview-no-gpu = Tidak ditemukan adaptor GPU yang dapat digunakan — kompositor tidak dapat berjalan di mesin ini.
preview-starting-compositor = Memulai kompositor…
preview-empty-scene = Scene ini kosong — tambahkan sumber di panel Sumber, lalu geser, skalakan, dan putar langsung di kanvas ini.
preview-fps = { $fps } fps
preview-dropped = { $dropped } terjatuh

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Tautan undangan diterima
remote-join-with-webcam = Gabung dengan webcam
remote-dismiss = Tutup
remote-hosting-guest = Menjadi host untuk tamu jarak jauh
remote-you-are-guest = Anda adalah tamu jarak jauh
remote-share-view-title = Bagikan layar Anda ke aplikasi tamu (mereka melihat tampilan Anda secara langsung)
remote-stop-sharing-view = Berhenti membagikan tampilan
remote-share-my-view = Bagikan tampilan saya
remote-allow-center-title = Izinkan tamu mengganti tampilan mana yang menempati bagian tengah (Anda tetap memegang kendali dan bisa mengembalikannya kapan saja)
remote-guest-switching = Peralihan oleh tamu:
remote-stop-screen = Hentikan layar
remote-share-screen = Bagikan layar
remote-share-screen-title-guest = Bagikan layar Anda dengan host (menjadi sumber yang bisa mereka jadikan tengah)
remote-center-request-label = Permintaan tampilan tengah
remote-center = Tengah
remote-center-cam-title = Minta host menjadikan kamera Anda sebagai tampilan tengah
remote-center-my-cam = Kamera saya
remote-center-screen-title = Minta host menjadikan layar yang Anda bagikan sebagai tampilan tengah
remote-center-my-screen = Layar saya
remote-center-host-title = Kembalikan tampilan tengah ke tampilan host
remote-center-host-view = Tampilan host
remote-end-session = Akhiri sesi
remote-leave = Keluar
remote-host-view-heading = Tampilan host
remote-host-shared-view-label = Tampilan yang dibagikan host
remote-guest-position-label = Posisi tamu
remote-guest-label = Tamu
remote-put-guest = Tempatkan tamu { $position }
remote-remove-title = Keluarkan tamu — mereka bisa bergabung lagi dengan tautan yang sama
remote-remove = Keluarkan
remote-ban-title = Blokir tamu — memblokir mereka dan membatalkan tautan undangan
remote-ban = Blokir
remote-guest-self-muted = tamu membisukan diri
remote-unmute-guest = Bunyikan tamu
remote-mute-guest = Bisukan tamu
remote-muted-by-host = Dibisukan oleh host
remote-unmute-mic = Bunyikan mik
remote-mute-mic = Bisukan mik
remote-waiting-for-host = menunggu host


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = sumber
sources-fallback-video = video
sources-fallback-error = kesalahan
sources-kind-unknown = ?
sources-missing-source = (sumber hilang)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Layar
sources-badge-window = Jendela
sources-badge-portal = Portal
sources-badge-camera = Kamera
sources-badge-image = Gambar
sources-badge-media = Media
sources-badge-guest = Tamu
sources-badge-color = Warna
sources-badge-text = Teks
sources-badge-scene = Scene
sources-badge-slides = Slide
sources-badge-chat = Chat
sources-badge-audio-in = Audio Masuk
sources-badge-audio-out = Audio Keluar
sources-badge-app-audio = Audio Aplikasi

# Add-source menu items
sources-add-display = Tangkapan Layar
sources-add-window = Tangkapan Jendela
sources-add-game = Tangkapan Game (baca dulu)
sources-add-webcam = Perangkat Tangkapan Video
sources-add-image = Gambar
sources-add-media = Media (berkas video/gambar)
sources-add-remote-guest = Tamu Jarak Jauh (uji coba P2P)
sources-add-color = Warna
sources-add-text = Teks
sources-add-nested-scene = Scene Bersarang
sources-add-slideshow = Tayangan Gambar
sources-add-chat-overlay = Overlay Chat Langsung
sources-add-audio-input = Tangkapan Input Audio
sources-add-audio-output = Tangkapan Output Audio
sources-add-app-audio = Audio Aplikasi (Windows)
sources-add-existing = Sumber yang ada…

# Panel header + toolbar buttons
sources-panel-title = Sumber
sources-group-title = Kelompokkan sumber — pilih dua item atau lebih, lalu Buat grup; item yang dikelompokkan bergerak dan tampil/sembunyi bersama
sources-group-aria = Kelompokkan sumber
sources-arrange = Susun: layar + sudut
sources-add-source = Tambah sumber
sources-browser-source-note = Browser Source hadir sebagai komponen sesuai-permintaan tersendiri (mesin Chromium ~180 MB — tidak pernah dibundel). Saat ini: tangkap jendela browser sungguhan dengan Tangkapan Jendela + chroma/color key, atau buka chat/peringatan sebagai Dok (Kontrol → Dok).

# Empty state
sources-empty = Tidak ada sumber di scene ini — tambahkan Tangkapan Layar, Jendela, Webcam, Gambar, Warna, atau Teks dengan “+”. Geser, skalakan, dan putar di kanvas; tombol di sisi kanan mengatur ulang urutan tumpukan.

# Per-row controls
sources-already-in-group = Sudah ada di { $name }
sources-pick-for-new-group = Pilih untuk grup baru
sources-pick-item-for-group = Pilih { $name } untuk grup baru
sources-hide = Sembunyikan
sources-show = Tampilkan
sources-hide-item = Sembunyikan { $name }
sources-show-item = Tampilkan { $name }
sources-unfocus-title = Batalkan fokus — pulihkan tata letak
sources-focus-title = Fokus — penuhi kanvas (Sorot Pembicara)
sources-unfocus-item = Batalkan fokus { $name }
sources-focus-item = Fokuskan { $name }
sources-center-title = Tengah — jadikan ini tampilan tengah bersama (kamera pindah ke rel)
sources-center-item = Jadikan tengah { $name }
sources-rename-item = Ganti nama { $name }
sources-in-group = Di grup { $name }

# Row status + retry
sources-retry-error = Coba lagi — { $message }
sources-retry-item = Coba lagi { $name }
sources-status-error = status: kesalahan
sources-open-privacy-title = Buka pengaturan privasi macOS untuk izin ini
sources-open-privacy-item = Buka pengaturan privasi untuk { $name }
sources-privacy-settings-button = pengaturan
sources-status-starting = memulai…
sources-status-live = langsung
sources-status-aria = status: { $state }

# Media row pause/resume
sources-media-resume-title = Lanjutkan video (langsung di stream)
sources-media-pause-title = Jeda video — tahan frame + bisukan, langsung di stream
sources-media-resume-item = Lanjutkan { $name }
sources-media-pause-item = Jeda { $name }

# Hover controls
sources-unlock = Buka kunci
sources-lock = Kunci
sources-unlock-item = Buka kunci { $name }
sources-lock-item = Kunci { $name }
sources-raise-title = Naikkan dalam tumpukan
sources-raise-item = Naikkan { $name }
sources-lower-title = Turunkan dalam tumpukan
sources-lower-item = Turunkan { $name }
sources-filters-title = Filter & campuran
sources-filters-item = Filter untuk { $name }
sources-properties-title = Properti
sources-properties-item = Properti dari { $name }
sources-remove-title = Hapus dari scene ini
sources-remove-item = Hapus { $name }

# Grouping footer
sources-create-group = Buat grup ({ $count })
sources-cancel = Batal

# Groups list
sources-groups-aria = Grup sumber
sources-hide-group = Sembunyikan grup
sources-show-group = Tampilkan grup
sources-item-count = · { $count } item
sources-ungroup-title = Pisahkan grup — item tetap di tempatnya
sources-ungroup-item = Pisahkan grup { $name }

# Live Chat Overlay picker
sources-chat-title = Tambah Overlay Chat Langsung
sources-chat-youtube-label = YouTube — URL channel, watch, atau live_chat (tanpa kunci, tanpa masuk)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  atau URL watch?v=
sources-chat-twitch-label = Twitch — nama channel (dibaca anonim, tanpa akun)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — slug channel (endpoint publik, sebaik mungkin)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Pesan muncul dengan stempel waktu h:mm:ss AM/PM berjalan di latar transparan (default kanan atas; geser ke mana saja). Banjir chat hanya menua-keluarkan baris lama — tidak akan pernah menghentikan stream atau rekaman. Chat Facebook memerlukan token Graph Anda sendiri dan belum diimplementasikan — tidak pernah diperlukan dan tidak pernah menghalangi platform di atas.
sources-chat-add = Tambah overlay chat
sources-chat-default-name = Chat Langsung

# Image Slideshow picker
sources-slideshow-title = Tambah Tayangan Gambar
sources-slideshow-empty = Belum ada gambar — Telusuri menambahkannya secara berurutan.
sources-slideshow-remove-slide = Hapus slide { $number }
sources-slideshow-browse = Telusuri gambar…
sources-slideshow-per-slide-label = Per slide (ms)
sources-slideshow-crossfade-label = Crossfade (ms, 0 = potong)
sources-slideshow-loop-label = Loop (nonaktif = tahan slide terakhir)
sources-slideshow-shuffle-label = Acak tiap siklus
sources-slideshow-note = Crossfade memadukan gambar berukuran sama; ukuran berbeda dipotong tegas di batasnya (tanpa penskalaan diam-diam).
sources-slideshow-add = Tambah tayangan ({ $count })

# Nested Scene picker
sources-nested-title = Tambah Scene Bersarang
sources-nested-empty = Tidak ada scene lain untuk disarangkan — tambahkan scene kedua dulu.
sources-nested-scene-name = Scene: { $name }
sources-nested-note = Scene bersarang dirender langsung pada ukuran kanvas program dan mengikuti editannya sendiri; transformasi, filter, dan campuran berlaku padanya seperti sumber lain. Sumber audionya bergabung ke mix saat scene yang menampilkannya menjadi program.

# Display / Window capture picker
sources-capture-display-title = Tambah Tangkapan Layar
sources-capture-window-title = Tambah Tangkapan Jendela
sources-capture-looking = Mencari sumber…
sources-capture-none-displays = Tidak ada yang bisa ditangkap di sini — tidak ada layar yang ditemukan.
sources-capture-none-windows = Tidak ada yang bisa ditangkap di sini — tidak ada jendela yang ditemukan.
sources-capture-portal-note = Di Wayland, dialog sistem memilih layar atau jendela — aplikasi tidak bisa menangkap secara global di sana, jadi itulah jalur yang jujur (dan satu-satunya).
sources-capture-window-note = Pratinjau diperbarui langsung. Jendela yang diminimalkan menampilkan frame terakhirnya (atau tidak ada) sampai Anda memulihkannya.
sources-thumb-no-preview = tanpa pratinjau
sources-thumb-loading = memuat…

# Video Capture Device picker
sources-webcam-title = Tambah Perangkat Tangkapan Video
sources-webcam-looking = Mencari kamera…
sources-webcam-none = Tidak ada kamera atau kartu tangkap yang ditemukan.
sources-webcam-format-label = Format
sources-webcam-format-auto-loading = Otomatis (memuat format…)
sources-webcam-format-auto = Otomatis (resolusi tertinggi)
sources-webcam-card-presets-label = Preset kartu:
sources-webcam-preset-title = Pilih mode { $label } yang diiklankan kartu ini
sources-webcam-add = Tambah kamera

# Audio Input / Output capture picker
sources-audio-output-title = Tambah Tangkapan Output Audio
sources-audio-input-title = Tambah Tangkapan Input Audio
sources-audio-default-output = Output default (yang Anda dengar)
sources-audio-default-input = Input default
sources-audio-looking = Mencari perangkat audio…
sources-audio-none-output = Tidak ada perangkat tangkap audio desktop yang ditemukan di sini.
sources-audio-none-input = Tidak ada mikrofon atau line-in yang ditemukan.
sources-audio-input-note = Strip mixer mendapat meter VU, fader, bisukan, monitoring, filter (denoise, gate, kompresor…), dan penetapan track. Semuanya tetap di mesin ini.

# Application Audio picker
sources-appaudio-title = Tambah Audio Aplikasi
sources-appaudio-looking = Mencari aplikasi yang mengeluarkan suara…
sources-appaudio-none = Tidak ada aplikasi yang mengeluarkan suara saat ini — mulai pemutaran di aplikasi, lalu segarkan.
sources-appaudio-refresh = ⟳ Segarkan
sources-appaudio-note = Menangkap tepat audio aplikasi itu — VU, fader, bisukan, filter, dan track-nya sendiri.

# Game Capture picker
sources-game-title = Tangkapan Game
sources-game-checking = Memeriksa…
sources-game-use-portal = Gunakan Tangkapan Layar (Portal)
sources-game-use-window = Gunakan Tangkapan Jendela saja

# Image picker
sources-image-title = Tambah Gambar
sources-image-file-label = Berkas gambar (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Tambah gambar

# Path field
sources-browse = Telusuri…

# Media picker
sources-media-title = Tambah Media
sources-media-file-label = Berkas media (mp4, mkv, webm, mov, .frec, atau gambar)
sources-media-loop-label = Loop (mulai ulang dari awal saat selesai)
sources-media-note = .frec diputar melalui codec freally-video milik sendiri — tidak ada yang perlu diunduh. Format wire (mp4/mkv/webm/…) didekode melalui komponen FFmpeg sesuai-permintaan; audionya masuk ke mixer sebagai strip tersendiri.
sources-media-add = Tambah media

# Invite expiry options
sources-ttl-15min = 15 mnt
sources-ttl-30min = 30 mnt
sources-ttl-1hour = 1 jam
sources-ttl-1day = 1 hari

# Remote Guest form
sources-remote-copy-failed = tidak bisa menyalin — pilih tautan dan salin manual
sources-remote-join-failed = gagal bergabung: { $error }
sources-remote-title = Tamu Jarak Jauh (uji coba P2P)
sources-remote-host-heading = Host — undang tamu
sources-remote-start-hosting = Mulai jadi host
sources-remote-expires-label = Kedaluwarsa
sources-remote-invite-expiry-aria = Kedaluwarsa undangan
sources-remote-invite-link-aria = Tautan undangan
sources-remote-copied = Tersalin ✓
sources-remote-copy = Salin
sources-remote-share-note = Bagikan tautan ini (Discord / teks / email). Ia membawa sesi Anda dan kedaluwarsa sesuai pengaturan. Tamu membukanya dan bergabung dengan webcam mereka.
sources-remote-qr-note = Pindai di ponsel untuk bergabung langsung dari browser — kamera + mik, tanpa instalasi. Tautan freally:// yang bisa disalin di atas terbuka di Freally Capture pada mesin yang memilikinya.
sources-remote-guest-heading = Tamu — bergabung dengan undangan
sources-remote-paste-placeholder = tempel tautan undangan
sources-remote-invite-input-aria = Tautan undangan atau id sesi
sources-remote-join = Gabung dengan webcam
sources-remote-session-note = Kontrol sesi langsung (bisukan, akhiri) tetap ada di bilah di bagian atas jendela utama — Anda bisa menutup dialog ini.
sources-remote-stop-session = Hentikan sesi

# Invite QR
sources-invite-qr-aria = Kode QR tautan undangan

# Remote device pickers
sources-devices-output-unavailable = perutean output tidak tersedia — memutar di perangkat default
sources-devices-mic-test-failed = tes mik gagal: { $error }
sources-devices-heading = Perangkat audio sesi
sources-devices-microphone-label = Mikrofon
sources-devices-microphone-aria = Mikrofon sesi
sources-devices-system-default = Default sistem
sources-devices-output-label = Output
sources-devices-output-aria = Output audio sesi
sources-devices-stop-test = Hentikan tes
sources-devices-test = Tes — dengar diri sendiri
sources-devices-testing-note = bicara ke mik — Anda mendengar perangkat yang dipilih secara langsung
sources-devices-idle-note = mengulang mik Anda ke output (headphone menghindari feedback)

# TURN relay section
sources-turn-save-failed = tidak bisa menyimpan: { $error }
sources-turn-summary = Jaringan — relai TURN opsional (lanjutan)
sources-turn-note-1 = Sesi terhubung langsung (P2P) — gratis, tanpa perlu relai. Jika KEDUA sisi berada di balik NAT ketat, jalur langsung bisa gagal; relai TURN yang Anda jalankan sendiri lalu membawa medianya. Melewati ini tidak masalah — sebagian besar koneksi bekerja langsung saja.
sources-turn-note-2 = Opsi gratis: Oracle Cloud "Always Free" menjalankan coturn tanpa biaya (catatan: Oracle meminta kartu kredit saat mendaftar, tapi bentuk Always-Free tetap gratis). Langkah: 1) buat VM gratis, 2) pasang coturn, 3) buka UDP 3478, 4) atur user/kata sandi, 5) masukkan turn:ip-vm-anda:3478 + kredensialnya di sini. Kredensial Anda tetap di berkas pengaturan lokal dan tidak pernah dicatat.
sources-turn-url-label = URL TURN
sources-turn-url-placeholder = turn:host:3478 (kosong = langsung saja)
sources-turn-url-aria = URL TURN
sources-turn-username-label = Nama pengguna
sources-turn-username-aria = Nama pengguna TURN
sources-turn-credential-label = Kredensial
sources-turn-credential-aria = Kredensial TURN
sources-turn-note-3 = Relai aktif setelah ketiga bidang terisi (server TURN memerlukan kredensial) dan berlaku untuk sesi berikutnya yang Anda mulai atau ikuti. Verifikasi dengan panggilan tes relai-saja antara dua mesin Anda sendiri.
sources-turn-settings-unavailable = pengaturan tidak tersedia (mode browser)

# Color picker
sources-color-title = Tambah Warna
sources-color-label = Warna
sources-color-width-label = Lebar
sources-color-height-label = Tinggi
sources-color-add = Tambah warna

# Text picker
sources-text-title = Tambah Teks
sources-text-label = Teks
sources-text-default = Teks
sources-text-color-label = Warna
sources-text-color-aria = Warna teks
sources-text-size-label = Ukuran (px)
sources-text-note = Keluarga font, perataan, pembungkusan, dan RTL ada di Properti sumber. Noto Sans bawaan (termasuk Arab/Ibrani) adalah default — identik di setiap mesin.
sources-text-add = Tambah teks

# Existing source picker
sources-existing-title = Tambah sumber yang ada
sources-existing-empty = Belum ada sumber — tambahkan satu ke scene mana pun dulu. Sumber yang ada bersifat berbagi: mengganti nama atau mengonfigurasi ulang satu memperbarui setiap scene yang menampilkannya.

# Screen + corners layout
sources-slot-off = Nonaktif
sources-slot-center = Tengah (layar)
sources-slot-top-left = Kiri Atas
sources-slot-top-right = Kanan Atas
sources-slot-bottom-left = Kiri Bawah
sources-slot-bottom-right = Kanan Bawah
sources-layout-title = Susun: Layar + sudut
sources-layout-empty = Tambahkan tangkapan layar dan satu atau lebih kamera ke scene ini dulu, lalu susun di sini.
sources-layout-note = Letakkan layar di tengah dan hingga empat kamera di sudut — tata letak penjelas / podcast Anda. Setiap sudut menampung webcam, jendela panggilan yang ditangkap, atau klip media. Anda bisa menggeser semuanya di kanvas setelahnya.
sources-layout-slot-aria = Slot untuk { $name }
sources-layout-apply = Terapkan tata letak


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Kontrol
controls-start-stop-title-stop = Hentikan dan finalkan rekaman
controls-start-stop-title-start = Rekam feed program dengan konfigurasi Pengaturan → Output
controls-finalizing = ◌ Memfinalkan…
controls-stop-recording = ■ Hentikan Rekaman
controls-start-recording = ● Mulai Merekam
controls-marker-title = Jatuhkan penanda bab pada saat ini — masuk ke REKAMAN (bab mkv, atau berkas sidecar). Penanda stream di sisi platform memerlukan akun platform, yang aplikasi ini tidak pernah minta.
controls-marker = ◈ Penanda
controls-pause-title-resume = Lanjutkan — berkas berlanjut sebagai satu linimasa berkesinambungan
controls-pause-title-pause = Jeda — tidak ada frame yang ditulis; melanjutkan meneruskan berkas yang sama dan dapat diputar
controls-resume-recording = ▶ Lanjutkan Rekaman
controls-pause-recording = ⏸ Jeda Rekaman
controls-reactions-label = Reaksi (dipanggang ke dalam program)
controls-reactions-title = Apungkan reaksi di atas program — direkam DAN di-stream, jadi replay menampilkan momen persisnya. Penonton di chat juga memicunya (emoji reaksi mereka mengapung otomatis); banjir hanya membatasi yang ada di layar.
controls-react = Reaksi { $emoji }
controls-virtual-camera-title = Kamera virtual memerlukan komponen driver bertanda tangan tersendiri per OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / ekstensi CoreMediaIO macOS / v4l2loopback Linux) — hadir sebagai milestone tersendiri. Model feed sudah siap untuknya: program, kanvas vertikal, atau satu sumber, dengan mik virtual berpasangan di Windows/Linux (macOS tidak punya API mik virtual — dikatakan jujur).
controls-virtual-camera = ⌁ Mulai Kamera Virtual
controls-files-title = Rekaman selesai + aksi remux-ke-mp4
controls-files = ▤ Berkas…
controls-output-title = Format rekaman, encoder, folder, track, dan pemisahan
controls-output = ⚙ Output…
controls-stream-title = Target Go Live: layanan, kunci stream, encoder, bitrate
controls-stream = ⦿ Stream…
controls-codecs-title = Komponen wire-codec ffmpeg sesuai-permintaan (berlabel jelas, tidak pernah dibundel)
controls-codecs = ⬡ Codec…
controls-replay-title = Panjang replay buffer + preset kualitas
controls-replay = ⟲ Replay…
controls-keys-title = Hotkey global: rekam, Go Live, transisi, simpan replay
controls-keys = ⌨ Tombol…
controls-scripts-title = Skrip Lua dalam sandbox: bereaksi terhadap peristiwa go-live/scene/rekaman, mengendalikan studio
controls-scripts = ⚡ Skrip…
controls-docks-title = Dok browser: buka popout chat, halaman peringatan, atau tombol Companion sebagai jendela di samping studio
controls-docks = ⧉ Dok…
controls-remote-title = API remote WebSocket untuk pengontrol Stream Deck / Companion (nonaktif secara default)
controls-remote = ⌁ Remote…
controls-profiles-title = Profil (pengaturan) + koleksi scene — snapshot yang bisa diganti
controls-profiles = ▣ Profil…
controls-bug-title = Laporkan bug — anonim, opt-in (tidak ada yang dikirim otomatis)
controls-bug = 🐞 Laporkan bug…
controls-updates-title = Periksa pembaruan — bertanda tangan, terverifikasi, tidak ada yang diunduh tanpa klik
controls-updates = ⭳ Periksa pembaruan…
controls-saved = Tersimpan: { $path }

# --- MixerDock.tsx ---
mixer-title = Mixer Audio
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Beralih ke strip horizontal
mixer-switch-to-vertical = Beralih ke strip vertikal
mixer-layout-aria-vertical = Tata letak mixer: vertikal — beralih ke horizontal
mixer-layout-aria-horizontal = Tata letak mixer: horizontal — beralih ke vertikal
mixer-empty = Tidak ada sumber audio di scene ini — tambahkan Tangkapan Input Audio (mik) atau Tangkapan Output Audio (audio desktop) dengan “+” di Sumber. Strip mendapat meter VU, fader, bisukan, monitoring, filter, dan penetapan track.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Loudness program (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Loudness sesaat (400 ms)
mixer-short-term-title = Loudness jangka pendek (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Perangkat output monitor
mixer-default-output = Output default

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memori
stats-dropped = Terjatuh
stats-render = Render
stats-gpu = GPU
stats-gpu-compositing = mengomposit
stats-gpu-idle = diam
stats-vertical-fps = FPS 9:16
stats-targets-label = Target stream
stats-shared-encode = · encode bersama
stats-starting = Memulai kompositor…

# --- ScenesRail.tsx ---
scenes-title = Scene
scenes-new-scene-name = Scene
scenes-add = Tambah scene
scenes-empty = Menghubungkan ke inti studio…
scenes-rename = Ganti nama { $name }
scenes-on-program = Di program
scenes-preview = Pratinjau { $name }
scenes-switch-to = Beralih ke { $name }
scenes-move-up = Naik
scenes-move-up-aria = Pindahkan { $name } ke atas
scenes-move-down = Turun
scenes-move-down-aria = Pindahkan { $name } ke bawah
scenes-last-stays = Scene terakhir tetap ada
scenes-remove = Hapus scene ini
scenes-remove-aria = Hapus { $name }


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
channelstrip-level = Level
channelstrip-monitor-off = Monitor nonaktif
channelstrip-monitor-only = Monitor saja (tidak dalam mix)
channelstrip-monitor-and-output = Monitor dan output
channelstrip-status-error = kesalahan
channelstrip-status-live = langsung
channelstrip-status-waiting-audio = menunggu audio
channelstrip-status = status: { $state }
channelstrip-status-waiting = menunggu
channelstrip-mute = Bisukan
channelstrip-unmute = Bunyikan
channelstrip-mute-source = Bisukan { $name }
channelstrip-unmute-source = Bunyikan { $name }
channelstrip-scene-mix-on = Mix per-scene AKTIF — strip ini menimpa mix global untuk scene ini (klik untuk mengikuti mix global lagi)
channelstrip-scene-mix-off = Mix per-scene — beri strip ini fader/bisukan sendiri untuk scene saat ini
channelstrip-scene-mix-label = Mix per-scene untuk { $name }
channelstrip-monitor-cycle = { $mode } — klik untuk berputar
channelstrip-monitor-mode = Mode monitor { $name }: { $mode }
channelstrip-audio-filters-title = Filter audio (denoise, gate, kompresor…)
channelstrip-audio-filters-label = Filter audio untuk { $name }
channelstrip-advanced-title = Offset sinkron & hotkey push-to-talk
channelstrip-advanced-label = Pengaturan audio lanjutan untuk { $name }
channelstrip-track-assignment = Penetapan track
channelstrip-track = Track { $n }
channelstrip-track-assigned = Track { $n } (ditetapkan)
channelstrip-track-label = Track { $n } untuk { $name }
channelstrip-device-error = kesalahan perangkat
channelstrip-audio-device-error = kesalahan perangkat audio
channelstrip-volume-label = Volume { $name } dalam desibel
channelstrip-ptt-hold = Push-to-talk: tahan { $key }
channelstrip-sync-offset = Offset sinkron (ms, 0–{ $max } — menunda audio ini)
channelstrip-ptt-hotkey = Hotkey push-to-talk (senyap kecuali ditahan)
channelstrip-ptt-placeholder = mis. Ctrl+Shift+T atau F13
channelstrip-ptt-aria = Hotkey push-to-talk
channelstrip-ptm-hotkey = Hotkey push-to-mute (senyap saat ditahan)
channelstrip-ptm-placeholder = mis. Ctrl+Shift+M
channelstrip-ptm-aria = Hotkey push-to-mute
channelstrip-hotkeys-note = Hotkey bekerja saat aplikasi lain sedang fokus. Di Linux/Wayland, hotkey global mungkin tidak tersedia — itu batasan kompositor, dikatakan jujur.
channelstrip-apply = Terapkan


# --- LiveButton.tsx ---
livebutton-failure-ended = stream berakhir
livebutton-title-live = Akhiri stream — setiap target (rekaman yang berjalan tetap lanjut)
livebutton-title-offline = Go live ke setiap target Pengaturan → Stream yang diaktifkan
livebutton-end-stream = ■ Akhiri Stream
livebutton-aria-reconnecting = Menyambung ulang
livebutton-aria-live = Langsung
livebutton-badge-retry = coba lagi { $n }
livebutton-badge-live = langsung
livebutton-go-live = ⦿ Go Live


# --- RecDot.tsx ---
recdot-paused-aria = Rekaman dijeda
recdot-recording-aria = Merekam
recdot-tracks-one = { $count } track audio direkam
recdot-tracks-other = { $count } track audio direkam
recdot-paused = dijeda


# --- ReplayControls.tsx ---
replaycontrols-saved = Replay tersimpan — { $name }
replaycontrols-failure-stopped = buffer berhenti
replaycontrols-title-disarm = Nonaktifkan replay buffer (membuang riwayat yang belum disimpan)
replaycontrols-title-arm = Aktifkan replay buffer bergulir — menyimpan N detik terakhir siap disimpan (encode ringannya sendiri; stream dan rekaman tak tersentuh)
replaycontrols-replay-seconds = ⟲ Replay { $seconds }s
replaycontrols-arm = ⟲ Aktifkan Replay Buffer
replaycontrols-save-title = Simpan N detik terakhir ke folder rekaman (juga di hotkey Simpan-Replay)
replaycontrols-save = ⤓ Simpan


# --- PropertiesDialog.tsx ---
properties-title = Properti — { $name }
properties-name = Nama
properties-cancel = Batal
properties-apply = Terapkan
properties-youtube = YouTube — URL channel / watch / live_chat (tanpa kunci, tanpa masuk, selamanya)
properties-twitch = Twitch — nama channel (anonim)
properties-kick = Kick — slug channel (endpoint publik)
properties-width-px = Lebar (px)
properties-lines = Baris
properties-font-px = Font (px)
properties-images = Berkas gambar (satu path per baris, ditampilkan berurutan)
properties-per-slide = Per slide (ms)
properties-crossfade = Crossfade (ms, 0 = potong)
properties-loop-slideshow = Loop (nonaktif = tahan slide terakhir)
properties-shuffle = Acak tiap siklus
properties-nested-scene = Scene yang disusun sumber ini (scene yang sudah memuat scene ini ditolak)
properties-portal-note = Portal ScreenCast Wayland memilih layar atau jendela di dialog sistem setiap kali sumber ini dimulai — tidak ada yang perlu dikonfigurasi di sini, memang begitu dirancang.
properties-appaudio-capturing = Menangkap audio dari { $exe }
properties-appaudio-exe-fallback = sebuah aplikasi
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Tambahkan ulang sumber untuk menargetkan aplikasi berbeda (id proses berubah saat aplikasi dimulai ulang).
properties-image-file = Berkas gambar
properties-media-file = Berkas media (mp4, mkv, webm, mov, .frec, atau gambar)
properties-media-loop = Loop (mulai ulang dari awal saat selesai)
properties-media-hwdecode = Dekode perangkat keras (jatuh ke perangkat lunak sendiri)
properties-media-note = .frec diputar melalui codec freally-video milik sendiri — tidak ada yang perlu diunduh. Format video lain didekode melalui komponen FFmpeg sesuai-permintaan. Audio berkas mendapat strip mixer sendiri; offset sinkron strip menyetel halus penyelarasan A/V. Klip tanpa audio membiarkan stripnya senyap.
properties-color = Warna
properties-width = Lebar
properties-height = Tinggi
properties-text = Teks
properties-font-family = Keluarga font (sistem; kosong = default)
properties-size-px = Ukuran (px)
properties-text-color = Warna teks
properties-align = Perataan
properties-align-left = kiri
properties-align-center = tengah
properties-align-right = kanan
properties-line-spacing = Spasi baris
properties-wrap-width = Lebar bungkus (px; 0 = nonaktif)
properties-force-rtl = Paksa kanan-ke-kiri
properties-text-note = Rendering memakai shaping sesungguhnya (penyambungan Arab, ligatur) dan pengurutan baris bidi. Keluarga Noto Sans bawaan (termasuk Arab/Ibrani) adalah default; keluarga sistem juga berfungsi. CJK memakai font sistem untuk saat ini.
properties-repick-capturing = Menangkap: { $label }
properties-repick-looking = Mencari sumber…
properties-repick-none-displays = Tidak ada layar yang ditemukan untuk dipilih ulang.
properties-repick-none-windows = Tidak ada jendela yang ditemukan untuk dipilih ulang.
properties-repick-again = Pilih lagi:
properties-device = Perangkat
properties-video-current-device = (perangkat saat ini)
properties-format = Format
properties-format-auto-loading = Otomatis (memuat format…)
properties-format-auto = Otomatis (resolusi tertinggi)
properties-audio-capture-of = Tangkap audio dari
properties-audio-default-output = Output default (yang Anda dengar)
properties-audio-default-input = Input default
properties-audio-default-suffix = (default)
properties-audio-current-device = (perangkat saat ini: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Gain
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Kompresor
audiofilters-name-limiter = Limiter
audiofilters-name-eq = EQ 3-Band
audiofilters-name-denoise = Denoise
audiofilters-name-ducking = Ducking
audiofilters-title = Filter audio — { $name }
audiofilters-chain-header = Rantai filter (atas jalan dulu, sebelum fader)
audiofilters-add = + Tambah filter
audiofilters-add-menu = Tambah filter audio
audiofilters-empty = Belum ada filter — denoise mik (DSP klasik, tanpa ML), gate ruangan, jinakkan puncak dengan kompresor, atau duck musik di bawah suara Anda.
audiofilters-enable = Aktifkan { $name }
audiofilters-run-earlier = Jalankan lebih awal
audiofilters-move-up = Pindahkan { $name } ke atas
audiofilters-run-later = Jalankan lebih akhir
audiofilters-move-down = Pindahkan { $name } ke bawah
audiofilters-remove-title = Hapus filter
audiofilters-remove = Hapus { $name }
audiofilters-gain-db = Gain (dB)
audiofilters-open-db = Buka pada (dB)
audiofilters-close-db = Tutup pada (dB)
audiofilters-attack-ms = Attack (ms)
audiofilters-hold-ms = Hold (ms)
audiofilters-release-ms = Release (ms)
audiofilters-ratio = Rasio (:1)
audiofilters-threshold-db = Ambang (dB)
audiofilters-output-gain-db = Gain output (dB)
audiofilters-ceiling-db = Ceiling (dB)
audiofilters-low-db = Low (dB)
audiofilters-mid-db = Mid (dB)
audiofilters-high-db = High (dB)
audiofilters-strength = Kekuatan
audiofilters-denoise-note = Penekanan spektral DSP-klasik milik sendiri — noise tetap (kipas, desis) turun sementara ucapan lolos. Tanpa ML, tanpa model, sesuai charter.
audiofilters-duck-under = Duck di bawah
audiofilters-ducking-trigger = Sumber pemicu ducking
audiofilters-pick-trigger = (pilih pemicu — mis. mik Anda)
audiofilters-trigger-at-db = Picu pada (dB)
audiofilters-duck-by-db = Duck sebesar (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Render Delay
filters-name-color-correction = Koreksi Warna
filters-name-lut = Terapkan LUT
filters-name-blur = Blur
filters-name-mask = Masker Gambar
filters-name-sharpen = Pertajam
filters-name-scroll = Gulir
filters-name-crop = Pangkas
filters-title = Filter — { $name }
filters-blend-mode = Mode campuran
filters-chain-header = Rantai filter (atas jalan dulu)
filters-add = + Tambah filter
filters-add-menu = Tambah filter
filters-empty = Belum ada filter — chroma key webcam, koreksi warna tangkapan, atau gulir ticker.
filters-enable = Aktifkan { $name }
filters-run-earlier = Jalankan lebih awal
filters-move-up = Pindahkan { $name } ke atas
filters-run-later = Jalankan lebih akhir
filters-move-down = Pindahkan { $name } ke bawah
filters-remove-title = Hapus filter
filters-remove = Hapus { $name }
filters-key-color-rgb = Warna kunci (warna apa pun, jarak RGB)
filters-similarity = Kemiripan
filters-smoothness = Kehalusan
filters-luma-min = Luma min (kunci lebih gelap keluar)
filters-luma-max = Luma maks (kunci lebih terang keluar)
filters-delay = Delay (ms — hanya video, mis. untuk sinkron dengan audio; dibatasi 500)
filters-key-color = Warna kunci
filters-spill = Spill
filters-gamma = Gamma
filters-brightness = Kecerahan
filters-contrast = Kontras
filters-saturation = Saturasi
filters-hue-shift = Pergeseran hue
filters-opacity = Opasitas
filters-cube-file = Berkas .cube
filters-amount = Jumlah
filters-radius = Radius
filters-mask-image = Gambar masker
filters-mask-mode = Mode
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = balik
filters-speed-x = Kecepatan X (px/s)
filters-speed-y = Kecepatan Y (px/s)
filters-crop-left = kiri
filters-crop-top = atas
filters-crop-right = kanan
filters-crop-bottom = bawah
filters-crop-aria = pangkas { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Segarkan
pickershell-refresh-title = Segarkan daftar
pickershell-close = Tutup


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Laporkan bug
bugreport-intro = Laporan bersifat anonim dan opt-in — tidak ada yang dikirim otomatis. Anda akan meninjau teks persis di bawah, lalu mengirimkannya lewat isu GitHub yang sudah terisi atau aplikasi email Anda. Tanpa data pribadi (path home dan nama pengguna Anda disamarkan); tanpa akun, tanpa server.
bugreport-crash-notice = Freally Capture tertutup tak terduga pada jalannya sebelumnya — detail crash anonim disertakan di bawah. Melaporkannya membantu memperbaikinya cepat.
bugreport-description-label = Apa yang sedang Anda lakukan saat itu terjadi? (opsional)
bugreport-description-placeholder = mis. pratinjau membeku saat saya menambah webcam kedua
bugreport-include-crash = Sertakan detail crash anonim dari jalannya terakhir
bugreport-preview-label = Persis apa yang akan dikirim
bugreport-open-github = Buka isu GitHub
bugreport-gmail-title = Membuka jendela tulis Gmail di browser Anda, sudah terisi. Belum masuk? Google menampilkan layar login-nya dulu.
bugreport-compose-gmail = Tulis di Gmail
bugreport-email-title = Membuka draf di aplikasi email default PC ini (Outlook, Thunderbird, Mail…)
bugreport-send-email = Kirim email
bugreport-copied = Tersalin ✓
bugreport-copy-report = Salin laporan
bugreport-dismiss-crash = Abaikan crash
bugreport-copy-failed = tidak bisa menyalin — pilih teks dan salin manual
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = APA YANG TERJADI
bugreport-preview-no-description = (tidak ada deskripsi diberikan)
bugreport-preview-diagnostics = DIAGNOSTIK ANONIM (tanpa data pribadi)
bugreport-preview-from = Dari: Freally Capture
bugreport-preview-crash-excerpt = --- kutipan crash ---


# --- Updates.tsx ---
updates-title = Pembaruan perangkat lunak
updates-checking = Memeriksa pembaruan…
updates-uptodate = Anda memakai versi terbaru.
updates-check-again = Periksa lagi
updates-available = Versi { $version } tersedia
updates-current-version = (Anda punya { $current })
updates-release-notes-label = Versi { $version } — Catatan rilis
updates-confirm = Ingin memperbarui sekarang? Unduhan diverifikasi terhadap kunci penandatangan bawaan sebelum diterapkan. Freally Capture menutup, penginstal berjalan, dan versi baru terbuka sendiri.
updates-yes-update-now = Ya, perbarui sekarang
updates-no-not-now = Tidak, nanti saja
updates-downloading = Mengunduh { $version }…
updates-starting = memulai…
updates-installed = Pembaruan terpasang.
updates-restart-now = Mulai ulang sekarang
updates-restart-later = Mulai ulang nanti
updates-try-again = Coba lagi


# --- Models.tsx ---
models-title = Komponen
models-ffmpeg-heading = FFmpeg — wire codec
models-badge-third-party = Pihak ketiga · tidak dibundel
models-ffmpeg-desc = Mesin milik Freally Capture merekam freally-video (.frec) lossless tanpa tambahan apa pun. Merekam format wire yang diharapkan platform dan pemutar — H.264/AAC (dan HEVC/AV1) dalam mp4/mkv/mov/webm — memakai FFmpeg, alat terpisah yang aplikasi ini tidak pernah sertakan: codec itu terbebani paten, jadi tetap opsional dan berlabel jelas. Ia diunduh sesuai-permintaan dari build yang dipatok di bawah, diverifikasi SHA-256 sebelum pemakaian pertama, disimpan per-pengguna, dan dijalankan sebagai proses terpisah. Lisensinya (LGPL/GPL) miliknya sendiri — lihat THIRD-PARTY-NOTICES.
models-checking = Memeriksa…
models-ffmpeg-not-installed = Tidak terpasang. Tersedia: FFmpeg { $version } dari { $source } (unduhan { $size }).
models-ffmpeg-none-pinned = Belum ada build FFmpeg yang dipatok untuk platform ini — rekaman wire-codec tidak tersedia di sini. Rekaman freally-video lossless tidak terpengaruh.
models-ffmpeg-download-verify = Unduh & verifikasi ({ $size })
models-downloading = Mengunduh…
models-download-of = dari
models-cancel = Batal
models-ffmpeg-verifying = Memverifikasi unduhan terhadap SHA-256 yang dipatok…
models-ffmpeg-extracting = Membongkar…
models-ffmpeg-ready = Terpasang & terverifikasi — { $version }
models-remove = Hapus
models-ffmpeg-retry = Coba unduh lagi
models-network-note = Unduhan adalah satu-satunya aksi jaringan di panel ini dan tidak pernah dimulai sendiri. Checksum gagal membatalkan pemasangan — aplikasi menolak menjalankan byte yang tak bisa dijaminnya.
models-cef-heading = Runtime Browser Source — Chromium (CEF)
models-cef-desc = Browser source merender halaman web (peringatan, widget, overlay) melalui Chromium Embedded Framework — runtime ~100 MB yang aplikasi ini tidak pernah sertakan. Ia diunduh sesuai-permintaan dari indeks build CEF resmi, diverifikasi terhadap SHA-1 indeks itu sebelum apa pun dibongkar, dan disimpan per-pengguna. Browser source yang merender melaluinya hadir dengan milestone-nya sendiri; ini memasang runtime yang dibutuhkannya.
models-cef-download-install = Unduh & pasang
models-cef-unsupported = CEF tidak menerbitkan build untuk platform ini — browser source tidak tersedia di sini.
models-cef-resolving = Menyelesaikan build stabil terbaru…
models-cef-verifying = Memverifikasi unduhan terhadap SHA-1 indeks…
models-cef-extracting = Membongkar runtime…
models-cef-ready = Terpasang — CEF { $version }.
models-cef-retry = Coba lagi
models-integrations-heading = Integrasi opsional
models-badge-never-bundled = Tidak pernah dibundel
models-ndi-detected = Terdeteksi
models-ndi-not-installed = Tidak terpasang
models-vst-available = Tersedia
models-vst-not-available = Tidak tersedia


# --- Recordings.tsx ---
recordings-title = Rekaman
recordings-loading = Membaca folder…
recordings-empty = Belum ada rekaman — Mulai Merekam menulis ke folder yang diatur di Output.
recordings-frec-label = lossless milik sendiri (freally-video)
recordings-remux-title = Bungkus ulang sebagai mp4 — stream copy, tanpa encode ulang, tanpa perubahan kualitas (butuh komponen FFmpeg)
recordings-remuxing = Membungkus ulang…
recordings-remux-to-mp4 = Remux ke MP4
recordings-export-mp4-title = Dekode .frec milik sendiri dan encode ulang ke MP4 (H.264/AAC) agar diputar di pemutar apa pun — butuh komponen FFmpeg
recordings-exporting = Mengekspor…
recordings-export-mp4 = Ekspor → MP4
recordings-export-mkv-title = Dekode .frec milik sendiri dan encode ulang ke MKV agar diputar di pemutar apa pun
recordings-starting = memulai…
recordings-frames = { $done } / { $total } frame
recordings-cancel = Batal
recordings-export-cancelled = Ekspor dibatalkan.
recordings-exported-to = Diekspor ke { $path }
recordings-remuxed-to = Di-remux ke { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Buka rekaman .frec
openfrec-desc = Freally Capture merekam format .frec lossless milik sendiri — ia tidak memutarnya. Freally Player akan memutar .frec langsung saat dirilis. Untuk saat ini, ekspor ke MP4/MKV dan ia diputar di pemutar apa pun (VLC, pemutar OS Anda, apa saja).
openfrec-exported-to = Diekspor ke { $path }
openfrec-exporting = Mengekspor…
openfrec-starting = memulai…
openfrec-export-mp4 = Ekspor → MP4
openfrec-export-mkv = Ekspor → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Kanvas vertikal (9:16)
vertical-enable = Aktifkan kanvas kedua — dapat direkam dan di-stream terpisah dari program
vertical-scene-label = Scene yang disusun kanvas ini
vertical-width = Lebar
vertical-height = Tinggi
vertical-preview-alt = Pratinjau kanvas vertikal
vertical-note = Posisi item piksel-akurat di seluruh kanvas: pilih scene ini di rel Scene untuk menyusunnya sementara pratinjau ini menampilkan hasil vertikal. Target stream memilih kanvas ini di ⦿ Stream…; Pengaturan → Output bisa merekamnya bersama berkas utama.
vertical-close = Tutup


# --- EulaGate.tsx ---
eula-title = Freally Capture — Perjanjian Lisensi
eula-version = v{ $version }
eula-intro = Silakan baca dan terima perjanjian ini untuk memakai Freally Capture. Singkatnya: ini alat netral, dan Anda sepenuhnya bertanggung jawab atas apa yang Anda tangkap, rekam, dan siarkan — serta atas kepemilikan haknya.
eula-thanks = Terima kasih telah membaca.
eula-scroll-hint = Gulir ke akhir untuk melanjutkan.
eula-decline = Tolak & Keluar
eula-agree = Saya Setuju


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Pengaturan — Output
output-loading = Pengaturan masih memuat…
output-container-frec = freally-video (.frec) — lossless, milik sendiri, tidak ada yang perlu diunduh
output-container-mkv = MKV — tahan crash; remux ke mp4 nanti
output-container-mp4 = MP4 — diputar di mana saja
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Lossless
output-preset-lossless-title = Codec freally-video milik sendiri — bit-eksak, tanpa unduhan
output-preset-high-label = Kualitas tinggi
output-preset-high-title = MP4, encoder terbaik terdeteksi, CQ 16 nyaris-lossless, preset Kualitas
output-preset-balanced-label = Seimbang
output-preset-balanced-title = MKV, encoder terbaik terdeteksi, CQ 23, preset Seimbang
output-recording-format = Format rekaman
output-ffmpeg-warning = Format ini butuh komponen FFmpeg (wire codec — tidak dibundel). .frec lossless tidak butuh apa pun.
output-install = Pasang…
output-recordings-folder = Folder rekaman
output-folder-placeholder = Folder Video OS
output-filename-prefix = Awalan nama berkas
output-frame-rate = Laju frame
output-fps-option = { $fps } fps
output-split-every = Pisah tiap (menit, 0 = nonaktif)
output-output-width = Lebar output (0 = kanvas; hanya format wire)
output-output-height = Tinggi output (0 = kanvas)
output-record-vertical = Rekam juga kanvas vertikal (berkas paralel “… (vertical)”; butuh kanvas 9:16 diaktifkan)
output-audio-tracks = Track audio
output-recorded-tracks-group = Track yang direkam
output-track-last-one = Setidaknya satu track harus merekam
output-record-track-on = Rekam track { $index }: aktif
output-record-track-off = Rekam track { $index }: nonaktif
output-encoder-heading = Encoder
output-video-encoder = Encoder video
output-encoder-auto = Otomatis — terbaik terdeteksi (H.264)
output-encoder-unavailable = — tidak tersedia di sini
output-preset = Preset
output-preset-quality = Kualitas
output-preset-balanced-option = Seimbang
output-preset-performance = Performa
output-rate-control = Kontrol laju
output-rc-cqp = CQP (kualitas konstan)
output-rc-cbr = CBR (bitrate konstan)
output-rc-vbr = VBR (bitrate variabel)
output-cq = CQ (0–51, lebih rendah = lebih baik)
output-bitrate = Bitrate (kbps)
output-keyframe = Interval keyframe (s)
output-audio-bitrate = Bitrate audio (kbps / track)
output-presets = Preset:

# --- SettingsStream.tsx ---
stream-title = Pengaturan — Stream
stream-target-enabled = Target { $index } aktif
stream-target = Target { $index }
stream-remove = Hapus
stream-service = Layanan
stream-canvas = Kanvas
stream-canvas-main = Utama (program)
stream-canvas-vertical = Vertikal (9:16 — aktifkan di studio)
stream-ingest-srt = URL ingest SRT
stream-ingest-whip = URL endpoint WHIP
stream-ingest-url = URL ingest
stream-ingest-override = (timpa — kosong = preset layanan)
stream-key-srt = streamid (opsional — ditambahkan sebagai ?streamid=…; diperlakukan sebagai rahasia)
stream-key-whip = Token Bearer (opsional — dikirim sebagai header Authorization; sebuah rahasia)
stream-key-custom = Kunci stream (dari server Anda — diperlakukan sebagai rahasia)
stream-key-service = Kunci stream (dari dasbor kreator Anda — diperlakukan sebagai rahasia)
stream-key-aria = Kunci stream { $index }
stream-key-hide = Sembunyikan
stream-key-show = Tampilkan
stream-encoder = Encoder (H.264 — yang dibawa RTMP, SRT, dan WHIP semua)
stream-encoder-auto = Otomatis — encoder H.264 terbaik terdeteksi
stream-encoder-unavailable = (tidak tersedia di sini)
stream-video-bitrate = Bitrate video (kbps, CBR)
stream-audio-bitrate = Bitrate audio (kbps)
stream-fps = FPS
stream-keyframe = Interval keyframe (s)
stream-audio-track = Track audio (1–6)
stream-output-width = Lebar output (0 = kanvas)
stream-output-height = Tinggi output (0 = kanvas)
stream-add-target = + Tambah target
stream-go-live-note = Go Live menerbitkan ke setiap target yang diaktifkan sekaligus, langsung ke tiap platform. Target dengan pengaturan encoder identik berbagi satu encode.
stream-auto-record = Mulai merekam saat saya go live (rekaman tetap berhenti secara terpisah)
stream-ffmpeg-note-before = Wire codec streaming berjalan melalui komponen ffmpeg sesuai-permintaan yang berlabel —
stream-ffmpeg-note-link = kelola di sini
stream-ffmpeg-note-after = . Rekaman lokal terus berjalan apa pun yang dilakukan stream.
stream-cancel = Batal
stream-save = Simpan

# --- SettingsReplay.tsx ---
replay-title = Pengaturan — Replay Buffer
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 mnt
replay-length-2min = 2 mnt
replay-length-5min = 5 mnt
replay-quality-low = Rendah (3 Mbps)
replay-quality-standard = Standar (6 Mbps)
replay-quality-high = Tinggi (12 Mbps)
replay-length-presets = Preset panjang
replay-quality-presets = Preset kualitas
replay-length-seconds = Panjang (detik)
replay-video-bitrate = Bitrate video (kbps)
replay-fps = FPS
replay-audio-track = Track audio (1–6)
replay-note = Saat aktif, buffer menjalankan encode ringannya sendiri ke ring on-disk yang dibatasi — sekitar { $mb } MB pada pengaturan ini. Menyimpan menjahit ring tanpa encode ulang dan tak pernah menyentuh stream atau rekaman. Perubahan berlaku saat Anda mengaktifkan berikutnya.
replay-cancel = Batal
replay-save = Simpan

# --- SettingsRemote.tsx ---
remote-title = Pengaturan — Kontrol Jarak Jauh
remote-enable = Aktifkan API remote WebSocket
remote-password = Kata sandi (wajib — pengontrol mengautentikasi dengannya)
remote-password-placeholder = kata sandi untuk pengontrol Anda
remote-password-hide = Sembunyikan
remote-password-show = Tampilkan
remote-port = Port
remote-allow-lan = Izinkan koneksi LAN (default hanya mesin ini)
remote-note = Nonaktif = port tertutup. Aktif = WebSocket terlindung kata sandi di 127.0.0.1 (atau LAN Anda saat diizinkan) yang bisa mengganti scene, menjalankan transisi, memulai/menghentikan stream dan rekaman, menyimpan replay, dan mengatur bisukan/volume — aksi yang sama dengan UI, tidak lebih. Ia tidak bisa membaca berkas. Perlakukan kata sandi seperti kredensial mana pun; utamakan hanya-mesin-ini kecuali Anda memang mengontrol dari perangkat lain.
remote-password-required = Kata sandi diperlukan untuk mengaktifkan API remote.
remote-cancel = Batal
remote-save = Simpan

# --- SettingsHotkeys.tsx ---
hotkeys-title = Pengaturan — Hotkey
hotkeys-record = Mulai / hentikan rekaman
hotkeys-record-placeholder = mis. Ctrl+Shift+R
hotkeys-go-live = Go Live / Akhiri Stream
hotkeys-go-live-placeholder = mis. Ctrl+Shift+L
hotkeys-transition = Transisi Mode Studio
hotkeys-transition-placeholder = mis. Ctrl+Shift+T atau F13
hotkeys-save-replay = Simpan Replay (N detik terakhir)
hotkeys-save-replay-placeholder = mis. Ctrl+Shift+S
hotkeys-add-marker = Jatuhkan penanda bab (rekaman)
hotkeys-add-marker-placeholder = mis. Ctrl+Shift+K
hotkeys-note = Hotkey bersifat global — mereka menyala saat aplikasi lain sedang fokus. Kosong = tak terikat. Tombol push-to-talk/bisukan mixer ada di menu ⋯ tiap strip. Di Linux/Wayland, hotkey global mungkin tidak tersedia (batasan kompositor) — tombolnya tetap berfungsi.
hotkeys-cancel = Batal
hotkeys-save = Simpan

# --- WorkspaceDialog.tsx ---
workspace-title = Profil & Koleksi Scene
workspace-profiles = Profil
workspace-profiles-hint = Profil adalah pengaturan Anda — target stream, output, hotkey. Ganti per acara atau per platform.
workspace-collections = Koleksi scene
workspace-collections-hint = Koleksi adalah scene + sumber Anda. Buat menggandakan yang sekarang sebagai titik awal.
workspace-active = Aktif
workspace-switch-to = Beralih ke { $name }
workspace-active-marker = ● aktif
workspace-new-name-placeholder = nama baru…
workspace-new-name-label = Nama { $title } baru
workspace-create = Buat

# --- OBS import (CAP-M02) ---
workspace-import-obs = Impor dari OBS…
workspace-import-obs-hint = Bawa masuk koleksi scene OBS (scenes.json miliknya). Koleksi Anda saat ini disimpan lebih dulu.
workspace-import-busy = Mengimpor…
workspace-import-title = "{ $name }" diimpor
workspace-import-summary = { $scenes } scene · { $sources } sumber · { $items } item
workspace-import-dismiss = Tutup
workspace-import-clean = Semuanya terimpor dengan baik.
workspace-import-geometry-caveat = Ukuran dan posisi disesuaikan dari tata letak OBS — periksa tiap scene dan pilih ulang perangkat tangkapan.
workspace-import-notes-title = Diimpor dengan catatan
workspace-import-skipped-title = Tidak diimpor
import-note-needsReselect = Pilih ulang perangkat/monitor/jendela
import-note-gameCaptureAsWindow = Game Capture → Window Capture
import-note-referencesFile = Periksa jalur berkas
import-note-filterDropped = Beberapa filter tidak didukung
import-note-geometryApproximated = Posisi/ukuran diperkirakan
import-skip-unsupportedKind = Tidak ada tipe sumber setara
import-skip-group = Grup belum didukung

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Tautkan ulang berkas yang hilang…
doctor-title = Berkas hilang
doctor-scanning = Memindai…
doctor-all-good = Semua berkas yang dirujuk ada. Tidak ada yang perlu ditautkan ulang.
doctor-intro = { $count } berkas yang dirujuk tidak ditemukan di komputer ini. Tunjukkan lokasi baru masing-masing — setiap scene yang memakainya diperbaiki sekaligus.
doctor-relinked = { $count } rujukan ditautkan ulang.
doctor-uses = dipakai { $count }×
doctor-locate = Temukan…
doctor-locate-folder = Cari di folder…
doctor-locate-folder-hint = Pilih folder; setiap berkas hilang dicocokkan berdasarkan nama dan ditautkan ulang.
doctor-kind-image = gambar
doctor-kind-media = media
doctor-kind-slideshow = tayangan slaid
doctor-kind-font = fon
doctor-kind-lut = LUT
doctor-kind-mask = topeng
history-relinkFiles = Tautkan ulang berkas

# --- ScriptsDialog.tsx ---
scripts-title = Skrip (Lua)
scripts-empty = Belum ada skrip — tambahkan berkas .lua. Lihat scripts/sample.lua untuk API-nya: bereaksi terhadap peristiwa go-live/scene/rekaman dan jalankan perintah yang sama dengan API remote.
scripts-enable = Aktifkan { $path }
scripts-remove = Hapus { $path }
scripts-path-label = Path skrip
scripts-add = Tambah
scripts-note = Skrip berjalan dalam sandbox — tanpa akses berkas atau OS; mereka hanya bisa memanggil perintah studio yang sama dengan API remote (ganti scene, transisi, rekam/stream/replay, bisukan). Kesalahan skrip dicatat dan dibendung. Perubahan berlaku dalam sedetik.
scripts-error-not-lua = Arahkan ke berkas .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Dok browser
browser-dock-empty = Belum ada dok — tambahkan popout chat, halaman peringatan, atau tombol web Companion Anda.
browser-dock-open = Buka
browser-dock-remove = Hapus { $name }
browser-dock-name-placeholder = nama (mis. Twitch Chat)
browser-dock-name-label = Nama dok
browser-dock-url-label = URL dok
browser-dock-note = Dok terbuka sebagai jendela tersendiri yang bisa Anda letakkan di samping studio. Halaman tidak mendapat akses ke aplikasi — hanya merender. Hanya URL http(s); dok terbuka hanya saat Anda klik Buka.
browser-dock-error-name = Beri nama dok (mis. Twitch Chat).
browser-dock-error-url = URL dok harus dimulai dengan http:// atau https://.

# --- studio-preview-pane ---
studio-preview-label = Pratinjau Mode Studio
studio-preview-heading = Pratinjau
studio-preview-hint = klik sebuah scene untuk memuatnya di sini
studio-preview-empty = Pratinjau akan muncul di sini.
studio-preview-mirrors = mencerminkan program
studio-preview-transition-select = Transisi
studio-preview-duration = Durasi transisi (ms)
studio-preview-commit-title = Tayangkan Pratinjau → Program lewat transisi (penonton melihatnya)
studio-preview-transitioning = Bertransisi…
studio-preview-transition-button = Transisi ⇄
studio-preview-luma-placeholder = gambar sapuan skala abu-abu (png/jpg)
studio-preview-luma-label = Gambar Sapuan Luma
studio-preview-browse = Telusuri…
studio-preview-filter-images = Gambar
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = video stinger (ProRes 4444 .mov mempertahankan alfanya)
studio-preview-stinger-label = Berkas video Stinger
studio-preview-stinger-cut-label = Titik potong Stinger (ms)
studio-preview-stinger-cut-title = Saat pergantian scene terjadi di balik stinger (ms ke dalam transisi)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Potong
transition-kind-fade = Pudar
transition-kind-slide-left = Geser ←
transition-kind-slide-right = Geser →
transition-kind-slide-up = Geser ↑
transition-kind-slide-down = Geser ↓
transition-kind-swipe-left = Usap ←
transition-kind-swipe-right = Usap →
transition-kind-luma-linear = Sapuan Luma (linear)
transition-kind-luma-radial = Sapuan Luma (radial)
transition-kind-luma-horizontal = Sapuan Luma (horizontal)
transition-kind-luma-diamond = Sapuan Luma (belah ketupat)
transition-kind-luma-clock = Sapuan Luma (jam)
transition-kind-image = Sapuan Gambar (kustom)
transition-kind-stinger = Stinger (video)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Kustom (RTMP/RTMPS)
stream-service-srt = SRT (swakelola)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Tentang
about-tagline = Rekam dan stream seperti studio — tanpa akun, tanpa cloud.
about-version = Versi
about-created-by = Dibuat oleh
about-project-started = Proyek dimulai
about-first-stable = Rilis stabil pertama
about-first-stable-pending = Belum — 1.0.0 sedang dalam proses
about-platform = Platform
about-local-first = Freally Capture berjalan sepenuhnya di mesin Anda. Tanpa akun, tanpa telemetri, tanpa cloud — satu-satunya hal yang meninggalkan komputer Anda adalah stream yang Anda pilih untuk dikirim.
about-website = Situs web
about-issues = Laporkan masalah
about-license = Lisensi
about-eula = EULA
about-third-party = Pemberitahuan pihak ketiga
about-check-updates = Periksa pembaruan…

# --- unified settings modal (TASK-906) ---
settings-title = Pengaturan
settings-language-section = Bahasa
settings-language = Bahasa antarmuka
settings-language-system = Default sistem
settings-language-note = Bahasa yang Anda pilih di sini akan diingat. “Default sistem” mengikuti sistem operasi Anda. Teks yang belum diterjemahkan kembali ke bahasa Inggris.
settings-appearance-section = Tampilan
settings-theme = Tema
settings-theme-dark = Gelap
settings-theme-light = Terang
settings-theme-custom = Kustom
settings-accent = Aksen
settings-general-section = Umum
settings-show-stats-dock = Tampilkan dok statistik
settings-more-section = Pengaturan lainnya
settings-open-output = Rekaman…
settings-open-stream = Streaming…
settings-open-replay = Replay…
settings-open-hotkeys = Hotkey…
settings-open-remote = API remote…
settings-open-about = Tentang…
controls-settings = ⚙ Pengaturan…
controls-settings-title = Bahasa, tampilan, dan preferensi seluruh aplikasi
# --- command palette (TASK-904) ---
palette-title = Palet perintah
palette-search = Cari scene, sumber, dan aksi
palette-placeholder = Cari scene, sumber, aksi…
palette-no-results = Tidak ada yang cocok dengan “{ $query }”
palette-hint = ↑ ↓ untuk berpindah · Enter untuk menjalankan · Esc untuk menutup
palette-group-scenes = Scene
palette-group-sources = Sumber
palette-group-actions = Aksi
palette-transition = Transisi Pratinjau → Program
palette-save-replay = Simpan replay
palette-add-marker = Jatuhkan penanda bab
palette-vertical-canvas = Kanvas vertikal (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Selamat datang di Freally Capture
wizard-welcome = Dua langkah singkat: periksa kemampuan mesin Anda, lalu mulai sebuah scene. Hanya sekitar tiga puluh detik, dan semuanya bisa Anda ubah nanti.
wizard-local-first = Tidak ada apa pun di sini yang meninggalkan komputer Anda. Freally Capture tanpa akun, tanpa telemetri, dan tanpa cloud.
wizard-start = Mulai
wizard-skip = Lewati
wizard-hardware-title = Kemampuan mesin Anda
wizard-probing = Memeriksa kartu grafis dan prosesor Anda…
wizard-encoder = Encoder
wizard-canvas = Kanvas
wizard-bitrate = Bitrate
wizard-probe-found = Ditemukan: { $gpus } · { $cores } inti fisik
wizard-no-gpu = tanpa GPU khusus
wizard-apply = Gunakan pengaturan ini
wizard-keep-current = Pertahankan yang saya punya
wizard-template-title = Mulai dengan sebuah scene
wizard-template-screen = Tangkap layar saya
wizard-template-screen-note = Menambahkan Tangkapan Layar dari monitor utama Anda. Tempat paling umum untuk memulai.
wizard-template-empty = Mulai kosong
wizard-template-empty-note = Sebuah scene kosong. Tambahkan sumber sendiri dengan tombol +.
wizard-done = Anda sudah siap.
wizard-done-hint = Tekan Ctrl+K kapan saja untuk mencari scene, sumber, dan aksi. Pengaturan ada di balik tombol ⚙.
wizard-close = Mulai streaming

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Kartu grafis Anda bisa meng-encode video sendiri, sehingga prosesor tetap bebas untuk sisa studio.
autoconfig-reason-software = Tidak ada encoder perangkat keras yang bisa dipakai, jadi prosesor yang akan meng-encode. Itu tetap berfungsi, hanya lebih membebani CPU.
autoconfig-reason-quality-hardware = 1080p pada 60 frame per detik, dengan bitrate yang diterima setiap platform besar.
autoconfig-reason-quality-software = 30 frame per detik, karena encoding perangkat lunak pada 60 menjatuhkan frame di sebagian besar prosesor.
autoconfig-reason-quality-low-cores = Bitrate lebih rendah, karena prosesor ini punya sedikit inti dan encoding perangkat lunak akan berebut dengan kompositor untuknya.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Rekaman dimulai
announce-recording-paused = Rekaman dijeda
announce-recording-stopped = Rekaman dihentikan
announce-live-started = Anda sedang tayang langsung
announce-live-ended = Siaran berakhir
announce-reconnecting = Koneksi terputus, menyambung ulang
announce-stream-failed = Siaran gagal
announce-frames-dropped = { $count } frame terjatuh

# CAP-M01 — undo/redo edit history
palette-undo = Urungkan
palette-redo = Ulangi
palette-edit-history = Riwayat penyuntingan…
history-title = Riwayat penyuntingan
history-empty = Belum ada yang bisa diurungkan.
history-current = Keadaan saat ini
history-close = Tutup
history-addScene = Tambah adegan
history-renameScene = Ganti nama adegan
history-removeScene = Hapus adegan
history-reorderScene = Susun ulang adegan
history-addSource = Tambah sumber
history-removeSource = Hapus sumber
history-reorderSource = Susun ulang sumber
history-renameSource = Ganti nama sumber
history-transformSource = Pindahkan sumber
history-toggleVisibility = Alihkan visibilitas
history-toggleLock = Alihkan kunci
history-setBlendMode = Ubah mode campuran
history-editSourceProperties = Edit properti
history-applyLayout = Atur tata letak
history-moveToSeat = Pindah ke tempat
history-groupSources = Kelompokkan sumber
history-ungroupSources = Pisahkan grup
history-toggleGroupVisibility = Alihkan grup
history-setSceneAudio = Audio adegan
history-setVerticalCanvas = Kanvas vertikal
history-addFilter = Tambah filter
history-removeFilter = Hapus filter
history-reorderFilter = Susun ulang filter
history-editFilter = Edit filter
history-toggleFilter = Alihkan filter
history-setVolume = Sesuaikan volume
history-toggleMute = Alihkan bisu
history-setMonitor = Ubah pemantauan
history-setTracks = Ubah trek
history-setSyncOffset = Sesuaikan sinkron A/V
history-setAudioHotkeys = Pintasan audio

# CAP-M04 — alignment aids
settings-alignment-section = Bantuan perataan
settings-smart-guides = Panduan cerdas (jepret saat menyeret)
settings-safe-areas = Hamparan area aman
settings-rulers = Penggaris
align-group = Ratakan ke kanvas
align-left = Ratakan kiri
align-hcenter = Tengahkan horizontal
align-right = Ratakan kanan
align-top = Ratakan atas
align-vcenter = Tengahkan vertikal
align-bottom = Ratakan bawah

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Ratakan & distribusikan pilihan
arrange-left = Ratakan tepi kiri
arrange-hcenter = Tengahkan horizontal
arrange-right = Ratakan tepi kanan
arrange-top = Ratakan tepi atas
arrange-vcenter = Tengahkan vertikal
arrange-bottom = Ratakan tepi bawah
distribute-h = Distribusikan horizontal
distribute-v = Distribusikan vertikal
guides-group = Panduan
guides-add-v = Tambah panduan vertikal
guides-add-h = Tambah panduan horizontal
history-arrangeItems = Susun item
history-editGuides = Edit panduan

# CAP-M05 — edit transform + copy/paste
transform-title = Edit transformasi — { $name }
transform-anchor = Jangkar
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotasi
transform-crop = Pangkas
transform-crop-left = Kiri
transform-crop-top = Atas
transform-crop-right = Kanan
transform-crop-bottom = Bawah
transform-no-size = Ukuran dan pangkas tersedia setelah sumber melaporkan dimensinya.
transform-copy = Salin transformasi
transform-paste = Tempel transformasi
transform-close = Tutup
filters-copy = Salin filter ({ $count })
filters-paste = Tempel filter ({ $count })
palette-edit-transform = Edit transformasi…
history-pasteFilters = Tempel filter

# CAP-M26 — keying workbench
workbench-title = Meja keying — { $name }
workbench-mode-keyed = Ter-key
workbench-mode-source = Sumber
workbench-mode-matte = Matte
workbench-mode-split = Terbagi
workbench-eyedropper = Pipet
workbench-eyedropper-hint = Klik sumber untuk mengambil warna kunci.
workbench-loupe = Lup
workbench-split = Pembagian
workbench-preview-alt = Pratinjau meja keying
workbench-tune = Setel
workbench-close = Tutup

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Klik adegan untuk beralih ke sana.
multiview-hint-stage = Klik adegan untuk menyiapkannya di pratinjau.
palette-multiview = Monitor multiview

# CAP-M07 — projectors
projector-title = Buka proyektor
projector-source = Sumber
projector-target-program = Program
projector-target-preview = Pratinjau
projector-target-scene = Scene…
projector-target-source = Sumber…
projector-target-multiview = Multiview
projector-which-scene = Scene mana
projector-which-source = Sumber mana
projector-none = Tidak ada yang ditampilkan
projector-display = Layar
projector-windowed = Jendela mengambang (layar ini)
projector-display-option = Layar { $n } — { $w }×{ $h }
projector-primary = (utama)
projector-open = Buka
projector-cancel = Batal
projector-exit-hint = Tekan Esc untuk keluar
palette-projector = Buka proyektor…

# CAP-M08 — still-frame grab
palette-still = Ambil bingkai…
still-saved-toast = Bingkai disimpan: { $name }
still-failed-toast = Gagal mengambil bingkai: { $error }
hotkeys-still = Ambil bingkai
hotkeys-still-placeholder = mis. Ctrl+Shift+P
