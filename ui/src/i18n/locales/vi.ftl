# Freally Capture — vi
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Chế độ Studio
toggle-on = bật
toggle-off = tắt
stats = Thống kê
core-ok = lõi OK
hide-stats-dock = Ẩn bảng thống kê
show-stats-dock = Hiện bảng thống kê


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Không thể lưu cài đặt — thay đổi sẽ không được giữ lại sau khi khởi động lại.
studio-mode-leave = Thoát Chế độ Studio
studio-mode-enter-title = Chế độ Studio — chỉnh sửa cảnh xem trước, đưa nó lên chương trình bằng một chuyển cảnh
vertical-canvas-title = Khung hình đầu ra thứ hai (dọc 9:16) — có thể ghi và phát trực tiếp độc lập
app-version = v{ $version }
core-error = lõi LỖI
core-unreachable = không kết nối được lõi (chế độ trình duyệt)
connecting-to-core = đang kết nối tới lõi…
filters-source-fallback = Nguồn

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Xem trước chương trình
preview-program-output = Đầu ra chương trình
preview-canvas-editor = Trình chỉnh sửa khung hình
preview-px-to-edge-label = Số pixel tới các cạnh khung
preview-px-to-edge = px tới cạnh · Trái { $left } · Trên { $top } · Phải { $right } · Dưới { $bottom }
preview-program-heading = Chương trình
preview-no-gpu = Không tìm thấy bộ điều hợp GPU khả dụng — bộ dựng hình không thể chạy trên máy này.
preview-starting-compositor = Đang khởi động bộ dựng hình…
preview-empty-scene = Cảnh này trống — thêm một nguồn trong Nguồn, rồi kéo, chỉnh tỷ lệ và xoay nó ngay trên khung hình này.
preview-fps = { $fps } fps
preview-dropped = { $dropped } khung bị bỏ

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Đã nhận liên kết mời
remote-join-with-webcam = Tham gia bằng webcam
remote-dismiss = Bỏ qua
remote-hosting-guest = Đang làm chủ phòng cho khách từ xa
remote-you-are-guest = Bạn là khách từ xa
remote-share-view-title = Chia sẻ màn hình của bạn tới ứng dụng của khách (họ thấy khung hình của bạn trực tiếp)
remote-stop-sharing-view = Dừng chia sẻ khung hình
remote-share-my-view = Chia sẻ khung hình của tôi
remote-allow-center-title = Cho phép khách chuyển khung hình nào giữ vị trí trung tâm (bạn vẫn kiểm soát và có thể chuyển lại bất cứ lúc nào)
remote-guest-switching = Khách đang chuyển:
remote-stop-screen = Dừng màn hình
remote-share-screen = Chia sẻ màn hình
remote-share-screen-title-guest = Chia sẻ màn hình của bạn với chủ phòng (nó trở thành một nguồn mà họ có thể đưa vào trung tâm)
remote-center-request-label = Yêu cầu khung hình trung tâm
remote-center = Trung tâm
remote-center-cam-title = Yêu cầu chủ phòng đưa camera của bạn vào trung tâm
remote-center-my-cam = Cam của tôi
remote-center-screen-title = Yêu cầu chủ phòng đưa màn hình chia sẻ của bạn vào trung tâm
remote-center-my-screen = Màn hình của tôi
remote-center-host-title = Trả trung tâm về khung hình của chủ phòng
remote-center-host-view = Khung hình chủ phòng
remote-end-session = Kết thúc phiên
remote-leave = Rời đi
remote-host-view-heading = Khung hình chủ phòng
remote-host-shared-view-label = Khung hình chia sẻ của chủ phòng
remote-guest-position-label = Vị trí khách
remote-guest-label = Khách
remote-put-guest = Đặt khách { $position }
remote-remove-title = Xóa khách — họ có thể tham gia lại bằng cùng liên kết
remote-remove = Xóa
remote-ban-title = Cấm khách — chặn họ và vô hiệu hóa liên kết mời
remote-ban = Cấm
remote-guest-self-muted = khách tự tắt tiếng
remote-unmute-guest = Bật tiếng khách
remote-mute-guest = Tắt tiếng khách
remote-muted-by-host = Bị chủ phòng tắt tiếng
remote-unmute-mic = Bật tiếng micro
remote-mute-mic = Tắt tiếng micro
remote-waiting-for-host = đang chờ chủ phòng


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = nguồn
sources-fallback-video = video
sources-fallback-error = lỗi
sources-kind-unknown = ?
sources-missing-source = (thiếu nguồn)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Màn hình
sources-badge-window = Cửa sổ
sources-badge-portal = Portal
sources-badge-camera = Camera
sources-badge-image = Ảnh
sources-badge-media = Media
sources-badge-guest = Khách
sources-badge-color = Màu
sources-badge-text = Văn bản
sources-badge-scene = Cảnh
sources-badge-slides = Slide
sources-badge-chat = Chat
sources-badge-audio-in = Âm thanh vào
sources-badge-audio-out = Âm thanh ra
sources-badge-app-audio = Âm thanh ứng dụng
sources-badge-test-bars = Thanh
sources-badge-test-grid = Lưới
sources-badge-test-sweep = Quét
sources-badge-test-tone = Âm
sources-badge-test-sync = Đồng bộ
sources-badge-timer = Hẹn giờ

# Add-source menu items
sources-add-display = Chụp Màn hình
sources-add-window = Chụp Cửa sổ
sources-add-game = Chụp Trò chơi (đọc trước)
sources-add-webcam = Thiết bị Chụp Video
sources-add-image = Ảnh
sources-add-media = Media (tệp video/ảnh)
sources-add-remote-guest = Khách Từ xa (thử nghiệm P2P)
sources-add-color = Màu
sources-add-text = Văn bản
sources-add-timer = Hẹn giờ / Đồng hồ
sources-add-nested-scene = Cảnh Lồng nhau
sources-add-slideshow = Trình chiếu Ảnh
sources-add-chat-overlay = Lớp phủ Chat Trực tiếp
sources-add-test-signal = Tín hiệu thử
sources-add-audio-input = Chụp Đầu vào Âm thanh
sources-add-audio-output = Chụp Đầu ra Âm thanh
sources-add-app-audio = Âm thanh Ứng dụng (Windows)
sources-add-existing = Nguồn hiện có…

# Panel header + toolbar buttons
sources-panel-title = Nguồn
sources-group-title = Nhóm nguồn — chọn hai mục trở lên, rồi Tạo nhóm; các mục trong nhóm di chuyển và hiện/ẩn cùng nhau
sources-group-aria = Nhóm nguồn
sources-arrange = Sắp xếp: màn hình + các góc
sources-add-source = Thêm một nguồn
sources-browser-source-note = Nguồn Trình duyệt sẽ ra mắt như thành phần theo yêu cầu riêng của nó (một engine Chromium ~180 MB — không bao giờ đóng gói kèm). Hiện tại: chụp một cửa sổ trình duyệt thật bằng Chụp Cửa sổ + khóa chroma/màu, hoặc mở chat/thông báo dưới dạng Dock (Điều khiển → Docks).

# Empty state
sources-empty = Không có nguồn nào trong cảnh này — thêm Chụp Màn hình, Cửa sổ, Webcam, Ảnh, Màu hoặc Văn bản bằng “+”. Kéo, chỉnh tỷ lệ và xoay chúng trên khung hình; các nút bên phải sắp xếp lại thứ tự lớp.

# Per-row controls
sources-already-in-group = Đã ở trong { $name }
sources-pick-for-new-group = Chọn cho nhóm mới
sources-pick-item-for-group = Chọn { $name } cho nhóm mới
sources-hide = Ẩn
sources-show = Hiện
sources-hide-item = Ẩn { $name }
sources-show-item = Hiện { $name }
sources-unfocus-title = Bỏ lấy nét — khôi phục bố cục
sources-focus-title = Lấy nét — lấp đầy khung hình (Làm nổi Người nói)
sources-unfocus-item = Bỏ lấy nét { $name }
sources-focus-item = Lấy nét { $name }
sources-center-title = Trung tâm — đặt đây làm khung hình trung tâm chia sẻ (các cam chuyển ra thanh bên)
sources-center-item = Đưa { $name } vào trung tâm
sources-rename-item = Đổi tên { $name }
sources-in-group = Trong nhóm { $name }

# Row status + retry
sources-retry-error = Thử lại — { $message }
sources-retry-item = Thử lại { $name }
sources-status-error = trạng thái: lỗi
sources-open-privacy-title = Mở cài đặt quyền riêng tư macOS cho quyền này
sources-open-privacy-item = Mở cài đặt quyền riêng tư cho { $name }
sources-privacy-settings-button = cài đặt
sources-status-starting = đang khởi động…
sources-status-live = trực tiếp
sources-status-aria = trạng thái: { $state }

# Media row pause/resume
sources-media-resume-title = Tiếp tục video (trực tiếp trên luồng)
sources-media-pause-title = Tạm dừng video — giữ khung hình + tắt tiếng, trực tiếp trên luồng
sources-media-resume-item = Tiếp tục { $name }
sources-media-pause-item = Tạm dừng { $name }

# Hover controls
sources-unlock = Mở khóa
sources-lock = Khóa
sources-unlock-item = Mở khóa { $name }
sources-lock-item = Khóa { $name }
sources-raise-title = Đưa lên trong ngăn xếp
sources-raise-item = Đưa { $name } lên
sources-lower-title = Đưa xuống trong ngăn xếp
sources-lower-item = Đưa { $name } xuống
sources-filters-title = Bộ lọc & hòa trộn
sources-filters-item = Bộ lọc cho { $name }
sources-properties-title = Thuộc tính
sources-properties-item = Thuộc tính của { $name }
sources-remove-title = Xóa khỏi cảnh này
sources-remove-item = Xóa { $name }

# Grouping footer
sources-create-group = Tạo nhóm ({ $count })
sources-cancel = Hủy

# Groups list
sources-groups-aria = Nhóm nguồn
sources-hide-group = Ẩn nhóm
sources-show-group = Hiện nhóm
sources-item-count = · { $count } mục
sources-ungroup-title = Bỏ nhóm — các mục vẫn ở nguyên vị trí
sources-ungroup-item = Bỏ nhóm { $name }

# Live Chat Overlay picker
sources-chat-title = Thêm Lớp phủ Chat Trực tiếp
sources-chat-youtube-label = YouTube — URL kênh, watch, hoặc live_chat (không cần khóa, không đăng nhập)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  hoặc một URL watch?v=
sources-chat-twitch-label = Twitch — tên kênh (đọc ẩn danh, không cần tài khoản)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — slug kênh (điểm cuối công khai, nỗ lực tối đa)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Tin nhắn xuất hiện kèm dấu thời gian h:mm:ss AM/PM đang chạy trên nền trong suốt (mặc định trên cùng bên phải; kéo nó tới bất cứ đâu). Cơn lũ chat chỉ làm các dòng cũ trôi đi — nó không bao giờ có thể làm nghẽn luồng hay bản ghi. Chat Facebook cần token Graph của riêng bạn và chưa được triển khai — nó không bao giờ bắt buộc và không bao giờ chặn các nền tảng ở trên.
sources-chat-add = Thêm lớp phủ chat
sources-chat-default-name = Chat Trực tiếp

# Image Slideshow picker
sources-slideshow-title = Thêm Trình chiếu Ảnh
sources-slideshow-empty = Chưa có ảnh nào — Duyệt sẽ thêm chúng theo thứ tự.
sources-slideshow-remove-slide = Xóa slide { $number }
sources-slideshow-browse = Duyệt ảnh…
sources-slideshow-per-slide-label = Mỗi slide (ms)
sources-slideshow-crossfade-label = Chuyển mờ (ms, 0 = cắt)
sources-slideshow-loop-label = Lặp (tắt = giữ slide cuối)
sources-slideshow-shuffle-label = Xáo trộn mỗi vòng
sources-slideshow-note = Chuyển mờ hòa trộn các ảnh cùng kích thước; ảnh khác kích thước sẽ cắt cứng ở ranh giới (không tự thay đổi tỷ lệ).
sources-slideshow-add = Thêm trình chiếu ({ $count })

# Nested Scene picker
sources-nested-title = Thêm Cảnh Lồng nhau
sources-nested-empty = Không có cảnh khác để lồng — thêm cảnh thứ hai trước.
sources-nested-scene-name = Cảnh: { $name }
sources-nested-note = Cảnh lồng dựng trực tiếp ở kích thước khung hình chương trình và theo các chỉnh sửa riêng của nó; biến đổi, bộ lọc và hòa trộn áp dụng cho nó như bất kỳ nguồn nào. Các nguồn âm thanh của nó hòa vào bản trộn khi một cảnh hiển thị nó đang là chương trình.

# Display / Window capture picker
sources-capture-display-title = Thêm Chụp Màn hình
sources-capture-window-title = Thêm Chụp Cửa sổ
sources-capture-looking = Đang tìm nguồn…
sources-capture-none-displays = Không có gì để chụp ở đây — không tìm thấy màn hình nào.
sources-capture-none-windows = Không có gì để chụp ở đây — không tìm thấy cửa sổ nào.
sources-capture-portal-note = Trên Wayland, hộp thoại hệ thống chọn màn hình hoặc cửa sổ — ứng dụng không thể chụp toàn cục ở đó, nên đó là con đường trung thực (và duy nhất).
sources-capture-window-note = Bản xem trước cập nhật trực tiếp. Cửa sổ thu nhỏ hiển thị khung hình cuối cùng (hoặc không có) cho đến khi bạn khôi phục nó.
sources-thumb-no-preview = không có xem trước
sources-thumb-loading = đang tải…

# Video Capture Device picker
sources-webcam-title = Thêm Thiết bị Chụp Video
sources-webcam-looking = Đang tìm camera…
sources-webcam-none = Không tìm thấy camera hoặc card thu hình nào.
sources-webcam-format-label = Định dạng
sources-webcam-format-auto-loading = Tự động (đang tải định dạng…)
sources-webcam-format-auto = Tự động (độ phân giải cao nhất)
sources-webcam-card-presets-label = Cài đặt sẵn của card:
sources-webcam-preset-title = Chọn chế độ { $label } mà card này công bố
sources-webcam-add = Thêm camera

# Audio Input / Output capture picker
sources-audio-output-title = Thêm Chụp Đầu ra Âm thanh
sources-audio-input-title = Thêm Chụp Đầu vào Âm thanh
sources-audio-default-output = Đầu ra mặc định (âm thanh bạn nghe)
sources-audio-default-input = Đầu vào mặc định
sources-audio-looking = Đang tìm thiết bị âm thanh…
sources-audio-none-output = Không tìm thấy thiết bị chụp âm thanh máy tính nào ở đây.
sources-audio-none-input = Không tìm thấy micro hoặc đầu vào line nào.
sources-audio-input-note = Các dải trộn có VU-mét, fader, tắt tiếng, giám sát, bộ lọc (khử ồn, cổng, nén…), và gán track. Mọi thứ ở lại trên máy này.

# Application Audio picker
sources-appaudio-title = Thêm Âm thanh Ứng dụng
sources-appaudio-looking = Đang tìm ứng dụng đang phát âm thanh…
sources-appaudio-none = Hiện không có ứng dụng nào đang phát âm thanh — bắt đầu phát trong ứng dụng, rồi làm mới.
sources-appaudio-refresh = ⟳ Làm mới
sources-appaudio-note = Chụp chính xác âm thanh của ứng dụng đó — với VU, fader, tắt tiếng, bộ lọc và track riêng của nó.

