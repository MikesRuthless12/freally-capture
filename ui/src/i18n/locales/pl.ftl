# Freally Capture — pl
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Tryb studyjny
toggle-on = wł.
toggle-off = wył.
stats = Statystyki
core-ok = rdzeń OK
hide-stats-dock = Ukryj panel statystyk
show-stats-dock = Pokaż panel statystyk


# =============================================================
# --- shell ---
# =============================================================

# --- App shell (App.tsx) ---
app-save-error = Nie udało się zapisać ustawień — zmiana nie przetrwa ponownego uruchomienia.
studio-mode-leave = Wyjdź z trybu studyjnego
studio-mode-enter-title = Tryb studyjny — edytuj scenę podglądu i przenieś ją do programu za pomocą przejścia
vertical-canvas-title = Drugie płótno wyjściowe (pionowe 9:16) — można je nagrywać i transmitować niezależnie
app-version = v{ $version }
core-error = rdzeń BŁĄD
core-unreachable = rdzeń nieosiągalny (tryb przeglądarki)
connecting-to-core = łączenie z rdzeniem…
filters-source-fallback = Źródło

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Podgląd programu
preview-program-output = Wyjście programu
preview-canvas-editor = Edytor płótna
preview-px-to-edge-label = Piksele do krawędzi kadru
preview-px-to-edge = px do krawędzi L { $left } · G { $top } · P { $right } · D { $bottom }
preview-program-heading = Program
preview-no-gpu = Nie znaleziono użytecznego adaptera GPU — kompozytor nie może działać na tym komputerze.
preview-starting-compositor = Uruchamianie kompozytora…
preview-empty-scene = Ta scena jest pusta — dodaj źródło w panelu Źródła, a następnie przeciągaj, skaluj i obracaj je tutaj na płótnie.
preview-fps = { $fps } fps
preview-dropped = { $dropped } utraconych

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Otrzymano link z zaproszeniem
remote-join-with-webcam = Dołącz z kamerą
remote-dismiss = Odrzuć
remote-hosting-guest = Hostujesz zdalnego gościa
remote-you-are-guest = Jesteś zdalnym gościem
remote-share-view-title = Udostępnij swój ekran w aplikacji gościa (widzi Twój widok na żywo)
remote-stop-sharing-view = Zatrzymaj udostępnianie widoku
remote-share-my-view = Udostępnij mój widok
remote-allow-center-title = Pozwól gościowi przełączać, który widok zajmuje centrum (nadal masz kontrolę i możesz w każdej chwili przełączyć z powrotem)
remote-guest-switching = Przełączanie przez gościa:
remote-stop-screen = Zatrzymaj ekran
remote-share-screen = Udostępnij ekran
remote-share-screen-title-guest = Udostępnij swój ekran hostowi (staje się źródłem, które może wyśrodkować)
remote-center-request-label = Prośba o widok centralny
remote-center = Wyśrodkuj
remote-center-cam-title = Poproś hosta o wyśrodkowanie Twojej kamery
remote-center-my-cam = Moja kamera
remote-center-screen-title = Poproś hosta o wyśrodkowanie Twojego udostępnionego ekranu
remote-center-my-screen = Mój ekran
remote-center-host-title = Oddaj centrum widokowi hosta
remote-center-host-view = Widok hosta
remote-end-session = Zakończ sesję
remote-leave = Opuść
remote-host-view-heading = Widok hosta
remote-host-shared-view-label = Udostępniony widok hosta
remote-guest-position-label = Pozycja gościa
remote-guest-label = Gość
remote-put-guest = Umieść gościa { $position }
remote-remove-title = Usuń gościa — może dołączyć ponownie z tym samym linkiem
remote-remove = Usuń
remote-ban-title = Zbanuj gościa — blokuje go i unieważnia link z zaproszeniem
remote-ban = Zbanuj
remote-guest-self-muted = gość sam się wyciszył
remote-unmute-guest = Wyłącz wyciszenie gościa
remote-mute-guest = Wycisz gościa
remote-muted-by-host = Wyciszony przez hosta
remote-unmute-mic = Wyłącz wyciszenie mikrofonu
remote-mute-mic = Wycisz mikrofon
remote-waiting-for-host = oczekiwanie na hosta


# =============================================================
# --- sources-rail ---
# =============================================================

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = źródło
sources-fallback-video = wideo
sources-fallback-error = błąd
sources-kind-unknown = ?
sources-missing-source = (brak źródła)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Ekran
sources-badge-window = Okno
sources-badge-portal = Portal
sources-badge-camera = Kamera
sources-badge-image = Obraz
sources-badge-media = Media
sources-badge-guest = Gość
sources-badge-color = Kolor
sources-badge-text = Tekst
sources-badge-scene = Scena
sources-badge-slides = Slajdy
sources-badge-chat = Czat
sources-badge-audio-in = Wejście audio
sources-badge-audio-out = Wyjście audio
sources-badge-app-audio = Audio aplikacji

# Add-source menu items
sources-add-display = Przechwytywanie ekranu
sources-add-window = Przechwytywanie okna
sources-add-game = Przechwytywanie gry (przeczytaj najpierw)
sources-add-webcam = Urządzenie przechwytujące obraz
sources-add-image = Obraz
sources-add-media = Media (plik wideo/obraz)
sources-add-remote-guest = Zdalny gość (próba P2P)
sources-add-color = Kolor
sources-add-text = Tekst
sources-add-nested-scene = Scena zagnieżdżona
sources-add-slideshow = Pokaz slajdów
sources-add-chat-overlay = Nakładka czatu na żywo
sources-add-audio-input = Przechwytywanie wejścia dźwięku
sources-add-audio-output = Przechwytywanie wyjścia dźwięku
sources-add-app-audio = Dźwięk aplikacji (Windows)
sources-add-existing = Istniejące źródło…

# Panel header + toolbar buttons
sources-panel-title = Źródła
sources-group-title = Grupuj źródła — wybierz co najmniej dwa elementy, a następnie Utwórz grupę; zgrupowane elementy przesuwają się oraz pokazują/ukrywają razem
sources-group-aria = Grupuj źródła
sources-arrange = Rozmieść: ekran + rogi
sources-add-source = Dodaj źródło
sources-browser-source-note = Źródło przeglądarki jest dostarczane jako osobny, pobierany na żądanie komponent (silnik Chromium o rozmiarze ~180 MB — nigdy nie jest dołączany). Na razie: przechwyć prawdziwe okno przeglądarki za pomocą Przechwytywania okna + klucza chrominancji/koloru albo otwórz czat/alerty jako dok (Sterowanie → Doki).

# Empty state
sources-empty = Brak źródeł w tej scenie — dodaj Przechwytywanie ekranu, Okno, Kamerę, Obraz, Kolor lub Tekst za pomocą „+”. Przeciągaj, skaluj i obracaj je na płótnie; przyciski po prawej stronie zmieniają kolejność w stosie.

# Per-row controls
sources-already-in-group = Już w { $name }
sources-pick-for-new-group = Wybierz do nowej grupy
sources-pick-item-for-group = Wybierz { $name } do nowej grupy
sources-hide = Ukryj
sources-show = Pokaż
sources-hide-item = Ukryj { $name }
sources-show-item = Pokaż { $name }
sources-unfocus-title = Wyłącz skupienie — przywróć układ
sources-focus-title = Skup — wypełnij płótno (Wyróżnij mówiącego)
sources-unfocus-item = Wyłącz skupienie na { $name }
sources-focus-item = Skup na { $name }
sources-center-title = Wyśrodkuj — ustaw to jako wspólny widok centralny (kamery przenoszą się na pasek)
sources-center-item = Wyśrodkuj { $name }
sources-rename-item = Zmień nazwę { $name }
sources-in-group = W grupie { $name }

# Row status + retry
sources-retry-error = Ponów — { $message }
sources-retry-item = Ponów { $name }
sources-status-error = status: błąd
sources-open-privacy-title = Otwórz ustawienia prywatności macOS dla tego uprawnienia
sources-open-privacy-item = Otwórz ustawienia prywatności dla { $name }
sources-privacy-settings-button = ustawienia
sources-status-starting = uruchamianie…
sources-status-live = na żywo
sources-status-aria = status: { $state }

# Media row pause/resume
sources-media-resume-title = Wznów wideo (na żywo w transmisji)
sources-media-pause-title = Wstrzymaj wideo — zatrzymaj klatkę i wycisz, na żywo w transmisji
sources-media-resume-item = Wznów { $name }
sources-media-pause-item = Wstrzymaj { $name }

# Hover controls
sources-unlock = Odblokuj
sources-lock = Zablokuj
sources-unlock-item = Odblokuj { $name }
sources-lock-item = Zablokuj { $name }
sources-raise-title = Podnieś w stosie
sources-raise-item = Podnieś { $name }
sources-lower-title = Obniż w stosie
sources-lower-item = Obniż { $name }
sources-filters-title = Filtry i mieszanie
sources-filters-item = Filtry dla { $name }
sources-properties-title = Właściwości
sources-properties-item = Właściwości { $name }
sources-remove-title = Usuń z tej sceny
sources-remove-item = Usuń { $name }

