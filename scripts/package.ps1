# Build a portable LAN Share folder for Windows.
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$Out = Join-Path $Root "dist\lan-share"

Write-Host "==> Building release binary..."
Set-Location $Root
cargo build --release

Write-Host "==> Packaging to $Out"
if (Test-Path $Out) { Remove-Item -Recurse -Force $Out }
New-Item -ItemType Directory -Force -Path $Out, "$Out\static", "$Out\uploads" | Out-Null

Copy-Item "$Root\target\release\lan-share.exe" $Out
Copy-Item -Recurse "$Root\static\*" "$Out\static"

@"
@echo off
cd /d "%~dp0"
echo Starting LAN Share...
echo Open http://localhost:5000 on this PC.
lan-share.exe
pause
"@ | Set-Content -Path "$Out\run.bat" -Encoding ASCII

@"
LAN Share - portable package
============================

1. Connect this PC to Wi-Fi.
2. Double-click run.bat (or lan-share.exe in this folder).
3. Open http://localhost:5000 on this PC.
4. On phones: use http://192.168.x.x:5000 from the terminal (NOT localhost).

Allow Windows Firewall for port 5000 if prompted.
"@ | Set-Content -Path "$Out\README.txt" -Encoding UTF8

Write-Host ""
Write-Host "Done. Portable app: $Out"
Write-Host "Zip the lan-share folder and share it."
