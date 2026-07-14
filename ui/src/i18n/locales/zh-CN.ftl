# Freally Capture — zh-CN
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = 工作室模式
toggle-on = 开
toggle-off = 关
stats = 统计
core-ok = 内核正常
hide-stats-dock = 隐藏统计面板
show-stats-dock = 显示统计面板


# =============================================================
# --- shell ---
# =============================================================
# shell

# --- App shell (App.tsx) ---
app-save-error = 无法保存设置 — 此次更改在重启后将不会保留。
studio-mode-leave = 退出工作室模式
studio-mode-enter-title = 工作室模式 — 编辑预览场景，再通过转场提交到节目
vertical-canvas-title = 第二个（竖屏 9:16）输出画布 — 可独立录制和推流
app-version = v{ $version }
core-error = 内核错误
core-unreachable = 内核不可达（浏览器模式）
connecting-to-core = 正在连接内核…
filters-source-fallback = 来源

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = 节目预览
preview-program-output = 节目输出
preview-canvas-editor = 画布编辑器
preview-px-to-edge-label = 距画面边缘的像素
preview-px-to-edge = 距边缘 左 { $left } · 上 { $top } · 右 { $right } · 下 { $bottom }
preview-program-heading = 节目
preview-no-gpu = 未找到可用的 GPU 适配器 — 合成器无法在此机器上运行。
preview-starting-compositor = 正在启动合成器…
preview-empty-scene = 此场景为空 — 请在"来源"中添加一个来源，然后直接在画布上拖动、缩放和旋转它。
preview-fps = { $fps } fps
preview-dropped = 已丢 { $dropped } 帧

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = 已收到邀请链接
remote-join-with-webcam = 用摄像头加入
remote-dismiss = 忽略
remote-hosting-guest = 正在主持远程嘉宾
remote-you-are-guest = 你是远程嘉宾
remote-share-view-title = 向嘉宾的应用分享你的画面（他们会实时看到你的视图）
remote-stop-sharing-view = 停止分享视图
remote-share-my-view = 分享我的视图
remote-allow-center-title = 允许嘉宾切换由哪个视图占据中心（你仍保持控制权，可随时切回）
remote-guest-switching = 嘉宾切换：
remote-stop-screen = 停止屏幕
remote-share-screen = 分享屏幕
remote-share-screen-title-guest = 与主持人分享你的屏幕（它会成为一个可被居中的来源）
remote-center-request-label = 居中视图请求
remote-center = 居中
remote-center-cam-title = 请求主持人将你的摄像头居中
remote-center-my-cam = 我的摄像头
remote-center-screen-title = 请求主持人将你分享的屏幕居中
remote-center-my-screen = 我的屏幕
remote-center-host-title = 将中心交还给主持人的视图
remote-center-host-view = 主持人视图
remote-end-session = 结束会话
remote-leave = 离开
remote-host-view-heading = 主持人视图
remote-host-shared-view-label = 主持人分享的视图
remote-guest-position-label = 嘉宾位置
remote-guest-label = 嘉宾
remote-put-guest = 将嘉宾放到 { $position }
remote-remove-title = 移除嘉宾 — 他们可用同一链接重新加入
remote-remove = 移除
remote-ban-title = 封禁嘉宾 — 屏蔽他们并使邀请链接失效
remote-ban = 封禁
remote-guest-self-muted = 嘉宾已自行静音
remote-unmute-guest = 取消嘉宾静音
remote-mute-guest = 将嘉宾静音
remote-muted-by-host = 已被主持人静音
remote-unmute-mic = 取消麦克风静音
remote-mute-mic = 麦克风静音
remote-waiting-for-host = 正在等待主持人


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = 来源
sources-fallback-video = 视频
sources-fallback-error = 错误
sources-kind-unknown = ?
sources-missing-source = （来源缺失）

# Kind badges (small uppercase tag on each source row)
sources-badge-display = 显示器
sources-badge-window = 窗口
sources-badge-portal = 门户
sources-badge-camera = 摄像头
sources-badge-image = 图像
sources-badge-media = 媒体
sources-badge-guest = 嘉宾
sources-badge-color = 颜色
sources-badge-text = 文本
sources-badge-scene = 场景
sources-badge-slides = 幻灯片
sources-badge-chat = 聊天
sources-badge-audio-in = 音频输入
sources-badge-audio-out = 音频输出
sources-badge-app-audio = 应用音频
sources-badge-test-bars = 彩条
sources-badge-test-grid = 网格
sources-badge-test-sweep = 扫描
sources-badge-test-tone = 音调
sources-badge-test-sync = 同步
sources-badge-timer = 计时器

# Add-source menu items
sources-add-display = 显示器采集
sources-add-window = 窗口采集
sources-add-game = 游戏采集（请先阅读）
sources-add-webcam = 视频采集设备
sources-add-image = 图像
sources-add-media = 媒体（视频/图像文件）
sources-add-remote-guest = 远程嘉宾（P2P 试验）
sources-add-color = 色源
sources-add-text = 文本
sources-add-timer = 计时器 / 时钟
sources-add-nested-scene = 嵌套场景
sources-add-slideshow = 图像幻灯片
sources-add-chat-overlay = 实时聊天叠加
sources-add-test-signal = 测试信号
sources-add-audio-input = 音频输入采集
sources-add-audio-output = 音频输出采集
sources-add-app-audio = 应用程序音频（Windows）
sources-add-existing = 已有来源…

# Panel header + toolbar buttons
sources-panel-title = 来源
sources-group-title = 分组来源 — 选择两个或更多项目，然后点击"创建分组"；分组内的项目会一起移动并一起显示/隐藏
sources-group-aria = 分组来源
sources-arrange = 排列：屏幕 + 四角
sources-add-source = 添加来源
sources-browser-source-note = 浏览器来源作为独立的按需组件里程碑发布（约 180 MB 的 Chromium 引擎 — 从不打包在内）。目前：用"窗口采集" + 色度/颜色键采集真实的浏览器窗口，或将聊天/提醒作为停靠窗打开（控制 → 停靠窗）。

# Empty state
sources-empty = 此场景中没有来源 — 用"+"添加显示器采集、窗口、摄像头、图像、色源或文本。在画布上拖动、缩放和旋转它们；右侧按钮可重新排列堆叠顺序。

# Per-row controls
sources-already-in-group = 已在 { $name } 中
sources-pick-for-new-group = 选入新分组
sources-pick-item-for-group = 将 { $name } 选入新分组
sources-hide = 隐藏
sources-show = 显示
sources-hide-item = 隐藏 { $name }
sources-show-item = 显示 { $name }
sources-unfocus-title = 取消聚焦 — 恢复布局
sources-focus-title = 聚焦 — 填满画布（突出说话者）
sources-unfocus-item = 取消聚焦 { $name }
sources-focus-item = 聚焦 { $name }
sources-center-title = 居中 — 使其成为共享的中心视图（摄像头移到侧栏）
sources-center-item = 居中 { $name }
sources-rename-item = 重命名 { $name }
sources-in-group = 位于分组 { $name }

# Row status + retry
sources-retry-error = 重试 — { $message }
sources-retry-item = 重试 { $name }
sources-status-error = 状态：错误
sources-open-privacy-title = 打开此权限的 macOS 隐私设置
sources-open-privacy-item = 打开 { $name } 的隐私设置
sources-privacy-settings-button = 设置
sources-status-starting = 正在启动…
sources-status-live = 运行中
sources-status-aria = 状态：{ $state }

# Media row pause/resume
sources-media-resume-title = 恢复视频（在直播流中实时播放）
sources-media-pause-title = 暂停视频 — 定格画面并静音，在直播流中实时生效
sources-media-resume-item = 恢复 { $name }
sources-media-pause-item = 暂停 { $name }

# Hover controls
sources-unlock = 解锁
sources-lock = 锁定
sources-unlock-item = 解锁 { $name }
sources-lock-item = 锁定 { $name }
sources-raise-title = 在堆叠中上移
sources-raise-item = 上移 { $name }
sources-lower-title = 在堆叠中下移
sources-lower-item = 下移 { $name }
sources-filters-title = 滤镜与混合
sources-filters-item = { $name } 的滤镜
sources-properties-title = 属性
sources-properties-item = { $name } 的属性
sources-remove-title = 从此场景中移除
sources-remove-item = 移除 { $name }

# Grouping footer
sources-create-group = 创建分组（{ $count }）
sources-cancel = 取消

# Groups list
sources-groups-aria = 来源分组
sources-hide-group = 隐藏分组
sources-show-group = 显示分组
sources-item-count = · { $count } 个项目
sources-ungroup-title = 取消分组 — 项目保持原位
sources-ungroup-item = 取消分组 { $name }

# Live Chat Overlay picker
sources-chat-title = 添加实时聊天叠加
sources-chat-youtube-label = YouTube — 频道、观看或 live_chat 网址（无需密钥，无需登录）
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  或一个 watch?v= 网址
sources-chat-twitch-label = Twitch — 频道名称（匿名读取，无需账户）
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — 频道 slug（公开端点，尽力而为）
sources-chat-kick-placeholder = yourchannel
sources-chat-note = 消息以透明背景显示，并带有滚动的 h:mm:ss AM/PM 时间戳（默认右上角；可拖到任意位置）。聊天刷屏只会让旧消息逐渐消失 — 永远不会拖慢直播流或录制。Facebook 聊天需要你自己的 Graph 令牌，目前尚未实现 — 它从不是必需的，也从不影响上述平台。
sources-chat-add = 添加聊天叠加
sources-chat-default-name = 实时聊天

# Image Slideshow picker
sources-slideshow-title = 添加图像幻灯片
sources-slideshow-empty = 尚无图像 —"浏览"会按顺序添加它们。
sources-slideshow-remove-slide = 移除幻灯片 { $number }
sources-slideshow-browse = 浏览图像…
sources-slideshow-per-slide-label = 每张幻灯片（毫秒）
sources-slideshow-crossfade-label = 交叉淡化（毫秒，0 = 直切）
sources-slideshow-loop-label = 循环（关闭 = 停留在最后一张）
sources-slideshow-shuffle-label = 每轮随机播放
sources-slideshow-note = 交叉淡化会混合尺寸相同的图像；尺寸不同则在切换边界直接硬切（不做隐式缩放）。
sources-slideshow-add = 添加幻灯片（{ $count }）

# Nested Scene picker
sources-nested-title = 添加嵌套场景
sources-nested-empty = 没有其他可嵌套的场景 — 请先添加第二个场景。
sources-nested-scene-name = 场景：{ $name }
sources-nested-note = 嵌套场景以节目画布尺寸实时渲染，并跟随其自身的编辑；变换、滤镜和混合会像对待任何来源一样应用于它。当显示它的场景处于节目时，其音频源会加入混音。

# Display / Window capture picker
sources-capture-display-title = 添加显示器采集
sources-capture-window-title = 添加窗口采集
sources-capture-looking = 正在查找来源…
sources-capture-none-displays = 这里没有可采集的内容 — 未找到显示器。
sources-capture-none-windows = 这里没有可采集的内容 — 未找到窗口。
sources-capture-portal-note = 在 Wayland 上，由系统对话框选择屏幕或窗口 — 应用无法在那里进行全局采集，所以这是最诚实（也是唯一）的路径。
sources-capture-window-note = 预览会实时更新。最小化的窗口会显示其最后一帧（或不显示），直到你还原它。
sources-thumb-no-preview = 无预览
sources-thumb-loading = 正在加载…

# Video Capture Device picker
sources-webcam-title = 添加视频采集设备
sources-webcam-looking = 正在查找摄像头…
sources-webcam-none = 未找到摄像头或采集卡。
sources-webcam-format-label = 格式
sources-webcam-format-auto-loading = 自动（正在加载格式…）
sources-webcam-format-auto = 自动（最高分辨率）
sources-webcam-card-presets-label = 采集卡预设：
sources-webcam-preset-title = 选择此采集卡所支持的 { $label } 模式
sources-webcam-add = 添加摄像头

# Audio Input / Output capture picker
sources-audio-output-title = 添加音频输出采集
sources-audio-input-title = 添加音频输入采集
sources-audio-default-output = 默认输出（你听到的声音）
sources-audio-default-input = 默认输入
sources-audio-looking = 正在查找音频设备…
sources-audio-none-output = 这里未找到桌面音频采集设备。
sources-audio-none-input = 未找到麦克风或线路输入。
sources-audio-input-note = 混音条会获得 VU 电平表、推子、静音、监听、滤镜（降噪、门限、压缩器…）和轨道分配。一切都留在本机。

