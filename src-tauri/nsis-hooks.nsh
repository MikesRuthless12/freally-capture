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

  ; Desktop shortcut. Tauri's NSIS template creates only the Start-Menu entry
  ; and its config schema has no desktop-shortcut option (verified against the
  ; 0.300.0 setup on Windows 11: no Desktop icon after install), so it lives
  ; here. No icon args: the shortcut takes the target exe's own icon. $DESKTOP
  ; follows the installer's per-user/per-machine context, same as the Start
  ; Menu entry. Updates re-create it by design — the installer's contract is
  ; "installed apps have their icon", stated here once.
  DetailPrint "Creating the Desktop shortcut"
  CreateShortcut "$DESKTOP\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"
!macroend

; `APP_UNASSOCIATE` removes the whole `Freally Recording` ProgId key (and the
; `DefaultIcon` under it) — only the Desktop shortcut needs our own cleanup.
!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$DESKTOP\${PRODUCTNAME}.lnk"
!macroend