# Game Capture picker
sources-game-title = Chụp Trò chơi
sources-game-checking = Đang kiểm tra…
sources-game-use-portal = Dùng Chụp Màn hình (Portal)
sources-game-use-window = Dùng Chụp Cửa sổ thay thế

# Image picker
sources-image-title = Thêm Ảnh
sources-image-file-label = Tệp ảnh (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Thêm ảnh

# Path field
sources-browse = Duyệt…

# Media picker
sources-media-title = Thêm Media
sources-media-file-label = Tệp media (mp4, mkv, webm, mov, .frec, hoặc một ảnh)
sources-media-loop-label = Lặp (bắt đầu lại từ đầu khi kết thúc)
sources-media-note = .frec phát qua codec freally-video sở hữu riêng — không cần tải gì. Các định dạng đường truyền (mp4/mkv/webm/…) giải mã qua thành phần FFmpeg theo yêu cầu; âm thanh của nó vào bộ trộn như một dải riêng.
sources-media-add = Thêm media

# Invite expiry options
sources-ttl-15min = 15 phút
sources-ttl-30min = 30 phút
sources-ttl-1hour = 1 giờ
sources-ttl-1day = 1 ngày

# Remote Guest form
sources-remote-copy-failed = không sao chép được — chọn liên kết và sao chép thủ công
sources-remote-join-failed = tham gia thất bại: { $error }
sources-remote-title = Khách Từ xa (thử nghiệm P2P)
sources-remote-host-heading = Chủ phòng — mời một khách
sources-remote-start-hosting = Bắt đầu làm chủ phòng
sources-remote-expires-label = Hết hạn
sources-remote-invite-expiry-aria = Thời gian hết hạn của lời mời
sources-remote-invite-link-aria = Liên kết mời
sources-remote-copied = Đã sao chép ✓
sources-remote-copy = Sao chép
sources-remote-share-note = Chia sẻ liên kết này (Discord / tin nhắn / email). Nó mang phiên của bạn và hết hạn theo cài đặt. Khách mở nó và tham gia bằng webcam của họ.
sources-remote-qr-note = Quét trên điện thoại để tham gia thẳng từ trình duyệt — camera + micro, không cần cài đặt. Liên kết freally:// có thể sao chép ở trên sẽ mở trong Freally Capture trên máy đã cài nó.
sources-remote-guest-heading = Khách — tham gia bằng lời mời
sources-remote-paste-placeholder = dán liên kết mời
sources-remote-invite-input-aria = Liên kết mời hoặc id phiên
sources-remote-join = Tham gia bằng webcam
sources-remote-session-note = Điều khiển phiên trực tiếp (tắt tiếng, kết thúc) ở lại trên thanh đầu cửa sổ chính — bạn có thể đóng hộp thoại này.
sources-remote-stop-session = Dừng phiên

# Invite QR
sources-invite-qr-aria = Mã QR liên kết mời

# Remote device pickers
sources-devices-output-unavailable = định tuyến đầu ra không khả dụng — đang phát trên thiết bị mặc định
sources-devices-mic-test-failed = kiểm tra micro thất bại: { $error }
sources-devices-heading = Thiết bị âm thanh của phiên
sources-devices-microphone-label = Micro
sources-devices-microphone-aria = Micro của phiên
sources-devices-system-default = Mặc định của hệ thống
sources-devices-output-label = Đầu ra
sources-devices-output-aria = Đầu ra âm thanh của phiên
sources-devices-stop-test = Dừng kiểm tra
sources-devices-test = Kiểm tra — nghe chính bạn
sources-devices-testing-note = nói vào micro — bạn đang nghe các thiết bị đã chọn trực tiếp
sources-devices-idle-note = lặp micro của bạn ra đầu ra (tai nghe tránh hú)

# TURN relay section
sources-turn-save-failed = không lưu được: { $error }
sources-turn-summary = Mạng — relay TURN tùy chọn (nâng cao)
sources-turn-note-1 = Các phiên kết nối trực tiếp (P2P) — miễn phí, không cần relay. Nếu CẢ HAI bên đều nằm sau NAT nghiêm ngặt, đường trực tiếp có thể thất bại; khi đó một relay TURN bạn tự chạy sẽ mang media. Bỏ qua phần này cũng ổn — hầu hết kết nối đều chạy trực tiếp.
sources-turn-note-2 = Lựa chọn miễn phí: Oracle Cloud "Always Free" chạy coturn miễn phí (lưu ý: Oracle yêu cầu thẻ tín dụng khi đăng ký, nhưng gói Always-Free vẫn miễn phí). Các bước: 1) tạo VM miễn phí, 2) cài coturn, 3) mở UDP 3478, 4) đặt user/mật khẩu, 5) nhập turn:your-vm-ip:3478 + thông tin đăng nhập ở đây. Thông tin đăng nhập của bạn ở lại trong tệp cài đặt cục bộ và không bao giờ được ghi log.
sources-turn-url-label = URL TURN
sources-turn-url-placeholder = turn:host:3478 (trống = chỉ trực tiếp)
sources-turn-url-aria = URL TURN
sources-turn-username-label = Tên đăng nhập
sources-turn-username-aria = Tên đăng nhập TURN
sources-turn-credential-label = Thông tin đăng nhập
sources-turn-credential-aria = Thông tin đăng nhập TURN
sources-turn-note-3 = Relay hoạt động khi cả ba trường được đặt (máy chủ TURN yêu cầu thông tin đăng nhập) và áp dụng cho phiên tiếp theo bạn bắt đầu hoặc tham gia. Xác minh nó bằng một cuộc gọi thử chỉ-relay giữa hai máy của chính bạn.
sources-turn-settings-unavailable = cài đặt không khả dụng (chế độ trình duyệt)

# Color picker
sources-color-title = Thêm Màu
sources-color-label = Màu
sources-color-width-label = Chiều rộng
sources-color-height-label = Chiều cao
sources-color-add = Thêm màu
sources-testsignal-title = Thêm tín hiệu thử
sources-testsignal-pattern-label = Mẫu
sources-testsignal-bars = Thanh màu SMPTE
sources-testsignal-grid = Lưới hiệu chuẩn
sources-testsignal-sweep = Quét chuyển động
sources-testsignal-tone = Âm 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Chớp + bíp đồng bộ A/V
sources-testsignal-note = Kiểm tra cảnh, bộ mã hóa, máy chiếu và đích phát mà không cần cắm camera. Mẫu chớp + bíp cấp cho bàn hiệu chuẩn đồng bộ A/V.
sources-testsignal-add = Thêm tín hiệu thử
sources-timer-title = Thêm hẹn giờ
sources-timer-mode-label = Chế độ
sources-timer-wall-clock = Đồng hồ treo tường
sources-timer-countdown = Đếm ngược
sources-timer-stopwatch = Bấm giờ
sources-timer-since-live = Thời gian từ khi lên sóng
sources-timer-since-recording = Thời gian từ khi ghi
sources-timer-note = Thời lượng, định dạng, kiểu dáng và hành động khi hết đếm ngược nằm trong Thuộc tính của nguồn.
sources-timer-add = Thêm hẹn giờ

# Text picker
sources-text-title = Thêm Văn bản
sources-text-label = Văn bản
sources-text-default = Văn bản
sources-text-color-label = Màu
sources-text-color-aria = Màu văn bản
sources-text-size-label = Cỡ (px)
sources-text-note = Họ phông chữ, canh lề, ngắt dòng và RTL nằm trong Thuộc tính của nguồn. Noto Sans đi kèm (gồm cả Ả Rập/Do Thái) là mặc định — giống hệt nhau trên mọi máy.
sources-text-add = Thêm văn bản

# Existing source picker
sources-existing-title = Thêm một nguồn hiện có
sources-existing-empty = Chưa có nguồn nào — thêm một nguồn vào bất kỳ cảnh nào trước. Các nguồn hiện có được chia sẻ: đổi tên hoặc cấu hình lại một nguồn sẽ cập nhật mọi cảnh hiển thị nó.

# Screen + corners layout
sources-slot-off = Tắt
sources-slot-center = Trung tâm (màn hình)
sources-slot-top-left = Trên-Trái
sources-slot-top-right = Trên-Phải
sources-slot-bottom-left = Dưới-Trái
sources-slot-bottom-right = Dưới-Phải
sources-layout-title = Sắp xếp: Màn hình + các góc
sources-layout-empty = Thêm một chụp màn hình và một hoặc nhiều camera vào cảnh này trước, rồi sắp xếp chúng ở đây.
sources-layout-note = Đặt một màn hình ở trung tâm và tối đa bốn camera ở các góc — bố cục giải thích / podcast của bạn. Mỗi góc chứa một webcam, một cửa sổ cuộc gọi đã chụp, hoặc một clip media. Bạn có thể kéo bất kỳ cái nào trên khung hình sau đó.
sources-layout-slot-aria = Vị trí cho { $name }
sources-layout-apply = Áp dụng bố cục


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Điều khiển
controls-start-stop-title-stop = Dừng và hoàn tất bản ghi
controls-start-stop-title-start = Ghi luồng chương trình với cấu hình Cài đặt → Đầu ra
controls-finalizing = ◌ Đang hoàn tất…
controls-stop-recording = ■ Dừng Ghi
controls-start-recording = ● Bắt đầu Ghi
controls-marker-title = Thả một marker chương tại thời điểm này — nó vào BẢN GHI (chương mkv, hoặc tệp đi kèm). Marker luồng phía nền tảng cần tài khoản nền tảng, thứ mà ứng dụng này không bao giờ hỏi.
controls-marker = ◈ Marker
controls-pause-title-resume = Tiếp tục — tệp tiếp tục như một dòng thời gian liền mạch
controls-pause-title-pause = Tạm dừng — không khung hình nào được ghi; tiếp tục sẽ nối vào cùng tệp phát được
controls-resume-recording = ▶ Tiếp tục Ghi
controls-pause-recording = ⏸ Tạm dừng Ghi
controls-reactions-label = Phản ứng (gắn sẵn vào chương trình)
controls-reactions-title = Cho một phản ứng nổi lên trên chương trình — được ghi VÀ phát trực tiếp, nên bản phát lại hiển thị đúng khoảnh khắc đó. Người xem trong chat cũng kích hoạt chúng (emoji phản ứng của họ tự động nổi lên); cơn lũ chỉ giới hạn những gì hiển thị trên màn hình.
controls-react = Phản ứng { $emoji }
controls-virtual-camera-title = Camera ảo cần thành phần trình điều khiển đã ký riêng cho mỗi HĐH (Win11 MFCreateVirtualCamera / Win10 DirectShow / phần mở rộng CoreMediaIO macOS / v4l2loopback Linux) — nó ra mắt như cột mốc riêng. Mô hình luồng đã sẵn sàng cho nó: chương trình, khung hình dọc, hoặc một nguồn duy nhất, kèm micro ảo ghép đôi trên Windows/Linux (macOS không có API micro ảo — nói thật lòng).
controls-virtual-camera = ⌁ Khởi động Camera Ảo
controls-saved = Đã lưu: { $path }