# Application Audio picker
sources-appaudio-title = 添加应用程序音频
sources-appaudio-looking = 正在查找正在发声的应用…
sources-appaudio-none = 目前没有应用在发声 — 请先在应用中开始播放，然后刷新。
sources-appaudio-refresh = ⟳ 刷新
sources-appaudio-note = 精确采集该应用的音频 — 拥有自己的 VU 电平表、推子、静音、滤镜和轨道。

# Game Capture picker
sources-game-title = 游戏采集
sources-game-checking = 正在检查…
sources-game-use-portal = 使用屏幕采集（门户）
sources-game-use-window = 改用窗口采集

# Image picker
sources-image-title = 添加图像
sources-image-file-label = 图像文件（PNG、JPEG、BMP、GIF、WebP…）
sources-image-add = 添加图像

# Path field
sources-browse = 浏览…

# Media picker
sources-media-title = 添加媒体
sources-media-file-label = 媒体文件（mp4、mkv、webm、mov、.frec 或图像）
sources-media-loop-label = 循环（播放到结尾后从头重新开始）
sources-media-note = .frec 通过自有的 freally-video 编解码器播放 — 无需下载。有线格式（mp4/mkv/webm/…）通过按需的 FFmpeg 组件解码；其音频会作为独立的混音条进入混音器。
sources-media-add = 添加媒体

# Invite expiry options
sources-ttl-15min = 15 分钟
sources-ttl-30min = 30 分钟
sources-ttl-1hour = 1 小时
sources-ttl-1day = 1 天

# Remote Guest form
sources-remote-copy-failed = 无法复制 — 请选中链接并手动复制
sources-remote-join-failed = 加入失败：{ $error }
sources-remote-title = 远程嘉宾（P2P 试验）
sources-remote-host-heading = 主持人 — 邀请嘉宾
sources-remote-start-hosting = 开始主持
sources-remote-expires-label = 到期
sources-remote-invite-expiry-aria = 邀请到期时间
sources-remote-invite-link-aria = 邀请链接
sources-remote-copied = 已复制 ✓
sources-remote-copy = 复制
sources-remote-share-note = 分享此链接（Discord / 短信 / 邮件）。它携带你的会话，并按设定到期。嘉宾打开它即可用摄像头加入。
sources-remote-qr-note = 用手机扫描即可直接从浏览器加入 — 摄像头 + 麦克风，无需安装。上方可复制的 freally:// 链接会在已安装 Freally Capture 的机器上打开。
sources-remote-guest-heading = 嘉宾 — 用邀请加入
sources-remote-paste-placeholder = 粘贴邀请链接
sources-remote-invite-input-aria = 邀请链接或会话 id
sources-remote-join = 用摄像头加入
sources-remote-session-note = 直播会话控件（静音、结束）保留在主窗口顶部的栏上 — 你可以关闭此对话框。
sources-remote-stop-session = 停止会话

# Invite QR
sources-invite-qr-aria = 邀请链接二维码

# Remote device pickers
sources-devices-output-unavailable = 输出路由不可用 — 正在默认设备上播放
sources-devices-mic-test-failed = 麦克风测试失败：{ $error }
sources-devices-heading = 会话音频设备
sources-devices-microphone-label = 麦克风
sources-devices-microphone-aria = 会话麦克风
sources-devices-system-default = 系统默认
sources-devices-output-label = 输出
sources-devices-output-aria = 会话音频输出
sources-devices-stop-test = 停止测试
sources-devices-test = 测试 — 听到自己的声音
sources-devices-testing-note = 对着麦克风说话 — 你正在实时听到所选设备的声音
sources-devices-idle-note = 将麦克风回环到输出（用耳机可避免回授）

# TURN relay section
sources-turn-save-failed = 无法保存：{ $error }
sources-turn-summary = 网络 — 可选的 TURN 中继（高级）
sources-turn-note-1 = 会话直接连接（P2P）— 免费，无需中继。如果双方都处于严格 NAT 之后，直连路径可能失败；此时由你自己运行的 TURN 中继来承载媒体。跳过此项没问题 — 大多数连接仅靠直连即可工作。
sources-turn-note-2 = 免费选项：Oracle Cloud "Always Free" 可免费运行 coturn（注意：Oracle 在注册时会要求信用卡，但 Always-Free 规格保持免费）。步骤：1) 创建免费 VM，2) 安装 coturn，3) 开放 UDP 3478，4) 设置用户名/密码，5) 在此输入 turn:your-vm-ip:3478 及凭据。你的凭据保存在本地设置文件中，永不记录。
sources-turn-url-label = TURN 网址
sources-turn-url-placeholder = turn:host:3478（留空 = 仅直连）
sources-turn-url-aria = TURN 网址
sources-turn-username-label = 用户名
sources-turn-username-aria = TURN 用户名
sources-turn-credential-label = 凭据
sources-turn-credential-aria = TURN 凭据
sources-turn-note-3 = 三个字段都填写后中继才会启用（TURN 服务器需要凭据），并应用于你下一次开始或加入的会话。可在你自己的两台机器之间用仅中继的测试通话来验证。
sources-turn-settings-unavailable = 设置不可用（浏览器模式）

# Color picker
sources-color-title = 添加色源
sources-color-label = 颜色
sources-color-width-label = 宽度
sources-color-height-label = 高度
sources-color-add = 添加色源
sources-testsignal-title = 添加测试信号
sources-testsignal-pattern-label = 图案
sources-testsignal-bars = SMPTE 彩条
sources-testsignal-grid = 校准网格
sources-testsignal-sweep = 运动扫描
sources-testsignal-tone = 1 kHz 音调（−20 dBFS）
sources-testsignal-flash-beep = A/V 同步闪光 + 提示音
sources-testsignal-note = 无需接入摄像头即可检查场景、编码器、投影仪和推流目标。闪光 + 提示音图案用于 A/V 同步工作台。
sources-testsignal-add = 添加测试信号
sources-timer-title = 添加计时器
sources-timer-mode-label = 模式
sources-timer-wall-clock = 挂钟
sources-timer-countdown = 倒计时
sources-timer-stopwatch = 秒表
sources-timer-since-live = 开播以来
sources-timer-since-recording = 录制以来
sources-timer-note = 时长、格式、样式和倒计时结束动作都在来源的“属性”中设置。
sources-timer-add = 添加计时器

# Text picker
sources-text-title = 添加文本
sources-text-label = 文本
sources-text-default = 文本
sources-text-color-label = 颜色
sources-text-color-aria = 文本颜色
sources-text-size-label = 大小（px）
sources-text-note = 字体族、对齐、换行和 RTL 在来源的"属性"中设置。内置的 Noto Sans（含阿拉伯语/希伯来语）为默认字体 — 在每台机器上都完全一致。
sources-text-add = 添加文本

# Existing source picker
sources-existing-title = 添加已有来源
sources-existing-empty = 尚无任何来源 — 请先向任意场景添加一个。已有来源是共享的：重命名或重新配置其中一个，会更新显示它的所有场景。

# Screen + corners layout
sources-slot-off = 关闭
sources-slot-center = 中心（屏幕）
sources-slot-top-left = 左上
sources-slot-top-right = 右上
sources-slot-bottom-left = 左下
sources-slot-bottom-right = 右下
sources-layout-title = 排列：屏幕 + 四角
sources-layout-empty = 请先向此场景添加一个屏幕采集和一个或多个摄像头，然后在此排列它们。
sources-layout-note = 把屏幕放在中心，最多四个摄像头放在四角 — 你的讲解 / 播客布局。每个角落可放一个摄像头、一个采集的通话窗口或一段媒体片段。之后你可以在画布上拖动它们中的任意一个。
sources-layout-slot-aria = { $name } 的插槽
sources-layout-apply = 应用布局


# =============================================================
# --- docks ---
# =============================================================
# docks

# --- ControlsDock.tsx ---
controls-title = 控制
controls-start-stop-title-stop = 停止并完成录制
controls-start-stop-title-start = 使用"设置 → 输出"配置录制节目画面
controls-finalizing = ◌ 正在完成…
controls-stop-recording = ■ 停止录制
controls-start-recording = ● 开始录制
controls-marker-title = 在此刻打入一个章节标记 — 它会落入录制文件（mkv 章节或一个附属文件）。平台侧的直播流标记需要平台账户，本应用从不索取。
controls-marker = ◈ 标记
controls-pause-title-resume = 恢复 — 文件将作为一条连续的时间线继续
controls-pause-title-pause = 暂停 — 不写入任何帧；恢复时会继续同一个可播放的文件
controls-resume-recording = ▶ 恢复录制
controls-pause-recording = ⏸ 暂停录制
controls-reactions-label = 反应（烘焙进节目）
controls-reactions-title = 让一个反应漂浮在节目上方 — 同时录制并推流，因此回放会呈现确切的时刻。聊天中的观众也能触发它们（他们的反应表情会自动漂浮）；刷屏只会限制屏幕上的数量。
controls-react = 反应 { $emoji }
controls-virtual-camera-title = 虚拟摄像头需要每个操作系统各自的签名驱动组件（Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO 扩展 / Linux v4l2loopback）— 它作为独立里程碑发布。馈送模型已为其就绪：节目、竖屏画布或单个来源，并在 Windows/Linux 上配有一个配对的虚拟麦克风（macOS 没有虚拟麦克风 API — 如实相告）。
controls-virtual-camera = ⌁ 启动虚拟摄像头
controls-files-title = 已完成的录制 + 重封装为 mp4 的操作
controls-files = ▤ 文件…
controls-output-title = 录制格式、编码器、文件夹、轨道和分段
controls-output = ⚙ 输出…
controls-stream-title = 开始直播的目标：服务、串流密钥、编码器、比特率
controls-stream = ⦿ 推流…
controls-codecs-title = 按需的 ffmpeg 有线编解码器组件（明确标注，从不打包）
controls-codecs = ⬡ 编解码器…
controls-replay-title = 重放缓存长度 + 质量预设
controls-replay = ⟲ 重放…
controls-keys-title = 全局热键：录制、开始直播、转场、保存重放
controls-keys = ⌨ 热键…
controls-scripts-title = 沙盒化的 Lua 脚本：响应开始直播/场景/录制事件，驱动工作室
controls-scripts = ⚡ 脚本…
controls-docks-title = 浏览器停靠窗：将聊天弹窗、提醒页面或 Companion 按钮作为工作室旁的一个窗口打开
controls-docks = ⧉ 停靠窗…
controls-remote-title = 用于 Stream Deck / Companion 控制器的 WebSocket 远程 API（默认关闭）
controls-remote = ⌁ 远程…
controls-profiles-title = 配置文件（设置）+ 场景集合 — 可切换的快照
controls-profiles = ▣ 配置文件…
controls-bug-title = 报告错误 — 匿名、自愿（不会自动发送任何内容）
controls-bug = 🐞 报告错误…
controls-updates-title = 检查更新 — 经过签名和验证，未经点击不会下载任何内容
controls-updates = ⭳ 检查更新…
controls-saved = 已保存：{ $path }

# --- MixerDock.tsx ---
mixer-title = 音频混音器
mixer-monitor-error = 监听：{ $error }
mixer-switch-to-horizontal = 切换到水平混音条
mixer-switch-to-vertical = 切换到垂直混音条
mixer-layout-aria-vertical = 混音器布局：垂直 — 切换到水平
mixer-layout-aria-horizontal = 混音器布局：水平 — 切换到垂直
mixer-empty = 此场景中没有音频源 — 在"来源"中用"+"添加音频输入采集（麦克风）或音频输出采集（桌面音频）。混音条会获得 VU 电平表、推子、静音、监听、滤镜和轨道分配。
mixer-advanced-title = 音频 — { $name }
mixer-loudness-label = 节目响度（LUFS）
mixer-lufs = LUFS
mixer-momentary-title = 瞬时响度（400 毫秒）
mixer-short-term-title = 短期响度（3 秒）
mixer-lufs-short = S { $value }
mixer-monitor-label = 监听
mixer-monitor-device-aria = 监听输出设备
mixer-default-output = 默认输出

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = 内存
stats-dropped = 丢帧
stats-render = 渲染
stats-gpu = GPU
stats-gpu-compositing = 合成中
stats-gpu-idle = 空闲
stats-vertical-fps = 9:16 FPS
stats-targets-label = 推流目标
stats-shared-encode = · 共享编码
stats-starting = 正在启动合成器…

# --- ScenesRail.tsx ---
scenes-title = 场景
scenes-new-scene-name = 场景
scenes-add = 添加场景
scenes-empty = 正在连接工作室内核…
scenes-rename = 重命名 { $name }
scenes-on-program = 处于节目
scenes-preview = 预览 { $name }
scenes-switch-to = 切换到 { $name }
scenes-move-up = 上移
scenes-move-up-aria = 上移 { $name }
scenes-move-down = 下移
scenes-move-down-aria = 下移 { $name }
scenes-last-stays = 最后一个场景会保留
scenes-remove = 移除此场景
scenes-remove-aria = 移除 { $name }


