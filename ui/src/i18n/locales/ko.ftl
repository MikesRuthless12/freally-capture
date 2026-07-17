# Freally Capture — ko
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = 스튜디오 모드
toggle-on = 켜짐
toggle-off = 꺼짐
stats = 통계
core-ok = 코어 정상
hide-stats-dock = 통계 독 숨기기
show-stats-dock = 통계 독 표시


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = 설정을 저장하지 못했습니다 — 변경 사항이 재시작 후 유지되지 않습니다.
studio-mode-leave = 스튜디오 모드 나가기
studio-mode-enter-title = 스튜디오 모드 — 미리 보기 장면을 편집한 뒤 전환 효과와 함께 프로그램으로 송출합니다
vertical-canvas-title = 두 번째(세로 9:16) 출력 캔버스 — 독립적으로 녹화하고 방송할 수 있습니다
app-version = v{ $version }
core-error = 코어 오류
core-unreachable = 코어에 연결할 수 없음 (브라우저 모드)
connecting-to-core = 코어에 연결 중…
filters-source-fallback = 소스

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = 프로그램 미리 보기
preview-program-output = 프로그램 출력
preview-canvas-editor = 캔버스 편집기
preview-px-to-edge-label = 프레임 가장자리까지의 픽셀
preview-px-to-edge = 가장자리까지 px  L { $left } · T { $top } · R { $right } · B { $bottom }
preview-program-heading = 프로그램
preview-no-gpu = 사용 가능한 GPU 어댑터를 찾지 못했습니다 — 이 컴퓨터에서는 컴포지터를 실행할 수 없습니다.
preview-starting-compositor = 컴포지터를 시작하는 중…
preview-empty-scene = 이 장면은 비어 있습니다 — 소스에서 소스를 추가한 다음, 바로 이 캔버스에서 드래그, 크기 조절, 회전하세요.
preview-fps = { $fps } fps
preview-dropped = { $dropped } 드롭됨

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = 초대 링크를 받았습니다
remote-join-with-webcam = 웹캠으로 참여
remote-dismiss = 닫기
remote-hosting-guest = 원격 게스트 호스팅 중
remote-you-are-guest = 원격 게스트로 참여 중입니다
remote-share-view-title = 게스트의 앱으로 화면을 공유합니다 (게스트가 당신의 화면을 실시간으로 봅니다)
remote-stop-sharing-view = 화면 공유 중지
remote-share-my-view = 내 화면 공유
remote-allow-center-title = 게스트가 어떤 화면을 중앙에 둘지 전환하도록 허용합니다 (제어권은 당신에게 있으며 언제든 되돌릴 수 있습니다)
remote-guest-switching = 게스트 전환:
remote-stop-screen = 화면 중지
remote-share-screen = 화면 공유
remote-share-screen-title-guest = 호스트와 화면을 공유합니다 (호스트가 중앙에 둘 수 있는 소스가 됩니다)
remote-center-request-label = 중앙 화면 요청
remote-center = 중앙
remote-center-cam-title = 호스트에게 내 카메라를 중앙에 둘 것을 요청합니다
remote-center-my-cam = 내 카메라
remote-center-screen-title = 호스트에게 내 공유 화면을 중앙에 둘 것을 요청합니다
remote-center-my-screen = 내 화면
remote-center-host-title = 중앙 화면을 호스트의 화면으로 되돌립니다
remote-center-host-view = 호스트 화면
remote-end-session = 세션 종료
remote-leave = 나가기
remote-host-view-heading = 호스트 화면
remote-host-shared-view-label = 호스트가 공유한 화면
remote-guest-position-label = 게스트 위치
remote-guest-label = 게스트
remote-put-guest = 게스트를 { $position }에 배치
remote-remove-title = 게스트 제거 — 같은 링크로 다시 참여할 수 있습니다
remote-remove = 제거
remote-ban-title = 게스트 차단 — 게스트를 차단하고 초대 링크를 무효화합니다
remote-ban = 차단
remote-guest-self-muted = 게스트가 스스로 음소거함
remote-unmute-guest = 게스트 음소거 해제
remote-mute-guest = 게스트 음소거
remote-muted-by-host = 호스트가 음소거함
remote-unmute-mic = 마이크 음소거 해제
remote-mute-mic = 마이크 음소거
remote-waiting-for-host = 호스트를 기다리는 중


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = 소스
sources-fallback-video = 비디오
sources-fallback-error = 오류
sources-kind-unknown = ?
sources-missing-source = (소스 없음)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = 디스플레이
sources-badge-window = 윈도우
sources-badge-portal = 포털
sources-badge-camera = 카메라
sources-badge-image = 이미지
sources-badge-media = 미디어
sources-badge-guest = 게스트
sources-badge-color = 색상
sources-badge-text = 텍스트
sources-badge-scene = 장면
sources-badge-slides = 슬라이드
sources-badge-chat = 채팅
sources-badge-audio-in = 오디오 입력
sources-badge-audio-out = 오디오 출력
sources-badge-app-audio = 앱 오디오
sources-badge-test-bars = 바
sources-badge-test-grid = 그리드
sources-badge-test-sweep = 스위프
sources-badge-test-tone = 톤
sources-badge-test-sync = 동기화
sources-badge-timer = 타이머

# Add-source menu items
sources-add-display = 디스플레이 캡처
sources-add-window = 윈도우 캡처
sources-add-game = 게임 캡처 (먼저 읽어보세요)
sources-add-webcam = 비디오 캡처 장치
sources-add-image = 이미지
sources-add-media = 미디어 (비디오/이미지 파일)
sources-add-remote-guest = 원격 게스트 (P2P 시험판)
sources-add-color = 색상
sources-add-text = 텍스트
sources-add-timer = 타이머 / 시계
sources-add-nested-scene = 중첩 장면
sources-add-slideshow = 이미지 슬라이드쇼
sources-add-chat-overlay = 실시간 채팅 오버레이
sources-add-test-signal = 테스트 신호
sources-add-audio-input = 오디오 입력 캡처
sources-add-audio-output = 오디오 출력 캡처
sources-add-app-audio = 애플리케이션 오디오 (Windows)
sources-add-existing = 기존 소스…

# Panel header + toolbar buttons
sources-panel-title = 소스
sources-group-title = 소스 그룹화 — 두 개 이상의 항목을 선택한 다음 그룹 만들기를 누르세요. 그룹화된 항목은 함께 이동하고 함께 표시/숨김됩니다
sources-group-aria = 소스 그룹화
sources-arrange = 배치: 화면 + 모서리
sources-add-source = 소스 추가
sources-browser-source-note = 브라우저 소스는 별도의 온디맨드 컴포넌트 마일스톤으로 제공됩니다 (~180 MB Chromium 엔진 — 절대 번들되지 않음). 지금은: 윈도우 캡처 + 크로마/색상 키로 실제 브라우저 창을 캡처하거나, 채팅/알림을 독으로 여세요 (컨트롤 → 독).

# Empty state
sources-empty = 이 장면에 소스가 없습니다 — "+"로 디스플레이 캡처, 윈도우, 웹캠, 이미지, 색상 또는 텍스트를 추가하세요. 캔버스에서 드래그, 크기 조절, 회전할 수 있으며, 오른쪽 버튼으로 스택 순서를 바꿉니다.

# Per-row controls
sources-already-in-group = 이미 { $name }에 있음
sources-pick-for-new-group = 새 그룹에 넣을 항목 선택
sources-pick-item-for-group = 새 그룹에 { $name } 선택
sources-hide = 숨기기
sources-show = 표시
sources-hide-item = { $name } 숨기기
sources-show-item = { $name } 표시
sources-unfocus-title = 포커스 해제 — 레이아웃 복원
sources-focus-title = 포커스 — 캔버스 채우기 (발표자 강조)
sources-unfocus-item = { $name } 포커스 해제
sources-focus-item = { $name } 포커스
sources-center-title = 중앙 — 이것을 공유 중앙 화면으로 설정 (카메라는 레일로 이동)
sources-center-item = { $name } 중앙에 두기
sources-rename-item = { $name } 이름 바꾸기
sources-in-group = { $name } 그룹 안에 있음

# Row status + retry
sources-retry-error = 다시 시도 — { $message }
sources-retry-item = { $name } 다시 시도
sources-status-error = 상태: 오류
sources-open-privacy-title = 이 권한에 대한 macOS 개인정보 보호 설정을 엽니다
sources-open-privacy-item = { $name }의 개인정보 보호 설정 열기
sources-privacy-settings-button = 설정
sources-status-starting = 시작하는 중…
sources-status-live = 라이브
sources-status-aria = 상태: { $state }

# Media row pause/resume
sources-media-resume-title = 비디오 재개 (방송에 라이브로 송출됨)
sources-media-pause-title = 비디오 일시정지 — 프레임을 정지하고 무음 처리, 방송에는 라이브로 나감
sources-media-resume-item = { $name } 재개
sources-media-pause-item = { $name } 일시정지

# Hover controls
sources-unlock = 잠금 해제
sources-lock = 잠금
sources-unlock-item = { $name } 잠금 해제
sources-lock-item = { $name } 잠금
sources-raise-title = 스택에서 위로
sources-raise-item = { $name } 위로
sources-lower-title = 스택에서 아래로
sources-lower-item = { $name } 아래로
sources-filters-title = 필터 및 블렌드
sources-filters-item = { $name }의 필터
sources-properties-title = 속성
sources-properties-item = { $name }의 속성
sources-remove-title = 이 장면에서 제거
sources-remove-item = { $name } 제거

# Grouping footer
sources-create-group = 그룹 만들기 ({ $count })
sources-cancel = 취소

# Groups list
sources-groups-aria = 소스 그룹
sources-hide-group = 그룹 숨기기
sources-show-group = 그룹 표시
sources-item-count = · { $count }개 항목
sources-ungroup-title = 그룹 해제 — 항목은 그 자리에 남습니다
sources-ungroup-item = { $name } 그룹 해제

# Live Chat Overlay picker
sources-chat-title = 실시간 채팅 오버레이 추가
sources-chat-youtube-label = YouTube — 채널, watch 또는 live_chat URL (키 없음, 로그인 없음)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  또는 watch?v= URL
sources-chat-twitch-label = Twitch — 채널 이름 (익명으로 읽음, 계정 불필요)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — 채널 슬러그 (공개 엔드포인트, 최선의 노력)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = 메시지는 투명한 배경에 h:mm:ss AM/PM 타임스탬프와 함께 나타납니다 (기본값은 오른쪽 위, 원하는 곳으로 드래그 가능). 채팅이 폭주해도 오래된 줄만 사라질 뿐, 방송이나 녹화를 멈추게 할 수는 없습니다. Facebook 채팅은 본인의 Graph 토큰이 필요하며 아직 구현되지 않았습니다 — 절대 필수가 아니며 위 플랫폼을 막지 않습니다.
sources-chat-add = 채팅 오버레이 추가
sources-chat-default-name = 실시간 채팅

# Image Slideshow picker
sources-slideshow-title = 이미지 슬라이드쇼 추가
sources-slideshow-empty = 아직 이미지가 없습니다 — 찾아보기로 순서대로 추가합니다.
sources-slideshow-remove-slide = 슬라이드 { $number } 제거
sources-slideshow-browse = 이미지 찾아보기…
sources-slideshow-per-slide-label = 슬라이드당 (ms)
sources-slideshow-crossfade-label = 크로스페이드 (ms, 0 = 즉시 전환)
sources-slideshow-loop-label = 반복 (끄면 마지막 슬라이드 유지)
sources-slideshow-shuffle-label = 매 주기마다 섞기
sources-slideshow-note = 크로스페이드는 크기가 같은 이미지를 섞습니다. 크기가 다르면 경계에서 즉시 전환됩니다 (자동 크기 조절 없음).
sources-slideshow-add = 슬라이드쇼 추가 ({ $count })

# Nested Scene picker
sources-nested-title = 중첩 장면 추가
sources-nested-empty = 중첩할 다른 장면이 없습니다 — 먼저 두 번째 장면을 추가하세요.
sources-nested-scene-name = 장면: { $name }
sources-nested-note = 중첩된 장면은 프로그램 캔버스 크기로 실시간 렌더링되며 자체 편집을 따릅니다. 변형, 필터, 블렌드가 다른 소스처럼 적용됩니다. 이 장면을 표시하는 장면이 프로그램일 때 그 오디오 소스가 믹스에 합류합니다.

# Display / Window capture picker
sources-capture-display-title = 디스플레이 캡처 추가
sources-capture-window-title = 윈도우 캡처 추가
sources-capture-looking = 소스를 찾는 중…
sources-capture-none-displays = 캡처할 것이 없습니다 — 디스플레이를 찾지 못했습니다.
sources-capture-none-windows = 캡처할 것이 없습니다 — 창을 찾지 못했습니다.
sources-capture-portal-note = Wayland에서는 시스템 대화상자가 화면이나 창을 선택합니다 — 앱이 전역적으로 캡처할 수 없으므로, 그것이 정직하고 유일한 방법입니다.
sources-capture-window-note = 미리 보기는 실시간으로 갱신됩니다. 최소화된 창은 복원할 때까지 마지막 프레임(또는 아무것도)을 표시합니다.
sources-thumb-no-preview = 미리 보기 없음
sources-thumb-loading = 불러오는 중…

# Video Capture Device picker
sources-webcam-title = 비디오 캡처 장치 추가
sources-webcam-looking = 카메라를 찾는 중…
sources-webcam-none = 카메라나 캡처 카드를 찾지 못했습니다.
sources-webcam-format-label = 형식
sources-webcam-format-auto-loading = 자동 (형식 불러오는 중…)
sources-webcam-format-auto = 자동 (최고 해상도)
sources-webcam-card-presets-label = 카드 프리셋:
sources-webcam-preset-title = 이 카드가 알리는 { $label } 모드를 선택하세요
sources-webcam-add = 카메라 추가

# Audio Input / Output capture picker
sources-audio-output-title = 오디오 출력 캡처 추가
sources-audio-input-title = 오디오 입력 캡처 추가
sources-audio-default-output = 기본 출력 (들리는 소리)
sources-audio-default-input = 기본 입력
sources-audio-looking = 오디오 장치를 찾는 중…
sources-audio-none-output = 데스크톱 오디오 캡처 장치를 찾지 못했습니다.
sources-audio-none-input = 마이크나 라인 입력을 찾지 못했습니다.
sources-audio-input-note = 믹서 스트립에는 VU 미터, 페이더, 음소거, 모니터링, 필터(잡음 제거, 게이트, 컴프레서…), 트랙 지정이 있습니다. 모든 것은 이 컴퓨터에 머뭅니다.

# Application Audio picker
sources-appaudio-title = 애플리케이션 오디오 추가
sources-appaudio-looking = 소리를 내는 앱을 찾는 중…
sources-appaudio-none = 지금 소리를 내는 앱이 없습니다 — 앱에서 재생을 시작한 뒤 새로 고침하세요.
sources-appaudio-refresh = ⟳ 새로 고침
sources-appaudio-note = 해당 앱의 오디오만 정확히 캡처합니다 — 자체 VU, 페이더, 음소거, 필터, 트랙을 가집니다.

# Game Capture picker
sources-game-title = 게임 캡처
sources-game-checking = 확인 중…
sources-game-use-portal = 화면 캡처 사용 (포털)
sources-game-use-window = 대신 윈도우 캡처 사용

