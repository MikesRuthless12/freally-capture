# Freally Capture — ja
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = スタジオモード
toggle-on = オン
toggle-off = オフ
stats = 統計
core-ok = コア正常
hide-stats-dock = 統計パネルを隠す
show-stats-dock = 統計パネルを表示


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = 設定を保存できませんでした — 変更は再起動後に失われます。
studio-mode-leave = スタジオモードを終了
studio-mode-enter-title = スタジオモード — プレビューシーンを編集し、トランジションでプログラムに反映します
vertical-canvas-title = 2つ目の（縦型 9:16）出力キャンバス — 独立して録画・配信できます
app-version = v{ $version }
core-error = コア エラー
core-unreachable = コアに到達できません（ブラウザモード）
connecting-to-core = コアに接続中…
filters-source-fallback = ソース

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = プログラムプレビュー
preview-program-output = プログラム出力
preview-canvas-editor = キャンバスエディター
preview-px-to-edge-label = フレーム端までのピクセル
preview-px-to-edge = 端までのpx 左 { $left } · 上 { $top } · 右 { $right } · 下 { $bottom }
preview-program-heading = プログラム
preview-no-gpu = 使用可能なGPUアダプターが見つかりませんでした — このマシンではコンポジターを実行できません。
preview-starting-compositor = コンポジターを起動中…
preview-empty-scene = このシーンは空です — ソースでソースを追加してから、このキャンバス上でドラッグ、拡大縮小、回転してください。
preview-fps = { $fps } fps
preview-dropped = { $dropped } ドロップ

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = 招待リンクを受信しました
remote-join-with-webcam = ウェブカメラで参加
remote-dismiss = 閉じる
remote-hosting-guest = リモートゲストをホスト中
remote-you-are-guest = あなたはリモートゲストです
remote-share-view-title = 画面をゲストのアプリに共有します（ゲストにあなたのビューがライブで見えます）
remote-stop-sharing-view = ビュー共有を停止
remote-share-my-view = ビューを共有
remote-allow-center-title = ゲストがどのビューを中央にするか切り替えることを許可します（あなたは制御を保持し、いつでも切り替えできます）
remote-guest-switching = ゲストの切り替え:
remote-stop-screen = 画面を停止
remote-share-screen = 画面を共有
remote-share-screen-title-guest = ホストに画面を共有します（ホストが中央にできるソースになります）
remote-center-request-label = 中央ビューのリクエスト
remote-center = 中央
remote-center-cam-title = ホストにカメラを中央にするよう依頼します
remote-center-my-cam = マイカメラ
remote-center-screen-title = ホストに共有画面を中央にするよう依頼します
remote-center-my-screen = マイ画面
remote-center-host-title = 中央をホストのビューに戻します
remote-center-host-view = ホストビュー
remote-end-session = セッションを終了
remote-leave = 退出
remote-host-view-heading = ホストビュー
remote-host-shared-view-label = ホストの共有ビュー
remote-guest-position-label = ゲストの位置
remote-guest-label = ゲスト
remote-put-guest = ゲストを{ $position }に配置
remote-remove-title = ゲストを削除 — 同じリンクで再参加できます
remote-remove = 削除
remote-ban-title = ゲストをBAN — ブロックして招待リンクを無効化します
remote-ban = BAN
remote-guest-self-muted = ゲストが自分でミュート
remote-unmute-guest = ゲストのミュートを解除
remote-mute-guest = ゲストをミュート
remote-muted-by-host = ホストにミュートされています
remote-unmute-mic = マイクのミュートを解除
remote-mute-mic = マイクをミュート
remote-waiting-for-host = ホストを待機中


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = ソース
sources-fallback-video = 映像
sources-fallback-error = エラー
sources-kind-unknown = ?
sources-missing-source = （ソースがありません）

# Kind badges (small uppercase tag on each source row)
sources-badge-display = ディスプレイ
sources-badge-window = ウィンドウ
sources-badge-portal = ポータル
sources-badge-camera = カメラ
sources-badge-image = 画像
sources-badge-media = メディア
sources-badge-guest = ゲスト
sources-badge-color = カラー
sources-badge-text = テキスト
sources-badge-scene = シーン
sources-badge-slides = スライド
sources-badge-chat = チャット
sources-badge-audio-in = 音声入力
sources-badge-audio-out = 音声出力
sources-badge-app-audio = アプリ音声
sources-badge-test-bars = バー
sources-badge-test-grid = グリッド
sources-badge-test-sweep = スイープ
sources-badge-test-tone = トーン
sources-badge-test-sync = 同期
sources-badge-timer = タイマー

# Add-source menu items
sources-add-display = 画面キャプチャ
sources-add-window = ウィンドウキャプチャ
sources-add-game = ゲームキャプチャ（先にお読みください）
sources-add-webcam = 映像キャプチャデバイス
sources-add-image = 画像
sources-add-media = メディア（動画/画像ファイル）
sources-add-remote-guest = リモートゲスト（P2P試験版）
sources-add-color = カラー
sources-add-text = テキスト
sources-add-timer = タイマー / 時計
sources-add-nested-scene = ネストされたシーン
sources-add-slideshow = 画像スライドショー
sources-add-chat-overlay = ライブチャットオーバーレイ
sources-add-test-signal = テスト信号
sources-add-audio-input = 音声入力キャプチャ
sources-add-audio-output = 音声出力キャプチャ
sources-add-app-audio = アプリケーション音声（Windows）
sources-add-existing = 既存のソース…

# Panel header + toolbar buttons
sources-panel-title = ソース
sources-group-title = ソースをグループ化 — 2つ以上の項目を選んで「グループを作成」。グループ化した項目は一緒に移動し、表示/非表示します
sources-group-aria = ソースをグループ化
sources-arrange = 配置: 画面 + 四隅
sources-add-source = ソースを追加
sources-browser-source-note = ブラウザソースは独自のオンデマンドコンポーネントのマイルストーンとして提供されます（約180 MBのChromiumエンジン — 決してバンドルしません）。現在は: ウィンドウキャプチャ + クロマ/カラーキーで実際のブラウザウィンドウをキャプチャするか、チャット/アラートをドックとして開いてください（コントロール → ドック）。

# Empty state
sources-empty = このシーンにソースがありません — 「+」で画面キャプチャ、ウィンドウ、ウェブカメラ、画像、カラー、テキストを追加してください。キャンバス上でドラッグ、拡大縮小、回転できます。右側のボタンで重なり順を変更します。

# Per-row controls
sources-already-in-group = すでに { $name } に含まれています
sources-pick-for-new-group = 新しいグループ用に選択
sources-pick-item-for-group = { $name } を新しいグループ用に選択
sources-hide = 非表示
sources-show = 表示
sources-hide-item = { $name } を非表示
sources-show-item = { $name } を表示
sources-unfocus-title = フォーカス解除 — レイアウトを元に戻します
sources-focus-title = フォーカス — キャンバスいっぱいに表示（スピーカーを強調）
sources-unfocus-item = { $name } のフォーカスを解除
sources-focus-item = { $name } をフォーカス
sources-center-title = 中央 — これを共有中央ビューにします（カメラはレールに移動します）
sources-center-item = { $name } を中央に
sources-rename-item = { $name } の名前を変更
sources-in-group = グループ { $name } 内

# Row status + retry
sources-retry-error = 再試行 — { $message }
sources-retry-item = { $name } を再試行
sources-status-error = 状態: エラー
sources-open-privacy-title = この権限のためにmacOSのプライバシー設定を開きます
sources-open-privacy-item = { $name } のプライバシー設定を開く
sources-privacy-settings-button = 設定
sources-status-starting = 起動中…
sources-status-live = ライブ
sources-status-aria = 状態: { $state }

# Media row pause/resume
sources-media-resume-title = 動画を再開（配信にライブ反映）
sources-media-pause-title = 動画を一時停止 — フレームを保持して無音に、配信にはライブ反映します
sources-media-resume-item = { $name } を再開
sources-media-pause-item = { $name } を一時停止

# Hover controls
sources-unlock = ロック解除
sources-lock = ロック
sources-unlock-item = { $name } のロックを解除
sources-lock-item = { $name } をロック
sources-raise-title = 重なり順を上げる
sources-raise-item = { $name } を上げる
sources-lower-title = 重なり順を下げる
sources-lower-item = { $name } を下げる
sources-filters-title = フィルタとブレンド
sources-filters-item = { $name } のフィルタ
sources-properties-title = プロパティ
sources-properties-item = { $name } のプロパティ
sources-remove-title = このシーンから削除
sources-remove-item = { $name } を削除

# Grouping footer
sources-create-group = グループを作成（{ $count }）
sources-cancel = キャンセル

# Groups list
sources-groups-aria = ソースグループ
sources-hide-group = グループを非表示
sources-show-group = グループを表示
sources-item-count = · { $count } 項目
sources-ungroup-title = グループ解除 — 項目はそのまま残ります
sources-ungroup-item = { $name } をグループ解除

# Live Chat Overlay picker
sources-chat-title = ライブチャットオーバーレイを追加
sources-chat-youtube-label = YouTube — チャンネル、watch、または live_chat のURL（キー不要、サインイン不要）
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  または watch?v= のURL
sources-chat-twitch-label = Twitch — チャンネル名（匿名で読み取り、アカウント不要）
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — チャンネルスラッグ（公開エンドポイント、ベストエフォート）
sources-chat-kick-placeholder = yourchannel
sources-chat-note = メッセージは h:mm:ss AM/PM のタイムスタンプ付きで透明な背景に表示されます（デフォルトは右上、どこへでもドラッグできます）。チャットが殺到しても古い行が消えるだけで、配信や録画を止めることはありません。Facebookチャットは独自のGraphトークンが必要で、まだ実装されていません — 決して必須ではなく、上記のプラットフォームを妨げることもありません。
sources-chat-add = チャットオーバーレイを追加
sources-chat-default-name = ライブチャット

# Image Slideshow picker
sources-slideshow-title = 画像スライドショーを追加
sources-slideshow-empty = まだ画像がありません — 参照で順番に追加します。
sources-slideshow-remove-slide = スライド { $number } を削除
sources-slideshow-browse = 画像を参照…
sources-slideshow-per-slide-label = スライドごと（ms）
sources-slideshow-crossfade-label = クロスフェード（ms、0 = カット）
sources-slideshow-loop-label = ループ（オフ = 最後のスライドを保持）
sources-slideshow-shuffle-label = サイクルごとにシャッフル
sources-slideshow-note = クロスフェードは同じサイズの画像をブレンドします。異なるサイズは境界でハードカットします（無音のリスケールはしません）。
sources-slideshow-add = スライドショーを追加（{ $count }）

# Nested Scene picker
sources-nested-title = ネストされたシーンを追加
sources-nested-empty = ネストできる他のシーンがありません — 先に2つ目のシーンを追加してください。
sources-nested-scene-name = シーン: { $name }
sources-nested-note = ネストされたシーンはプログラムキャンバスサイズでライブにレンダリングされ、独自の編集に従います。変形、フィルタ、ブレンドは他のソースと同様に適用されます。それを表示するシーンがプログラムの間、その音声ソースがミックスに加わります。

# Display / Window capture picker
sources-capture-display-title = 画面キャプチャを追加
sources-capture-window-title = ウィンドウキャプチャを追加
sources-capture-looking = ソースを検索中…
sources-capture-none-displays = キャプチャできるものがありません — ディスプレイが見つかりませんでした。
sources-capture-none-windows = キャプチャできるものがありません — ウィンドウが見つかりませんでした。
sources-capture-portal-note = Waylandでは、システムダイアログが画面またはウィンドウを選びます — アプリはそこでグローバルにキャプチャできないため、これが正直な（そして唯一の）方法です。
sources-capture-window-note = プレビューはライブで更新されます。最小化されたウィンドウは、復元するまで最後のフレーム（またはなし）を表示します。
sources-thumb-no-preview = プレビューなし
sources-thumb-loading = 読み込み中…

# Video Capture Device picker
sources-webcam-title = 映像キャプチャデバイスを追加
sources-webcam-looking = カメラを検索中…
sources-webcam-none = カメラまたはキャプチャカードが見つかりませんでした。
sources-webcam-format-label = フォーマット
sources-webcam-format-auto-loading = 自動（フォーマットを読み込み中…）
sources-webcam-format-auto = 自動（最高解像度）
sources-webcam-card-presets-label = カードプリセット:
sources-webcam-preset-title = このカードが公表する { $label } モードを選択
sources-webcam-add = カメラを追加

# Audio Input / Output capture picker
sources-audio-output-title = 音声出力キャプチャを追加
sources-audio-input-title = 音声入力キャプチャを追加
sources-audio-default-output = デフォルト出力（あなたが聞いている音）
sources-audio-default-input = デフォルト入力
sources-audio-looking = 音声デバイスを検索中…
sources-audio-none-output = デスクトップ音声のキャプチャデバイスが見つかりませんでした。
sources-audio-none-input = マイクまたはライン入力が見つかりませんでした。
sources-audio-input-note = ミキサーストリップにはVUメーター、フェーダー、ミュート、モニタリング、フィルタ（ノイズ抑制、ゲート、コンプレッサーなど）、トラック割り当てが付きます。すべてこのマシン内で完結します。

# Application Audio picker
sources-appaudio-title = アプリケーション音声を追加
sources-appaudio-looking = 音を出しているアプリを検索中…
sources-appaudio-none = 現在、音を出しているアプリがありません — アプリで再生を開始してから更新してください。
sources-appaudio-refresh = ⟳ 更新
sources-appaudio-note = そのアプリの音声だけを正確にキャプチャします — 独自のVU、フェーダー、ミュート、フィルタ、トラックを持ちます。