# =============================================================
# --- components ---
# =============================================================
# components

# --- ChannelStrip.tsx ---
channelstrip-level = 电平
channelstrip-monitor-off = 关闭监听
channelstrip-monitor-only = 仅监听（不进入混音）
channelstrip-monitor-and-output = 监听并输出
channelstrip-status-error = 错误
channelstrip-status-live = 运行中
channelstrip-status-waiting-audio = 正在等待音频
channelstrip-status = 状态：{ $state }
channelstrip-status-waiting = 等待中
channelstrip-mute = 静音
channelstrip-unmute = 取消静音
channelstrip-mute-source = 将 { $name } 静音
channelstrip-unmute-source = 取消 { $name } 的静音
channelstrip-scene-mix-on = 逐场景混音已开启 — 此混音条为该场景覆盖全局混音（点击可重新跟随全局混音）
channelstrip-scene-mix-off = 逐场景混音 — 为当前场景赋予此混音条独立的推子/静音
channelstrip-scene-mix-label = { $name } 的逐场景混音
channelstrip-monitor-cycle = { $mode } — 点击循环切换
channelstrip-monitor-mode = { $name } 的监听模式：{ $mode }
channelstrip-audio-filters-title = 音频滤镜（降噪、门限、压缩器…）
channelstrip-audio-filters-label = { $name } 的音频滤镜
channelstrip-advanced-title = 同步偏移与一键说话热键
channelstrip-advanced-label = { $name } 的高级音频设置
channelstrip-track-assignment = 轨道分配
channelstrip-track = 轨道 { $n }
channelstrip-track-assigned = 轨道 { $n }（已分配）
channelstrip-track-label = { $name } 的轨道 { $n }
channelstrip-device-error = 设备错误
channelstrip-audio-device-error = 音频设备错误
channelstrip-volume-label = { $name } 的音量（分贝）
channelstrip-ptt-hold = 一键说话：按住 { $key }
channelstrip-sync-offset = 同步偏移（毫秒，0–{ $max } — 延迟此音频）
channelstrip-solo-title = 独听（PFL）— 监听只播放被独听的通道；节目混音不受影响
channelstrip-solo-source = 独听 { $name }（PFL）
channelstrip-pan-label = 声像平衡（双击复位）
channelstrip-pan-aria = { $name } 的声像平衡
channelstrip-mono-label = 下混为单声道
channelstrip-ptt-hotkey = 一键说话热键（不按住时静音）
channelstrip-ptt-placeholder = 例如 Ctrl+Shift+T 或 F13
channelstrip-ptt-aria = 一键说话热键
channelstrip-ptm-hotkey = 一键静音热键（按住时静音）
channelstrip-ptm-placeholder = 例如 Ctrl+Shift+M
channelstrip-ptm-aria = 一键静音热键
channelstrip-hotkeys-note = 热键在其他应用处于焦点时也有效。在 Linux/Wayland 上，全局热键可能不可用 — 这是合成器的限制，如实相告。
channelstrip-apply = 应用


# --- LiveButton.tsx ---
livebutton-failure-ended = 直播流已结束
livebutton-title-live = 结束直播 — 所有目标（正在进行的录制会继续）
livebutton-title-offline = 向"设置 → 推流"中每个已启用的目标开始直播
livebutton-end-stream = ■ 结束直播
livebutton-aria-reconnecting = 正在重新连接
livebutton-aria-live = 直播中
livebutton-badge-retry = 重试 { $n }
livebutton-badge-live = 直播中
livebutton-go-live = ⦿ 开始直播


# --- RecDot.tsx ---
recdot-paused-aria = 录制已暂停
recdot-recording-aria = 正在录制
recdot-tracks-one = 正在录制 { $count } 条音频轨道
recdot-tracks-other = 正在录制 { $count } 条音频轨道
recdot-paused = 已暂停


# --- ReplayControls.tsx ---
replaycontrols-saved = 重放已保存 — { $name }
replaycontrols-failure-stopped = 缓存已停止
replaycontrols-title-disarm = 解除重放缓存（丢弃未保存的历史）
replaycontrols-title-arm = 启用滚动重放缓存 — 随时保留最后 N 秒以便保存（拥有自己的轻量编码；直播流和录制不受影响）
replaycontrols-replay-seconds = ⟲ 重放 { $seconds } 秒
replaycontrols-arm = ⟲ 启用重放缓存
replaycontrols-save-title = 将最后 N 秒保存到录制文件夹（保存重放热键同样有效）
replaycontrols-save = ⤓ 保存


# --- PropertiesDialog.tsx ---
properties-title = 属性 — { $name }
properties-name = 名称
properties-cancel = 取消
properties-apply = 应用
properties-youtube = YouTube — 频道 / 观看 / live_chat 网址（永远无需密钥，无需登录）
properties-twitch = Twitch — 频道名称（匿名）
properties-kick = Kick — 频道 slug（公开端点）
properties-width-px = 宽度（px）
properties-lines = 行数
properties-font-px = 字体（px）
properties-images = 图像文件（每行一个路径，按顺序显示）
properties-per-slide = 每张幻灯片（毫秒）
properties-crossfade = 交叉淡化（毫秒，0 = 直切）
properties-loop-slideshow = 循环（关闭 = 停留在最后一张）
properties-shuffle = 每轮随机播放
properties-nested-scene = 此来源合成的场景（已包含此来源的场景会被拒绝）
properties-portal-note = Wayland ScreenCast 门户会在此来源每次启动时于系统对话框中选择屏幕或窗口 — 依设计，这里没有可配置项。
properties-appaudio-capturing = 正在从 { $exe } 采集音频
properties-appaudio-exe-fallback = 一个应用程序
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = 重新添加来源以针对不同的应用（应用重启时进程 id 会变化）。
properties-image-file = 图像文件
properties-media-file = 媒体文件（mp4、mkv、webm、mov、.frec 或图像）
properties-media-loop = 循环（播放到结尾后从头重新开始）
properties-media-hwdecode = 硬件解码（会自行回退到软件解码）
properties-media-note = .frec 通过自有的 freally-video 编解码器播放 — 无需下载。其他视频格式通过按需的 FFmpeg 组件解码。文件的音频会获得独立的混音条；混音条的同步偏移可微调 A/V 对齐。没有音频的片段会让其混音条保持静默。
properties-color = 颜色
properties-width = 宽度
properties-height = 高度
properties-testtone-note = 连续的 1 kHz 正弦波，电平为 −20 dBFS。音量与静音在其调音台通道上控制；无其他可配置项。
properties-timer-format = 时间格式（strftime）
properties-timer-format-note = 如 %H:%M:%S（默认）、%I:%M %p、%A %H:%M — 无效格式会回退到 %H:%M:%S。
properties-timer-utc = UTC 偏移（分钟）
properties-timer-utc-placeholder = 本地时间
properties-timer-duration = 时长（秒）
properties-timer-target = 倒计时至（HH:MM）
properties-timer-target-note = 挂钟目标自动运行并每天重复；留空则使用时长并配合开始/暂停/重置。
properties-timer-end = 归零时
properties-timer-end-none = 什么都不做
properties-timer-end-flash = 闪烁计时器
properties-timer-end-switch = 切换场景
properties-timer-end-scene = 场景
properties-timer-size = 大小（px）
properties-timer-start = 开始
properties-timer-pause = 暂停
properties-timer-reset = 重置
properties-text-file = 从文件读取（路径；留空 = 使用上方文本）
properties-text-binding = 解析为
properties-text-binding-whole = 整个文件
properties-text-binding-csv = CSV 单元格
properties-text-binding-json = JSON 指针
properties-text-csv-row = 行
properties-text-csv-column = 列
properties-text-csv-column-placeholder = 名称或序号
properties-text-json-pointer = 指针
properties-text-file-note = 文件变化后约半秒内会重新读取。可容忍原子写入（临时文件 + 重命名）：切换期间屏幕上保留最后一个有效值。
avsync-title = A/V 同步校准
avsync-intro = 通过显示器和音箱播放内置的闪光 + 提示音图案，用要对齐的摄像头和麦克风把它拍回来，工作台就能测出两者的差距。回路经过屏幕和音箱，因此它们的少量延迟也包含在内。
avsync-video-label = 摄像头（视频源）
avsync-audio-label = 麦克风（音频源）
avsync-pick = 选择来源…
avsync-no-video = 请先把摄像头添加为来源 — 工作台测量的是来源，不是裸设备。
avsync-no-audio = 请先把麦克风添加为音频来源。
avsync-projector = 在以下显示器全屏节目
avsync-projector-open = 打开投影窗口
avsync-projector-window-title = 节目 — A/V 同步
avsync-start-note = 开始时会在当前场景顶层临时添加一个“A/V 同步图案”来源，并在监听设备上播放提示音。结束后全部移除。
avsync-manual = 同步偏移（ms，手动）
avsync-start = 开始校准
avsync-measuring = 测量约 12 秒 — 让摄像头对准闪烁的节目，并保持房间安静…
avsync-flash-seen = 摄像头看到了闪光
avsync-flash-waiting = 等待摄像头看到闪光…
avsync-beep-heard = 麦克风听到了提示音
avsync-beep-waiting = 等待麦克风听到提示音…
avsync-cancel = 取消
avsync-result-offset = 视频比音频晚 { $offset } ms 到达。
avsync-result-detail = 基于 { $cycles } 个周期测得，±{ $jitter } ms。
avsync-negative = 音频本来就比视频晚。再延迟音频无法修正这个方向 — 若这台摄像头的声音走另一条通道，请到那里调低其偏移。
avsync-over-cap = 测得的差距超出 { $max } ms 的同步偏移上限。这么大的差距通常意味着选错了来源 — 检查链路后重新测量。
avsync-applied = 已应用 — 麦克风的同步偏移现为 { $offset } ms。
avsync-apply = 将 { $offset } ms 应用到麦克风
avsync-again = 重新测量
avsync-close = 关闭
avsync-error-noFlash = 摄像头始终没有看到闪光。请让它对准闪烁的节目（全屏更好），确认来源处于活动状态后重测。
avsync-error-noBeep = 麦克风始终没有听到提示音。请确认监听设备可以出声、麦克风处于活动状态（未被按键说话拦住），然后重测。
avsync-error-tooFewCycles = 干净的闪光/提示音周期不够。请让图案在整个测量期间清晰可见、可闻。
avsync-error-notThePattern = 看到或听到的内容没有按图案的节奏重复 — 多半是房间的灯光或噪声，不是测试信号。
avsync-error-unstable = 各周期相差太大，无法给出可信的单一数值。固定摄像头、降低噪声后重测。
hotkey-audit-title = 快捷键总览
hotkey-audit-search = 搜索
hotkey-audit-filter = 功能
hotkey-audit-filter-all = 全部功能
hotkey-audit-col-key = 按键
hotkey-audit-col-action = 动作
hotkey-audit-col-where = 位置
hotkey-audit-col-status = 状态
hotkey-audit-ok = 正常
hotkey-audit-shared = { $count } 个绑定共用
hotkey-audit-unregistered = 未在系统注册（被其他程序占用或不可用）
hotkey-audit-invalid = 不是有效的快捷键
hotkey-audit-empty = 还没有快捷键 — 在设置 → 快捷键或调音台通道上绑定。
hotkey-audit-export = 导出速查表
hotkey-audit-exported = 已保存到 { $path }
hotkey-audit-note = 按键的绑定与修改在设置 → 快捷键（全局动作）和每个调音台通道（按键说话 / 按键静音）中完成；此表负责审计与记录。
hotkey-audit-action-record = 切换录制
hotkey-audit-action-go-live = 切换推流
hotkey-audit-action-transition = 执行转场
hotkey-audit-action-save-replay = 保存回放
hotkey-audit-action-add-marker = 添加标记
hotkey-audit-action-still = 截取静帧
hotkey-audit-action-panic = 紧急画面
hotkey-audit-action-timer-toggle = 开始/暂停所有计时器
hotkey-audit-action-timer-reset = 重置所有计时器
hotkey-audit-action-ptt = 按键说话
hotkey-audit-action-ptm = 按键静音
hotkey-audit-feature-recording = 录制
hotkey-audit-feature-streaming = 推流
hotkey-audit-feature-studio = 演播模式
hotkey-audit-feature-replay = 回放
hotkey-audit-feature-markers = 标记
hotkey-audit-feature-stills = 静帧
hotkey-audit-feature-panic = 紧急
hotkey-audit-feature-timers = 计时器
hotkey-audit-feature-audio = 音频（按来源）
properties-text = 文本
properties-font-family = 字体族（系统；留空 = 默认）
properties-size-px = 大小（px）
properties-text-color = 文本颜色
properties-align = 对齐
properties-align-left = 左对齐
properties-align-center = 居中
properties-align-right = 右对齐
properties-line-spacing = 行距
properties-wrap-width = 换行宽度（px；0 = 关闭）
properties-force-rtl = 强制从右到左
properties-text-note = 渲染使用真正的字形整形（阿拉伯语连写、连字）和双向文本行序。内置的 Noto Sans 字体族（含阿拉伯语/希伯来语）为默认；系统字体族也可使用。CJK 目前使用系统字体。
properties-repick-capturing = 正在采集：{ $label }
properties-repick-looking = 正在查找来源…
properties-repick-none-displays = 未找到可重新选择的显示器。
properties-repick-none-windows = 未找到可重新选择的窗口。
properties-repick-again = 重新选择：
properties-device = 设备
properties-video-current-device = （当前设备）
properties-format = 格式
properties-format-auto-loading = 自动（正在加载格式…）
properties-deinterlace = 反交错
properties-deinterlace-off = 关闭
properties-deinterlace-discard = 丢弃（单场行倍增）
properties-deinterlace-bob = Bob（场交替）
properties-deinterlace-linear = 线性（插值）
properties-deinterlace-blend = 混合（两场平均）
properties-deinterlace-adaptive = 运动自适应（yadif 类）
properties-field-order = 场序
properties-field-order-top = 上场优先
properties-field-order-bottom = 下场优先
properties-deinterlace-note = 用于隔行的采集卡信号。纯 CPU 处理，各系统一致；更改会重启设备（同更改格式）。
camera-controls-title = 摄像头控制
camera-controls-refresh = 刷新
camera-controls-reset = 重置配置
camera-controls-empty = 当前没有可用控制 — 设备必须正在推流（请先加入场景），且部分后端不提供任何控制（尤其是 macOS）。这是各系统的真实情况。
camera-controls-note = 更改即时生效并保存到该设备的配置中，重新插拔与重启后会自动重新应用。
camera-control-brightness = 亮度
camera-control-contrast = 对比度
camera-control-hue = 色调
camera-control-saturation = 饱和度
camera-control-sharpness = 锐度
camera-control-gamma = 伽马
camera-control-white-balance = 白平衡
camera-control-backlight = 背光补偿
camera-control-gain = 增益
camera-control-pan = 水平移动
camera-control-tilt = 俯仰
camera-control-zoom = 变焦
camera-control-exposure = 曝光
camera-control-iris = 光圈
camera-control-focus = 对焦
properties-format-auto = 自动（最高分辨率）
properties-audio-capture-of = 采集以下对象的音频
properties-audio-default-output = 默认输出（你听到的声音）
properties-audio-default-input = 默认输入
properties-audio-default-suffix = （默认）
properties-audio-current-device = （当前设备：{ $id }）


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = 增益
audiofilters-name-noise-gate = 噪声门限
audiofilters-name-compressor = 压缩器
audiofilters-name-limiter = 限制器
audiofilters-name-eq = 三段均衡器
audiofilters-name-denoise = 降噪
audiofilters-name-ducking = 音量闪避
audiofilters-title = 音频滤镜 — { $name }
audiofilters-chain-header = 滤镜链（顶部先运行，在推子之前）
audiofilters-add = + 添加滤镜
audiofilters-add-menu = 添加音频滤镜
audiofilters-empty = 尚无滤镜 — 为麦克风降噪（经典 DSP，无 ML）、对房间噪声设门限、用压缩器压制峰值，或让音乐在你说话时闪避。
audiofilters-enable = 启用 { $name }
audiofilters-run-earlier = 更早运行
audiofilters-move-up = 上移 { $name }
audiofilters-run-later = 更晚运行
audiofilters-move-down = 下移 { $name }
audiofilters-remove-title = 移除滤镜
audiofilters-remove = 移除 { $name }
audiofilters-gain-db = 增益（dB）
audiofilters-open-db = 开启阈值（dB）
audiofilters-close-db = 关闭阈值（dB）
audiofilters-attack-ms = 启动时间（毫秒）
audiofilters-hold-ms = 保持时间（毫秒）
audiofilters-release-ms = 释放时间（毫秒）
audiofilters-ratio = 压缩比（:1）
audiofilters-threshold-db = 阈值（dB）
audiofilters-output-gain-db = 输出增益（dB）
audiofilters-ceiling-db = 上限（dB）
audiofilters-low-db = 低频（dB）
audiofilters-mid-db = 中频（dB）
audiofilters-high-db = 高频（dB）
audiofilters-strength = 强度
audiofilters-denoise-note = 自有的经典 DSP 频谱抑制 — 稳定噪声（风扇、嘶声）被压低，而语音得以通过。依照章程，无 ML，无模型。
audiofilters-duck-under = 闪避于其下
audiofilters-ducking-trigger = 闪避触发源
audiofilters-pick-trigger = （选择一个触发源 — 例如你的麦克风）
audiofilters-trigger-at-db = 触发阈值（dB）
audiofilters-duck-by-db = 闪避量（dB）