# Grouping footer
sources-create-group = Utwórz grupę ({ $count })
sources-cancel = Anuluj

# Groups list
sources-groups-aria = Grupy źródeł
sources-hide-group = Ukryj grupę
sources-show-group = Pokaż grupę
sources-item-count = · { $count } elementów
sources-ungroup-title = Rozgrupuj — elementy pozostają na swoich miejscach
sources-ungroup-item = Rozgrupuj { $name }

# Live Chat Overlay picker
sources-chat-title = Dodaj nakładkę czatu na żywo
sources-chat-youtube-label = YouTube — adres URL kanału, watch lub live_chat (bez klucza, bez logowania)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  lub adres watch?v=
sources-chat-twitch-label = Twitch — nazwa kanału (odczyt anonimowy, bez konta)
sources-chat-twitch-placeholder = twojkanal
sources-chat-kick-label = Kick — slug kanału (publiczny punkt końcowy, w miarę możliwości)
sources-chat-kick-placeholder = twojkanal
sources-chat-note = Wiadomości pojawiają się z bieżącym znacznikiem czasu g:mm:ss AM/PM na przezroczystym tle (domyślnie w prawym górnym rogu; przeciągnij w dowolne miejsce). Zalew wiadomości powoduje jedynie usuwanie starych linii — nigdy nie może zatrzymać transmisji ani nagrywania. Czat Facebooka wymaga własnego tokenu Graph i nie jest jeszcze zaimplementowany — nigdy nie jest wymagany i nigdy nie blokuje powyższych platform.
sources-chat-add = Dodaj nakładkę czatu
sources-chat-default-name = Czat na żywo

# Image Slideshow picker
sources-slideshow-title = Dodaj pokaz slajdów
sources-slideshow-empty = Brak obrazów — Przeglądaj dodaje je w kolejności.
sources-slideshow-remove-slide = Usuń slajd { $number }
sources-slideshow-browse = Przeglądaj obrazy…
sources-slideshow-per-slide-label = Na slajd (ms)
sources-slideshow-crossfade-label = Przenikanie (ms, 0 = cięcie)
sources-slideshow-loop-label = Zapętl (wył. = zatrzymaj ostatni slajd)
sources-slideshow-shuffle-label = Losuj w każdym cyklu
sources-slideshow-note = Przenikanie łączy obrazy o jednakowym rozmiarze; obrazy o różnych rozmiarach są twardo cięte na granicy (bez cichego skalowania).
sources-slideshow-add = Dodaj pokaz slajdów ({ $count })

# Nested Scene picker
sources-nested-title = Dodaj scenę zagnieżdżoną
sources-nested-empty = Brak innej sceny do zagnieżdżenia — najpierw dodaj drugą scenę.
sources-nested-scene-name = Scena: { $name }
sources-nested-note = Scena zagnieżdżona renderuje się na żywo w rozmiarze płótna programu i podąża za własnymi edycjami; transformacje, filtry i mieszanie stosują się do niej jak do każdego źródła. Jej źródła audio dołączają do miksu, gdy scena ją wyświetlająca jest programem.

# Display / Window capture picker
sources-capture-display-title = Dodaj przechwytywanie ekranu
sources-capture-window-title = Dodaj przechwytywanie okna
sources-capture-looking = Szukanie źródeł…
sources-capture-none-displays = Nie ma tu nic do przechwycenia — nie znaleziono ekranów.
sources-capture-none-windows = Nie ma tu nic do przechwycenia — nie znaleziono okien.
sources-capture-portal-note = W środowisku Wayland to okno systemowe wybiera ekran lub okno — aplikacje nie mogą tam przechwytywać globalnie, więc to jest uczciwa (i jedyna) droga.
sources-capture-window-note = Podglądy aktualizują się na żywo. Zminimalizowane okno pokazuje swoją ostatnią klatkę (lub żadnej), dopóki go nie przywrócisz.
sources-thumb-no-preview = brak podglądu
sources-thumb-loading = ładowanie…

# Video Capture Device picker
sources-webcam-title = Dodaj urządzenie przechwytujące obraz
sources-webcam-looking = Szukanie kamer…
sources-webcam-none = Nie znaleziono kamer ani kart przechwytujących.
sources-webcam-format-label = Format
sources-webcam-format-auto-loading = Auto (ładowanie formatów…)
sources-webcam-format-auto = Auto (najwyższa rozdzielczość)
sources-webcam-card-presets-label = Presety karty:
sources-webcam-preset-title = Wybierz tryb { $label } oferowany przez tę kartę
sources-webcam-add = Dodaj kamerę

# Audio Input / Output capture picker
sources-audio-output-title = Dodaj przechwytywanie wyjścia dźwięku
sources-audio-input-title = Dodaj przechwytywanie wejścia dźwięku
sources-audio-default-output = Domyślne wyjście (to, co słyszysz)
sources-audio-default-input = Domyślne wejście
sources-audio-looking = Szukanie urządzeń audio…
sources-audio-none-output = Nie znaleziono tu urządzenia przechwytującego dźwięk pulpitu.
sources-audio-none-input = Nie znaleziono mikrofonów ani wejść liniowych.
sources-audio-input-note = Kanały miksera otrzymują wskaźnik VU, suwak głośności, wyciszenie, monitorowanie, filtry (redukcja szumów, bramka, kompresor…) oraz przypisanie ścieżki. Wszystko pozostaje na tym komputerze.

# Application Audio picker
sources-appaudio-title = Dodaj dźwięk aplikacji
sources-appaudio-looking = Szukanie aplikacji odtwarzających dźwięk…
sources-appaudio-none = Żadna aplikacja nie odtwarza teraz dźwięku — rozpocznij odtwarzanie w aplikacji, a następnie odśwież.
sources-appaudio-refresh = ⟳ Odśwież
sources-appaudio-note = Przechwytuje dokładnie dźwięk tej aplikacji — z własnym VU, suwakiem głośności, wyciszeniem, filtrami i ścieżką.

# Game Capture picker
sources-game-title = Przechwytywanie gry
sources-game-checking = Sprawdzanie…
sources-game-use-portal = Użyj przechwytywania ekranu (Portal)
sources-game-use-window = Użyj zamiast tego przechwytywania okna

