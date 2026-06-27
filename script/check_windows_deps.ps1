param(
  [switch]$Install
)

$ErrorActionPreference = "Stop"

if ([System.Environment]::OSVersion.Platform -ne [System.PlatformID]::Win32NT) {
  throw "This script must run on Windows."
}

$missing = New-Object System.Collections.Generic.List[string]

function Test-Command {
  param([string]$Name)
  return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Add-Missing {
  param([string]$Name)
  if (-not $missing.Contains($Name)) {
    $missing.Add($Name)
  }
}

function Test-VisualStudioBuildTools {
  $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
  if (-not (Test-Path $vswhere)) {
    return $false
  }

  $install = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
  return -not [string]::IsNullOrWhiteSpace($install)
}

function Test-WebView2Runtime {
  $paths = @(
    "HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
  )

  foreach ($path in $paths) {
    if (Test-Path $path) {
      return $true
    }
  }

  return $false
}

Write-Host "Checking Windows dependencies..."

if ((Test-Command "node") -and (Test-Command "npm")) {
  Write-Host "ok: Node.js $(node --version), npm $(npm --version)"
} else {
  Write-Host "missing: Node.js/npm"
  Add-Missing "node"
}

if ((Test-Command "rustc") -and (Test-Command "cargo") -and (Test-Command "rustup")) {
  Write-Host "ok: rustc $(rustc --version), cargo $(cargo --version)"
} else {
  Write-Host "missing: Rust/Cargo/rustup"
  Add-Missing "rust"
}

if (Test-VisualStudioBuildTools) {
  Write-Host "ok: Visual Studio Build Tools with MSVC"
} else {
  Write-Host "missing: Visual Studio Build Tools with MSVC"
  Add-Missing "vs-build-tools"
}

if (Test-WebView2Runtime) {
  Write-Host "ok: Microsoft WebView2 Runtime"
} else {
  Write-Host "missing: Microsoft WebView2 Runtime"
  Add-Missing "webview2"
}

if ($missing.Count -eq 0) {
  Write-Host "All Windows dependencies are installed."
  exit 0
}

Write-Host ""
Write-Host "Missing dependencies: $($missing -join ', ')"

if (-not $Install) {
  Write-Host "To try automatic install with winget, run:"
  Write-Host "  powershell -ExecutionPolicy Bypass -File script/check_windows_deps.ps1 -Install"
  exit 1
}

if (-not (Test-Command "winget")) {
  Write-Host "winget is not installed. Install dependencies manually:"
  Write-Host "  Node.js: https://nodejs.org/"
  Write-Host "  Rust: https://rustup.rs/"
  Write-Host "  Visual Studio Build Tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
  Write-Host "  WebView2 Runtime: https://developer.microsoft.com/microsoft-edge/webview2/"
  exit 1
}

foreach ($item in $missing) {
  switch ($item) {
    "node" {
      winget install --id OpenJS.NodeJS.LTS -e
    }
    "rust" {
      winget install --id Rustlang.Rustup -e
    }
    "vs-build-tools" {
      winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
    }
    "webview2" {
      winget install --id Microsoft.EdgeWebView2Runtime -e
    }
  }
}

Write-Host ""
Write-Host "Installers finished or were started. Restart PowerShell, then rerun:"
Write-Host "  powershell -ExecutionPolicy Bypass -File script/check_windows_deps.ps1"
