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
sources-badge-test-bars = Pasy
sources-badge-test-grid = Siatka
sources-badge-test-sweep = Przesuw
sources-badge-test-tone = Ton
sources-badge-test-sync = Sync
sources-badge-timer = Minutnik

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
sources-add-timer = Minutnik / Zegar
sources-add-nested-scene = Scena zagnieżdżona
sources-add-slideshow = Pokaz slajdów
sources-add-chat-overlay = Nakładka czatu na żywo
sources-add-test-signal = Sygnał testowy
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
sources-testsignal-title = Dodaj sygnał testowy
sources-testsignal-pattern-label = Wzór
sources-testsignal-bars = Pasy kolorów SMPTE
sources-testsignal-grid = Siatka kalibracyjna
sources-testsignal-sweep = Przemiatanie ruchu
sources-testsignal-tone = Ton 1 kHz (−20 dBFS)
sources-testsignal-flash-beep = Błysk + sygnał synchronizacji A/V
sources-testsignal-note = Sprawdzaj sceny, enkodery, projektory i cele transmisji bez podłączonej kamery. Wzór błysk + sygnał zasila warsztat synchronizacji A/V.
sources-testsignal-add = Dodaj sygnał testowy
sources-timer-title = Dodaj minutnik
sources-timer-mode-label = Tryb
sources-timer-wall-clock = Zegar ścienny
sources-timer-countdown = Odliczanie
sources-timer-stopwatch = Stoper
sources-timer-since-live = Czas od startu transmisji
sources-timer-since-recording = Czas od startu nagrania
sources-timer-note = Czas trwania, format, styl i akcje końca odliczania są we Właściwościach źródła.
sources-timer-add = Dodaj minutnik

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
controls-iso-lanes = Tory ISO nagrywające obok programu: { $count }
controls-pause-title-resume = Wznów — plik jest kontynuowany jako jedna ciągła oś czasu
controls-pause-title-pause = Wstrzymaj — żadne klatki nie są zapisywane; wznowienie kontynuuje ten sam odtwarzalny plik
controls-resume-recording = ▶ Wznów nagrywanie
controls-pause-recording = ⏸ Wstrzymaj nagrywanie
controls-reactions-label = Reakcje (wtopione w program)
controls-reactions-title = Wyświetl reakcję nad programem — nagrywaną ORAZ transmitowaną, aby powtórka pokazała dokładny moment. Widzowie na czacie też je wyzwalają (ich emoji reakcji unosi się automatycznie); zalew jedynie ogranicza to, co jest na ekranie.
controls-react = Reaguj { $emoji }
controls-virtual-camera-title = Kamera wirtualna wymaga własnego, podpisanego sterownika na każdy system operacyjny (Win11 MFCreateVirtualCamera / Win10 DirectShow / rozszerzenie CoreMediaIO macOS / Linux v4l2loopback) — jest dostarczana jako osobny etap. Model sygnału jest na nią gotowy: program, płótno pionowe lub pojedyncze źródło, ze sparowanym mikrofonem wirtualnym w Windows/Linux (macOS nie ma API mikrofonu wirtualnego — mówiąc szczerze).
controls-virtual-camera = ⌁ Uruchom kamerę wirtualną
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
mixer-routing = Routing
mixer-routing-title = Routing wyjścia audio

# --- RoutingMatrixDialog.tsx (CAP-N30) ---
routing-title = Routing audio
routing-intro = Przypisz paski do busów ścieżek, a następnie wyślij dowolny bus na wyjście fizyczne — sygnał do rejestratora sprzętowego, głośniki w innym pomieszczeniu lub odsłuch na słuchawki na wolnej ścieżce. Monitor zachowuje własne urządzenie; te trasy są dodawane z wierzchu, więc gdy żadna nie jest ustawiona, miks pozostaje bez zmian.
routing-sends-title = Wysyłki do ścieżek
routing-no-strips = Brak źródeł audio w tej scenie.
routing-source = Źródło
routing-track = Ścieżka { $n }
routing-send-aria = Wyślij { $source } do ścieżki { $n }
routing-outputs-title = Wyjścia fizyczne
routing-master = Master
routing-off = Wył.
routing-default-output = Domyślne wyjście
routing-device-aria = Urządzenie wyjściowe dla { $bus }
routing-trim-aria = Trim wyjścia dla { $bus }
routing-trim-db = { $db } dB
routing-muted = Wyciszone
routing-device-error = Urządzenie niedostępne

# --- DuckingMatrixDialog.tsx (CAP-N31) ---
mixer-ducking = Ducking
mixer-ducking-title = Macierz duckingu
ducking-title = Macierz duckingu
ducking-intro = Każde źródło może wyciszać dowolne inne. Komórka obniża cel (kolumnę), gdy tylko wyzwalacz (wiersz) się odezwie — wybierz komórkę, aby ustawić jej głębokość, próg i czasy. Każda para to osobny ducking, więc jeden kanał może być wyciszany przez kilka wyzwalaczy jednocześnie.
ducking-need-two = Dodaj co najmniej dwa źródła audio, aby stosować ducking między nimi.
ducking-trigger-target = Wyzwalacz ↓ / Cel →
ducking-cell-aria = { $trigger } wycisza { $target }
ducking-pair = { $trigger } → { $target }
ducking-remove = Usuń
ducking-amount = Ilość
ducking-threshold = Próg
ducking-attack = Atak
ducking-release = Zwolnienie
ducking-unit-db = dB
ducking-unit-ms = ms

# --- Loudness normalization (CAP-N34) ---
loudness-title = Normalizacja głośności
loudness-intro = Prowadzi program stopniowo w stronę docelowej głośności z limitem szczytów, aby transmisja i nagrania osiągały spójny poziom. Powoli i łagodnie — steruje, nigdy nie pompuje.
loudness-enable = Prowadź program do celu
loudness-target = Cel
loudness-target-option = { $target } LUFS
loudness-ceiling = Limit szczytów (dBFS)
loudness-note = −14 LUFS pasuje do odtwarzania w stylu YouTube; −16 to typowy cel streamingu; −23 to nadawanie EBU R128. Ten sam cel jest używany przez akcję Normalizuj po nagraniu.
ltc-badge = LTC
ltc-title = Timecode SMPTE (LTC)
ltc-intro = Generuj liniowy timecode SMPTE na ścieżce i odczytuj przychodzący LTC z dowolnego wejścia audio — klasyczny audiowy timecode do synchronizacji zewnętrznych rejestratorów i kamer w postprodukcji. W pełni offline.
ltc-generate = Generuj LTC na ścieżce
ltc-track = Ścieżka timecode
ltc-track-option = Ścieżka { $track }
ltc-fps = Liczba klatek
ltc-read = Odczytuj LTC z
ltc-read-off = Wył.
ltc-decoded = Przychodzący timecode
ltc-no-lock = brak sygnału
ltc-note = Generator synchronizuje się z porą dnia, non-drop. Nagraj jego ścieżkę (przypisz ją w ustawieniach Wyjścia) lub skieruj na wyjście, by zasilić sprzęt zewnętrzny. Czytnik zasila wiersz timecode w nakładce statystyk i stempluje znaczniki rozdziałów.
loudness-on = LUFS { $target }
loudness-off = Norm. wył.

# --- SoundboardDialog.tsx (CAP-N37) ---
mixer-soundboard = Soundboard
mixer-soundboard-title = Soundboard
soundboard-title = Soundboard
soundboard-add-pad = + Pad
soundboard-stop-all = Zatrzymaj wszystko
soundboard-edit = Edytuj
soundboard-empty = Nie ma jeszcze padów — dodaj jeden i przypisz lokalny klip audio.
soundboard-new-pad = Nowy pad
soundboard-no-clip = Brak klipu
soundboard-audio-files = Pliki audio
soundboard-name = Nazwa
soundboard-choose-clip = Wybierz klip…
soundboard-gain = Wzmocnienie
soundboard-choke = Choke
soundboard-choke-none = Brak
soundboard-loop = Zapętl
soundboard-auto-duck = Auto-ducking
soundboard-tracks = Ścieżki
soundboard-hotkey = Skrót
soundboard-hotkey-placeholder = np. Ctrl+Shift+1
soundboard-remove = Usuń

