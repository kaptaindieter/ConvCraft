<#
ConvCraft - End-User Release Packager (Windows x64)

Creates a self-contained ZIP for end users:
  - convcraft.exe
  - engines folders next to the exe (so the app finds them without fetch scripts)
  - assets (icons)
  - optional VC++ Redistributable + Install.ps1

Usage examples:
  powershell -ExecutionPolicy Bypass -File .\Pack.ps1
  powershell -ExecutionPolicy Bypass -File .\Pack.ps1 -SkipBuild -SkipEngines

Output:
  .\dist\ConvCraft-windows-x64.zip
#>

[CmdletBinding()]
param(
    [switch]$SkipBuild,
    [switch]$SkipEngines,
    [switch]$BuildCadcraft,
    [switch]$NoVCRedist
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = $PSScriptRoot
$distRoot = Join-Path $root 'dist'
$stageRoot = Join-Path $distRoot '_stage'
$appFolderName = 'ConvCraft'
$stage = Join-Path $stageRoot $appFolderName

$releaseDir = Join-Path $root 'target\release'
$exe = Join-Path $releaseDir 'convcraft.exe'

function Ensure-Dir([string]$p) { New-Item -ItemType Directory -Force $p | Out-Null }

function Get-VcRedistPath {
    $cacheDir = Join-Path $distRoot '_cache'
    Ensure-Dir $cacheDir
    $cached = Join-Path $cacheDir 'vc_redist.x64.exe'
    if (Test-Path $cached) { return $cached }
    Write-Host '== Download VC++ Redistributable (x64) ==' -ForegroundColor Cyan
    $url = 'https://aka.ms/vs/17/release/vc_redist.x64.exe'
    Invoke-WebRequest -Uri $url -OutFile $cached -UseBasicParsing
    return $cached
}

function Write-InstallScript([string]$destPath) {
    $lines = @(
        '[CmdletBinding()]',
        'param(',
        "  [string]`$InstallDir = (Join-Path `$env:LOCALAPPDATA 'Programs\\ArtCraft\\ConvCraft')",
        ')',
        '',
        'Set-StrictMode -Version Latest',
        "`$ErrorActionPreference = 'Stop'",
        '',
        'function Test-VcRuntime {',
        "  `$keys = @(",
        "    'HKLM:\\SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64',",
        "    'HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64'",
        '  )',
        '  foreach ($k in $keys) {',
        '    try {',
        '      $p = Get-ItemProperty -Path $k -ErrorAction Stop',
        '      if ($p.Installed -eq 1) { return $true }',
        '    } catch {}',
        '  }',
        '  return $false',
        '}',
        '',
        "Write-Host '== ConvCraft Installer ==' -ForegroundColor Cyan",
        '',
        'if (-not (Test-VcRuntime)) {',
        "  `$redist = Join-Path `$PSScriptRoot 'vc_redist.x64.exe'",
        '  if (Test-Path $redist) {',
        "    Write-Host 'Installing Microsoft Visual C++ Redistributable (2015-2022) ...' -ForegroundColor Yellow",
        "    Start-Process -FilePath `$redist -ArgumentList '/install','/passive','/norestart' -Wait",
        '  } else {',
        "    Write-Host 'WARNING: VC++ runtime not detected and vc_redist.x64.exe not found next to this installer.' -ForegroundColor Yellow",
        "    Write-Host 'Download: https://aka.ms/vs/17/release/vc_redist.x64.exe' -ForegroundColor Yellow",
        '  }',
        '}',
        '',
        'New-Item -ItemType Directory -Force $InstallDir | Out-Null',
        'Write-Host ("Copying files to: {0}" -f $InstallDir) -ForegroundColor Cyan',
        'Copy-Item -LiteralPath (Join-Path $PSScriptRoot ''*'') -Destination $InstallDir -Recurse -Force',
        '',
        "Write-Host ''",
        "Write-Host 'Done. Start ConvCraft via:' -ForegroundColor Green",
        "Write-Host (Join-Path `$InstallDir 'convcraft.exe') -ForegroundColor Green"
    )
    Set-Content -LiteralPath $destPath -Value ($lines -join "`r`n") -Encoding UTF8
}

function Write-Readme([string]$destPath) {
    $lines = @(
        'ConvCraft (Windows) - Quick Install',
        '===================================',
        '',
        '1) Unzip this folder anywhere (e.g. C:\ArtCraft\ConvCraft).',
        '2) Run Install.ps1 (recommended). It installs the VC++ runtime if needed and copies files to LocalAppData.',
        '3) Start ConvCraft via convcraft.exe.',
        '',
        'Troubleshooting:',
        ' - If an engine feature is missing, ensure the engine folders (cadcraft/, pdfcraft/, ...) are next to convcraft.exe.'
    )
    Set-Content -LiteralPath $destPath -Value ($lines -join "`r`n") -Encoding UTF8
}

Write-Host '== ConvCraft Release Pack ==' -ForegroundColor Cyan

Ensure-Dir $distRoot
Ensure-Dir $stageRoot

if (Test-Path $stage) { Remove-Item -LiteralPath $stage -Recurse -Force }
Ensure-Dir $stage

if (-not $SkipBuild) {
    Write-Host '== Build (cargo build --release -p convcraft) ==' -ForegroundColor Cyan
    Push-Location $root
    try {
        cargo build --release --color never -p convcraft
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed (exit $LASTEXITCODE)" }
    } finally { Pop-Location }
}

if (-not (Test-Path $exe)) {
    throw "Missing built exe: $exe (build first or omit -SkipBuild)"
}

Write-Host '== Stage exe ==' -ForegroundColor Cyan
Copy-Item -LiteralPath $exe -Destination (Join-Path $stage 'convcraft.exe') -Force

Write-Host '== Stage assets (optional) ==' -ForegroundColor Cyan
if (Test-Path (Join-Path $root 'assets')) {
    Copy-Item -LiteralPath (Join-Path $root 'assets') -Destination (Join-Path $stage 'assets') -Recurse -Force
}

Write-Host '== Stage docs/licenses (optional) ==' -ForegroundColor Cyan
foreach ($pat in @('LICENSE*','NOTICE*','THIRD_PARTY_NOTICES*','README*')) {
    Get-ChildItem -LiteralPath $root -File -Filter $pat -ErrorAction SilentlyContinue |
        ForEach-Object { Copy-Item -LiteralPath $_.FullName -Destination $stage -Force }
}

if (-not $SkipEngines) {
    Write-Host '== Ensure engines (fetch-engines.ps1) ==' -ForegroundColor Cyan
    $fetch = Join-Path $root 'fetch-engines.ps1'
    if (Test-Path $fetch) {
        # If engines folder is missing or empty, fetch.
        $enginesDir = Join-Path $root 'engines'
        $needFetch = $true
        if (Test-Path $enginesDir) {
            $dirs = Get-ChildItem -LiteralPath $enginesDir -Directory -ErrorAction SilentlyContinue |
                Where-Object { $_.Name -notin @('.cache','.cadcraft-src') }
            if ($dirs.Count -gt 0) { $needFetch = $false }
        }

        if ($needFetch) {
            Write-Host '  fetching engines...' -ForegroundColor Yellow
            if ($BuildCadcraft) {
                & powershell -ExecutionPolicy Bypass -File $fetch -BuildCadcraft
            } else {
                & powershell -ExecutionPolicy Bypass -File $fetch
            }
            if ($LASTEXITCODE -ne 0) { throw "fetch-engines.ps1 failed (exit $LASTEXITCODE)" }
        } else {
            Write-Host '  engines already present.' -ForegroundColor DarkGray
        }
    } else {
        Write-Host 'WARNING: fetch-engines.ps1 not found; engines will not be included.' -ForegroundColor Yellow
    }

    Write-Host '== Stage engines ==' -ForegroundColor Cyan
    $enginesDir = Join-Path $root 'engines'
    if (-not (Test-Path $enginesDir)) {
        Write-Host 'WARNING: No engines directory found; ZIP will not contain engines.' -ForegroundColor Yellow
    } else {
        # Copy each engine folder into the stage root (so it sits next to convcraft.exe).
        Get-ChildItem -LiteralPath $enginesDir -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -notin @('.cache','.cadcraft-src') } |
            ForEach-Object {
                Copy-Item -LiteralPath $_.FullName -Destination $stage -Recurse -Force
            }
    }

    # Also sync engines next to the local exe (so your existing desktop shortcut works).
    if ((Test-Path $exe) -and (Test-Path $enginesDir)) {
        Write-Host '== Sync engines next to local exe ==' -ForegroundColor Cyan
        Get-ChildItem -LiteralPath $enginesDir -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -notin @('.cache','.cadcraft-src') } |
            ForEach-Object {
                Copy-Item -LiteralPath $_.FullName -Destination $releaseDir -Recurse -Force
            }
    }
}

Write-InstallScript -destPath (Join-Path $stage 'Install.ps1')
Write-Readme -destPath (Join-Path $stage 'README.txt')

if (-not $NoVCRedist) {
    $redist = Get-VcRedistPath
    Copy-Item -LiteralPath $redist -Destination (Join-Path $stage 'vc_redist.x64.exe') -Force
}

$zip = Join-Path $distRoot 'ConvCraft-windows-x64.zip'
if (Test-Path $zip) { Remove-Item -LiteralPath $zip -Force }

Write-Host "== Create ZIP: $zip ==" -ForegroundColor Cyan
Compress-Archive -LiteralPath $stage -DestinationPath $zip -Force

Write-Host ''
Write-Host 'DONE.' -ForegroundColor Green
Write-Host "ZIP: $zip" -ForegroundColor Green