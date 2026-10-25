# Lessons Learned

Living log of things that went wrong or surprisingly right, and the rule they produced.
Generic lessons (not tied to this project) are tagged **[KIT]** and get promoted into the
`project-kickstart` plugin during the retrospective.

| Date | Context (task / conversation) | What happened | Rule / fix adopted | Scope |
|---|---|---|---|---|
| 2026-10-25 | M0.4 Audio Endpoint COM Integration | Windows COM interface pointers (e.g. IMMDeviceEnumerator) drop after `CoUninitialize()` if in outer scope, causing STATUS_ACCESS_VIOLATION | Explicitly scope COM objects in an inner block so RAII drops interfaces before `CoUninitialize()`, and free PWSTR via `CoTaskMemFree` | [KIT] |
| 2026-10-25 | Project Setup (Kickstart) | Windows Bluetooth RFCOMM services may be locked or require explicit permissions when opened via WinRT if already connected as audio | Always design transport with a fallback to Winsock `AF_BTH` socket and verify via M0 spike before writing application logic | project |
| 2026-10-25 | Project Setup (Kickstart) | PowerShell 5 defaults to UTF-16/ANSI encoding when redirecting output | Always use dedicated file writing tools or `Set-Content -Encoding utf8` in PowerShell | [KIT] |
| 2026-10-25 | Project Setup (Kickstart) | Windows only permits one process to own a tray icon or single-instance named pipe mutex | Automated tests must use unique identifiers and timeout protection | [KIT] |