# --- FiltersDialog.tsx ---
filters-name-chroma-key = 色度键
filters-name-color-key = 颜色键
filters-name-luma-key = 亮度键
filters-name-render-delay = 渲染延迟
filters-name-color-correction = 色彩校正
filters-name-lut = 应用 LUT
filters-name-blur = 模糊
filters-name-mask = 图像遮罩
filters-name-sharpen = 锐化
filters-name-scroll = 滚动
filters-name-crop = 裁剪
filters-title = 滤镜 — { $name }
filters-blend-mode = 混合模式
filters-chain-header = 滤镜链（顶部先运行）
filters-add = + 添加滤镜
filters-add-menu = 添加滤镜
filters-empty = 尚无滤镜 — 为摄像头做色度键、为采集画面做色彩校正，或滚动一条字幕。
filters-enable = 启用 { $name }
filters-run-earlier = 更早运行
filters-move-up = 上移 { $name }
filters-run-later = 更晚运行
filters-move-down = 下移 { $name }
filters-remove-title = 移除滤镜
filters-remove = 移除 { $name }
filters-key-color-rgb = 键控颜色（任意颜色，RGB 距离）
filters-similarity = 相似度
filters-smoothness = 平滑度
filters-luma-min = 亮度下限（更暗的部分被键出）
filters-luma-max = 亮度上限（更亮的部分被键出）
filters-delay = 延迟（毫秒 — 仅视频，例如用于与音频同步；上限 500）
filters-key-color = 键控颜色
filters-spill = 溢色
filters-gamma = 伽马
filters-brightness = 亮度
filters-contrast = 对比度
filters-saturation = 饱和度
filters-hue-shift = 色相偏移
filters-opacity = 不透明度
filters-cube-file = .cube 文件
filters-amount = 数量
filters-radius = 半径
filters-mask-image = 遮罩图像
filters-mask-mode = 模式
filters-mask-alpha = alpha
filters-mask-luma = 亮度
filters-mask-invert = 反转
filters-speed-x = 速度 X（px/s）
filters-speed-y = 速度 Y（px/s）
filters-crop-left = 左
filters-crop-top = 上
filters-crop-right = 右
filters-crop-bottom = 下
filters-crop-aria = 裁剪 { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = 刷新
pickershell-refresh-title = 刷新列表
pickershell-close = 关闭


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs

# --- BugReport.tsx ---
bugreport-title = 报告错误
bugreport-intro = 报告是匿名且自愿的 — 不会自动发送任何内容。你会先审阅下方的确切文本，然后通过预填的 GitHub issue 或你的邮件应用提交。不含个人数据（你的主目录路径和用户名已隐去）；无需账户，无需服务器。
bugreport-crash-notice = Freally Capture 在上一次运行中意外关闭 — 下方包含匿名的崩溃详情。报告它们有助于快速修复。
bugreport-description-label = 发生时你在做什么？（可选）
bugreport-description-placeholder = 例如：添加第二个摄像头时预览卡住了
bugreport-include-crash = 包含上次运行的匿名崩溃详情
bugreport-preview-label = 将确切发送的内容
bugreport-open-github = 打开 GitHub issue
bugreport-gmail-title = 在浏览器中打开 Gmail 的撰写窗口，并预先填好。未登录？Google 会先显示其登录界面。
bugreport-compose-gmail = 在 Gmail 中撰写
bugreport-email-title = 在此电脑默认使用的邮件应用（Outlook、Thunderbird、Mail…）中打开一封草稿
bugreport-send-email = 发送邮件
bugreport-copied = 已复制 ✓
bugreport-copy-report = 复制报告
bugreport-dismiss-crash = 忽略崩溃
bugreport-copy-failed = 无法复制 — 请选中文本并手动复制
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = 发生了什么
bugreport-preview-no-description = （未提供描述）
bugreport-preview-diagnostics = 匿名诊断（无个人数据）
bugreport-preview-from = 来自：Freally Capture
bugreport-preview-crash-excerpt = --- 崩溃摘录 ---


# --- Updates.tsx ---
updates-title = 软件更新
updates-checking = 正在检查更新…
updates-uptodate = 你已是最新版本。
updates-check-again = 再次检查
updates-available = 版本 { $version } 可用
updates-current-version = （你当前为 { $current }）
updates-release-notes-label = 版本 { $version } — 发行说明
updates-confirm = 是否立即更新？下载会在应用前用内置签名密钥进行验证。Freally Capture 会关闭，安装程序运行，新版本随后自行重新打开。
updates-yes-update-now = 是，立即更新
updates-no-not-now = 否，暂不
updates-downloading = 正在下载 { $version }…
updates-starting = 正在启动…
updates-installed = 更新已安装。
updates-restart-now = 立即重启
updates-restart-later = 稍后重启
updates-try-again = 重试


# --- Models.tsx ---
models-title = 组件
models-ffmpeg-heading = FFmpeg — 有线编解码器
models-badge-third-party = 第三方 · 未打包
models-ffmpeg-desc = Freally Capture 自有的引擎无需任何额外内容即可录制无损的 freally-video（.frec）。而录制平台和播放器所期望的有线格式 — mp4/mkv/mov/webm 中的 H.264/AAC（以及 HEVC/AV1）— 使用 FFmpeg，一个本应用从不随附的独立工具：这些编解码器受专利约束，因此它保持可选并明确标注。它会按需从下方固定的构建下载，首次使用前经 SHA-256 验证，按用户缓存，并作为独立进程驱动。其许可证（LGPL/GPL）自成一体 — 见 THIRD-PARTY-NOTICES。
models-checking = 正在检查…
models-ffmpeg-not-installed = 未安装。可用：来自 { $source } 的 FFmpeg { $version }（{ $size } 下载）。
models-ffmpeg-none-pinned = 此平台尚未固定任何 FFmpeg 构建 — 有线编解码器录制在此不可用。无损 freally-video 录制不受影响。
models-ffmpeg-download-verify = 下载并验证（{ $size }）
models-downloading = 正在下载…
models-download-of = /
models-cancel = 取消
models-ffmpeg-verifying = 正在对照固定的 SHA-256 验证下载…
models-ffmpeg-extracting = 正在解包…
models-ffmpeg-ready = 已安装并验证 — { $version }
models-remove = 移除
models-ffmpeg-retry = 重试下载
models-network-note = 下载是此面板上唯一的网络操作，且从不自行开始。校验和失败会中止安装 — 应用拒绝运行它无法担保的字节。
models-cef-heading = 浏览器来源运行时 — Chromium（CEF）
models-cef-desc = 浏览器来源通过 Chromium Embedded Framework 渲染网页（提醒、小组件、叠加）— 一个约 100 MB 的运行时，本应用从不随附。它按需从官方 CEF 构建索引下载，在解包任何内容前对照该索引的 SHA-1 验证，并按用户缓存。通过它渲染的浏览器来源随其自身里程碑到来；此处安装的是它所需的运行时。
models-cef-download-install = 下载并安装
models-cef-unsupported = CEF 未为此平台发布构建 — 浏览器来源在此不可用。
models-cef-resolving = 正在解析最新的稳定构建…
models-cef-verifying = 正在对照索引 SHA-1 验证下载…
models-cef-extracting = 正在解包运行时…
models-cef-ready = 已安装 — CEF { $version }。
models-cef-retry = 重试
models-integrations-heading = 可选集成
models-badge-never-bundled = 从不打包
models-ndi-detected = 已检测到
models-ndi-not-installed = 未安装
models-vst-available = 可用
models-vst-not-available = 不可用


# --- Recordings.tsx ---
recordings-title = 录制
recordings-loading = 正在读取文件夹…
recordings-empty = 尚无录制 —"开始录制"会写入"输出"中设定的文件夹。
recordings-frec-label = 自有无损（freally-video）
recordings-remux-title = 重封装为 mp4 — 流复制，不重新编码，不改变质量（需要 FFmpeg 组件）
recordings-remuxing = 正在重封装…
recordings-remux-to-mp4 = 重封装为 MP4
recordings-export-mp4-title = 解码自有的 .frec 并重新编码为 MP4（H.264/AAC），使其能在任何播放器中播放 — 需要 FFmpeg 组件
recordings-exporting = 正在导出…
recordings-export-mp4 = 导出 → MP4
recordings-export-mkv-title = 解码自有的 .frec 并重新编码为 MKV，使其能在任何播放器中播放
recordings-starting = 正在启动…
recordings-frames = { $done } / { $total } 帧
recordings-cancel = 取消
recordings-export-cancelled = 导出已取消。
recordings-exported-to = 已导出到 { $path }
recordings-remuxed-to = 已重封装到 { $path }


# --- OpenedFrec.tsx ---
openfrec-title = 打开 .frec 录制
openfrec-desc = Freally Capture 录制自有的无损 .frec 格式 — 但不播放它。Freally Player 发布后将直接播放 .frec。目前，将其导出为 MP4/MKV，即可在任何播放器（VLC、你的操作系统播放器等）中播放。
openfrec-exported-to = 已导出到 { $path }
openfrec-exporting = 正在导出…
openfrec-starting = 正在启动…
openfrec-export-mp4 = 导出 → MP4
openfrec-export-mkv = 导出 → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = 竖屏画布（9:16）
vertical-enable = 启用第二个画布 — 可独立于节目录制和推流
vertical-scene-label = 此画布合成的场景
vertical-width = 宽度
vertical-height = 高度
vertical-preview-alt = 竖屏画布预览
vertical-note = 项目位置在各画布间像素精确：在"场景"侧栏中选择此场景即可排列它，同时此预览会显示竖屏结果。推流目标在 ⦿ 推流… 中选择此画布；"设置 → 输出"可在录制主文件的同时录制它。
vertical-close = 关闭


# --- EulaGate.tsx ---
eula-title = Freally Capture — 许可协议
eula-version = v{ $version }
eula-intro = 请阅读并接受本协议以使用 Freally Capture。简而言之：它是一个中立的工具，你对你所采集、录制和广播的内容 — 以及对拥有相应权利 — 负全部责任。
eula-thanks = 感谢阅读。
eula-scroll-hint = 滚动到末尾以继续。
eula-decline = 拒绝并退出
eula-agree = 我同意


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = 输出
output-loading = 设置仍在加载…
output-container-frec = freally-video（.frec）— 无损、自有、无需下载
output-container-mkv = MKV — 抗崩溃；之后可重封装为 mp4
output-container-mp4 = MP4 — 到处都能播放
output-container-mov = MOV
output-container-webm = WebM（AV1 + Opus）
output-preset-lossless-label = 无损
output-preset-lossless-title = 自有的 freally-video 编解码器 — 逐位精确，无需下载
output-preset-high-label = 高质量
output-preset-high-title = MP4，检测到的最佳编码器，接近无损 CQ 16，质量预设
output-preset-balanced-label = 均衡
output-preset-balanced-title = MKV，检测到的最佳编码器，CQ 23，均衡预设
output-recording-format = 录制格式
output-ffmpeg-warning = 此格式需要 FFmpeg 组件（有线编解码器 — 未打包）。无损 .frec 无需任何内容。
output-install = 安装…
output-recordings-folder = 录制文件夹
output-folder-placeholder = 操作系统的"视频"文件夹
output-filename-prefix = 文件名前缀
output-recording-template = 录制文件名
output-replay-template = 重放文件名
output-still-template = 静帧文件名
output-template-tokens = 占位符： {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = 重放文件夹
output-still-folder = 静帧文件夹
output-same-folder-placeholder = 录制文件夹
output-frame-rate = 帧率
output-fps-option = { $fps } fps
output-split-every = 分段间隔（分钟，0 = 关闭）
output-output-width = 输出宽度（0 = 画布；仅有线格式）
output-output-height = 输出高度（0 = 画布）
output-record-vertical = 同时录制竖屏画布（一个并行的"…（竖屏）"文件；需要启用 9:16 画布）
output-audio-tracks = 音频轨道
output-recorded-tracks-group = 录制的轨道
output-track-last-one = 至少需要录制一条轨道
output-record-track-on = 录制轨道 { $index }：开
output-record-track-off = 录制轨道 { $index }：关
output-encoder-heading = 编码器
output-video-encoder = 视频编码器
output-encoder-auto = 自动 — 检测到的最佳（H.264）
output-encoder-unavailable = — 在此不可用
output-preset = 预设
output-preset-quality = 质量
output-preset-balanced-option = 均衡
output-preset-performance = 性能
output-rate-control = 码率控制
output-rc-cqp = CQP（恒定质量）
output-rc-cbr = CBR（恒定比特率）
output-rc-vbr = VBR（可变比特率）
output-cq = CQ（0–51，越低越好）
output-bitrate = 比特率（kbps）
output-keyframe = 关键帧间隔（秒）
output-audio-bitrate = 音频比特率（kbps / 轨道）
output-presets = 预设：

# --- SettingsStream.tsx ---
stream-title = 设置 — 推流
stream-target-enabled = 目标 { $index } 已启用
stream-target = 目标 { $index }
stream-remove = 移除
stream-service = 服务
stream-canvas = 画布
stream-canvas-main = 主（节目）
stream-canvas-vertical = 竖屏（9:16 — 在工作室中启用它）
stream-ingest-srt = SRT 接入 URL
stream-ingest-whip = WHIP 端点 URL
stream-ingest-url = 接入 URL
stream-ingest-override = （覆盖 — 留空 = 服务预设）
stream-key-srt = streamid（可选 — 作为 ?streamid=… 附加；视为机密）
stream-key-whip = Bearer 令牌（可选 — 作为 Authorization 头发送；机密）
stream-key-custom = 串流密钥（来自你的服务器 — 视为机密）
stream-key-service = 串流密钥（来自你的创作者面板 — 视为机密）
stream-key-aria = 串流密钥 { $index }
stream-key-hide = 隐藏
stream-key-show = 显示
stream-encoder = 编码器（H.264 — RTMP、SRT 和 WHIP 都承载它）
stream-encoder-auto = 自动 — 检测到的最佳 H.264 编码器
stream-encoder-unavailable = （在此不可用）
stream-video-bitrate = 视频比特率（kbps，CBR）
stream-audio-bitrate = 音频比特率（kbps）
stream-fps = FPS
stream-keyframe = 关键帧间隔（秒）
stream-audio-track = 音频轨道（1–6）
stream-output-width = 输出宽度（0 = 画布）
stream-output-height = 输出高度（0 = 画布）
stream-add-target = + 添加目标
stream-go-live-note = 开始直播会同时向每个已启用的目标发布，直连各平台。编码器设置相同的目标会共享单次编码。
stream-auto-record = 我开始直播时开始录制（录制仍可独立停止）
stream-ffmpeg-note-before = 推流的有线编解码器通过标注的按需 ffmpeg 组件运行 —
stream-ffmpeg-note-link = 在此管理
stream-ffmpeg-note-after = 。无论直播流发生什么，本地录制都会继续运行。
stream-cancel = 取消
stream-save = 保存

# --- SettingsReplay.tsx ---
replay-title = 设置 — 重放缓存
replay-length-15s = 15 秒
replay-length-30s = 30 秒
replay-length-1min = 1 分钟
replay-length-2min = 2 分钟
replay-length-5min = 5 分钟
replay-quality-low = 低（3 Mbps）
replay-quality-standard = 标准（6 Mbps）
replay-quality-high = 高（12 Mbps）
replay-length-presets = 长度预设
replay-quality-presets = 质量预设
replay-length-seconds = 长度（秒）
replay-video-bitrate = 视频比特率（kbps）
replay-fps = FPS
replay-audio-track = 音频轨道（1–6）
replay-note = 启用时，缓存会将自己的轻量编码写入一个有界的磁盘环形缓冲 — 在这些设置下约 { $mb } MB。保存会在不重新编码的情况下拼接该环形缓冲，且从不触及直播流或录制。更改会在你下次启用时生效。
replay-cancel = 取消
replay-save = 保存

# --- SettingsRemote.tsx ---
remote-title = 设置 — 远程控制
remote-enable = 启用 WebSocket 远程 API
remote-password = 密码（必填 — 控制器以此进行认证）
remote-password-placeholder = 给你的控制器设置一个密码
remote-password-hide = 隐藏
remote-password-show = 显示
remote-port = 端口
remote-allow-lan = 允许 LAN 连接（默认仅限本机）
remote-note = 关 = 端口关闭。开 = 在 127.0.0.1（或选择开放时你的 LAN）上一个受密码保护的 WebSocket，可切换场景、执行转场、开始/停止直播流和录制、保存重放，以及设置静音/音量 — 与界面相同的操作，仅此而已。它无法读取文件。请像对待任何凭据一样对待该密码；除非你确实要从其他设备控制，否则优先仅限本机。
remote-password-required = 启用远程 API 需要密码。
remote-cancel = 取消
remote-save = 保存

# --- SettingsHotkeys.tsx ---
hotkeys-title = 设置 — 热键
hotkeys-record = 开始 / 停止录制
hotkeys-record-placeholder = 例如 Ctrl+Shift+R
hotkeys-go-live = 开始直播 / 结束直播
hotkeys-go-live-placeholder = 例如 Ctrl+Shift+L
hotkeys-transition = 工作室模式转场
hotkeys-transition-placeholder = 例如 Ctrl+Shift+T 或 F13
hotkeys-save-replay = 保存重放（最后 N 秒）
hotkeys-save-replay-placeholder = 例如 Ctrl+Shift+S
hotkeys-add-marker = 打入章节标记（录制）
hotkeys-add-marker-placeholder = 例如 Ctrl+Shift+K
hotkeys-note = 热键是全局的 — 在其他应用处于焦点时也会触发。留空 = 未绑定。混音器的一键说话/静音键在每个混音条的 ⋯ 菜单中。在 Linux/Wayland 上，全局热键可能不可用（合成器限制）— 按钮仍可正常工作。
hotkeys-cancel = 取消
hotkeys-save = 保存

# --- WorkspaceDialog.tsx ---
workspace-title = 配置文件与场景集合
workspace-profiles = 配置文件
workspace-profiles-hint = 配置文件即你的设置 — 推流目标、输出、热键。可按节目或平台切换。
workspace-collections = 场景集合
workspace-collections-hint = 集合即你的场景 + 来源。"创建"会以当前集合为起点复制一份。
workspace-active = 活动
workspace-switch-to = 切换到 { $name }
workspace-active-marker = ● 活动
workspace-new-name-placeholder = 新名称…
workspace-new-name-label = 新的 { $title } 名称
workspace-create = 创建

# --- OBS import (CAP-M02) ---
workspace-import-obs = 从 OBS 导入…
workspace-import-obs-hint = 导入一个 OBS 场景集合（其 scenes.json）。当前集合会先被保存。
workspace-import-busy = 正在导入…
workspace-import-title = 已导入"{ $name }"
workspace-import-summary = { $scenes } 个场景 · { $sources } 个源 · { $items } 个项目
workspace-import-dismiss = 关闭
workspace-import-clean = 全部顺利导入。
workspace-import-geometry-caveat = 大小和位置根据 OBS 布局进行适配——请检查每个场景，并重新选择采集设备。
workspace-import-notes-title = 导入（含提示）
workspace-import-skipped-title = 未导入
import-note-needsReselect = 重新选择设备/显示器/窗口
import-note-gameCaptureAsWindow = 游戏捕获 → 窗口捕获
import-note-referencesFile = 检查文件路径
import-note-filterDropped = 部分滤镜不受支持
import-note-geometryApproximated = 位置/大小为近似值
import-skip-unsupportedKind = 无对应的源类型
import-skip-group = 暂不支持分组

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = 重新链接缺失文件…
doctor-title = 缺失文件
doctor-scanning = 正在扫描…
doctor-all-good = 所有引用的文件都存在，无需重新链接。
doctor-intro = 在这台电脑上找不到 { $count } 个被引用的文件。为每个指定新位置——使用它的每个场景都会一次性修复。
doctor-relinked = 已重新链接 { $count } 处引用。
doctor-uses = 使用 { $count } 次
doctor-locate = 定位…
doctor-locate-folder = 在文件夹中查找…
doctor-locate-folder-hint = 选择一个文件夹；每个缺失文件按名称匹配并重新链接。
doctor-kind-image = 图像
doctor-kind-media = 媒体
doctor-kind-slideshow = 幻灯片
doctor-kind-font = 字体
doctor-kind-lut = LUT
doctor-kind-mask = 遮罩
history-relinkFiles = 重新链接文件

# --- ScriptsDialog.tsx ---
scripts-title = 脚本（Lua）
scripts-empty = 尚无脚本 — 添加一个 .lua 文件。API 见 scripts/sample.lua：响应开始直播/场景/录制事件，并驱动与远程 API 相同的命令。
scripts-enable = 启用 { $path }
scripts-remove = 移除 { $path }
scripts-path-label = 脚本路径
scripts-add = 添加
scripts-note = 脚本在沙盒中运行 — 无文件或操作系统访问权限；它们只能调用与远程 API 相同的工作室命令（切换场景、转场、录制/推流/重放、静音）。脚本错误会被记录并隔离。更改会在一秒内生效。
scripts-error-not-lua = 请指向一个 .lua 文件。

# --- BrowserDock.tsx ---
browser-dock-title = 浏览器停靠窗
browser-dock-empty = 尚无停靠窗 — 添加一个聊天弹窗、一个提醒页面，或你的 Companion 网页按钮。
browser-dock-open = 打开
browser-dock-remove = 移除 { $name }
browser-dock-name-placeholder = 名称（例如 Twitch Chat）
browser-dock-name-label = 停靠窗名称
browser-dock-url-label = 停靠窗 URL
browser-dock-note = 停靠窗会作为独立窗口打开，你可以将其放在工作室旁边。该页面无法访问应用 — 它只是渲染。仅支持 http(s) 网址；停靠窗仅在你点击"打开"时才打开。
browser-dock-error-name = 请为停靠窗命名（例如 Twitch Chat）。
browser-dock-error-url = 停靠窗 URL 必须以 http:// 或 https:// 开头。

# --- studio-preview-pane ---
studio-preview-label = 工作室模式预览
studio-preview-heading = 预览
studio-preview-hint = 点击场景以在此处加载
studio-preview-empty = 预览将显示在此处。
studio-preview-mirrors = 镜像节目
studio-preview-transition-select = 转场
studio-preview-duration = 转场时长（ms）
studio-preview-commit-title = 通过转场将 预览 → 节目 提交（观众会看到）
studio-preview-transitioning = 转场中…
studio-preview-transition-button = 转场 ⇄
studio-preview-luma-placeholder = 灰度擦除图像（png/jpg）
studio-preview-luma-label = 亮度擦除图像
studio-preview-browse = 浏览…
studio-preview-filter-images = 图像
studio-preview-filter-video = 视频
studio-preview-stinger-placeholder = 垫场视频（ProRes 4444 .mov 保留其 alpha 通道）
studio-preview-stinger-label = 垫场视频文件
studio-preview-stinger-cut-label = 垫场切换点（ms）
studio-preview-stinger-cut-title = 场景切换在垫场下发生的时刻（转场开始后的毫秒数）

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = 切换
transition-kind-fade = 淡入淡出
transition-kind-slide-left = 滑动 ←
transition-kind-slide-right = 滑动 →
transition-kind-slide-up = 滑动 ↑
transition-kind-slide-down = 滑动 ↓
transition-kind-swipe-left = 划擦 ←
transition-kind-swipe-right = 划擦 →
transition-kind-luma-linear = 亮度擦除（线性）
transition-kind-luma-radial = 亮度擦除（径向）
transition-kind-luma-horizontal = 亮度擦除（水平）
transition-kind-luma-diamond = 亮度擦除（菱形）
transition-kind-luma-clock = 亮度擦除（时钟）
transition-kind-image = 图像擦除（自定义）
transition-kind-stinger = 垫场（视频）

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = 自定义（RTMP/RTMPS）
stream-service-srt = SRT（自托管）
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = 关于
about-tagline = 像专业工作室一样录制和推流 — 无需账户，无需云端。
about-version = 版本
about-created-by = 创建者
about-project-started = 项目启动
about-first-stable = 首个稳定版本
about-first-stable-pending = 尚未发布 — 1.0.0 正在开发中
about-platform = 平台
about-local-first = Freally Capture 完全在你的机器上运行。无需账户，无遥测，无云端 — 唯一离开你电脑的只有你选择发送的直播流。
about-website = 网站
about-issues = 报告问题
about-license = 许可证
about-eula = EULA
about-third-party = 第三方声明
about-check-updates = 检查更新…

# --- unified settings modal (TASK-906) ---
settings-title = 设置
settings-language-section = 语言
settings-language = 界面语言
settings-language-system = 系统默认
settings-language-note = 在此选择的语言会被记住。"系统默认"会跟随你的操作系统。未翻译的文本会回退为英文。
settings-appearance-section = 外观
settings-theme = 主题
settings-theme-dark = 深色
settings-theme-light = 浅色
settings-theme-custom = 自定义
settings-accent = 强调色
settings-general-section = 常规
settings-show-stats-dock = 显示统计面板
settings-more-section = 更多设置
settings-open-output = 录制…
settings-open-stream = 推流…
settings-open-replay = 重放…
settings-open-hotkeys = 热键…
settings-open-remote = 远程 API…
settings-open-about = 关于…
controls-settings = ⚙ 设置…
controls-settings-title = 语言、外观和全应用范围的偏好设置

# --- command palette (TASK-904) ---
palette-title = 命令面板
palette-search = 搜索场景、来源和操作
palette-placeholder = 搜索场景、来源、操作…
palette-no-results = 没有与“{ $query }”匹配的内容
palette-hint = ↑ ↓ 移动 · Enter 执行 · Esc 关闭
palette-group-scenes = 场景
palette-group-sources = 来源
palette-group-actions = 操作
palette-transition = 转场 预览 → 节目
palette-save-replay = 保存重放
palette-add-marker = 打入章节标记
palette-vertical-canvas = 竖屏（9:16）画布…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = 欢迎使用 Freally Capture
wizard-welcome = 两个简单的步骤：先看看你的机器能做什么，再开始一个场景。大约需要三十秒，而且之后一切都可以更改。
wizard-local-first = 这里的一切都不会离开你的电脑。Freally Capture 没有账户、没有遥测，也没有云端。
wizard-start = 开始使用
wizard-skip = 跳过
wizard-hardware-title = 你的机器能做什么
wizard-probing = 正在查看你的显卡和处理器…
wizard-encoder = 编码器
wizard-canvas = 画布
wizard-bitrate = 比特率
wizard-probe-found = 找到：{ $gpus } · { $cores } 个物理核心
wizard-no-gpu = 无独立 GPU
wizard-apply = 使用这些设置
wizard-keep-current = 保持我现有的设置
wizard-template-title = 从一个场景开始
wizard-template-screen = 采集我的屏幕
wizard-template-screen-note = 为你的主显示器添加一个显示器采集。这是最常见的起点。
wizard-template-empty = 从空白开始
wizard-template-empty-note = 一个空场景。用 + 按钮自己添加来源。
wizard-done = 一切就绪。
wizard-done-hint = 随时按 Ctrl+K 即可搜索场景、来源和操作。设置就在 ⚙ 按钮后面。
wizard-close = 开始推流

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = 你的显卡可以自行编码视频，让处理器腾出手来处理工作室的其余工作。
autoconfig-reason-software = 未找到可用的硬件编码器，因此将由处理器来编码。这样也能用，只是会多占用一些 CPU。
autoconfig-reason-quality-hardware = 1080p、每秒 60 帧，比特率也是每个主流平台都接受的水平。
autoconfig-reason-quality-software = 每秒 30 帧，因为软件编码在 60 帧时会在大多数处理器上丢帧。
autoconfig-reason-quality-low-cores = 较低的比特率，因为这台处理器核心不多，软件编码会与合成器争抢它们。

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = 录制已开始
announce-recording-paused = 录制已暂停
announce-recording-stopped = 录制已停止
announce-live-started = 你已开始直播
announce-live-ended = 直播已结束
announce-reconnecting = 连接已断开，正在重新连接
announce-stream-failed = 直播失败
announce-frames-dropped = 丢失了 { $count } 帧

# CAP-M01 — undo/redo edit history
palette-undo = 撤销
palette-redo = 重做
palette-edit-history = 编辑历史…
history-title = 编辑历史
history-empty = 暂无可撤销的编辑。
history-current = 当前状态
history-close = 关闭
history-addScene = 添加场景
history-renameScene = 重命名场景
history-removeScene = 删除场景
history-reorderScene = 重新排序场景
history-addSource = 添加源
history-removeSource = 删除源
history-reorderSource = 重新排序源
history-renameSource = 重命名源
history-transformSource = 移动源
history-toggleVisibility = 切换可见性
history-toggleLock = 切换锁定
history-setBlendMode = 更改混合模式
history-editSourceProperties = 编辑属性
history-applyLayout = 排列布局
history-moveToSeat = 移动到位置
history-groupSources = 编组源
history-ungroupSources = 取消编组
history-toggleGroupVisibility = 切换编组
history-setSceneAudio = 场景音频
history-setVerticalCanvas = 竖版画布
history-addFilter = 添加滤镜
history-removeFilter = 删除滤镜
history-reorderFilter = 重新排序滤镜
history-editFilter = 编辑滤镜
history-toggleFilter = 切换滤镜
history-setVolume = 调整音量
history-toggleMute = 切换静音
history-setMonitor = 更改监听
history-setTracks = 更改音轨
history-setSyncOffset = 调整音视频同步
history-setAudioHotkeys = 音频快捷键

# CAP-M04 — alignment aids
settings-alignment-section = 对齐辅助
settings-smart-guides = 智能参考线（拖动时吸附）
settings-safe-areas = 安全区叠加
settings-rulers = 标尺
align-group = 对齐到画布
align-left = 左对齐
align-hcenter = 水平居中
align-right = 右对齐
align-top = 顶部对齐
align-vcenter = 垂直居中
align-bottom = 底部对齐

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = 对齐并分布所选
arrange-left = 左边缘对齐
arrange-hcenter = 水平居中
arrange-right = 右边缘对齐
arrange-top = 上边缘对齐
arrange-vcenter = 垂直居中
arrange-bottom = 下边缘对齐
distribute-h = 水平分布
distribute-v = 垂直分布
guides-group = 参考线
guides-add-v = 添加垂直参考线
guides-add-h = 添加水平参考线
guides-clear = 清除所有参考线
history-arrangeItems = 排列项目
history-editGuides = 编辑参考线

# CAP-M05 — edit transform + copy/paste
transform-title = 编辑变换 — { $name }
transform-anchor = 锚点
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = 旋转
transform-crop = 裁剪
transform-crop-left = 左
transform-crop-top = 上
transform-crop-right = 右
transform-crop-bottom = 下
transform-no-size = 源报告其尺寸后，大小和裁剪即可使用。
transform-copy = 复制变换
transform-paste = 粘贴变换
transform-close = 关闭
filters-copy = 复制滤镜 ({ $count })
filters-paste = 粘贴滤镜 ({ $count })
palette-edit-transform = 编辑变换…
history-pasteFilters = 粘贴滤镜

# CAP-M26 — keying workbench
workbench-title = 抠像工作台 — { $name }
workbench-mode-keyed = 已抠像
workbench-mode-source = 源
workbench-mode-matte = 蒙版
workbench-mode-split = 分屏
workbench-eyedropper = 吸管
workbench-eyedropper-hint = 点击源以取样键控颜色。
workbench-loupe = 放大镜
workbench-split = 分屏
workbench-preview-alt = 抠像工作台预览
workbench-tune = 调整
workbench-close = 关闭

# CAP-M06 — multiview monitor
multiview-title = 多画面预监
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = 点击场景切换到该场景。
multiview-hint-stage = 点击场景将其放入预览。
palette-multiview = 多画面预监

# CAP-M07 — projectors
projector-title = 打开投影
projector-source = 源
projector-target-program = 节目
projector-target-preview = 预览
projector-target-scene = 场景…
projector-target-source = 源…
projector-target-multiview = 多画面
projector-which-scene = 哪个场景
projector-which-source = 哪个源
projector-none = 没有可显示的内容
projector-display = 显示器
projector-windowed = 浮动窗口（当前屏幕）
projector-display-option = 显示器 { $n } — { $w }×{ $h }
projector-primary = （主）
projector-open = 打开
projector-cancel = 取消
projector-exit-hint = 按 Esc 退出
palette-projector = 打开投影…

# CAP-M08 — still-frame grab
palette-still = 抓取静帧…
still-saved-toast = 静帧已保存：{ $name }
still-failed-toast = 静帧抓取失败：{ $error }
hotkeys-still = 抓取静帧
hotkeys-still-placeholder = 例如 Ctrl+Shift+P

# CAP-M13 — source health dashboard
palette-source-health = 源健康状态…
palette-av-sync = A/V 同步校准…
palette-hotkey-audit = 快捷键总览…
health-title = 源健康状态
health-col-source = 源
health-col-state = 状态
health-col-resolution = 分辨率
health-col-fps = FPS
health-col-last-frame = 最后一帧
health-col-dropped = 丢弃
health-col-retries = 重启次数
health-col-actions = 操作
health-state-live = 实时
health-state-waiting = 等待中
health-state-error = 错误
health-state-inactive = 未激活
health-restart = 重启
health-properties = 属性
health-empty = 此场景集合还没有源。
health-seconds = { $value } 秒

# CAP-M23 — quit guard + orderly shutdown
quit-title = 退出 Freally Capture？
quit-body = 现在退出将按顺序安全执行以下操作：
quit-consequence-stream = 结束直播并断开与服务的连接。
quit-consequence-recording = 停止录制并完成文件封装。
quit-consequence-replay = 关闭重放缓存 — 未保存的重放画面将被丢弃。
quit-confirm = 安全退出
quit-quitting = 正在关闭…
quit-cancel = 取消

# CAP-M11 — crash-safe recording salvage
salvage-title = 恢复中断的录制？
salvage-body = 上次会话在这些录制仍在写入时意外结束。修复会在原文件旁写入一份可播放的副本 — 原文件绝不会被更改。
salvage-repair = 修复
salvage-repairing = 正在修复…
salvage-done = 已修复
salvage-repaired = 已修复 → { $name }
salvage-failed = 修复失败：{ $error }
salvage-dismiss = 暂不

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = 编码器故障 — 已从 { $from } 切换到 { $to }。直播已重新连接并继续。
fallback-toast-recording = 编码器故障 — 已从 { $from } 切换到 { $to }。录制在新文件中继续。
fallback-note = 编码器回退：{ $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = 节目音频已静默
alarm-clipping = 节目音频正在削波
alarm-black = 节目画面为黑屏
alarm-frozen = 节目画面已有一段时间没有变化
alarm-lowDisk = 磁盘空间：按当前码率约剩 { $minutes } 分钟
alarm-dismiss = 关闭警报
alarm-cleared = 已解除：{ $alarm }

# CAP-M22 — panic button
palette-panic = 紧急 — 切到隐私画面
panic-banner-title = 紧急
panic-banner-body = 节目正在显示隐私画面；所有音频已静音，捕获已停止。直播和录制保持运行。
panic-restore = 恢复…
panic-restore-confirm = 恢复节目？
panic-restore-yes = 恢复
panic-restore-cancel = 取消
hotkeys-panic = 紧急（隐私画面）
hotkeys-panic-placeholder = 如 Ctrl+Shift+F12
hotkeys-timer-toggle = 开始/暂停所有计时器
hotkeys-timer-toggle-placeholder = 如 Ctrl+Shift+T
hotkeys-timer-reset = 重置所有计时器
hotkeys-timer-reset-placeholder = 如 Ctrl+Shift+0
panic-slate-color = 隐私画面颜色
panic-slate-image = 隐私画面图片
panic-slate-image-placeholder = 可选图片路径

# CAP-M24 — redacted diagnostics bundle
diag-title = 诊断包
diag-intro = 导出一个已脱敏的 .zip（配置快照、编码器探测、近期统计 — 绝不包含密钥、路径和名称），供手动附加到 GitHub issue。不会向任何地方发送。
diag-preview = 查看内容
diag-hide-preview = 隐藏预览
diag-export = 导出 .zip
diag-exported = 已导出：{ $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = 开播前检查
preflight-intro = 所有阻断项必须为绿色；其余是诚实的提醒。
preflight-item-targets = 已配置推流目标（密钥/URL）
preflight-item-encoder = 有可用的编码器
preflight-item-sources = 所有源健康
preflight-item-disk = 录制所需磁盘空间
preflight-item-mic = 麦克风电平
preflight-item-desktopAudio = 桌面音频电平
preflight-item-replay = 重放缓存已就绪
preflight-targets-detail = 已启用 { $count } 个
preflight-sources-detail = { $count } 个源出错
preflight-disk-detail = 按当前码率约 { $minutes } 分钟
preflight-fix-stream = 推流设置…
preflight-fix-components = 组件…
preflight-fix-sources = 源健康状态…
preflight-fix-replay = 启用
preflight-optional = 可选
preflight-hold = 全部绿色前暂缓开播
preflight-cancel = 取消
preflight-go-anyway = 仍然开播
preflight-go-live = 开播


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = 背景
scenes-backdrop-aria = { $name } 的背景
backdrop-title = 背景 — { $name }
backdrop-hint = 固定在本场景所有内容之后的壁纸——图片、动态 GIF 或循环视频。采集画面始终在其上方；在画布上滚动即可缩放。
backdrop-choose = 选择图片或视频…
backdrop-remove = 移除背景
backdrop-none = 未设置背景。
backdrop-position = 位置
backdrop-split-full = 整个画布
backdrop-split-left = 左半边
backdrop-split-right = 右半边
backdrop-split-top = 上半边
backdrop-split-bottom = 下半边
backdrop-sync = 开始录制时同时开始播放
backdrop-sync-hint = 录制前停留在第一帧；每次录制都从头播放视频。
backdrop-preview-play = 预览播放
backdrop-preview-pause = 暂停预览
backdrop-filter-all = 背景（图片和视频）
backdrop-filter-images = 图片
backdrop-filter-media = 视频和 GIF
sources-backdrop-badge = 背景壁纸（固定在最底层）
sources-backdrop-pinned = 背景始终固定在最底层
filters-name-flip = 翻转
filters-flip-horizontal = 水平
filters-flip-vertical = 垂直
history-setSceneBackdrop = 设置背景
history-setBackdropSplit = 移动背景
history-setBackdropSync = 背景录制同步
backdrop-scrub = 播放位置
backdrop-loop = 循环
backdrop-reverse = 倒放
backdrop-reverse-hint = 倒放会一次性生成倒序副本（视频需要 ffmpeg 组件；GIF 立即倒放）——长文件首次切换可能较慢。
filters-scaling = 缩放
filters-scaling-hint = 面向复古/像素内容的像素完美模式；「整数」还会把绘制尺寸吸附到整数倍（手柄显示逻辑尺寸）。
filters-scaling-auto = 平滑
filters-scaling-nearest = 最近邻
filters-scaling-integer = 整数（整数倍）
filters-scaling-sharp = 锐利双线性
history-setScaling = 更改缩放
hotkeys-zoom-100 = 缩放：重置（100%）
hotkeys-zoom-150 = 缩放：推近到 150%
hotkeys-zoom-200 = 缩放：推近 2×
hotkeys-zoom-placeholder = Ctrl+Shift+2
sources-follow-title = 缩放时跟随光标（Windows；在画布上滚动即可缩放）
sources-follow-item = 切换 { $name } 的光标跟随
filters-autocrop = ✂ 自动裁剪黑边
filters-autocrop-title = 扫描下一帧的黑边（上下/左右）并裁剪（可撤销）。暗场景绝不会被误裁。
filters-autocrop-follow = 分辨率变化时重新检测
history-autoCrop = 自动裁剪黑边
sources-link-audio = 同时采集该应用的音频（联动：隐藏即静音，移除窗口一并移除）
history-addLinkedWindow = 添加窗口 + 联动音频
sources-hdr-title = 该显示器为 HDR——打开色调映射（画布保持 SDR）
sources-hdr-item = { $name } 的 HDR 色调映射
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = 该显示器输出 HDR。不做色调映射时高光会被裁剪，采集在 SDR 画布上显得发白。更改自下一帧起生效。
sources-hdr-enable-suggested = 启用建议（maxRGB，200 尼特）
sources-hdr-operator = 算子
sources-hdr-op-clip = 裁剪（关）
sources-hdr-op-maxrgb = maxRGB（保色相）
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 拐点（SDR 精确）
sources-hdr-paper-white = 纸白
sources-hdr-nits = 尼特
projector-target-passthrough = 直通监视器（低延迟）
projector-which-device = 设备
projector-passthrough-none = 请先添加显示器、窗口或采集设备。
projector-passthrough-about = 设备原始帧——不经场景、滤镜和合成器。显示实测延迟；音频仍通过混音通道监听。
projector-passthrough-hint = 直通 — Esc 关闭
projector-latency = { $ms } 毫秒
projector-latency-measuring = 测量中…
controls-automation = 自动化
controls-automation-title = Rules, macros & studio variables (CAP-N01/N02)
automation-title = 自动化 — 规则、宏与变量
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = 规则
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = 开
automation-rule-name = Rule name
automation-remove = Remove
automation-when = 当
automation-then-run = 则运行
automation-no-macro = (no macro)
automation-macros = 宏
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = 运行
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = 工作室变量
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
controls-rundown = 流程单
controls-rundown-title = The show rundown: a timed scene playlist (CAP-N09)
rundown-title = 节目流程单
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = 开始
rundown-next = 下一步 ▸
rundown-stop = 停止
rundown-idle = 未运行
rundown-next-up = 接下来：{ $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + 步骤
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
automation-layer = 层
automation-layer-hint = 仅在该层激活时触发（留空＝所有层）。层是黏滞的：层键切换后保持（操作系统全局热键 API 无法实现按住切换层）。
automation-chord-hint = 普通按键（Ctrl+Shift+M）或双击和弦（Ctrl+K, 3）。和弦的第二个键仅在等待期间被占用。
controls-panel = 局域网面板
controls-panel-title = The LAN touch panel + tally lights (CAP-N06/N07)
panel-title = 局域网面板与 Tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = 开启面板
panel-port = 端口
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = 密码
panel-show = 显示
panel-hide = 隐藏
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = 保存
osc-title = OSC 控制面
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = 监听 OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
controls-ptz = PTZ
controls-ptz-title = PTZ camera control — VISCA over IP (CAP-N08)
ptz-title = PTZ 摄像机
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = 摄像机
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = 地址
ptz-port = 端口
ptz-speed = 速度
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
ptz-presets = 预设
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = 调用
ptz-store = 存储
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
controls-midi = MIDI
controls-midi-title = MIDI control surfaces — learn pads and faders (CAP-N03)
midi-title = MIDI 控制面
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = 输入
midi-output = 输出（回馈）
midi-none = (none)
midi-learn = 学习
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = 动作
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
panel-lan-warning = ⚠ 局域网流量未加密——密码以明文 HTTP 出现在网址中。请仅在受信任的网络中使用。
osc-lan-warning = ⚠ OSC 没有密码——网络上任何设备都能发送这些命令。仅在受信任的网络中使用局域网模式。

# System-stats HUD source (CAP-N14)
sources-badge-stats = 统计
sources-add-system-stats = 性能统计（HUD）
sources-stats-title = 添加性能 HUD
sources-stats-note = 把工作室真实测得的数字显示给观众 — fps、CPU、内存、渲染时间、丢帧数和实时码率。显示哪些行以及大小、颜色在源的属性中设置。GPU 占用未测量，因此不显示。
sources-stats-add = 添加统计 HUD
properties-stats-show-fps = 显示 FPS
properties-stats-show-cpu = 显示 CPU
properties-stats-show-memory = 显示内存
properties-stats-show-render = 显示渲染时间
properties-stats-show-dropped = 显示丢帧
properties-stats-show-bitrate = 显示码率
properties-stats-size = 大小（px）
properties-stats-note = HUD 直接在节目画面上绘制简洁的通用标签（FPS, CPU, MEM, RENDER, DROPPED, BITRATE）；未开播时码率行显示“—”。

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = 可视化
sources-add-visualizer = 音频可视化
sources-visualizer-title = 添加音频可视化
sources-visualizer-style-label = 样式
sources-visualizer-style-bars = 频谱柱
sources-visualizer-style-scope = 示波器
sources-visualizer-style-vu = VU 表
sources-visualizer-target-label = 监听对象
sources-visualizer-target-master = 主混音
sources-visualizer-target-track = 轨道 { $n }
sources-visualizer-note = 绘制真正进入混音的信号（推子后）——静音的源显示为平线，与听到的完全一致。大小、颜色、柱数和回落速度在源的属性中设置。
sources-visualizer-add = 添加可视化
properties-vis-bands = 柱数
properties-vis-decay = 回落速度（dB/s）
properties-vis-peak-hold = 峰值保持标记
properties-vis-missing-source = （源缺失）

# Speedrun split timer source (CAP-N18)
sources-badge-splits = 分段
sources-add-split-timer = 速通分段计时器
sources-splits-title = 添加分段计时器
sources-splits-file-label = LiveSplit .lss 文件
sources-splits-comparison-label = 对比对象
sources-splits-comparison-pb = 个人最佳
sources-splits-comparison-best = 最佳分段
sources-splits-comparison-average = 平均
sources-splits-note = 文件以只读方式导入——绝不会写回。请在 设置 → 快捷键 中绑定全局的分段 / 撤销 / 跳过 / 重置按键。刻意不支持读取进程内存的自动分段器。
sources-splits-add = 添加分段计时器
properties-splits-size = 大小（px）
properties-splits-ahead = 领先
properties-splits-behind = 落后
properties-splits-gold = 金段
properties-splits-split = 分段
properties-splits-undo = 撤销
properties-splits-skip = 跳过
properties-splits-reset = 重置
properties-splits-note = 按钮控制正在运行的计时器（全局快捷键在任何应用中都可执行相同操作）。跑动记录绝不会写入 .lss 文件。
hotkeys-split-split = 分段计时器：开始 / 分段
hotkeys-split-undo = 分段计时器：撤销分段
hotkeys-split-skip = 分段计时器：跳过分段
hotkeys-split-reset = 分段计时器：重置
hotkeys-split-placeholder = 如 Numpad1
hotkey-audit-action-split-split = 分段（分段计时器）
hotkey-audit-action-split-undo = 撤销分段
hotkey-audit-action-split-skip = 跳过分段
hotkey-audit-action-split-reset = 重置分段计时器
hotkey-audit-feature-split-timer = 分段计时器

# Media playlist source (CAP-N17)
sources-badge-playlist = 播放列表
sources-add-playlist = 媒体播放列表（无缝）
sources-playlist-title = 添加媒体播放列表
sources-playlist-files-label = 文件（每行一个，自上而下播放）
sources-playlist-browse = 浏览…
sources-playlist-loop = 循环
sources-playlist-shuffle = 随机（每次启动抽取一次；循环时重复该顺序）
sources-playlist-hold-last = 结束时保持最后一帧
sources-playlist-note = 整个裁剪后的列表经带标识的 ffmpeg 组件无缝播放（仅限 wire 格式——.frec 与静态图片请用媒体/幻灯片）。条目要么全是视频要么全是音频，不能混合。裁剪、提示点和“正在播放”变量在属性中设置。
sources-playlist-add = 添加播放列表
properties-playlist-items = 条目（自上而下播放）
properties-playlist-up = 上移
properties-playlist-down = 下移
properties-playlist-remove = 移除条目
properties-playlist-in = 起点（秒）
properties-playlist-out = 终点（秒）
properties-playlist-cues = 提示点（秒，逗号分隔）
properties-playlist-add-item = + 添加条目
properties-playlist-loop = 循环
properties-playlist-shuffle = 随机
properties-playlist-hold-last = 保持最后一帧
properties-playlist-hw = 硬件解码
properties-playlist-variable = “正在播放”变量（留空 = 关闭）
properties-playlist-previous = ⏮ 上一个
properties-playlist-next = ⏭ 下一个
properties-playlist-note = 提示点按钮和上一个/下一个控制正在播放的列表；条目修改在“应用”后生效（列表会重新开始）。在文本源中放入 {"{{"}yourVariable{"}}"} 即可显示正在播放的条目名。
hotkeys-playlist-next = 播放列表：下一个条目
hotkeys-playlist-previous = 播放列表：上一个条目
hotkeys-playlist-placeholder = 如 Ctrl+Alt+Right
hotkey-audit-action-playlist-next = 播放列表下一个
hotkey-audit-action-playlist-previous = 播放列表上一个
hotkey-audit-feature-playlist = 播放列表

# Instant replay source (CAP-N10)
sources-badge-replay = 回放
sources-add-replay = 即时回放
sources-replay-title = 添加即时回放
sources-replay-seconds-label = 回放长度（秒）
sources-replay-speed-label = 速度
sources-replay-speed-full = 100%（带音频）
sources-replay-speed-half = 50% 慢动作（静音）
sources-replay-speed-quarter = 25% 慢动作（静音）
sources-replay-note = 触发回放前保持透明。请先启用回放缓冲（控制面板）并绑定回放快捷键——触发时截取缓冲的最后片段在节目中播放，结束后恢复透明。
sources-replay-add = 添加即时回放
properties-replay-roll = ⏵ 触发回放
properties-replay-note = 触发会把已启用的回放缓冲截成片段并按所选速度播放——仅重定时，绝不插帧。慢动作刻意静音。播放中可拖动/暂停；结束后源恢复透明。
hotkeys-replay-roll = 即时回放：触发
hotkeys-replay-roll-placeholder = 如 Ctrl+Shift+I
hotkey-audit-action-replay-roll = 触发即时回放

# Input overlay source (CAP-N13)
sources-badge-input = 输入
sources-add-input-overlay = 输入叠加层（按键/手柄）
sources-input-title = 添加输入叠加层
sources-input-layout-label = 布局
sources-input-layout-wasd = WASD + 鼠标
sources-input-layout-keyboard = 紧凑键盘 + 鼠标
sources-input-layout-gamepad = 手柄（双摇杆）
sources-input-layout-fightstick = 街机摇杆
sources-input-color-label = 按键
sources-input-accent-label = 按下
sources-input-privacy-note = 隐私：仅当此来源在场景中处于直播状态时才读取输入，且只轮询布局中的固定按键——只是瞬时查看“现在是否按下”，绝不是钩子。不记录、不存储、不向任何地方发送任何内容；输入的文字永远不会被捕获。
sources-input-os-note = 键盘和鼠标状态目前仅在 Windows 上读取——其他系统按键显示为未按下（如实说明，绝不伪装）。手柄通过 gilrs 库在所有平台可用；绘制第一个已连接的手柄，未找到手柄时布局保持未按下。
sources-input-add = 添加输入叠加层

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = 光标特效
filters-cursorfx-hint = 在 Windows 上（应用自行绘制光标），特效直接画进采集画面，因此会出现在录制和直播中。macOS 和 Linux 由系统合成光标，所以这些特效仅限 Windows。更改即时生效。
filters-cursorfx-halo = 光标光晕
filters-cursorfx-halo-color = 颜色
filters-cursorfx-halo-radius = 半径 (px)
filters-cursorfx-ripples = 点击波纹
filters-cursorfx-left-color = 左键
filters-cursorfx-right-color = 右键
filters-cursorfx-keystrokes = 按键提示
filters-cursorfx-keystrokes-hint = 按住期间在光标旁显示固定按键集（字母、数字、修饰键、方向键）。仅在开启时才读取按键，且只是直接画进画面，绝不存储或记录日志。

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = 字幕条
sources-add-title = 字幕条 / 计分板
sources-title-title = 添加字幕条
sources-title-template-label = 从模板开始
sources-title-template-lower-third = 下横栏(色条 + 姓名 + 头衔)
sources-title-template-scoreboard = 计分板(底板 + 4 个格子)
sources-title-template-blank = 空白画布
sources-title-width-label = 画布宽度
sources-title-height-label = 画布高度
sources-title-template-name = 姓名
sources-title-template-subtitle = 头衔
sources-title-template-home = 主队
sources-title-template-away = 客队
sources-title-note = 由文字/图片/色块图层组成的字幕条,带入场/出场动画,本地合成 — 不是浏览器源。图层、文件绑定与 {"{{"}变量{"}}"} 以及实时控制都在源的属性里。
sources-title-add = 添加字幕条
properties-title-layers = 图层(按顺序绘制 — 靠后的行在上层)
properties-title-kind-text = 文字
properties-title-kind-image = 图片
properties-title-kind-rect = 色块
properties-title-x = X
properties-title-y = Y
properties-title-outline = 描边 (px)
properties-title-outline-color = 描边
properties-title-shadow = 阴影
properties-title-animation = 入场/出场动画
properties-title-anim-none = 无(硬切)
properties-title-anim-fade = 淡入淡出
properties-title-anim-slide-left = 向左滑入
properties-title-anim-slide-up = 向上滑入
properties-title-anim-wipe = 划像
properties-title-duration = 时长 (ms)
properties-title-fire-in = ▶ 播放入场
properties-title-fire-out = ◼ 播放出场
properties-title-set-live = 实时更新
properties-title-set-live-note = 立即把这段文字推送到正在播出的字幕条 — 无需应用,无需重启
properties-title-up = 图层上移
properties-title-down = 图层下移
properties-title-remove = 删除图层
properties-title-add-text = + 文字
properties-title-add-image = + 图片
properties-title-add-rect = + 色块
properties-title-note = 入场/出场与"实时更新"直接控制正在运行的字幕条;图层修改在"应用"后生效(字幕条会重启并重新入场)。文字格可绑定受监视的文件(CSV 单元格 / JSON 值 / 整个文件)并展开 {"{{"}变量{"}}"} — "实时更新"优先于两者。

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = 局域网接入（SRT/RTMP 监听）
sources-lan-title = 添加局域网接入监听器
sources-lan-protocol-label = 协议
sources-lan-protocol-srt = SRT（可加密 — 推荐）
sources-lan-protocol-rtmp = RTMP（无身份验证）
sources-lan-port-label = 端口（1024–65535）
sources-lan-passphrase-label = 密码短语（留空 = 开放）
sources-lan-passphrase-hint = SRT 密码短语为 10–79 个字符；发送端必须使用相同的密码短语。
sources-lan-open-warning = 未设密码短语：此网络上的任何人都能以未加密方式向该来源推流。除非网络只属于你，否则请设置一个。
sources-lan-rtmp-warning = RTMP 没有身份验证 — 此网络上的任何人都能向该端口推流。建议使用带密码短语的 SRT。
sources-lan-url-label = 将发送端应用指向
sources-lan-qr-aria = 接入 URL 二维码
sources-lan-note = 仅限局域网：只在此机器的本地地址上监听，且仅在该来源存在期间有效，绝不触碰互联网 — 在你网络中的发送者先发送之前，没有任何数据离开这台机器。解码由明确标示的 ffmpeg 组件完成。在发送者连接之前，画布会显示此 URL。
sources-lan-add = 开始监听
properties-lan-note = 应用协议、端口或密码短语的更改会重启监听器 — 发送端需要重新连接。视频流会适配到 1920×1080 画布。

# Freally Link source & output (CAP-N12)
sources-badge-link = 链路
sources-add-freally-link = Freally Link（另一实例）
sources-link-title = 添加 Freally Link
sources-link-about = 通过你自己的网络接收另一台 Freally Capture 实例的节目（视频 + 主混音）。请先在发送端启用“Freally Link 输出”。v1 通过 TCP 传输 Motion-JPEG：在有线局域网或良好的 Wi-Fi 上表现出色，在弱链路上对带宽如实相告。
sources-link-scan = 扫描局域网
sources-link-scanning = 正在扫描…
sources-link-none = 未找到 Freally Link 输出。请在另一实例上启用“Freally Link 输出”（控制 → LAN 面板），或在下方输入其地址。
sources-link-host = 地址
sources-link-port = 端口
sources-link-key = 配对密钥
sources-link-key-hint = 发送端"Freally Link 输出"设置中的密钥——没有它，发送端一帧画面也不会发送。
sources-link-add = 添加链路
properties-link-note = 未连接时，该来源会显示“连接中”画面，并以递增的间隔自动重试——绝不会卡在旧画面上。每个发送端只允许一个接收端；发送端忙时会礼貌地重试。
link-title = Freally Link 输出
link-about = 将本实例的节目（视频 + 主混音）分享给你自己网络中的另一台（仅一台）Freally Capture；它会在对方那里显示为“Freally Link”来源（双机推流、备用监视器）。默认关闭；启用前不会广播也不会监听。v1 通过 TCP 传输 Motion-JPEG + 未压缩音频——为有线局域网或良好 Wi-Fi 而生，绝不上互联网。
link-enable = 在我的网络上分享节目
link-name = 实例名称
link-key = 配对密钥
link-key-hint = 至少 8 个字符——接收端必须先输入此密钥，才会收到任何一帧画面。
link-lan-warning = ⚠ 接收端必须先出示配对密钥才能收到任何内容，但 v1 中流本身不加密——请仅在可信网络中使用。
link-serving = 接收端可通过“扫描局域网”找到本实例，或手动添加此地址：
link-off-hint = 启用分享后将打开端口，并向局域网扫描公告本实例。