# Image picker
sources-image-title = Dodaj obraz
sources-image-file-label = Plik obrazu (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Dodaj obraz

# Path field
sources-browse = Przeglądaj…

# Media picker
sources-media-title = Dodaj media
sources-media-file-label = Plik multimedialny (mp4, mkv, webm, mov, .frec lub obraz)
sources-media-loop-label = Zapętl (po zakończeniu zacznij od początku)
sources-media-note = .frec odtwarza się przez własny kodek freally-video — nic do pobrania. Formaty transmisyjne (mp4/mkv/webm/…) dekodują się przez pobierany na żądanie komponent FFmpeg; ich dźwięk trafia do miksera jako osobny kanał.
sources-media-add = Dodaj media

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 godzina
sources-ttl-1day = 1 dzień

# Remote Guest form
sources-remote-copy-failed = nie udało się skopiować — zaznacz link i skopiuj ręcznie
sources-remote-join-failed = dołączenie nie powiodło się: { $error }
sources-remote-title = Zdalny gość (próba P2P)
sources-remote-host-heading = Host — zaproś gościa
sources-remote-start-hosting = Rozpocznij hostowanie
sources-remote-expires-label = Wygasa
sources-remote-invite-expiry-aria = Wygaśnięcie zaproszenia
sources-remote-invite-link-aria = Link z zaproszeniem
sources-remote-copied = Skopiowano ✓
sources-remote-copy = Kopiuj
sources-remote-share-note = Udostępnij ten link (Discord / SMS / e-mail). Zawiera Twoją sesję i wygasa zgodnie z ustawieniem. Gość otwiera go i dołącza ze swoją kamerą.
sources-remote-qr-note = Zeskanuj telefonem, aby dołączyć bezpośrednio z przeglądarki — kamera + mikrofon, bez instalacji. Kopiowalny link freally:// powyżej otwiera się w Freally Capture na komputerze, który go ma.
sources-remote-guest-heading = Gość — dołącz przez zaproszenie
sources-remote-paste-placeholder = wklej link z zaproszeniem
sources-remote-invite-input-aria = Link z zaproszeniem lub identyfikator sesji
sources-remote-join = Dołącz z kamerą
sources-remote-session-note = Sterowanie sesją na żywo (wyciszenie, zakończenie) pozostaje na pasku u góry głównego okna — możesz zamknąć to okno.
sources-remote-stop-session = Zatrzymaj sesję

# Invite QR
sources-invite-qr-aria = Kod QR linku z zaproszeniem

# Remote device pickers
sources-devices-output-unavailable = przekierowanie wyjścia niedostępne — odtwarzanie na urządzeniu domyślnym
sources-devices-mic-test-failed = test mikrofonu nie powiódł się: { $error }
sources-devices-heading = Urządzenia audio sesji
sources-devices-microphone-label = Mikrofon
sources-devices-microphone-aria = Mikrofon sesji
sources-devices-system-default = Domyślne systemu
sources-devices-output-label = Wyjście
sources-devices-output-aria = Wyjście audio sesji
sources-devices-stop-test = Zatrzymaj test
sources-devices-test = Test — usłysz siebie
sources-devices-testing-note = mów do mikrofonu — słyszysz wybrane urządzenia na żywo
sources-devices-idle-note = zapętla mikrofon na wyjście (słuchawki zapobiegają sprzężeniu)

# TURN relay section
sources-turn-save-failed = nie udało się zapisać: { $error }
sources-turn-summary = Sieć — opcjonalny przekaźnik TURN (zaawansowane)
sources-turn-note-1 = Sesje łączą się bezpośrednio (P2P) — za darmo, bez przekaźnika. Jeśli OBIE strony znajdują się za restrykcyjnymi NAT-ami, ścieżka bezpośrednia może zawieść; wtedy media przenosi przekaźnik TURN, który sam uruchamiasz. Pominięcie tego jest w porządku — większość połączeń działa tylko bezpośrednio.
sources-turn-note-2 = Darmowa opcja: Oracle Cloud „Always Free” uruchamia coturn bez opłat (uwaga: Oracle prosi o kartę kredytową przy rejestracji, ale kształt Always-Free pozostaje darmowy). Kroki: 1) utwórz darmową maszynę wirtualną, 2) zainstaluj coturn, 3) otwórz UDP 3478, 4) ustaw użytkownika/hasło, 5) wpisz tutaj turn:ip-twojej-vm:3478 + dane logowania. Twoje dane logowania pozostają w lokalnym pliku ustawień i nigdy nie są rejestrowane.
sources-turn-url-label = Adres URL TURN
sources-turn-url-placeholder = turn:host:3478 (puste = tylko bezpośrednio)
sources-turn-url-aria = Adres URL TURN
sources-turn-username-label = Nazwa użytkownika
sources-turn-username-aria = Nazwa użytkownika TURN
sources-turn-credential-label = Dane logowania
sources-turn-credential-aria = Dane logowania TURN
sources-turn-note-3 = Przekaźnik uruchamia się po ustawieniu wszystkich trzech pól (serwer TURN wymaga danych logowania) i stosuje się do następnej sesji, którą rozpoczniesz lub do której dołączysz. Zweryfikuj go połączeniem testowym tylko przez przekaźnik między dwoma własnymi komputerami.
sources-turn-settings-unavailable = ustawienia niedostępne (tryb przeglądarki)

# Color picker
sources-color-title = Dodaj kolor
sources-color-label = Kolor
sources-color-width-label = Szerokość
sources-color-height-label = Wysokość
sources-color-add = Dodaj kolor

# Text picker
sources-text-title = Dodaj tekst
sources-text-label = Tekst
sources-text-default = Tekst
sources-text-color-label = Kolor
sources-text-color-aria = Kolor tekstu
sources-text-size-label = Rozmiar (px)
sources-text-note = Rodzina czcionek, wyrównanie, zawijanie i RTL znajdują się we Właściwościach źródła. Dołączona czcionka Noto Sans (w tym arabska/hebrajska) jest domyślna — identyczna na każdym komputerze.
sources-text-add = Dodaj tekst

# Existing source picker
sources-existing-title = Dodaj istniejące źródło
sources-existing-empty = Nie ma jeszcze żadnych źródeł — najpierw dodaj jedno do dowolnej sceny. Istniejące źródła są współdzielone: zmiana nazwy lub rekonfiguracja jednego aktualizuje każdą scenę, która je wyświetla.

# Screen + corners layout
sources-slot-off = Wył.
sources-slot-center = Środek (ekran)
sources-slot-top-left = Lewy górny
sources-slot-top-right = Prawy górny
sources-slot-bottom-left = Lewy dolny
sources-slot-bottom-right = Prawy dolny
sources-layout-title = Rozmieść: ekran + rogi
sources-layout-empty = Najpierw dodaj do tej sceny przechwytywanie ekranu oraz jedną lub więcej kamer, a następnie rozmieść je tutaj.
sources-layout-note = Umieść ekran na środku i maksymalnie cztery kamery w rogach — Twój układ do objaśnień / podcastu. Każdy róg mieści kamerę, przechwycone okno rozmowy lub klip multimedialny. Każdą z nich możesz potem przeciągnąć na płótnie.
sources-layout-slot-aria = Miejsce dla { $name }
sources-layout-apply = Zastosuj układ


# =============================================================
# --- docks ---
# =============================================================

# --- ControlsDock.tsx ---
controls-title = Sterowanie
controls-start-stop-title-stop = Zatrzymaj i sfinalizuj nagrywanie
controls-start-stop-title-start = Nagrywaj sygnał programu zgodnie z konfiguracją Ustawienia → Wyjście
controls-finalizing = ◌ Finalizowanie…
controls-stop-recording = ■ Zatrzymaj nagrywanie
controls-start-recording = ● Rozpocznij nagrywanie
controls-marker-title = Umieść znacznik rozdziału w tej chwili — trafia do NAGRANIA (rozdziały mkv lub plik towarzyszący). Znaczniki transmisji po stronie platformy wymagają kont platform, o które ta aplikacja nigdy nie prosi.
controls-marker = ◈ Znacznik
controls-pause-title-resume = Wznów — plik jest kontynuowany jako jedna ciągła oś czasu
controls-pause-title-pause = Wstrzymaj — żadne klatki nie są zapisywane; wznowienie kontynuuje ten sam odtwarzalny plik
controls-resume-recording = ▶ Wznów nagrywanie
controls-pause-recording = ⏸ Wstrzymaj nagrywanie
controls-reactions-label = Reakcje (wtopione w program)
controls-reactions-title = Wyświetl reakcję nad programem — nagrywaną ORAZ transmitowaną, aby powtórka pokazała dokładny moment. Widzowie na czacie też je wyzwalają (ich emoji reakcji unosi się automatycznie); zalew jedynie ogranicza to, co jest na ekranie.
controls-react = Reaguj { $emoji }
controls-virtual-camera-title = Kamera wirtualna wymaga własnego, podpisanego sterownika na każdy system operacyjny (Win11 MFCreateVirtualCamera / Win10 DirectShow / rozszerzenie CoreMediaIO macOS / Linux v4l2loopback) — jest dostarczana jako osobny etap. Model sygnału jest na nią gotowy: program, płótno pionowe lub pojedyncze źródło, ze sparowanym mikrofonem wirtualnym w Windows/Linux (macOS nie ma API mikrofonu wirtualnego — mówiąc szczerze).
controls-virtual-camera = ⌁ Uruchom kamerę wirtualną
controls-files-title = Ukończone nagrania + akcja remuksowania do mp4
controls-files = ▤ Pliki…
controls-output-title = Format nagrywania, enkoder, folder, ścieżki i dzielenie
controls-output = ⚙ Wyjście…
controls-stream-title = Cel transmisji na żywo: usługa, klucz transmisji, enkoder, bitrate
controls-stream = ⦿ Transmisja…
controls-codecs-title = Pobierany na żądanie komponent kodeków transmisyjnych ffmpeg (wyraźnie oznaczony, nigdy nie dołączany)
controls-codecs = ⬡ Kodeki…
controls-replay-title = Długość bufora powtórek + presety jakości
controls-replay = ⟲ Powtórka…
controls-keys-title = Globalne skróty klawiszowe: nagrywanie, transmisja na żywo, przejście, zapis powtórki
controls-keys = ⌨ Klawisze…
controls-scripts-title = Skrypty Lua w piaskownicy: reagują na zdarzenia transmisji/sceny/nagrywania, sterują studiem
controls-scripts = ⚡ Skrypty…
controls-docks-title = Doki przeglądarki: otwórz wyskakujący czat, stronę alertów lub przyciski Companion jako okno obok studia
controls-docks = ⧉ Doki…
controls-remote-title = Zdalne API WebSocket dla kontrolerów Stream Deck / Companion (domyślnie wyłączone)
controls-remote = ⌁ Zdalne…
controls-profiles-title = Profile (ustawienia) + kolekcje scen — przełączalne migawki
controls-profiles = ▣ Profile…
controls-bug-title = Zgłoś błąd — anonimowo, dobrowolnie (nic nie jest wysyłane automatycznie)
controls-bug = 🐞 Zgłoś błąd…
controls-updates-title = Sprawdź aktualizacje — podpisane, zweryfikowane, nic nie pobiera się bez kliknięcia
controls-updates = ⭳ Sprawdź aktualizacje…
controls-saved = Zapisano: { $path }

