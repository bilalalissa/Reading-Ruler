param(
  [ValidateSet("nsis", "msi")]
  [string]$Bundle = "nsis"
)

$ErrorActionPreference = "Stop"

if ([System.Environment]::OSVersion.Platform -ne [System.PlatformID]::Win32NT) {
  throw "Windows local packaging must run on Windows."
}

npx tauri build --bundles $Bundle --no-sign

$bundleRoot = Join-Path "src-tauri" "target\release\bundle"
$installers = Get-ChildItem -Path $bundleRoot -Recurse -File |
  Where-Object { $_.Extension -in ".exe", ".msi" } |
  Sort-Object FullName

if (-not $installers) {
  throw "No Windows installer was produced under $bundleRoot."
}

$version = node -p "require('./package.json').version"
$checksumPath = Join-Path $bundleRoot "Reading Ruler_${version}_windows_$Bundle.sha256"
$checksums = foreach ($installer in $installers) {
  $hash = Get-FileHash -Algorithm SHA256 -Path $installer.FullName
  "$($hash.Hash.ToLowerInvariant())  $($installer.FullName)"
}
$checksums | Set-Content -Path $checksumPath -Encoding ASCII

Write-Host "Windows local artifacts:"
$installers | ForEach-Object { Write-Host "  $($_.FullName)" }
Write-Host "  $checksumPath"
