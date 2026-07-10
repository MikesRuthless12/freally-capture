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
sources-add-nested-scene = Cảnh Lồng nhau
sources-add-slideshow = Trình chiếu Ảnh
sources-add-chat-overlay = Lớp phủ Chat Trực tiếp
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
controls-files-title = Các bản ghi đã hoàn tất + hành động remux sang mp4
controls-files = ▤ Tệp…
controls-output-title = Định dạng ghi, bộ mã hóa, thư mục, track và chia nhỏ
controls-output = ⚙ Đầu ra…
controls-stream-title = Đích Phát trực tiếp: dịch vụ, khóa luồng, bộ mã hóa, bitrate
controls-stream = ⦿ Luồng…
controls-codecs-title = Thành phần codec đường truyền ffmpeg theo yêu cầu (dán nhãn rõ ràng, không bao giờ đóng gói kèm)
controls-codecs = ⬡ Codec…
controls-replay-title = Độ dài bộ đệm phát lại + cài đặt sẵn chất lượng
controls-replay = ⟲ Phát lại…
controls-keys-title = Phím tắt toàn cục: ghi, Phát trực tiếp, chuyển cảnh, lưu phát lại
controls-keys = ⌨ Phím…
controls-scripts-title = Script Lua trong hộp cát: phản ứng với sự kiện phát trực tiếp/cảnh/ghi, điều khiển studio
controls-scripts = ⚡ Script…
controls-docks-title = Dock trình duyệt: mở chat popout, trang thông báo, hoặc nút Companion như cửa sổ bên cạnh studio
controls-docks = ⧉ Docks…
controls-remote-title = API điều khiển từ xa WebSocket cho bộ điều khiển Stream Deck / Companion (mặc định tắt)
controls-remote = ⌁ Từ xa…
controls-profiles-title = Hồ sơ (cài đặt) + bộ sưu tập cảnh — ảnh chụp có thể chuyển đổi
controls-profiles = ▣ Hồ sơ…
controls-bug-title = Báo lỗi — ẩn danh, tự nguyện (không gì được gửi tự động)
controls-bug = 🐞 Báo lỗi…
controls-updates-title = Kiểm tra cập nhật — đã ký, đã xác minh, không gì tải về nếu không nhấp
controls-updates = ⭳ Kiểm tra cập nhật…
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
hotkeys-record-placeholder = ví dụ Ctrl+Shift+R
hotkeys-go-live = Phát trực tiếp / Kết thúc Luồng
hotkeys-go-live-placeholder = ví dụ Ctrl+Shift+L
hotkeys-transition = Chuyển cảnh Chế độ Studio
hotkeys-transition-placeholder = ví dụ Ctrl+Shift+T hoặc F13
hotkeys-save-replay = Lưu Phát lại (N giây cuối)
hotkeys-save-replay-placeholder = ví dụ Ctrl+Shift+S
hotkeys-add-marker = Thả một marker chương (ghi)
hotkeys-add-marker-placeholder = ví dụ Ctrl+Shift+K
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
settings-more-section = Cài đặt khác
settings-open-output = Ghi hình…
settings-open-stream = Phát trực tiếp…
settings-open-replay = Phát lại…
settings-open-hotkeys = Phím tắt…
settings-open-remote = API từ xa…
settings-open-about = Giới thiệu…
controls-settings = ⚙ Cài đặt…
controls-settings-title = Ngôn ngữ, giao diện và các tùy chọn toàn ứng dụng

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