# Game Capture picker
sources-game-title = ゲームキャプチャ
sources-game-checking = 確認中…
sources-game-use-portal = 画面キャプチャ（ポータル）を使用
sources-game-use-window = 代わりにウィンドウキャプチャを使用

# Image picker
sources-image-title = 画像を追加
sources-image-file-label = 画像ファイル（PNG、JPEG、BMP、GIF、WebPなど）
sources-image-add = 画像を追加

# Path field
sources-browse = 参照…

# Media picker
sources-media-title = メディアを追加
sources-media-file-label = メディアファイル（mp4、mkv、webm、mov、.frec、または画像）
sources-media-loop-label = ループ（最後に先頭から再開）
sources-media-note = .frec は自社の freally-video コーデックで再生されます — ダウンロード不要です。ワイヤーフォーマット（mp4/mkv/webm/…）はオンデマンドのFFmpegコンポーネントでデコードされ、その音声は独自のストリップとしてミキサーに入ります。
sources-media-add = メディアを追加

# Invite expiry options
sources-ttl-15min = 15分
sources-ttl-30min = 30分
sources-ttl-1hour = 1時間
sources-ttl-1day = 1日

# Remote Guest form
sources-remote-copy-failed = コピーできませんでした — リンクを選択して手動でコピーしてください
sources-remote-join-failed = 参加に失敗しました: { $error }
sources-remote-title = リモートゲスト（P2P試験版）
sources-remote-host-heading = ホスト — ゲストを招待
sources-remote-start-hosting = ホストを開始
sources-remote-expires-label = 有効期限
sources-remote-invite-expiry-aria = 招待の有効期限
sources-remote-invite-link-aria = 招待リンク
sources-remote-copied = コピーしました ✓
sources-remote-copy = コピー
sources-remote-share-note = このリンクを共有してください（Discord / テキスト / メール）。あなたのセッションを含み、設定どおりに期限切れになります。ゲストがそれを開き、ウェブカメラで参加します。
sources-remote-qr-note = スマートフォンでスキャンすればブラウザから直接参加できます — カメラ + マイク、インストール不要。上のコピー可能な freally:// リンクは、それを持つマシンでFreally Captureを開きます。
sources-remote-guest-heading = ゲスト — 招待で参加
sources-remote-paste-placeholder = 招待リンクを貼り付け
sources-remote-invite-input-aria = 招待リンクまたはセッションID
sources-remote-join = ウェブカメラで参加
sources-remote-session-note = ライブセッションのコントロール（ミュート、終了）はメインウィンドウ上部のバーに残ります — このダイアログは閉じてかまいません。
sources-remote-stop-session = セッションを停止

# Invite QR
sources-invite-qr-aria = 招待リンクのQRコード

# Remote device pickers
sources-devices-output-unavailable = 出力ルーティングは利用できません — デフォルトデバイスで再生します
sources-devices-mic-test-failed = マイクテストに失敗しました: { $error }
sources-devices-heading = セッションの音声デバイス
sources-devices-microphone-label = マイク
sources-devices-microphone-aria = セッションのマイク
sources-devices-system-default = システムデフォルト
sources-devices-output-label = 出力
sources-devices-output-aria = セッションの音声出力
sources-devices-stop-test = テストを停止
sources-devices-test = テスト — 自分の声を聞く
sources-devices-testing-note = マイクに話してください — 選択したデバイスの音がライブで聞こえます
sources-devices-idle-note = マイクを出力にループします（ハウリングを避けるにはヘッドホンを）

# TURN relay section
sources-turn-save-failed = 保存できませんでした: { $error }
sources-turn-summary = ネットワーク — オプションのTURNリレー（詳細設定）
sources-turn-note-1 = セッションは直接（P2P）接続します — 無料で、リレーは不要です。両側が厳格なNATの背後にある場合、直接経路は失敗することがあります。その場合は自分で運用するTURNリレーがメディアを中継します。これを省略しても問題ありません — ほとんどの接続は直接のみで動作します。
sources-turn-note-2 = 無料の選択肢: Oracle Cloud の「Always Free」で coturn を無料で実行できます（注: Oracleは登録時にクレジットカードを求めますが、Always-Free の構成は無料のままです）。手順: 1）無料VMを作成、2）coturn をインストール、3）UDP 3478 を開放、4）ユーザー/パスワードを設定、5）ここに turn:your-vm-ip:3478 と認証情報を入力。認証情報はローカルの設定ファイルに保存され、ログには残りません。
sources-turn-url-label = TURN URL
sources-turn-url-placeholder = turn:host:3478（空 = 直接のみ）
sources-turn-url-aria = TURN URL
sources-turn-username-label = ユーザー名
sources-turn-username-aria = TURN ユーザー名
sources-turn-credential-label = 認証情報
sources-turn-credential-aria = TURN 認証情報
sources-turn-note-3 = 3つのフィールドすべてを設定するとリレーが有効になり（TURNサーバーは認証情報を必要とします）、次に開始または参加するセッションに適用されます。自分の2台のマシン間でリレー専用のテスト通話を行って確認してください。
sources-turn-settings-unavailable = 設定は利用できません（ブラウザモード）

# Color picker
sources-color-title = カラーを追加
sources-color-label = カラー
sources-color-width-label = 幅
sources-color-height-label = 高さ
sources-color-add = カラーを追加
sources-testsignal-title = テスト信号を追加
sources-testsignal-pattern-label = パターン
sources-testsignal-bars = SMPTE カラーバー
sources-testsignal-grid = キャリブレーショングリッド
sources-testsignal-sweep = モーションスイープ
sources-testsignal-tone = 1 kHz トーン（−20 dBFS）
sources-testsignal-flash-beep = A/V 同期フラッシュ＋ビープ
sources-testsignal-note = カメラを接続せずにシーン・エンコーダー・プロジェクター・配信先を確認できます。フラッシュ＋ビープのパターンは A/V 同期ワークベンチで使われます。
sources-testsignal-add = テスト信号を追加
sources-timer-title = タイマーを追加
sources-timer-mode-label = モード
sources-timer-wall-clock = 壁時計
sources-timer-countdown = カウントダウン
sources-timer-stopwatch = ストップウォッチ
sources-timer-since-live = 配信開始からの時間
sources-timer-since-recording = 録画開始からの時間
sources-timer-note = 長さ・書式・スタイル・カウントダウン終了アクションはソースのプロパティで設定します。
sources-timer-add = タイマーを追加

# Text picker
sources-text-title = テキストを追加
sources-text-label = テキスト
sources-text-default = テキスト
sources-text-color-label = カラー
sources-text-color-aria = テキストの色
sources-text-size-label = サイズ（px）
sources-text-note = フォントファミリー、配置、折り返し、RTLはソースのプロパティにあります。バンドルされた Noto Sans（アラビア語/ヘブライ語を含む）がデフォルトで、どのマシンでも同一です。
sources-text-add = テキストを追加

# Existing source picker
sources-existing-title = 既存のソースを追加
sources-existing-empty = まだソースがありません — 先にいずれかのシーンに追加してください。既存のソースは共有されます: 1つの名前変更や再設定は、それを表示するすべてのシーンに反映されます。

# Screen + corners layout
sources-slot-off = オフ
sources-slot-center = 中央（画面）
sources-slot-top-left = 左上
sources-slot-top-right = 右上
sources-slot-bottom-left = 左下
sources-slot-bottom-right = 右下
sources-layout-title = 配置: 画面 + 四隅
sources-layout-empty = 先にこのシーンに画面キャプチャと1台以上のカメラを追加してから、ここで配置してください。
sources-layout-note = 画面を中央に、最大4台のカメラを四隅に配置します — 解説/ポッドキャストのレイアウトです。各隅にはウェブカメラ、キャプチャした通話ウィンドウ、またはメディアクリップを配置できます。後からキャンバス上でどれでもドラッグできます。
sources-layout-slot-aria = { $name } のスロット
sources-layout-apply = レイアウトを適用


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = コントロール
controls-start-stop-title-stop = 録画を停止して確定します
controls-start-stop-title-start = 設定 → 出力 の構成でプログラムフィードを録画します
controls-finalizing = ◌ 確定中…
controls-stop-recording = ■ 録画停止
controls-start-recording = ● 録画開始
controls-marker-title = この瞬間にチャプターマーカーを打ちます — 録画に記録されます（mkvチャプター、またはサイドカーファイル）。プラットフォーム側のストリームマーカーはプラットフォームアカウントが必要で、このアプリは決して要求しません。
controls-marker = ◈ マーカー
controls-pause-title-resume = 再開 — ファイルは1つの連続したタイムラインとして続きます
controls-pause-title-pause = 一時停止 — フレームは書き込まれません。再開すると同じ再生可能ファイルが続きます
controls-resume-recording = ▶ 録画を再開
controls-pause-recording = ⏸ 録画を一時停止
controls-reactions-label = リアクション（プログラムに焼き込み）
controls-reactions-title = プログラム上にリアクションを浮かべます — 録画も配信もされるので、リプレイに正確な瞬間が映ります。チャットの視聴者もこれをトリガーできます（そのリアクション絵文字が自動的に浮かびます）。殺到しても画面上の数が制限されるだけです。
controls-react = リアクション { $emoji }
controls-virtual-camera-title = 仮想カメラはOSごとに独自の署名済みドライバコンポーネントが必要です（Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO 拡張 / Linux v4l2loopback） — 独自のマイルストーンとして提供されます。フィードモデルは準備済みです: プログラム、縦型キャンバス、または単一ソースを、Windows/Linux ではペアの仮想マイクとともに（macOS には仮想マイクAPIがありません — 正直に言います）。
controls-virtual-camera = ⌁ 仮想カメラを開始
controls-files-title = 完了した録画 + mp4へのremuxアクション
controls-files = ▤ ファイル…
controls-output-title = 録画フォーマット、エンコーダ、フォルダ、トラック、分割
controls-output = ⚙ 出力…
controls-stream-title = 配信開始のターゲット: サービス、ストリームキー、エンコーダ、ビットレート
controls-stream = ⦿ 配信…
controls-codecs-title = オンデマンドの ffmpeg ワイヤーコーデックコンポーネント（明確に表示、決してバンドルしません）
controls-codecs = ⬡ コーデック…
controls-replay-title = リプレイバッファの長さ + 品質プリセット
controls-replay = ⟲ リプレイ…
controls-keys-title = グローバルホットキー: 録画、配信開始、トランジション、リプレイ保存
controls-keys = ⌨ キー…
controls-scripts-title = サンドボックス化されたLuaスクリプト: 配信開始/シーン/録画イベントに反応してスタジオを操作します
controls-scripts = ⚡ スクリプト…
controls-docks-title = ブラウザドック: チャットのポップアウト、アラートページ、Companionボタンをスタジオの横にウィンドウとして開きます
controls-docks = ⧉ ドック…
controls-remote-title = Stream Deck / Companion コントローラー向けの WebSocket リモートAPI（デフォルトはオフ）
controls-remote = ⌁ リモート…
controls-profiles-title = プロファイル（設定）+ シーンコレクション — 切り替え可能なスナップショット
controls-profiles = ▣ プロファイル…
controls-bug-title = バグを報告 — 匿名、オプトイン（自動送信は一切ありません）
controls-bug = 🐞 バグを報告…
controls-updates-title = アップデートを確認 — 署名済みで検証され、クリックなしでは何もダウンロードされません
controls-updates = ⭳ アップデートを確認…
controls-saved = 保存しました: { $path }

# --- MixerDock.tsx ---
mixer-title = 音声ミキサー
mixer-monitor-error = モニター: { $error }
mixer-switch-to-horizontal = 横型ストリップに切り替え
mixer-switch-to-vertical = 縦型ストリップに切り替え
mixer-layout-aria-vertical = ミキサーレイアウト: 縦型 — 横型に切り替え
mixer-layout-aria-horizontal = ミキサーレイアウト: 横型 — 縦型に切り替え
mixer-empty = このシーンに音声ソースがありません — ソースで「+」から音声入力キャプチャ（マイク）または音声出力キャプチャ（デスクトップ音声）を追加してください。ストリップにはVUメーター、フェーダー、ミュート、モニタリング、フィルタ、トラック割り当てが付きます。
mixer-advanced-title = 音声 — { $name }
mixer-loudness-label = プログラムのラウドネス（LUFS）
mixer-lufs = LUFS
mixer-momentary-title = 瞬間ラウドネス（400 ms）
mixer-short-term-title = 短期ラウドネス（3 s）
mixer-lufs-short = S { $value }
mixer-monitor-label = モニター
mixer-monitor-device-aria = モニター出力デバイス
mixer-default-output = デフォルト出力

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = メモリ
stats-dropped = ドロップ
stats-render = レンダリング
stats-gpu = GPU
stats-gpu-compositing = 合成中
stats-gpu-idle = アイドル
stats-vertical-fps = 9:16 FPS
stats-targets-label = 配信ターゲット
stats-shared-encode = · 共有エンコード
stats-starting = コンポジターを起動中…