# --- PluginsDialog.tsx (CAP-N33) ---
mixer-plugins = Wtyczki
mixer-plugins-title = Wtyczki audio (CLAP / VST3)
plugins-title = Wtyczki audio
plugins-scanning = Skanowanie…
plugins-none = Nie znaleziono wtyczek CLAP ani VST3 w standardowych folderach.

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Pamięć
stats-dropped = Utracone
stats-render = Renderowanie
stats-gpu = GPU
stats-gpu-compositing = kompozycja
stats-gpu-idle = bezczynny
stats-disk = Dysk
stats-disk-free = wolne
stats-disk-left = Nagr. pozostało
stats-disk-rate = ≈ { $rate } MB/s nagrywanie
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
channelstrip-solo-title = Solo (PFL) — odsłuch słyszy tylko solowane paski; miks programu bez zmian
channelstrip-solo-source = Solo { $name } (PFL)
channelstrip-pan-label = Balans (podwójne kliknięcie zeruje)
channelstrip-pan-aria = Balans { $name }
channelstrip-mono-label = Zmiksuj do mono
channelstrip-automix-label = Auto-miks (podział wzmocnienia)
channelstrip-automix-note = Podział wzmocnienia: mikser utrzymuje stały łączny poziom wszystkich kanałów w trybie auto-miksu i przekazuje go temu, kto właśnie mówi — idealne dla paneli z wieloma mikrofonami i podcastów. Wyłączone, dopóki nie dodasz kanału.
channelstrip-mix-minus-label = Mix-minus (N−1)
channelstrip-mix-minus-note = Tworzy pozbawiony echa powrót dla tego źródła — wszyscy w programie oprócz samego źródła. Użyj go dla zdalnego gościa, aby nie słyszał własnego opóźnionego głosu.
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
properties-testtone-note = Ciągła sinusoida 1 kHz przy −20 dBFS. Poziom i wyciszenie są na pasku miksera; nic więcej nie trzeba ustawiać.
properties-timer-format = Format czasu (strftime)
properties-timer-format-note = np. %H:%M:%S (domyślny), %I:%M %p, %A %H:%M — błędny wzorzec wraca do %H:%M:%S.
properties-timer-utc = Przesunięcie UTC (minuty)
properties-timer-utc-placeholder = czas lokalny
properties-timer-duration = Czas trwania (sekundy)
properties-timer-target = Odliczaj do (HH:MM)
properties-timer-target-note = Cel zegarowy biegnie sam i powtarza się codziennie; zostaw puste, by użyć czasu trwania ze Start/Pauza/Reset.
properties-timer-end = Przy zerze
properties-timer-end-none = Nic nie rób
properties-timer-end-flash = Migaj minutnikiem
properties-timer-end-switch = Przełącz scenę
properties-timer-end-scene = Scena
properties-timer-size = Rozmiar (px)
properties-timer-start = Start
properties-timer-pause = Pauza
properties-timer-reset = Reset
properties-text-file = Czytaj z pliku (ścieżka; puste = tekst powyżej)
properties-text-binding = Interpretuj jako
properties-text-binding-whole = Cały plik
properties-text-binding-csv = Komórka CSV
properties-text-binding-json = Wskaźnik JSON
properties-text-csv-row = Wiersz
properties-text-csv-column = Kolumna
properties-text-csv-column-placeholder = nazwa lub numer
properties-text-json-pointer = Wskaźnik
properties-text-file-note = Plik jest ponownie czytany w pół sekundy po zmianie. Zapisy atomowe (temp + zmiana nazwy) są tolerowane: ostatnia dobra wartość zostaje na ekranie podczas podmiany.
avsync-title = Kalibracja synchronizacji A/V
avsync-intro = Odtwórz wbudowany wzór błysk + sygnał przez ekran i głośniki, uchwyć go kamerą i mikrofonem, które chcesz zsynchronizować — warsztat zmierzy różnicę. Pętla przechodzi przez ekran i głośniki, więc ich drobne opóźnienia są wliczone.
avsync-video-label = Kamera (źródło wideo)
avsync-audio-label = Mikrofon (źródło dźwięku)
avsync-pick = Wybierz źródło…
avsync-no-video = Najpierw dodaj kamerę jako źródło — warsztat mierzy źródła, nie surowe urządzenia.
avsync-no-audio = Najpierw dodaj mikrofon jako źródło dźwięku.
avsync-projector = Pełny ekran programu na
avsync-projector-open = Otwórz projektor
avsync-projector-window-title = Program — synchronizacja A/V
avsync-start-note = Start tymczasowo dodaje źródło „Wzór synchronizacji A/V” nad bieżącą sceną i odtwarza sygnał na urządzeniu odsłuchu. Po zakończeniu wszystko znika.
avsync-manual = Przesunięcie synchronizacji (ms, ręcznie)
avsync-start = Rozpocznij kalibrację
avsync-measuring = Pomiar przez ok. 12 sekund — skieruj kamerę na migający program i zachowaj spokój w pomieszczeniu…
avsync-flash-seen = Kamera widzi błysk
avsync-flash-waiting = Oczekiwanie, aż kamera zobaczy błysk…
avsync-beep-heard = Mikrofon słyszy sygnał
avsync-beep-waiting = Oczekiwanie, aż mikrofon usłyszy sygnał…
avsync-cancel = Anuluj
avsync-result-offset = Wideo dociera { $offset } ms po dźwięku.
avsync-result-detail = Zmierzono w { $cycles } cyklach, ±{ $jitter } ms.
avsync-negative = Dźwięk już dociera później niż wideo. Opóźnianie dźwięku nie naprawi tego kierunku — jeśli dźwięk tej kamery niesie inny pasek, obniż tam jego przesunięcie.
avsync-over-cap = Zmierzona różnica przekracza limit przesunięcia { $max } ms. Taka przepaść zwykle oznacza złe źródło — sprawdź tor i zmierz ponownie.
avsync-applied = Zastosowano — przesunięcie mikrofonu wynosi teraz { $offset } ms.
avsync-apply = Zastosuj { $offset } ms do mikrofonu
avsync-again = Zmierz ponownie
avsync-close = Zamknij
avsync-error-noFlash = Kamera nigdy nie zobaczyła błysku. Skieruj ją na migający program (pełny ekran pomaga), upewnij się, że źródło działa, i zmierz ponownie.
avsync-error-noBeep = Mikrofon nigdy nie usłyszał sygnału. Upewnij się, że urządzenie odsłuchu jest słyszalne, a mikrofon aktywny (nie zablokowany push-to-talk), i zmierz ponownie.
avsync-error-tooFewCycles = Za mało czystych cykli błysk/sygnał. Utrzymuj wzór dobrze widoczny i słyszalny przez cały pomiar.
avsync-error-notThePattern = To, co widziano lub słyszano, nie powtarza się w rytmie wzoru — to raczej światło lub hałas pokoju, nie sygnał testowy.
avsync-error-unstable = Cykle zbyt się różnią, by zaufać jednej liczbie. Ustabilizuj kamerę, ogranicz hałas i zmierz ponownie.
hotkey-audit-title = Mapa skrótów
hotkey-audit-search = Szukaj
hotkey-audit-filter = Funkcja
hotkey-audit-filter-all = Wszystkie funkcje
hotkey-audit-col-key = Klawisz
hotkey-audit-col-action = Akcja
hotkey-audit-col-where = Gdzie
hotkey-audit-col-status = Stan
hotkey-audit-ok = OK
hotkey-audit-shared = Współdzielony przez { $count } przypisania
hotkey-audit-unregistered = Niezarejestrowany w systemie (zajęty gdzie indziej lub niedostępny)
hotkey-audit-invalid = Nieprawidłowy skrót
hotkey-audit-empty = Brak skrótów — przypisz je w Ustawienia → Skróty lub na pasku miksera.
hotkey-audit-export = Eksportuj ściągę
hotkey-audit-exported = Zapisano w { $path }
hotkey-audit-note = Klawisze przypisujesz i zmieniasz w Ustawienia → Skróty (akcje globalne) i na każdym pasku miksera (push-to-talk / push-to-mute); ta tabela je audytuje i dokumentuje.
hotkey-audit-action-record = Przełącz nagrywanie
hotkey-audit-action-go-live = Przełącz transmisję
hotkey-audit-action-transition = Wykonaj przejście
hotkey-audit-action-save-replay = Zapisz powtórkę
hotkey-audit-action-add-marker = Dodaj znacznik
hotkey-audit-action-still = Zrób stopklatkę
hotkey-audit-action-panic = Ekran awaryjny
hotkey-audit-action-timer-toggle = Start/pauza wszystkich minutników
hotkey-audit-action-timer-reset = Reset wszystkich minutników
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Nagrywanie
hotkey-audit-feature-streaming = Transmisja
hotkey-audit-feature-studio = Tryb studio
hotkey-audit-feature-replay = Powtórka
hotkey-audit-feature-markers = Znaczniki
hotkey-audit-feature-stills = Stopklatki
hotkey-audit-feature-panic = Awaria
hotkey-audit-feature-timers = Minutniki
hotkey-audit-feature-audio = Dźwięk (na źródło)
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
properties-deinterlace = Usuwanie przeplotu
properties-deinterlace-off = Wyłączone
properties-deinterlace-discard = Odrzuć (podwój linie jednego pola)
properties-deinterlace-bob = Bob (pola na przemian)
properties-deinterlace-linear = Liniowo (interpolacja)
properties-deinterlace-blend = Mieszanie (średnia pól)
properties-deinterlace-adaptive = Adaptacyjne do ruchu (klasa yadif)
properties-field-order = Kolejność pól
properties-field-order-top = Najpierw górne pole
properties-field-order-bottom = Najpierw dolne pole
properties-deinterlace-note = Dla sygnałów z przeplotem z kart przechwytujących. Czysty CPU, identycznie na każdym OS; zmiana restartuje urządzenie (jak zmiana formatu).
camera-controls-title = Sterowanie kamerą
camera-controls-refresh = Odśwież
camera-controls-reset = Resetuj profil
camera-controls-empty = Teraz brak regulacji — urządzenie musi streamować (najpierw dodaj je do sceny), a część backendów nic nie zgłasza (zwłaszcza macOS). To uczciwy stan per OS.
camera-controls-note = Zmiany działają na żywo i zapisują się w profilu urządzenia, ponownie stosowanym po podłączeniu i restarcie.
camera-control-brightness = Jasność
camera-control-contrast = Kontrast
camera-control-hue = Odcień
camera-control-saturation = Nasycenie
camera-control-sharpness = Ostrość
camera-control-gamma = Gamma
camera-control-white-balance = Balans bieli
camera-control-backlight = Kompensacja tylnego światła
camera-control-gain = Wzmocnienie
camera-control-pan = Panorama
camera-control-tilt = Pochylenie
camera-control-zoom = Zoom
camera-control-exposure = Ekspozycja
camera-control-iris = Przysłona
camera-control-focus = Ostrość (fokus)
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
audiofilters-name-parametric-eq = Korektor parametryczny
audiofilters-name-de-esser = De-esser
audiofilters-name-rumble-guard = Filtr antydudnieniowy
# --- Voice-chain presets (CAP-N39) ---
audiofilters-voice-preset = Preset
audiofilters-voice-preset-pick = Preset głosu…
audiofilters-voice-broadcast = Głos broadcastowy
audiofilters-voice-podcast = Głos podcastowy
audiofilters-voice-clean = Czysty głos
audiofilters-voice-none = Wyczyść łańcuch
# --- De-esser + rumble guard params (CAP-N36) ---
audiofilters-deesser-freq = Częstotliwość sybilantów (Hz)
audiofilters-deesser-amount = Maks. redukcja (dB)
audiofilters-rumble-freq = Obcięcie niskich (Hz)
audiofilters-title = Filtry audio — { $name }

