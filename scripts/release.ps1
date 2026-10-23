# Builds a release of Oxide into dist\:
#   Oxide-<version>-portable.zip   oxide.exe + content\tracks, unzip anywhere and run
#   the NSIS setup .exe and the .msi that `tauri build` makes
# Then attach them to a GitHub release tagged v<version> (version from Cargo.toml).
#
#   .\scripts\release.ps1
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"(.+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
Write-Host "Oxide $version"

$watch = [Diagnostics.Stopwatch]::StartNew()
pnpm tauri build
if ($LASTEXITCODE -ne 0) { throw "tauri build failed" }
Write-Host ("built in {0:n0} s" -f $watch.Elapsed.TotalSeconds)

$dist = Join-Path $root 'dist'
if (Test-Path $dist) { Remove-Item $dist -Recurse -Force }
New-Item -ItemType Directory -Force $dist | Out-Null

# Portable: the exe finds a `content` folder next to itself (see content_dir in src-tauri/src/lib.rs).
$stage = Join-Path $env:TEMP "oxide-portable-$version"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Force (Join-Path $stage 'content') | Out-Null
Copy-Item target\release\oxide.exe $stage
Copy-Item src-tauri\bundle-content\tracks (Join-Path $stage 'content\tracks') -Recurse
$zip = Join-Path $dist "Oxide-$version-portable.zip"
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip
Remove-Item $stage -Recurse -Force

Copy-Item target\release\bundle\nsis\*.exe $dist
Copy-Item target\release\bundle\msi\*.msi $dist
Get-ChildItem $dist | ForEach-Object { "{0}  {1:n1} MB" -f $_.Name, ($_.Length / 1MB) }
Write-Host "Attach these to a GitHub release tagged v$version."