# --- ScenesRail.tsx ---
scenes-title = シーン
scenes-new-scene-name = シーン
scenes-add = シーンを追加
scenes-empty = スタジオコアに接続中…
scenes-rename = { $name } の名前を変更
scenes-on-program = プログラム中
scenes-preview = { $name } をプレビュー
scenes-switch-to = { $name } に切り替え
scenes-move-up = 上に移動
scenes-move-up-aria = { $name } を上に移動
scenes-move-down = 下に移動
scenes-move-down-aria = { $name } を下に移動
scenes-last-stays = 最後のシーンは残ります
scenes-remove = このシーンを削除
scenes-remove-aria = { $name } を削除


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
channelstrip-level = レベル
channelstrip-monitor-off = モニターオフ
channelstrip-monitor-only = モニターのみ（ミックスには入れない）
channelstrip-monitor-and-output = モニターと出力
channelstrip-status-error = エラー
channelstrip-status-live = ライブ
channelstrip-status-waiting-audio = 音声を待機中
channelstrip-status = 状態: { $state }
channelstrip-status-waiting = 待機中
channelstrip-mute = ミュート
channelstrip-unmute = ミュート解除
channelstrip-mute-source = { $name } をミュート
channelstrip-unmute-source = { $name } のミュートを解除
channelstrip-scene-mix-on = シーンごとのミックス オン — このストリップはこのシーンのグローバルミックスを上書きします（クリックすると再びグローバルミックスに従います）
channelstrip-scene-mix-off = シーンごとのミックス — このストリップに現在のシーン専用のフェーダー/ミュートを与えます
channelstrip-scene-mix-label = { $name } のシーンごとのミックス
channelstrip-monitor-cycle = { $mode } — クリックで切り替え
channelstrip-monitor-mode = { $name } のモニターモード: { $mode }
channelstrip-audio-filters-title = 音声フィルタ（ノイズ抑制、ゲート、コンプレッサーなど）
channelstrip-audio-filters-label = { $name } の音声フィルタ
channelstrip-advanced-title = 同期オフセット & プッシュトゥトークのホットキー
channelstrip-advanced-label = { $name } の詳細な音声設定
channelstrip-track-assignment = トラック割り当て
channelstrip-track = トラック { $n }
channelstrip-track-assigned = トラック { $n }（割り当て済み）
channelstrip-track-label = { $name } のトラック { $n }
channelstrip-device-error = デバイスエラー
channelstrip-audio-device-error = 音声デバイスエラー
channelstrip-volume-label = { $name } の音量（デシベル）
channelstrip-ptt-hold = プッシュトゥトーク: { $key } を押している間
channelstrip-sync-offset = 同期オフセット（ms、0–{ $max } — この音声を遅らせます）
channelstrip-solo-title = ソロ（PFL）— モニターにはソロのストリップだけが聞こえ、番組ミックスは変わりません
channelstrip-solo-source = { $name } をソロ（PFL）
channelstrip-pan-label = バランス（ダブルクリックでリセット）
channelstrip-pan-aria = { $name } のバランス
channelstrip-mono-label = モノラルにダウンミックス
channelstrip-ptt-hotkey = プッシュトゥトークのホットキー（押している間だけ発声）
channelstrip-ptt-placeholder = 例: Ctrl+Shift+T または F13
channelstrip-ptt-aria = プッシュトゥトークのホットキー
channelstrip-ptm-hotkey = プッシュトゥミュートのホットキー（押している間は無音）
channelstrip-ptm-placeholder = 例: Ctrl+Shift+M
channelstrip-ptm-aria = プッシュトゥミュートのホットキー
channelstrip-hotkeys-note = ホットキーは他のアプリがフォーカスされていても動作します。Linux/Wayland ではグローバルホットキーが利用できない場合があります — それはコンポジターの制限で、正直に言います。
channelstrip-apply = 適用


# --- LiveButton.tsx ---
livebutton-failure-ended = 配信が終了しました
livebutton-title-live = 配信を終了 — すべてのターゲット（実行中の録画は続きます）
livebutton-title-offline = 有効なすべての 設定 → 配信 ターゲットへ配信開始します
livebutton-end-stream = ■ 配信を終了
livebutton-aria-reconnecting = 再接続中
livebutton-aria-live = ライブ
livebutton-badge-retry = 再試行 { $n }
livebutton-badge-live = ライブ
livebutton-go-live = ⦿ 配信開始


# --- RecDot.tsx ---
recdot-paused-aria = 録画を一時停止中
recdot-recording-aria = 録画中
recdot-tracks-one = { $count } 音声トラックを録画中
recdot-tracks-other = { $count } 音声トラックを録画中
recdot-paused = 一時停止中


# --- ReplayControls.tsx ---
replaycontrols-saved = リプレイを保存しました — { $name }
replaycontrols-failure-stopped = バッファが停止しました
replaycontrols-title-disarm = リプレイバッファを解除（未保存の履歴を破棄します）
replaycontrols-title-arm = ローリングリプレイバッファを準備 — 直近N秒をいつでも保存できる状態に保ちます（独自の軽量エンコード。配信と録画には影響しません）
replaycontrols-replay-seconds = ⟲ リプレイ { $seconds }秒
replaycontrols-arm = ⟲ リプレイバッファを準備
replaycontrols-save-title = 直近N秒を録画フォルダに保存します（リプレイ保存ホットキーでも可）
replaycontrols-save = ⤓ 保存


# --- PropertiesDialog.tsx ---
properties-title = プロパティ — { $name }
properties-name = 名前
properties-cancel = キャンセル
properties-apply = 適用
properties-youtube = YouTube — チャンネル / watch / live_chat のURL（キー不要、サインイン不要、ずっと）
properties-twitch = Twitch — チャンネル名（匿名）
properties-kick = Kick — チャンネルスラッグ（公開エンドポイント）
properties-width-px = 幅（px）
properties-lines = 行数
properties-font-px = フォント（px）
properties-images = 画像ファイル（1行に1パス、順番に表示）
properties-per-slide = スライドごと（ms）
properties-crossfade = クロスフェード（ms、0 = カット）
properties-loop-slideshow = ループ（オフ = 最後のスライドを保持）
properties-shuffle = サイクルごとにシャッフル
properties-nested-scene = このソースが構成するシーン（すでにこれを含むシーンは拒否されます）
properties-portal-note = Wayland の ScreenCast ポータルは、このソースが起動するたびにシステムダイアログで画面またはウィンドウを選びます — 設計上、ここに設定するものはありません。
properties-appaudio-capturing = { $exe } から音声をキャプチャ中
properties-appaudio-exe-fallback = アプリケーション
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = 別のアプリを対象にするにはソースを追加し直してください（アプリを再起動するとプロセスIDが変わります）。
properties-image-file = 画像ファイル
properties-media-file = メディアファイル（mp4、mkv、webm、mov、.frec、または画像）
properties-media-loop = ループ（最後に先頭から再開）
properties-media-hwdecode = ハードウェアデコード（自動的にソフトウェアにフォールバック）
properties-media-note = .frec は自社の freally-video コーデックで再生されます — ダウンロード不要です。他の動画フォーマットはオンデマンドのFFmpegコンポーネントでデコードされます。ファイルの音声は独自のミキサーストリップを持ち、ストリップの同期オフセットでA/Vの整合を微調整します。音声のないクリップはストリップが無音のままです。
properties-color = カラー
properties-width = 幅
properties-height = 高さ
properties-testtone-note = −20 dBFS の連続 1 kHz サイン波です。レベルとミュートはミキサーストリップで操作します。ほかに設定はありません。
properties-timer-format = 時刻書式（strftime）
properties-timer-format-note = 例：%H:%M:%S（既定）、%I:%M %p、%A %H:%M — 無効なパターンは %H:%M:%S に戻ります。
properties-timer-utc = UTC オフセット（分）
properties-timer-utc-placeholder = ローカル時刻
properties-timer-duration = 長さ（秒）
properties-timer-target = この時刻までカウントダウン（HH:MM）
properties-timer-target-note = 壁時計ターゲットは自動で動き毎日繰り返します。空にすると長さ＋開始/一時停止/リセットで動きます。
properties-timer-end = ゼロ到達時
properties-timer-end-none = 何もしない
properties-timer-end-flash = タイマーを点滅
properties-timer-end-switch = シーンを切り替え
properties-timer-end-scene = シーン
properties-timer-size = サイズ（px）
properties-timer-start = 開始
properties-timer-pause = 一時停止
properties-timer-reset = リセット
properties-text-file = ファイルから読み込む（パス。空 = 上のテキストを使用）
properties-text-binding = 解析方法
properties-text-binding-whole = ファイル全体
properties-text-binding-csv = CSV セル
properties-text-binding-json = JSON ポインタ
properties-text-csv-row = 行
properties-text-csv-column = 列
properties-text-csv-column-placeholder = 名前または番号
properties-text-json-pointer = ポインタ
properties-text-file-note = 変更から約 0.5 秒以内に再読み込みします。アトミック書き込み（一時ファイル＋リネーム）にも耐性があり、入れ替えの間は最後の正常値が表示され続けます。
avsync-title = A/V 同期キャリブレーション
avsync-intro = 内蔵のフラッシュ＋ビープをディスプレイとスピーカーで再生し、合わせたいカメラとマイクで捉え直すと、ワークベンチがそのずれを測定します。ループは画面とスピーカーを経由するため、その小さな遅延も含まれます。
avsync-video-label = カメラ（映像ソース）
avsync-audio-label = マイク（音声ソース）
avsync-pick = ソースを選択…
avsync-no-video = まずカメラをソースとして追加してください — ワークベンチはデバイスではなくソースを測定します。
avsync-no-audio = まずマイクを音声ソースとして追加してください。
avsync-projector = プログラムを全画面表示する先
avsync-projector-open = プロジェクターを開く
avsync-projector-window-title = プログラム — A/V 同期
avsync-start-note = 開始すると現在のシーンの最前面に一時的な「A/V 同期パターン」ソースが追加され、ビープがモニターデバイスで鳴ります。終了時にすべて取り除かれます。
avsync-manual = 同期オフセット（ms・手動）
avsync-start = キャリブレーション開始
avsync-measuring = 約 12 秒間測定します — カメラを点滅するプログラムに向け、部屋を静かに保ってください…
avsync-flash-seen = カメラがフラッシュを捉えています
avsync-flash-waiting = カメラがフラッシュを捉えるのを待っています…
avsync-beep-heard = マイクがビープを拾っています
avsync-beep-waiting = マイクがビープを拾うのを待っています…
avsync-cancel = キャンセル
avsync-result-offset = 映像は音声より { $offset } ms 遅れて届いています。
avsync-result-detail = { $cycles } 周期で測定、±{ $jitter } ms。
avsync-negative = 音声はすでに映像より遅れて届いています。音声をさらに遅らせても直りません — このカメラの音を別のストリップが担っているなら、そちらのオフセットを下げてください。
avsync-over-cap = 測定されたずれは同期オフセット上限 { $max } ms を超えています。これほどのずれは選ぶソースを誤っていることが多いです — 経路を確認して測り直してください。
avsync-applied = 適用しました — マイクの同期オフセットは現在 { $offset } ms です。
avsync-apply = マイクに { $offset } ms を適用
avsync-again = もう一度測定
avsync-close = 閉じる
avsync-error-noFlash = カメラはフラッシュを一度も捉えませんでした。点滅するプログラムに向け（全画面が有効）、ソースがライブか確認して測り直してください。
avsync-error-noBeep = マイクはビープを一度も拾いませんでした。モニターデバイスが聞こえること、マイクがライブであること（プッシュトゥトークで遮られていないこと）を確認して測り直してください。
avsync-error-tooFewCycles = きれいなフラッシュ／ビープ周期が足りません。測定中はパターンをはっきり見え、聞こえる状態に保ってください。
avsync-error-notThePattern = 検出されたものがパターンのリズムで繰り返していません — 部屋の照明や騒音であってテスト信号ではない可能性が高いです。
avsync-error-unstable = 周期ごとの結果がばらつきすぎて一つの数値を信頼できません。カメラを固定し、騒音を減らして測り直してください。
hotkey-audit-title = ホットキー一覧
hotkey-audit-search = 検索
hotkey-audit-filter = 機能
hotkey-audit-filter-all = すべての機能
hotkey-audit-col-key = キー
hotkey-audit-col-action = アクション
hotkey-audit-col-where = 場所
hotkey-audit-col-status = 状態
hotkey-audit-ok = OK
hotkey-audit-shared = { $count } 件の割り当てで共有
hotkey-audit-unregistered = OS に未登録（他で使用中か利用不可）
hotkey-audit-invalid = 有効なショートカットではありません
hotkey-audit-empty = ホットキーはまだありません — 設定 → ホットキー、またはミキサーストリップで割り当ててください。
hotkey-audit-export = チートシートを書き出す
hotkey-audit-exported = { $path } に保存しました
hotkey-audit-note = キーの割り当て・変更は設定 → ホットキー（グローバル操作）と各ミキサーストリップ（プッシュトゥトーク／ミュート）で行います。この表は監査と記録のためのものです。
hotkey-audit-action-record = 録画の切り替え
hotkey-audit-action-go-live = 配信の切り替え
hotkey-audit-action-transition = トランジション実行
hotkey-audit-action-save-replay = リプレイ保存
hotkey-audit-action-add-marker = マーカー追加
hotkey-audit-action-still = 静止画キャプチャ
hotkey-audit-action-panic = パニック画面
hotkey-audit-action-timer-toggle = 全タイマー開始/一時停止
hotkey-audit-action-timer-reset = 全タイマーリセット
hotkey-audit-action-ptt = プッシュトゥトーク
hotkey-audit-action-ptm = プッシュトゥミュート
hotkey-audit-feature-recording = 録画
hotkey-audit-feature-streaming = 配信
hotkey-audit-feature-studio = スタジオモード
hotkey-audit-feature-replay = リプレイ
hotkey-audit-feature-markers = マーカー
hotkey-audit-feature-stills = 静止画
hotkey-audit-feature-panic = パニック
hotkey-audit-feature-timers = タイマー
hotkey-audit-feature-audio = 音声（ソース別）
properties-text = テキスト
properties-font-family = フォントファミリー（システム、空欄 = デフォルト）
properties-size-px = サイズ（px）
properties-text-color = テキストの色
properties-align = 配置
properties-align-left = 左
properties-align-center = 中央
properties-align-right = 右
properties-line-spacing = 行間
properties-wrap-width = 折り返し幅（px、0 = オフ）
properties-force-rtl = 右から左を強制
properties-text-note = レンダリングは実際のシェーピング（アラビア語の連結、リガチャ）と双方向の行順序を使用します。バンドルされた Noto Sans ファミリー（アラビア語/ヘブライ語を含む）がデフォルトで、システムのファミリーも使えます。CJKは現状システムフォントを使用します。
properties-repick-capturing = キャプチャ中: { $label }
properties-repick-looking = ソースを検索中…
properties-repick-none-displays = 再選択できるディスプレイが見つかりません。
properties-repick-none-windows = 再選択できるウィンドウが見つかりません。
properties-repick-again = 再度選択:
properties-device = デバイス
properties-video-current-device = （現在のデバイス）
properties-format = フォーマット
properties-format-auto-loading = 自動（フォーマットを読み込み中…）
properties-deinterlace = デインターレース
properties-deinterlace-off = オフ
properties-deinterlace-discard = 破棄（片フィールドをライン倍化）
properties-deinterlace-bob = ボブ（フィールドを交互に）
properties-deinterlace-linear = リニア（補間）
properties-deinterlace-blend = ブレンド（フィールド平均）
properties-deinterlace-adaptive = 動き適応（yadif 級）
properties-field-order = フィールド順
properties-field-order-top = トップフィールド先
properties-field-order-bottom = ボトムフィールド先
properties-deinterlace-note = インターレースのキャプチャカード入力向け。純粋な CPU 処理で全 OS 同一。変更するとデバイスが再起動します（フォーマット変更と同様）。
camera-controls-title = カメラコントロール
camera-controls-refresh = 更新
camera-controls-reset = プロファイルをリセット
camera-controls-empty = 現在コントロールはありません — デバイスがストリーミング中である必要があり（先にシーンへ追加）、バックエンドによっては何も報告しません（特に macOS）。これが OS ごとの正直な状態です。
camera-controls-note = 変更は即座に反映され、このデバイスのプロファイルに保存されます。再接続や再起動時に再適用されます。
camera-control-brightness = 明るさ
camera-control-contrast = コントラスト
camera-control-hue = 色相
camera-control-saturation = 彩度
camera-control-sharpness = シャープネス
camera-control-gamma = ガンマ
camera-control-white-balance = ホワイトバランス
camera-control-backlight = 逆光補正
camera-control-gain = ゲイン
camera-control-pan = パン
camera-control-tilt = チルト
camera-control-zoom = ズーム
camera-control-exposure = 露出
camera-control-iris = アイリス
camera-control-focus = フォーカス
properties-format-auto = 自動（最高解像度）
properties-audio-capture-of = 音声をキャプチャする対象
properties-audio-default-output = デフォルト出力（あなたが聞いている音）
properties-audio-default-input = デフォルト入力
properties-audio-default-suffix = （デフォルト）
properties-audio-current-device = （現在のデバイス: { $id }）


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = ゲイン
audiofilters-name-noise-gate = ノイズゲート
audiofilters-name-compressor = コンプレッサー
audiofilters-name-limiter = リミッター
audiofilters-name-eq = 3バンドEQ
audiofilters-name-denoise = ノイズ抑制
audiofilters-name-ducking = ダッキング
audiofilters-title = 音声フィルタ — { $name }
audiofilters-chain-header = フィルタチェーン（上が先に、フェーダーの前に実行）
audiofilters-add = + フィルタを追加
audiofilters-add-menu = 音声フィルタを追加
audiofilters-empty = まだフィルタがありません — マイクをノイズ抑制（クラシックDSP、MLなし）、部屋をゲート、コンプレッサーでピークを抑える、声の下で音楽をダッキング。
audiofilters-enable = { $name } を有効化
audiofilters-run-earlier = 早く実行
audiofilters-move-up = { $name } を上に移動
audiofilters-run-later = 遅く実行
audiofilters-move-down = { $name } を下に移動
audiofilters-remove-title = フィルタを削除
audiofilters-remove = { $name } を削除
audiofilters-gain-db = ゲイン（dB）
audiofilters-open-db = 開くレベル（dB）
audiofilters-close-db = 閉じるレベル（dB）
audiofilters-attack-ms = アタック（ms）
audiofilters-hold-ms = ホールド（ms）
audiofilters-release-ms = リリース（ms）
audiofilters-ratio = レシオ（:1）
audiofilters-threshold-db = スレッショルド（dB）
audiofilters-output-gain-db = 出力ゲイン（dB）
audiofilters-ceiling-db = シーリング（dB）
audiofilters-low-db = 低域（dB）
audiofilters-mid-db = 中域（dB）
audiofilters-high-db = 高域（dB）
audiofilters-strength = 強さ
audiofilters-denoise-note = 自社のクラシックDSPスペクトル抑制 — 定常ノイズ（ファン、ヒス）が下がり、音声は通過します。憲章に従い、MLもモデルもありません。
audiofilters-duck-under = ダッキングの対象
audiofilters-ducking-trigger = ダッキングのトリガーソース
audiofilters-pick-trigger = （トリガーを選択 — 例: あなたのマイク）
audiofilters-trigger-at-db = トリガーレベル（dB）
audiofilters-duck-by-db = ダッキング量（dB）


