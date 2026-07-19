# Third-Party Data & Platform Components — Freally Capture

This document lists third-party **data** and **platform** components used by Freally
Capture beyond the source-code dependencies tracked in `THIRD-PARTY-NOTICES`.
Everything here is used under a permissive, **attribution-only** license (or is public
domain / self-generated), which permits commercial distribution. No copyleft
(GPL/LGPL), ShareAlike (CC-BY-SA), or NonCommercial (CC-BY-NC) data is used.

## Teleprompter autocomplete dictionaries

The teleprompter's word/phrase autocomplete ships one dictionary per app language
(`ui/src/dict/<locale>.json`), lazy-loaded for the active language only.

### Tatoeba Project — CC BY 2.0 FR
Word and phrase frequency lists derived from the [Tatoeba](https://tatoeba.org)
sentence corpora, licensed **CC BY 2.0 FR** (attribution; commercial use permitted).
Used for: Arabic (ar), German (de), Spanish (es), French (fr), Indonesian (id),
Italian (it), Dutch (nl, in part), Portuguese – Brazil (pt-BR), Russian (ru),
Ukrainian (uk), Vietnamese (vi, in part), Chinese – Simplified (zh-CN, in part).

> Contains data from the Tatoeba Project (https://tatoeba.org), licensed under CC BY 2.0 FR.

### English (en) — SCOWL + public domain
Words from **SCOWL 2020.12.07** (http://wordlist.aspell.net/), a permissive
(BSD/MIT-style, attribution-only, sale explicitly permitted) word list that
incorporates the public-domain *Moby Words II* and *UK English Wordlist*. Phrases and
the curated high-frequency head are self-generated.

### Polish (pl) — CC BY 4.0
Word frequencies from the **Leipzig Corpora Collection** / Wortschatz Leipzig
(https://wortschatz.uni-leipzig.de), corpus `pol_news_2020_1M`, licensed **CC BY 4.0**.

> Contains data from the Leipzig Corpora Collection (https://wortschatz.uni-leipzig.de), licensed under CC BY 4.0.

### Dutch (nl) — OpenTaal (BSD-3-Clause / CC BY 3.0)
Additional Dutch words from the **OpenTaal wordlist**
(https://github.com/OpenTaal/opentaal-wordlist), **BSD-3-Clause / CC BY 3.0**,
combined with the Tatoeba data above.

### Chinese – Simplified (zh-CN) — OpenCC (Apache-2.0)
Traditional→Simplified normalization used **OpenCC**'s `TSCharacters` map
(https://github.com/BYVoid/OpenCC), **Apache-2.0**, as a transform only — no OpenCC
data is bundled.

### Self-generated (no third-party license)
Hindi (hi), Japanese (ja), Korean (ko), and Turkish (tr), plus supplements for
Dutch, Vietnamese, and the English phrase list, were generated for this project and
carry no third-party license.

## Read-aloud (text-to-speech)

The teleprompter's **Read aloud** mode uses the operating system's own speech
synthesis — **no third-party TTS engine is bundled**:

- **Windows** — SAPI 5 / OneCore voices via the WebView Web Speech API (`window.speechSynthesis`).
- **macOS** — AVSpeechSynthesis via the WebView Web Speech API (`window.speechSynthesis`).
- **Linux** — the Web Speech API where the WebView provides it; otherwise a native
  fallback that shells out to `spd-say` (speech-dispatcher) or `espeak-ng` **if
  installed on the user's system**. These are invoked as external system tools and
  are not bundled with Freally Capture.