# Image picker
sources-image-title = 이미지 추가
sources-image-file-label = 이미지 파일 (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = 이미지 추가

# Path field
sources-browse = 찾아보기…

# Media picker
sources-media-title = 미디어 추가
sources-media-file-label = 미디어 파일 (mp4, mkv, webm, mov, .frec 또는 이미지)
sources-media-loop-label = 반복 (끝에서 처음부터 다시 시작)
sources-media-note = .frec는 자체 freally-video 코덱으로 재생됩니다 — 내려받을 것이 없습니다. 컨테이너 형식(mp4/mkv/webm/…)은 온디맨드 FFmpeg 컴포넌트로 디코딩되며, 그 오디오는 자체 스트립으로 믹서에 들어갑니다.
sources-media-add = 미디어 추가

# Invite expiry options
sources-ttl-15min = 15분
sources-ttl-30min = 30분
sources-ttl-1hour = 1시간
sources-ttl-1day = 1일

# Remote Guest form
sources-remote-copy-failed = 복사하지 못했습니다 — 링크를 선택해 직접 복사하세요
sources-remote-join-failed = 참여 실패: { $error }
sources-remote-title = 원격 게스트 (P2P 시험판)
sources-remote-host-heading = 호스트 — 게스트 초대
sources-remote-start-hosting = 호스팅 시작
sources-remote-expires-label = 만료
sources-remote-invite-expiry-aria = 초대 만료
sources-remote-invite-link-aria = 초대 링크
sources-remote-copied = 복사됨 ✓
sources-remote-copy = 복사
sources-remote-share-note = 이 링크를 공유하세요 (Discord / 문자 / 이메일). 세션 정보를 담고 있으며 설정한 대로 만료됩니다. 게스트가 링크를 열고 웹캠으로 참여합니다.
sources-remote-qr-note = 휴대폰으로 스캔하면 브라우저에서 바로 참여합니다 — 카메라 + 마이크, 설치 불필요. 위의 복사 가능한 freally:// 링크는 Freally Capture가 설치된 컴퓨터에서 앱으로 열립니다.
sources-remote-guest-heading = 게스트 — 초대로 참여
sources-remote-paste-placeholder = 초대 링크를 붙여넣으세요
sources-remote-invite-input-aria = 초대 링크 또는 세션 ID
sources-remote-join = 웹캠으로 참여
sources-remote-session-note = 라이브 세션 컨트롤(음소거, 종료)은 메인 창 상단 바에 남아 있습니다 — 이 대화상자는 닫아도 됩니다.
sources-remote-stop-session = 세션 중지

# Invite QR
sources-invite-qr-aria = 초대 링크 QR 코드

# Remote device pickers
sources-devices-output-unavailable = 출력 라우팅을 사용할 수 없음 — 기본 장치로 재생 중
sources-devices-mic-test-failed = 마이크 테스트 실패: { $error }
sources-devices-heading = 세션 오디오 장치
sources-devices-microphone-label = 마이크
sources-devices-microphone-aria = 세션 마이크
sources-devices-system-default = 시스템 기본값
sources-devices-output-label = 출력
sources-devices-output-aria = 세션 오디오 출력
sources-devices-stop-test = 테스트 중지
sources-devices-test = 테스트 — 내 소리 듣기
sources-devices-testing-note = 마이크에 말해 보세요 — 선택한 장치로 실시간으로 듣고 있습니다
sources-devices-idle-note = 마이크를 출력으로 되돌립니다 (헤드폰을 쓰면 하울링을 피할 수 있습니다)

# TURN relay section
sources-turn-save-failed = 저장하지 못했습니다: { $error }
sources-turn-summary = 네트워크 — 선택적 TURN 릴레이 (고급)
sources-turn-note-1 = 세션은 직접(P2P) 연결됩니다 — 무료이며 릴레이가 필요 없습니다. 양쪽 모두 엄격한 NAT 뒤에 있으면 직접 경로가 실패할 수 있는데, 그때는 직접 운영하는 TURN 릴레이가 미디어를 전달합니다. 건너뛰어도 괜찮습니다 — 대부분의 연결은 직접만으로 작동합니다.
sources-turn-note-2 = 무료 옵션: Oracle Cloud "Always Free"로 coturn을 무료로 실행할 수 있습니다 (참고: Oracle은 가입 시 신용카드를 요구하지만 Always-Free 구성은 무료로 유지됩니다). 단계: 1) 무료 VM 생성, 2) coturn 설치, 3) UDP 3478 개방, 4) 사용자/비밀번호 설정, 5) 여기에 turn:your-vm-ip:3478 + 자격 증명 입력. 자격 증명은 로컬 설정 파일에 저장되며 절대 기록되지 않습니다.
sources-turn-url-label = TURN URL
sources-turn-url-placeholder = turn:host:3478 (비우면 직접 연결만)
sources-turn-url-aria = TURN URL
sources-turn-username-label = 사용자 이름
sources-turn-username-aria = TURN 사용자 이름
sources-turn-credential-label = 자격 증명
sources-turn-credential-aria = TURN 자격 증명
sources-turn-note-3 = 세 필드가 모두 설정되면 릴레이가 작동하며(TURN 서버에는 자격 증명이 필요합니다) 다음에 시작하거나 참여하는 세션에 적용됩니다. 본인의 두 컴퓨터 간 릴레이 전용 테스트 통화로 확인하세요.
sources-turn-settings-unavailable = 설정을 사용할 수 없음 (브라우저 모드)

# Color picker
sources-color-title = 색상 추가
sources-color-label = 색상
sources-color-width-label = 너비
sources-color-height-label = 높이
sources-color-add = 색상 추가
sources-testsignal-title = 테스트 신호 추가
sources-testsignal-pattern-label = 패턴
sources-testsignal-bars = SMPTE 컬러 바
sources-testsignal-grid = 보정 그리드
sources-testsignal-sweep = 모션 스위프
sources-testsignal-tone = 1 kHz 톤 (−20 dBFS)
sources-testsignal-flash-beep = A/V 동기화 플래시 + 비프
sources-testsignal-note = 카메라 없이 장면, 인코더, 프로젝터, 스트림 대상을 점검하세요. 플래시 + 비프 패턴은 A/V 동기화 워크벤치에 사용됩니다.
sources-testsignal-add = 테스트 신호 추가
sources-timer-title = 타이머 추가
sources-timer-mode-label = 모드
sources-timer-wall-clock = 벽시계
sources-timer-countdown = 카운트다운
sources-timer-stopwatch = 스톱워치
sources-timer-since-live = 방송 시작 후 시간
sources-timer-since-recording = 녹화 시작 후 시간
sources-timer-note = 길이·형식·스타일·카운트다운 종료 동작은 소스의 속성에서 설정합니다.
sources-timer-add = 타이머 추가

# Text picker
sources-text-title = 텍스트 추가
sources-text-label = 텍스트
sources-text-default = 텍스트
sources-text-color-label = 색상
sources-text-color-aria = 텍스트 색상
sources-text-size-label = 크기 (px)
sources-text-note = 글꼴, 정렬, 줄바꿈, RTL은 소스의 속성에 있습니다. 번들된 Noto Sans(아랍어/히브리어 포함)가 기본값이며 — 모든 컴퓨터에서 동일합니다.
sources-text-add = 텍스트 추가

# Existing source picker
sources-existing-title = 기존 소스 추가
sources-existing-empty = 아직 소스가 없습니다 — 먼저 아무 장면에나 하나 추가하세요. 기존 소스는 공유됩니다: 하나의 이름을 바꾸거나 다시 구성하면 그것을 표시하는 모든 장면이 갱신됩니다.

# Screen + corners layout
sources-slot-off = 끔
sources-slot-center = 중앙 (화면)
sources-slot-top-left = 왼쪽 위
sources-slot-top-right = 오른쪽 위
sources-slot-bottom-left = 왼쪽 아래
sources-slot-bottom-right = 오른쪽 아래
sources-layout-title = 배치: 화면 + 모서리
sources-layout-empty = 먼저 이 장면에 화면 캡처와 하나 이상의 카메라를 추가한 다음 여기에서 배치하세요.
sources-layout-note = 중앙에 화면을 두고 모서리에 최대 네 대의 카메라를 배치하세요 — 설명/팟캐스트 레이아웃입니다. 각 모서리에는 웹캠, 캡처한 통화 창, 또는 미디어 클립을 둘 수 있습니다. 이후 캔버스에서 자유롭게 드래그할 수 있습니다.
sources-layout-slot-aria = { $name } 슬롯
sources-layout-apply = 레이아웃 적용


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = 컨트롤
controls-start-stop-title-stop = 녹화를 중지하고 마무리합니다
controls-start-stop-title-start = 설정 → 출력 구성으로 프로그램 피드를 녹화합니다
controls-finalizing = ◌ 마무리 중…
controls-stop-recording = ■ 녹화 중지
controls-start-recording = ● 녹화 시작
controls-marker-title = 이 순간에 챕터 마커를 남깁니다 — 녹화 파일에 들어갑니다 (mkv 챕터 또는 사이드카 파일). 플랫폼 측 방송 마커는 플랫폼 계정이 필요하며, 이 앱은 절대 요구하지 않습니다.
controls-marker = ◈ 마커
controls-iso-lanes = 프로그램과 나란히 녹화 중인 ISO 레인: { $count }
controls-pause-title-resume = 재개 — 파일이 하나의 연속된 타임라인으로 이어집니다
controls-pause-title-pause = 일시정지 — 프레임이 기록되지 않으며, 재개하면 같은 재생 가능한 파일로 이어집니다
controls-resume-recording = ▶ 녹화 재개
controls-pause-recording = ⏸ 녹화 일시정지
controls-reactions-label = 리액션 (프로그램에 합성됨)
controls-reactions-title = 프로그램 위에 리액션을 띄웁니다 — 녹화되고 방송되어 다시 보기에서 그 순간이 그대로 나옵니다. 채팅 시청자도 이를 트리거합니다 (시청자의 리액션 이모지가 자동으로 떠오릅니다). 폭주해도 화면에 표시되는 수만 제한됩니다.
controls-react = 리액션 { $emoji }
controls-virtual-camera-title = 가상 카메라는 OS별로 자체 서명된 드라이버 컴포넌트가 필요합니다 (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO 확장 / Linux v4l2loopback) — 자체 마일스톤으로 제공됩니다. 피드 모델은 준비되어 있습니다: 프로그램, 세로 캔버스, 또는 단일 소스를, Windows/Linux에서는 짝을 이루는 가상 마이크와 함께 (macOS에는 가상 마이크 API가 없습니다 — 정직하게 말합니다).
controls-virtual-camera = ⌁ 가상 카메라 시작
controls-saved = 저장됨: { $path }

