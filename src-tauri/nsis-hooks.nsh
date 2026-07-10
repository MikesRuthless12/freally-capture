; Freally Capture — NSIS installer hooks.
;
; Give `.frec` recordings their own Explorer icon.
;
; Tauri cannot do this from `tauri.conf.json`: its `fileAssociations` schema has
; no `icon` field (only ext / name / description / role / mimeType / …), and its
; NSIS template hardcodes the association's DefaultIcon to the app executable:
;
;   !insertmacro APP_ASSOCIATE "frec" "Freally Recording" "…" \
;       "$INSTDIR\${MAINBINARYNAME}.exe,0"   ; <- the app icon, not frec.ico
;
; So every `.frec` would show the studio's own icon, while `icons/frec.ico` sat
; unused in the install directory (it ships via `bundle.resources`).
;
; `APP_ASSOCIATE` writes the ProgId key `Software\Classes\<name>\DefaultIcon`,
; where `<name>` is the association's `name` — "Freally Recording". This hook is
; inserted *after* that block, so re-writing the value here wins.
;
; SHELL_CONTEXT (`SHCTX`) is whichever hive the installer chose (per-user or
; per-machine), so the write lands beside the association it is correcting.

!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Registering the .frec file icon"
  WriteRegStr SHCTX "Software\Classes\Freally Recording\DefaultIcon" "" "$INSTDIR\icons\frec.ico"

  ; SHCNE_ASSOCCHANGED (0x08000000) — tell Explorer the association changed so it
  ; repaints existing .frec files immediately instead of after a sign-out.
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)'
!macroend

; Nothing to undo: `APP_UNASSOCIATE` removes the whole `Freally Recording` ProgId
; key, and `DefaultIcon` lives under it.