# --- MixerDock.tsx ---
mixer-title = Bộ trộn Âm thanh
mixer-monitor-error = giám sát: { $error }
mixer-switch-to-horizontal = Chuyển sang dải ngang
mixer-switch-to-vertical = Chuyển sang dải dọc
mixer-layout-aria-vertical = Bố cục bộ trộn: dọc — chuyển sang ngang
mixer-layout-aria-horizontal = Bố cục bộ trộn: ngang — chuyển sang dọc
mixer-empty = Không có nguồn âm thanh trong cảnh này — thêm Chụp Đầu vào Âm thanh (micro) hoặc Chụp Đầu ra Âm thanh (âm thanh máy tính) bằng “+” trong Nguồn. Các dải có VU-mét, fader, tắt tiếng, giám sát, bộ lọc và gán track.
mixer-advanced-title = Âm thanh — { $name }
mixer-loudness-label = Độ lớn chương trình (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Độ lớn tức thời (400 ms)
mixer-short-term-title = Độ lớn ngắn hạn (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Giám sát
mixer-monitor-device-aria = Thiết bị đầu ra giám sát
mixer-default-output = Đầu ra mặc định

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Bộ nhớ
stats-dropped = Bị bỏ
stats-render = Dựng hình
stats-gpu = GPU
stats-gpu-compositing = đang dựng hình
stats-gpu-idle = nhàn rỗi
stats-vertical-fps = FPS 9:16
stats-targets-label = Đích luồng
stats-shared-encode = · mã hóa chung
stats-starting = Đang khởi động bộ dựng hình…

# --- ScenesRail.tsx ---
scenes-title = Cảnh
scenes-new-scene-name = Cảnh
scenes-add = Thêm một cảnh
scenes-empty = Đang kết nối tới lõi studio…
scenes-rename = Đổi tên { $name }
scenes-on-program = Trên chương trình
scenes-preview = Xem trước { $name }
scenes-switch-to = Chuyển sang { $name }
scenes-move-up = Di chuyển lên
scenes-move-up-aria = Di chuyển { $name } lên
scenes-move-down = Di chuyển xuống
scenes-move-down-aria = Di chuyển { $name } xuống
scenes-last-stays = Cảnh cuối cùng ở lại
scenes-remove = Xóa cảnh này
scenes-remove-aria = Xóa { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Mức
channelstrip-monitor-off = Tắt giám sát
channelstrip-monitor-only = Chỉ giám sát (không trong bản trộn)
channelstrip-monitor-and-output = Giám sát và đầu ra
channelstrip-status-error = lỗi
channelstrip-status-live = trực tiếp
channelstrip-status-waiting-audio = đang chờ âm thanh
channelstrip-status = trạng thái: { $state }
channelstrip-status-waiting = đang chờ
channelstrip-mute = Tắt tiếng
channelstrip-unmute = Bật tiếng
channelstrip-mute-source = Tắt tiếng { $name }
channelstrip-unmute-source = Bật tiếng { $name }
channelstrip-scene-mix-on = Bản trộn theo cảnh BẬT — dải này ghi đè bản trộn toàn cục cho cảnh này (nhấp để theo lại bản trộn toàn cục)
channelstrip-scene-mix-off = Bản trộn theo cảnh — cho dải này fader/tắt tiếng riêng cho cảnh hiện tại
channelstrip-scene-mix-label = Bản trộn theo cảnh cho { $name }
channelstrip-monitor-cycle = { $mode } — nhấp để đổi
channelstrip-monitor-mode = Chế độ giám sát của { $name }: { $mode }
channelstrip-audio-filters-title = Bộ lọc âm thanh (khử ồn, cổng, nén…)
channelstrip-audio-filters-label = Bộ lọc âm thanh cho { $name }
channelstrip-advanced-title = Bù trễ đồng bộ & phím nhấn-để-nói
channelstrip-advanced-label = Cài đặt âm thanh nâng cao cho { $name }
channelstrip-track-assignment = Gán track
channelstrip-track = Track { $n }
channelstrip-track-assigned = Track { $n } (đã gán)
channelstrip-track-label = Track { $n } cho { $name }
channelstrip-device-error = lỗi thiết bị
channelstrip-audio-device-error = lỗi thiết bị âm thanh
channelstrip-volume-label = Âm lượng của { $name } tính bằng decibel
channelstrip-ptt-hold = Nhấn-để-nói: giữ { $key }
channelstrip-sync-offset = Bù đồng bộ (ms, 0–{ $max } — làm trễ âm thanh này)
channelstrip-solo-title = Solo (PFL) — kênh nghe kiểm chỉ phát các dải được solo; mix chương trình không đổi
channelstrip-solo-source = Solo { $name } (PFL)
channelstrip-pan-label = Cân bằng (nhấp đúp để đặt lại)
channelstrip-pan-aria = Cân bằng của { $name }
channelstrip-mono-label = Trộn xuống mono
channelstrip-ptt-hotkey = Phím nhấn-để-nói (im lặng trừ khi giữ)
channelstrip-ptt-placeholder = ví dụ Ctrl+Shift+T hoặc F13
channelstrip-ptt-aria = Phím nhấn-để-nói
channelstrip-ptm-hotkey = Phím nhấn-để-tắt-tiếng (im lặng khi đang giữ)
channelstrip-ptm-placeholder = ví dụ Ctrl+Shift+M
channelstrip-ptm-aria = Phím nhấn-để-tắt-tiếng
channelstrip-hotkeys-note = Phím tắt hoạt động khi ứng dụng khác đang được lấy nét. Trên Linux/Wayland, phím tắt toàn cục có thể không khả dụng — đó là giới hạn của trình dựng cửa sổ, nói thật lòng.
channelstrip-apply = Áp dụng


# --- LiveButton.tsx ---
livebutton-failure-ended = luồng đã kết thúc
livebutton-title-live = Kết thúc luồng — mọi đích (bản ghi đang chạy vẫn tiếp tục)
livebutton-title-offline = Phát trực tiếp tới mọi đích Cài đặt → Luồng đã bật
livebutton-end-stream = ■ Kết thúc Luồng
livebutton-aria-reconnecting = Đang kết nối lại
livebutton-aria-live = Trực tiếp
livebutton-badge-retry = thử lại { $n }
livebutton-badge-live = trực tiếp
livebutton-go-live = ⦿ Phát Trực tiếp


# --- RecDot.tsx ---
recdot-paused-aria = Đã tạm dừng ghi
recdot-recording-aria = Đang ghi
recdot-tracks-one = đang ghi { $count } track âm thanh
recdot-tracks-other = đang ghi { $count } track âm thanh
recdot-paused = tạm dừng


# --- ReplayControls.tsx ---
replaycontrols-saved = Đã lưu phát lại — { $name }
replaycontrols-failure-stopped = bộ đệm đã dừng
replaycontrols-title-disarm = Hủy kích hoạt bộ đệm phát lại (bỏ lịch sử chưa lưu)
replaycontrols-title-arm = Kích hoạt bộ đệm phát lại cuộn — giữ N giây cuối sẵn sàng để lưu (mã hóa nhẹ riêng của nó; luồng và bản ghi không bị ảnh hưởng)
replaycontrols-replay-seconds = ⟲ Phát lại { $seconds }s
replaycontrols-arm = ⟲ Kích hoạt Bộ đệm Phát lại
replaycontrols-save-title = Lưu N giây cuối vào thư mục bản ghi (cũng có trên phím tắt Lưu-Phát-lại)
replaycontrols-save = ⤓ Lưu


# --- PropertiesDialog.tsx ---
properties-title = Thuộc tính — { $name }
properties-name = Tên
properties-cancel = Hủy
properties-apply = Áp dụng
properties-youtube = YouTube — URL kênh / watch / live_chat (không cần khóa, không đăng nhập, không bao giờ)
properties-twitch = Twitch — tên kênh (ẩn danh)
properties-kick = Kick — slug kênh (điểm cuối công khai)
properties-width-px = Chiều rộng (px)
properties-lines = Dòng
properties-font-px = Phông (px)
properties-images = Tệp ảnh (mỗi đường dẫn một dòng, hiển thị theo thứ tự)
properties-per-slide = Mỗi slide (ms)
properties-crossfade = Chuyển mờ (ms, 0 = cắt)
properties-loop-slideshow = Lặp (tắt = giữ slide cuối)
properties-shuffle = Xáo trộn mỗi vòng
properties-nested-scene = Cảnh mà nguồn này soạn (một cảnh đã chứa cảnh này sẽ bị từ chối)
properties-portal-note = Portal ScreenCast của Wayland chọn màn hình hoặc cửa sổ trong hộp thoại hệ thống mỗi khi nguồn này khởi động — không có gì để cấu hình ở đây, theo thiết kế.
properties-appaudio-capturing = Đang chụp âm thanh từ { $exe }
properties-appaudio-exe-fallback = một ứng dụng
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Thêm lại nguồn để nhắm tới ứng dụng khác (id tiến trình thay đổi khi ứng dụng khởi động lại).
properties-image-file = Tệp ảnh
properties-media-file = Tệp media (mp4, mkv, webm, mov, .frec, hoặc một ảnh)
properties-media-loop = Lặp (bắt đầu lại từ đầu khi kết thúc)
properties-media-hwdecode = Giải mã phần cứng (tự động chuyển về phần mềm khi cần)
properties-media-note = .frec phát qua codec freally-video sở hữu riêng — không cần tải gì. Các định dạng video khác giải mã qua thành phần FFmpeg theo yêu cầu. Âm thanh của tệp có dải trộn riêng; bù đồng bộ của dải tinh chỉnh căn chỉnh A/V. Clip không có âm thanh để dải của nó im lặng.
properties-color = Màu
properties-width = Chiều rộng
properties-height = Chiều cao
properties-testtone-note = Sóng sin 1 kHz liên tục ở −20 dBFS. Âm lượng và tắt tiếng nằm trên dải mixer; không còn gì khác để cấu hình.
properties-timer-format = Định dạng giờ (strftime)
properties-timer-format-note = vd. %H:%M:%S (mặc định), %I:%M %p, %A %H:%M — mẫu sai sẽ quay về %H:%M:%S.
properties-timer-utc = Chênh lệch UTC (phút)
properties-timer-utc-placeholder = giờ địa phương
properties-timer-duration = Thời lượng (giây)
properties-timer-target = Đếm ngược tới (HH:MM)
properties-timer-target-note = Mốc giờ treo tường tự chạy và lặp mỗi ngày; để trống để dùng thời lượng với Bắt đầu/Tạm dừng/Đặt lại.
properties-timer-end = Khi về 0
properties-timer-end-none = Không làm gì
properties-timer-end-flash = Nhấp nháy hẹn giờ
properties-timer-end-switch = Chuyển cảnh
properties-timer-end-scene = Cảnh
properties-timer-size = Cỡ (px)
properties-timer-start = Bắt đầu
properties-timer-pause = Tạm dừng
properties-timer-reset = Đặt lại
properties-text-file = Đọc từ tệp (đường dẫn; trống = dùng văn bản ở trên)
properties-text-binding = Phân tích thành
properties-text-binding-whole = Cả tệp
properties-text-binding-csv = Ô CSV
properties-text-binding-json = Con trỏ JSON
properties-text-csv-row = Hàng
properties-text-csv-column = Cột
properties-text-csv-column-placeholder = tên hoặc số
properties-text-json-pointer = Con trỏ
properties-text-file-note = Tệp được đọc lại trong nửa giây sau khi thay đổi. Ghi nguyên tử (tệp tạm + đổi tên) được dung nạp: giá trị tốt cuối cùng vẫn hiển thị trong lúc hoán đổi.
avsync-title = Hiệu chuẩn đồng bộ A/V
avsync-intro = Phát mẫu chớp + bíp tích hợp qua màn hình và loa, thu lại bằng camera và micro bạn muốn căn chỉnh — bàn hiệu chuẩn sẽ đo độ lệch. Vòng lặp đi qua màn hình và loa nên độ trễ nhỏ của chúng cũng được tính.
avsync-video-label = Camera (nguồn video)
avsync-audio-label = Micro (nguồn âm thanh)
avsync-pick = Chọn nguồn…
avsync-no-video = Hãy thêm camera làm nguồn trước — bàn hiệu chuẩn đo nguồn, không đo thiết bị thô.
avsync-no-audio = Hãy thêm micro làm nguồn âm thanh trước.
avsync-projector = Toàn màn hình chương trình trên
avsync-projector-open = Mở máy chiếu
avsync-projector-window-title = Chương trình — đồng bộ A/V
avsync-start-note = Khi bắt đầu, một nguồn "Mẫu đồng bộ A/V" tạm thời được thêm lên trên cảnh hiện tại và tiếng bíp phát trên thiết bị nghe kiểm. Mọi thứ được gỡ khi kết thúc.
avsync-manual = Độ lệch đồng bộ (ms, thủ công)
avsync-start = Bắt đầu hiệu chuẩn
avsync-measuring = Đang đo khoảng 12 giây — hướng camera vào chương trình đang chớp và giữ căn phòng yên tĩnh…
avsync-flash-seen = Camera thấy chớp sáng
avsync-flash-waiting = Đang chờ camera thấy chớp sáng…
avsync-beep-heard = Micro nghe thấy tiếng bíp
avsync-beep-waiting = Đang chờ micro nghe thấy tiếng bíp…
avsync-cancel = Hủy
avsync-result-offset = Video đến sau âm thanh { $offset } ms.
avsync-result-detail = Đo qua { $cycles } chu kỳ, ±{ $jitter } ms.
avsync-negative = Âm thanh vốn đã đến sau video. Làm trễ âm thanh không sửa được chiều này — nếu dải khác mang tiếng của camera này, hãy giảm độ lệch ở đó.
avsync-over-cap = Độ lệch đo được vượt trần { $max } ms. Chênh lệch cỡ đó thường do chọn sai nguồn — kiểm tra chuỗi rồi đo lại.
avsync-applied = Đã áp dụng — độ lệch đồng bộ của micro giờ là { $offset } ms.
avsync-apply = Áp dụng { $offset } ms cho micro
avsync-again = Đo lại
avsync-close = Đóng
avsync-error-noFlash = Camera chưa từng thấy chớp sáng. Hướng nó vào chương trình đang chớp (toàn màn hình sẽ dễ hơn), bảo đảm nguồn đang chạy rồi đo lại.
avsync-error-noBeep = Micro chưa từng nghe tiếng bíp. Bảo đảm thiết bị nghe kiểm phát ra tiếng và micro đang chạy (không bị chặn bởi nhấn-để-nói), rồi đo lại.
avsync-error-tooFewCycles = Chưa đủ chu kỳ chớp/bíp sạch. Giữ mẫu hiển thị rõ và nghe rõ trong suốt lần đo.
avsync-error-notThePattern = Những gì thấy hoặc nghe không lặp theo nhịp của mẫu — nhiều khả năng là ánh sáng hay tiếng ồn trong phòng, không phải tín hiệu thử.
avsync-error-unstable = Các chu kỳ lệch nhau quá nhiều để tin một con số. Cố định camera, giảm tiếng ồn rồi đo lại.
hotkey-audit-title = Bản đồ phím tắt
hotkey-audit-search = Tìm kiếm
hotkey-audit-filter = Tính năng
hotkey-audit-filter-all = Mọi tính năng
hotkey-audit-col-key = Phím
hotkey-audit-col-action = Hành động
hotkey-audit-col-where = Ở đâu
hotkey-audit-col-status = Trạng thái
hotkey-audit-ok = Ổn
hotkey-audit-shared = { $count } gán phím dùng chung
hotkey-audit-unregistered = Chưa đăng ký với hệ điều hành (bị chiếm nơi khác hoặc không khả dụng)
hotkey-audit-invalid = Phím tắt không hợp lệ
hotkey-audit-empty = Chưa có phím tắt — gán trong Cài đặt → Phím tắt hoặc trên dải mixer.
hotkey-audit-export = Xuất bảng ghi nhớ
hotkey-audit-exported = Đã lưu vào { $path }
hotkey-audit-note = Gán và đổi phím trong Cài đặt → Phím tắt (hành động toàn cục) và trên từng dải mixer (nhấn-để-nói / nhấn-để-tắt); bảng này kiểm tra và ghi lại chúng.
hotkey-audit-action-record = Bật/tắt ghi hình
hotkey-audit-action-go-live = Bật/tắt phát sóng
hotkey-audit-action-transition = Thực hiện chuyển cảnh
hotkey-audit-action-save-replay = Lưu replay
hotkey-audit-action-add-marker = Thêm điểm đánh dấu
hotkey-audit-action-still = Chụp ảnh tĩnh
hotkey-audit-action-panic = Màn hình khẩn cấp
hotkey-audit-action-timer-toggle = Bắt đầu/tạm dừng mọi hẹn giờ
hotkey-audit-action-timer-reset = Đặt lại mọi hẹn giờ
hotkey-audit-action-ptt = Nhấn để nói
hotkey-audit-action-ptm = Nhấn để tắt tiếng
hotkey-audit-feature-recording = Ghi hình
hotkey-audit-feature-streaming = Phát sóng
hotkey-audit-feature-studio = Chế độ studio
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Điểm đánh dấu
hotkey-audit-feature-stills = Ảnh tĩnh
hotkey-audit-feature-panic = Khẩn cấp
hotkey-audit-feature-timers = Hẹn giờ
hotkey-audit-feature-audio = Âm thanh (theo nguồn)
properties-text = Văn bản
properties-font-family = Họ phông chữ (hệ thống; để trống = mặc định)
properties-size-px = Cỡ (px)
properties-text-color = Màu văn bản
properties-align = Canh lề
properties-align-left = trái
properties-align-center = giữa
properties-align-right = phải
properties-line-spacing = Giãn dòng
properties-wrap-width = Chiều rộng ngắt dòng (px; 0 = tắt)
properties-force-rtl = Buộc phải-sang-trái
properties-text-note = Kết xuất dùng shaping thật (nối chữ Ả Rập, ligature) và sắp xếp dòng bidi. Họ phông Noto Sans đi kèm (gồm cả Ả Rập/Do Thái) là mặc định; các họ phông hệ thống cũng dùng được. CJK hiện dùng phông hệ thống.
properties-repick-capturing = Đang chụp: { $label }
properties-repick-looking = Đang tìm nguồn…
properties-repick-none-displays = Không tìm thấy màn hình nào để chọn lại.
properties-repick-none-windows = Không tìm thấy cửa sổ nào để chọn lại.
properties-repick-again = Chọn lại:
properties-device = Thiết bị
properties-video-current-device = (thiết bị hiện tại)
properties-format = Định dạng
properties-format-auto-loading = Tự động (đang tải định dạng…)
properties-deinterlace = Khử xen kẽ
properties-deinterlace-off = Tắt
properties-deinterlace-discard = Loại bỏ (nhân đôi dòng một field)
properties-deinterlace-bob = Bob (luân phiên field)
properties-deinterlace-linear = Tuyến tính (nội suy)
properties-deinterlace-blend = Trộn (trung bình hai field)
properties-deinterlace-adaptive = Thích ứng chuyển động (lớp yadif)
properties-field-order = Thứ tự field
properties-field-order-top = Field trên trước
properties-field-order-bottom = Field dưới trước
properties-deinterlace-note = Dành cho tín hiệu xen kẽ từ card thu. CPU thuần, giống nhau trên mọi HĐH; thay đổi sẽ khởi động lại thiết bị (như đổi định dạng).
camera-controls-title = Điều khiển camera
camera-controls-refresh = Làm mới
camera-controls-reset = Đặt lại hồ sơ
camera-controls-empty = Hiện chưa có điều khiển — thiết bị phải đang phát (thêm vào cảnh trước), và một số backend không báo gì (nhất là macOS). Đây là trạng thái trung thực theo từng HĐH.
camera-controls-note = Thay đổi áp dụng ngay và được lưu vào hồ sơ thiết bị, áp dụng lại khi cắm lại và khởi động lại.
camera-control-brightness = Độ sáng
camera-control-contrast = Tương phản
camera-control-hue = Sắc độ
camera-control-saturation = Bão hòa
camera-control-sharpness = Độ nét
camera-control-gamma = Gamma
camera-control-white-balance = Cân bằng trắng
camera-control-backlight = Bù ngược sáng
camera-control-gain = Khuếch đại
camera-control-pan = Quay ngang
camera-control-tilt = Nghiêng
camera-control-zoom = Thu phóng
camera-control-exposure = Phơi sáng
camera-control-iris = Khẩu độ
camera-control-focus = Lấy nét
properties-format-auto = Tự động (độ phân giải cao nhất)
properties-audio-capture-of = Chụp âm thanh của
properties-audio-default-output = Đầu ra mặc định (âm thanh bạn nghe)
properties-audio-default-input = Đầu vào mặc định
properties-audio-default-suffix = (mặc định)
properties-audio-current-device = (thiết bị hiện tại: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Khuếch đại (Gain)
audiofilters-name-noise-gate = Cổng Nhiễu (Noise Gate)
audiofilters-name-compressor = Bộ nén (Compressor)
audiofilters-name-limiter = Bộ giới hạn (Limiter)
audiofilters-name-eq = EQ 3-Băng
audiofilters-name-denoise = Khử ồn
audiofilters-name-ducking = Ducking
audiofilters-title = Bộ lọc âm thanh — { $name }
audiofilters-chain-header = Chuỗi bộ lọc (trên cùng chạy trước, trước fader)
audiofilters-add = + Thêm bộ lọc
audiofilters-add-menu = Thêm một bộ lọc âm thanh
audiofilters-empty = Chưa có bộ lọc nào — khử ồn micro (DSP cổ điển, không ML), gate căn phòng, kiềm đỉnh bằng bộ nén, hoặc duck nhạc dưới giọng bạn.
audiofilters-enable = Bật { $name }
audiofilters-run-earlier = Chạy sớm hơn
audiofilters-move-up = Di chuyển { $name } lên
audiofilters-run-later = Chạy muộn hơn
audiofilters-move-down = Di chuyển { $name } xuống
audiofilters-remove-title = Xóa bộ lọc
audiofilters-remove = Xóa { $name }
audiofilters-gain-db = Khuếch đại (dB)
audiofilters-open-db = Mở tại (dB)
audiofilters-close-db = Đóng tại (dB)
audiofilters-attack-ms = Attack (ms)
audiofilters-hold-ms = Giữ (ms)
audiofilters-release-ms = Release (ms)
audiofilters-ratio = Tỷ lệ (:1)
audiofilters-threshold-db = Ngưỡng (dB)
audiofilters-output-gain-db = Khuếch đại đầu ra (dB)
audiofilters-ceiling-db = Trần (dB)
audiofilters-low-db = Thấp (dB)
audiofilters-mid-db = Trung (dB)
audiofilters-high-db = Cao (dB)
audiofilters-strength = Cường độ
audiofilters-denoise-note = Khử phổ DSP cổ điển sở hữu riêng — nhiễu ổn định (quạt, xì) giảm trong khi giọng nói đi qua. Không ML, không mô hình, theo điều lệ.
audiofilters-duck-under = Duck dưới
audiofilters-ducking-trigger = Nguồn kích hoạt ducking
audiofilters-pick-trigger = (chọn một nguồn kích hoạt — ví dụ micro của bạn)
audiofilters-trigger-at-db = Kích hoạt tại (dB)
audiofilters-duck-by-db = Duck xuống (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Khóa Chroma (Chroma Key)
filters-name-color-key = Khóa Màu (Color Key)
filters-name-luma-key = Khóa Luma (Luma Key)
filters-name-render-delay = Trễ Kết xuất
filters-name-color-correction = Hiệu chỉnh Màu
filters-name-lut = Áp dụng LUT
filters-name-blur = Làm mờ
filters-name-mask = Mặt nạ Ảnh
filters-name-sharpen = Làm nét
filters-name-scroll = Cuộn
filters-name-crop = Cắt
filters-title = Bộ lọc — { $name }
filters-blend-mode = Chế độ hòa trộn
filters-chain-header = Chuỗi bộ lọc (trên cùng chạy trước)
filters-add = + Thêm bộ lọc
filters-add-menu = Thêm một bộ lọc
filters-empty = Chưa có bộ lọc nào — khóa chroma một webcam, hiệu chỉnh màu một bản chụp, hoặc cuộn một dòng chữ chạy.
filters-enable = Bật { $name }
filters-run-earlier = Chạy sớm hơn
filters-move-up = Di chuyển { $name } lên
filters-run-later = Chạy muộn hơn
filters-move-down = Di chuyển { $name } xuống
filters-remove-title = Xóa bộ lọc
filters-remove = Xóa { $name }
filters-key-color-rgb = Màu khóa (bất kỳ màu nào, khoảng cách RGB)
filters-similarity = Độ tương đồng
filters-smoothness = Độ mượt
filters-luma-min = Luma tối thiểu (khóa vùng tối hơn ra)
filters-luma-max = Luma tối đa (khóa vùng sáng hơn ra)
filters-delay = Trễ (ms — chỉ video, ví dụ để đồng bộ với âm thanh; giới hạn ở 500)
filters-key-color = Màu khóa
filters-spill = Tràn màu (Spill)
filters-gamma = Gamma
filters-brightness = Độ sáng
filters-contrast = Độ tương phản
filters-saturation = Độ bão hòa
filters-hue-shift = Dịch tông màu
filters-opacity = Độ mờ đục
filters-cube-file = tệp .cube
filters-amount = Lượng
filters-radius = Bán kính
filters-mask-image = Ảnh mặt nạ
filters-mask-mode = Chế độ
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = đảo ngược
filters-speed-x = Tốc độ X (px/s)
filters-speed-y = Tốc độ Y (px/s)
filters-crop-left = trái
filters-crop-top = trên
filters-crop-right = phải
filters-crop-bottom = dưới
filters-crop-aria = cắt { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Làm mới
pickershell-refresh-title = Làm mới danh sách
pickershell-close = Đóng


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Báo lỗi
bugreport-intro = Báo cáo là ẩn danh và tự nguyện — không gì được gửi tự động. Bạn sẽ xem lại chính xác nội dung bên dưới, rồi gửi qua một issue GitHub điền sẵn hoặc ứng dụng email của bạn. Không có dữ liệu cá nhân (đường dẫn nhà và tên người dùng được ẩn); không tài khoản, không máy chủ.
bugreport-crash-notice = Freally Capture đã đóng bất ngờ ở lần chạy trước — chi tiết sự cố ẩn danh được đính kèm bên dưới. Báo cáo chúng giúp sửa nhanh.
bugreport-description-label = Bạn đang làm gì khi nó xảy ra? (tùy chọn)
bugreport-description-placeholder = ví dụ bản xem trước bị đơ khi tôi thêm webcam thứ hai
bugreport-include-crash = Bao gồm chi tiết sự cố ẩn danh từ lần chạy trước
bugreport-preview-label = Chính xác những gì sẽ được gửi
bugreport-open-github = Mở issue GitHub
bugreport-gmail-title = Mở cửa sổ soạn thư của Gmail trong trình duyệt, điền sẵn. Chưa đăng nhập? Google hiển thị màn hình đăng nhập trước.
bugreport-compose-gmail = Soạn trong Gmail
bugreport-email-title = Mở bản nháp trong ứng dụng thư mặc định của PC này (Outlook, Thunderbird, Mail…)
bugreport-send-email = Gửi email
bugreport-copied = Đã sao chép ✓
bugreport-copy-report = Sao chép báo cáo
bugreport-dismiss-crash = Bỏ qua sự cố
bugreport-copy-failed = không sao chép được — chọn văn bản và sao chép thủ công
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = ĐÃ XẢY RA GÌ
bugreport-preview-no-description = (không có mô tả)
bugreport-preview-diagnostics = CHẨN ĐOÁN ẨN DANH (không có dữ liệu cá nhân)
bugreport-preview-from = Từ: Freally Capture
bugreport-preview-crash-excerpt = --- trích đoạn sự cố ---


# --- Updates.tsx ---
updates-title = Cập nhật phần mềm
updates-checking = Đang kiểm tra cập nhật…
updates-uptodate = Bạn đang dùng phiên bản mới nhất.
updates-check-again = Kiểm tra lại
updates-available = Phiên bản { $version } đã có
updates-current-version = (bạn đang có { $current })
updates-release-notes-label = Phiên bản { $version } — Ghi chú phát hành
updates-confirm = Bạn muốn cập nhật ngay bây giờ? Bản tải về được xác minh với khóa ký đi kèm trước khi áp dụng. Freally Capture đóng lại, trình cài đặt chạy, và phiên bản mới tự mở lại.
updates-yes-update-now = Có, cập nhật ngay
updates-no-not-now = Không, để sau
updates-downloading = Đang tải { $version }…
updates-starting = đang khởi động…
updates-installed = Đã cài đặt cập nhật.
updates-restart-now = Khởi động lại ngay
updates-restart-later = Khởi động lại sau
updates-try-again = Thử lại


# --- Models.tsx ---
models-title = Thành phần
models-ffmpeg-heading = FFmpeg — codec đường truyền
models-badge-third-party = Bên thứ ba · không đóng gói kèm
models-ffmpeg-desc = Engine riêng của Freally Capture ghi freally-video (.frec) không mất dữ liệu mà không cần gì thêm. Ghi các định dạng đường truyền mà nền tảng và trình phát mong đợi — H.264/AAC (và HEVC/AV1) trong mp4/mkv/mov/webm — dùng FFmpeg, một công cụ riêng mà ứng dụng này không bao giờ đi kèm: các codec đó bị ràng buộc bằng sáng chế, nên nó vẫn tùy chọn và được dán nhãn rõ ràng. Nó được tải về theo yêu cầu từ bản dựng đã ghim bên dưới, xác minh SHA-256 trước lần dùng đầu, lưu bộ nhớ đệm theo người dùng, và chạy như một tiến trình riêng. Giấy phép của nó (LGPL/GPL) là của riêng nó — xem THIRD-PARTY-NOTICES.
models-checking = Đang kiểm tra…
models-ffmpeg-not-installed = Chưa cài đặt. Có sẵn: FFmpeg { $version } từ { $source } (tải { $size }).
models-ffmpeg-none-pinned = Chưa có bản dựng FFmpeg nào được ghim cho nền tảng này — ghi codec đường truyền không khả dụng ở đây. Ghi freally-video không mất dữ liệu không bị ảnh hưởng.
models-ffmpeg-download-verify = Tải & xác minh ({ $size })
models-downloading = Đang tải…
models-download-of = trên
models-cancel = Hủy
models-ffmpeg-verifying = Đang xác minh bản tải về với SHA-256 đã ghim…
models-ffmpeg-extracting = Đang giải nén…
models-ffmpeg-ready = Đã cài & xác minh — { $version }
models-remove = Xóa
models-ffmpeg-retry = Thử tải lại
models-network-note = Bản tải về là hành động mạng duy nhất trên bảng này và không bao giờ tự khởi động. Checksum thất bại sẽ hủy cài đặt — ứng dụng từ chối chạy các byte mà nó không thể bảo đảm.
models-cef-heading = Runtime Nguồn Trình duyệt — Chromium (CEF)
models-cef-desc = Nguồn trình duyệt kết xuất trang web (thông báo, widget, lớp phủ) qua Chromium Embedded Framework — một runtime ~100 MB mà ứng dụng này không bao giờ đi kèm. Nó tải về theo yêu cầu từ chỉ mục bản dựng CEF chính thức, được xác minh với SHA-1 của chỉ mục đó trước khi giải nén bất cứ gì, và lưu bộ nhớ đệm theo người dùng. Nguồn trình duyệt kết xuất qua nó sẽ đến với cột mốc riêng; phần này cài đặt runtime mà nó cần.
models-cef-download-install = Tải & cài đặt
models-cef-unsupported = CEF không phát hành bản dựng nào cho nền tảng này — nguồn trình duyệt không khả dụng ở đây.
models-cef-resolving = Đang phân giải bản dựng ổn định mới nhất…
models-cef-verifying = Đang xác minh bản tải về với SHA-1 của chỉ mục…
models-cef-extracting = Đang giải nén runtime…
models-cef-ready = Đã cài đặt — CEF { $version }.
models-cef-retry = Thử lại
models-integrations-heading = Tích hợp tùy chọn
models-badge-never-bundled = Không bao giờ đóng gói kèm
models-ndi-detected = Đã phát hiện
models-ndi-not-installed = Chưa cài đặt
models-vst-available = Có sẵn
models-vst-not-available = Không có sẵn


# --- Recordings.tsx ---
recordings-title = Bản ghi
recordings-loading = Đang đọc thư mục…
recordings-empty = Chưa có bản ghi nào — Bắt đầu Ghi sẽ viết vào thư mục đặt trong Đầu ra.
recordings-frec-label = không mất dữ liệu sở hữu riêng (freally-video)
recordings-remux-title = Đóng gói lại thành mp4 — sao chép luồng, không mã hóa lại, không đổi chất lượng (cần thành phần FFmpeg)
recordings-remuxing = Đang remux…
recordings-remux-to-mp4 = Remux sang MP4
recordings-export-mp4-title = Giải mã .frec sở hữu riêng và mã hóa lại thành MP4 (H.264/AAC) để phát được trong mọi trình phát — cần thành phần FFmpeg
recordings-exporting = Đang xuất…
recordings-export-mp4 = Xuất → MP4
recordings-export-mkv-title = Giải mã .frec sở hữu riêng và mã hóa lại thành MKV để phát được trong mọi trình phát
recordings-starting = đang khởi động…
recordings-frames = { $done } / { $total } khung
recordings-cancel = Hủy
recordings-export-cancelled = Đã hủy xuất.
recordings-exported-to = Đã xuất tới { $path }
recordings-remuxed-to = Đã remux tới { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Mở bản ghi .frec
openfrec-desc = Freally Capture ghi định dạng .frec không mất dữ liệu sở hữu riêng — nó không phát định dạng này. Freally Player sẽ phát .frec trực tiếp khi ra mắt. Hiện tại, xuất nó sang MP4/MKV và nó phát được trong mọi trình phát (VLC, trình phát của HĐH, bất cứ thứ gì).
openfrec-exported-to = Đã xuất tới { $path }
openfrec-exporting = Đang xuất…
openfrec-starting = đang khởi động…
openfrec-export-mp4 = Xuất → MP4
openfrec-export-mkv = Xuất → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Khung hình dọc (9:16)
vertical-enable = Bật khung hình thứ hai — có thể ghi và phát trực tiếp độc lập với chương trình
vertical-scene-label = Cảnh mà khung hình này soạn
vertical-width = Chiều rộng
vertical-height = Chiều cao
vertical-preview-alt = Xem trước khung hình dọc
vertical-note = Vị trí các mục là pixel-chính-xác trên mọi khung hình: chọn cảnh này trong thanh Cảnh để sắp xếp nó trong khi bản xem trước này hiển thị kết quả dọc. Các đích luồng chọn khung hình này trong ⦿ Luồng…; Cài đặt → Đầu ra có thể ghi nó cùng với tệp chính.
vertical-close = Đóng


# --- EulaGate.tsx ---
eula-title = Freally Capture — Thỏa thuận Cấp phép
eula-version = v{ $version }
eula-intro = Vui lòng đọc và chấp nhận thỏa thuận này để dùng Freally Capture. Tóm lại: đây là một công cụ trung lập, và bạn hoàn toàn chịu trách nhiệm về những gì bạn chụp, ghi và phát sóng — cũng như về việc có quyền đối với chúng.
eula-thanks = Cảm ơn bạn đã đọc.
eula-scroll-hint = Cuộn tới cuối để tiếp tục.
eula-decline = Từ chối & Thoát
eula-agree = Tôi Đồng ý


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Cài đặt — Đầu ra
output-loading = Cài đặt vẫn đang tải…
output-container-frec = freally-video (.frec) — không mất dữ liệu, sở hữu riêng, không cần tải gì
output-container-mkv = MKV — chịu lỗi sự cố; remux sang mp4 sau
output-container-mp4 = MP4 — phát được ở mọi nơi
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Không mất dữ liệu
output-preset-lossless-title = Codec freally-video sở hữu riêng — chính xác từng bit, không cần tải
output-preset-high-label = Chất lượng cao
output-preset-high-title = MP4, bộ mã hóa tốt nhất phát hiện được, CQ 16 gần như không mất dữ liệu, cài đặt sẵn Chất lượng
output-preset-balanced-label = Cân bằng
output-preset-balanced-title = MKV, bộ mã hóa tốt nhất phát hiện được, CQ 23, cài đặt sẵn Cân bằng
output-recording-format = Định dạng ghi
output-ffmpeg-warning = Định dạng này cần thành phần FFmpeg (codec đường truyền — không đóng gói kèm). .frec không mất dữ liệu không cần gì.
output-install = Cài đặt…
output-recordings-folder = Thư mục bản ghi
output-folder-placeholder = Thư mục Video của HĐH
output-filename-prefix = Tiền tố tên tệp
output-recording-template = Tên tệp bản ghi
output-replay-template = Tên tệp phát lại
output-still-template = Tên tệp khung hình
output-template-tokens = Token: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Thư mục phát lại
output-still-folder = Thư mục khung hình
output-same-folder-placeholder = Thư mục bản ghi
output-frame-rate = Tốc độ khung hình
output-fps-option = { $fps } fps
output-split-every = Chia mỗi (phút, 0 = tắt)
output-output-width = Chiều rộng đầu ra (0 = khung hình; chỉ định dạng đường truyền)
output-output-height = Chiều cao đầu ra (0 = khung hình)
output-record-vertical = Cũng ghi khung hình dọc (một tệp “… (dọc)” song song; cần bật khung hình 9:16)
output-audio-tracks = Track âm thanh
output-recorded-tracks-group = Các track được ghi
output-track-last-one = Ít nhất một track phải ghi
output-record-track-on = Ghi track { $index }: bật
output-record-track-off = Ghi track { $index }: tắt
output-encoder-heading = Bộ mã hóa
output-video-encoder = Bộ mã hóa video
output-encoder-auto = Tự động — tốt nhất phát hiện được (H.264)
output-encoder-unavailable = — không khả dụng ở đây
output-preset = Cài đặt sẵn
output-preset-quality = Chất lượng
output-preset-balanced-option = Cân bằng
output-preset-performance = Hiệu năng
output-rate-control = Kiểm soát tốc độ
output-rc-cqp = CQP (chất lượng không đổi)
output-rc-cbr = CBR (bitrate không đổi)
output-rc-vbr = VBR (bitrate thay đổi)
output-cq = CQ (0–51, thấp hơn = tốt hơn)
output-bitrate = Bitrate (kbps)
output-keyframe = Khoảng keyframe (s)
output-audio-bitrate = Bitrate âm thanh (kbps / track)
output-presets = Cài đặt sẵn:

# --- SettingsStream.tsx ---
stream-title = Cài đặt — Luồng
stream-target-enabled = Đích { $index } đã bật
stream-target = Đích { $index }
stream-remove = Xóa
stream-service = Dịch vụ
stream-canvas = Khung hình
stream-canvas-main = Chính (chương trình)
stream-canvas-vertical = Dọc (9:16 — bật nó trong studio)
stream-ingest-srt = URL nạp SRT
stream-ingest-whip = URL điểm cuối WHIP
stream-ingest-url = URL nạp
stream-ingest-override = (ghi đè — trống = cài đặt sẵn của dịch vụ)
stream-key-srt = streamid (tùy chọn — thêm vào dưới dạng ?streamid=…; coi như bí mật)
stream-key-whip = Bearer token (tùy chọn — gửi dưới dạng header Authorization; một bí mật)
stream-key-custom = Khóa luồng (từ máy chủ của bạn — coi như bí mật)
stream-key-service = Khóa luồng (từ bảng điều khiển nhà sáng tạo của bạn — coi như bí mật)
stream-key-aria = Khóa luồng { $index }
stream-key-hide = Ẩn
stream-key-show = Hiện
stream-encoder = Bộ mã hóa (H.264 — thứ mà RTMP, SRT và WHIP đều mang)
stream-encoder-auto = Tự động — bộ mã hóa H.264 tốt nhất phát hiện được
stream-encoder-unavailable = (không khả dụng ở đây)
stream-video-bitrate = Bitrate video (kbps, CBR)
stream-audio-bitrate = Bitrate âm thanh (kbps)
stream-fps = FPS
stream-keyframe = Khoảng keyframe (s)
stream-audio-track = Track âm thanh (1–6)
stream-output-width = Chiều rộng đầu ra (0 = khung hình)
stream-output-height = Chiều cao đầu ra (0 = khung hình)
stream-add-target = + Thêm đích
stream-go-live-note = Phát trực tiếp xuất bản tới mọi đích đã bật cùng lúc, trực tiếp tới từng nền tảng. Các đích có cùng cài đặt bộ mã hóa chia sẻ một lần mã hóa.
stream-auto-record = Bắt đầu ghi khi tôi phát trực tiếp (bản ghi vẫn dừng độc lập)
stream-ffmpeg-note-before = Các codec đường truyền phát trực tiếp chạy qua thành phần ffmpeg theo yêu cầu được dán nhãn —
stream-ffmpeg-note-link = quản lý nó ở đây
stream-ffmpeg-note-after = . Bản ghi cục bộ tiếp tục chạy bất kể luồng làm gì.
stream-cancel = Hủy
stream-save = Lưu

# --- SettingsReplay.tsx ---
replay-title = Cài đặt — Bộ đệm Phát lại
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 phút
replay-length-2min = 2 phút
replay-length-5min = 5 phút
replay-quality-low = Thấp (3 Mbps)
replay-quality-standard = Tiêu chuẩn (6 Mbps)
replay-quality-high = Cao (12 Mbps)
replay-length-presets = Cài đặt sẵn độ dài
replay-quality-presets = Cài đặt sẵn chất lượng
replay-length-seconds = Độ dài (giây)
replay-video-bitrate = Bitrate video (kbps)
replay-fps = FPS
replay-audio-track = Track âm thanh (1–6)
replay-note = Khi đã kích hoạt, bộ đệm chạy mã hóa nhẹ riêng của nó vào một vòng đệm giới hạn trên đĩa — khoảng { $mb } MB ở các cài đặt này. Lưu sẽ ghép vòng đệm mà không mã hóa lại và không bao giờ chạm vào luồng hay bản ghi. Thay đổi áp dụng vào lần kích hoạt tiếp theo.
replay-cancel = Hủy
replay-save = Lưu

# --- SettingsRemote.tsx ---
remote-title = Cài đặt — Điều khiển Từ xa
remote-enable = Bật API điều khiển từ xa WebSocket
remote-password = Mật khẩu (bắt buộc — bộ điều khiển xác thực bằng nó)
remote-password-placeholder = một mật khẩu cho các bộ điều khiển của bạn
remote-password-hide = Ẩn
remote-password-show = Hiện
remote-port = Cổng
remote-allow-lan = Cho phép kết nối LAN (mặc định chỉ máy này)
remote-note = Tắt = cổng bị đóng. Bật = một WebSocket được bảo vệ bằng mật khẩu trên 127.0.0.1 (hoặc LAN của bạn khi bật) có thể chuyển cảnh, chạy chuyển cảnh, bắt đầu/dừng luồng và ghi, lưu phát lại, và đặt tắt tiếng/âm lượng — cùng các hành động như UI, không hơn. Nó không thể đọc tệp. Hãy coi mật khẩu như bất kỳ thông tin đăng nhập nào; ưu tiên chỉ-máy-này trừ khi bạn cụ thể điều khiển từ thiết bị khác.
remote-password-required = Cần mật khẩu để bật API từ xa.
remote-cancel = Hủy
remote-save = Lưu

# --- SettingsHotkeys.tsx ---
hotkeys-title = Cài đặt — Phím tắt
hotkeys-record = Bắt đầu / dừng ghi
hotkeys-go-live = Phát trực tiếp / Kết thúc Luồng
hotkeys-transition = Chuyển cảnh Chế độ Studio
hotkeys-save-replay = Lưu Phát lại (N giây cuối)
hotkeys-add-marker = Thả một marker chương (ghi)
hotkeys-note = Phím tắt là toàn cục — chúng kích hoạt khi ứng dụng khác đang được lấy nét. Để trống = không gán. Các phím nhấn-để-nói/tắt-tiếng của bộ trộn nằm trong menu ⋯ của từng dải. Trên Linux/Wayland, phím tắt toàn cục có thể không khả dụng (giới hạn của trình dựng cửa sổ) — các nút vẫn hoạt động.
hotkeys-cancel = Hủy
hotkeys-save = Lưu

# --- WorkspaceDialog.tsx ---
workspace-title = Hồ sơ & Bộ sưu tập Cảnh
workspace-profiles = Hồ sơ
workspace-profiles-hint = Một hồ sơ là các cài đặt của bạn — đích luồng, đầu ra, phím tắt. Chuyển đổi theo mỗi buổi diễn hoặc mỗi nền tảng.
workspace-collections = Bộ sưu tập cảnh
workspace-collections-hint = Một bộ sưu tập là các cảnh + nguồn của bạn. Tạo sẽ nhân đôi bộ hiện tại làm điểm bắt đầu.
workspace-active = Đang hoạt động
workspace-switch-to = Chuyển sang { $name }
workspace-active-marker = ● đang hoạt động
workspace-new-name-placeholder = tên mới…
workspace-new-name-label = Tên { $title } mới
workspace-create = Tạo

# --- OBS import (CAP-M02) ---
workspace-import-obs = Nhập từ OBS…
workspace-import-obs-hint = Mang vào một bộ sưu tập cảnh OBS (tệp scenes.json). Bộ sưu tập hiện tại của bạn sẽ được lưu trước.
workspace-import-busy = Đang nhập…
workspace-import-title = Đã nhập "{ $name }"
workspace-import-summary = { $scenes } cảnh · { $sources } nguồn · { $items } mục
workspace-import-dismiss = Đóng
workspace-import-clean = Mọi thứ đã được nhập trọn vẹn.
workspace-import-geometry-caveat = Kích thước và vị trí được khớp theo bố cục OBS — hãy xem lại từng cảnh và chọn lại thiết bị ghi.
workspace-import-notes-title = Đã nhập kèm ghi chú
workspace-import-skipped-title = Không được nhập
import-note-needsReselect = Chọn lại thiết bị/màn hình/cửa sổ
import-note-gameCaptureAsWindow = Ghi trò chơi → Ghi cửa sổ
import-note-referencesFile = Kiểm tra đường dẫn tệp
import-note-filterDropped = Một số bộ lọc không được hỗ trợ
import-note-geometryApproximated = Vị trí/kích thước gần đúng
import-skip-unsupportedKind = Không có loại nguồn tương đương
import-skip-group = Nhóm chưa được hỗ trợ

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Liên kết lại tệp bị thiếu…
doctor-title = Tệp bị thiếu
doctor-scanning = Đang quét…
doctor-all-good = Mọi tệp được tham chiếu đều tồn tại. Không có gì để liên kết lại.
doctor-intro = Không tìm thấy { $count } tệp được tham chiếu trên máy này. Hãy chỉ vị trí mới cho từng tệp — mọi cảnh dùng nó đều được sửa cùng lúc.
doctor-relinked = Đã liên kết lại { $count } tham chiếu.
doctor-uses = dùng { $count }×
doctor-locate = Định vị…
doctor-locate-folder = Tìm trong thư mục…
doctor-locate-folder-hint = Chọn một thư mục; mỗi tệp bị thiếu được khớp theo tên và liên kết lại.
doctor-kind-image = hình ảnh
doctor-kind-media = phương tiện
doctor-kind-slideshow = trình chiếu
doctor-kind-font = phông chữ
doctor-kind-lut = LUT
doctor-kind-mask = mặt nạ
history-relinkFiles = Liên kết lại tệp

# --- ScriptsDialog.tsx ---
scripts-title = Script (Lua)
scripts-empty = Chưa có script nào — thêm một tệp .lua. Xem scripts/sample.lua để biết API: phản ứng với sự kiện phát trực tiếp/cảnh/ghi và điều khiển cùng các lệnh như API từ xa.
scripts-enable = Bật { $path }
scripts-remove = Xóa { $path }
scripts-path-label = Đường dẫn script
scripts-add = Thêm
scripts-note = Script chạy trong hộp cát — không truy cập tệp hay HĐH; chúng chỉ có thể gọi cùng các lệnh studio như API từ xa (đổi cảnh, chạy chuyển cảnh, ghi/luồng/phát lại, tắt tiếng). Lỗi script được ghi log và giữ trong tầm kiểm soát. Thay đổi áp dụng trong vòng một giây.
scripts-error-not-lua = Trỏ tới một tệp .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Dock trình duyệt
browser-dock-empty = Chưa có dock nào — thêm một chat popout, một trang thông báo, hoặc các nút web Companion của bạn.
browser-dock-open = Mở
browser-dock-remove = Xóa { $name }
browser-dock-name-placeholder = tên (ví dụ Twitch Chat)
browser-dock-name-label = Tên dock
browser-dock-url-label = URL dock
browser-dock-note = Một dock mở như cửa sổ riêng mà bạn có thể đặt cạnh studio. Trang không có quyền truy cập ứng dụng — nó chỉ kết xuất. Chỉ URL http(s); dock chỉ mở khi bạn nhấp Mở.
browser-dock-error-name = Đặt tên cho dock (ví dụ Twitch Chat).
browser-dock-error-url = URL dock phải bắt đầu bằng http:// hoặc https://.

# --- studio-preview-pane ---
studio-preview-label = Xem trước Chế độ Studio
studio-preview-heading = Xem trước
studio-preview-hint = nhấp vào một cảnh để tải nó vào đây
studio-preview-empty = Bản xem trước sẽ xuất hiện ở đây.
studio-preview-mirrors = phản chiếu chương trình
studio-preview-transition-select = Chuyển cảnh
studio-preview-duration = Thời lượng chuyển cảnh (ms)
studio-preview-commit-title = Chuyển Xem trước → Chương trình qua hiệu ứng chuyển cảnh (khán giả sẽ thấy)
studio-preview-transitioning = Đang chuyển cảnh…
studio-preview-transition-button = Chuyển cảnh ⇄
studio-preview-luma-placeholder = ảnh wipe thang xám (png/jpg)
studio-preview-luma-label = Ảnh Luma wipe
studio-preview-browse = Duyệt…
studio-preview-filter-images = Hình ảnh
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = video stinger (ProRes 4444 .mov giữ được alpha)
studio-preview-stinger-label = Tệp video Stinger
studio-preview-stinger-cut-label = Điểm cắt Stinger (ms)
studio-preview-stinger-cut-title = Thời điểm chuyển cảnh diễn ra dưới lớp stinger (ms tính từ đầu hiệu ứng chuyển cảnh)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Cắt
transition-kind-fade = Mờ dần
transition-kind-slide-left = Trượt ←
transition-kind-slide-right = Trượt →
transition-kind-slide-up = Trượt ↑
transition-kind-slide-down = Trượt ↓
transition-kind-swipe-left = Vuốt ←
transition-kind-swipe-right = Vuốt →
transition-kind-luma-linear = Luma wipe (tuyến tính)
transition-kind-luma-radial = Luma wipe (xuyên tâm)
transition-kind-luma-horizontal = Luma wipe (ngang)
transition-kind-luma-diamond = Luma wipe (hình thoi)
transition-kind-luma-clock = Luma wipe (đồng hồ)
transition-kind-image = Wipe ảnh (tùy chỉnh)
transition-kind-stinger = Stinger (video)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Tùy chỉnh (RTMP/RTMPS)
stream-service-srt = SRT (tự lưu trữ)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Giới thiệu
about-tagline = Ghi và phát trực tiếp như một studio — không tài khoản, không đám mây.
about-version = Phiên bản
about-created-by = Được tạo bởi
about-project-started = Dự án bắt đầu
about-first-stable = Bản phát hành ổn định đầu tiên
about-first-stable-pending = Chưa — 1.0.0 đang được thực hiện
about-platform = Nền tảng
about-local-first = Freally Capture chạy hoàn toàn trên máy của bạn. Không tài khoản, không đo lường từ xa, không đám mây — thứ duy nhất rời khỏi máy tính của bạn là luồng mà bạn chọn gửi đi.
about-website = Trang web
about-issues = Báo cáo sự cố
about-license = Giấy phép
about-eula = EULA
about-third-party = Thông báo bên thứ ba
about-check-updates = Kiểm tra cập nhật…

# --- unified settings modal (TASK-906) ---
settings-title = Cài đặt
settings-language-section = Ngôn ngữ
settings-language = Ngôn ngữ giao diện
settings-language-system = Mặc định của hệ thống
settings-language-note = Ngôn ngữ bạn chọn ở đây sẽ được ghi nhớ. “Mặc định của hệ thống” theo hệ điều hành của bạn. Văn bản chưa dịch sẽ hiển thị bằng tiếng Anh.
settings-appearance-section = Giao diện
settings-theme = Chủ đề
settings-theme-dark = Tối
settings-theme-light = Sáng
settings-theme-custom = Tùy chỉnh
settings-accent = Màu nhấn
settings-general-section = Chung
settings-show-stats-dock = Hiện bảng thống kê
settings-open-about = Giới thiệu…

# --- command palette (TASK-904) ---
palette-title = Bảng lệnh
palette-search = Tìm cảnh, nguồn và hành động
palette-placeholder = Tìm cảnh, nguồn, hành động…
palette-no-results = Không có gì khớp với “{ $query }”
palette-hint = ↑ ↓ để di chuyển · Enter để chạy · Esc để đóng
palette-group-scenes = Cảnh
palette-group-sources = Nguồn
palette-group-actions = Hành động
palette-transition = Chuyển cảnh Xem trước → Chương trình
palette-save-replay = Lưu phát lại
palette-add-marker = Thả một marker chương
palette-vertical-canvas = Khung hình dọc (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Chào mừng bạn đến với Freally Capture
wizard-welcome = Hai bước nhanh gọn: xem máy của bạn làm được những gì, rồi bắt đầu một cảnh. Chỉ mất khoảng ba mươi giây, và bạn có thể thay đổi mọi thứ sau này.
wizard-local-first = Không có gì ở đây rời khỏi máy tính của bạn. Freally Capture không có tài khoản, không đo lường từ xa, và không đám mây.
wizard-start = Bắt đầu
wizard-skip = Bỏ qua
wizard-hardware-title = Máy của bạn làm được những gì
wizard-probing = Đang xem card đồ họa và bộ xử lý của bạn…
wizard-encoder = Bộ mã hóa
wizard-canvas = Khung hình
wizard-bitrate = Bitrate
wizard-probe-found = Tìm thấy: { $gpus } · { $cores } lõi vật lý
wizard-no-gpu = không có GPU riêng
wizard-apply = Dùng các cài đặt này
wizard-keep-current = Giữ những gì tôi đang có
wizard-template-title = Bắt đầu với một cảnh
wizard-template-screen = Chụp màn hình của tôi
wizard-template-screen-note = Thêm một Chụp Màn hình cho màn hình chính của bạn. Đây là nơi phổ biến nhất để bắt đầu.
wizard-template-empty = Bắt đầu với cảnh trống
wizard-template-empty-note = Một cảnh trống. Bạn tự thêm nguồn bằng nút +.
wizard-done = Bạn đã thiết lập xong.
wizard-done-hint = Nhấn Ctrl+K bất cứ lúc nào để tìm cảnh, nguồn và hành động. Các cài đặt nằm sau nút ⚙.
wizard-close = Bắt đầu phát trực tiếp

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Card đồ họa của bạn có thể tự mã hóa video, để bộ xử lý được rảnh cho phần còn lại của studio.
autoconfig-reason-software = Không tìm thấy bộ mã hóa phần cứng khả dụng, nên bộ xử lý sẽ đảm nhận việc mã hóa, cách này vẫn chạy tốt, chỉ là tốn nhiều CPU hơn.
autoconfig-reason-quality-hardware = 1080p ở 60 khung hình mỗi giây, tại một bitrate mà mọi nền tảng lớn đều chấp nhận.
autoconfig-reason-quality-software = 30 khung hình mỗi giây, vì mã hóa bằng phần mềm ở mức 60 sẽ rớt khung trên hầu hết bộ xử lý.
autoconfig-reason-quality-low-cores = Một bitrate thấp hơn, vì bộ xử lý này có ít lõi và mã hóa bằng phần mềm sẽ tranh giành chúng với bộ dựng hình.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Đã bắt đầu ghi hình
announce-recording-paused = Đã tạm dừng ghi hình
announce-recording-stopped = Đã dừng ghi hình
announce-live-started = Bạn đang phát trực tiếp
announce-live-ended = Đã kết thúc phát trực tiếp
announce-reconnecting = Mất kết nối, đang kết nối lại
announce-stream-failed = Phát trực tiếp thất bại
announce-frames-dropped = Đã rớt { $count } khung hình

# CAP-M01 — undo/redo edit history
palette-undo = Hoàn tác
palette-redo = Làm lại
palette-edit-history = Lịch sử chỉnh sửa…
history-title = Lịch sử chỉnh sửa
history-empty = Chưa có gì để hoàn tác.
history-current = Trạng thái hiện tại
history-close = Đóng
history-addScene = Thêm cảnh
history-renameScene = Đổi tên cảnh
history-removeScene = Xóa cảnh
history-reorderScene = Sắp xếp lại cảnh
history-addSource = Thêm nguồn
history-removeSource = Xóa nguồn
history-reorderSource = Sắp xếp lại nguồn
history-renameSource = Đổi tên nguồn
history-transformSource = Di chuyển nguồn
history-toggleVisibility = Bật/tắt hiển thị
history-toggleLock = Bật/tắt khóa
history-setBlendMode = Đổi chế độ hòa trộn
history-editSourceProperties = Chỉnh sửa thuộc tính
history-applyLayout = Sắp xếp bố cục
history-moveToSeat = Chuyển đến vị trí
history-groupSources = Nhóm nguồn
history-ungroupSources = Bỏ nhóm nguồn
history-toggleGroupVisibility = Bật/tắt nhóm
history-setSceneAudio = Âm thanh cảnh
history-setVerticalCanvas = Khung dọc
history-addFilter = Thêm bộ lọc
history-removeFilter = Xóa bộ lọc
history-reorderFilter = Sắp xếp lại bộ lọc
history-editFilter = Chỉnh sửa bộ lọc
history-toggleFilter = Bật/tắt bộ lọc
history-setVolume = Điều chỉnh âm lượng
history-toggleMute = Bật/tắt tắt tiếng
history-setMonitor = Đổi giám sát
history-setTracks = Đổi bản nhạc
history-setSyncOffset = Điều chỉnh đồng bộ A/V
history-setAudioHotkeys = Phím tắt âm thanh

# CAP-M04 — alignment aids
settings-alignment-section = Trợ giúp căn chỉnh
settings-smart-guides = Đường gióng thông minh (hít khi kéo)
settings-safe-areas = Lớp phủ vùng an toàn
settings-rulers = Thước
align-group = Căn theo khung
align-left = Căn trái
align-hcenter = Căn giữa theo chiều ngang
align-right = Căn phải
align-top = Căn trên
align-vcenter = Căn giữa theo chiều dọc
align-bottom = Căn dưới

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Căn chỉnh & phân bố mục đã chọn
arrange-left = Căn cạnh trái
arrange-hcenter = Căn giữa theo chiều ngang
arrange-right = Căn cạnh phải
arrange-top = Căn cạnh trên
arrange-vcenter = Căn giữa theo chiều dọc
arrange-bottom = Căn cạnh dưới
distribute-h = Phân bố theo chiều ngang
distribute-v = Phân bố theo chiều dọc
guides-group = Đường gióng
guides-add-v = Thêm đường gióng dọc
guides-add-h = Thêm đường gióng ngang
guides-clear = Xóa tất cả đường gióng
history-arrangeItems = Sắp xếp mục
history-editGuides = Sửa đường gióng

# CAP-M05 — edit transform + copy/paste
transform-title = Chỉnh sửa biến đổi — { $name }
transform-anchor = Điểm neo
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Xoay
transform-crop = Cắt xén
transform-crop-left = Trái
transform-crop-top = Trên
transform-crop-right = Phải
transform-crop-bottom = Dưới
transform-no-size = Kích thước và cắt xén sẽ khả dụng khi nguồn báo cáo kích thước của nó.
transform-copy = Sao chép biến đổi
transform-paste = Dán biến đổi
transform-close = Đóng
filters-copy = Sao chép bộ lọc ({ $count })
filters-paste = Dán bộ lọc ({ $count })
palette-edit-transform = Chỉnh sửa biến đổi…
history-pasteFilters = Dán bộ lọc

# CAP-M26 — keying workbench
workbench-title = Bàn tách nền — { $name }
workbench-mode-keyed = Đã tách
workbench-mode-source = Nguồn
workbench-mode-matte = Mặt nạ
workbench-mode-split = Chia đôi
workbench-eyedropper = Ống hút màu
workbench-eyedropper-hint = Nhấp vào nguồn để lấy màu khóa.
workbench-loupe = Kính lúp
workbench-split = Chia
workbench-preview-alt = Xem trước bàn tách nền
workbench-tune = Tinh chỉnh
workbench-close = Đóng

# CAP-M06 — multiview monitor
multiview-title = Đa khung
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Nhấp vào một cảnh để chuyển sang cảnh đó.
multiview-hint-stage = Nhấp vào một cảnh để dàn dựng trong bản xem trước.
palette-multiview = Màn hình đa khung

# CAP-M07 — projectors
projector-title = Mở máy chiếu
projector-source = Nguồn
projector-target-program = Chương trình
projector-target-preview = Xem trước
projector-target-scene = Cảnh…
projector-target-source = Nguồn…
projector-target-multiview = Multiview
projector-which-scene = Cảnh nào
projector-which-source = Nguồn nào
projector-none = Không có gì để hiển thị
projector-display = Màn hình
projector-windowed = Cửa sổ nổi (màn hình này)
projector-display-option = Màn hình { $n } — { $w }×{ $h }
projector-primary = (chính)
projector-open = Mở
projector-cancel = Hủy
projector-exit-hint = Nhấn Esc để thoát
palette-projector = Mở máy chiếu…

# CAP-M08 — still-frame grab
palette-still = Chụp khung hình…
still-saved-toast = Đã lưu khung hình: { $name }
still-failed-toast = Chụp khung hình thất bại: { $error }
hotkeys-still = Chụp khung hình

# CAP-M13 — source health dashboard
palette-source-health = Tình trạng nguồn…
palette-av-sync = Hiệu chuẩn đồng bộ A/V…
palette-hotkey-audit = Bản đồ phím tắt…
health-title = Tình trạng nguồn
health-col-source = Nguồn
health-col-state = Trạng thái
health-col-resolution = Độ phân giải
health-col-fps = FPS
health-col-last-frame = Khung hình cuối
health-col-dropped = Bị bỏ
health-col-retries = Số lần khởi động lại
health-col-actions = Hành động
health-state-live = Trực tiếp
health-state-waiting = Đang chờ
health-state-error = Lỗi
health-state-inactive = Không hoạt động
health-restart = Khởi động lại
health-properties = Thuộc tính
health-empty = Bộ sưu tập này chưa có nguồn nào.
health-seconds = { $value } giây

# CAP-M23 — quit guard + orderly shutdown
quit-title = Thoát Freally Capture?
quit-body = Thoát ngay bây giờ sẽ thực hiện an toàn theo thứ tự:
quit-consequence-stream = Kết thúc phát trực tiếp và ngắt kết nối khỏi dịch vụ.
quit-consequence-recording = Dừng ghi hình và hoàn tất các tệp.
quit-consequence-replay = Tắt bộ đệm phát lại — cảnh phát lại chưa lưu sẽ bị loại bỏ.
quit-confirm = Thoát an toàn
quit-quitting = Đang tắt…
quit-cancel = Hủy

# CAP-M11 — crash-safe recording salvage
salvage-title = Khôi phục các bản ghi bị gián đoạn?
salvage-body = Phiên trước đã kết thúc đột ngột khi các bản ghi này vẫn đang được ghi. Sửa chữa tạo một bản sao phát được bên cạnh bản gốc — tệp gốc không bao giờ bị thay đổi.
salvage-repair = Sửa chữa
salvage-repairing = Đang sửa…
salvage-done = Đã sửa
salvage-repaired = Đã sửa → { $name }
salvage-failed = Sửa chữa thất bại: { $error }
salvage-dismiss = Để sau

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Lỗi bộ mã hóa — đã chuyển từ { $from } sang { $to }. Luồng phát đã kết nối lại và tiếp tục.
fallback-toast-recording = Lỗi bộ mã hóa — đã chuyển từ { $from } sang { $to }. Bản ghi tiếp tục trong tệp mới.
fallback-note = Bộ mã hóa dự phòng: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Âm thanh chương trình đã im lặng
alarm-clipping = Âm thanh chương trình đang bị vỡ tiếng
alarm-black = Hình ảnh chương trình bị đen
alarm-frozen = Hình ảnh chương trình đã lâu không thay đổi
alarm-lowDisk = Dung lượng đĩa: còn khoảng { $minutes } phút với bitrate hiện tại
alarm-dismiss = Bỏ qua cảnh báo
alarm-cleared = Đã giải quyết: { $alarm }

# CAP-M22 — panic button
palette-panic = Khẩn cấp — chuyển sang màn chắn riêng tư
panic-banner-title = Khẩn cấp
panic-banner-body = Chương trình đang hiển thị màn chắn riêng tư; toàn bộ âm thanh bị tắt và các nguồn thu bị dừng. Luồng phát và bản ghi vẫn tiếp tục.
panic-restore = Khôi phục…
panic-restore-confirm = Khôi phục chương trình?
panic-restore-yes = Khôi phục
panic-restore-cancel = Hủy
hotkeys-panic = Khẩn cấp (màn chắn riêng tư)
hotkeys-timer-toggle = Bắt đầu/tạm dừng mọi hẹn giờ
hotkeys-timer-reset = Đặt lại mọi hẹn giờ
panic-slate-color = Màu màn chắn khẩn cấp
panic-slate-image = Ảnh màn chắn khẩn cấp
panic-slate-image-placeholder = Đường dẫn ảnh tùy chọn

# CAP-M24 — redacted diagnostics bundle
diag-title = Gói chẩn đoán
diag-intro = Xuất một tệp .zip đã ẩn thông tin nhạy cảm (ảnh chụp cấu hình, thăm dò bộ mã hóa, thống kê gần đây — bí mật, đường dẫn và tên không bao giờ được đưa vào) để đính kèm thủ công vào issue GitHub. Không gửi gì đi đâu cả.
diag-preview = Xem nội dung
diag-hide-preview = Ẩn xem trước
diag-export = Xuất .zip
diag-exported = Đã xuất: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Kiểm tra trước khi phát
preflight-intro = Mọi mục chặn phải xanh; phần còn lại là nhắc nhở trung thực.
preflight-item-targets = Đã cấu hình đích phát (khóa/URL)
preflight-item-encoder = Có bộ mã hóa dùng được
preflight-item-sources = Mọi nguồn khỏe mạnh
preflight-item-disk = Dung lượng đĩa cho bản ghi
preflight-item-mic = Đo mức micro
preflight-item-desktopAudio = Đo mức âm thanh máy tính
preflight-item-replay = Bộ đệm phát lại sẵn sàng
preflight-targets-detail = { $count } đang bật
preflight-sources-detail = { $count } nguồn bị lỗi
preflight-disk-detail = ~{ $minutes } phút với bitrate hiện tại
preflight-fix-stream = Cài đặt phát…
preflight-fix-components = Thành phần…
preflight-fix-sources = Tình trạng nguồn…
preflight-fix-replay = Kích hoạt
preflight-optional = tùy chọn
preflight-hold = Giữ Go Live đến khi tất cả xanh
preflight-cancel = Hủy
preflight-go-anyway = Vẫn phát trực tiếp
preflight-go-live = Phát trực tiếp


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Nền
scenes-backdrop-aria = Nền của { $name }
backdrop-title = Nền — { $name }
backdrop-hint = Hình nền được ghim phía sau mọi thứ trong cảnh này — một ảnh, GIF động hoặc video lặp. Phần thu hình của bạn luôn ở trên; cuộn trên khung vẽ để thu phóng.
backdrop-choose = Chọn ảnh hoặc video…
backdrop-remove = Gỡ nền
backdrop-none = Chưa đặt nền.
backdrop-position = Vị trí
backdrop-split-full = Toàn khung vẽ
backdrop-split-left = Nửa trái
backdrop-split-right = Nửa phải
backdrop-split-top = Nửa trên
backdrop-split-bottom = Nửa dưới
backdrop-sync = Bắt đầu phát khi bắt đầu ghi hình
backdrop-sync-hint = Dừng ở khung hình đầu cho đến khi bạn ghi; mỗi lần quay video phát lại từ đầu.
backdrop-preview-play = Phát thử
backdrop-preview-pause = Tạm dừng phát thử
backdrop-filter-all = Nền (ảnh và video)
backdrop-filter-images = Ảnh
backdrop-filter-media = Video và GIF
sources-backdrop-badge = Hình nền (ghim dưới cùng)
sources-backdrop-pinned = Nền luôn được ghim ở dưới cùng
filters-name-flip = Lật
filters-flip-horizontal = Ngang
filters-flip-vertical = Dọc
history-setSceneBackdrop = Đặt nền
history-setBackdropSplit = Di chuyển nền
history-setBackdropSync = Đồng bộ nền với ghi hình
backdrop-scrub = Vị trí phát
backdrop-loop = Lặp
backdrop-reverse = Phát ngược
backdrop-reverse-hint = Phát ngược chỉ dựng một bản sao đảo ngược một lần (video cần thành phần ffmpeg; GIF đảo ngay lập tức) — lần chuyển đầu có thể mất thời gian với tệp dài.
filters-scaling = Tỷ lệ
filters-scaling-hint = Chế độ chuẩn từng pixel cho nội dung retro/pixel; Số nguyên còn khớp kích thước vẽ vào bội số nguyên (tay cầm hiển thị kích thước lôgic).
filters-scaling-auto = Mượt
filters-scaling-nearest = Lân cận gần nhất
filters-scaling-integer = Số nguyên (bội nguyên)
filters-scaling-sharp = Song tuyến sắc nét
history-setScaling = Đổi tỷ lệ
hotkeys-zoom-100 = Thu phóng: đặt lại (100%)
hotkeys-zoom-150 = Thu phóng: phóng tới 150%
hotkeys-zoom-200 = Thu phóng: phóng 2×
sources-follow-title = Bám theo con trỏ khi phóng to (Windows; cuộn trên khung vẽ để thu phóng)
sources-follow-item = Bật/tắt bám con trỏ cho { $name }
filters-autocrop = ✂ Tự cắt dải đen
filters-autocrop-title = Quét khung hình kế tiếp để tìm dải đen trên/dưới hoặc hai bên rồi cắt (hoàn tác được). Cảnh tối không bao giờ bị cắt.
filters-autocrop-follow = Kiểm tra lại khi đổi độ phân giải
history-autoCrop = Tự cắt dải đen
sources-link-audio = Thu luôn âm thanh của ứng dụng này (liên kết: ẩn sẽ tắt tiếng, gỡ cửa sổ sẽ gỡ theo)
history-addLinkedWindow = Thêm cửa sổ + âm thanh liên kết
sources-hdr-title = Màn hình này là HDR — mở tone map (khung vẽ vẫn SDR)
sources-hdr-item = Tone map HDR cho { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Màn hình này xuất HDR. Không có tone map, vùng sáng bị cắt và hình thu trông bạc màu trên khung vẽ SDR. Thay đổi áp dụng từ khung hình kế tiếp.
sources-hdr-enable-suggested = Bật đề xuất (maxRGB, 200 nit)
sources-hdr-operator = Toán tử
sources-hdr-op-clip = Cắt (tắt)
sources-hdr-op-maxrgb = maxRGB (giữ sắc màu)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = Gối BT.2408 (SDR chính xác)
sources-hdr-paper-white = Trắng giấy
sources-hdr-nits = nit
projector-target-passthrough = Màn hình xuyên suốt (độ trễ thấp)
projector-which-device = Thiết bị
projector-passthrough-none = Hãy thêm màn hình, cửa sổ hoặc thiết bị thu trước.
projector-passthrough-about = Khung hình thô từ thiết bị — không cảnh, không bộ lọc, không compositor. Hiển thị độ trễ đo được; âm thanh vẫn nghe qua kênh mixer.
projector-passthrough-hint = Xuyên suốt — Esc để đóng
projector-latency = { $ms } ms
projector-latency-measuring = đang đo…
automation-title = Tự động hoá — quy tắc, macro & biến
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Quy tắc
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Bật
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Khi
automation-then-run = thì chạy
automation-no-macro = (no macro)
automation-macros = Macro
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Chạy
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Biến studio
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
rundown-title = Kịch bản chương trình
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Bắt đầu
rundown-next = Tiếp ▸
rundown-stop = Dừng
rundown-idle = Chưa chạy
rundown-next-up = Tiếp theo: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Bước
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
automation-layer = Lớp
automation-layer-hint = Chỉ kích hoạt khi lớp này đang bật (để trống = mọi lớp). Lớp có tính dính: phím lớp chuyển và giữ nguyên (API phím tắt toàn cục của HĐH không hỗ trợ lớp giữ phím).
automation-chord-hint = Phím thường (Ctrl+Shift+M) hoặc hợp phím hai nhịp (Ctrl+K, 3). Phím thứ hai chỉ bị chiếm khi hợp phím đang chờ.
panel-title = Bảng LAN & tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Phục vụ bảng
panel-port = Cổng
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Mật khẩu
panel-show = Hiện
panel-hide = Ẩn
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Lưu
osc-title = Bề mặt điều khiển OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Nghe OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = Máy quay PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Máy quay
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Địa chỉ
ptz-port = Cổng
ptz-speed = Tốc độ
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
ptz-presets = Cài sẵn
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Gọi lại
ptz-store = Lưu
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = Bề mặt điều khiển MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Đầu vào
midi-output = Đầu ra (phản hồi)
midi-none = (none)
midi-learn = Học
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Hành động
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
panel-lan-warning = ⚠ Lưu lượng LAN không được mã hoá — mật khẩu nằm trong URL qua HTTP. Chỉ dùng trên mạng đáng tin.
osc-lan-warning = ⚠ OSC không có mật khẩu — mọi thiết bị trong mạng đều có thể gửi các lệnh này. Chỉ dùng chế độ LAN trên mạng đáng tin.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Thống kê
sources-add-system-stats = Thống kê hiệu năng (HUD)
sources-stats-title = Thêm HUD hiệu năng
sources-stats-note = Hiển thị cho người xem các con số đo thực của studio ngay trong chương trình — fps, CPU, bộ nhớ, thời gian render, khung hình bị rơi và bitrate trực tiếp. Chọn dòng hiển thị, cỡ chữ và màu trong Thuộc tính của nguồn. Mức dùng GPU không hiển thị vì không được đo.
sources-stats-add = Thêm HUD thống kê
properties-stats-show-fps = Hiện FPS
properties-stats-show-cpu = Hiện CPU
properties-stats-show-memory = Hiện bộ nhớ
properties-stats-show-render = Hiện thời gian render
properties-stats-show-dropped = Hiện khung hình rơi
properties-stats-show-bitrate = Hiện bitrate
properties-stats-size = Cỡ (px)
properties-stats-note = HUD vẽ các nhãn gọn, phổ quát (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) thẳng vào chương trình; khi không phát trực tiếp, dòng bitrate hiển thị “—”.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Trực quan hoá
sources-add-visualizer = Trực quan hoá âm thanh
sources-visualizer-title = Thêm bộ trực quan hoá âm thanh
sources-visualizer-style-label = Kiểu
sources-visualizer-style-bars = Cột phổ
sources-visualizer-style-scope = Máy hiện sóng
sources-visualizer-style-vu = Đồng hồ VU
sources-visualizer-target-label = Nghe từ
sources-visualizer-target-master = Mix tổng
sources-visualizer-target-track = Track { $n }
sources-visualizer-note = Vẽ tín hiệu thực sự được trộn (sau fader) — nguồn bị tắt tiếng hiển thị phẳng, đúng như nó nghe được. Kích thước, màu, số cột và tốc độ rơi nằm trong Thuộc tính của nguồn.
sources-visualizer-add = Thêm bộ trực quan hoá
properties-vis-bands = Số cột
properties-vis-decay = Tốc độ rơi (dB/s)
properties-vis-peak-hold = Vạch giữ đỉnh
properties-vis-missing-source = (thiếu nguồn)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Split
sources-add-split-timer = Đồng hồ split speedrun
sources-splits-title = Thêm đồng hồ split
sources-splits-file-label = Tệp .lss của LiveSplit
sources-splits-comparison-label = So sánh với
sources-splits-comparison-pb = Kỷ lục cá nhân
sources-splits-comparison-best = Các chặng tốt nhất
sources-splits-comparison-average = Trung bình
sources-splits-note = Tệp được nhập ở chế độ chỉ đọc — không bao giờ ghi ngược lại. Gán các phím toàn cục Split / Undo / Skip / Reset trong Cài đặt → Phím tắt. Auto-splitter qua bộ nhớ tiến trình cố ý không được hỗ trợ.
sources-splits-add = Thêm đồng hồ split
properties-splits-size = Cỡ (px)
properties-splits-ahead = Dẫn trước
properties-splits-behind = Bị chậm
properties-splits-gold = Vàng
properties-splits-split = Split
properties-splits-undo = Hoàn tác
properties-splits-skip = Bỏ qua
properties-splits-reset = Đặt lại
properties-splits-note = Các nút điều khiển đồng hồ đang chạy (phím tắt toàn cục làm điều tương tự từ bất kỳ ứng dụng nào). Lần chạy không bao giờ được ghi vào tệp .lss.
hotkeys-split-split = Đồng hồ split: bắt đầu / split
hotkeys-split-undo = Đồng hồ split: hoàn tác split
hotkeys-split-skip = Đồng hồ split: bỏ qua chặng
hotkeys-split-reset = Đồng hồ split: đặt lại
hotkey-audit-action-split-split = Split (đồng hồ split)
hotkey-audit-action-split-undo = Hoàn tác split
hotkey-audit-action-split-skip = Bỏ qua chặng
hotkey-audit-action-split-reset = Đặt lại đồng hồ split
hotkey-audit-feature-split-timer = Đồng hồ split

# Media playlist source (CAP-N17)
sources-badge-playlist = Danh sách phát
sources-add-playlist = Danh sách phát media (liền mạch)
sources-playlist-title = Thêm danh sách phát media
sources-playlist-files-label = Tệp (mỗi dòng một tệp, phát từ trên xuống)
sources-playlist-browse = Duyệt…
sources-playlist-loop = Lặp
sources-playlist-shuffle = Xáo trộn (rút một lần mỗi lần bắt đầu; khi lặp giữ nguyên thứ tự đó)
sources-playlist-hold-last = Giữ khung hình cuối khi kết thúc
sources-playlist-note = Phát toàn bộ danh sách đã cắt liền mạch qua thành phần ffmpeg được ghi rõ (chỉ định dạng wire — .frec và ảnh dùng Media/Trình chiếu). Các mục hoặc toàn video hoặc toàn âm thanh, không trộn lẫn. Cắt, điểm cue và biến «now playing» nằm trong Thuộc tính.
sources-playlist-add = Thêm danh sách phát
properties-playlist-items = Mục (từ trên xuống)
properties-playlist-up = Lên
properties-playlist-down = Xuống
properties-playlist-remove = Xoá mục
properties-playlist-in = Từ (giây)
properties-playlist-out = Đến (giây)
properties-playlist-cues = Cue (giây, phân tách bằng dấu phẩy)
properties-playlist-add-item = + Thêm mục
properties-playlist-loop = Lặp
properties-playlist-shuffle = Xáo trộn
properties-playlist-hold-last = Giữ khung hình cuối
properties-playlist-hw = Giải mã phần cứng
properties-playlist-variable = Biến «now playing» (trống = tắt)
properties-playlist-previous = ⏮ Trước
properties-playlist-next = ⏭ Sau
properties-playlist-note = Nút cue và Sau/Trước điều khiển danh sách ĐANG phát; thay đổi mục có hiệu lực khi Áp dụng (danh sách khởi động lại). Đặt {"{{"}yourVariable{"}}"} vào nguồn Văn bản để hiển thị mục đang phát.
hotkeys-playlist-next = Danh sách phát: mục sau
hotkeys-playlist-previous = Danh sách phát: mục trước
hotkey-audit-action-playlist-next = Danh sách phát: sau
hotkey-audit-action-playlist-previous = Danh sách phát: trước
hotkey-audit-feature-playlist = Danh sách phát

# Instant replay source (CAP-N10)
sources-badge-replay = Phát lại
sources-add-replay = Phát lại tức thì
sources-replay-title = Thêm phát lại tức thì
sources-replay-seconds-label = Độ dài roll (giây)
sources-replay-speed-label = Tốc độ
sources-replay-speed-full = 100% (có âm thanh)
sources-replay-speed-half = Chậm 50% (im lặng)
sources-replay-speed-quarter = Chậm 25% (im lặng)
sources-replay-note = Trong suốt cho đến khi bạn roll. Kích hoạt bộ đệm phát lại (Điều khiển) và gán phím Roll — roll cắt những khoảnh khắc cuối của bộ đệm, phát vào chương trình rồi trở lại trong suốt.
sources-replay-add = Thêm phát lại tức thì
properties-replay-roll = ⏵ Roll phát lại
properties-replay-note = Roll cắt bộ đệm ĐANG KÍCH HOẠT thành clip và phát ở tốc độ đã chọn — chỉ đổi nhịp, không bao giờ nội suy. Chậm là im lặng có chủ đích. Tua và tạm dừng hoạt động khi đang phát; kết thúc thì nguồn trở lại trong suốt.
hotkeys-replay-roll = Phát lại tức thì: roll
hotkey-audit-action-replay-roll = Roll phát lại tức thì

# Input overlay source (CAP-N13)
sources-badge-input = Đầu vào
sources-add-input-overlay = Lớp phủ đầu vào (phím/tay cầm)
sources-input-title = Thêm lớp phủ đầu vào
sources-input-layout-label = Bố cục
sources-input-layout-wasd = WASD + chuột
sources-input-layout-keyboard = Bàn phím gọn + chuột
sources-input-layout-gamepad = Tay cầm (hai cần)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Phím
sources-input-accent-label = Đang nhấn
sources-input-privacy-note = Quyền riêng tư: đầu vào chỉ được đọc khi nguồn này đang phát trực tiếp trong một cảnh, và chỉ các phím cố định của bố cục được thăm dò — một lần xem tức thời "có đang nhấn không?", không bao giờ là hook. Không có gì được ghi lại, lưu trữ hay gửi đi đâu; văn bản gõ vào không bao giờ bị thu.
sources-input-os-note = Trạng thái bàn phím và chuột hiện chỉ đọc được trên Windows — các hệ khác vẽ phím ở trạng thái chưa nhấn (nói thẳng, không giả lập). Tay cầm hoạt động ở mọi nơi qua thư viện gilrs; tay cầm kết nối đầu tiên sẽ được vẽ, và nếu không có thì bố cục giữ nguyên chưa nhấn.
sources-input-add = Thêm lớp phủ đầu vào

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Hiệu ứng con trỏ
filters-cursorfx-hint = Trên Windows (nơi ứng dụng tự vẽ con trỏ), hiệu ứng được vẽ thẳng vào khung hình thu, nên xuất hiện trong bản ghi và luồng phát. macOS và Linux ghép con trỏ ở phía hệ điều hành, nên các hiệu ứng này chỉ có trên Windows. Thay đổi áp dụng ngay lập tức.
filters-cursorfx-halo = Quầng sáng con trỏ
filters-cursorfx-halo-color = Màu
filters-cursorfx-halo-radius = Bán kính (px)
filters-cursorfx-ripples = Gợn sóng khi bấm
filters-cursorfx-left-color = Chuột trái
filters-cursorfx-right-color = Chuột phải
filters-cursorfx-keystrokes = Hiện phím bấm
filters-cursorfx-keystrokes-hint = Hiển thị một bộ phím cố định (chữ cái, chữ số, phím bổ trợ, mũi tên) cạnh con trỏ khi đang giữ. Phím chỉ được đọc khi bật tính năng này, được vẽ thẳng vào khung hình và không bao giờ được lưu hay ghi log.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Tiêu đề
sources-add-title = Tiêu đề / Bảng điểm
sources-title-title = Thêm tiêu đề
sources-title-template-label = Bắt đầu từ
sources-title-template-lower-third = Thanh dưới (thanh + tên + chức danh)
sources-title-template-scoreboard = Bảng điểm (tấm nền + 4 ô)
sources-title-template-blank = Khung trống
sources-title-width-label = Chiều rộng khung
sources-title-height-label = Chiều cao khung
sources-title-template-name = Tên
sources-title-template-subtitle = Chức danh
sources-title-template-home = CHỦ NHÀ
sources-title-template-away = KHÁCH
sources-title-note = Tiêu đề nhiều lớp (chữ / ảnh / hộp) với hoạt ảnh vào/ra, ghép cục bộ — không phải nguồn trình duyệt. Các lớp, liên kết tệp và {"{{"}biến{"}}"} cùng điều khiển trực tiếp nằm trong Thuộc tính của nguồn.
sources-title-add = Thêm tiêu đề
properties-title-layers = Lớp (vẽ theo thứ tự — hàng sau nằm trên)
properties-title-kind-text = Chữ
properties-title-kind-image = Ảnh
properties-title-kind-rect = Hộp
properties-title-x = X
properties-title-y = Y
properties-title-outline = Viền (px)
properties-title-outline-color = Viền
properties-title-shadow = Bóng
properties-title-animation = Hoạt ảnh vào/ra
properties-title-anim-none = Không (cắt)
properties-title-anim-fade = Mờ dần
properties-title-anim-slide-left = Trượt sang trái
properties-title-anim-slide-up = Trượt lên trên
properties-title-anim-wipe = Quét
properties-title-duration = Thời lượng (ms)
properties-title-fire-in = ▶ Chạy vào
properties-title-fire-out = ◼ Chạy ra
properties-title-set-live = Đưa lên trực tiếp
properties-title-set-live-note = Đẩy chữ này vào tiêu đề ĐANG PHÁT ngay — không cần Áp dụng, không khởi động lại
properties-title-up = Đưa lớp lên
properties-title-down = Đưa lớp xuống
properties-title-remove = Xóa lớp
properties-title-add-text = + Chữ
properties-title-add-image = + Ảnh
properties-title-add-rect = + Hộp
properties-title-note = Chạy vào/ra và "Đưa lên trực tiếp" điều khiển tiêu đề ĐANG CHẠY; sửa lớp có hiệu lực khi Áp dụng (tiêu đề khởi động lại và vào lại). Ô chữ có thể gắn với tệp được theo dõi (ô CSV / giá trị JSON / cả tệp) và thay {"{{"}biến{"}}"} — "Đưa lên trực tiếp" thắng cả hai.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = Tiếp nhận LAN (trình lắng nghe SRT/RTMP)
sources-lan-title = Thêm trình lắng nghe tiếp nhận LAN
sources-lan-protocol-label = Giao thức
sources-lan-protocol-srt = SRT (mã hóa được — khuyến nghị)
sources-lan-protocol-rtmp = RTMP (không xác thực)
sources-lan-port-label = Cổng (1024–65535)
sources-lan-passphrase-label = Cụm mật khẩu (trống = mở)
sources-lan-passphrase-hint = Cụm mật khẩu SRT dài 10–79 ký tự; bên gửi phải dùng đúng cụm đó.
sources-lan-open-warning = Không có cụm mật khẩu: bất kỳ ai trong mạng này đều có thể đẩy vào nguồn này, không mã hóa. Hãy đặt một cụm trừ khi mạng chỉ của riêng bạn.
sources-lan-rtmp-warning = RTMP không có xác thực — bất kỳ ai trong mạng này đều có thể gửi tới cổng này. Nên dùng SRT kèm cụm mật khẩu.
sources-lan-url-label = Trỏ ứng dụng bên gửi tới
sources-lan-qr-aria = Mã QR của URL tiếp nhận
sources-lan-note = Chỉ LAN: lắng nghe trên địa chỉ cục bộ của máy này, chỉ khi nguồn còn tồn tại, và không bao giờ chạm tới internet — không gì rời khỏi máy cho đến khi một bên gửi trong mạng của bạn gửi trước. Việc giải mã đi qua thành phần ffmpeg được ghi nhãn rõ ràng. Khung hình hiển thị URL này cho tới khi bên gửi kết nối.
sources-lan-add = Bắt đầu lắng nghe
properties-lan-note = Áp dụng thay đổi giao thức, cổng hay cụm mật khẩu sẽ khởi động lại trình lắng nghe — bên gửi phải kết nối lại. Luồng được vừa khít vào khung 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Liên kết
sources-add-freally-link = Freally Link (phiên bản khác)
sources-link-title = Thêm Freally Link
sources-link-about = Nhận chương trình của một Freally Capture khác — video và âm thanh master — qua chính mạng của bạn. Hãy bật "Đầu ra Freally Link" trên máy gửi trước. v1 truyền motion-JPEG qua TCP: tuyệt vời trên LAN có dây hoặc Wi-Fi tốt, trung thực về băng thông trên đường truyền yếu.
sources-link-scan = Quét mạng LAN
sources-link-scanning = Đang quét…
sources-link-none = Không tìm thấy đầu ra Freally Link nào. Hãy bật "Đầu ra Freally Link" trên máy kia (Điều khiển → Bảng LAN) hoặc nhập địa chỉ bên dưới.
sources-link-host = Địa chỉ
sources-link-port = Cổng
sources-link-key = Khóa ghép nối
sources-link-key-hint = Khóa trong cài đặt "Đầu ra Freally Link" của bên gửi — không có nó, bên gửi không truyền một khung hình nào.
sources-link-add = Thêm liên kết
properties-link-note = Khi chưa kết nối, nguồn hiển thị màn hình "đang kết nối" và tự thử lại với thời gian chờ tăng dần — không bao giờ đứng hình ở khung cũ. Mỗi máy gửi chỉ một máy nhận; máy gửi đang bận sẽ được thử lại một cách lịch sự.
link-title = Đầu ra Freally Link
link-about = Chia sẻ chương trình của phiên bản này — video và âm thanh master — với MỘT Freally Capture khác trong chính mạng của bạn; ở đó nó xuất hiện như nguồn "Freally Link" (stream hai máy, màn hình phụ). Tắt theo mặc định; không có gì phát tín hiệu hay lắng nghe cho đến khi bạn bật. v1 truyền motion-JPEG + âm thanh không nén qua TCP — dành cho LAN có dây hoặc Wi-Fi tốt, không bao giờ cho internet.
link-enable = Chia sẻ chương trình trên mạng của tôi
link-name = Tên phiên bản
link-key = Khóa ghép nối
link-key-hint = Ít nhất 8 ký tự — bên nhận phải nhập khóa này trước khi bất kỳ khung hình nào được truyền.
link-lan-warning = ⚠ Bên nhận phải xuất trình khóa ghép nối trước khi nhận bất cứ thứ gì, nhưng bản thân luồng không được mã hóa ở v1 — chỉ dùng trên mạng bạn tin cậy.
link-serving = Máy nhận có thể tìm phiên bản này bằng "Quét mạng LAN" hoặc thêm thủ công tại:
link-off-hint = Bật chia sẻ để mở cổng và thông báo phiên bản này cho các lần quét LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Menu ứng dụng
menu-file = Tệp
menu-edit = Chỉnh sửa
menu-view = Xem
menu-docks = Docks
menu-profile = Hồ sơ
menu-collection = Bộ sưu tập cảnh
menu-tools = Công cụ
menu-help = Trợ giúp
menu-rename = Đổi tên
menu-remove = Xoá
menu-import = Nhập
menu-export = Xuất
menu-file-show-recordings = Hiện bản ghi
menu-file-remux = Remux sang MP4…
menu-file-settings = Cài đặt…
menu-file-show-settings-folder = Hiện thư mục cài đặt
menu-file-exit = Thoát
menu-edit-undo = Hoàn tác
menu-edit-redo = Làm lại
menu-edit-history = Lịch sử chỉnh sửa…
menu-edit-copy-transform = Sao chép biến đổi
menu-edit-paste-transform = Dán biến đổi
menu-edit-copy-filters = Sao chép bộ lọc
menu-edit-paste-filters = Dán bộ lọc
menu-edit-transform = Biến đổi…
menu-edit-lock-preview = Khoá xem trước
menu-view-fullscreen = Giao diện toàn màn hình
menu-stats-dock = Bảng thống kê
menu-view-multiview = Màn hình đa khung…
menu-view-projectors = Máy chiếu…
menu-view-source-health = Tình trạng nguồn…
menu-view-still = Chụp khung hình
menu-docks-browser = Docks trình duyệt…
menu-docks-lock = Khoá docks
menu-docks-reset = Đặt lại bố cục docks
menu-profile-manage = Quản lý hồ sơ…
menu-collection-manage = Quản lý bộ sưu tập cảnh…
menu-collection-import-obs = Nhập từ OBS…
menu-collection-missing = Kiểm tra tệp bị thiếu…
menu-tools-wizard = Chạy trình hướng dẫn thiết lập
menu-tools-wizard-title = Trình hướng dẫn thiết lập chạy ở lần khởi động đầu tiên; hiện chưa có cách chạy lại.
menu-tools-automation = Quy tắc tự động hoá & macro…
menu-tools-rundown = Hiện kịch bản chương trình…
menu-tools-hotkeys = Bản đồ phím tắt…
menu-tools-av-sync = Hiệu chuẩn đồng bộ A/V…
menu-tools-scripts = Script Lua…
menu-tools-components = Thành phần…
menu-tools-midi = Điều khiển MIDI…
menu-tools-ptz = Máy quay PTZ…
menu-tools-remote = API điều khiển từ xa…
menu-tools-panel = Bảng LAN & tally…
menu-help-portal = Cổng trợ giúp
menu-help-website = Truy cập trang web
menu-help-discord = Tham gia máy chủ Discord
menu-help-bug = Báo lỗi…
menu-help-updates = Kiểm tra cập nhật…
menu-help-whats-new = Có gì mới
menu-help-about = Giới thiệu…

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Danh mục cài đặt
settings-cat-general = Chung
settings-cat-appearance = Giao diện
settings-cat-streaming = Luồng
settings-cat-output = Đầu ra
settings-cat-replay = Phát lại
settings-cat-hotkeys = Phím tắt
settings-cat-network = Mạng
settings-cat-accessibility = Trợ năng
settings-cat-about = Giới thiệu
settings-ok = OK
settings-cancel = Hủy
settings-apply = Áp dụng
settings-save = Lưu
settings-loading = Đang tải cài đặt…
settings-hotkeys-filter = Lọc phím tắt
settings-hotkeys-filter-placeholder = Nhập để lọc hành động hoặc phím…
settings-hotkeys-no-match = Không có phím tắt nào khớp với “{ $query }”.
settings-hotkey-none = Không có
settings-hotkey-group-ctrl = Ctrl + phím
settings-hotkey-group-ctrl-shift = Ctrl + Shift + phím
settings-hotkey-group-ctrl-alt = Ctrl + Alt + phím
settings-hotkey-group-function = Phím chức năng
settings-hotkey-group-numpad = Bàn phím số
settings-panic-section = Màn chắn khẩn cấp
settings-meter-section = Đồng hồ mức của bộ trộn
settings-meter-note = Các màu mà đồng hồ mức của Bộ trộn Âm thanh quét qua, từ yên lặng đến vỡ tiếng. Cài đặt sẵn an toàn cho người mù màu dùng dải màu xanh dương → cam vẫn dễ đọc với chứng mù màu đỏ-lục.
settings-meter-preset = Màu đồng hồ mức
settings-meter-preset-default = Xanh lá / vàng / đỏ
settings-meter-preset-colorblind = An toàn cho người mù màu (xanh dương / cam)
settings-meter-preset-custom = Tùy chỉnh
settings-meter-low = Bình thường
settings-meter-mid = To
settings-meter-high = Vỡ tiếng
settings-meter-preview = Xem trước