# --- FiltersDialog.tsx ---
filters-name-chroma-key = クロマキー
filters-name-color-key = カラーキー
filters-name-luma-key = ルマキー
filters-name-render-delay = レンダリング遅延
filters-name-color-correction = 色補正
filters-name-lut = LUTを適用
filters-name-blur = ぼかし
filters-name-mask = イメージマスク
filters-name-sharpen = シャープ
filters-name-scroll = スクロール
filters-name-crop = クロップ
filters-title = フィルタ — { $name }
filters-blend-mode = ブレンドモード
filters-chain-header = フィルタチェーン（上が先に実行）
filters-add = + フィルタを追加
filters-add-menu = フィルタを追加
filters-empty = まだフィルタがありません — ウェブカメラをクロマキー、キャプチャを色補正、ティッカーをスクロール。
filters-enable = { $name } を有効化
filters-run-earlier = 早く実行
filters-move-up = { $name } を上に移動
filters-run-later = 遅く実行
filters-move-down = { $name } を下に移動
filters-remove-title = フィルタを削除
filters-remove = { $name } を削除
filters-key-color-rgb = キーカラー（任意の色、RGB距離）
filters-similarity = 類似度
filters-smoothness = 滑らかさ
filters-luma-min = ルマ最小（暗い部分をキーアウト）
filters-luma-max = ルマ最大（明るい部分をキーアウト）
filters-delay = 遅延（ms — 映像のみ、例: 音声と同期。最大500）
filters-key-color = キーカラー
filters-spill = スピル
filters-gamma = ガンマ
filters-brightness = 明るさ
filters-contrast = コントラスト
filters-saturation = 彩度
filters-hue-shift = 色相シフト
filters-opacity = 不透明度
filters-cube-file = .cube ファイル
filters-amount = 量
filters-radius = 半径
filters-mask-image = マスク画像
filters-mask-mode = モード
filters-mask-alpha = アルファ
filters-mask-luma = ルマ
filters-mask-invert = 反転
filters-speed-x = 速度X（px/s）
filters-speed-y = 速度Y（px/s）
filters-crop-left = 左
filters-crop-top = 上
filters-crop-right = 右
filters-crop-bottom = 下
filters-crop-aria = クロップ { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = 更新
pickershell-refresh-title = リストを更新
pickershell-close = 閉じる


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = バグを報告
bugreport-intro = レポートは匿名でオプトインです — 自動送信は一切ありません。下の正確なテキストを確認してから、事前入力済みのGitHub issueまたはメールアプリで送信します。個人データはありません（ホームパスとユーザー名は伏せられます）。アカウント不要、サーバー不要。
bugreport-crash-notice = Freally Capture が前回の実行で予期せず終了しました — 匿名のクラッシュ詳細を下に含めています。報告すると素早い修正に役立ちます。
bugreport-description-label = 発生時に何をしていましたか？（任意）
bugreport-description-placeholder = 例: 2台目のウェブカメラを追加したらプレビューが固まった
bugreport-include-crash = 前回の実行の匿名クラッシュ詳細を含める
bugreport-preview-label = 送信される内容の正確な表示
bugreport-open-github = GitHub issue を開く
bugreport-gmail-title = ブラウザでGmailの作成ウィンドウを事前入力済みで開きます。サインアウト中の場合、Googleはまずログイン画面を表示します。
bugreport-compose-gmail = Gmailで作成
bugreport-email-title = このPCがデフォルトで使うメールアプリ（Outlook、Thunderbird、Mailなど）で下書きを開きます
bugreport-send-email = メールを送信
bugreport-copied = コピーしました ✓
bugreport-copy-report = レポートをコピー
bugreport-dismiss-crash = クラッシュを閉じる
bugreport-copy-failed = コピーできませんでした — テキストを選択して手動でコピーしてください
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = 何が起きたか
bugreport-preview-no-description = （説明なし）
bugreport-preview-diagnostics = 匿名診断情報（個人データなし）
bugreport-preview-from = 送信元: Freally Capture
bugreport-preview-crash-excerpt = --- クラッシュ抜粋 ---


# --- Updates.tsx ---
updates-title = ソフトウェア更新
updates-checking = アップデートを確認中…
updates-uptodate = 最新バージョンです。
updates-check-again = もう一度確認
updates-available = バージョン { $version } が利用可能です
updates-current-version = （現在 { $current }）
updates-release-notes-label = バージョン { $version } — リリースノート
updates-confirm = 今すぐ更新しますか？ダウンロードは適用前にバンドルされた署名鍵で検証されます。Freally Capture が閉じ、インストーラーが実行され、新しいバージョンが自動的に再び開きます。
updates-yes-update-now = はい、今すぐ更新
updates-no-not-now = いいえ、後で
updates-downloading = { $version } をダウンロード中…
updates-starting = 起動中…
updates-installed = アップデートをインストールしました。
updates-restart-now = 今すぐ再起動
updates-restart-later = 後で再起動
updates-try-again = もう一度試す


# --- Models.tsx ---
models-title = コンポーネント
models-ffmpeg-heading = FFmpeg — ワイヤーコーデック
models-badge-third-party = サードパーティ · 非バンドル
models-ffmpeg-desc = Freally Capture 独自のエンジンは、追加なしでロスレスの freally-video（.frec）を録画します。プラットフォームやプレーヤーが期待するワイヤーフォーマット — mp4/mkv/mov/webm の H.264/AAC（および HEVC/AV1）— の録画には FFmpeg を使います。これはこのアプリが決して同梱しない別ツールです: これらのコーデックは特許で保護されているため、オプションのまま明確に表示されます。下の固定ビルドからオンデマンドでダウンロードされ、初回使用前に SHA-256 で検証され、ユーザーごとにキャッシュされ、別プロセスとして駆動されます。そのライセンス（LGPL/GPL）は独自のものです — THIRD-PARTY-NOTICES を参照してください。
models-checking = 確認中…
models-ffmpeg-not-installed = 未インストール。利用可能: { $source } の FFmpeg { $version }（{ $size } のダウンロード）。
models-ffmpeg-none-pinned = このプラットフォーム向けに固定された FFmpeg ビルドはまだありません — ここではワイヤーコーデックの録画は利用できません。ロスレスの freally-video 録画には影響しません。
models-ffmpeg-download-verify = ダウンロード & 検証（{ $size }）
models-downloading = ダウンロード中…
models-download-of = /
models-cancel = キャンセル
models-ffmpeg-verifying = 固定された SHA-256 に対してダウンロードを検証中…
models-ffmpeg-extracting = 展開中…
models-ffmpeg-ready = インストール & 検証済み — { $version }
models-remove = 削除
models-ffmpeg-retry = ダウンロードを再試行
models-network-note = このパネルでのネットワーク動作はダウンロードだけで、自動で始まることはありません。チェックサムが失敗するとインストールは中止されます — アプリは保証できないバイトを実行することを拒否します。
models-cef-heading = ブラウザソースのランタイム — Chromium（CEF）
models-cef-desc = ブラウザソースは、Chromium Embedded Framework を通じてWebページ（アラート、ウィジェット、オーバーレイ）をレンダリングします — これはこのアプリが決して同梱しない約100 MBのランタイムです。公式のCEFビルドインデックスからオンデマンドでダウンロードされ、展開前にそのインデックスのSHA-1に対して検証され、ユーザーごとにキャッシュされます。それを通じてレンダリングするブラウザソースは独自のマイルストーンで登場します。これはそれが必要とするランタイムをインストールします。
models-cef-download-install = ダウンロード & インストール
models-cef-unsupported = CEFはこのプラットフォーム向けのビルドを公開していません — ここではブラウザソースは利用できません。
models-cef-resolving = 最新の安定版ビルドを解決中…
models-cef-verifying = インデックスの SHA-1 に対してダウンロードを検証中…
models-cef-extracting = ランタイムを展開中…
models-cef-ready = インストール済み — CEF { $version }。
models-cef-retry = 再試行
models-integrations-heading = オプションの統合
models-badge-never-bundled = 決してバンドルしません
models-ndi-detected = 検出済み
models-ndi-not-installed = 未インストール
models-vst-available = 利用可能
models-vst-not-available = 利用不可


# --- Recordings.tsx ---
recordings-title = 録画
recordings-loading = フォルダを読み込み中…
recordings-empty = まだ録画がありません — 録画開始で 出力 に設定したフォルダに書き込まれます。
recordings-frec-label = 自社のロスレス（freally-video）
recordings-remux-title = mp4として再ラップ — ストリームコピー、再エンコードなし、品質変化なし（FFmpegコンポーネントが必要）
recordings-remuxing = Remux中…
recordings-remux-to-mp4 = MP4にRemux
recordings-export-mp4-title = 自社の .frec をデコードして MP4（H.264/AAC）に再エンコードし、どのプレーヤーでも再生できるようにします — FFmpegコンポーネントが必要
recordings-exporting = エクスポート中…
recordings-export-mp4 = エクスポート → MP4
recordings-export-mkv-title = 自社の .frec をデコードして MKV に再エンコードし、どのプレーヤーでも再生できるようにします
recordings-starting = 起動中…
recordings-frames = { $done } / { $total } フレーム
recordings-cancel = キャンセル
recordings-export-cancelled = エクスポートをキャンセルしました。
recordings-exported-to = { $path } にエクスポートしました
recordings-remuxed-to = { $path } にRemuxしました


# --- OpenedFrec.tsx ---
openfrec-title = .frec 録画を開く
openfrec-desc = Freally Capture は自社のロスレス .frec フォーマットを録画します — 再生はしません。Freally Player がリリースされると .frec を直接再生します。今はMP4/MKVにエクスポートすれば、どのプレーヤー（VLC、OSのプレーヤー、何でも）でも再生できます。
openfrec-exported-to = { $path } にエクスポートしました
openfrec-exporting = エクスポート中…
openfrec-starting = 起動中…
openfrec-export-mp4 = エクスポート → MP4
openfrec-export-mkv = エクスポート → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = 縦型キャンバス（9:16）
vertical-enable = 2つ目のキャンバスを有効化 — プログラムとは独立して録画・配信できます
vertical-scene-label = このキャンバスが構成するシーン
vertical-width = 幅
vertical-height = 高さ
vertical-preview-alt = 縦型キャンバスのプレビュー
vertical-note = 項目の位置はキャンバス間でピクセル単位で正確です: このプレビューが縦型の結果を表示している間、シーンレールでこのシーンを選択して配置します。配信ターゲットは ⦿ 配信… でこのキャンバスを選びます。設定 → 出力 でメインファイルと並行して録画できます。
vertical-close = 閉じる


# --- EulaGate.tsx ---
eula-title = Freally Capture — 使用許諾契約
eula-version = v{ $version }
eula-intro = Freally Capture を使用するには、この契約を読んで同意してください。要するに: これは中立的なツールであり、あなたがキャプチャ、録画、配信するもの、そしてそれらの権利を持つことについては、あなただけが責任を負います。
eula-thanks = お読みいただきありがとうございます。
eula-scroll-hint = 続けるには最後までスクロールしてください。
eula-decline = 拒否して終了
eula-agree = 同意します


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = 出力
output-loading = 設定を読み込み中です…
output-container-frec = freally-video（.frec） — ロスレス、自社製、ダウンロード不要
output-container-mkv = MKV — クラッシュ耐性あり、後でmp4にremux
output-container-mp4 = MP4 — どこでも再生可能
output-container-mov = MOV
output-container-webm = WebM（AV1 + Opus）
output-preset-lossless-label = ロスレス
output-preset-lossless-title = 自社の freally-video コーデック — ビット完全、ダウンロード不要
output-preset-high-label = 高品質
output-preset-high-title = MP4、検出された最良のエンコーダ、ほぼロスレスの CQ 16、Quality プリセット
output-preset-balanced-label = バランス
output-preset-balanced-title = MKV、検出された最良のエンコーダ、CQ 23、Balanced プリセット
output-recording-format = 録画フォーマット
output-ffmpeg-warning = このフォーマットには FFmpeg コンポーネント（ワイヤーコーデック — 非バンドル）が必要です。ロスレスの .frec には何も要りません。
output-install = インストール…
output-recordings-folder = 録画フォルダ
output-folder-placeholder = OSのビデオフォルダ
output-filename-prefix = ファイル名のプレフィックス
output-recording-template = 録画のファイル名
output-replay-template = リプレイのファイル名
output-still-template = 静止画のファイル名
output-template-tokens = トークン: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = リプレイフォルダ
output-still-folder = 静止画フォルダ
output-same-folder-placeholder = 録画フォルダ
output-frame-rate = フレームレート
output-fps-option = { $fps } fps
output-split-every = 分割間隔（分、0 = オフ）
output-output-width = 出力幅（0 = キャンバス、ワイヤーフォーマットのみ）
output-output-height = 出力高さ（0 = キャンバス）
output-record-vertical = 縦型キャンバスも録画（並行した「…（vertical）」ファイル。9:16 キャンバスの有効化が必要）
output-audio-tracks = 音声トラック
output-recorded-tracks-group = 録画トラック
output-track-last-one = 少なくとも1つのトラックを録画する必要があります
output-record-track-on = トラック { $index } を録画: オン
output-record-track-off = トラック { $index } を録画: オフ
output-encoder-heading = エンコーダ
output-video-encoder = 映像エンコーダ
output-encoder-auto = 自動 — 検出された最良（H.264）
output-encoder-unavailable = — ここでは利用不可
output-preset = プリセット
output-preset-quality = 品質
output-preset-balanced-option = バランス
output-preset-performance = パフォーマンス
output-rate-control = レート制御
output-rc-cqp = CQP（固定品質）
output-rc-cbr = CBR（固定ビットレート）
output-rc-vbr = VBR（可変ビットレート）
output-cq = CQ（0–51、低いほど高品質）
output-bitrate = ビットレート（kbps）
output-keyframe = キーフレーム間隔（s）
output-audio-bitrate = 音声ビットレート（kbps / トラック）
output-presets = プリセット:

# --- SettingsStream.tsx ---
stream-title = 設定 — 配信
stream-target-enabled = ターゲット { $index } 有効
stream-target = ターゲット { $index }
stream-remove = 削除
stream-service = サービス
stream-canvas = キャンバス
stream-canvas-main = メイン（プログラム）
stream-canvas-vertical = 縦型（9:16 — スタジオで有効化）
stream-ingest-srt = SRT インジェストURL
stream-ingest-whip = WHIP エンドポイントURL
stream-ingest-url = インジェストURL
stream-ingest-override = （上書き — 空 = サービスのプリセット）
stream-key-srt = streamid（任意 — ?streamid=… として付加、秘密情報として扱われます）
stream-key-whip = Bearer トークン（任意 — Authorization ヘッダーとして送信、秘密情報）
stream-key-custom = ストリームキー（あなたのサーバーから — 秘密情報として扱われます）
stream-key-service = ストリームキー（クリエイターダッシュボードから — 秘密情報として扱われます）
stream-key-aria = ストリームキー { $index }
stream-key-hide = 非表示
stream-key-show = 表示
stream-encoder = エンコーダ（H.264 — RTMP、SRT、WHIP がすべて運ぶもの）
stream-encoder-auto = 自動 — 検出された最良の H.264 エンコーダ
stream-encoder-unavailable = （ここでは利用不可）
stream-video-bitrate = 映像ビットレート（kbps、CBR）
stream-audio-bitrate = 音声ビットレート（kbps）
stream-fps = FPS
stream-keyframe = キーフレーム間隔（s）
stream-audio-track = 音声トラック（1–6）
stream-output-width = 出力幅（0 = キャンバス）
stream-output-height = 出力高さ（0 = キャンバス）
stream-add-target = + ターゲットを追加
stream-go-live-note = 配信開始は、有効なすべてのターゲットへ同時に、各プラットフォームへ直接公開します。同一のエンコーダ設定を持つターゲットは単一のエンコードを共有します。
stream-auto-record = 配信開始時に録画も開始（録画は引き続き独立して停止します）
stream-ffmpeg-note-before = 配信のワイヤーコーデックは、表示されたオンデマンドの ffmpeg コンポーネントを通じて実行されます —
stream-ffmpeg-note-link = ここで管理
stream-ffmpeg-note-after = 。ローカル録画は配信の状態に関係なく実行され続けます。
stream-cancel = キャンセル
stream-save = 保存

# --- SettingsReplay.tsx ---
replay-title = 設定 — リプレイバッファ
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 分
replay-length-2min = 2 分
replay-length-5min = 5 分
replay-quality-low = 低（3 Mbps）
replay-quality-standard = 標準（6 Mbps）
replay-quality-high = 高（12 Mbps）
replay-length-presets = 長さプリセット
replay-quality-presets = 品質プリセット
replay-length-seconds = 長さ（秒）
replay-video-bitrate = 映像ビットレート（kbps）
replay-fps = FPS
replay-audio-track = 音声トラック（1–6）
replay-note = 準備中、バッファは独自の軽量エンコードで、境界のあるディスク上のリングに書き込みます — この設定では約 { $mb } MB です。保存はリングを再エンコードなしでつなぎ、配信や録画には一切触れません。変更は次に準備したときに適用されます。
replay-cancel = キャンセル
replay-save = 保存

# --- SettingsRemote.tsx ---
remote-title = 設定 — リモート制御
remote-enable = WebSocket リモートAPIを有効化
remote-password = パスワード（必須 — コントローラーはこれで認証します）
remote-password-placeholder = コントローラー用のパスワード
remote-password-hide = 非表示
remote-password-show = 表示
remote-port = ポート
remote-allow-lan = LAN 接続を許可（デフォルトはこのマシンのみ）
remote-note = オフ = ポートは閉じています。オン = 127.0.0.1（オプトインすればLANも）上のパスワード保護された WebSocket で、シーン切り替え、トランジション実行、配信と録画の開始/停止、リプレイ保存、ミュート/音量の設定ができます — UIと同じ操作で、それ以上はできません。ファイルの読み取りはできません。パスワードは他の認証情報と同様に扱ってください。別のデバイスから制御する必要が特にない限り、このマシンのみを推奨します。
remote-password-required = リモートAPIを有効にするにはパスワードが必要です。
remote-cancel = キャンセル
remote-save = 保存

# --- SettingsHotkeys.tsx ---
hotkeys-title = 設定 — ホットキー
hotkeys-record = 録画の開始 / 停止
hotkeys-record-placeholder = 例: Ctrl+Shift+R
hotkeys-go-live = 配信開始 / 配信終了
hotkeys-go-live-placeholder = 例: Ctrl+Shift+L
hotkeys-transition = スタジオモードのトランジション
hotkeys-transition-placeholder = 例: Ctrl+Shift+T または F13
hotkeys-save-replay = リプレイを保存（直近N秒）
hotkeys-save-replay-placeholder = 例: Ctrl+Shift+S
hotkeys-add-marker = チャプターマーカーを打つ（録画）
hotkeys-add-marker-placeholder = 例: Ctrl+Shift+K
hotkeys-note = ホットキーはグローバルです — 他のアプリがフォーカスされていても発火します。空欄 = 未割り当て。ミキサーのプッシュトゥトーク/ミュートキーは各ストリップの ⋯ メニューにあります。Linux/Wayland ではグローバルホットキーが利用できない場合があります（コンポジターの制限） — ボタンは引き続き動作します。
hotkeys-cancel = キャンセル
hotkeys-save = 保存

# --- WorkspaceDialog.tsx ---
workspace-title = プロファイル & シーンコレクション
workspace-profiles = プロファイル
workspace-profiles-hint = プロファイルはあなたの設定です — 配信ターゲット、出力、ホットキー。番組ごと、プラットフォームごとに切り替えます。
workspace-collections = シーンコレクション
workspace-collections-hint = コレクションはあなたのシーン + ソースです。作成は現在のものを複製して出発点にします。
workspace-active = アクティブ
workspace-switch-to = { $name } に切り替え
workspace-active-marker = ● アクティブ
workspace-new-name-placeholder = 新しい名前…
workspace-new-name-label = 新しい { $title } の名前
workspace-create = 作成

# --- OBS import (CAP-M02) ---
workspace-import-obs = OBS からインポート…
workspace-import-obs-hint = OBS のシーンコレクション（その scenes.json）を取り込みます。現在のコレクションは先に保存されます。
workspace-import-busy = インポート中…
workspace-import-title = 「{ $name }」をインポートしました
workspace-import-summary = シーン { $scenes } · ソース { $sources } · アイテム { $items }
workspace-import-dismiss = 閉じる
workspace-import-clean = すべて問題なく取り込まれました。
workspace-import-geometry-caveat = サイズと位置は OBS のレイアウトから合わせています。各シーンを確認し、キャプチャデバイスを選び直してください。
workspace-import-notes-title = 注意付きでインポート
workspace-import-skipped-title = 未インポート
import-note-needsReselect = デバイス/モニター/ウィンドウを選び直す
import-note-gameCaptureAsWindow = ゲームキャプチャ → ウィンドウキャプチャ
import-note-referencesFile = ファイルパスを確認
import-note-filterDropped = 一部のフィルターは非対応
import-note-geometryApproximated = 位置/サイズは近似
import-skip-unsupportedKind = 相当するソースがありません
import-skip-group = グループは未対応です

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = 不明なファイルを再リンク…
doctor-title = 不明なファイル
doctor-scanning = スキャン中…
doctor-all-good = 参照されているファイルはすべて見つかりました。再リンクは不要です。
doctor-intro = 参照されている { $count } 個のファイルがこのコンピューターで見つかりません。それぞれの新しい場所を指定してください。使用中のすべてのシーンが一度に修正されます。
doctor-relinked = { $count } 件の参照を再リンクしました。
doctor-uses = { $count }× 使用
doctor-locate = 場所を指定…
doctor-locate-folder = フォルダー内を検索…
doctor-locate-folder-hint = フォルダーを選ぶと、各ファイルを名前で照合して再リンクします。
doctor-kind-image = 画像
doctor-kind-media = メディア
doctor-kind-slideshow = スライドショー
doctor-kind-font = フォント
doctor-kind-lut = LUT
doctor-kind-mask = マスク
history-relinkFiles = ファイルを再リンク

# --- ScriptsDialog.tsx ---
scripts-title = スクリプト（Lua）
scripts-empty = まだスクリプトがありません — .lua ファイルを追加してください。APIは scripts/sample.lua を参照: 配信開始/シーン/録画イベントに反応し、リモートAPIと同じコマンドを操作します。
scripts-enable = { $path } を有効化
scripts-remove = { $path } を削除
scripts-path-label = スクリプトのパス
scripts-add = 追加
scripts-note = スクリプトはサンドボックスで実行されます — ファイルやOSへのアクセスはできません。リモートAPIと同じスタジオコマンド（シーン切り替え、トランジション、録画/配信/リプレイ、ミュート）だけを呼び出せます。スクリプトのエラーはログに記録され、封じ込められます。変更は1秒以内に適用されます。
scripts-error-not-lua = .lua ファイルを指定してください。

# --- BrowserDock.tsx ---
browser-dock-title = ブラウザドック
browser-dock-empty = まだドックがありません — チャットのポップアウト、アラートページ、またはCompanionのWebボタンを追加してください。
browser-dock-open = 開く
browser-dock-remove = { $name } を削除
browser-dock-name-placeholder = 名前（例: Twitch Chat）
browser-dock-name-label = ドック名
browser-dock-url-label = ドックのURL
browser-dock-note = ドックはスタジオの横に配置できる独自のウィンドウとして開きます。ページはアプリにアクセスできません — 単にレンダリングするだけです。http(s) のURLのみ。ドックは「開く」をクリックしたときだけ開きます。
browser-dock-error-name = ドックに名前を付けてください（例: Twitch Chat）。
browser-dock-error-url = ドックのURLは http:// または https:// で始まる必要があります。

# --- studio-preview-pane ---
studio-preview-label = スタジオモードのプレビュー
studio-preview-heading = プレビュー
studio-preview-hint = シーンをクリックしてここに読み込みます
studio-preview-empty = ここにプレビューが表示されます。
studio-preview-mirrors = プログラムをミラーリング
studio-preview-transition-select = トランジション
studio-preview-duration = トランジションの長さ (ms)
studio-preview-commit-title = プレビュー → プログラム をトランジションで確定（視聴者に表示されます）
studio-preview-transitioning = トランジション中…
studio-preview-transition-button = トランジション ⇄
studio-preview-luma-placeholder = グレースケールのワイプ画像 (png/jpg)
studio-preview-luma-label = ルマワイプ画像
studio-preview-browse = 参照…
studio-preview-filter-images = 画像
studio-preview-filter-video = 動画
studio-preview-stinger-placeholder = スティンガー動画 (ProRes 4444 .mov はアルファを保持します)
studio-preview-stinger-label = スティンガー動画ファイル
studio-preview-stinger-cut-label = スティンガーのカットポイント (ms)
studio-preview-stinger-cut-title = スティンガーの下でシーンが切り替わるタイミング（トランジション開始からのms）

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = カット
transition-kind-fade = フェード
transition-kind-slide-left = スライド ←
transition-kind-slide-right = スライド →
transition-kind-slide-up = スライド ↑
transition-kind-slide-down = スライド ↓
transition-kind-swipe-left = スワイプ ←
transition-kind-swipe-right = スワイプ →
transition-kind-luma-linear = ルマワイプ（リニア）
transition-kind-luma-radial = ルマワイプ（放射状）
transition-kind-luma-horizontal = ルマワイプ（水平）
transition-kind-luma-diamond = ルマワイプ（ダイヤモンド）
transition-kind-luma-clock = ルマワイプ（時計）
transition-kind-image = 画像ワイプ（カスタム）
transition-kind-stinger = スティンガー（動画）

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = カスタム (RTMP/RTMPS)
stream-service-srt = SRT（セルフホスト）
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = バージョン情報
about-tagline = スタジオのように録画・配信 — アカウント不要、クラウド不要。
about-version = バージョン
about-created-by = 作成者
about-project-started = プロジェクト開始
about-first-stable = 初の安定版リリース
about-first-stable-pending = まだです — 1.0.0 は進行中
about-platform = プラットフォーム
about-local-first = Freally Capture はすべてあなたのマシン上で動作します。アカウントなし、テレメトリなし、クラウドなし — あなたのコンピューターから出ていくのは、あなたが送ることを選んだ配信だけです。
about-website = ウェブサイト
about-issues = 問題を報告
about-license = ライセンス
about-eula = EULA
about-third-party = サードパーティの通知
about-check-updates = アップデートを確認…

# --- unified settings modal (TASK-906) ---
settings-title = 設定
settings-language-section = 言語
settings-language = インターフェース言語
settings-language-system = システムデフォルト
settings-language-note = ここで選んだ言語は記憶されます。「システムデフォルト」はお使いのオペレーティングシステムに従います。翻訳されていないテキストは英語にフォールバックします。
settings-appearance-section = 外観
settings-theme = テーマ
settings-theme-dark = ダーク
settings-theme-light = ライト
settings-theme-custom = カスタム
settings-accent = アクセント
settings-general-section = 一般
settings-show-stats-dock = 統計パネルを表示
settings-more-section = その他の設定
settings-open-output = 録画…
settings-open-stream = 配信…
settings-open-replay = リプレイ…
settings-open-hotkeys = ホットキー…
settings-open-remote = リモートAPI…
settings-open-about = バージョン情報…
controls-settings = ⚙ 設定…
controls-settings-title = 言語、外観、アプリ全体の設定

# --- command palette (TASK-904) ---
palette-title = コマンドパレット
palette-search = シーン、ソース、アクションを検索
palette-placeholder = シーン、ソース、アクションを検索…
palette-no-results = “{ $query }” に一致するものがありません
palette-hint = ↑ ↓ で移動 · Enter で実行 · Esc で閉じる
palette-group-scenes = シーン
palette-group-sources = ソース
palette-group-actions = アクション
palette-transition = プレビュー → プログラムへトランジション
palette-save-replay = リプレイを保存
palette-add-marker = チャプターマーカーを打つ
palette-vertical-canvas = 縦型（9:16）キャンバス…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Freally Capture へようこそ
wizard-welcome = かんたんな2ステップです: お使いのマシンにできることを確認して、シーンを始めましょう。30秒ほどで終わり、あとから何でも変更できます。
wizard-local-first = ここでの操作は、あなたのコンピューターから外に出ることはありません。Freally Capture にはアカウントも、テレメトリも、クラウドもありません。
wizard-start = 始める
wizard-skip = スキップ
wizard-hardware-title = お使いのマシンにできること
wizard-probing = グラフィックカードとプロセッサーを確認しています…
wizard-encoder = エンコーダ
wizard-canvas = キャンバス
wizard-bitrate = ビットレート
wizard-probe-found = 検出: { $gpus } · { $cores } 物理コア
wizard-no-gpu = 専用GPUなし
wizard-apply = この設定を使う
wizard-keep-current = 今のままにする
wizard-template-title = シーンから始める
wizard-template-screen = 画面をキャプチャ
wizard-template-screen-note = メインモニターの画面キャプチャを追加します。最もよくある始め方です。
wizard-template-empty = 空の状態で始める
wizard-template-empty-note = 空のシーンです。ソースは + ボタンで自分で追加します。
wizard-done = 準備ができました。
wizard-done-hint = いつでも Ctrl+K を押すと、シーン、ソース、アクションを検索できます。設定は ⚙ ボタンの中にあります。
wizard-close = 配信を始める

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = グラフィックカードが自分で映像をエンコードできるので、プロセッサーはスタジオの他の処理にそのまま使えます。
autoconfig-reason-software = 使えるハードウェアエンコーダが見つからなかったので、プロセッサーがエンコードします。問題なく動きますが、CPUをより多く使います。
autoconfig-reason-quality-hardware = 60フレーム毎秒の 1080p で、主要なすべてのプラットフォームが受け入れるビットレートです。
autoconfig-reason-quality-software = 30フレーム毎秒です。60でのソフトウェアエンコードは、ほとんどのプロセッサーでフレームを落とすためです。
autoconfig-reason-quality-low-cores = 低めのビットレートです。このプロセッサーはコアが少なく、ソフトウェアエンコードがコンポジターとコアを奪い合うためです。

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = 録画を開始しました
announce-recording-paused = 録画を一時停止しました
announce-recording-stopped = 録画を停止しました
announce-live-started = ライブ配信中です
announce-live-ended = 配信を終了しました
announce-reconnecting = 接続が切れたため再接続しています
announce-stream-failed = 配信に失敗しました
announce-frames-dropped = { $count } フレームをドロップしました

# CAP-M01 — undo/redo edit history
palette-undo = 元に戻す
palette-redo = やり直す
palette-edit-history = 編集履歴…
history-title = 編集履歴
history-empty = 元に戻す操作はまだありません。
history-current = 現在の状態
history-close = 閉じる
history-addScene = シーンを追加
history-renameScene = シーン名を変更
history-removeScene = シーンを削除
history-reorderScene = シーンを並べ替え
history-addSource = ソースを追加
history-removeSource = ソースを削除
history-reorderSource = ソースを並べ替え
history-renameSource = ソース名を変更
history-transformSource = ソースを移動
history-toggleVisibility = 表示を切り替え
history-toggleLock = ロックを切り替え
history-setBlendMode = ブレンドモードを変更
history-editSourceProperties = プロパティを編集
history-applyLayout = レイアウトを配置
history-moveToSeat = 位置に移動
history-groupSources = ソースをグループ化
history-ungroupSources = グループを解除
history-toggleGroupVisibility = グループを切り替え
history-setSceneAudio = シーン音声
history-setVerticalCanvas = 縦型キャンバス
history-addFilter = フィルターを追加
history-removeFilter = フィルターを削除
history-reorderFilter = フィルターを並べ替え
history-editFilter = フィルターを編集
history-toggleFilter = フィルターを切り替え
history-setVolume = 音量を調整
history-toggleMute = ミュートを切り替え
history-setMonitor = モニタリングを変更
history-setTracks = トラックを変更
history-setSyncOffset = A/V同期を調整
history-setAudioHotkeys = 音声ショートカット

# CAP-M04 — alignment aids
settings-alignment-section = 整列補助
settings-smart-guides = スマートガイド（ドラッグ時にスナップ）
settings-safe-areas = セーフエリア表示
settings-rulers = ルーラー
align-group = キャンバスに整列
align-left = 左に整列
align-hcenter = 水平方向に中央揃え
align-right = 右に整列
align-top = 上に整列
align-vcenter = 垂直方向に中央揃え
align-bottom = 下に整列

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = 選択を整列・分布
arrange-left = 左端を揃える
arrange-hcenter = 水平方向に中央揃え
arrange-right = 右端を揃える
arrange-top = 上端を揃える
arrange-vcenter = 垂直方向に中央揃え
arrange-bottom = 下端を揃える
distribute-h = 水平方向に分布
distribute-v = 垂直方向に分布
guides-group = ガイド
guides-add-v = 垂直ガイドを追加
guides-add-h = 水平ガイドを追加
guides-clear = すべてのガイドを消去
history-arrangeItems = アイテムを整列
history-editGuides = ガイドを編集

# CAP-M05 — edit transform + copy/paste
transform-title = 変形を編集 — { $name }
transform-anchor = アンカー
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = 回転
transform-crop = クロップ
transform-crop-left = 左
transform-crop-top = 上
transform-crop-right = 右
transform-crop-bottom = 下
transform-no-size = サイズとクロップは、ソースが寸法を報告すると利用できます。
transform-copy = 変形をコピー
transform-paste = 変形を貼り付け
transform-close = 閉じる
filters-copy = フィルターをコピー ({ $count })
filters-paste = フィルターを貼り付け ({ $count })
palette-edit-transform = 変形を編集…
history-pasteFilters = フィルターを貼り付け

# CAP-M26 — keying workbench
workbench-title = キーイング作業台 — { $name }
workbench-mode-keyed = キー適用
workbench-mode-source = ソース
workbench-mode-matte = マット
workbench-mode-split = 分割
workbench-eyedropper = スポイト
workbench-eyedropper-hint = ソースをクリックしてキー色を取得します。
workbench-loupe = ルーペ
workbench-split = 分割
workbench-preview-alt = キーイング作業台のプレビュー
workbench-tune = 調整
workbench-close = 閉じる

# CAP-M06 — multiview monitor
multiview-title = マルチビュー
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = シーンをクリックして切り替えます。
multiview-hint-stage = シーンをクリックしてプレビューに準備します。
palette-multiview = マルチビューモニター

# CAP-M07 — projectors
projector-title = プロジェクターを開く
projector-source = ソース
projector-target-program = プログラム
projector-target-preview = プレビュー
projector-target-scene = シーン…
projector-target-source = ソース…
projector-target-multiview = マルチビュー
projector-which-scene = どのシーン
projector-which-source = どのソース
projector-none = 表示するものがありません
projector-display = ディスプレイ
projector-windowed = フローティングウィンドウ（この画面）
projector-display-option = ディスプレイ { $n } — { $w }×{ $h }
projector-primary = （メイン）
projector-open = 開く
projector-cancel = キャンセル
projector-exit-hint = Esc キーで終了
palette-projector = プロジェクターを開く…

# CAP-M08 — still-frame grab
palette-still = 静止画をキャプチャ…
still-saved-toast = 静止画を保存しました: { $name }
still-failed-toast = 静止画のキャプチャに失敗しました: { $error }
hotkeys-still = 静止画をキャプチャ
hotkeys-still-placeholder = 例: Ctrl+Shift+P

# CAP-M13 — source health dashboard
palette-source-health = ソースの状態…
palette-av-sync = A/V 同期キャリブレーション…
palette-hotkey-audit = ホットキー一覧…
health-title = ソースの状態
health-col-source = ソース
health-col-state = 状態
health-col-resolution = 解像度
health-col-fps = FPS
health-col-last-frame = 最終フレーム
health-col-dropped = ドロップ
health-col-retries = 再起動回数
health-col-actions = 操作
health-state-live = ライブ
health-state-waiting = 待機中
health-state-error = エラー
health-state-inactive = 非アクティブ
health-restart = 再起動
health-properties = プロパティ
health-empty = このコレクションにはまだソースがありません。
health-seconds = { $value } 秒

# CAP-M23 — quit guard + orderly shutdown
quit-title = Freally Capture を終了しますか？
quit-body = 今終了すると、次の処理が順に安全に実行されます：
quit-consequence-stream = ライブ配信を終了し、サービスから切断します。
quit-consequence-recording = 録画を停止し、ファイルをファイナライズします。
quit-consequence-replay = リプレイバッファを停止します — 未保存のリプレイ映像は破棄されます。
quit-confirm = 安全に終了
quit-quitting = シャットダウン中…
quit-cancel = キャンセル

# CAP-M11 — crash-safe recording salvage
salvage-title = 中断された録画を復旧しますか？
salvage-body = 前回のセッションは、これらの録画の書き込み中に予期せず終了しました。修復すると元のファイルの隣に再生可能なコピーが作成されます — 元のファイルは変更されません。
salvage-repair = 修復
salvage-repairing = 修復中…
salvage-done = 修復済み
salvage-repaired = 修復済み → { $name }
salvage-failed = 修復に失敗しました: { $error }
salvage-dismiss = 後で

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = エンコーダー障害 — { $from } から { $to } に切り替えました。配信は再接続され継続中です。
fallback-toast-recording = エンコーダー障害 — { $from } から { $to } に切り替えました。録画は新しいファイルで継続します。
fallback-note = エンコーダーのフォールバック: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = プログラム音声が無音になりました
alarm-clipping = プログラム音声がクリッピングしています
alarm-black = プログラム映像が真っ黒です
alarm-frozen = プログラム映像がしばらく変化していません
alarm-lowDisk = ディスク容量: 現在のビットレートで残り約 { $minutes } 分
alarm-dismiss = アラームを閉じる
alarm-cleared = 解消: { $alarm }

# CAP-M22 — panic button
palette-panic = パニック — プライバシースレートに切り替え
panic-banner-title = パニック
panic-banner-body = プログラムはプライバシースレートを表示中。音声はすべてミュート、キャプチャは停止。配信と録画は継続します。
panic-restore = 復帰…
panic-restore-confirm = プログラムを復帰しますか？
panic-restore-yes = 復帰
panic-restore-cancel = キャンセル
hotkeys-panic = パニック（プライバシースレート）
hotkeys-panic-placeholder = 例: Ctrl+Shift+F12
hotkeys-timer-toggle = すべてのタイマーを開始/一時停止
hotkeys-timer-toggle-placeholder = 例：Ctrl+Shift+T
hotkeys-timer-reset = すべてのタイマーをリセット
hotkeys-timer-reset-placeholder = 例：Ctrl+Shift+0
panic-slate-color = パニックスレートの色
panic-slate-image = パニックスレートの画像
panic-slate-image-placeholder = 任意の画像パス

# CAP-M24 — redacted diagnostics bundle
diag-title = 診断バンドル
diag-intro = GitHub Issue に手動で添付するための赤入れ済み .zip（設定スナップショット、エンコーダープローブ、最近の統計 — 機密・パス・名前は決して含まれません）をエクスポートします。どこにも送信されません。
diag-preview = 内容を確認
diag-hide-preview = プレビューを隠す
diag-export = .zip をエクスポート
diag-exported = エクスポート済み: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = 配信前チェック
preflight-intro = ブロック項目はすべて緑である必要があります。残りは正直な注意です。
preflight-item-targets = 配信先の設定（キー/URL）
preflight-item-encoder = 使用可能なエンコーダーあり
preflight-item-sources = すべてのソースが正常
preflight-item-disk = 録画用のディスク容量
preflight-item-mic = マイクの音量メーター
preflight-item-desktopAudio = デスクトップ音声のメーター
preflight-item-replay = リプレイバッファ待機中
preflight-targets-detail = { $count } 件有効
preflight-sources-detail = { $count } 件のソースがエラー
preflight-disk-detail = 現在のビットレートで約 { $minutes } 分
preflight-fix-stream = 配信設定…
preflight-fix-components = コンポーネント…
preflight-fix-sources = ソースの状態…
preflight-fix-replay = 待機
preflight-optional = 任意
preflight-hold = すべて緑になるまで配信開始を保留
preflight-cancel = キャンセル
preflight-go-anyway = それでも配信開始
preflight-go-live = 配信開始


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = 背景
scenes-backdrop-aria = { $name } の背景
backdrop-title = 背景 — { $name }
backdrop-hint = このシーンのすべての背後に固定される壁紙です。画像・アニメGIF・ループ動画が使えます。キャプチャは常に前面にあり、キャンバス上でスクロールするとズームできます。
backdrop-choose = 画像または動画を選択…
backdrop-remove = 背景を削除
backdrop-none = 背景は未設定です。
backdrop-position = 配置
backdrop-split-full = キャンバス全体
backdrop-split-left = 左半分
backdrop-split-right = 右半分
backdrop-split-top = 上半分
backdrop-split-bottom = 下半分
backdrop-sync = 録画開始と同時に再生を開始
backdrop-sync-hint = 録画するまで最初のフレームで待機し、テイクごとに動画を最初から再生します。
backdrop-preview-play = プレビュー再生
backdrop-preview-pause = プレビューを一時停止
backdrop-filter-all = 背景（画像と動画）
backdrop-filter-images = 画像
backdrop-filter-media = 動画とGIF
sources-backdrop-badge = 背景の壁紙（最背面に固定）
sources-backdrop-pinned = 背景は最背面に固定されたままです
filters-name-flip = 反転
filters-flip-horizontal = 左右
filters-flip-vertical = 上下
history-setSceneBackdrop = 背景を設定
history-setBackdropSplit = 背景を移動
history-setBackdropSync = 背景の録画同期
backdrop-scrub = 再生位置
backdrop-loop = ループ
backdrop-reverse = 逆再生
backdrop-reverse-hint = 逆再生は逆向きのコピーを一度だけ生成します（動画は ffmpeg コンポーネントが必要、GIF は即時）。長いファイルでは初回の切り替えに時間がかかることがあります。
filters-scaling = スケーリング
filters-scaling-hint = レトロ/ピクセル向けのピクセルパーフェクト表示。「整数」は描画サイズも整数倍にスナップします（ハンドルは論理サイズを表示）。
filters-scaling-auto = スムーズ
filters-scaling-nearest = ニアレストネイバー
filters-scaling-integer = 整数（整数倍）
filters-scaling-sharp = シャープバイリニア
history-setScaling = スケーリングを変更
hotkeys-zoom-100 = ズーム：リセット（100%）
hotkeys-zoom-150 = ズーム：150% に寄る
hotkeys-zoom-200 = ズーム：2× に寄る
hotkeys-zoom-placeholder = Ctrl+Shift+2
sources-follow-title = ズーム中にカーソルを追従（Windows。ズームはキャンバス上でスクロール）
sources-follow-item = { $name } のカーソル追従を切り替え
filters-autocrop = ✂ 黒帯を自動クロップ
filters-autocrop-title = 次のフレームをレターボックス/ピラーボックスの帯についてスキャンし、クロップします（取り消し可能）。暗いシーンは決してクロップされません。
filters-autocrop-follow = 解像度変更時に再チェック
history-autoCrop = 黒帯の自動クロップ
sources-link-audio = このアプリの音声も取り込む（連動：非表示でミュート、ウィンドウ削除で一緒に削除）
history-addLinkedWindow = ウィンドウ＋連動音声を追加
sources-hdr-title = このディスプレイは HDR — トーンマップを開く（キャンバスは SDR のまま）
sources-hdr-item = { $name } の HDR トーンマップ
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = このディスプレイは HDR 出力です。トーンマップなしではハイライトがクリップし、SDR キャンバスで白っぽく見えます。変更は次のフレームから反映されます。
sources-hdr-enable-suggested = 推奨を有効化（maxRGB、200 nit）
sources-hdr-operator = オペレーター
sources-hdr-op-clip = クリップ（オフ）
sources-hdr-op-maxrgb = maxRGB（色相保持）
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 ニー（SDR を厳密維持）
sources-hdr-paper-white = ペーパーホワイト
sources-hdr-nits = nit
projector-target-passthrough = パススルーモニター（低遅延）
projector-which-device = デバイス
projector-passthrough-none = まずディスプレイ・ウィンドウ・キャプチャデバイスを追加してください。
projector-passthrough-about = デバイスの生フレーム — シーンもフィルターもコンポジターも通しません。実測レイテンシを表示します。音声は引き続きミキサーのストリップでモニターします。
projector-passthrough-hint = パススルー — Esc で閉じる
projector-latency = { $ms } ms
projector-latency-measuring = 計測中…
controls-automation = オートメーション
controls-automation-title = Rules, macros & studio variables (CAP-N01/N02)
automation-title = オートメーション — ルール・マクロ・変数
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = ルール
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = オン
automation-rule-name = Rule name
automation-remove = Remove
automation-when = 条件
automation-then-run = 実行するマクロ
automation-no-macro = (no macro)
automation-macros = マクロ
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = 実行
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = スタジオ変数
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
controls-rundown = 進行表
controls-rundown-title = The show rundown: a timed scene playlist (CAP-N09)
rundown-title = 番組進行表
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = 開始
rundown-next = 次へ ▸
rundown-stop = 停止
rundown-idle = 停止中
rundown-next-up = 次: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + ステップ
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
automation-layer = レイヤー
automation-layer-hint = このレイヤーが有効なときだけ発火します（空欄＝全レイヤー）。レイヤーは固定式です（OS のグローバルホットキー API では押し続け式レイヤーを実現できません）。
automation-chord-hint = 単一キー（Ctrl+Shift+M）または2ストロークのコード（Ctrl+K, 3）。コードの2打目はコード待機中だけ占有されます。
controls-panel = LANパネル
controls-panel-title = The LAN touch panel + tally lights (CAP-N06/N07)
panel-title = LANパネルとタリー
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = パネルを配信
panel-port = ポート
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = パスワード
panel-show = 表示
panel-hide = 隠す
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = 保存
osc-title = OSC コントロールサーフェス
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = OSC を受信
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
controls-ptz = PTZ
controls-ptz-title = PTZ camera control — VISCA over IP (CAP-N08)
ptz-title = PTZ カメラ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = カメラ
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = アドレス
ptz-port = ポート
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
ptz-presets = プリセット
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = 呼び出し
ptz-store = 保存
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
midi-title = MIDI コントロールサーフェス
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = 入力
midi-output = 出力（フィードバック）
midi-none = (none)
midi-learn = ラーン
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = 動作
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
panel-lan-warning = ⚠ LAN 通信は暗号化されません — パスワードは平文 HTTP の URL に載ります。信頼できるネットワークでのみ使用してください。
osc-lan-warning = ⚠ OSC にパスワードはありません — ネットワーク上のどの機器でもこれらのコマンドを送れます。LAN モードは信頼できるネットワークでのみ。

# System-stats HUD source (CAP-N14)
sources-badge-stats = 統計
sources-add-system-stats = パフォーマンス統計（HUD）
sources-stats-title = パフォーマンスHUDを追加
sources-stats-note = スタジオの実測値 — fps、CPU、メモリ、レンダリング時間、ドロップフレーム、ライブビットレート — を視聴者向けにプログラムへ表示します。表示する行・サイズ・色はソースのプロパティで設定できます。GPU使用率は計測していないため表示しません。
sources-stats-add = 統計HUDを追加
properties-stats-show-fps = FPSを表示
properties-stats-show-cpu = CPUを表示
properties-stats-show-memory = メモリを表示
properties-stats-show-render = レンダリング時間を表示
properties-stats-show-dropped = ドロップフレームを表示
properties-stats-show-bitrate = ビットレートを表示
properties-stats-size = サイズ（px）
properties-stats-note = HUDは簡潔な共通ラベル（FPS, CPU, MEM, RENDER, DROPPED, BITRATE）をそのままプログラムに描画します。配信していないときはビットレート行に「—」が表示されます。

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = ビジュアライザー
sources-add-visualizer = オーディオビジュアライザー
sources-visualizer-title = オーディオビジュアライザーを追加
sources-visualizer-style-label = スタイル
sources-visualizer-style-bars = スペクトラムバー
sources-visualizer-style-scope = オシロスコープ
sources-visualizer-style-vu = VUメーター
sources-visualizer-target-label = リッスン対象
sources-visualizer-target-master = マスターミックス
sources-visualizer-target-track = トラック { $n }
sources-visualizer-note = 実際にミックスされる信号（フェーダー後）を描画します。ミュートしたソースは音のとおりフラットに表示されます。サイズ・色・バー数・降下速度はソースのプロパティで設定できます。
sources-visualizer-add = ビジュアライザーを追加
properties-vis-bands = バー数
properties-vis-decay = 降下速度（dB/s）
properties-vis-peak-hold = ピークホールド表示
properties-vis-missing-source = （ソースが見つかりません）

# Speedrun split timer source (CAP-N18)
sources-badge-splits = スプリット
sources-add-split-timer = スピードラン・スプリットタイマー
sources-splits-title = スプリットタイマーを追加
sources-splits-file-label = LiveSplit の .lss ファイル
sources-splits-comparison-label = 比較対象
sources-splits-comparison-pb = 自己ベスト
sources-splits-comparison-best = ベストセグメント
sources-splits-comparison-average = 平均
sources-splits-note = ファイルは読み取り専用でインポートされ、書き戻されることはありません。設定 → ホットキーでグローバルの Split / Undo / Skip / Reset キーを割り当ててください。プロセスメモリ方式のオートスプリッターは意図的に非対応です。
sources-splits-add = スプリットタイマーを追加
properties-splits-size = サイズ（px）
properties-splits-ahead = リード
properties-splits-behind = ビハインド
properties-splits-gold = ゴールド
properties-splits-split = スプリット
properties-splits-undo = 取り消し
properties-splits-skip = スキップ
properties-splits-reset = リセット
properties-splits-note = ボタンは実行中のタイマーを操作します（グローバルホットキーはどのアプリからでも同じ操作ができます）。走行記録が .lss に保存されることはありません。
hotkeys-split-split = スプリットタイマー：開始 / スプリット
hotkeys-split-undo = スプリットタイマー：スプリットを取り消す
hotkeys-split-skip = スプリットタイマー：セグメントをスキップ
hotkeys-split-reset = スプリットタイマー：リセット
hotkeys-split-placeholder = 例：Numpad1
hotkey-audit-action-split-split = スプリット（スプリットタイマー）
hotkey-audit-action-split-undo = スプリットを取り消す
hotkey-audit-action-split-skip = セグメントをスキップ
hotkey-audit-action-split-reset = スプリットタイマーをリセット
hotkey-audit-feature-split-timer = スプリットタイマー

# Media playlist source (CAP-N17)
sources-badge-playlist = プレイリスト
sources-add-playlist = メディアプレイリスト（ギャップレス）
sources-playlist-title = メディアプレイリストを追加
sources-playlist-files-label = ファイル（1 行に 1 つ、上から順に再生）
sources-playlist-browse = 参照…
sources-playlist-loop = ループ
sources-playlist-shuffle = シャッフル（開始ごとに 1 回抽選。ループ時は同じ順を繰り返します）
sources-playlist-hold-last = 終了時に最後のフレームを保持
sources-playlist-note = トリム済みリスト全体を、表示付き ffmpeg コンポーネント経由でギャップレス再生します（wire 形式のみ — .frec と静止画は Media/スライドショーで）。項目はすべて動画かすべて音声で、混在はできません。トリム・キューポイント・「再生中」変数はプロパティにあります。
sources-playlist-add = プレイリストを追加
properties-playlist-items = 項目（上から順に再生）
properties-playlist-up = 上へ
properties-playlist-down = 下へ
properties-playlist-remove = 項目を削除
properties-playlist-in = 開始 (秒)
properties-playlist-out = 終了 (秒)
properties-playlist-cues = キュー（秒、カンマ区切り）
properties-playlist-add-item = + 項目を追加
properties-playlist-loop = ループ
properties-playlist-shuffle = シャッフル
properties-playlist-hold-last = 最後のフレームを保持
properties-playlist-hw = ハードウェアデコード
properties-playlist-variable = 「再生中」変数（空 = 無効）
properties-playlist-previous = ⏮ 前へ
properties-playlist-next = ⏭ 次へ
properties-playlist-note = キューと「次へ／前へ」は再生中のプレイリストを操作します。項目の変更は「適用」で反映されます（プレイリストは再起動）。テキストソースに {"{{"}yourVariable{"}}"} を入れると再生中の項目名を表示できます。
hotkeys-playlist-next = プレイリスト：次の項目
hotkeys-playlist-previous = プレイリスト：前の項目
hotkeys-playlist-placeholder = 例：Ctrl+Alt+Right
hotkey-audit-action-playlist-next = プレイリスト次へ
hotkey-audit-action-playlist-previous = プレイリスト前へ
hotkey-audit-feature-playlist = プレイリスト

# Instant replay source (CAP-N10)
sources-badge-replay = リプレイ
sources-add-replay = インスタントリプレイ
sources-replay-title = インスタントリプレイを追加
sources-replay-seconds-label = ロールの長さ（秒）
sources-replay-speed-label = 速度
sources-replay-speed-full = 100%（音声あり）
sources-replay-speed-half = 50% スロー（無音）
sources-replay-speed-quarter = 25% スロー（無音）
sources-replay-note = ロールするまで透明のままです。リプレイバッファを有効化（コントロール）し、ロールのホットキーを割り当ててください。ロールはバッファ直近の映像を切り出して番組に再生し、終わると透明に戻ります。
sources-replay-add = インスタントリプレイを追加
properties-replay-roll = ⏵ リプレイをロール
properties-replay-note = ロールは有効化済みバッファをクリップにして選択した速度で再生します（リタイミングのみ、補間なし）。スローは意図的に無音です。再生中はスクラブ／一時停止が使え、終了後ソースは透明に戻ります。
hotkeys-replay-roll = インスタントリプレイ：ロール
hotkeys-replay-roll-placeholder = 例：Ctrl+Shift+I
hotkey-audit-action-replay-roll = インスタントリプレイをロール

# Input overlay source (CAP-N13)
sources-badge-input = 入力
sources-add-input-overlay = 入力オーバーレイ（キー/パッド）
sources-input-title = 入力オーバーレイを追加
sources-input-layout-label = レイアウト
sources-input-layout-wasd = WASD + マウス
sources-input-layout-keyboard = コンパクトキーボード + マウス
sources-input-layout-gamepad = ゲームパッド（デュアルスティック）
sources-input-layout-fightstick = アケコン
sources-input-color-label = キー
sources-input-accent-label = 押下時
sources-input-privacy-note = プライバシー: 入力はこのソースがシーンでライブの間だけ読み取られ、レイアウトの固定キーのみをポーリングします — 「今押されているか」を見る瞬間的な確認で、フックは一切使いません。何も記録・保存・送信されず、入力した文章がキャプチャされることもありません。
sources-input-os-note = キーボードとマウスの状態は現在 Windows でのみ読み取れます — 他の OS ではキーは押されていない状態で描かれます（正直に明記し、偽装はしません）。ゲームパッドは gilrs ライブラリでどの OS でも動作し、最初に接続されたコントローラーを描画します。見つからない場合は未押下のまま表示します。
sources-input-add = 入力オーバーレイを追加

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = カーソル効果
filters-cursorfx-hint = Windows ではアプリ自身がカーソルを描くため、効果はキャプチャに直接描き込まれ、録画や配信にもそのまま映ります。macOS と Linux は OS 側でカーソルを合成するため、この効果は Windows 専用です。変更は即時に反映されます。
filters-cursorfx-halo = カーソルハロー
filters-cursorfx-halo-color = 色
filters-cursorfx-halo-radius = 半径 (px)
filters-cursorfx-ripples = クリック波紋
filters-cursorfx-left-color = 左クリック
filters-cursorfx-right-color = 右クリック
filters-cursorfx-keystrokes = キー表示
filters-cursorfx-keystrokes-hint = 押している間だけ、固定のキーセット（文字・数字・修飾キー・矢印）をカーソルの近くに表示します。キーはこの機能が有効な間しか読み取られず、フレームに直接描かれるだけで、保存もログ記録も一切されません。

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = タイトル
sources-add-title = タイトル / スコアボード
sources-title-title = タイトルを追加
sources-title-template-label = テンプレート
sources-title-template-lower-third = ローワーサード(バー+名前+肩書き)
sources-title-template-scoreboard = スコアボード(プレート+4セル)
sources-title-template-blank = 空のキャンバス
sources-title-width-label = キャンバス幅
sources-title-height-label = キャンバス高さ
sources-title-template-name = 名前
sources-title-template-subtitle = 肩書き
sources-title-template-home = ホーム
sources-title-template-away = アウェイ
sources-title-note = テキスト/画像/ボックスのレイヤーで作るタイトル。イン/アウトのアニメーション付きでローカル合成 — ブラウザーソースではありません。レイヤー、ファイル連携と {"{{"}変数{"}}"}、ライブ操作はソースのプロパティにあります。
sources-title-add = タイトルを追加
properties-title-layers = レイヤー(順に描画 — 後の行が上に重なる)
properties-title-kind-text = テキスト
properties-title-kind-image = 画像
properties-title-kind-rect = ボックス
properties-title-x = X
properties-title-y = Y
properties-title-outline = 縁取り (px)
properties-title-outline-color = 縁取り
properties-title-shadow = 影
properties-title-animation = イン/アウトのアニメーション
properties-title-anim-none = なし(カット)
properties-title-anim-fade = フェード
properties-title-anim-slide-left = 左へスライド
properties-title-anim-slide-up = 上へスライド
properties-title-anim-wipe = ワイプ
properties-title-duration = 長さ (ms)
properties-title-fire-in = ▶ イン再生
properties-title-fire-out = ◼ アウト再生
properties-title-set-live = ライブ反映
properties-title-set-live-note = このテキストを今すぐ表示中のタイトルへ反映します — 適用も再起動も不要
properties-title-up = レイヤーを上へ
properties-title-down = レイヤーを下へ
properties-title-remove = レイヤーを削除
properties-title-add-text = + テキスト
properties-title-add-image = + 画像
properties-title-add-rect = + ボックス
properties-title-note = イン/アウトと「ライブ反映」は動作中のタイトルを直接操作します。レイヤー編集は「適用」で反映されます(タイトルは再起動し、再度インします)。テキストセルは監視ファイル(CSVセル / JSON値 / ファイル全体)に連携でき、{"{{"}変数{"}}"} を展開します — 「ライブ反映」は両方より優先されます。

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = LAN インジェスト（SRT/RTMP リスナー）
sources-lan-title = LAN インジェストリスナーを追加
sources-lan-protocol-label = プロトコル
sources-lan-protocol-srt = SRT（暗号化可 — 推奨）
sources-lan-protocol-rtmp = RTMP（認証なし）
sources-lan-port-label = ポート（1024–65535）
sources-lan-passphrase-label = パスフレーズ（空 = オープン）
sources-lan-passphrase-hint = SRT のパスフレーズは 10〜79 文字です。送信側も同じものを使う必要があります。
sources-lan-open-warning = パスフレーズなし: このネットワーク上の誰でも暗号化なしでこのソースに送信できます。自分だけのネットワークでない限り設定してください。
sources-lan-rtmp-warning = RTMP には認証がありません — このネットワーク上の誰でもこのポートに送信できます。パスフレーズ付きの SRT を推奨します。
sources-lan-url-label = 送信側アプリの宛先
sources-lan-qr-aria = インジェスト URL の QR コード
sources-lan-note = LAN 専用: このマシンのローカルアドレスで、ソースが存在する間だけ待ち受け、インターネットには一切触れません — あなたのネットワーク上の送信者が先に送らない限り、何もマシンから出ていきません。デコードは明示された ffmpeg コンポーネントで行います。送信者が接続するまで、キャンバスにはこの URL が表示されます。
sources-lan-add = 待ち受け開始
properties-lan-note = プロトコル・ポート・パスフレーズの変更を適用するとリスナーが再起動します — 送信側は再接続が必要です。ストリームは 1920×1080 のキャンバスにフィットされます。

# Freally Link source & output (CAP-N12)
sources-badge-link = リンク
sources-add-freally-link = Freally Link(別のインスタンス)
sources-link-title = Freally Link を追加
sources-link-about = 自分のネットワーク経由で、別の Freally Capture インスタンスのプログラム(映像とマスター音声)を受信します。先に送信側で「Freally Link 出力」を有効にしてください。v1 は TCP 上のモーション JPEG で配信します。有線 LAN や良好な Wi-Fi に最適で、弱い回線では帯域について正直に振る舞います。
sources-link-scan = LAN をスキャン
sources-link-scanning = スキャン中…
sources-link-none = Freally Link の出力が見つかりません。相手のインスタンスで「Freally Link 出力」を有効にする(コントロール → LAN パネル)か、下にアドレスを入力してください。
sources-link-host = アドレス
sources-link-port = ポート
sources-link-key = ペアリングキー
sources-link-key-hint = 送信側の「Freally Link 出力」設定にあるキー。これがないと送信側は 1 フレームも配信しません。
sources-link-add = リンクを追加
properties-link-note = 未接続の間は「接続中」の画面を表示し、待ち時間を増やしながら自動で再試行します。古いフレームで固まることはありません。送信側 1 台につき受信側は 1 台。使用中の送信側へは礼儀正しく再試行します。
link-title = Freally Link 出力
link-about = このインスタンスのプログラム(映像とマスター音声)を、自分のネットワーク上のもう 1 台の Freally Capture と共有します。相手側では「Freally Link」ソースとして表示されます(2 台構成の配信や控え室モニターに)。既定ではオフ。有効にするまで何も告知せず、待ち受けもしません。v1 は TCP でモーション JPEG と非圧縮音声を送ります。有線 LAN か良好な Wi-Fi 向けで、インターネットには決して出ません。
link-enable = 自分のネットワークでプログラムを共有
link-name = インスタンス名
link-key = ペアリングキー
link-key-hint = 8 文字以上。受信側がこのキーを入力するまで、1 フレームも配信されません。
link-lan-warning = ⚠ 受信側はペアリングキーを提示するまで何も受信できませんが、ストリーム自体は v1 では暗号化されません。信頼できるネットワークでのみ使用してください。
link-serving = 受信側は「LAN をスキャン」でこのインスタンスを見つけるか、次のアドレスで手動追加できます:
link-off-hint = 共有を有効にするとポートが開き、LAN スキャンにこのインスタンスが告知されます。
