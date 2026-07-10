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
sources-add-nested-scene = ネストされたシーン
sources-add-slideshow = 画像スライドショー
sources-add-chat-overlay = ライブチャットオーバーレイ
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