# --- MixerDock.tsx ---
mixer-title = Mikser dźwięku
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Przełącz na kanały poziome
mixer-switch-to-vertical = Przełącz na kanały pionowe
mixer-layout-aria-vertical = Układ miksera: pionowy — przełącz na poziomy
mixer-layout-aria-horizontal = Układ miksera: poziomy — przełącz na pionowy
mixer-empty = Brak źródeł audio w tej scenie — dodaj Przechwytywanie wejścia dźwięku (mikrofon) lub Przechwytywanie wyjścia dźwięku (dźwięk pulpitu) za pomocą „+” w Źródłach. Kanały otrzymują wskaźnik VU, suwak głośności, wyciszenie, monitorowanie, filtry oraz przypisanie ścieżki.
mixer-advanced-title = Dźwięk — { $name }
mixer-loudness-label = Głośność programu (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Głośność chwilowa (400 ms)
mixer-short-term-title = Głośność krótkoterminowa (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Urządzenie wyjściowe monitora
mixer-default-output = Domyślne wyjście

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Pamięć
stats-dropped = Utracone
stats-render = Renderowanie
stats-gpu = GPU
stats-gpu-compositing = kompozycja
stats-gpu-idle = bezczynny
stats-vertical-fps = 9:16 FPS
stats-targets-label = Cele transmisji
stats-shared-encode = · wspólne kodowanie
stats-starting = Uruchamianie kompozytora…

# --- ScenesRail.tsx ---
scenes-title = Sceny
scenes-new-scene-name = Scena
scenes-add = Dodaj scenę
scenes-empty = Łączenie z rdzeniem studia…
scenes-rename = Zmień nazwę { $name }
scenes-on-program = Na programie
scenes-preview = Podgląd { $name }
scenes-switch-to = Przełącz na { $name }
scenes-move-up = Przenieś w górę
scenes-move-up-aria = Przenieś { $name } w górę
scenes-move-down = Przenieś w dół
scenes-move-down-aria = Przenieś { $name } w dół
scenes-last-stays = Ostatnia scena pozostaje
scenes-remove = Usuń tę scenę
scenes-remove-aria = Usuń { $name }


# =============================================================
# --- components ---
# =============================================================

# --- ChannelStrip.tsx ---
channelstrip-level = Poziom
channelstrip-monitor-off = Monitor wył.
channelstrip-monitor-only = Tylko monitor (poza miksem)
channelstrip-monitor-and-output = Monitor i wyjście
channelstrip-status-error = błąd
channelstrip-status-live = na żywo
channelstrip-status-waiting-audio = oczekiwanie na dźwięk
channelstrip-status = status: { $state }
channelstrip-status-waiting = oczekiwanie
channelstrip-mute = Wycisz
channelstrip-unmute = Wyłącz wyciszenie
channelstrip-mute-source = Wycisz { $name }
channelstrip-unmute-source = Wyłącz wyciszenie { $name }
channelstrip-scene-mix-on = Miks na scenę WŁ. — ten kanał zastępuje globalny miks dla tej sceny (kliknij, aby ponownie podążać za globalnym miksem)
channelstrip-scene-mix-off = Miks na scenę — nadaj temu kanałowi własny suwak/wyciszenie dla bieżącej sceny
channelstrip-scene-mix-label = Miks na scenę dla { $name }
channelstrip-monitor-cycle = { $mode } — kliknij, aby przełączyć
channelstrip-monitor-mode = Tryb monitora { $name }: { $mode }
channelstrip-audio-filters-title = Filtry audio (redukcja szumów, bramka, kompresor…)
channelstrip-audio-filters-label = Filtry audio dla { $name }
channelstrip-advanced-title = Przesunięcie synchronizacji i skróty naciśnij aby mówić
channelstrip-advanced-label = Zaawansowane ustawienia audio dla { $name }
channelstrip-track-assignment = Przypisanie ścieżki
channelstrip-track = Ścieżka { $n }
channelstrip-track-assigned = Ścieżka { $n } (przypisana)
channelstrip-track-label = Ścieżka { $n } dla { $name }
channelstrip-device-error = błąd urządzenia
channelstrip-audio-device-error = błąd urządzenia audio
channelstrip-volume-label = Głośność { $name } w decybelach
channelstrip-ptt-hold = Naciśnij aby mówić: przytrzymaj { $key }
channelstrip-sync-offset = Przesunięcie synchronizacji (ms, 0–{ $max } — opóźnia ten dźwięk)
channelstrip-ptt-hotkey = Skrót naciśnij aby mówić (cisza, gdy nieprzytrzymany)
channelstrip-ptt-placeholder = np. Ctrl+Shift+T lub F13
channelstrip-ptt-aria = Skrót naciśnij aby mówić
channelstrip-ptm-hotkey = Skrót naciśnij aby wyciszyć (cisza podczas przytrzymania)
channelstrip-ptm-placeholder = np. Ctrl+Shift+M
channelstrip-ptm-aria = Skrót naciśnij aby wyciszyć
channelstrip-hotkeys-note = Skróty działają, gdy inne aplikacje są aktywne. W Linux/Wayland globalne skróty mogą być niedostępne — to ograniczenie kompozytora, mówiąc szczerze.
channelstrip-apply = Zastosuj


# --- LiveButton.tsx ---
livebutton-failure-ended = transmisja zakończona
livebutton-title-live = Zakończ transmisję — każdy cel (trwające nagrywanie jest kontynuowane)
livebutton-title-offline = Wejdź na żywo do każdego włączonego celu Ustawienia → Transmisja
livebutton-end-stream = ■ Zakończ transmisję
livebutton-aria-reconnecting = Ponowne łączenie
livebutton-aria-live = Na żywo
livebutton-badge-retry = ponów { $n }
livebutton-badge-live = na żywo
livebutton-go-live = ⦿ Wejdź na żywo


# --- RecDot.tsx ---
recdot-paused-aria = Nagrywanie wstrzymane
recdot-recording-aria = Nagrywanie
recdot-tracks-one = { $count } nagrywana ścieżka audio
recdot-tracks-other = { $count } nagrywanych ścieżek audio
recdot-paused = wstrzymane


# --- ReplayControls.tsx ---
replaycontrols-saved = Powtórka zapisana — { $name }
replaycontrols-failure-stopped = bufor zatrzymany
replaycontrols-title-disarm = Rozbrój bufor powtórek (odrzuca niezapisaną historię)
replaycontrols-title-arm = Uzbrój przewijany bufor powtórek — utrzymuje ostatnie N sekund gotowe do zapisu (własne, lekkie kodowanie; transmisja i nagrywanie pozostają nietknięte)
replaycontrols-replay-seconds = ⟲ Powtórka { $seconds }s
replaycontrols-arm = ⟲ Uzbrój bufor powtórek
replaycontrols-save-title = Zapisz ostatnie N sekund do folderu nagrań (także pod skrótem Zapisz powtórkę)
replaycontrols-save = ⤓ Zapisz


# --- PropertiesDialog.tsx ---
properties-title = Właściwości — { $name }
properties-name = Nazwa
properties-cancel = Anuluj
properties-apply = Zastosuj
properties-youtube = YouTube — adres URL kanału / watch / live_chat (nigdy bez klucza ani logowania)
properties-twitch = Twitch — nazwa kanału (anonimowo)
properties-kick = Kick — slug kanału (publiczny punkt końcowy)
properties-width-px = Szerokość (px)
properties-lines = Wiersze
properties-font-px = Czcionka (px)
properties-images = Pliki obrazów (jedna ścieżka w wierszu, wyświetlane w kolejności)
properties-per-slide = Na slajd (ms)
properties-crossfade = Przenikanie (ms, 0 = cięcie)
properties-loop-slideshow = Zapętl (wył. = zatrzymaj ostatni slajd)
properties-shuffle = Losuj w każdym cyklu
properties-nested-scene = Scena komponowana przez to źródło (scena już zawierająca tę jest odrzucana)
properties-portal-note = Portal ScreenCast Wayland wybiera ekran lub okno w oknie systemowym przy każdym uruchomieniu tego źródła — z założenia nie ma tu nic do skonfigurowania.
properties-appaudio-capturing = Przechwytywanie dźwięku z { $exe }
properties-appaudio-exe-fallback = aplikacja
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Dodaj źródło ponownie, aby wskazać inną aplikację (identyfikator procesu zmienia się po ponownym uruchomieniu aplikacji).
properties-image-file = Plik obrazu
properties-media-file = Plik multimedialny (mp4, mkv, webm, mov, .frec lub obraz)
properties-media-loop = Zapętl (po zakończeniu zacznij od początku)
properties-media-hwdecode = Dekodowanie sprzętowe (samoczynnie przechodzi na programowe)
properties-media-note = .frec odtwarza się przez własny kodek freally-video — nic do pobrania. Inne formaty wideo dekodują się przez pobierany na żądanie komponent FFmpeg. Dźwięk pliku otrzymuje własny kanał miksera; przesunięcie synchronizacji kanału precyzyjnie dostraja synchronizację A/V. Klip bez dźwięku pozostawia swój kanał cichym.
properties-color = Kolor
properties-width = Szerokość
properties-height = Wysokość
properties-text = Tekst
properties-font-family = Rodzina czcionek (systemowa; puste = domyślna)
properties-size-px = Rozmiar (px)
properties-text-color = Kolor tekstu
properties-align = Wyrównanie
properties-align-left = do lewej
properties-align-center = do środka
properties-align-right = do prawej
properties-line-spacing = Odstęp między wierszami
properties-wrap-width = Szerokość zawijania (px; 0 = wył.)
properties-force-rtl = Wymuś od prawej do lewej
properties-text-note = Renderowanie używa rzeczywistego kształtowania (łączenie arabskie, ligatury) i dwukierunkowego porządku wierszy. Dołączona rodzina Noto Sans (w tym arabska/hebrajska) jest domyślna; systemowe rodziny też działają. CJK na razie używa czcionek systemowych.
properties-repick-capturing = Przechwytywanie: { $label }
properties-repick-looking = Szukanie źródeł…
properties-repick-none-displays = Nie znaleziono ekranów do ponownego wyboru.
properties-repick-none-windows = Nie znaleziono okien do ponownego wyboru.
properties-repick-again = Wybierz ponownie:
properties-device = Urządzenie
properties-video-current-device = (bieżące urządzenie)
properties-format = Format
properties-format-auto-loading = Auto (ładowanie formatów…)
properties-format-auto = Auto (najwyższa rozdzielczość)
properties-audio-capture-of = Przechwytuj dźwięk z
properties-audio-default-output = Domyślne wyjście (to, co słyszysz)
properties-audio-default-input = Domyślne wejście
properties-audio-default-suffix = (domyślne)
properties-audio-current-device = (bieżące urządzenie: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Wzmocnienie
audiofilters-name-noise-gate = Bramka szumów
audiofilters-name-compressor = Kompresor
audiofilters-name-limiter = Limiter
audiofilters-name-eq = Korektor 3-pasmowy
audiofilters-name-denoise = Redukcja szumów
audiofilters-name-ducking = Wyciszanie (ducking)
audiofilters-title = Filtry audio — { $name }
audiofilters-chain-header = Łańcuch filtrów (górny działa pierwszy, przed suwakiem)
audiofilters-add = + Dodaj filtr
audiofilters-add-menu = Dodaj filtr audio
audiofilters-empty = Brak filtrów — zredukuj szumy mikrofonu (klasyczny DSP, bez ML), zamknij pomieszczenie bramką, ujarzmij piki kompresorem lub wycisz muzykę pod swoim głosem.
audiofilters-enable = Włącz { $name }
audiofilters-run-earlier = Uruchom wcześniej
audiofilters-move-up = Przenieś { $name } w górę
audiofilters-run-later = Uruchom później
audiofilters-move-down = Przenieś { $name } w dół
audiofilters-remove-title = Usuń filtr
audiofilters-remove = Usuń { $name }
audiofilters-gain-db = Wzmocnienie (dB)
audiofilters-open-db = Otwórz przy (dB)
audiofilters-close-db = Zamknij przy (dB)
audiofilters-attack-ms = Atak (ms)
audiofilters-hold-ms = Przytrzymanie (ms)
audiofilters-release-ms = Zwolnienie (ms)
audiofilters-ratio = Współczynnik (:1)
audiofilters-threshold-db = Próg (dB)
audiofilters-output-gain-db = Wzmocnienie wyjścia (dB)
audiofilters-ceiling-db = Sufit (dB)
audiofilters-low-db = Niskie (dB)
audiofilters-mid-db = Średnie (dB)
audiofilters-high-db = Wysokie (dB)
audiofilters-strength = Siła
audiofilters-denoise-note = Własne, klasyczne DSP z tłumieniem widmowym — stały szum (wentylatory, syk) spada, a mowa przechodzi. Bez ML, bez modeli, zgodnie z kartą.
audiofilters-duck-under = Wycisz pod
audiofilters-ducking-trigger = Źródło wyzwalające wyciszanie
audiofilters-pick-trigger = (wybierz wyzwalacz — np. swój mikrofon)
audiofilters-trigger-at-db = Wyzwól przy (dB)
audiofilters-duck-by-db = Wycisz o (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Klucz koloru
filters-name-luma-key = Luma Key
filters-name-render-delay = Opóźnienie renderowania
filters-name-color-correction = Korekcja koloru
filters-name-lut = Zastosuj LUT
filters-name-blur = Rozmycie
filters-name-mask = Maska obrazu
filters-name-sharpen = Wyostrzanie
filters-name-scroll = Przewijanie
filters-name-crop = Kadrowanie
filters-title = Filtry — { $name }
filters-blend-mode = Tryb mieszania
filters-chain-header = Łańcuch filtrów (górny działa pierwszy)
filters-add = + Dodaj filtr
filters-add-menu = Dodaj filtr
filters-empty = Brak filtrów — nałóż chroma key na kamerę, popraw kolory przechwytywania lub przewijaj pasek informacyjny.
filters-enable = Włącz { $name }
filters-run-earlier = Uruchom wcześniej
filters-move-up = Przenieś { $name } w górę
filters-run-later = Uruchom później
filters-move-down = Przenieś { $name } w dół
filters-remove-title = Usuń filtr
filters-remove = Usuń { $name }
filters-key-color-rgb = Kolor kluczowy (dowolny kolor, odległość RGB)
filters-similarity = Podobieństwo
filters-smoothness = Gładkość
filters-luma-min = Min luminancji (ciemniejsze usuwane)
filters-luma-max = Maks luminancji (jaśniejsze usuwane)
filters-delay = Opóźnienie (ms — tylko wideo, np. do synchronizacji z dźwiękiem; maks. 500)
filters-key-color = Kolor kluczowy
filters-spill = Poświata
filters-gamma = Gamma
filters-brightness = Jasność
filters-contrast = Kontrast
filters-saturation = Nasycenie
filters-hue-shift = Przesunięcie barwy
filters-opacity = Krycie
filters-cube-file = Plik .cube
filters-amount = Ilość
filters-radius = Promień
filters-mask-image = Obraz maski
filters-mask-mode = Tryb
filters-mask-alpha = alfa
filters-mask-luma = luminancja
filters-mask-invert = odwróć
filters-speed-x = Prędkość X (px/s)
filters-speed-y = Prędkość Y (px/s)
filters-crop-left = lewa
filters-crop-top = góra
filters-crop-right = prawa
filters-crop-bottom = dół
filters-crop-aria = przytnij { $side }


# --- PickerShell.tsx ---
pickershell-refresh-aria = Odśwież
pickershell-refresh-title = Odśwież listę
pickershell-close = Zamknij


# =============================================================
# --- dialogs ---
# =============================================================

# --- BugReport.tsx ---
bugreport-title = Zgłoś błąd
bugreport-intro = Zgłoszenia są anonimowe i dobrowolne — nic nie jest wysyłane automatycznie. Sprawdzisz dokładny tekst poniżej, a następnie prześlesz go przez wstępnie wypełnione zgłoszenie na GitHubie lub swoją aplikację e-mail. Żadnych danych osobowych (Twoja ścieżka domowa i nazwa użytkownika są ukryte); żadnego konta, żadnego serwera.
bugreport-crash-notice = Freally Capture nieoczekiwanie zamknął się podczas poprzedniego uruchomienia — anonimowe szczegóły awarii znajdują się poniżej. Ich zgłoszenie pomaga szybko to naprawić.
bugreport-description-label = Co robiłeś, gdy to się stało? (opcjonalnie)
bugreport-description-placeholder = np. podgląd zamarł, gdy dodałem drugą kamerę
bugreport-include-crash = Dołącz anonimowe szczegóły awarii z ostatniego uruchomienia
bugreport-preview-label = Dokładnie to, co zostanie wysłane
bugreport-open-github = Otwórz zgłoszenie na GitHubie
bugreport-gmail-title = Otwiera okno tworzenia wiadomości Gmail w przeglądarce, wstępnie wypełnione. Wylogowany? Google najpierw pokaże ekran logowania.
bugreport-compose-gmail = Utwórz w Gmailu
bugreport-email-title = Otwiera roboczą wiadomość w domyślnej aplikacji pocztowej tego komputera (Outlook, Thunderbird, Mail…)
bugreport-send-email = Wyślij e-mail
bugreport-copied = Skopiowano ✓
bugreport-copy-report = Kopiuj zgłoszenie
bugreport-dismiss-crash = Odrzuć awarię
bugreport-copy-failed = nie udało się skopiować — zaznacz tekst i skopiuj ręcznie
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = CO SIĘ STAŁO
bugreport-preview-no-description = (nie podano opisu)
bugreport-preview-diagnostics = ANONIMOWA DIAGNOSTYKA (brak danych osobowych)
bugreport-preview-from = Od: Freally Capture
bugreport-preview-crash-excerpt = --- fragment awarii ---


# --- Updates.tsx ---
updates-title = Aktualizacja oprogramowania
updates-checking = Sprawdzanie aktualizacji…
updates-uptodate = Masz najnowszą wersję.
updates-check-again = Sprawdź ponownie
updates-available = Dostępna jest wersja { $version }
updates-current-version = (masz { $current })
updates-release-notes-label = Wersja { $version } — Informacje o wydaniu
updates-confirm = Czy chcesz teraz zaktualizować? Pobrany plik jest weryfikowany dołączonym kluczem podpisującym przed zastosowaniem. Freally Capture zamyka się, uruchamia się instalator, a nowa wersja otwiera się samoczynnie.
updates-yes-update-now = Tak, zaktualizuj teraz
updates-no-not-now = Nie, nie teraz
updates-downloading = Pobieranie { $version }…
updates-starting = uruchamianie…
updates-installed = Aktualizacja zainstalowana.
updates-restart-now = Uruchom ponownie teraz
updates-restart-later = Uruchom ponownie później
updates-try-again = Spróbuj ponownie


# --- Models.tsx ---
models-title = Komponenty
models-ffmpeg-heading = FFmpeg — kodeki transmisyjne
models-badge-third-party = Zewnętrzny · niedołączany
models-ffmpeg-desc = Własny silnik Freally Capture nagrywa bezstratny format freally-video (.frec) bez niczego dodatkowego. Nagrywanie formatów transmisyjnych oczekiwanych przez platformy i odtwarzacze — H.264/AAC (oraz HEVC/AV1) w mp4/mkv/mov/webm — korzysta z FFmpeg, osobnego narzędzia, z którym ta aplikacja nigdy nie jest dostarczana: te kodeki są obciążone patentami, więc pozostaje opcjonalny i wyraźnie oznaczony. Jest pobierany na żądanie z przypiętej wersji poniżej, weryfikowany sumą SHA-256 przed pierwszym użyciem, buforowany dla każdego użytkownika i sterowany jako osobny proces. Jego licencja (LGPL/GPL) jest niezależna — zobacz THIRD-PARTY-NOTICES.
models-checking = Sprawdzanie…
models-ffmpeg-not-installed = Niezainstalowany. Dostępny: FFmpeg { $version } z { $source } (pobieranie { $size }).
models-ffmpeg-none-pinned = Żadna wersja FFmpeg nie jest jeszcze przypięta dla tej platformy — nagrywanie w kodekach transmisyjnych jest tu niedostępne. Bezstratne nagrywanie freally-video nie jest tym objęte.
models-ffmpeg-download-verify = Pobierz i zweryfikuj ({ $size })
models-downloading = Pobieranie…
models-download-of = z
models-cancel = Anuluj
models-ffmpeg-verifying = Weryfikowanie pobranego pliku względem przypiętej sumy SHA-256…
models-ffmpeg-extracting = Rozpakowywanie…
models-ffmpeg-ready = Zainstalowano i zweryfikowano — { $version }
models-remove = Usuń
models-ffmpeg-retry = Ponów pobieranie
models-network-note = Pobieranie to jedyna akcja sieciowa na tym panelu i nigdy nie rozpoczyna się samoczynnie. Nieudana suma kontrolna przerywa instalację — aplikacja odmawia uruchomienia bajtów, za które nie może ręczyć.
models-cef-heading = Środowisko uruchomieniowe źródła przeglądarki — Chromium (CEF)
models-cef-desc = Źródła przeglądarki renderują strony internetowe (alerty, widżety, nakładki) przez Chromium Embedded Framework — środowisko uruchomieniowe o rozmiarze ~100 MB, z którym ta aplikacja nigdy nie jest dostarczana. Pobiera się na żądanie z oficjalnego indeksu wersji CEF, jest weryfikowane względem sumy SHA-1 z tego indeksu przed rozpakowaniem czegokolwiek i buforowane dla każdego użytkownika. Źródło przeglądarki, które przez nie renderuje, pojawia się na własnym etapie; to instaluje potrzebne mu środowisko uruchomieniowe.
models-cef-download-install = Pobierz i zainstaluj
models-cef-unsupported = CEF nie publikuje wersji dla tej platformy — źródła przeglądarki są tu niedostępne.
models-cef-resolving = Ustalanie najnowszej stabilnej wersji…
models-cef-verifying = Weryfikowanie pobranego pliku względem sumy SHA-1 z indeksu…
models-cef-extracting = Rozpakowywanie środowiska uruchomieniowego…
models-cef-ready = Zainstalowano — CEF { $version }.
models-cef-retry = Ponów
models-integrations-heading = Opcjonalne integracje
models-badge-never-bundled = Nigdy niedołączane
models-ndi-detected = Wykryto
models-ndi-not-installed = Niezainstalowane
models-vst-available = Dostępne
models-vst-not-available = Niedostępne


# --- Recordings.tsx ---
recordings-title = Nagrania
recordings-loading = Odczytywanie folderu…
recordings-empty = Brak nagrań — Rozpocznij nagrywanie zapisuje do folderu ustawionego w Wyjściu.
recordings-frec-label = własny bezstratny (freally-video)
recordings-remux-title = Przepakuj jako mp4 — kopia strumienia, bez ponownego kodowania, bez zmiany jakości (wymaga komponentu FFmpeg)
recordings-remuxing = Remuksowanie…
recordings-remux-to-mp4 = Remuksuj do MP4
recordings-export-mp4-title = Zdekoduj własny .frec i przekoduj do MP4 (H.264/AAC), aby odtwarzał się w każdym odtwarzaczu — wymaga komponentu FFmpeg
recordings-exporting = Eksportowanie…
recordings-export-mp4 = Eksportuj → MP4
recordings-export-mkv-title = Zdekoduj własny .frec i przekoduj do MKV, aby odtwarzał się w każdym odtwarzaczu
recordings-starting = uruchamianie…
recordings-frames = { $done } / { $total } klatek
recordings-cancel = Anuluj
recordings-export-cancelled = Eksport anulowany.
recordings-exported-to = Wyeksportowano do { $path }
recordings-remuxed-to = Zremuksowano do { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Otwórz nagranie .frec
openfrec-desc = Freally Capture nagrywa własny bezstratny format .frec — nie odtwarza go. Freally Player będzie odtwarzać .frec bezpośrednio po wydaniu. Na razie wyeksportuj do MP4/MKV, a odtworzy się w każdym odtwarzaczu (VLC, odtwarzacz systemowy, cokolwiek).
openfrec-exported-to = Wyeksportowano do { $path }
openfrec-exporting = Eksportowanie…
openfrec-starting = uruchamianie…
openfrec-export-mp4 = Eksportuj → MP4
openfrec-export-mkv = Eksportuj → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Płótno pionowe (9:16)
vertical-enable = Włącz drugie płótno — można je nagrywać i transmitować niezależnie od programu
vertical-scene-label = Scena komponowana przez to płótno
vertical-width = Szerokość
vertical-height = Wysokość
vertical-preview-alt = Podgląd płótna pionowego
vertical-note = Pozycje elementów są wierne co do piksela na wszystkich płótnach: wybierz tę scenę na pasku Sceny, aby ją rozmieścić, podczas gdy ten podgląd pokazuje wynik pionowy. Cele transmisji wybierają to płótno w ⦿ Transmisja…; Ustawienia → Wyjście mogą nagrywać je obok głównego pliku.
vertical-close = Zamknij


# --- EulaGate.tsx ---
eula-title = Freally Capture — Umowa licencyjna
eula-version = v{ $version }
eula-intro = Przeczytaj i zaakceptuj tę umowę, aby korzystać z Freally Capture. W skrócie: to neutralne narzędzie, a Ty ponosisz wyłączną odpowiedzialność za to, co przechwytujesz, nagrywasz i nadajesz — oraz za posiadanie do tego praw.
eula-thanks = Dziękujemy za przeczytanie.
eula-scroll-hint = Przewiń do końca, aby kontynuować.
eula-decline = Odrzuć i zakończ
eula-agree = Zgadzam się


# =============================================================
# --- settings ---
# =============================================================

# --- SettingsOutput.tsx ---
output-title = Wyjście
output-loading = Ustawienia wciąż się ładują…
output-container-frec = freally-video (.frec) — bezstratny, własny, nic do pobrania
output-container-mkv = MKV — odporny na awarie; remuksuj do mp4 później
output-container-mp4 = MP4 — odtwarza się wszędzie
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Bezstratny
output-preset-lossless-title = Własny kodek freally-video — dokładny co do bitu, bez pobierania
output-preset-high-label = Wysoka jakość
output-preset-high-title = MP4, najlepszy wykryty enkoder, prawie bezstratny CQ 16, preset Jakość
output-preset-balanced-label = Zrównoważony
output-preset-balanced-title = MKV, najlepszy wykryty enkoder, CQ 23, preset Zrównoważony
output-recording-format = Format nagrywania
output-ffmpeg-warning = Ten format wymaga komponentu FFmpeg (kodeki transmisyjne — niedołączane). Bezstratny .frec nie wymaga niczego.
output-install = Zainstaluj…
output-recordings-folder = Folder nagrań
output-folder-placeholder = Folder Wideo systemu
output-filename-prefix = Prefiks nazwy pliku
output-frame-rate = Liczba klatek
output-fps-option = { $fps } fps
output-split-every = Dziel co (minuty, 0 = wył.)
output-output-width = Szerokość wyjścia (0 = płótno; tylko formaty transmisyjne)
output-output-height = Wysokość wyjścia (0 = płótno)
output-record-vertical = Nagrywaj także płótno pionowe (równoległy plik „… (vertical)”; wymaga włączonego płótna 9:16)
output-audio-tracks = Ścieżki audio
output-recorded-tracks-group = Nagrywane ścieżki
output-track-last-one = Co najmniej jedna ścieżka musi być nagrywana
output-record-track-on = Nagrywaj ścieżkę { $index }: wł.
output-record-track-off = Nagrywaj ścieżkę { $index }: wył.
output-encoder-heading = Enkoder
output-video-encoder = Enkoder wideo
output-encoder-auto = Auto — najlepszy wykryty (H.264)
output-encoder-unavailable = — niedostępny tutaj
output-preset = Preset
output-preset-quality = Jakość
output-preset-balanced-option = Zrównoważony
output-preset-performance = Wydajność
output-rate-control = Kontrola przepływności
output-rc-cqp = CQP (stała jakość)
output-rc-cbr = CBR (stały bitrate)
output-rc-vbr = VBR (zmienny bitrate)
output-cq = CQ (0–51, niżej = lepiej)
output-bitrate = Bitrate (kbps)
output-keyframe = Interwał klatek kluczowych (s)
output-audio-bitrate = Bitrate audio (kbps / ścieżkę)
output-presets = Presety:

# --- SettingsStream.tsx ---
stream-title = Ustawienia — Transmisja
stream-target-enabled = Cel { $index } włączony
stream-target = Cel { $index }
stream-remove = Usuń
stream-service = Usługa
stream-canvas = Płótno
stream-canvas-main = Główne (program)
stream-canvas-vertical = Pionowe (9:16 — włącz je w studiu)
stream-ingest-srt = Adres URL wejścia SRT
stream-ingest-whip = Adres URL punktu końcowego WHIP
stream-ingest-url = Adres URL wejścia
stream-ingest-override = (zastąp — puste = preset usługi)
stream-key-srt = streamid (opcjonalnie — dołączany jako ?streamid=…; traktowany jako sekret)
stream-key-whip = Token Bearer (opcjonalnie — wysyłany jako nagłówek Authorization; sekret)
stream-key-custom = Klucz transmisji (z Twojego serwera — traktowany jako sekret)
stream-key-service = Klucz transmisji (z Twojego panelu twórcy — traktowany jako sekret)
stream-key-aria = Klucz transmisji { $index }
stream-key-hide = Ukryj
stream-key-show = Pokaż
stream-encoder = Enkoder (H.264 — to, co przenoszą RTMP, SRT i WHIP)
stream-encoder-auto = Auto — najlepszy wykryty enkoder H.264
stream-encoder-unavailable = (niedostępny tutaj)
stream-video-bitrate = Bitrate wideo (kbps, CBR)
stream-audio-bitrate = Bitrate audio (kbps)
stream-fps = FPS
stream-keyframe = Interwał klatek kluczowych (s)
stream-audio-track = Ścieżka audio (1–6)
stream-output-width = Szerokość wyjścia (0 = płótno)
stream-output-height = Wysokość wyjścia (0 = płótno)
stream-add-target = + Dodaj cel
stream-go-live-note = Wejdź na żywo publikuje jednocześnie do każdego włączonego celu, bezpośrednio na każdą platformę. Cele z identycznymi ustawieniami enkodera współdzielą jedno kodowanie.
stream-auto-record = Rozpocznij nagrywanie po wejściu na żywo (nagrywanie i tak zatrzymuje się niezależnie)
stream-ffmpeg-note-before = Transmisyjne kodeki wire działają przez oznaczony, pobierany na żądanie komponent ffmpeg —
stream-ffmpeg-note-link = zarządzaj nim tutaj
stream-ffmpeg-note-after = . Lokalne nagrywanie działa dalej, niezależnie od tego, co robi transmisja.
stream-cancel = Anuluj
stream-save = Zapisz

# --- SettingsReplay.tsx ---
replay-title = Ustawienia — Bufor powtórek
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Niska (3 Mbps)
replay-quality-standard = Standardowa (6 Mbps)
replay-quality-high = Wysoka (12 Mbps)
replay-length-presets = Presety długości
replay-quality-presets = Presety jakości
replay-length-seconds = Długość (sekundy)
replay-video-bitrate = Bitrate wideo (kbps)
replay-fps = FPS
replay-audio-track = Ścieżka audio (1–6)
replay-note = Gdy jest uzbrojony, bufor uruchamia własne, lekkie kodowanie do ograniczonego pierścienia na dysku — około { $mb } MB przy tych ustawieniach. Zapis zszywa pierścień bez ponownego kodowania i nigdy nie dotyka transmisji ani nagrywania. Zmiany są stosowane przy następnym uzbrojeniu.
replay-cancel = Anuluj
replay-save = Zapisz

# --- SettingsRemote.tsx ---
remote-title = Ustawienia — Zdalne sterowanie
remote-enable = Włącz zdalne API WebSocket
remote-password = Hasło (wymagane — kontrolery uwierzytelniają się nim)
remote-password-placeholder = hasło dla Twoich kontrolerów
remote-password-hide = Ukryj
remote-password-show = Pokaż
remote-port = Port
remote-allow-lan = Zezwól na połączenia LAN (domyślnie tylko ten komputer)
remote-note = Wył. = port jest zamknięty. Wł. = chroniony hasłem WebSocket na 127.0.0.1 (lub Twojej sieci LAN po włączeniu), który może przełączać sceny, uruchamiać przejście, rozpoczynać/zatrzymywać transmisję i nagrywanie, zapisywać powtórki oraz ustawiać wyciszenia/głośności — te same akcje co interfejs, nic więcej. Nie może odczytywać plików. Traktuj hasło jak każde dane logowania; preferuj tryb tylko dla tego komputera, chyba że celowo sterujesz z innego urządzenia.
remote-password-required = Do włączenia zdalnego API wymagane jest hasło.
remote-cancel = Anuluj
remote-save = Zapisz

# --- SettingsHotkeys.tsx ---
hotkeys-title = Ustawienia — Skróty klawiszowe
hotkeys-record = Rozpocznij / zatrzymaj nagrywanie
hotkeys-record-placeholder = np. Ctrl+Shift+R
hotkeys-go-live = Wejdź na żywo / Zakończ transmisję
hotkeys-go-live-placeholder = np. Ctrl+Shift+L
hotkeys-transition = Przejście trybu studyjnego
hotkeys-transition-placeholder = np. Ctrl+Shift+T lub F13
hotkeys-save-replay = Zapisz powtórkę (ostatnie N sekund)
hotkeys-save-replay-placeholder = np. Ctrl+Shift+S
hotkeys-add-marker = Umieść znacznik rozdziału (nagrywanie)
hotkeys-add-marker-placeholder = np. Ctrl+Shift+K
hotkeys-note = Skróty są globalne — działają, gdy inne aplikacje są aktywne. Puste = nieprzypisane. Klawisze naciśnij aby mówić/wyciszyć miksera znajdują się w menu ⋯ każdego kanału. W Linux/Wayland globalne skróty mogą być niedostępne (ograniczenie kompozytora) — przyciski nadal działają.
hotkeys-cancel = Anuluj
hotkeys-save = Zapisz

# --- WorkspaceDialog.tsx ---
workspace-title = Profile i kolekcje scen
workspace-profiles = Profile
workspace-profiles-hint = Profil to Twoje ustawienia — cel transmisji, wyjście, skróty klawiszowe. Przełączaj na program lub platformę.
workspace-collections = Kolekcje scen
workspace-collections-hint = Kolekcja to Twoje sceny + źródła. Utwórz duplikuje bieżącą jako punkt wyjścia.
workspace-active = Aktywny
workspace-switch-to = Przełącz na { $name }
workspace-active-marker = ● aktywny
workspace-new-name-placeholder = nowa nazwa…
workspace-new-name-label = Nowa nazwa { $title }
workspace-create = Utwórz

# --- ScriptsDialog.tsx ---
scripts-title = Skrypty (Lua)
scripts-empty = Brak skryptów — dodaj plik .lua. Zobacz scripts/sample.lua, aby poznać API: reaguj na zdarzenia transmisji/sceny/nagrywania i steruj tymi samymi poleceniami co zdalne API.
scripts-enable = Włącz { $path }
scripts-remove = Usuń { $path }
scripts-path-label = Ścieżka skryptu
scripts-add = Dodaj
scripts-note = Skrypty działają w piaskownicy — bez dostępu do plików ani systemu; mogą wywoływać tylko te same polecenia studia co zdalne API (przełączanie scen, przejście, nagrywanie/transmisja/powtórka, wyciszenia). Błąd skryptu jest rejestrowany i izolowany. Zmiany są stosowane w ciągu sekundy.
scripts-error-not-lua = Wskaż plik .lua.

# --- BrowserDock.tsx ---
browser-dock-title = Doki przeglądarki
browser-dock-empty = Brak doków — dodaj wyskakujący czat, stronę alertów lub swoje przyciski internetowe Companion.
browser-dock-open = Otwórz
browser-dock-remove = Usuń { $name }
browser-dock-name-placeholder = nazwa (np. Czat Twitch)
browser-dock-name-label = Nazwa doku
browser-dock-url-label = Adres URL doku
browser-dock-note = Dok otwiera się jako osobne okno, które możesz umieścić obok studia. Strona nie ma dostępu do aplikacji — po prostu się renderuje. Tylko adresy http(s); doki otwierają się dopiero po kliknięciu Otwórz.
browser-dock-error-name = Nazwij dok (np. Czat Twitch).
browser-dock-error-url = Adres URL doku musi zaczynać się od http:// lub https://.

# --- studio-preview-pane ---
studio-preview-label = Podgląd trybu studyjnego
studio-preview-heading = Podgląd
studio-preview-hint = kliknij scenę, aby załadować ją tutaj
studio-preview-empty = Podgląd pojawi się tutaj.
studio-preview-mirrors = odzwierciedla program
studio-preview-transition-select = Przejście
studio-preview-duration = Czas trwania przejścia (ms)
studio-preview-commit-title = Zatwierdź Podgląd → Program przez przejście (widzowie to zobaczą)
studio-preview-transitioning = Przechodzenie…
studio-preview-transition-button = Przejście ⇄
studio-preview-luma-placeholder = obraz wymazywania w skali szarości (png/jpg)
studio-preview-luma-label = Obraz wymazywania Luma
studio-preview-browse = Przeglądaj…
studio-preview-filter-images = Obrazy
studio-preview-filter-video = Wideo
studio-preview-stinger-placeholder = wideo stinger (ProRes 4444 .mov zachowuje kanał alfa)
studio-preview-stinger-label = Plik wideo stingera
studio-preview-stinger-cut-label = Punkt cięcia stingera (ms)
studio-preview-stinger-cut-title = Kiedy zamiana sceny następuje pod stingerem (ms od początku przejścia)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Cięcie
transition-kind-fade = Przenikanie
transition-kind-slide-left = Wsuwanie ←
transition-kind-slide-right = Wsuwanie →
transition-kind-slide-up = Wsuwanie ↑
transition-kind-slide-down = Wsuwanie ↓
transition-kind-swipe-left = Przesuwanie ←
transition-kind-swipe-right = Przesuwanie →
transition-kind-luma-linear = Wymazywanie Luma (liniowe)
transition-kind-luma-radial = Wymazywanie Luma (promieniste)
transition-kind-luma-horizontal = Wymazywanie Luma (poziome)
transition-kind-luma-diamond = Wymazywanie Luma (romb)
transition-kind-luma-clock = Wymazywanie Luma (zegar)
transition-kind-image = Wymazywanie obrazem (własne)
transition-kind-stinger = Stinger (wideo)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Niestandardowy (RTMP/RTMPS)
stream-service-srt = SRT (własny hosting)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = Informacje
about-tagline = Nagrywaj i transmituj jak w studiu — bez kont, bez chmury.
about-version = Wersja
about-created-by = Utworzone przez
about-project-started = Rozpoczęcie projektu
about-first-stable = Pierwsze stabilne wydanie
about-first-stable-pending = Jeszcze nie — 1.0.0 w przygotowaniu
about-platform = Platforma
about-local-first = Freally Capture działa w całości na Twoim komputerze. Żadnych kont, żadnej telemetrii, żadnej chmury — jedyne, co opuszcza Twój komputer, to transmisja, którą wybrałeś do wysłania.
about-website = Strona internetowa
about-issues = Zgłoś problem
about-license = Licencja
about-eula = EULA
about-third-party = Informacje o oprogramowaniu zewnętrznym
about-check-updates = Sprawdź aktualizacje…

# --- unified settings modal (TASK-906) ---
settings-title = Ustawienia
settings-language-section = Język
settings-language = Język interfejsu
settings-language-system = Domyślny systemu
settings-language-note = Wybrany tutaj język jest zapamiętywany. „Domyślny systemu” podąża za Twoim systemem operacyjnym. Nieprzetłumaczony tekst wraca do angielskiego.
settings-appearance-section = Wygląd
settings-theme = Motyw
settings-theme-dark = Ciemny
settings-theme-light = Jasny
settings-theme-custom = Niestandardowy
settings-accent = Akcent
settings-general-section = Ogólne
settings-show-stats-dock = Pokaż panel statystyk
settings-more-section = Więcej ustawień
settings-open-output = Nagrywanie…
settings-open-stream = Transmisja…
settings-open-replay = Powtórka…
settings-open-hotkeys = Skróty klawiszowe…
settings-open-remote = Zdalne API…
settings-open-about = Informacje…
controls-settings = ⚙ Ustawienia…
controls-settings-title = Język, wygląd i preferencje całej aplikacji

# --- command palette (TASK-904) ---
palette-title = Paleta poleceń
palette-search = Szukaj scen, źródeł i akcji
palette-placeholder = Szukaj scen, źródeł, akcji…
palette-no-results = Nic nie pasuje do “{ $query }”
palette-hint = ↑ ↓ aby przejść · Enter aby uruchomić · Esc aby zamknąć
palette-group-scenes = Scena
palette-group-sources = Źródło
palette-group-actions = Akcja
palette-transition = Przejście Podgląd → Program
palette-save-replay = Zapisz powtórkę
palette-add-marker = Umieść znacznik rozdziału
palette-vertical-canvas = Płótno pionowe (9:16)…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Witaj w Freally Capture
wizard-welcome = Dwa szybkie kroki: sprawdzimy, co potrafi Twój komputer, a potem uruchomimy scenę. Zajmie to około trzydziestu sekund, a wszystko możesz później zmienić.
wizard-local-first = Nic stąd nie opuszcza Twojego komputera. Freally Capture nie ma kont, telemetrii ani chmury.
wizard-start = Zaczynajmy
wizard-skip = Pomiń
wizard-hardware-title = Co potrafi Twój komputer
wizard-probing = Sprawdzamy Twoją kartę graficzną i procesor…
wizard-encoder = Koder
wizard-canvas = Płótno
wizard-bitrate = Bitrate
wizard-probe-found = Znaleziono: { $gpus } · { $cores } rdzeni fizycznych
wizard-no-gpu = brak dedykowanego GPU
wizard-apply = Użyj tych ustawień
wizard-keep-current = Zostaw to, co mam
wizard-template-title = Zacznij od sceny
wizard-template-screen = Przechwyć mój ekran
wizard-template-screen-note = Dodaje Przechwytywanie ekranu Twojego głównego monitora. Najczęstszy punkt startu.
wizard-template-empty = Zacznij od pustej
wizard-template-empty-note = Pusta scena. Źródła dodasz samodzielnie przyciskiem +.
wizard-done = Wszystko gotowe.
wizard-done-hint = W dowolnej chwili naciśnij Ctrl+K, aby wyszukać sceny, źródła i akcje. Ustawienia znajdziesz pod przyciskiem ⚙.
wizard-close = Zacznij transmisję

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Twoja karta graficzna potrafi samodzielnie kodować wideo, dzięki czemu procesor pozostaje wolny dla reszty studia.
autoconfig-reason-software = Nie znaleziono użytecznego kodera sprzętowego, więc kodowaniem zajmie się procesor. To działa, po prostu obciąża bardziej CPU.
autoconfig-reason-quality-hardware = 1080p przy 60 klatkach na sekundę, z bitrate akceptowanym przez każdą dużą platformę.
autoconfig-reason-quality-software = 30 klatek na sekundę, ponieważ kodowanie programowe przy 60 gubi klatki na większości procesorów.
autoconfig-reason-quality-low-cores = Niższy bitrate, ponieważ ten procesor ma niewiele rdzeni, a kodowanie programowe będzie o nie rywalizować z kompozytorem.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Nagrywanie rozpoczęte
announce-recording-paused = Nagrywanie wstrzymane
announce-recording-stopped = Nagrywanie zatrzymane
announce-live-started = Jesteś na żywo
announce-live-ended = Transmisja zakończona
announce-reconnecting = Utracono połączenie, ponowne łączenie
announce-stream-failed = Transmisja nie powiodła się
announce-frames-dropped = Utracono klatki: { $count }