# --- MixerDock.tsx ---
mixer-title = 오디오 믹서
mixer-monitor-error = 모니터: { $error }
mixer-switch-to-horizontal = 가로 스트립으로 전환
mixer-switch-to-vertical = 세로 스트립으로 전환
mixer-layout-aria-vertical = 믹서 레이아웃: 세로 — 가로로 전환
mixer-layout-aria-horizontal = 믹서 레이아웃: 가로 — 세로로 전환
mixer-empty = 이 장면에 오디오 소스가 없습니다 — 소스에서 "+"로 오디오 입력 캡처(마이크)나 오디오 출력 캡처(데스크톱 오디오)를 추가하세요. 스트립에는 VU 미터, 페이더, 음소거, 모니터링, 필터, 트랙 지정이 있습니다.
mixer-advanced-title = 오디오 — { $name }
mixer-loudness-label = 프로그램 라우드니스 (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = 순간 라우드니스 (400 ms)
mixer-short-term-title = 단기 라우드니스 (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = 모니터
mixer-monitor-device-aria = 모니터 출력 장치
mixer-default-output = 기본 출력
mixer-routing = 라우팅
mixer-routing-title = 오디오 출력 라우팅

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = 오디오 라우팅
routing-intro = 스트립을 트랙 버스에 할당한 다음, 원하는 버스를 물리적 출력으로 보냅니다 — 하드웨어 레코더 피드, 다른 방의 스피커, 또는 여분 트랙의 헤드폰 큐 등입니다. 모니터는 자체 장치를 유지하며, 이 경로는 그 위에 추가되므로 아무것도 설정하지 않으면 믹스는 그대로입니다.
routing-sends-title = 트랙 센드
routing-no-strips = 이 장면에 오디오 소스가 없습니다.
routing-source = 소스
routing-track = 트랙 { $n }
routing-send-aria = { $source }을(를) 트랙 { $n }으로 보내기
routing-outputs-title = 물리적 출력
routing-master = 마스터
routing-off = 끔
routing-default-output = 기본 출력
routing-device-aria = { $bus }의 출력 장치
routing-trim-aria = { $bus }의 출력 트림
routing-trim-db = { $db } dB
routing-muted = 음소거됨
routing-device-error = 장치를 사용할 수 없음

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = 더킹
mixer-ducking-title = 더킹 매트릭스
ducking-title = 더킹 매트릭스
ducking-intro = 어떤 소스든 다른 소스를 더킹할 수 있습니다. 트리거(행)가 소리를 낼 때마다 셀이 대상(열)을 낮춥니다 — 셀을 선택해 깊이, 스레숄드, 타이밍을 설정하세요. 각 쌍은 개별 더킹이므로 한 스트립을 여러 트리거가 동시에 더킹할 수 있습니다.
ducking-need-two = 서로 더킹하려면 오디오 소스를 두 개 이상 추가하세요.
ducking-trigger-target = 트리거 ↓ / 대상 →
ducking-cell-aria = { $trigger }이(가) { $target }을(를) 더킹
ducking-pair = { $trigger } → { $target }
ducking-remove = 제거
ducking-amount = 양
ducking-threshold = 스레숄드
ducking-attack = 어택
ducking-release = 릴리스
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = 라우드니스 정규화
loudness-intro = 피크 실링과 함께 프로그램을 라우드니스 목표로 천천히 이끌어, 스트림과 녹화가 일관된 레벨에 도달하도록 합니다. 느리고 부드럽게 — 조종할 뿐, 절대 펌핑하지 않습니다.
loudness-enable = 프로그램을 목표로 이끌기
loudness-target = 목표
loudness-target-option = { $target } LUFS
loudness-ceiling = 피크 실링 (dBFS)
loudness-note = −14 LUFS는 YouTube 스타일 재생에 적합합니다. −16은 일반적인 스트리밍 목표입니다. −23은 EBU R128 방송입니다. 녹화 후 정규화 작업에도 동일한 목표가 사용됩니다.
ltc-badge = LTC
ltc-title = SMPTE 타임코드(LTC)
ltc-intro = 트랙에 SMPTE 선형 타임코드를 생성하고, 어떤 오디오 입력에서든 들어오는 LTC를 읽습니다 — 외부 레코더와 카메라를 후반에 동기화하는 고전적인 오디오 타임코드. 완전 오프라인.
ltc-generate = 트랙에 LTC 생성
ltc-track = 타임코드 트랙
ltc-track-option = 트랙 { $track }
ltc-fps = 프레임 레이트
ltc-read = LTC 읽기 소스
ltc-read-off = 끔
ltc-decoded = 수신 타임코드
ltc-no-lock = 신호 없음
ltc-note = 생성기는 하루의 시각에 잼싱크합니다(논드롭). 그 트랙을 녹화하거나(출력 설정에서 지정) 출력으로 라우팅해 외부 장비에 공급하세요. 리더는 통계 오버레이의 타임코드 줄을 구동하고 챕터 마커에 각인합니다.
loudness-on = LUFS { $target }
loudness-off = 정규화 끔

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = 사운드보드
mixer-soundboard-title = 사운드보드
soundboard-title = 사운드보드
soundboard-add-pad = + 패드
soundboard-stop-all = 모두 중지
soundboard-edit = 편집
soundboard-empty = 아직 패드가 없습니다 — 하나 추가하고 로컬 오디오 클립을 지정하세요.
soundboard-new-pad = 새 패드
soundboard-no-clip = 클립 없음
soundboard-audio-files = 오디오 파일
soundboard-name = 이름
soundboard-choose-clip = 클립 선택…
soundboard-gain = 게인
soundboard-choke = 초크
soundboard-choke-none = 없음
soundboard-loop = 반복
soundboard-auto-duck = 자동 더킹
soundboard-tracks = 트랙
soundboard-hotkey = 단축키
soundboard-hotkey-placeholder = 예: Ctrl+Shift+1
soundboard-remove = 제거

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = 플러그인
mixer-plugins-title = 오디오 플러그인 (CLAP / VST3)
plugins-title = 오디오 플러그인
plugins-scanning = 검색 중…
plugins-none = 표준 폴더에서 CLAP 또는 VST3 플러그인을 찾을 수 없습니다.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = 메모리
stats-dropped = 드롭됨
stats-render = 렌더
stats-gpu = GPU
stats-gpu-compositing = 합성 중
stats-gpu-idle = 유휴
stats-disk = 디스크
stats-disk-free = 여유
stats-disk-left = 녹화 남음
stats-disk-rate = ≈ { $rate } MB/s 녹화
stats-vertical-fps = 9:16 FPS
stats-targets-label = 스트림 대상
stats-shared-encode = · 공유 인코딩
stats-starting = 컴포지터를 시작하는 중…

# --- ScenesRail.tsx ---
scenes-title = 장면
scenes-new-scene-name = 장면
scenes-add = 장면 추가
scenes-empty = 스튜디오 코어에 연결하는 중…
scenes-rename = { $name } 이름 바꾸기
scenes-on-program = 프로그램에 표시 중
scenes-preview = { $name } 미리 보기
scenes-switch-to = { $name }(으)로 전환
scenes-move-up = 위로 이동
scenes-move-up-aria = { $name } 위로 이동
scenes-move-down = 아래로 이동
scenes-move-down-aria = { $name } 아래로 이동
scenes-last-stays = 마지막 장면은 유지됩니다
scenes-remove = 이 장면 제거
scenes-remove-aria = { $name } 제거


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = 레벨
channelstrip-monitor-off = 모니터 끔
channelstrip-monitor-only = 모니터만 (믹스에는 포함 안 됨)
channelstrip-monitor-and-output = 모니터 및 출력
channelstrip-status-error = 오류
channelstrip-status-live = 라이브
channelstrip-status-waiting-audio = 오디오 대기 중
channelstrip-status = 상태: { $state }
channelstrip-status-waiting = 대기 중
channelstrip-mute = 음소거
channelstrip-unmute = 음소거 해제
channelstrip-mute-source = { $name } 음소거
channelstrip-unmute-source = { $name } 음소거 해제
channelstrip-scene-mix-on = 장면별 믹스 켜짐 — 이 스트립이 이 장면에서 전역 믹스를 재정의합니다 (다시 전역 믹스를 따르려면 클릭)
channelstrip-scene-mix-off = 장면별 믹스 — 현재 장면에 이 스트립만의 페이더/음소거를 부여합니다
channelstrip-scene-mix-label = { $name }의 장면별 믹스
channelstrip-monitor-cycle = { $mode } — 클릭하여 순환
channelstrip-monitor-mode = { $name }의 모니터 모드: { $mode }
channelstrip-audio-filters-title = 오디오 필터 (잡음 제거, 게이트, 컴프레서…)
channelstrip-audio-filters-label = { $name }의 오디오 필터
channelstrip-advanced-title = 싱크 오프셋 및 푸시투토크 단축키
channelstrip-advanced-label = { $name }의 고급 오디오 설정
channelstrip-track-assignment = 트랙 지정
channelstrip-track = 트랙 { $n }
channelstrip-track-assigned = 트랙 { $n } (지정됨)
channelstrip-track-label = { $name }의 트랙 { $n }
channelstrip-device-error = 장치 오류
channelstrip-audio-device-error = 오디오 장치 오류
channelstrip-volume-label = { $name }의 볼륨 (데시벨)
channelstrip-ptt-hold = 푸시투토크: { $key } 누르고 있기
channelstrip-sync-offset = 싱크 오프셋 (ms, 0–{ $max } — 이 오디오를 지연시킴)
channelstrip-solo-title = 솔로(PFL) — 모니터는 솔로된 스트립만 들리며 프로그램 믹스는 그대로입니다
channelstrip-solo-source = { $name } 솔로(PFL)
channelstrip-pan-label = 밸런스 (더블 클릭으로 초기화)
channelstrip-pan-aria = { $name } 밸런스
channelstrip-mono-label = 모노로 다운믹스
channelstrip-automix-label = 오토믹스 (게인 셰어링)
channelstrip-automix-note = 게인 셰어링: 믹서가 모든 오토믹스 스트립의 합산 레벨을 일정하게 유지하고, 지금 말하는 사람에게 넘겨줍니다 — 멀티 마이크 패널과 팟캐스트에 적합합니다. 스트립을 추가하기 전까지는 꺼져 있습니다.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = 이 소스를 위한 에코 없는 리턴을 생성합니다 — 이 소스 자신을 제외한 프로그램의 모든 사람. 원격 게스트에게 사용하면 자신의 지연된 목소리를 듣지 않습니다.
channelstrip-ptt-hotkey = 푸시투토크 단축키 (누르고 있을 때만 소리)
channelstrip-ptt-placeholder = 예: Ctrl+Shift+T 또는 F13
channelstrip-ptt-aria = 푸시투토크 단축키
channelstrip-ptm-hotkey = 푸시투뮤트 단축키 (누르고 있는 동안 무음)
channelstrip-ptm-placeholder = 예: Ctrl+Shift+M
channelstrip-ptm-aria = 푸시투뮤트 단축키
channelstrip-hotkeys-note = 단축키는 다른 앱이 활성화된 동안에도 작동합니다. Linux/Wayland에서는 전역 단축키를 사용할 수 없을 수 있습니다 — 컴포지터의 한계이며, 정직하게 말합니다.
channelstrip-apply = 적용


# --- LiveButton.tsx ---
livebutton-failure-ended = 방송이 종료되었습니다
livebutton-title-live = 방송 종료 — 모든 대상 (진행 중인 녹화는 계속됩니다)
livebutton-title-offline = 활성화된 모든 설정 → 스트림 대상으로 라이브 시작
livebutton-end-stream = ■ 방송 종료
livebutton-aria-reconnecting = 재연결 중
livebutton-aria-live = 라이브
livebutton-badge-retry = 재시도 { $n }
livebutton-badge-live = 라이브
livebutton-go-live = ⦿ 라이브 시작


# --- RecDot.tsx ---
recdot-paused-aria = 녹화 일시정지됨
recdot-recording-aria = 녹화 중
recdot-tracks-one = 오디오 트랙 { $count }개 녹화 중
recdot-tracks-other = 오디오 트랙 { $count }개 녹화 중
recdot-paused = 일시정지됨


# --- ReplayControls.tsx ---
replaycontrols-saved = 리플레이 저장됨 — { $name }
replaycontrols-failure-stopped = 버퍼가 중지되었습니다
replaycontrols-title-disarm = 리플레이 버퍼 해제 (저장하지 않은 기록을 버립니다)
replaycontrols-title-arm = 롤링 리플레이 버퍼 활성화 — 최근 N초를 저장 준비 상태로 유지합니다 (자체 경량 인코딩. 방송과 녹화는 영향받지 않습니다)
replaycontrols-replay-seconds = ⟲ 리플레이 { $seconds }초
replaycontrols-arm = ⟲ 리플레이 버퍼 활성화
replaycontrols-save-title = 최근 N초를 녹화 폴더에 저장 (리플레이 저장 단축키로도 가능)
replaycontrols-save = ⤓ 저장


# --- PropertiesDialog.tsx ---
properties-title = 속성 — { $name }
properties-name = 이름
properties-cancel = 취소
properties-apply = 적용
properties-youtube = YouTube — 채널 / watch / live_chat URL (키 없음, 로그인 없음, 절대)
properties-twitch = Twitch — 채널 이름 (익명)
properties-kick = Kick — 채널 슬러그 (공개 엔드포인트)
properties-width-px = 너비 (px)
properties-lines = 줄
properties-font-px = 글꼴 (px)
properties-images = 이미지 파일 (한 줄에 하나씩, 순서대로 표시)
properties-per-slide = 슬라이드당 (ms)
properties-crossfade = 크로스페이드 (ms, 0 = 즉시 전환)
properties-loop-slideshow = 반복 (끄면 마지막 슬라이드 유지)
properties-shuffle = 매 주기마다 섞기
properties-nested-scene = 이 소스가 구성하는 장면 (이미 이 장면을 포함한 장면은 거부됨)
properties-portal-note = Wayland ScreenCast 포털은 이 소스가 시작될 때마다 시스템 대화상자에서 화면이나 창을 선택합니다 — 설계상 여기서 설정할 것이 없습니다.
properties-appaudio-capturing = { $exe }에서 오디오 캡처 중
properties-appaudio-exe-fallback = 애플리케이션
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = 다른 앱을 대상으로 하려면 소스를 다시 추가하세요 (앱을 재시작하면 프로세스 ID가 바뀝니다).
properties-image-file = 이미지 파일
properties-media-file = 미디어 파일 (mp4, mkv, webm, mov, .frec 또는 이미지)
properties-media-loop = 반복 (끝에서 처음부터 다시 시작)
properties-media-hwdecode = 하드웨어 디코딩 (자동으로 소프트웨어로 대체됨)
properties-media-note = .frec는 자체 freally-video 코덱으로 재생됩니다 — 내려받을 것이 없습니다. 다른 비디오 형식은 온디맨드 FFmpeg 컴포넌트로 디코딩됩니다. 파일의 오디오는 자체 믹서 스트립을 가지며, 스트립의 싱크 오프셋이 A/V 정렬을 미세 조정합니다. 오디오가 없는 클립은 스트립을 무음으로 둡니다.
properties-color = 색상
properties-width = 너비
properties-height = 높이
properties-testtone-note = −20 dBFS의 연속 1 kHz 사인파입니다. 레벨과 음소거는 믹서 스트립에서 조절하며, 그 외 설정할 것은 없습니다.
properties-timer-format = 시간 형식 (strftime)
properties-timer-format-note = 예: %H:%M:%S(기본), %I:%M %p, %A %H:%M — 잘못된 패턴은 %H:%M:%S로 돌아갑니다.
properties-timer-utc = UTC 오프셋 (분)
properties-timer-utc-placeholder = 현지 시간
properties-timer-duration = 길이 (초)
properties-timer-target = 이 시각까지 카운트다운 (HH:MM)
properties-timer-target-note = 벽시계 목표는 스스로 진행되며 매일 반복됩니다. 비워 두면 길이 + 시작/일시정지/재설정으로 동작합니다.
properties-timer-end = 0이 되면
properties-timer-end-none = 아무것도 안 함
properties-timer-end-flash = 타이머 깜빡임
properties-timer-end-switch = 장면 전환
properties-timer-end-scene = 장면
properties-timer-size = 크기 (px)
properties-timer-start = 시작
properties-timer-pause = 일시정지
properties-timer-reset = 재설정
properties-text-file = 파일에서 읽기 (경로, 비우면 위 텍스트 사용)
properties-text-binding = 해석 방식
properties-text-binding-whole = 파일 전체
properties-text-binding-csv = CSV 셀
properties-text-binding-json = JSON 포인터
properties-text-csv-row = 행
properties-text-csv-column = 열
properties-text-csv-column-placeholder = 이름 또는 번호
properties-text-json-pointer = 포인터
properties-text-file-note = 변경 후 0.5초 안에 다시 읽습니다. 원자적 쓰기(임시 파일 + 이름 변경)도 견딥니다: 교체 중에는 마지막 정상 값이 화면에 유지됩니다.
avsync-title = A/V 동기화 보정
avsync-intro = 내장 플래시+비프 패턴을 디스플레이와 스피커로 재생하고, 맞추려는 카메라와 마이크로 다시 담으면 워크벤치가 그 차이를 측정합니다. 루프가 화면과 스피커를 거치므로 그 작은 지연도 포함됩니다.
avsync-video-label = 카메라 (비디오 소스)
avsync-audio-label = 마이크 (오디오 소스)
avsync-pick = 소스 선택…
avsync-no-video = 먼저 카메라를 소스로 추가하세요 — 워크벤치는 장치가 아니라 소스를 측정합니다.
avsync-no-audio = 먼저 마이크를 오디오 소스로 추가하세요.
avsync-projector = 프로그램 전체 화면 표시 위치
avsync-projector-open = 프로젝터 열기
avsync-projector-window-title = 프로그램 — A/V 동기화
avsync-start-note = 시작하면 현재 장면 위에 임시 "A/V 동기화 패턴" 소스가 추가되고 비프가 모니터 장치에서 재생됩니다. 끝나면 모두 제거됩니다.
avsync-manual = 동기화 오프셋 (ms, 수동)
avsync-start = 보정 시작
avsync-measuring = 약 12초간 측정합니다 — 카메라를 깜빡이는 프로그램에 향하게 하고 방을 조용히 유지하세요…
avsync-flash-seen = 카메라가 플래시를 봅니다
avsync-flash-waiting = 카메라가 플래시를 보기를 기다리는 중…
avsync-beep-heard = 마이크가 비프를 듣습니다
avsync-beep-waiting = 마이크가 비프를 듣기를 기다리는 중…
avsync-cancel = 취소
avsync-result-offset = 비디오가 오디오보다 { $offset } ms 늦게 도착합니다.
avsync-result-detail = { $cycles }회 주기로 측정, ±{ $jitter } ms.
avsync-negative = 오디오가 이미 비디오보다 늦습니다. 오디오 지연으로는 이 방향을 고칠 수 없습니다 — 이 카메라의 소리를 다른 스트립이 담당한다면 그쪽 오프셋을 낮추세요.
avsync-over-cap = 측정된 차이가 동기화 오프셋 한도 { $max } ms를 넘습니다. 이 정도 차이는 대개 소스를 잘못 고른 것입니다 — 체인을 확인하고 다시 측정하세요.
avsync-applied = 적용됨 — 마이크의 동기화 오프셋은 이제 { $offset } ms입니다.
avsync-apply = 마이크에 { $offset } ms 적용
avsync-again = 다시 측정
avsync-close = 닫기
avsync-error-noFlash = 카메라가 플래시를 한 번도 보지 못했습니다. 깜빡이는 프로그램을 향하게 하고(전체 화면 권장) 소스가 라이브인지 확인한 뒤 다시 측정하세요.
avsync-error-noBeep = 마이크가 비프를 한 번도 듣지 못했습니다. 모니터 장치가 들리는지, 마이크가 라이브인지(푸시투토크로 막히지 않았는지) 확인하고 다시 측정하세요.
avsync-error-tooFewCycles = 깨끗한 플래시/비프 주기가 부족합니다. 측정 내내 패턴이 잘 보이고 들리게 유지하세요.
avsync-error-notThePattern = 감지된 것이 패턴의 리듬으로 반복되지 않습니다 — 테스트 신호가 아니라 방의 조명이나 소음일 가능성이 큽니다.
avsync-error-unstable = 주기 간 편차가 너무 커서 하나의 값을 신뢰할 수 없습니다. 카메라를 고정하고 소음을 줄인 뒤 다시 측정하세요.
hotkey-audit-title = 단축키 지도
hotkey-audit-search = 검색
hotkey-audit-filter = 기능
hotkey-audit-filter-all = 모든 기능
hotkey-audit-col-key = 키
hotkey-audit-col-action = 동작
hotkey-audit-col-where = 위치
hotkey-audit-col-status = 상태
hotkey-audit-ok = 정상
hotkey-audit-shared = { $count }개 바인딩이 공유
hotkey-audit-unregistered = OS에 등록되지 않음(다른 곳에서 사용 중이거나 사용 불가)
hotkey-audit-invalid = 올바른 단축키가 아님
hotkey-audit-empty = 아직 단축키가 없습니다 — 설정 → 단축키 또는 믹서 스트립에서 지정하세요.
hotkey-audit-export = 치트 시트 내보내기
hotkey-audit-exported = { $path }에 저장됨
hotkey-audit-note = 키 지정과 변경은 설정 → 단축키(전역 동작)와 각 믹서 스트립(푸시투토크/푸시투뮤트)에서 합니다. 이 표는 감사와 문서화를 담당합니다.
hotkey-audit-action-record = 녹화 전환
hotkey-audit-action-go-live = 방송 전환
hotkey-audit-action-transition = 전환 실행
hotkey-audit-action-save-replay = 리플레이 저장
hotkey-audit-action-add-marker = 마커 추가
hotkey-audit-action-still = 스틸 캡처
hotkey-audit-action-panic = 패닉 화면
hotkey-audit-action-timer-toggle = 모든 타이머 시작/일시정지
hotkey-audit-action-timer-reset = 모든 타이머 재설정
hotkey-audit-action-ptt = 푸시투토크
hotkey-audit-action-ptm = 푸시투뮤트
hotkey-audit-feature-recording = 녹화
hotkey-audit-feature-streaming = 방송
hotkey-audit-feature-studio = 스튜디오 모드
hotkey-audit-feature-replay = 리플레이
hotkey-audit-feature-markers = 마커
hotkey-audit-feature-stills = 스틸
hotkey-audit-feature-panic = 패닉
hotkey-audit-feature-timers = 타이머
hotkey-audit-feature-audio = 오디오(소스별)
properties-text = 텍스트
properties-font-family = 글꼴 (시스템; 비우면 기본값)
properties-size-px = 크기 (px)
properties-text-color = 텍스트 색상
properties-align = 정렬
properties-align-left = 왼쪽
properties-align-center = 가운데
properties-align-right = 오른쪽
properties-line-spacing = 줄 간격
properties-wrap-width = 줄바꿈 너비 (px; 0 = 끔)
properties-force-rtl = 오른쪽에서 왼쪽 강제
properties-text-note = 렌더링은 실제 셰이핑(아랍어 결합, 합자)과 양방향 줄 정렬을 사용합니다. 번들된 Noto Sans 계열(아랍어/히브리어 포함)이 기본값이며, 시스템 계열도 작동합니다. CJK는 현재 시스템 글꼴을 사용합니다.
properties-repick-capturing = 캡처 중: { $label }
properties-repick-looking = 소스를 찾는 중…
properties-repick-none-displays = 다시 선택할 디스플레이를 찾지 못했습니다.
properties-repick-none-windows = 다시 선택할 창을 찾지 못했습니다.
properties-repick-again = 다시 선택:
properties-device = 장치
properties-video-current-device = (현재 장치)
properties-format = 형식
properties-format-auto-loading = 자동 (형식 불러오는 중…)
properties-deinterlace = 디인터레이스
properties-deinterlace-off = 끔
properties-deinterlace-discard = 버림(한 필드 라인 복제)
properties-deinterlace-bob = 밥(필드 교대)
properties-deinterlace-linear = 선형(보간)
properties-deinterlace-blend = 블렌드(필드 평균)
properties-deinterlace-adaptive = 움직임 적응형(yadif급)
properties-field-order = 필드 순서
properties-field-order-top = 상단 필드 먼저
properties-field-order-bottom = 하단 필드 먼저
properties-deinterlace-note = 인터레이스 캡처카드 입력용. 순수 CPU 처리로 모든 OS에서 동일하며, 변경하면 장치가 다시 시작됩니다(형식 변경과 같음).
camera-controls-title = 카메라 컨트롤
camera-controls-refresh = 새로 고침
camera-controls-reset = 프로필 재설정
camera-controls-empty = 지금은 컨트롤이 없습니다 — 장치가 스트리밍 중이어야 하며(먼저 장면에 추가), 일부 백엔드는 아무것도 보고하지 않습니다(특히 macOS). OS별 정직한 상태입니다.
camera-controls-note = 변경은 즉시 적용되고 장치 프로필에 저장되며, 재연결·재시작 시 다시 적용됩니다.
camera-control-brightness = 밝기
camera-control-contrast = 대비
camera-control-hue = 색조
camera-control-saturation = 채도
camera-control-sharpness = 선명도
camera-control-gamma = 감마
camera-control-white-balance = 화이트 밸런스
camera-control-backlight = 역광 보정
camera-control-gain = 게인
camera-control-pan = 팬
camera-control-tilt = 틸트
camera-control-zoom = 줌
camera-control-exposure = 노출
camera-control-iris = 조리개
camera-control-focus = 초점
properties-format-auto = 자동 (최고 해상도)
properties-audio-capture-of = 오디오를 캡처할 대상
properties-audio-default-output = 기본 출력 (들리는 소리)
properties-audio-default-input = 기본 입력
properties-audio-default-suffix = (기본값)
properties-audio-current-device = (현재 장치: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = 게인
audiofilters-name-noise-gate = 노이즈 게이트
audiofilters-name-compressor = 컴프레서
audiofilters-name-limiter = 리미터
audiofilters-name-eq = 3밴드 EQ
audiofilters-name-denoise = 잡음 제거
audiofilters-name-ducking = 더킹
audiofilters-name-parametric-eq = 파라메트릭 EQ
audiofilters-name-de-esser = 디에서
audiofilters-name-rumble-guard = 럼블 필터
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = 프리셋
audiofilters-voice-preset-pick = 음성 프리셋…
audiofilters-voice-broadcast = 방송 음성
audiofilters-voice-podcast = 팟캐스트 음성
audiofilters-voice-clean = 깨끗한 음성
audiofilters-voice-none = 체인 지우기
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = 치찰음 주파수 (Hz)
audiofilters-deesser-amount = 최대 감쇠 (dB)
audiofilters-rumble-freq = 로우 컷 (Hz)
audiofilters-title = 오디오 필터 — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = 실시간 스펙트럼이 있는 파라메트릭 EQ 응답 곡선
eq-band-type = 유형
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + 밴드
eq-remove-band = 밴드 제거
eq-type-bell = 벨
eq-type-lowShelf = 로우 셸프
eq-type-highShelf = 하이 셸프
eq-type-notch = 노치
eq-type-highPass = 하이패스
eq-type-lowPass = 로우패스
audiofilters-chain-header = 필터 체인 (위쪽이 먼저 실행됨, 페이더 이전)
audiofilters-add = + 필터 추가
audiofilters-add-menu = 오디오 필터 추가
audiofilters-empty = 아직 필터가 없습니다 — 마이크 잡음을 제거(클래식 DSP, ML 없음)하거나, 방 소음을 게이트하거나, 컴프레서로 피크를 다듬거나, 음악을 목소리 아래로 더킹하세요.
audiofilters-enable = { $name } 활성화
audiofilters-run-earlier = 더 먼저 실행
audiofilters-move-up = { $name } 위로
audiofilters-run-later = 더 나중에 실행
audiofilters-move-down = { $name } 아래로
audiofilters-remove-title = 필터 제거
audiofilters-remove = { $name } 제거
audiofilters-gain-db = 게인 (dB)
audiofilters-open-db = 열림 기준 (dB)
audiofilters-close-db = 닫힘 기준 (dB)
audiofilters-attack-ms = 어택 (ms)
audiofilters-hold-ms = 홀드 (ms)
audiofilters-release-ms = 릴리스 (ms)
audiofilters-ratio = 비율 (:1)
audiofilters-threshold-db = 스레숄드 (dB)
audiofilters-output-gain-db = 출력 게인 (dB)
audiofilters-ceiling-db = 실링 (dB)
audiofilters-low-db = 저역 (dB)
audiofilters-mid-db = 중역 (dB)
audiofilters-high-db = 고역 (dB)
audiofilters-strength = 강도
audiofilters-denoise-note = 자체 클래식 DSP 스펙트럼 억제 — 일정한 잡음(팬, 히스)은 줄이고 음성은 통과시킵니다. 헌장에 따라 ML도 모델도 없습니다.
audiofilters-duck-under = 아래로 더킹할 대상
audiofilters-ducking-trigger = 더킹 트리거 소스
audiofilters-pick-trigger = (트리거 선택 — 예: 내 마이크)
audiofilters-trigger-at-db = 트리거 기준 (dB)
audiofilters-duck-by-db = 더킹 정도 (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = 크로마 키
filters-name-color-key = 색상 키
filters-name-luma-key = 루마 키
filters-name-render-delay = 렌더 지연
filters-name-color-correction = 색 보정
filters-name-lut = LUT 적용
filters-name-blur = 블러
filters-name-mask = 이미지 마스크
filters-name-sharpen = 샤픈
filters-name-scroll = 스크롤
filters-name-crop = 자르기
filters-title = 필터 — { $name }
filters-blend-mode = 블렌드 모드
filters-chain-header = 필터 체인 (위쪽이 먼저 실행됨)
filters-add = + 필터 추가
filters-add-menu = 필터 추가
filters-empty = 아직 필터가 없습니다 — 웹캠에 크로마 키를 적용하거나, 캡처를 색 보정하거나, 티커를 스크롤하세요.
filters-enable = { $name } 활성화
filters-run-earlier = 더 먼저 실행
filters-move-up = { $name } 위로
filters-run-later = 더 나중에 실행
filters-move-down = { $name } 아래로
filters-remove-title = 필터 제거
filters-remove = { $name } 제거
filters-key-color-rgb = 키 색상 (임의 색상, RGB 거리)
filters-similarity = 유사도
filters-smoothness = 부드러움
filters-luma-min = 루마 최솟값 (어두운 부분 키 아웃)
filters-luma-max = 루마 최댓값 (밝은 부분 키 아웃)
filters-delay = 지연 (ms — 비디오 전용, 예: 오디오와 동기화; 최대 500)
filters-key-color = 키 색상
filters-spill = 스필
filters-gamma = 감마
filters-brightness = 밝기
filters-contrast = 대비
filters-saturation = 채도
filters-hue-shift = 색조 이동
filters-opacity = 불투명도
filters-cube-file = .cube 파일
filters-amount = 양
filters-radius = 반경
filters-name-shader = 셰이더 (WGSL)
filters-shader-gallery = 갤러리
filters-shader-gallery-pick = 프리셋 불러오기…
filters-shader-gallery-grayscale = 그레이스케일
filters-shader-gallery-invert = 반전
filters-shader-gallery-scanlines = 스캔라인
filters-shader-gallery-vignette = 비네팅
filters-shader-source = 셰이더 소스 (WGSL)
filters-shader-hint = vec4를 반환하는 WGSL effect(uv, color, p, texel, time)를 작성하세요. 슬라이더용으로 // @param name min max default로 매개변수를 주석 처리하세요. 잘못된 셰이더는 무시되며, 컴파일될 때까지 소스는 필터 없이 렌더링됩니다.
filters-name-bezier-mask = 베지어 마스크
filters-mask-editor-hint = 점을 드래그해 이동하고, 더블클릭해 추가하며, 점을 우클릭해 제거합니다.
filters-mask-shape = 모양
filters-mask-shape-pick = 프리셋…
filters-mask-shape-rectangle = 직사각형
filters-mask-shape-diamond = 마름모
filters-mask-shape-hexagon = 육각형
filters-mask-shape-circle = 원
filters-mask-feather = 페더
filters-mask-export-wipe = 와이프로 내보내기…
filters-mask-image = 마스크 이미지
filters-mask-mode = 모드
filters-mask-alpha = 알파
filters-mask-luma = 루마
filters-mask-invert = 반전
filters-speed-x = 속도 X (px/s)
filters-speed-y = 속도 Y (px/s)
filters-crop-left = 왼쪽
filters-crop-top = 위
filters-crop-right = 오른쪽
filters-crop-bottom = 아래
filters-crop-aria = { $side } 자르기


# --- PickerShell.tsx ---
pickershell-refresh-aria = 새로 고침
pickershell-refresh-title = 목록 새로 고침
pickershell-close = 닫기


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = 버그 신고
bugreport-intro = 신고는 익명이며 선택형입니다 — 자동으로 전송되는 것은 없습니다. 아래의 정확한 내용을 검토한 다음 미리 채워진 GitHub 이슈나 이메일 앱으로 제출합니다. 개인 데이터는 없습니다 (홈 경로와 사용자 이름은 가려집니다). 계정도, 서버도 없습니다.
bugreport-crash-notice = Freally Capture가 이전 실행에서 예기치 않게 종료되었습니다 — 익명 크래시 세부 정보가 아래에 포함되어 있습니다. 신고하면 빠르게 고치는 데 도움이 됩니다.
bugreport-description-label = 문제가 발생했을 때 무엇을 하고 있었나요? (선택 사항)
bugreport-description-placeholder = 예: 두 번째 웹캠을 추가했을 때 미리 보기가 멈췄습니다
bugreport-include-crash = 마지막 실행의 익명 크래시 세부 정보 포함
bugreport-preview-label = 전송될 정확한 내용
bugreport-open-github = GitHub 이슈 열기
bugreport-gmail-title = 브라우저에서 Gmail 작성 창을 미리 채워서 엽니다. 로그아웃 상태인가요? Google이 먼저 로그인 화면을 표시합니다.
bugreport-compose-gmail = Gmail에서 작성
bugreport-email-title = 이 PC의 기본 메일 앱(Outlook, Thunderbird, Mail…)에서 초안을 엽니다
bugreport-send-email = 이메일 보내기
bugreport-copied = 복사됨 ✓
bugreport-copy-report = 신고 내용 복사
bugreport-dismiss-crash = 크래시 닫기
bugreport-copy-failed = 복사하지 못했습니다 — 텍스트를 선택해 직접 복사하세요
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = 무슨 일이 있었나
bugreport-preview-no-description = (설명이 제공되지 않음)
bugreport-preview-diagnostics = 익명 진단 정보 (개인 데이터 없음)
bugreport-preview-from = 보낸 곳: Freally Capture
bugreport-preview-crash-excerpt = --- 크래시 발췌 ---


# --- Updates.tsx ---
updates-title = 소프트웨어 업데이트
updates-checking = 업데이트 확인 중…
updates-uptodate = 최신 버전입니다.
updates-check-again = 다시 확인
updates-available = 버전 { $version }을(를) 사용할 수 있습니다
updates-current-version = (현재 { $current })
updates-release-notes-label = 버전 { $version } — 릴리스 노트
updates-confirm = 지금 업데이트하시겠습니까? 다운로드는 적용 전에 번들된 서명 키로 검증됩니다. Freally Capture가 닫히고 설치 프로그램이 실행된 뒤 새 버전이 스스로 다시 열립니다.
updates-yes-update-now = 예, 지금 업데이트
updates-no-not-now = 아니요, 나중에
updates-downloading = { $version } 다운로드 중…
updates-starting = 시작하는 중…
updates-installed = 업데이트가 설치되었습니다.
updates-restart-now = 지금 다시 시작
updates-restart-later = 나중에 다시 시작
updates-try-again = 다시 시도


# --- Models.tsx ---
models-title = 컴포넌트
models-ffmpeg-heading = FFmpeg — 컨테이너 코덱
models-badge-third-party = 서드파티 · 번들 안 됨
models-ffmpeg-desc = Freally Capture의 자체 엔진은 추가 요소 없이 무손실 freally-video(.frec)를 녹화합니다. 플랫폼과 플레이어가 기대하는 컨테이너 형식 — mp4/mkv/mov/webm의 H.264/AAC(및 HEVC/AV1) — 을 녹화하려면 이 앱이 절대 함께 배포하지 않는 별도 도구인 FFmpeg를 사용합니다: 이 코덱들은 특허에 얽혀 있어 선택 사항으로 남고 명확히 표시됩니다. 아래 고정된 빌드에서 온디맨드로 다운로드되며, 첫 사용 전에 SHA-256으로 검증되고, 사용자별로 캐시되며, 별도 프로세스로 구동됩니다. 그 라이선스(LGPL/GPL)는 별도입니다 — THIRD-PARTY-NOTICES를 참고하세요.
models-checking = 확인 중…
models-ffmpeg-not-installed = 설치되지 않음. 사용 가능: { $source }의 FFmpeg { $version } ({ $size } 다운로드).
models-ffmpeg-none-pinned = 이 플랫폼에 고정된 FFmpeg 빌드가 아직 없습니다 — 여기서는 컨테이너 코덱 녹화를 사용할 수 없습니다. 무손실 freally-video 녹화는 영향받지 않습니다.
models-ffmpeg-download-verify = 다운로드 및 검증 ({ $size })
models-downloading = 다운로드 중…
models-download-of = /
models-cancel = 취소
models-ffmpeg-verifying = 고정된 SHA-256으로 다운로드를 검증하는 중…
models-ffmpeg-extracting = 압축 푸는 중…
models-ffmpeg-ready = 설치 및 검증됨 — { $version }
models-remove = 제거
models-ffmpeg-retry = 다운로드 다시 시도
models-network-note = 이 패널에서 다운로드는 유일한 네트워크 작업이며 절대 스스로 시작하지 않습니다. 체크섬이 실패하면 설치가 중단됩니다 — 앱은 보증할 수 없는 바이트를 실행하지 않습니다.
models-cef-heading = 브라우저 소스 런타임 — Chromium (CEF)
models-cef-desc = 브라우저 소스는 Chromium Embedded Framework를 통해 웹 페이지(알림, 위젯, 오버레이)를 렌더링합니다 — 이 앱이 절대 함께 배포하지 않는 ~100 MB 런타임입니다. 공식 CEF 빌드 인덱스에서 온디맨드로 다운로드되고, 압축을 풀기 전에 해당 인덱스의 SHA-1로 검증되며, 사용자별로 캐시됩니다. 이를 통해 렌더링되는 브라우저 소스는 자체 마일스톤으로 제공되며, 여기서는 그에 필요한 런타임을 설치합니다.
models-cef-download-install = 다운로드 및 설치
models-cef-unsupported = CEF는 이 플랫폼용 빌드를 게시하지 않습니다 — 여기서는 브라우저 소스를 사용할 수 없습니다.
models-cef-resolving = 최신 안정 빌드를 확인하는 중…
models-cef-verifying = 인덱스 SHA-1로 다운로드를 검증하는 중…
models-cef-extracting = 런타임 압축 푸는 중…
models-cef-ready = 설치됨 — CEF { $version }.
models-cef-retry = 다시 시도
models-integrations-heading = 선택적 통합
models-badge-never-bundled = 절대 번들 안 됨
models-ndi-detected = 감지됨
models-ndi-not-installed = 설치되지 않음
models-vst-available = 사용 가능
models-vst-not-available = 사용 불가


# --- Recordings.tsx ---
recordings-title = 녹화 파일
recordings-loading = 폴더를 읽는 중…
recordings-empty = 아직 녹화 파일이 없습니다 — 녹화 시작은 출력에서 설정한 폴더에 기록합니다.
recordings-frec-label = 자체 무손실 (freally-video)
recordings-remux-title = mp4로 다시 포장 — 스트림 복사, 재인코딩 없음, 품질 변화 없음 (FFmpeg 컴포넌트 필요)
recordings-trim = 다듬기
recordings-trim-title = 이 녹화에서 클립을 잘라냅니다 — 키프레임에 맞춘 컷은 재인코딩 없이 내보냅니다
recordings-verify = 검증
recordings-verify-title = 파일 무결성 확인 — 컨테이너 구조, 연속성, A/V 인터리브, 길이
recordings-verifying = 검증 중…
verify-dismiss = 닫기
verify-verdict-pass = { $name } — 무결성 정상
verify-verdict-warn = { $name } — 경고와 함께 검증됨
verify-verdict-fail = { $name } — 문제 발견
verify-container = 컨테이너
verify-video-continuity = 비디오 연속성
verify-audio-continuity = 오디오 연속성
verify-av-interleave = A/V 인터리브
verify-duration = 길이
recordings-alpha-label = 알파
recordings-prores-title = 알파를 보존하는 ProRes 4444 .mov 마스터 내보내기(편집용)
recordings-qtrle-title = 알파를 보존하는 QuickTime Animation .mov 내보내기(최대 호환)
trim-title = 다듬기 — { $name }
trim-loading = 파일을 읽는 중…
trim-preview-alt = 미리보기 프레임
trim-position = 재생 위치
trim-step-second-back = 1초 뒤로
trim-step-frame-back = 1프레임 뒤로
trim-step-frame-forward = 1프레임 앞으로
trim-step-second-forward = 1초 앞으로
trim-snap = 키프레임
trim-snap-title = 가장 가까운 키프레임에 스냅 — 그 지점의 컷은 재인코딩 없이 내보냅니다
trim-set-in = 시작점 설정
trim-set-out = 끝점 설정
trim-range-invalid = 끝점은 시작점 뒤에 있어야 합니다.
trim-copy-badge = ✓ 재인코딩 없이 내보냅니다 — 시작점이 키프레임 위에 있습니다.
trim-reencode-badge = 재인코딩됩니다: 시작점이 키프레임 사이에 있습니다("키프레임"으로 스냅하면 무손실 컷).
trim-export = 클립 내보내기
trim-export-916 = 9:16
trim-export-916-title = 세로형으로 리프레임 내보내기(세로 캔버스 크기로 중앙 크롭) — 항상 재인코딩합니다
recordings-remuxing = 리먹싱 중…
recordings-remux-to-mp4 = MP4로 리먹스
recordings-export-mp4-title = 자체 .frec를 디코딩하고 MP4(H.264/AAC)로 재인코딩하여 어떤 플레이어에서도 재생되게 합니다 — FFmpeg 컴포넌트 필요
recordings-exporting = 내보내는 중…
recordings-export-mp4 = MP4로 내보내기
recordings-export-mkv-title = 자체 .frec를 디코딩하고 MKV로 재인코딩하여 어떤 플레이어에서도 재생되게 합니다
recordings-starting = 시작하는 중…
recordings-frames = { $done } / { $total } 프레임
recordings-cancel = 취소
recordings-export-cancelled = 내보내기가 취소되었습니다.
recordings-exported-to = { $path }(으)로 내보냄
recordings-remuxed-to = { $path }(으)로 리먹스함
recordings-normalize = 정규화
recordings-normalizing = 정규화 중…
recordings-normalize-title = 라우드니스를 목표로 정규화 (사본을 씁니다)
recordings-normalized-to = { $path }(으)로 정규화함

# --- Audio-only recording (CAP-N38) ---
audiorec-title = 오디오 전용
audiorec-format = 오디오 녹음 형식
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = 오디오 녹음
audiorec-stop = 중지
audiorec-pause = 일시정지
audiorec-resume = 재개
audiorec-recording = REC { $sec }초
audiorec-saved = 트랙 파일 { $count }개 저장됨


# --- OpenedFrec.tsx ---
openfrec-title = .frec 녹화 파일 열기
openfrec-desc = Freally Capture는 자체 무손실 .frec 형식으로 녹화합니다 — 재생하지는 않습니다. Freally Player가 출시되면 .frec를 직접 재생합니다. 지금은 MP4/MKV로 내보내면 어떤 플레이어(VLC, OS 플레이어 등)에서도 재생됩니다.
openfrec-exported-to = { $path }(으)로 내보냄
openfrec-exporting = 내보내는 중…
openfrec-starting = 시작하는 중…
openfrec-export-mp4 = MP4로 내보내기
openfrec-export-mkv = MKV로 내보내기


# --- VerticalCanvasDialog.tsx ---
vertical-title = 세로 캔버스 (9:16)
vertical-enable = 두 번째 캔버스 활성화 — 프로그램과 독립적으로 녹화하고 방송 가능
vertical-scene-label = 이 캔버스가 구성하는 장면
vertical-width = 너비
vertical-height = 높이
vertical-preview-alt = 세로 캔버스 미리 보기
vertical-note = 항목 위치는 캔버스 간에 픽셀 단위로 정확합니다: 장면 레일에서 이 장면을 선택하면 이 미리 보기가 세로 결과를 보여주는 동안 배치할 수 있습니다. 스트림 대상은 ⦿ 스트림…에서 이 캔버스를 선택합니다. 설정 → 출력에서 메인 파일과 함께 녹화할 수 있습니다.
vertical-close = 닫기


# --- EulaGate.tsx ---
eula-title = Freally Capture — 라이선스 계약
eula-version = v{ $version }
eula-intro = Freally Capture를 사용하려면 이 계약을 읽고 동의하세요. 요컨대: 이것은 중립적인 도구이며, 당신이 캡처, 녹화, 방송하는 것과 그에 대한 권리 보유는 전적으로 당신의 책임입니다.
eula-thanks = 읽어주셔서 감사합니다.
eula-scroll-hint = 계속하려면 끝까지 스크롤하세요.
eula-decline = 거부하고 종료
eula-agree = 동의합니다


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = 출력
output-loading = 설정을 아직 불러오는 중…
output-container-frec = freally-video (.frec) — 무손실, 자체, 내려받을 것 없음
output-container-mkv = MKV — 크래시에 강함; 나중에 mp4로 리먹스
output-container-mp4 = MP4 — 어디서나 재생됨
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = 무손실
output-preset-lossless-title = 자체 freally-video 코덱 — 비트 정확, 다운로드 없음
output-preset-high-label = 고품질
output-preset-high-title = MP4, 최적 감지 인코더, 준무손실 CQ 16, 품질 프리셋
output-preset-balanced-label = 균형
output-preset-balanced-title = MKV, 최적 감지 인코더, CQ 23, 균형 프리셋
output-recording-format = 녹화 형식
output-ffmpeg-warning = 이 형식은 FFmpeg 컴포넌트(컨테이너 코덱 — 번들 안 됨)가 필요합니다. 무손실 .frec는 아무것도 필요 없습니다.
output-install = 설치…
output-recordings-folder = 녹화 폴더
output-folder-placeholder = OS 비디오 폴더
output-filename-prefix = 파일 이름 접두사
output-recording-template = 녹화 파일 이름
output-replay-template = 리플레이 파일 이름
output-still-template = 스틸 파일 이름
output-template-tokens = 토큰: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = 리플레이 폴더
output-still-folder = 스틸 폴더
output-same-folder-placeholder = 녹화 폴더
output-frame-rate = 프레임 레이트
output-fps-option = { $fps } fps
output-split-every = 분할 간격 (분, 0 = 끔)
output-output-width = 출력 너비 (0 = 캔버스; 컨테이너 형식만)
output-output-height = 출력 높이 (0 = 캔버스)
output-record-vertical = 세로 캔버스도 녹화 (별도의 "… (vertical)" 파일; 9:16 캔버스 활성화 필요)
output-audio-tracks = 오디오 트랙
output-recorded-tracks-group = 녹화 트랙
output-track-last-one = 최소 한 개의 트랙은 녹화해야 합니다
output-record-track-on = 트랙 { $index } 녹화: 켜짐
output-record-track-off = 트랙 { $index } 녹화: 꺼짐
output-encoder-heading = 인코더
output-video-encoder = 비디오 인코더
output-encoder-auto = 자동 — 최적 감지 (H.264)
output-encoder-unavailable = — 여기서는 사용 불가
output-preset = 프리셋
output-preset-quality = 품질
output-preset-balanced-option = 균형
output-preset-performance = 성능
output-rate-control = 레이트 컨트롤
output-rc-cqp = CQP (고정 품질)
output-rc-cbr = CBR (고정 비트레이트)
output-rc-vbr = VBR (가변 비트레이트)
output-cq = CQ (0–51, 낮을수록 좋음)
output-bitrate = 비트레이트 (kbps)
output-keyframe = 키프레임 간격 (s)
output-audio-bitrate = 오디오 비트레이트 (kbps / 트랙)
output-iso-heading = ISO 녹화
output-iso-explainer = 선택한 소스를 각각 프로그램과 나란히 자체 파일로 깨끗하게 녹화합니다 — 합성 전, 캔버스 크기와 프레임 레이트로 기록되어 모든 파일이 편집 타임라인에 정렬되어 들어갑니다. 미드레인지 GPU에서는 2개 레인이 무난하며, 레인이 하나 늘 때마다 렌더링과 인코딩이 한 번씩 추가됩니다.
output-iso-none = 아직 컬렉션에 소스가 없습니다.
output-iso-source-on = "{ $name }"을(를) 자체 ISO 파일로 녹화 중 — 클릭하여 중지
output-iso-source-off = "{ $name }"을(를) 자체 ISO 파일로 녹화
output-iso-post-filter = 소스의 필터를 적용해 녹화(포스트 필터); 선택 해제 시 원본 소스를 녹화합니다
output-iso-format = ISO 형식
output-iso-encoder = ISO 비디오 인코더
output-alpha-frec = 투명도(알파)와 함께 녹화 — 투명 배경 위의 프로그램
output-alpha-title = 레코더는 자체 투명 렌더를 받습니다. 미리보기와 스트림은 그대로입니다. 녹화 목록에서 ProRes 4444 또는 QTRLE로 내보내세요 — MP4/MKV는 알파를 평탄화합니다.
output-split-events = 다음 시점에도 새 파일 시작… (각 파트는 정확히 이벤트에서 시작; 최소 길이 1초)
output-split-on-scene = 장면 전환
output-split-on-marker = 마커
output-split-on-rundown = 런다운 단계
output-auto-markers = 스튜디오 이벤트에서 자동으로 챕터 마커 추가(장면 전환, 리플레이 저장, 재연결, 프레임 드롭, 알람, 규칙)
output-auto-markers-title = 유형별 마커는 녹화의 챕터(mkv) 또는 .chapters.txt 사이드카에 수동 마커 단축키와 나란히 기록됩니다
output-pipeline-heading = 녹화 후 파이프라인
output-pipeline-explainer = 녹화가 완료되면 이 단계들을 메인 파일에 순서대로 백그라운드에서 실행합니다. 닫힌 액션 집합입니다 — "명령 실행" 단계는 의도적으로 없습니다. 체인은 첫 실패에서 멈춥니다.
output-pipeline-enabled = 녹화가 끝날 때마다 파이프라인 실행
output-pipeline-add = 단계 추가…
output-pipeline-up = 위로
output-pipeline-down = 아래로
output-pipeline-remove = 단계 제거
output-pipeline-template = 이름 바꾸기 템플릿(CAP-M25 토큰)
output-pipeline-folder = 폴더
pipeline-queue = 녹화 후 파이프라인
pipeline-verify = 검증
pipeline-remux = MP4로 리먹스
pipeline-normalize = 라우드니스 정규화
pipeline-rename = 이름 바꾸기
pipeline-move = 폴더로 이동
pipeline-copy = 폴더로 복사
pipeline-reveal = 파일 관리자에서 보기
pipeline-luaEvent = Lua 스크립트에 알림
output-presets = 프리셋:

# --- SettingsStream.tsx ---
stream-title = 설정 — 스트림
stream-target-enabled = 대상 { $index } 활성화됨
stream-target = 대상 { $index }
stream-remove = 제거
stream-service = 서비스
stream-canvas = 캔버스
stream-canvas-main = 메인 (프로그램)
stream-canvas-vertical = 세로 (9:16 — 스튜디오에서 활성화)
stream-ingest-srt = SRT 인제스트 URL
stream-ingest-whip = WHIP 엔드포인트 URL
stream-ingest-url = 인제스트 URL
stream-ingest-override = (재정의 — 비우면 서비스 프리셋)
stream-key-srt = streamid (선택 사항 — ?streamid=…로 추가됨; 비밀로 취급)
stream-key-whip = Bearer 토큰 (선택 사항 — Authorization 헤더로 전송; 비밀)
stream-key-custom = 스트림 키 (서버에서 발급 — 비밀로 취급)
stream-key-service = 스트림 키 (크리에이터 대시보드에서 발급 — 비밀로 취급)
stream-key-aria = 스트림 키 { $index }
stream-key-hide = 숨기기
stream-key-show = 표시
stream-encoder = 인코더 (H.264 — RTMP, SRT, WHIP 모두가 전달하는 것)
stream-encoder-auto = 자동 — 최적 감지된 H.264 인코더
stream-encoder-unavailable = (여기서는 사용 불가)
stream-video-bitrate = 비디오 비트레이트 (kbps, CBR)
stream-audio-bitrate = 오디오 비트레이트 (kbps)
stream-fps = FPS
stream-keyframe = 키프레임 간격 (s)
stream-audio-track = 오디오 트랙 (1–6)
stream-output-width = 출력 너비 (0 = 캔버스)
stream-output-height = 출력 높이 (0 = 캔버스)
stream-add-target = + 대상 추가
stream-go-live-note = 라이브 시작은 활성화된 모든 대상에 동시에, 각 플랫폼으로 직접 송출합니다. 인코더 설정이 동일한 대상은 하나의 인코딩을 공유합니다.
stream-auto-record = 라이브를 시작할 때 녹화도 시작 (녹화는 여전히 독립적으로 중지됨)
stream-ffmpeg-note-before = 방송 컨테이너 코덱은 표시된 온디맨드 ffmpeg 컴포넌트를 통해 실행됩니다 —
stream-ffmpeg-note-link = 여기서 관리
stream-ffmpeg-note-after = . 로컬 녹화는 방송 상태와 관계없이 계속 실행됩니다.
stream-cancel = 취소
stream-save = 저장

# --- SettingsReplay.tsx ---
replay-title = 설정 — 리플레이 버퍼
replay-length-15s = 15초
replay-length-30s = 30초
replay-length-1min = 1분
replay-length-2min = 2분
replay-length-5min = 5분
replay-quality-low = 낮음 (3 Mbps)
replay-quality-standard = 표준 (6 Mbps)
replay-quality-high = 높음 (12 Mbps)
replay-length-presets = 길이 프리셋
replay-quality-presets = 품질 프리셋
replay-length-seconds = 길이 (초)
replay-video-bitrate = 비디오 비트레이트 (kbps)
replay-fps = FPS
replay-audio-track = 오디오 트랙 (1–6)
replay-note = 활성화되어 있는 동안 버퍼는 자체 경량 인코딩을 디스크상의 제한된 링에 실행합니다 — 이 설정에서는 약 { $mb } MB입니다. 저장은 재인코딩 없이 링을 이어 붙이며 방송이나 녹화에는 영향을 주지 않습니다. 변경 사항은 다음에 활성화할 때 적용됩니다.
replay-cancel = 취소
replay-save = 저장

# --- SettingsRemote.tsx ---
remote-title = 설정 — 원격 제어
remote-enable = WebSocket 원격 API 활성화
remote-password = 비밀번호 (필수 — 컨트롤러가 이것으로 인증합니다)
remote-password-placeholder = 컨트롤러용 비밀번호
remote-password-hide = 숨기기
remote-password-show = 표시
remote-port = 포트
remote-allow-lan = LAN 연결 허용 (기본값은 이 컴퓨터만)
remote-note = 꺼짐 = 포트가 닫힙니다. 켜짐 = 127.0.0.1(또는 허용 시 LAN)에서 비밀번호로 보호되는 WebSocket으로, 장면 전환, 전환 실행, 방송·녹화 시작/중지, 리플레이 저장, 음소거/볼륨 설정을 할 수 있습니다 — UI와 동일한 작업이며 그 이상은 없습니다. 파일을 읽을 수 없습니다. 비밀번호를 자격 증명처럼 취급하세요. 다른 기기에서 특별히 제어하는 경우가 아니면 이 컴퓨터만 사용을 권장합니다.
remote-password-required = 원격 API를 활성화하려면 비밀번호가 필요합니다.
remote-cancel = 취소
remote-save = 저장

# --- SettingsHotkeys.tsx ---
hotkeys-title = 설정 — 단축키
hotkeys-record = 녹화 시작 / 중지
hotkeys-go-live = 라이브 시작 / 방송 종료
hotkeys-transition = 스튜디오 모드 전환
hotkeys-save-replay = 리플레이 저장 (최근 N초)
hotkeys-add-marker = 챕터 마커 남기기 (녹화)
hotkeys-note = 단축키는 전역입니다 — 다른 앱이 활성화된 동안에도 작동합니다. 비우면 = 미지정. 믹서 푸시투토크/뮤트 키는 각 스트립의 ⋯ 메뉴에 있습니다. Linux/Wayland에서는 전역 단축키를 사용할 수 없을 수 있습니다(컴포지터 한계) — 버튼은 계속 작동합니다.
hotkeys-cancel = 취소
hotkeys-save = 저장

# --- WorkspaceDialog.tsx ---
workspace-title = 프로필 및 장면 모음
workspace-profiles = 프로필
workspace-profiles-hint = 프로필은 당신의 설정입니다 — 스트림 대상, 출력, 단축키. 방송별 또는 플랫폼별로 전환하세요.
workspace-collections = 장면 모음
workspace-collections-hint = 모음은 당신의 장면 + 소스입니다. 만들기는 현재 모음을 시작점으로 복제합니다.
workspace-active = 활성
workspace-switch-to = { $name }(으)로 전환
workspace-active-marker = ● 활성
workspace-new-name-placeholder = 새 이름…
workspace-new-name-label = 새 { $title } 이름
workspace-create = 만들기

# --- OBS import (CAP-M02) ---
workspace-import-obs = OBS에서 가져오기…
workspace-import-obs-hint = OBS 장면 모음(해당 scenes.json)을 가져옵니다. 현재 모음은 먼저 저장됩니다.
workspace-import-busy = 가져오는 중…
workspace-import-title = "{ $name }" 가져옴
workspace-import-summary = 장면 { $scenes }개 · 소스 { $sources }개 · 항목 { $items }개
workspace-import-dismiss = 닫기
workspace-import-clean = 모두 문제없이 가져왔습니다.
workspace-import-geometry-caveat = 크기와 위치는 OBS 레이아웃에서 맞춰집니다 — 각 장면을 확인하고 캡처 장치를 다시 선택하세요.
workspace-import-notes-title = 참고 사항과 함께 가져옴
workspace-import-skipped-title = 가져오지 않음
import-note-needsReselect = 장치/모니터/창 다시 선택
import-note-gameCaptureAsWindow = 게임 캡처 → 창 캡처
import-note-referencesFile = 파일 경로 확인
import-note-filterDropped = 일부 필터 미지원
import-note-geometryApproximated = 위치/크기 근사
import-skip-unsupportedKind = 동등한 소스 유형 없음
import-skip-group = 그룹은 아직 지원되지 않음

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = 누락된 파일 다시 연결…
doctor-title = 누락된 파일
doctor-scanning = 검사 중…
doctor-all-good = 참조된 모든 파일이 있습니다. 다시 연결할 것이 없습니다.
doctor-intro = 참조된 파일 { $count }개를 이 컴퓨터에서 찾을 수 없습니다. 각각의 새 위치를 지정하세요 — 이를 사용하는 모든 장면이 한 번에 고쳐집니다.
doctor-relinked = 참조 { $count }개를 다시 연결했습니다.
doctor-uses = { $count }회 사용
doctor-locate = 찾기…
doctor-locate-folder = 폴더에서 찾기…
doctor-locate-folder-hint = 폴더를 선택하면 각 누락 파일을 이름으로 찾아 다시 연결합니다.
doctor-kind-image = 이미지
doctor-kind-media = 미디어
doctor-kind-slideshow = 슬라이드쇼
doctor-kind-font = 글꼴
doctor-kind-lut = LUT
doctor-kind-mask = 마스크
history-relinkFiles = 파일 다시 연결

# --- ScriptsDialog.tsx ---
scripts-title = 스크립트 (Lua)
scripts-empty = 아직 스크립트가 없습니다 — .lua 파일을 추가하세요. API는 scripts/sample.lua를 참고하세요: 라이브/장면/녹화 이벤트에 반응하고 원격 API와 동일한 명령을 실행합니다.
scripts-enable = { $path } 활성화
scripts-remove = { $path } 제거
scripts-path-label = 스크립트 경로
scripts-add = 추가
scripts-note = 스크립트는 샌드박스에서 실행됩니다 — 파일이나 OS 접근 불가; 원격 API와 동일한 스튜디오 명령(장면 전환, 전환, 녹화/방송/리플레이, 음소거)만 호출할 수 있습니다. 스크립트 오류는 기록되고 격리됩니다. 변경 사항은 1초 이내에 적용됩니다.
scripts-error-not-lua = .lua 파일을 지정하세요.

# --- BrowserDock.tsx ---
browser-dock-title = 브라우저 독
browser-dock-empty = 아직 독이 없습니다 — 채팅 팝아웃, 알림 페이지, 또는 Companion 웹 버튼을 추가하세요.
browser-dock-open = 열기
browser-dock-remove = { $name } 제거
browser-dock-name-placeholder = 이름 (예: Twitch Chat)
browser-dock-name-label = 독 이름
browser-dock-url-label = 독 URL
browser-dock-note = 독은 스튜디오 옆에 배치할 수 있는 자체 창으로 열립니다. 페이지는 앱에 접근할 수 없습니다 — 그저 렌더링될 뿐입니다. http(s) URL만 가능하며, 독은 열기를 클릭할 때만 열립니다.
browser-dock-error-name = 독의 이름을 지정하세요 (예: Twitch Chat).
browser-dock-error-url = 독 URL은 http:// 또는 https://로 시작해야 합니다.

# --- studio-preview-pane ---
studio-preview-label = 스튜디오 모드 미리 보기
studio-preview-heading = 미리 보기
studio-preview-hint = 장면을 클릭하면 여기에 로드됩니다
studio-preview-empty = 미리 보기가 여기에 표시됩니다.
studio-preview-mirrors = 프로그램 미러링
studio-preview-transition-select = 전환
studio-preview-duration = 전환 지속 시간 (ms)
studio-preview-commit-title = 전환을 통해 미리 보기 → 프로그램으로 반영합니다 (시청자에게 보입니다)
studio-preview-transitioning = 전환 중…
studio-preview-transition-button = 전환 ⇄
studio-preview-luma-placeholder = 그레이스케일 와이프 이미지 (png/jpg)
studio-preview-luma-label = 루마 와이프 이미지
studio-preview-browse = 찾아보기…
studio-preview-filter-images = 이미지
studio-preview-filter-video = 동영상
studio-preview-stinger-placeholder = 스팅어 비디오 (ProRes 4444 .mov는 알파를 유지합니다)
studio-preview-stinger-label = 스팅어 비디오 파일
studio-preview-stinger-cut-label = 스팅어 컷 지점 (ms)
studio-preview-stinger-cut-title = 스팅어 뒤에서 장면 교체가 이루어지는 시점 (전환 시작 후 ms)
studio-preview-stinger-matte-label = 트랙 매트
studio-preview-stinger-matte-title = 트랙 매트 스팅어가 투명도를 담는 방식: 채움과 그 매트를 좌우로 나란히(가로) 또는 위아래로 쌓아(세로) 배치
studio-preview-stinger-duck-label = 프로그램 더킹
studio-preview-stinger-duck-title = 재생하는 동안 스팅어 자체 오디오 아래로 프로그램 오디오를 더킹 (0 = 끔)
studio-preview-stinger-duck-unit = dB

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = 컷
transition-kind-fade = 페이드
transition-kind-slide-left = 슬라이드 ←
transition-kind-slide-right = 슬라이드 →
transition-kind-slide-up = 슬라이드 ↑
transition-kind-slide-down = 슬라이드 ↓
transition-kind-swipe-left = 스와이프 ←
transition-kind-swipe-right = 스와이프 →
transition-kind-luma-linear = 루마 와이프 (선형)
transition-kind-luma-radial = 루마 와이프 (방사형)
transition-kind-luma-horizontal = 루마 와이프 (수평)
transition-kind-luma-diamond = 루마 와이프 (다이아몬드)
transition-kind-luma-clock = 루마 와이프 (시계)
transition-kind-image = 이미지 와이프 (사용자 지정)
transition-kind-stinger = 스팅어 (비디오)
transition-kind-move = 이동 (모프)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = 없음
stinger-matte-horizontal = 좌우 나란히
stinger-matte-vertical = 위아래로 쌓기

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = 사용자 지정 (RTMP/RTMPS)
stream-service-srt = SRT (자체 호스팅)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = 정보
about-tagline = 스튜디오처럼 녹화하고 방송하세요 — 계정도, 클라우드도 없습니다.
about-version = 버전
about-created-by = 만든 사람
about-project-started = 프로젝트 시작
about-first-stable = 첫 정식 릴리스
about-first-stable-pending = 아직 아님 — 1.0.0 진행 중
about-platform = 플랫폼
about-local-first = Freally Capture는 전적으로 당신의 컴퓨터에서 실행됩니다. 계정도, 텔레메트리도, 클라우드도 없습니다 — 컴퓨터를 떠나는 것은 당신이 보내기로 선택한 방송뿐입니다.
about-website = 웹사이트
about-issues = 문제 신고
about-license = 라이선스
about-eula = EULA
about-third-party = 서드파티 고지
about-check-updates = 업데이트 확인…

# --- unified settings modal (TASK-906) ---
settings-title = 설정
settings-language-section = 언어
settings-language = 인터페이스 언어
settings-language-system = 시스템 기본값
settings-language-note = 여기서 선택한 언어는 기억됩니다. "시스템 기본값"은 운영 체제를 따릅니다. 번역되지 않은 텍스트는 영어로 대체됩니다.
settings-appearance-section = 모양
settings-theme = 테마
settings-theme-dark = 어둡게
settings-theme-light = 밝게
settings-theme-custom = 사용자 지정
settings-accent = 강조 색
settings-general-section = 일반
settings-show-stats-dock = 통계 독 표시
settings-open-about = 정보…

# --- command palette (TASK-904) ---
palette-title = 명령 팔레트
palette-search = 장면, 소스, 작업 검색
palette-placeholder = 장면, 소스, 작업 검색…
palette-no-results = “{ $query }”와 일치하는 항목이 없습니다
palette-hint = ↑ ↓ 이동 · Enter 실행 · Esc 닫기
palette-group-scenes = 장면
palette-group-sources = 소스
palette-group-actions = 작업
palette-transition = 미리 보기 → 프로그램 전환
palette-save-replay = 리플레이 저장
palette-add-marker = 챕터 마커 남기기
palette-vertical-canvas = 세로 (9:16) 캔버스…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Freally Capture에 오신 것을 환영합니다
wizard-welcome = 간단한 두 단계면 됩니다. 먼저 이 컴퓨터가 무엇을 할 수 있는지 확인한 다음 장면을 시작합니다. 30초 남짓이면 되고, 나중에 무엇이든 바꿀 수 있습니다.
wizard-local-first = 여기서 당신의 컴퓨터를 떠나는 것은 아무것도 없습니다. Freally Capture에는 계정도, 텔레메트리도, 클라우드도 없습니다.
wizard-start = 시작하기
wizard-skip = 건너뛰기
wizard-hardware-title = 이 컴퓨터가 할 수 있는 것
wizard-probing = 그래픽 카드와 프로세서를 확인하는 중…
wizard-encoder = 인코더
wizard-canvas = 캔버스
wizard-bitrate = 비트레이트
wizard-probe-found = 발견됨: { $gpus } · 물리 코어 { $cores }개
wizard-no-gpu = 전용 GPU 없음
wizard-apply = 이 설정 사용하기
wizard-keep-current = 지금 설정 그대로 두기
wizard-template-title = 장면으로 시작하기
wizard-template-screen = 내 화면 캡처하기
wizard-template-screen-note = 주 모니터의 디스플레이 캡처를 추가합니다. 가장 많이 선택하는 시작 방법입니다.
wizard-template-empty = 빈 상태로 시작하기
wizard-template-empty-note = 비어 있는 장면입니다. + 버튼으로 소스를 직접 추가하세요.
wizard-done = 준비가 끝났습니다.
wizard-done-hint = 언제든지 Ctrl+K를 누르면 장면, 소스, 작업을 검색할 수 있습니다. 설정은 ⚙ 버튼 뒤에 있습니다.
wizard-close = 방송 시작하기

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = 그래픽 카드가 스스로 영상을 인코딩할 수 있어서, 프로세서는 스튜디오의 나머지 작업에 온전히 쓸 수 있습니다.
autoconfig-reason-software = 쓸 만한 하드웨어 인코더를 찾지 못해 프로세서가 인코딩하며, 잘 작동하지만 CPU를 더 많이 씁니다.
autoconfig-reason-quality-hardware = 모든 주요 플랫폼이 받아들이는 비트레이트로 1080p, 초당 60프레임입니다.
autoconfig-reason-quality-software = 대부분의 프로세서에서는 소프트웨어 인코딩으로 60프레임을 처리하면 프레임이 끊기기 때문에, 초당 30프레임입니다.
autoconfig-reason-quality-low-cores = 이 프로세서는 코어가 적어 소프트웨어 인코딩이 컴포지터와 코어를 두고 다투기 때문에, 비트레이트를 낮췄습니다.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = 녹화가 시작되었습니다
announce-recording-paused = 녹화가 일시정지되었습니다
announce-recording-stopped = 녹화가 중지되었습니다
announce-live-started = 지금 생방송 중입니다
announce-live-ended = 방송이 종료되었습니다
announce-reconnecting = 연결이 끊겨 다시 연결하는 중입니다
announce-stream-failed = 방송에 실패했습니다
announce-frames-dropped = 프레임 { $count }개가 누락되었습니다

# CAP-M01 — undo/redo edit history
palette-undo = 실행 취소
palette-redo = 다시 실행
palette-edit-history = 편집 기록…
history-title = 편집 기록
history-empty = 취소할 편집이 아직 없습니다.
history-current = 현재 상태
history-close = 닫기
history-addScene = 장면 추가
history-renameScene = 장면 이름 변경
history-removeScene = 장면 제거
history-reorderScene = 장면 순서 변경
history-addSource = 소스 추가
history-removeSource = 소스 제거
history-reorderSource = 소스 순서 변경
history-renameSource = 소스 이름 변경
history-transformSource = 소스 이동
history-toggleVisibility = 표시 전환
history-toggleLock = 잠금 전환
history-setBlendMode = 혼합 모드 변경
history-editSourceProperties = 속성 편집
history-applyLayout = 레이아웃 배치
history-moveToSeat = 자리로 이동
history-groupSources = 소스 그룹화
history-ungroupSources = 그룹 해제
history-toggleGroupVisibility = 그룹 전환
history-setSceneAudio = 장면 오디오
history-setVerticalCanvas = 세로 캔버스
history-addFilter = 필터 추가
history-removeFilter = 필터 제거
history-reorderFilter = 필터 순서 변경
history-editFilter = 필터 편집
history-toggleFilter = 필터 전환
history-setVolume = 볼륨 조정
history-toggleMute = 음소거 전환
history-setMonitor = 모니터링 변경
history-setTracks = 트랙 변경
history-setSyncOffset = A/V 동기화 조정
history-setAudioHotkeys = 오디오 단축키

# CAP-M04 — alignment aids
settings-alignment-section = 정렬 보조
settings-smart-guides = 스마트 가이드(드래그 시 스냅)
settings-safe-areas = 안전 영역 오버레이
settings-rulers = 눈금자
align-group = 캔버스에 정렬
align-left = 왼쪽 정렬
align-hcenter = 가로 가운데 정렬
align-right = 오른쪽 정렬
align-top = 위쪽 정렬
align-vcenter = 세로 가운데 정렬
align-bottom = 아래쪽 정렬

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = 선택 항목 정렬 및 분포
arrange-left = 왼쪽 가장자리 정렬
arrange-hcenter = 가로 가운데 정렬
arrange-right = 오른쪽 가장자리 정렬
arrange-top = 위쪽 가장자리 정렬
arrange-vcenter = 세로 가운데 정렬
arrange-bottom = 아래쪽 가장자리 정렬
distribute-h = 가로로 분포
distribute-v = 세로로 분포
guides-group = 안내선
guides-add-v = 세로 안내선 추가
guides-add-h = 가로 안내선 추가
guides-clear = 모든 안내선 지우기
history-arrangeItems = 항목 정렬
history-editGuides = 안내선 편집

# CAP-M05 — edit transform + copy/paste
transform-title = 변형 편집 — { $name }
transform-anchor = 기준점
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = 회전
transform-crop = 자르기
transform-crop-left = 왼쪽
transform-crop-top = 위
transform-crop-right = 오른쪽
transform-crop-bottom = 아래
transform-no-size = 크기와 자르기는 소스가 크기를 보고하면 사용할 수 있습니다.
transform-copy = 변형 복사
transform-paste = 변형 붙여넣기
transform-close = 닫기
filters-copy = 필터 복사 ({ $count })
filters-paste = 필터 붙여넣기 ({ $count })
palette-edit-transform = 변형 편집…
history-pasteFilters = 필터 붙여넣기

# CAP-M26 — keying workbench
workbench-title = 키잉 작업대 — { $name }
workbench-mode-keyed = 키 적용
workbench-mode-source = 소스
workbench-mode-matte = 매트
workbench-mode-split = 분할
workbench-eyedropper = 스포이트
workbench-eyedropper-hint = 소스를 클릭하여 키 색상을 추출합니다.
workbench-loupe = 루페
workbench-split = 분할
workbench-preview-alt = 키잉 작업대 미리보기
workbench-tune = 조정
workbench-close = 닫기

# CAP-M06 — multiview monitor
multiview-title = 멀티뷰
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = 장면을 클릭하여 전환합니다.
multiview-hint-stage = 장면을 클릭하여 미리보기에 준비합니다.
palette-multiview = 멀티뷰 모니터

# CAP-M07 — projectors
projector-title = 프로젝터 열기
projector-source = 소스
projector-target-program = 프로그램
projector-target-preview = 미리보기
projector-target-scene = 장면…
projector-target-source = 소스…
projector-target-multiview = 멀티뷰
projector-which-scene = 어느 장면
projector-which-source = 어느 소스
projector-none = 표시할 항목 없음
projector-display = 디스플레이
projector-windowed = 떠 있는 창(이 화면)
projector-display-option = 디스플레이 { $n } — { $w }×{ $h }
projector-primary = (기본)
projector-open = 열기
projector-cancel = 취소
projector-exit-hint = 종료하려면 Esc를 누르세요
palette-projector = 프로젝터 열기…

# CAP-M08 — still-frame grab
palette-still = 스틸 프레임 캡처…
still-saved-toast = 스틸 저장됨: { $name }
still-failed-toast = 스틸 캡처 실패: { $error }
hotkeys-still = 스틸 캡처

# CAP-M13 — source health dashboard
palette-source-health = 소스 상태…
palette-av-sync = A/V 동기화 보정…
palette-hotkey-audit = 단축키 지도…
health-title = 소스 상태
health-col-source = 소스
health-col-state = 상태
health-col-resolution = 해상도
health-col-fps = FPS
health-col-last-frame = 마지막 프레임
health-col-dropped = 드롭됨
health-col-retries = 재시작 횟수
health-col-actions = 동작
health-state-live = 라이브
health-state-waiting = 대기 중
health-state-error = 오류
health-state-inactive = 비활성
health-restart = 재시작
health-properties = 속성
health-empty = 이 컬렉션에는 아직 소스가 없습니다.
health-seconds = { $value }초

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture를 종료할까요?
quit-body = 지금 종료하면 다음 작업이 순서대로 안전하게 수행됩니다:
quit-consequence-stream = 라이브 스트림을 종료하고 서비스 연결을 해제합니다.
quit-consequence-recording = 녹화를 중지하고 파일을 마무리합니다.
quit-consequence-replay = 리플레이 버퍼를 종료합니다 — 저장하지 않은 리플레이 영상은 삭제됩니다.
quit-confirm = 안전하게 종료
quit-quitting = 종료 중…
quit-cancel = 취소

# CAP-M11 — crash-safe recording salvage
salvage-title = 중단된 녹화를 복구할까요?
salvage-body = 마지막 세션이 이 녹화 파일을 쓰는 도중 예기치 않게 종료되었습니다. 복구는 원본 옆에 재생 가능한 사본을 만듭니다 — 원본 파일은 절대 변경되지 않습니다.
salvage-repair = 복구
salvage-repairing = 복구 중…
salvage-done = 복구됨
salvage-repaired = 복구됨 → { $name }
salvage-failed = 복구 실패: { $error }
salvage-dismiss = 나중에

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = 인코더 오류 — { $from }에서 { $to }(으)로 전환했습니다. 스트림은 다시 연결되어 계속됩니다.
fallback-toast-recording = 인코더 오류 — { $from }에서 { $to }(으)로 전환했습니다. 녹화는 새 파일로 계속됩니다.
fallback-note = 인코더 대체: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = 프로그램 오디오가 무음이 되었습니다
alarm-clipping = 프로그램 오디오가 클리핑되고 있습니다
alarm-black = 프로그램 화면이 검게 나옵니다
alarm-frozen = 프로그램 화면이 한동안 변하지 않았습니다
alarm-lowDisk = 디스크 공간: 현재 비트레이트로 약 { $minutes }분 남음
alarm-dismiss = 알람 닫기
alarm-cleared = 해결됨: { $alarm }

# CAP-M22 — panic button
palette-panic = 패닉 — 프라이버시 슬레이트로 전환
panic-banner-title = 패닉
panic-banner-body = 프로그램이 프라이버시 슬레이트를 표시 중입니다. 모든 오디오는 음소거되고 캡처는 중지되었습니다. 스트림과 녹화는 계속됩니다.
panic-restore = 복원…
panic-restore-confirm = 프로그램을 복원할까요?
panic-restore-yes = 복원
panic-restore-cancel = 취소
hotkeys-panic = 패닉 (프라이버시 슬레이트)
hotkeys-timer-toggle = 모든 타이머 시작/일시정지
hotkeys-timer-reset = 모든 타이머 재설정
panic-slate-color = 패닉 슬레이트 색상
panic-slate-image = 패닉 슬레이트 이미지
panic-slate-image-placeholder = 선택적 이미지 경로

# CAP-M24 — redacted diagnostics bundle
diag-title = 진단 번들
diag-intro = GitHub 이슈에 직접 첨부할 수정된 .zip(설정 스냅샷, 인코더 프로브, 최근 통계 — 비밀, 경로, 이름은 절대 포함되지 않음)을 내보냅니다. 아무것도 전송되지 않습니다.
diag-preview = 내용 보기
diag-hide-preview = 미리보기 숨기기
diag-export = .zip 내보내기
diag-exported = 내보냄: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = 방송 전 점검
preflight-intro = 차단 항목은 모두 녹색이어야 합니다. 나머지는 정직한 알림입니다.
preflight-item-targets = 스트림 대상 구성됨 (키/URL)
preflight-item-encoder = 사용 가능한 인코더 있음
preflight-item-sources = 모든 소스 정상
preflight-item-disk = 녹화용 디스크 공간
preflight-item-mic = 마이크 미터링
preflight-item-desktopAudio = 데스크톱 오디오 미터링
preflight-item-replay = 리플레이 버퍼 대기
preflight-targets-detail = { $count }개 활성
preflight-sources-detail = { $count }개 소스 오류
preflight-disk-detail = 현재 비트레이트로 약 { $minutes }분
preflight-fix-stream = 스트림 설정…
preflight-fix-components = 구성 요소…
preflight-fix-sources = 소스 상태…
preflight-fix-replay = 대기
preflight-optional = 선택
preflight-hold = 모두 녹색이 될 때까지 방송 시작 보류
preflight-cancel = 취소
preflight-go-anyway = 그래도 방송 시작
preflight-go-live = 방송 시작


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = 배경
scenes-backdrop-aria = { $name } 배경
backdrop-title = 배경 — { $name }
backdrop-hint = 이 장면의 모든 것 뒤에 고정되는 배경화면입니다. 이미지, 움직이는 GIF, 반복 재생 동영상을 쓸 수 있습니다. 캐처는 항상 위에 있으며, 캔버스 위에서 스크롤하면 확대됩니다.
backdrop-choose = 이미지 또는 동영상 선택…
backdrop-remove = 배경 제거
backdrop-none = 배경이 없습니다.
backdrop-position = 위치
backdrop-split-full = 전체 캔버스
backdrop-split-left = 왼쪽 절반
backdrop-split-right = 오른쪽 절반
backdrop-split-top = 위쪽 절반
backdrop-split-bottom = 아래쪽 절반
backdrop-sync = 녹화 시작과 함께 재생 시작
backdrop-sync-hint = 녹화 전까지 첫 프레임에 머물고, 테이크마다 처음부터 재생합니다.
backdrop-preview-play = 미리 재생
backdrop-preview-pause = 미리보기 일시정지
backdrop-filter-all = 배경(이미지 및 동영상)
backdrop-filter-images = 이미지
backdrop-filter-media = 동영상 및 GIF
sources-backdrop-badge = 배경화면(맨 아래 고정)
sources-backdrop-pinned = 배경은 맨 아래에 고정됩니다
filters-name-flip = 뒤집기
filters-flip-horizontal = 좌우
filters-flip-vertical = 상하
history-setSceneBackdrop = 배경 설정
history-setBackdropSplit = 배경 이동
history-setBackdropSync = 배경 녹화 동기화
backdrop-scrub = 재생 위치
backdrop-loop = 반복
backdrop-reverse = 역재생
backdrop-reverse-hint = 역재생은 거꾸로 된 복사본을 한 번만 렌더링합니다(동영상은 ffmpeg 구성 요소 필요, GIF는 즉시). 긴 파일은 처음 전환에 시간이 걸릴 수 있습니다.
filters-scaling = 스케일링
filters-scaling-hint = 레트로/픽셀 콘텐츠용 픽셀 퍼펙트 모드입니다. 정수 모드는 그려지는 크기도 정수 배수로 고정합니다(핸들은 논리적 크기를 표시).
filters-scaling-auto = 부드럽게
filters-scaling-nearest = 최근접 이웃
filters-scaling-integer = 정수(정수 배)
filters-scaling-sharp = 샤프 바이리니어
history-setScaling = 스케일링 변경
hotkeys-zoom-100 = 줌: 초기화(100%)
hotkeys-zoom-150 = 줌: 150% 확대
hotkeys-zoom-200 = 줌: 2× 확대
sources-follow-title = 줌 중 커서 따라가기(Windows, 캔버스에서 스크롤하면 줌)
sources-follow-item = { $name } 커서 따라가기 전환
filters-autocrop = ✂ 검은 여백 자동 자르기
filters-autocrop-title = 다음 프레임에서 레터박스/필러박스 띠를 찾아 자릅니다(실행 취소 가능). 어두운 장면은 절대 잘리지 않습니다.
filters-autocrop-follow = 해상도가 바뀌면 다시 확인
history-autoCrop = 검은 여백 자동 자르기
sources-link-audio = 이 앱의 오디오도 캡처(연결됨: 숨기면 음소거, 창을 제거하면 함께 제거)
history-addLinkedWindow = 창 + 연결된 오디오 추가
sources-hdr-title = 이 디스플레이는 HDR — 톤 매핑 열기(캔버스는 SDR 유지)
sources-hdr-item = { $name }의 HDR 톤 매핑
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = 이 디스플레이는 HDR을 출력합니다. 톤 매핑이 없으면 하이라이트가 잘리고 SDR 캔버스에서 물 빠져 보입니다. 변경은 다음 프레임부터 적용됩니다.
sources-hdr-enable-suggested = 권장 설정 켜기(maxRGB, 200니트)
sources-hdr-operator = 연산자
sources-hdr-op-clip = 클립(끔)
sources-hdr-op-maxrgb = maxRGB(색상 유지)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 니(SDR 정확)
sources-hdr-paper-white = 페이퍼 화이트
sources-hdr-nits = 니트
projector-target-passthrough = 패스스루 모니터(저지연)
projector-which-device = 장치
projector-passthrough-none = 먼저 디스플레이, 창 또는 캡처 장치를 추가하세요.
projector-passthrough-about = 장치의 원본 프레임 — 장면도, 필터도, 컴포지터도 거치지 않습니다. 측정된 지연 시간을 표시합니다. 오디오는 여전히 믹서 스트립으로 모니터링합니다.
projector-passthrough-hint = 패스스루 — Esc로 닫기
projector-latency = { $ms } ms
projector-latency-measuring = 측정 중…
automation-title = 자동화 — 규칙, 매크로, 변수
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = 규칙
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = 켜기
automation-rule-name = Rule name
automation-remove = Remove
automation-when = 조건
automation-then-run = 실행할 매크로
automation-no-macro = (no macro)
automation-macros = 매크로
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = 실행
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = 스튜디오 변수
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
rundown-title = 쇼 큐시트
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = 시작
rundown-next = 다음 ▸
rundown-stop = 중지
rundown-idle = 정지됨
rundown-next-up = 다음: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + 단계
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
automation-layer = 레이어
automation-layer-hint = 이 레이어가 활성일 때만 실행됩니다(비우면 모든 레이어). 레이어는 고정식입니다(OS 전역 단축키 API로는 누르고 있는 방식의 레이어를 만들 수 없습니다).
automation-chord-hint = 단일 키(Ctrl+Shift+M) 또는 2타 코드(Ctrl+K, 3). 코드의 두 번째 키는 코드 대기 중에만 점유됩니다.
panel-title = LAN 패널 및 탈리
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = 패널 제공
panel-port = 포트
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = 비밀번호
panel-show = 표시
panel-hide = 숨기기
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = 저장
osc-title = OSC 컨트롤 서피스
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = OSC 수신
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = PTZ 카메라
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = 카메라
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = 주소
ptz-port = 포트
ptz-speed = 속도
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
ptz-presets = 프리셋
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = 불러오기
ptz-store = 저장
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = MIDI 컨트롤 서피스
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = 입력
midi-output = 출력(피드백)
midi-none = (none)
midi-learn = 학습
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = 동작
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
panel-lan-warning = ⚠ LAN 트래픽은 암호화되지 않습니다 — 비밀번호가 평문 HTTP URL에 실립니다. 신뢰하는 네트워크에서만 사용하세요.
osc-lan-warning = ⚠ OSC에는 비밀번호가 없습니다 — 네트워크의 모든 기기가 이 명령을 보낼 수 있습니다. LAN 모드는 신뢰하는 네트워크에서만.

# System-stats HUD source (CAP-N14)
sources-badge-stats = 통계
sources-add-system-stats = 성능 통계 (HUD)
sources-stats-title = 성능 HUD 추가
sources-stats-note = 스튜디오의 실제 측정값 — fps, CPU, 메모리, 렌더 시간, 드롭된 프레임, 라이브 비트레이트 — 를 시청자에게 프로그램 화면에 표시합니다. 표시할 줄, 크기, 색상은 소스의 속성에서 설정합니다. GPU 사용률은 측정되지 않으므로 표시하지 않습니다.
sources-stats-add = 통계 HUD 추가
properties-stats-show-fps = FPS 표시
properties-stats-show-cpu = CPU 표시
properties-stats-show-memory = 메모리 표시
properties-stats-show-render = 렌더 시간 표시
properties-stats-show-dropped = 드롭 프레임 표시
properties-stats-show-bitrate = 비트레이트 표시
properties-stats-show-timecode = 타임코드 표시(LTC)
properties-stats-size = 크기 (px)
properties-stats-note = HUD는 간결한 공통 라벨(FPS, CPU, MEM, RENDER, DROPPED, BITRATE)을 프로그램에 그대로 그립니다. 방송 중이 아니면 비트레이트 줄에 “—”가 표시됩니다.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = 비주얼라이저
sources-add-visualizer = 오디오 비주얼라이저
sources-visualizer-title = 오디오 비주얼라이저 추가
sources-visualizer-style-label = 스타일
sources-visualizer-style-bars = 스펙트럼 바
sources-visualizer-style-scope = 오실로스코프
sources-visualizer-style-vu = VU 미터
sources-visualizer-target-label = 청취 대상
sources-visualizer-target-master = 마스터 믹스
sources-visualizer-target-track = 트랙 { $n }
sources-visualizer-note = 실제로 믹스되는 신호(페이더 이후)를 그립니다. 음소거된 소스는 들리는 그대로 평평하게 표시됩니다. 크기, 색상, 바 개수, 하강 속도는 소스의 속성에서 설정합니다.
sources-visualizer-add = 비주얼라이저 추가
properties-vis-bands = 바 개수
properties-vis-decay = 하강 속도 (dB/s)
properties-vis-peak-hold = 피크 홀드 표시
properties-vis-missing-source = (소스 없음)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = 스플릿
sources-add-split-timer = 스피드런 스플릿 타이머
sources-splits-title = 스플릿 타이머 추가
sources-splits-file-label = LiveSplit .lss 파일
sources-splits-comparison-label = 비교 대상
sources-splits-comparison-pb = 개인 최고 기록
sources-splits-comparison-best = 베스트 세그먼트
sources-splits-comparison-average = 평균
sources-splits-note = 파일은 읽기 전용으로 가져오며 절대 다시 쓰지 않습니다. 설정 → 단축키에서 전역 Split / Undo / Skip / Reset 키를 지정하세요. 프로세스 메모리 방식 오토 스플리터는 의도적으로 지원하지 않습니다.
sources-splits-add = 스플릿 타이머 추가
properties-splits-size = 크기 (px)
properties-splits-ahead = 앞섬
properties-splits-behind = 뒤처짐
properties-splits-gold = 골드
properties-splits-split = 스플릿
properties-splits-undo = 되돌리기
properties-splits-skip = 건너뛰기
properties-splits-reset = 리셋
properties-splits-note = 버튼은 실행 중인 타이머를 조작합니다(전역 단축키는 어떤 앱에서든 같은 동작을 합니다). 기록은 .lss 파일에 저장되지 않습니다.
hotkeys-split-split = 스플릿 타이머: 시작 / 스플릿
hotkeys-split-undo = 스플릿 타이머: 스플릿 되돌리기
hotkeys-split-skip = 스플릿 타이머: 세그먼트 건너뛰기
hotkeys-split-reset = 스플릿 타이머: 리셋
hotkey-audit-action-split-split = 스플릿 (스플릿 타이머)
hotkey-audit-action-split-undo = 스플릿 되돌리기
hotkey-audit-action-split-skip = 세그먼트 건너뛰기
hotkey-audit-action-split-reset = 스플릿 타이머 리셋
hotkey-audit-feature-split-timer = 스플릿 타이머

# Media playlist source (CAP-N17)
sources-badge-playlist = 재생목록
sources-add-playlist = 미디어 재생목록 (끊김 없음)
sources-playlist-title = 미디어 재생목록 추가
sources-playlist-files-label = 파일 (한 줄에 하나, 위에서부터 재생)
sources-playlist-browse = 찾아보기…
sources-playlist-loop = 반복
sources-playlist-shuffle = 셔플 (시작할 때마다 한 번 추첨, 반복 시 같은 순서 유지)
sources-playlist-hold-last = 끝에서 마지막 프레임 유지
sources-playlist-note = 트리밍된 목록 전체를 표시된 ffmpeg 구성 요소로 끊김 없이 재생합니다(wire 형식 전용 — .frec와 이미지는 미디어/슬라이드쇼로). 항목은 모두 비디오이거나 모두 오디오여야 하며 섞을 수 없습니다. 트림, 큐 포인트, «지금 재생 중» 변수는 속성에 있습니다.
sources-playlist-add = 재생목록 추가
properties-playlist-items = 항목 (위에서부터 재생)
properties-playlist-up = 위로
properties-playlist-down = 아래로
properties-playlist-remove = 항목 제거
properties-playlist-in = 시작 (초)
properties-playlist-out = 끝 (초)
properties-playlist-cues = 큐 (초, 쉼표로 구분)
properties-playlist-add-item = + 항목 추가
properties-playlist-loop = 반복
properties-playlist-shuffle = 셔플
properties-playlist-hold-last = 마지막 프레임 유지
properties-playlist-hw = 하드웨어 디코드
properties-playlist-variable = «지금 재생 중» 변수 (비움 = 끔)
properties-playlist-previous = ⏮ 이전
properties-playlist-next = ⏭ 다음
properties-playlist-note = 큐 버튼과 다음/이전은 실행 중인 재생목록을 조작합니다. 항목 수정은 적용 시 반영됩니다(재생목록 재시작). 텍스트 소스에 {"{{"}yourVariable{"}}"} 를 넣으면 재생 중인 항목 이름이 표시됩니다.
hotkeys-playlist-next = 재생목록: 다음 항목
hotkeys-playlist-previous = 재생목록: 이전 항목
hotkey-audit-action-playlist-next = 재생목록 다음
hotkey-audit-action-playlist-previous = 재생목록 이전
hotkey-audit-feature-playlist = 재생목록

# Instant replay source (CAP-N10)
sources-badge-replay = 리플레이
sources-add-replay = 인스턴트 리플레이
sources-replay-title = 인스턴트 리플레이 추가
sources-replay-seconds-label = 롤 길이 (초)
sources-replay-speed-label = 속도
sources-replay-speed-full = 100% (오디오 포함)
sources-replay-speed-half = 50% 슬로모션 (무음)
sources-replay-speed-quarter = 25% 슬로모션 (무음)
sources-replay-note = 롤하기 전까지 투명 상태입니다. 리플레이 버퍼를 활성화(컨트롤)하고 롤 단축키를 지정하세요. 롤은 버퍼의 마지막 순간을 잘라 프로그램에 재생한 뒤 다시 투명해집니다.
sources-replay-add = 인스턴트 리플레이 추가
properties-replay-roll = ⏵ 리플레이 롤
properties-replay-note = 롤은 활성화된 버퍼를 클립으로 잘라 선택한 속도로 재생합니다 — 리타이밍만, 보간 없음. 슬로모션은 의도적으로 무음입니다. 재생 중 스크럽/일시정지가 동작하며, 끝나면 소스가 투명으로 돌아갑니다.
hotkeys-replay-roll = 인스턴트 리플레이: 롤
hotkey-audit-action-replay-roll = 인스턴트 리플레이 롤

# Input overlay source (CAP-N13)
sources-badge-input = 입력
sources-add-input-overlay = 입력 오버레이(키/패드)
sources-input-title = 입력 오버레이 추가
sources-input-layout-label = 레이아웃
sources-input-layout-wasd = WASD + 마우스
sources-input-layout-keyboard = 컴팩트 키보드 + 마우스
sources-input-layout-gamepad = 게임패드(듀얼 스틱)
sources-input-layout-fightstick = 파이트 스틱
sources-input-color-label = 키
sources-input-accent-label = 눌림
sources-input-privacy-note = 개인정보 보호: 입력은 이 소스가 장면에서 라이브인 동안에만 읽히며, 레이아웃의 고정 키만 폴링합니다 — "지금 눌려 있는가"를 순간적으로 확인할 뿐, 후킹은 절대 하지 않습니다. 아무것도 기록·저장되거나 어디로도 전송되지 않으며, 입력한 텍스트는 결코 캡처되지 않습니다.
sources-input-os-note = 키보드와 마우스 상태는 현재 Windows에서만 읽힙니다 — 다른 시스템에서는 키가 눌리지 않은 상태로 그려집니다(정직하게 표기하며, 가짜로 흉내 내지 않습니다). 게임패드는 gilrs 라이브러리로 어디서나 동작하며, 처음 연결된 컨트롤러를 그립니다. 컨트롤러가 없으면 레이아웃은 눌리지 않은 채로 남습니다.
sources-input-add = 입력 오버레이 추가

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = 커서 효과
filters-cursorfx-hint = Windows에서는 앱이 커서를 직접 그리므로 효과가 캡처에 바로 그려져 녹화와 방송에 그대로 나타납니다. macOS와 Linux는 커서를 시스템에서 합성하므로 이 효과는 Windows 전용입니다. 변경 사항은 즉시 적용됩니다.
filters-cursorfx-halo = 커서 후광
filters-cursorfx-halo-color = 색상
filters-cursorfx-halo-radius = 반경 (px)
filters-cursorfx-ripples = 클릭 물결
filters-cursorfx-left-color = 왼쪽 클릭
filters-cursorfx-right-color = 오른쪽 클릭
filters-cursorfx-keystrokes = 키 입력 표시
filters-cursorfx-keystrokes-hint = 누르고 있는 동안 고정된 키 집합(문자, 숫자, 보조키, 화살표)을 커서 옆에 표시합니다. 키는 이 기능이 켜져 있을 때만 읽히고 프레임에 바로 그려질 뿐, 절대 저장되거나 기록되지 않습니다.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = 타이틀
sources-add-title = 타이틀 / 점수판
sources-title-title = 타이틀 추가
sources-title-template-label = 시작 템플릿
sources-title-template-lower-third = 하단 자막바(바 + 이름 + 직함)
sources-title-template-scoreboard = 점수판(판 + 4칸)
sources-title-template-blank = 빈 캔버스
sources-title-width-label = 캔버스 너비
sources-title-height-label = 캔버스 높이
sources-title-template-name = 이름
sources-title-template-subtitle = 직함
sources-title-template-home = 홈
sources-title-template-away = 원정
sources-title-note = 텍스트/이미지/박스 레이어로 만드는 타이틀. 인/아웃 애니메이션과 함께 로컬에서 합성 — 브라우저 소스가 아닙니다. 레이어, 파일 바인딩과 {"{{"}변수{"}}"}, 라이브 제어는 소스의 속성에 있습니다.
sources-title-add = 타이틀 추가
properties-title-layers = 레이어(순서대로 그려짐 — 뒤 행이 위에 놓임)
properties-title-kind-text = 텍스트
properties-title-kind-image = 이미지
properties-title-kind-rect = 박스
properties-title-x = X
properties-title-y = Y
properties-title-outline = 외곽선 (px)
properties-title-outline-color = 외곽선
properties-title-shadow = 그림자
properties-title-animation = 인/아웃 애니메이션
properties-title-anim-none = 없음(컷)
properties-title-anim-fade = 페이드
properties-title-anim-slide-left = 왼쪽으로 슬라이드
properties-title-anim-slide-up = 위로 슬라이드
properties-title-anim-wipe = 와이프
properties-title-duration = 길이 (ms)
properties-title-fire-in = ▶ 인 실행
properties-title-fire-out = ◼ 아웃 실행
properties-title-set-live = 라이브 반영
properties-title-set-live-note = 이 텍스트를 지금 방송 중인 타이틀에 바로 반영 — 적용도 재시작도 없음
properties-title-up = 레이어 위로
properties-title-down = 레이어 아래로
properties-title-remove = 레이어 제거
properties-title-add-text = + 텍스트
properties-title-add-image = + 이미지
properties-title-add-rect = + 박스
properties-title-note = 인/아웃과 "라이브 반영"은 실행 중인 타이틀을 제어합니다. 레이어 편집은 적용 시 반영되며(타이틀이 재시작되어 다시 인 됩니다), 텍스트 칸은 감시 파일(CSV 셀 / JSON 값 / 파일 전체)에 바인딩하고 {"{{"}변수{"}}"}를 치환합니다 — "라이브 반영"이 둘 다보다 우선합니다.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN 인제스트(SRT/RTMP 리스너)
sources-lan-title = LAN 인제스트 리스너 추가
sources-lan-protocol-label = 프로토콜
sources-lan-protocol-srt = SRT(암호화 가능 — 권장)
sources-lan-protocol-rtmp = RTMP(인증 없음)
sources-lan-port-label = 포트(1024–65535)
sources-lan-passphrase-label = 패스프레이즈(비우면 개방)
sources-lan-passphrase-hint = SRT 패스프레이즈는 10–79자이며 송신 측도 같은 것을 사용해야 합니다.
sources-lan-open-warning = 패스프레이즈 없음: 이 네트워크의 누구나 암호화 없이 이 소스로 보낼 수 있습니다. 나만 쓰는 네트워크가 아니라면 설정하세요.
sources-lan-rtmp-warning = RTMP에는 인증이 없습니다 — 이 네트워크의 누구나 이 포트로 보낼 수 있습니다. 패스프레이즈를 쓰는 SRT를 권장합니다.
sources-lan-url-label = 송신 앱을 다음 주소로
sources-lan-qr-aria = 인제스트 URL QR 코드
sources-lan-note = LAN 전용: 이 컴퓨터의 로컬 주소에서 소스가 존재하는 동안에만 수신 대기하며 인터넷에는 절대 닿지 않습니다 — 네트워크의 송신자가 먼저 보내기 전에는 아무것도 컴퓨터를 떠나지 않습니다. 디코딩은 명확히 표시된 ffmpeg 구성 요소로 처리됩니다. 송신자가 연결될 때까지 캔버스에 이 URL이 표시됩니다.
sources-lan-add = 수신 대기 시작
properties-lan-note = 프로토콜·포트·패스프레이즈 변경을 적용하면 리스너가 다시 시작됩니다 — 송신 측은 다시 연결해야 합니다. 스트림은 1920×1080 캔버스에 맞춰집니다.

# Freally Link source & output (CAP-N12)
sources-badge-link = 링크
sources-add-freally-link = Freally Link (다른 인스턴스)
sources-link-title = Freally Link 추가
sources-link-about = 내 네트워크를 통해 다른 Freally Capture 인스턴스의 프로그램(영상 + 마스터 오디오)을 수신합니다. 먼저 보내는 쪽에서 "Freally Link 출력"을 켜세요. v1은 TCP로 모션 JPEG을 전송합니다. 유선 LAN이나 좋은 Wi-Fi에 알맞고, 약한 회선에서는 대역폭에 대해 정직하게 동작합니다.
sources-link-scan = LAN 검색
sources-link-scanning = 검색 중…
sources-link-none = Freally Link 출력을 찾지 못했습니다. 다른 인스턴스에서 "Freally Link 출력"을 켜거나(컨트롤 → LAN 패널) 아래에 주소를 입력하세요.
sources-link-host = 주소
sources-link-port = 포트
sources-link-key = 페어링 키
sources-link-key-hint = 송신 측 "Freally Link 출력" 설정의 키 — 없으면 송신 측이 단 한 프레임도 전송하지 않습니다.
sources-link-add = 링크 추가
properties-link-note = 연결되지 않은 동안에는 "연결 중" 화면을 표시하며 점점 늘어나는 간격으로 스스로 재시도합니다. 오래된 프레임에 멈추는 일은 없습니다. 송신자당 수신자는 하나이며, 사용 중인 송신자에게는 정중하게 재시도합니다.
link-title = Freally Link 출력
link-about = 이 인스턴스의 프로그램(영상 + 마스터 오디오)을 내 네트워크의 다른 Freally Capture 한 대와 공유합니다. 상대 쪽에는 "Freally Link" 소스로 나타납니다(2대 구성 스트리밍, 보조 모니터). 기본값은 꺼짐이며, 켜기 전에는 아무것도 알리거나 수신 대기하지 않습니다. v1은 TCP로 모션 JPEG + 무압축 오디오를 전송합니다. 유선 LAN이나 좋은 Wi-Fi용이며 인터넷으로는 절대 나가지 않습니다.
link-enable = 내 네트워크에서 프로그램 공유
link-name = 인스턴스 이름
link-key = 페어링 키
link-key-hint = 8자 이상 — 수신기가 이 키를 입력하기 전에는 단 한 프레임도 전송되지 않습니다.
link-lan-warning = ⚠ 수신기는 페어링 키를 제시해야 무엇이든 받을 수 있지만, 스트림 자체는 v1에서 암호화되지 않습니다. 신뢰할 수 있는 네트워크에서만 사용하세요.
link-serving = 수신자는 "LAN 검색"으로 이 인스턴스를 찾거나 다음 주소로 직접 추가할 수 있습니다:
link-off-hint = 공유를 켜면 포트가 열리고 LAN 검색에 이 인스턴스가 알려집니다.

# In-app menu bar (OBS-style chrome)
menu-bar-label = 애플리케이션 메뉴
menu-file = 파일
menu-edit = 편집
menu-view = 보기
menu-docks = 독
menu-profile = 프로필
menu-collection = 장면 모음
menu-tools = 도구
menu-help = 도움말
menu-rename = 이름 바꾸기
menu-remove = 제거
menu-import = 가져오기
menu-export = 내보내기
menu-file-show-recordings = 녹화 파일 표시
menu-file-remux = MP4로 리먹스…
menu-file-settings = 설정…
menu-file-show-settings-folder = 설정 폴더 표시
menu-file-exit = 종료
menu-edit-undo = 실행 취소
menu-edit-redo = 다시 실행
menu-edit-history = 편집 기록…
menu-edit-copy-transform = 변형 복사
menu-edit-paste-transform = 변형 붙여넣기
menu-edit-copy-filters = 필터 복사
menu-edit-paste-filters = 필터 붙여넣기
menu-edit-transform = 변형…
menu-edit-lock-preview = 미리보기 잠금
menu-view-fullscreen = 전체 화면 인터페이스
menu-stats-dock = 통계 독
menu-view-multiview = 멀티뷰 모니터…
menu-view-projectors = 프로젝터…
menu-view-source-health = 소스 상태…
menu-view-still = 스틸 프레임 캡처
menu-docks-browser = 브라우저 독…
menu-docks-lock = 독 잠금
menu-docks-reset = 독 배치 초기화
menu-profile-manage = 프로필 관리…
menu-collection-manage = 장면 모음 관리…
menu-collection-import-obs = OBS에서 가져오기…
menu-collection-missing = 누락된 파일 확인…
menu-tools-wizard = 설정 마법사 실행
menu-tools-wizard-title = 설정 마법사는 첫 실행 시 동작합니다. 다시 실행하는 방법은 아직 없습니다.
menu-tools-automation = 자동화 규칙 및 매크로…
menu-tools-rundown = 쇼 큐시트 표시…
menu-tools-hotkeys = 단축키 지도…
menu-tools-av-sync = A/V 동기화 보정…
menu-tools-scripts = Lua 스크립트…
menu-tools-components = 컴포넌트…
menu-tools-midi = MIDI 컨트롤…
menu-tools-ptz = PTZ 카메라…
menu-tools-remote = 원격 제어 API…
menu-tools-panel = LAN 패널 및 탈리…
menu-help-portal = 도움말 포털
menu-help-website = 웹사이트 방문
menu-help-discord = Discord 서버 참여
menu-help-bug = 버그 신고…
menu-help-updates = 업데이트 확인…
menu-help-whats-new = 새로운 기능
menu-help-about = 정보…
menu-help-more-apps = 더 많은 Freally 앱…
moreapps-title = 더 많은 Freally 앱

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = 설정 카테고리
settings-cat-general = 일반
settings-cat-appearance = 모양
settings-cat-streaming = 스트리밍
settings-cat-output = 출력
settings-cat-replay = 리플레이
settings-cat-hotkeys = 단축키
settings-cat-network = 네트워크
settings-cat-accessibility = 접근성
settings-cat-about = 정보
settings-ok = 확인
settings-cancel = 취소
settings-apply = 적용
settings-save = 저장
settings-loading = 설정 불러오는 중…
settings-hotkeys-filter = 단축키 필터
settings-hotkeys-filter-placeholder = 입력하여 동작이나 키를 필터링…
settings-hotkeys-no-match = “{ $query }”와 일치하는 단축키가 없습니다.
settings-hotkey-none = 없음
settings-hotkey-group-ctrl = Ctrl + 키
settings-hotkey-group-ctrl-shift = Ctrl + Shift + 키
settings-hotkey-group-ctrl-alt = Ctrl + Alt + 키
settings-hotkey-group-function = 기능 키
settings-hotkey-group-numpad = 숫자 패드
settings-panic-section = 패닉 슬레이트
settings-meter-section = 믹서 레벨 미터
settings-meter-note = 오디오 믹서의 레벨 미터가 조용한 상태에서 클리핑까지 거치는 색입니다. 색맹 안전 프리셋은 적록 색각 이상에서도 구분되는 파랑 → 주황 그라데이션을 사용합니다.
settings-meter-preset = 미터 색상
settings-meter-preset-default = 초록 / 노랑 / 빨강
settings-meter-preset-colorblind = 색맹 안전 (파랑 / 주황)
settings-meter-preset-custom = 사용자 지정
settings-meter-low = 보통
settings-meter-mid = 큼
settings-meter-high = 클리핑
settings-meter-preview = 미리 보기

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = 새로운 기능
whats-new-loading = 릴리스 노트 불러오는 중…
whats-new-version = 버전 { $version }의 새로운 기능
whats-new-empty = 이 버전에 대한 릴리스 노트가 없습니다.
filters-name-directional-blur = 방향 흐림
filters-name-radial-blur = 방사형 흐림
filters-name-zoom-blur = 줌 흐림
filters-name-pixelate = 픽셀화
filters-angle = 각도 (°)
filters-center-x = 중심 X
filters-center-y = 중심 Y
filters-block-size = 블록 크기 (px)
filters-name-freeze = 고정
filters-freeze-hint = 활성화하면 이 소스는 마지막 프레임을 유지합니다 — 프로그램, 미리보기, 녹화, 스트림이 모두 함께 고정됩니다. 이 필터를 켜거나 꺼서 고정하거나 해제하세요.
transform-3d = 3D 기울기
transform-rotation-x = 기울기 X (°)
transform-rotation-y = 기울기 Y (°)
transform-perspective = 원근
transform-reveal = 표시/숨기기
transform-reveal-ms = 표시 페이드인 (ms)
sources-clone-title = 복제 (같은 피드, 고유 필터)
sources-clone-item = { $name } 복제
menu-tools-downstream = 다운스트림 키어…
menu-tools-transition-rules = 전환 규칙…
dsk-title = 다운스트림 키어
dsk-hint = 프로그램 출력에 합성되는 오버레이 — 모든 장면 위에 표시되며 장면을 전환해도 그대로 유지됩니다(로고, LIVE 배지, 하단 자막). 목록 맨 위가 가장 앞에 그려집니다.
dsk-empty = 아직 키어가 없습니다 — 소스를 추가하여 모든 장면에 오버레이하세요.
dsk-enable = 이 키어 사용
dsk-move-up = 위로 이동 (맨 앞)
dsk-move-down = 아래로 이동
dsk-remove = 키어 제거
dsk-opacity = 불투명도
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = 크기
dsk-add = + 키어 추가
transition-rules-title = 전환 규칙
transition-rules-hint = 장면 쌍에 고유한 전환을 지정합니다. 첫 번째 장면에서 두 번째 장면으로 전환하면 기본값 대신 이 종류와 길이가 사용됩니다(스팅어/이미지 규칙은 전환 컨트롤에서 설정한 파일을 계속 사용합니다).
transition-rules-empty = 아직 규칙이 없습니다 — 모든 장면 쌍은 기본 전환을 사용합니다.
transition-rules-from = 출발
transition-rules-to = 도착
transition-rules-kind = 전환
transition-rules-duration = 길이 (ms)
transition-rules-add = 규칙 추가
transition-rules-remove = 규칙 제거
