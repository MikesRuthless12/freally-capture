# Bundled fonts — the Noto Sans complete family

These fonts are compiled into the app (`include_bytes!` in `src/text.rs`) so
the Text source renders **identically on every machine**, with no dependence
on what the OS happens to ship. They are **variable fonts** — each file is the
*complete* family across its weight (100–900) and width (62.5–100%) axes:

| File | Family | Why bundled |
|------|--------|-------------|
| `NotoSans[wdth,wght].ttf` | Noto Sans (upright) | the default face |
| `NotoSans-Italic[wdth,wght].ttf` | Noto Sans Italic | style support |
| `NotoSansArabic[wdth,wght].ttf` | Noto Sans Arabic | RTL script fallback |
| `NotoSansHebrew[wdth,wght].ttf` | Noto Sans Hebrew | RTL script fallback |

CJK is **not** bundled (Noto CJK is ~100 MB) — CJK text uses a system family
until a slimmed CJK subset is decided; the Text source's family picker keeps
every system font available.

## License

SIL Open Font License 1.1 — the verbatim text is vendored as `OFL.txt` and
acknowledged in `THIRD-PARTY-NOTICES.md`. Copyright 2022 The Noto Project
Authors (https://github.com/notofonts/latin-greek-cyrillic).

## Provenance (supply chain)

Fetched 2026-07-02 from the official **google/fonts** repository, pinned to
commit `6bc5aaa80150ffda7799ea091674125880945c0a`
(`https://raw.githubusercontent.com/google/fonts/6bc5aaa80150ffda7799ea091674125880945c0a/ofl/...`).

SHA-256:

```
bfb7bb691513f12e734dc346c03a03f784912432d7e3fa8e56efcf906fe86b3d  NotoSans[wdth,wght].ttf
58e6e0ebd1931b29a365aa2d3e2ee9a9e831a3af7cf3ad1462d4e72154f0b291  NotoSans-Italic[wdth,wght].ttf
63111b5b2e074dd48cc67692e0a2726d86ee94c1c37fe8598257b7b4e87e869e  NotoSansArabic[wdth,wght].ttf
7ef36a2c3593758cdb622e1bdef4f84523e92fbc3ccc667438dd80ff54c2de88  NotoSansHebrew[wdth,wght].ttf
cee9892f9f0cc8fe882c9e9537ee6a89621d86ee7ceaf70b02e2b2b1c25c061a  OFL.txt
```