# --- ParametricEqEditor.tsx (CAP-N35) ---
eq-graph-aria = Krzywa odpowiedzi korektora parametrycznego z widmem na żywo
eq-band-type = Typ
eq-freq = Hz
eq-gain = dB
eq-q = Q
eq-add-band = + Pasmo
eq-remove-band = Usuń pasmo
eq-type-bell = Dzwon
eq-type-lowShelf = Półka dolna
eq-type-highShelf = Półka górna
eq-type-notch = Notch
eq-type-highPass = Górnoprzepustowy
eq-type-lowPass = Dolnoprzepustowy
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
filters-name-shader = Shader (WGSL)
filters-shader-gallery = Galeria
filters-shader-gallery-pick = Wczytaj ustawienie…
filters-shader-gallery-grayscale = Skala szarości
filters-shader-gallery-invert = Odwróć
filters-shader-gallery-scanlines = Linie skanowania
filters-shader-gallery-vignette = Winieta
filters-shader-source = Kod shadera (WGSL)
filters-shader-hint = Napisz w WGSL funkcję effect(uv, color, p, texel, time) zwracającą vec4. Oznacz parametry przez // @param name min max default, aby uzyskać suwaki. Nieprawidłowy shader jest ignorowany — źródło jest renderowane bez filtra, dopóki się nie skompiluje.
filters-name-bezier-mask = Maska Béziera
filters-mask-editor-hint = Przeciągnij punkt, aby go przesunąć, kliknij dwukrotnie, aby dodać, kliknij prawym przyciskiem, aby usunąć punkt.
filters-mask-shape = Kształt
filters-mask-shape-pick = Ustawienie…
filters-mask-shape-rectangle = Prostokąt
filters-mask-shape-diamond = Romb
filters-mask-shape-hexagon = Sześciokąt
filters-mask-shape-circle = Okrąg
filters-mask-feather = Wtapianie
filters-mask-export-wipe = Eksportuj jako wycieranie…
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
recordings-trim = Przytnij
recordings-trim-title = Wytnij klip z tego nagrania — cięcia wyrównane do klatek kluczowych eksportują się bez rekodowania
recordings-verify = Zweryfikuj
recordings-verify-title = Sprawdź integralność pliku — strukturę kontenera, ciągłość, przeplot A/V, czas trwania
recordings-verifying = Weryfikowanie…
verify-dismiss = Zamknij
verify-verdict-pass = { $name } — integralność w porządku
verify-verdict-warn = { $name } — zweryfikowano z ostrzeżeniami
verify-verdict-fail = { $name } — znaleziono problemy
verify-container = Kontener
verify-video-continuity = Ciągłość wideo
verify-audio-continuity = Ciągłość audio
verify-av-interleave = Przeplot A/V
verify-duration = Czas trwania
recordings-alpha-label = alfa
recordings-prores-title = Eksportuj master .mov ProRes 4444 zachowujący alfę (do montażu)
recordings-qtrle-title = Eksportuj .mov QuickTime Animation zachowujący alfę (maksymalna zgodność)
trim-title = Przytnij — { $name }
trim-loading = Odczytywanie pliku…
trim-preview-alt = Klatka podglądu
trim-position = Pozycja odtwarzania
trim-step-second-back = Sekunda wstecz
trim-step-frame-back = Klatka wstecz
trim-step-frame-forward = Klatka naprzód
trim-step-second-forward = Sekunda naprzód
trim-snap = Klatka kluczowa
trim-snap-title = Przyciągnij do najbliższej klatki kluczowej — cięcie tam eksportuje się bez rekodowania
trim-set-in = Punkt wejścia
trim-set-out = Punkt wyjścia
trim-range-invalid = Punkt wyjścia musi być po punkcie wejścia.
trim-copy-badge = ✓ Eksport bez rekodowania — punkt wejścia leży na klatce kluczowej.
trim-reencode-badge = Zostanie zrekodowane: punkt wejścia leży między klatkami kluczowymi (użyj „Klatka kluczowa", by przyciągnąć do cięcia bezstratnego).
trim-export = Eksportuj klip
trim-export-916 = 9:16
trim-export-916-title = Eksport w pionie z nowym kadrem (wyśrodkowane przycięcie do rozmiaru pionowej kanwy) — zawsze rekoduje
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
recordings-normalize = Normalizuj
recordings-normalizing = Normalizowanie…
recordings-normalize-title = Normalizuj głośność do celu (zapisuje kopię)
recordings-normalized-to = Znormalizowano do { $path }

# --- Audio-only recording (CAP-N38) ---
audiorec-title = Tylko audio
audiorec-format = Format nagrywania audio
audiorec-format-wav = WAV
audiorec-format-flac = FLAC
audiorec-format-opus = Opus
audiorec-start = Nagraj audio
audiorec-stop = Zatrzymaj
audiorec-pause = Wstrzymaj
audiorec-resume = Wznów
audiorec-recording = REC { $sec }s
audiorec-saved = Zapisano { $count } plik(i) ścieżki


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
output-recording-template = Nazwa pliku nagrania
output-replay-template = Nazwa pliku powtórki
output-still-template = Nazwa pliku klatki
output-template-tokens = Tokeny: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Folder powtórek
output-still-folder = Folder klatek
output-same-folder-placeholder = Folder nagrań
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
output-iso-heading = Nagrywanie ISO
output-iso-explainer = Nagrywaj wybrane źródła w czystej postaci, każde do własnego pliku obok programu — przed kompozycją, w rozmiarze i klatkażu kanwy, dzięki czemu każdy plik trafia wyrównany na oś czasu montażu. Dwa tory są komfortowe na średniej klasy GPU; każdy kolejny to dodatkowy render i enkodowanie.
output-iso-none = W kolekcji nie ma jeszcze źródeł.
output-iso-source-on = „{ $name }” nagrywa się do własnego pliku ISO — kliknij, aby zatrzymać
output-iso-source-off = Nagrywaj „{ $name }” do własnego pliku ISO
output-iso-post-filter = Nagrywaj z filtrami źródła (post-filtr); bez zaznaczenia nagrywane jest surowe źródło
output-iso-format = Format ISO
output-iso-encoder = Enkoder wideo ISO
output-alpha-frec = Nagrywaj z przezroczystością (alfa) — program na przezroczystym tle
output-alpha-title = Rejestrator dostaje własny przezroczysty render; podgląd i stream pozostają normalne. Eksportuj do ProRes 4444 lub QTRLE z listy nagrań — MP4/MKV spłaszczają alfę.
output-split-events = Zaczynaj nowy plik także przy… (każda część zaczyna się dokładnie na zdarzeniu; minimalna długość 1 s)
output-split-on-scene = zmianie sceny
output-split-on-marker = znaczniku
output-split-on-rundown = kroku scenariusza
output-auto-markers = Automatycznie wstawiaj znaczniki rozdziałów przy zdarzeniach studia (zmiana sceny, zapis powtórki, ponowne łączenie, zgubione klatki, alarmy, reguły)
output-auto-markers-title = Typowane znaczniki trafiają do rozdziałów nagrania (mkv) lub pliku .chapters.txt, obok ręcznego skrótu znacznika
output-pipeline-heading = Potok po nagraniu
output-pipeline-explainer = Po sfinalizowaniu nagrania te kroki działają na pliku głównym, po kolei, w tle. Zamknięty zestaw akcji — celowo nie ma kroku „uruchom polecenie". Łańcuch zatrzymuje się na pierwszym błędzie.
output-pipeline-enabled = Uruchamiaj potok po każdym nagraniu
output-pipeline-add = Dodaj krok…
output-pipeline-up = W górę
output-pipeline-down = W dół
output-pipeline-remove = Usuń krok
output-pipeline-template = Szablon zmiany nazwy (tokeny CAP-M25)
output-pipeline-folder = Folder
pipeline-queue = Potok po nagraniu
pipeline-verify = Zweryfikuj
pipeline-remux = Remuxuj do MP4
pipeline-normalize = Znormalizuj głośność
pipeline-rename = Zmień nazwę
pipeline-move = Przenieś do folderu
pipeline-copy = Skopiuj do folderu
pipeline-reveal = Pokaż w menedżerze plików
pipeline-luaEvent = Powiadom skrypty Lua
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
hotkeys-go-live = Wejdź na żywo / Zakończ transmisję
hotkeys-transition = Przejście trybu studyjnego
hotkeys-save-replay = Zapisz powtórkę (ostatnie N sekund)
hotkeys-add-marker = Umieść znacznik rozdziału (nagrywanie)
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

# --- OBS import (CAP-M02) ---
workspace-import-obs = Importuj z OBS…
workspace-import-obs-hint = Wczytaj kolekcję scen OBS (jej scenes.json). Bieżąca kolekcja zostanie najpierw zapisana.
workspace-import-busy = Importowanie…
workspace-import-title = Zaimportowano „{ $name }"
workspace-import-summary = sceny: { $scenes } · źródła: { $sources } · elementy: { $items }
workspace-import-dismiss = Zamknij
workspace-import-clean = Wszystko zaimportowano bez problemów.
workspace-import-geometry-caveat = Rozmiary i pozycje są dopasowywane z układu OBS — sprawdź każdą scenę i wybierz ponownie urządzenia przechwytywania.
workspace-import-notes-title = Zaimportowano z uwagami
workspace-import-skipped-title = Nie zaimportowano
import-note-needsReselect = Wybierz ponownie urządzenie/monitor/okno
import-note-gameCaptureAsWindow = Przechwytywanie gry → Przechwytywanie okna
import-note-referencesFile = Sprawdź ścieżkę pliku
import-note-filterDropped = Niektóre filtry nieobsługiwane
import-note-geometryApproximated = Pozycja/rozmiar przybliżone
import-skip-unsupportedKind = Brak odpowiedniego typu źródła
import-skip-group = Grupy nie są jeszcze obsługiwane

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Połącz ponownie brakujące pliki…
doctor-title = Brakujące pliki
doctor-scanning = Skanowanie…
doctor-all-good = Wszystkie przywoływane pliki istnieją. Nie ma czego łączyć.
doctor-intro = Nie znaleziono { $count } przywoływanych plików na tym komputerze. Wskaż nowe położenie każdego — każda scena, która go używa, zostanie naprawiona naraz.
doctor-relinked = Połączono ponownie { $count } odwołań.
doctor-uses = użyto { $count }×
doctor-locate = Znajdź…
doctor-locate-folder = Szukaj w folderze…
doctor-locate-folder-hint = Wybierz folder; każdy brakujący plik zostanie dopasowany po nazwie i połączony ponownie.
doctor-kind-image = obraz
doctor-kind-media = multimedia
doctor-kind-slideshow = pokaz slajdów
doctor-kind-font = czcionka
doctor-kind-lut = LUT
doctor-kind-mask = maska
history-relinkFiles = Połącz pliki ponownie

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
studio-preview-stinger-matte-label = Matte ścieżki
studio-preview-stinger-matte-title = Jak stinger z matte ścieżki pakuje przezroczystość: wypełnienie i jego matte obok siebie (poziomo) lub jeden na drugim (pionowo)
studio-preview-stinger-duck-label = Wycisz program
studio-preview-stinger-duck-title = Wyciszaj dźwięk programu pod własnym dźwiękiem stingera podczas jego odtwarzania (0 = wył.)
studio-preview-stinger-duck-unit = dB

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
transition-kind-move = Przesunięcie (morfing)

# --- stinger track-matte modes (rendered from STINGER_MATTES in api/types.ts) ---
stinger-matte-none = Brak
stinger-matte-horizontal = Obok siebie
stinger-matte-vertical = Jeden na drugim

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
settings-open-about = Informacje…

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

# CAP-M01 — undo/redo edit history
palette-undo = Cofnij
palette-redo = Ponów
palette-edit-history = Historia zmian…
history-title = Historia zmian
history-empty = Nie ma jeszcze nic do cofnięcia.
history-current = Bieżący stan
history-close = Zamknij
history-addScene = Dodaj scenę
history-renameScene = Zmień nazwę sceny
history-removeScene = Usuń scenę
history-reorderScene = Zmień kolejność scen
history-addSource = Dodaj źródło
history-removeSource = Usuń źródło
history-reorderSource = Zmień kolejność źródeł
history-renameSource = Zmień nazwę źródła
history-transformSource = Przenieś źródło
history-toggleVisibility = Przełącz widoczność
history-toggleLock = Przełącz blokadę
history-setBlendMode = Zmień tryb mieszania
history-editSourceProperties = Edytuj właściwości
history-applyLayout = Rozmieść układ
history-moveToSeat = Przenieś na miejsce
history-groupSources = Grupuj źródła
history-ungroupSources = Rozgrupuj źródła
history-toggleGroupVisibility = Przełącz grupę
history-setSceneAudio = Dźwięk sceny
history-setVerticalCanvas = Płótno pionowe
history-addFilter = Dodaj filtr
history-removeFilter = Usuń filtr
history-reorderFilter = Zmień kolejność filtrów
history-editFilter = Edytuj filtr
history-toggleFilter = Przełącz filtr
history-setVolume = Dostosuj głośność
history-toggleMute = Przełącz wyciszenie
history-setMonitor = Zmień monitorowanie
history-setTracks = Zmień ścieżki
history-setSyncOffset = Dostosuj synchronizację A/V
history-setAudioHotkeys = Skróty audio

# CAP-M04 — alignment aids
settings-alignment-section = Pomoce wyrównania
settings-smart-guides = Inteligentne prowadnice (przyciąganie podczas przeciągania)
settings-safe-areas = Nakładki bezpiecznego obszaru
settings-rulers = Linijki
align-group = Wyrównaj do płótna
align-left = Wyrównaj do lewej
align-hcenter = Wyśrodkuj w poziomie
align-right = Wyrównaj do prawej
align-top = Wyrównaj do góry
align-vcenter = Wyśrodkuj w pionie
align-bottom = Wyrównaj do dołu

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Wyrównaj i rozmieść zaznaczenie
arrange-left = Wyrównaj lewe krawędzie
arrange-hcenter = Wyśrodkuj w poziomie
arrange-right = Wyrównaj prawe krawędzie
arrange-top = Wyrównaj górne krawędzie
arrange-vcenter = Wyśrodkuj w pionie
arrange-bottom = Wyrównaj dolne krawędzie
distribute-h = Rozmieść poziomo
distribute-v = Rozmieść pionowo
guides-group = Prowadnice
guides-add-v = Dodaj prowadnicę pionową
guides-add-h = Dodaj prowadnicę poziomą
guides-clear = Usuń wszystkie prowadnice
history-arrangeItems = Rozmieść elementy
history-editGuides = Edytuj prowadnice

# CAP-M05 — edit transform + copy/paste
transform-title = Edytuj transformację — { $name }
transform-anchor = Zakotwiczenie
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Obrót
transform-crop = Przycięcie
transform-crop-left = Lewa
transform-crop-top = Góra
transform-crop-right = Prawa
transform-crop-bottom = Dół
transform-no-size = Rozmiar i przycięcie będą dostępne, gdy źródło poda swoje wymiary.
transform-copy = Kopiuj transformację
transform-paste = Wklej transformację
transform-close = Zamknij
filters-copy = Kopiuj filtry ({ $count })
filters-paste = Wklej filtry ({ $count })
palette-edit-transform = Edytuj transformację…
history-pasteFilters = Wklej filtry

# CAP-M26 — keying workbench
workbench-title = Stół kluczowania — { $name }
workbench-mode-keyed = Z kluczem
workbench-mode-source = Źródło
workbench-mode-matte = Matte
workbench-mode-split = Podzielony
workbench-eyedropper = Kroplomierz
workbench-eyedropper-hint = Kliknij źródło, aby pobrać kolor klucza.
workbench-loupe = Lupa
workbench-split = Podział
workbench-preview-alt = Podgląd stołu kluczowania
workbench-tune = Dostrój
workbench-close = Zamknij

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Kliknij scenę, aby na nią przełączyć.
multiview-hint-stage = Kliknij scenę, aby przygotować ją w podglądzie.
palette-multiview = Monitor multiview

# CAP-M07 — projectors
projector-title = Otwórz projektor
projector-source = Źródło
projector-target-program = Program
projector-target-preview = Podgląd
projector-target-scene = Scena…
projector-target-source = Źródło…
projector-target-multiview = Multiview
projector-which-scene = Która scena
projector-which-source = Które źródło
projector-none = Nie ma czego pokazać
projector-display = Ekran
projector-windowed = Pływające okno (ten ekran)
projector-display-option = Ekran { $n } — { $w }×{ $h }
projector-primary = (główny)
projector-open = Otwórz
projector-cancel = Anuluj
projector-exit-hint = Naciśnij Esc, aby wyjść
palette-projector = Otwórz projektor…

# CAP-M08 — still-frame grab
palette-still = Przechwyć klatkę…
still-saved-toast = Klatka zapisana: { $name }
still-failed-toast = Przechwytywanie klatki nie powiodło się: { $error }
hotkeys-still = Przechwyć klatkę

# CAP-M13 — source health dashboard
palette-source-health = Stan źródeł…
palette-av-sync = Kalibracja synchronizacji A/V…
palette-hotkey-audit = Mapa skrótów…
health-title = Stan źródeł
health-col-source = Źródło
health-col-state = Stan
health-col-resolution = Rozdzielczość
health-col-fps = FPS
health-col-last-frame = Ostatnia klatka
health-col-dropped = Porzucone
health-col-retries = Restarty
health-col-actions = Akcje
health-state-live = Na żywo
health-state-waiting = Oczekiwanie
health-state-error = Błąd
health-state-inactive = Nieaktywne
health-restart = Uruchom ponownie
health-properties = Właściwości
health-empty = Ta kolekcja nie ma jeszcze źródeł.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Zamknąć Freally Capture?
quit-body = Zamknięcie teraz bezpiecznie wykona kolejno:
quit-consequence-stream = Zakończy transmisję na żywo i rozłączy się z usługą.
quit-consequence-recording = Zatrzyma nagrywanie i sfinalizuje pliki.
quit-consequence-replay = Wyłączy bufor powtórek — niezapisany materiał zostanie odrzucony.
quit-confirm = Zamknij bezpiecznie
quit-quitting = Zamykanie…
quit-cancel = Anuluj

# CAP-M11 — crash-safe recording salvage
salvage-title = Odzyskać przerwane nagrania?
salvage-body = Ostatnia sesja zakończyła się nieoczekiwanie, gdy te nagrania były jeszcze zapisywane. Naprawa tworzy odtwarzalną kopię obok oryginału — oryginalny plik nigdy nie jest zmieniany.
salvage-repair = Napraw
salvage-repairing = Naprawianie…
salvage-done = Naprawiono
salvage-repaired = Naprawiono → { $name }
salvage-failed = Naprawa nie powiodła się: { $error }
salvage-dismiss = Nie teraz

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Awaria enkodera — przełączono z { $from } na { $to }. Transmisja połączyła się ponownie i trwa dalej.
fallback-toast-recording = Awaria enkodera — przełączono z { $from } na { $to }. Nagrywanie trwa w nowym pliku.
fallback-note = Enkoder zapasowy: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Dźwięk programu ucichł
alarm-clipping = Dźwięk programu jest przesterowany
alarm-black = Obraz programu jest czarny
alarm-frozen = Obraz programu od dłuższej chwili się nie zmienia
alarm-lowDisk = Miejsce na dysku: zostało około { $minutes } min przy obecnym bitrate
alarm-dismiss = Zamknij alarm
alarm-cleared = Rozwiązano: { $alarm }

# CAP-M22 — panic button
palette-panic = Panika — przełącz na planszę prywatności
panic-banner-title = Panika
panic-banner-body = Program pokazuje planszę prywatności; cały dźwięk jest wyciszony, a przechwytywanie zatrzymane. Transmisja i nagrywanie trwają.
panic-restore = Przywróć…
panic-restore-confirm = Przywrócić program?
panic-restore-yes = Przywróć
panic-restore-cancel = Anuluj
hotkeys-panic = Panika (plansza prywatności)
hotkeys-timer-toggle = Start/pauza wszystkich minutników
hotkeys-timer-reset = Reset wszystkich minutników
panic-slate-color = Kolor planszy paniki
panic-slate-image = Obraz planszy paniki
panic-slate-image-placeholder = Opcjonalna ścieżka obrazu

# CAP-M24 — redacted diagnostics bundle
diag-title = Pakiet diagnostyczny
diag-intro = Eksportuje oczyszczony .zip (migawka konfiguracji, sonda enkoderów, ostatnie statystyki — sekrety, ścieżki i nazwy nigdy nie są dołączane) do ręcznego załączenia do zgłoszenia na GitHubie. Nic nie jest wysyłane.
diag-preview = Zobacz zawartość
diag-hide-preview = Ukryj podgląd
diag-export = Eksportuj .zip
diag-exported = Wyeksportowano: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Kontrola przed transmisją
preflight-intro = Każdy blokujący punkt musi być zielony; reszta to uczciwe wskazówki.
preflight-item-targets = Cele transmisji skonfigurowane (klucz/URL)
preflight-item-encoder = Dostępny użyteczny enkoder
preflight-item-sources = Wszystkie źródła sprawne
preflight-item-disk = Miejsce na dysku na nagranie
preflight-item-mic = Poziom mikrofonu
preflight-item-desktopAudio = Poziom dźwięku pulpitu
preflight-item-replay = Bufor powtórek uzbrojony
preflight-targets-detail = { $count } włączonych
preflight-sources-detail = { $count } źródło/a z błędem
preflight-disk-detail = ~{ $minutes } min przy obecnym bitrate
preflight-fix-stream = Ustawienia transmisji…
preflight-fix-components = Komponenty…
preflight-fix-sources = Stan źródeł…
preflight-fix-replay = Uzbrój
preflight-optional = opcjonalne
preflight-hold = Wstrzymaj Go Live, aż wszystko będzie zielone
preflight-cancel = Anuluj
preflight-go-anyway = Mimo to nadawaj
preflight-go-live = Nadawaj


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Tło
scenes-backdrop-aria = Tło sceny { $name }
backdrop-title = Tło — { $name }
backdrop-hint = Tapeta przypięta za wszystkim w tej scenie — obraz, animowany GIF lub zapętlone wideo. Twoje przechwytywanie zawsze jest na wierzchu; przewiń na kanwie, aby powiększyć.
backdrop-choose = Wybierz obraz lub wideo…
backdrop-remove = Usuń tło
backdrop-none = Brak tła.
backdrop-position = Położenie
backdrop-split-full = Cała kanwa
backdrop-split-left = Lewa połowa
backdrop-split-right = Prawa połowa
backdrop-split-top = Górna połowa
backdrop-split-bottom = Dolna połowa
backdrop-sync = Rozpocznij odtwarzanie wraz z nagrywaniem
backdrop-sync-hint = Zatrzymuje się na pierwszej klatce do startu nagrania; każde ujęcie zaczyna wideo od początku.
backdrop-preview-play = Podgląd odtwarzania
backdrop-preview-pause = Wstrzymaj podgląd
backdrop-filter-all = Tła (obrazy i wideo)
backdrop-filter-images = Obrazy
backdrop-filter-media = Wideo i GIF
sources-backdrop-badge = Tapeta tła (przypięta na dole)
sources-backdrop-pinned = Tło pozostaje przypięte na samym dole
filters-name-flip = Odbicie
filters-flip-horizontal = W poziomie
filters-flip-vertical = W pionie
history-setSceneBackdrop = Ustaw tło
history-setBackdropSplit = Przenieś tło
history-setBackdropSync = Synchronizacja tła z nagrywaniem
backdrop-scrub = Pozycja odtwarzania
backdrop-loop = Pętla
backdrop-reverse = Odtwarzaj od tyłu
backdrop-reverse-hint = Odwrócenie renderuje jednorazowo odwróconą kopię (wideo wymaga komponentu ffmpeg; GIF-y odwracają się od razu) — pierwsze przełączenie może potrwać przy długich plikach.
filters-scaling = Skalowanie
filters-scaling-hint = Tryby pixel-perfect dla treści retro/pikselowych; Całkowite dodatkowo przyciąga rysowany rozmiar do pełnych wielokrotności (uchwyty pokazują rozmiar logiczny).
filters-scaling-auto = Gładkie
filters-scaling-nearest = Najbliższy sąsiad
filters-scaling-integer = Całkowite (pełne ×)
filters-scaling-sharp = Ostre bilinearne
history-setScaling = Zmień skalowanie
hotkeys-zoom-100 = Zoom: resetuj (100%)
hotkeys-zoom-150 = Zoom: przybliż do 150%
hotkeys-zoom-200 = Zoom: przybliż 2×
sources-follow-title = Podążaj za kursorem podczas zbliżenia (Windows; przewiń na kanwie, aby powiększyć)
sources-follow-item = Przełącz podążanie za kursorem dla { $name }
filters-autocrop = ✂ Przytnij czarne pasy
filters-autocrop-title = Skanuje następną klatkę w poszukiwaniu pasów letterbox/pillarbox i je przycina (odwracalne). Ciemne sceny nigdy nie są przycinane.
filters-autocrop-follow = Sprawdź ponownie przy zmianie rozdzielczości
history-autoCrop = Automatyczne przycięcie pasów
sources-link-audio = Przechwytuj też dźwięk tej aplikacji (powiązane: ukrycie wycisza, usunięcie okna usuwa)
history-addLinkedWindow = Dodaj okno + powiązany dźwięk
sources-hdr-title = Ten ekran jest HDR — otwórz mapowanie tonów (kanwa pozostaje SDR)
sources-hdr-item = Mapowanie tonów HDR dla { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = Ten ekran wysyła HDR. Bez mapowania tonów światła się ścinają, a przechwycenie wygląda na sprane na kanwie SDR. Zmiany działają od następnej klatki.
sources-hdr-enable-suggested = Włącz sugerowane (maxRGB, 200 nitów)
sources-hdr-operator = Operator
sources-hdr-op-clip = Przycięcie (wył.)
sources-hdr-op-maxrgb = maxRGB (zachowuje odcień)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = Kolano BT.2408 (SDR dokładne)
sources-hdr-paper-white = Biel papieru
sources-hdr-nits = nity
projector-target-passthrough = Monitor przelotowy (niskie opóźnienie)
projector-which-device = Urządzenie
projector-passthrough-none = Dodaj najpierw ekran, okno lub urządzenie przechwytujące.
projector-passthrough-about = Surowe klatki urządzenia — bez scen, filtrów i kompozytora. Pokazuje zmierzone opóźnienie; dźwięk nadal monitorujesz na kanale miksera.
projector-passthrough-hint = Przelot — Esc zamyka
projector-latency = { $ms } ms
projector-latency-measuring = pomiar…
automation-title = Automatyzacja — reguły, makra i zmienne
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Reguły
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = Wł.
automation-rule-name = Rule name
automation-remove = Remove
automation-when = Gdy
automation-then-run = wtedy uruchom
automation-no-macro = (no macro)
automation-macros = Makra
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Uruchom
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Zmienne studia
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
rundown-title = Scenariusz programu
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Start
rundown-next = Dalej ▸
rundown-stop = Stop
rundown-idle = Nie działa
rundown-next-up = Następnie: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Krok
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
automation-layer = Warstwa
automation-layer-hint = Działa tylko przy aktywnej tej warstwie (puste = wszystkie). Warstwy są trwałe: klawisz warstwy przełącza i zostaje (API systemu nie ma warstw na przytrzymanie).
automation-chord-hint = Zwykły klawisz (Ctrl+Shift+M) lub akord dwuklawiszowy (Ctrl+K, 3). Drugi klawisz jest zajmowany tylko podczas oczekiwania na akord.
panel-title = Panel LAN i tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Udostępnij panel
panel-port = Port
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Hasło
panel-show = Pokaż
panel-hide = Ukryj
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Zapisz
osc-title = Powierzchnia sterowania OSC
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Nasłuchuj OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
ptz-title = Kamery PTZ
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Kamera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Adres
ptz-port = Port
ptz-speed = Prędkość
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
ptz-presets = Presety
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Przywołaj
ptz-store = Zapisz
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
midi-title = Powierzchnia sterowania MIDI
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Wejście
midi-output = Wyjście (sprzężenie)
midi-none = (none)
midi-learn = Ucz
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Akcja
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
panel-lan-warning = ⚠ Ruch LAN nie jest szyfrowany — hasło jest w adresie URL po HTTP. Używaj tylko w zaufanej sieci.
osc-lan-warning = ⚠ OSC nie ma hasła — każde urządzenie w sieci może wysłać te polecenia. Trybu LAN używaj tylko w zaufanej sieci.

# System-stats HUD source (CAP-N14)
sources-badge-stats = Stat.
sources-add-system-stats = Statystyki wydajności (HUD)
sources-stats-title = Dodaj HUD wydajności
sources-stats-note = Pokazuje widzom w programie prawdziwe zmierzone liczby studia — fps, CPU, pamięć, czas renderowania, zgubione klatki i bieżący bitrate. Wybór linii, rozmiar i kolor znajdują się we Właściwościach źródła. Użycie GPU nie jest pokazywane, bo nie jest mierzone.
sources-stats-add = Dodaj HUD statystyk
properties-stats-show-fps = Pokaż FPS
properties-stats-show-cpu = Pokaż CPU
properties-stats-show-memory = Pokaż pamięć
properties-stats-show-render = Pokaż czas renderowania
properties-stats-show-dropped = Pokaż zgubione klatki
properties-stats-show-bitrate = Pokaż bitrate
properties-stats-show-timecode = Pokaż timecode (LTC)
properties-stats-size = Rozmiar (px)
properties-stats-note = HUD rysuje zwięzłe uniwersalne etykiety (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) bezpośrednio w programie; gdy nic nie jest nadawane, linia bitrate pokazuje „—”.

# Audio visualizer source (CAP-N15)
sources-badge-visualizer = Wizualizer
sources-add-visualizer = Wizualizer dźwięku
sources-visualizer-title = Dodaj wizualizer dźwięku
sources-visualizer-style-label = Styl
sources-visualizer-style-bars = Słupki widma
sources-visualizer-style-scope = Oscyloskop
sources-visualizer-style-vu = Mierniki VU
sources-visualizer-target-label = Nasłuchuje
sources-visualizer-target-master = Miks główny
sources-visualizer-target-track = Ścieżka { $n }
sources-visualizer-note = Rysuje sygnał, który naprawdę trafia do miksu (post-fader) — wyciszone źródło jest płaskie, dokładnie tak, jak brzmi. Rozmiar, kolor, liczba słupków i tempo opadania są we Właściwościach źródła.
sources-visualizer-add = Dodaj wizualizer
properties-vis-bands = Słupki
properties-vis-decay = Tempo opadania (dB/s)
properties-vis-peak-hold = Znaczniki szczytów
properties-vis-missing-source = (brak źródła)

# Speedrun split timer source (CAP-N18)
sources-badge-splits = Splity
sources-add-split-timer = Timer splitów (speedrun)
sources-splits-title = Dodaj timer splitów
sources-splits-file-label = Plik .lss LiveSplit
sources-splits-comparison-label = Porównuj z
sources-splits-comparison-pb = Rekord osobisty
sources-splits-comparison-best = Najlepsze segmenty
sources-splits-comparison-average = Średnia
sources-splits-note = Plik jest importowany tylko do odczytu — nic nie jest do niego zapisywane. Przypisz globalne klawisze Split / Undo / Skip / Reset w Ustawienia → Skróty. Auto-splittery przez pamięć procesu celowo nie są obsługiwane.
sources-splits-add = Dodaj timer splitów
properties-splits-size = Rozmiar (px)
properties-splits-ahead = Przed
properties-splits-behind = Za
properties-splits-gold = Złoto
properties-splits-split = Split
properties-splits-undo = Cofnij
properties-splits-skip = Pomiń
properties-splits-reset = Resetuj
properties-splits-note = Przyciski sterują działającym timerem (globalne skróty robią to samo z każdej aplikacji). Bieg nigdy nie jest zapisywany do pliku .lss.
hotkeys-split-split = Timer splitów: start / split
hotkeys-split-undo = Timer splitów: cofnij split
hotkeys-split-skip = Timer splitów: pomiń segment
hotkeys-split-reset = Timer splitów: resetuj
hotkey-audit-action-split-split = Split (timer splitów)
hotkey-audit-action-split-undo = Cofnij split
hotkey-audit-action-split-skip = Pomiń segment
hotkey-audit-action-split-reset = Resetuj timer splitów
hotkey-audit-feature-split-timer = Timer splitów

# Media playlist source (CAP-N17)
sources-badge-playlist = Playlista
sources-add-playlist = Playlista mediów (bez przerw)
sources-playlist-title = Dodaj playlistę mediów
sources-playlist-files-label = Pliki (po jednym w wierszu, odtwarzane od góry)
sources-playlist-browse = Przeglądaj…
sources-playlist-loop = Zapętl
sources-playlist-shuffle = Losowo (jedno losowanie na start; w pętli powtarza tę kolejność)
sources-playlist-hold-last = Zatrzymaj ostatnią klatkę na końcu
sources-playlist-note = Odtwarza całą przyciętą listę bez przerw przez oznaczony komponent ffmpeg (tylko formaty wire — .frec i obrazy przez Media/Pokaz slajdów). Pozycje są wszystkie wideo albo wszystkie audio, nigdy mieszane. Przycięcia, punkty cue i zmienna „now playing” są we Właściwościach.
sources-playlist-add = Dodaj playlistę
properties-playlist-items = Pozycje (od góry do dołu)
properties-playlist-up = W górę
properties-playlist-down = W dół
properties-playlist-remove = Usuń pozycję
properties-playlist-in = Od (s)
properties-playlist-out = Do (s)
properties-playlist-cues = Cue (s, po przecinku)
properties-playlist-add-item = + Dodaj pozycję
properties-playlist-loop = Zapętl
properties-playlist-shuffle = Losowo
properties-playlist-hold-last = Zatrzymaj ostatnią klatkę
properties-playlist-hw = Dekodowanie sprzętowe
properties-playlist-variable = Zmienna „now playing” (puste = wył.)
properties-playlist-previous = ⏮ Poprzedni
properties-playlist-next = ⏭ Następny
properties-playlist-note = Przyciski cue i Następny/Poprzedni sterują DZIAŁAJĄCĄ playlistą; zmiany pozycji obowiązują po Zastosuj (playlista startuje od nowa). Wstaw {"{{"}yourVariable{"}}"} do źródła Tekst, by pokazać graną pozycję.
hotkeys-playlist-next = Playlista: następna pozycja
hotkeys-playlist-previous = Playlista: poprzednia pozycja
hotkey-audit-action-playlist-next = Playlista: następny
hotkey-audit-action-playlist-previous = Playlista: poprzedni
hotkey-audit-feature-playlist = Playlista

# Instant replay source (CAP-N10)
sources-badge-replay = Powtórka
sources-add-replay = Natychmiastowa powtórka
sources-replay-title = Dodaj natychmiastową powtórkę
sources-replay-seconds-label = Długość rolki (sekundy)
sources-replay-speed-label = Prędkość
sources-replay-speed-full = 100% (z dźwiękiem)
sources-replay-speed-half = 50% zwolnione (bez dźwięku)
sources-replay-speed-quarter = 25% zwolnione (bez dźwięku)
sources-replay-note = Pozostaje przezroczysta, aż odpalisz powtórkę. Uzbrój bufor powtórek (Sterowanie) i przypisz klawisz Roll — roll wycina ostatnie chwile bufora, odtwarza je w programie i wraca do przezroczystości.
sources-replay-add = Dodaj powtórkę
properties-replay-roll = ⏵ Odpal powtórkę
properties-replay-note = Roll wycina UZBROJONY bufor do klipu i odtwarza go z wybraną prędkością — przetaktowane, nigdy interpolowane. Zwolnione tempo jest celowo nieme. Przewijanie i pauza działają w trakcie; na końcu źródło wraca do przezroczystości.
hotkeys-replay-roll = Powtórka: odpal
hotkey-audit-action-replay-roll = Odpal powtórkę

# Input overlay source (CAP-N13)
sources-badge-input = Wejście
sources-add-input-overlay = Nakładka wejścia (klawisze/pad)
sources-input-title = Dodaj nakładkę wejścia
sources-input-layout-label = Układ
sources-input-layout-wasd = WASD + mysz
sources-input-layout-keyboard = Kompaktowa klawiatura + mysz
sources-input-layout-gamepad = Pad (dwie gałki)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Klawisze
sources-input-accent-label = Wciśnięte
sources-input-privacy-note = Prywatność: wejście jest odczytywane tylko wtedy, gdy to źródło jest na żywo w scenie, i odpytywane są wyłącznie stałe klawisze układu — chwilowy odczyt „czy jest teraz wciśnięty?”, nigdy hook. Nic nie jest logowane, zapisywane ani nigdzie wysyłane; wpisywany tekst nigdy nie jest przechwytywany.
sources-input-os-note = Stan klawiatury i myszy jest dziś odczytywany tylko w systemie Windows — inne systemy rysują klawisze niewciśnięte (powiedziane uczciwie, nigdy udawane). Pady działają wszędzie dzięki bibliotece gilrs; rysowany jest pierwszy podłączony kontroler, a bez kontrolera układ pozostaje niewciśnięty.
sources-input-add = Dodaj nakładkę wejścia

# Cursor highlight & click effects (CAP-N19)
filters-cursorfx-header = Efekty kursora
filters-cursorfx-hint = W systemie Windows (który sam rysuje kursor) są malowane bezpośrednio w przechwytywanym obrazie, więc widać je w nagraniach i transmisjach. macOS i Linux składają kursor po stronie systemu, więc te efekty działają tylko w Windows. Zmiany obowiązują od razu.
filters-cursorfx-halo = Poświata kursora
filters-cursorfx-halo-color = Kolor
filters-cursorfx-halo-radius = Promień (px)
filters-cursorfx-ripples = Fale kliknięć
filters-cursorfx-left-color = Lewy przycisk
filters-cursorfx-right-color = Prawy przycisk
filters-cursorfx-keystrokes = Podgląd klawiszy
filters-cursorfx-keystrokes-hint = Pokazuje stały zestaw klawiszy (litery, cyfry, modyfikatory, strzałki) obok kursora, dopóki są wciśnięte. Klawisze są odczytywane tylko przy włączonej opcji, rysowane prosto w klatce i nigdy nie są zapisywane ani logowane.

# Title & scoreboard designer source (CAP-N16)
sources-badge-title = Tytuł
sources-add-title = Tytuł / Tablica wyników
sources-title-title = Dodaj tytuł
sources-title-template-label = Zacznij od
sources-title-template-lower-third = Belka dolna (pasek + imię + podpis)
sources-title-template-scoreboard = Tablica wyników (płyta + 4 pola)
sources-title-template-blank = Puste płótno
sources-title-width-label = Szerokość płótna
sources-title-height-label = Wysokość płótna
sources-title-template-name = Imię
sources-title-template-subtitle = Tytuł
sources-title-template-home = GOSPODARZE
sources-title-template-away = GOŚCIE
sources-title-note = Tytuły warstwowe (tekst / obraz / pole) z animacją wejścia/wyjścia, składane lokalnie — bez źródła przeglądarki. Warstwy, powiązania z plikami i {"{{"}zmiennymi{"}}"} oraz sterowanie na żywo są we Właściwościach źródła.
sources-title-add = Dodaj tytuł
properties-title-layers = Warstwy (rysowane po kolei — późniejsze wiersze na wierzchu)
properties-title-kind-text = Tekst
properties-title-kind-image = Obraz
properties-title-kind-rect = Pole
properties-title-x = X
properties-title-y = Y
properties-title-outline = Obrys (px)
properties-title-outline-color = Obrys
properties-title-shadow = Cień
properties-title-animation = Animacja wejścia/wyjścia
properties-title-anim-none = Brak (cięcie)
properties-title-anim-fade = Przenikanie
properties-title-anim-slide-left = Wsuń w lewo
properties-title-anim-slide-up = Wsuń w górę
properties-title-anim-wipe = Roleta
properties-title-duration = Czas (ms)
properties-title-fire-in = ▶ Odpal wejście
properties-title-fire-out = ◼ Odpal wyjście
properties-title-set-live = Ustaw na żywo
properties-title-set-live-note = Natychmiast wypycha ten tekst do tytułu NA ŻYWO — bez Zastosuj, bez restartu
properties-title-up = Warstwa wyżej
properties-title-down = Warstwa niżej
properties-title-remove = Usuń warstwę
properties-title-add-text = + Tekst
properties-title-add-image = + Obraz
properties-title-add-rect = + Pole
properties-title-note = Wejście/wyjście i „Ustaw na żywo" sterują DZIAŁAJĄCYM tytułem; zmiany warstw obowiązują po Zastosuj (tytuł startuje od nowa i znów wjeżdża). Pola tekstowe mogą wiązać się z obserwowanym plikiem (komórka CSV / wartość JSON / cały plik) i interpolować {"{{"}zmienne{"}}"} — „Ustaw na żywo" wygrywa z oboma.

# LAN ingest source (CAP-N11)
sources-badge-lan-ingest = LAN
sources-add-lan-ingest = Ingest LAN (nasłuch SRT/RTMP)
sources-lan-title = Dodaj nasłuch ingestu LAN
sources-lan-protocol-label = Protokół
sources-lan-protocol-srt = SRT (możliwe szyfrowanie — zalecane)
sources-lan-protocol-rtmp = RTMP (brak uwierzytelniania)
sources-lan-port-label = Port (1024–65535)
sources-lan-passphrase-label = Hasło (puste = otwarte)
sources-lan-passphrase-hint = Hasła SRT mają 10–79 znaków; nadawca musi użyć tego samego.
sources-lan-open-warning = Brak hasła: każdy w tej sieci może zasilać to źródło, bez szyfrowania. Ustaw hasło, chyba że sieć należy tylko do ciebie.
sources-lan-rtmp-warning = RTMP nie ma uwierzytelniania — każdy w tej sieci może wysyłać na ten port. Wybierz raczej SRT z hasłem.
sources-lan-url-label = Skieruj aplikację nadawcy na
sources-lan-qr-aria = Kod QR adresu ingestu
sources-lan-note = Tylko LAN: nasłuchuje na lokalnym adresie tej maszyny, tylko dopóki źródło istnieje, i nigdy nie dotyka internetu — nic nie opuszcza maszyny, dopóki nadawca w twojej sieci nie wyśle pierwszy. Dekodowanie działa przez wyraźnie oznaczony komponent ffmpeg. Płótno pokazuje ten adres, dopóki nadawca się nie połączy.
sources-lan-add = Rozpocznij nasłuch
properties-lan-note = Zastosowanie zmiany protokołu, portu lub hasła restartuje nasłuch — nadawca musi połączyć się ponownie. Strumień jest dopasowywany do płótna 1920×1080.

# Freally Link source & output (CAP-N12)
sources-badge-link = Łącze
sources-add-freally-link = Freally Link (inna instancja)
sources-link-title = Dodaj Freally Link
sources-link-about = Odbiera program innej instancji Freally Capture — wideo i dźwięk master — przez twoją własną sieć. Najpierw włącz „Wyjście Freally Link” na instancji nadającej. v1 przesyła motion-JPEG po TCP: świetnie w przewodowym LAN lub dobrym Wi-Fi, uczciwie wobec pasma na słabych łączach.
sources-link-scan = Skanuj sieć LAN
sources-link-scanning = Skanowanie…
sources-link-none = Nie znaleziono wyjść Freally Link. Włącz „Wyjście Freally Link” na drugiej instancji (Sterowanie → Panel LAN) albo wpisz jej adres poniżej.
sources-link-host = Adres
sources-link-port = Port
sources-link-key = Klucz parowania
sources-link-key-hint = Klucz z ustawień „Wyjście Freally Link” nadawcy — bez niego nadawca nie wyśle ani jednej klatki.
sources-link-add = Dodaj łącze
properties-link-note = Bez połączenia źródło pokazuje planszę „łączenie” i samo ponawia próby z rosnącym odstępem — nigdy nie zastyga na starej klatce. Jeden odbiornik na nadawcę; zajęty nadawca jest grzecznie ponawiany.
link-title = Wyjście Freally Link
link-about = Udostępnij program tej instancji — wideo i dźwięk master — JEDNEJ innej instancji Freally Capture w twojej własnej sieci; pojawi się tam jako źródło „Freally Link” (streaming z dwóch komputerów, monitory pomocnicze). Domyślnie wyłączone; nic się nie ogłasza ani nie nasłuchuje, dopóki nie włączysz. v1 przesyła motion-JPEG + nieskompresowany dźwięk po TCP — dla przewodowego LAN lub dobrego Wi-Fi, nigdy dla internetu.
link-enable = Udostępniaj program w mojej sieci
link-name = Nazwa instancji
link-key = Klucz parowania
link-key-hint = Co najmniej 8 znaków — odbiorniki muszą podać ten klucz, zanim zostanie wysłana choćby jedna klatka.
link-lan-warning = ⚠ Odbiorniki muszą przedstawić klucz parowania, zanim cokolwiek zostanie wysłane, ale sam strumień nie jest w v1 szyfrowany — używaj tylko w zaufanej sieci.
link-serving = Odbiorniki znajdą tę instancję przez „Skanuj sieć LAN” albo dodadzą ją ręcznie pod:
link-off-hint = Włącz udostępnianie, aby otworzyć port i ogłaszać tę instancję skanom LAN.

# In-app menu bar (OBS-style chrome)
menu-bar-label = Menu aplikacji
menu-file = Plik
menu-edit = Edycja
menu-view = Widok
menu-docks = Doki
menu-profile = Profil
menu-collection = Kolekcja scen
menu-tools = Narzędzia
menu-help = Pomoc
menu-rename = Zmień nazwę
menu-remove = Usuń
menu-import = Importuj
menu-export = Eksportuj
menu-file-show-recordings = Pokaż nagrania
menu-file-remux = Remuksuj do MP4…
menu-file-settings = Ustawienia…
menu-file-show-settings-folder = Pokaż folder ustawień
menu-file-exit = Zakończ
menu-edit-undo = Cofnij
menu-edit-redo = Ponów
menu-edit-history = Historia zmian…
menu-edit-copy-transform = Kopiuj transformację
menu-edit-paste-transform = Wklej transformację
menu-edit-copy-filters = Kopiuj filtry
menu-edit-paste-filters = Wklej filtry
menu-edit-transform = Transformacja…
menu-edit-lock-preview = Zablokuj podgląd
menu-view-fullscreen = Interfejs pełnoekranowy
menu-stats-dock = Panel statystyk
menu-view-multiview = Monitor multiview…
menu-view-projectors = Projektory…
menu-view-source-health = Stan źródeł…
menu-view-still = Przechwyć klatkę
menu-docks-browser = Doki przeglądarki…
menu-docks-lock = Zablokuj doki
menu-docks-reset = Zresetuj układ doków
menu-profile-manage = Zarządzaj profilami…
menu-collection-manage = Zarządzaj kolekcjami scen…
menu-collection-import-obs = Importuj z OBS…
menu-collection-missing = Sprawdź brakujące pliki…
menu-tools-wizard = Uruchom kreatora konfiguracji
menu-tools-wizard-title = Kreator konfiguracji działa przy pierwszym uruchomieniu; ponowne uruchomienie nie jest jeszcze możliwe.
menu-tools-automation = Reguły automatyzacji i makra…
menu-tools-rundown = Pokaż scenariusz programu…
menu-tools-hotkeys = Mapa skrótów…
menu-tools-av-sync = Kalibracja synchronizacji A/V…
menu-tools-scripts = Skrypty Lua…
menu-tools-components = Komponenty…
menu-tools-midi = Sterowanie MIDI…
menu-tools-ptz = Kamery PTZ…
menu-tools-remote = API zdalnego sterowania…
menu-tools-panel = Panel LAN i tally…
menu-help-portal = Portal pomocy
menu-help-website = Odwiedź stronę internetową
menu-help-discord = Dołącz do serwera Discord
menu-help-bug = Zgłoś błąd…
menu-help-updates = Sprawdź aktualizacje…
menu-help-whats-new = Co nowego
menu-help-about = O programie…
menu-help-more-apps = Więcej aplikacji Freally…
moreapps-title = Więcej aplikacji Freally

# --- OBS-style Settings modal (obs-chrome): sidebar categories, OK/Cancel/Apply, hotkey pool, meter colors ---
settings-categories = Kategorie ustawień
settings-cat-general = Ogólne
settings-cat-appearance = Wygląd
settings-cat-streaming = Transmisja
settings-cat-output = Wyjście
settings-cat-replay = Powtórki
settings-cat-hotkeys = Skróty klawiszowe
settings-cat-network = Sieć
settings-cat-accessibility = Dostępność
settings-cat-about = Informacje
settings-ok = OK
settings-cancel = Anuluj
settings-apply = Zastosuj
settings-save = Zapisz
settings-loading = Wczytywanie ustawień…
settings-hotkeys-filter = Filtruj skróty
settings-hotkeys-filter-placeholder = Wpisz, aby filtrować akcje lub klawisze…
settings-hotkeys-no-match = Żaden skrót nie pasuje do “{ $query }”.
settings-hotkey-none = Brak
settings-hotkey-group-ctrl = Ctrl + klawisz
settings-hotkey-group-ctrl-shift = Ctrl + Shift + klawisz
settings-hotkey-group-ctrl-alt = Ctrl + Alt + klawisz
settings-hotkey-group-function = Klawisze funkcyjne
settings-hotkey-group-numpad = Klawiatura numeryczna
settings-panic-section = Plansza paniki
settings-meter-section = Wskaźniki poziomu miksera
settings-meter-note = Kolory, przez które przechodzą wskaźniki poziomu miksera dźwięku — od ciszy po przesterowanie. Preset przyjazny dla daltonistów używa gradientu niebieski → pomarańczowy, czytelnego przy zaburzeniach widzenia czerwieni i zieleni.
settings-meter-preset = Kolory wskaźnika
settings-meter-preset-default = Zielony / żółty / czerwony
settings-meter-preset-colorblind = Przyjazny dla daltonistów (niebieski / pomarańczowy)
settings-meter-preset-custom = Niestandardowy
settings-meter-low = Normalny
settings-meter-mid = Głośny
settings-meter-high = Przesterowanie
settings-meter-preview = Podgląd

# --- CAP-N: What's New, blur/pixelate/freeze filters, 3D transform, clone, Downstream Keyers ---
whats-new-title = Co nowego
whats-new-loading = Ładowanie informacji o wersji…
whats-new-version = Co nowego w wersji { $version }
whats-new-empty = Brak informacji o tej wersji.
filters-name-directional-blur = Rozmycie kierunkowe
filters-name-radial-blur = Rozmycie promieniste
filters-name-zoom-blur = Rozmycie zoomu
filters-name-pixelate = Pikselizacja
filters-angle = Kąt (°)
filters-center-x = Środek X
filters-center-y = Środek Y
filters-block-size = Rozmiar bloku (px)
filters-name-freeze = Zamroź
filters-freeze-hint = Gdy włączone, to źródło zatrzymuje ostatnią klatkę — program, podgląd, nagrywanie i transmisja zamrażają się razem. Przełącz ten filtr, aby zamrozić lub odmrozić.
transform-3d = Pochylenie 3D
transform-rotation-x = Pochylenie X (°)
transform-rotation-y = Pochylenie Y (°)
transform-perspective = Perspektywa
transform-reveal = Pokaż/ukryj
transform-reveal-ms = Płynne pojawianie (ms)
sources-clone-title = Klonuj (ten sam sygnał, własne filtry)
sources-clone-item = Klonuj { $name }
menu-tools-downstream = Klucze wyjściowe…
menu-tools-transition-rules = Reguły przejść…
dsk-title = Klucze wyjściowe
dsk-hint = Nakładki komponowane na wyjściu programowym — nad każdą sceną, pozostają na miejscu przy przełączaniu scen (logo, plakietka NA ŻYWO, belka dolna). Góra listy jest rysowana na wierzchu.
dsk-empty = Brak kluczy — dodaj źródło, aby nałożyć je na każdą scenę.
dsk-enable = Włącz ten klucz
dsk-move-up = Przenieś w górę (na wierzch)
dsk-move-down = Przenieś w dół
dsk-remove = Usuń klucz
dsk-opacity = Krycie
dsk-x = X (px)
dsk-y = Y (px)
dsk-scale = Skala
dsk-add = + Dodaj klucz
transition-rules-title = Reguły przejść
transition-rules-hint = Nadaj parze scen własne przejście. Gdy przechodzisz z pierwszej sceny do drugiej, używane są ten rodzaj i czas zamiast domyślnych (reguła Stinger/Obraz nadal używa pliku ustawionego w elementach sterujących przejściem).
transition-rules-empty = Brak reguł — każda para scen używa domyślnego przejścia.
transition-rules-from = Z
transition-rules-to = Do
transition-rules-kind = Przejście
transition-rules-duration = Czas (ms)
transition-rules-add = Dodaj regułę
transition-rules-remove = Usuń regułę
