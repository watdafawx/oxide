# Builds a release of Oxide into dist\:
#   Oxide-<version>-portable.zip   oxide.exe + content\tracks, unzip anywhere and run
#   the NSIS setup .exe and the .msi that `tauri build` makes
# Then attach them to a GitHub release tagged v<version> (version from Cargo.toml).
#
#   .\scripts\release.ps1             # everything (about 6.5 min; the MSI and setup.exe take about 4 of it)
#   .\scripts\release.ps1 -Portable   # only the portable zip, no installers (about 2.5 min)
#   .\scripts\release.ps1 -Fast       # a quick portable test build (profile.fast, about 2 min); not for publishing
param([switch]$Portable, [switch]$Fast)
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"(.+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
$profile = if ($Fast) { 'fast' } else { 'release' }
$installers = -not ($Portable -or $Fast)
Write-Host "Oxide $version ($profile$(if (-not $installers) { ', portable only' }))"

$tauriArgs = @('tauri', 'build')
if (-not $installers) { $tauriArgs += '--no-bundle' }
if ($Fast) { $tauriArgs += @('--', '--profile', 'fast') }
$watch = [Diagnostics.Stopwatch]::StartNew()
pnpm @tauriArgs
if ($LASTEXITCODE -ne 0) { throw "tauri build failed" }
Write-Host ("built in {0:n0} s" -f $watch.Elapsed.TotalSeconds)

$dist = Join-Path $root 'dist'
if (Test-Path $dist) { Remove-Item $dist -Recurse -Force }
New-Item -ItemType Directory -Force $dist | Out-Null

# Portable: the exe finds a `content` folder next to itself (see content_dir in src-tauri/src/lib.rs).
# build.rs stages the quests as src-tauri\bundle-content\content\tracks, so Windows' tar zips both
# straight from where they are: 3 s, where staging a copy and Compress-Archive took about 2.5 minutes.
$suffix = if ($Fast) { 'portable-test' } else { 'portable' }
$zip = Join-Path $dist "Oxide-$version-$suffix.zip"
& "$env:SystemRoot\System32\tar.exe" -a -cf $zip -C (Join-Path $root "target\$profile") oxide.exe -C (Join-Path $root 'src-tauri\bundle-content') content
if ($LASTEXITCODE -ne 0) { throw "zipping failed" }

if ($installers) {
    Copy-Item target\release\bundle\nsis\*.exe $dist
    Copy-Item target\release\bundle\msi\*.msi $dist
}
Get-ChildItem $dist | ForEach-Object { "{0}  {1:n1} MB" -f $_.Name, ($_.Length / 1MB) }
Write-Host ("done in {0:n0} s" -f $watch.Elapsed.TotalSeconds)
if ($Fast) { Write-Host "Test build only: publish with .\scripts\release.ps1 or -Portable." }
else { Write-Host "Attach these to a GitHub release tagged v$version." }
