param(
  [string]$Repo = $(if ($env:LOONGCLAW_REPO) { $env:LOONGCLAW_REPO } else { 'loongclaw/loongclaw' }),
  [string]$InstallDir = $(if ($env:LOONGCLAW_INSTALL_DIR) { $env:LOONGCLAW_INSTALL_DIR } else { Join-Path $env:USERPROFILE '.local\bin' })
)

$ErrorActionPreference = 'Stop'
$BinName = 'loongclaw.exe'
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"

function Write-Info([string]$msg) {
  Write-Host $msg
}

function Resolve-Arch {
  switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    'X64' { return 'x86_64' }
    'Arm64' { return 'aarch64' }
    default { throw "Unsupported architecture: $([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture)" }
  }
}

function Select-AssetUrl([object]$release, [string]$arch) {
  $patterns = @(
    "loongclaw-v?[0-9]+\.[0-9]+\.[0-9]+-$arch-pc-windows-msvc\.zip$",
    "loongclaw-v?[0-9]+\.[0-9]+\.[0-9]+-.*$arch.*windows.*\.zip$"
  )

  foreach ($p in $patterns) {
    $match = $release.assets | Where-Object { $_.browser_download_url -match $p } | Select-Object -First 1
    if ($null -ne $match) {
      return $match.browser_download_url
    }
  }

  return $null
}

function Path-Contains([string]$pathValue, [string]$dir) {
  if ([string]::IsNullOrWhiteSpace($pathValue)) { return $false }
  $needle = $dir.Trim().TrimEnd('\\').ToLowerInvariant()
  foreach ($part in $pathValue.Split(';')) {
    if ([string]::IsNullOrWhiteSpace($part)) { continue }
    if ($part.Trim().TrimEnd('\\').ToLowerInvariant() -eq $needle) {
      return $true
    }
  }
  return $false
}

function Ensure-UserPathContains([string]$dir) {
  $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
  if (Path-Contains $userPath $dir) {
    return $false
  }

  $newPath = if ([string]::IsNullOrWhiteSpace($userPath)) {
    $dir
  } else {
    "$userPath;$dir"
  }

  [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')

  # Also update current process PATH so this shell can find it immediately.
  if (-not (Path-Contains $env:Path $dir)) {
    $env:Path = "$env:Path;$dir"
  }

  return $true
}

$arch = Resolve-Arch
Write-Info "Installing loongclaw for windows/$arch..."

$release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ 'User-Agent' = 'loongclaw-install-script' }
$assetUrl = Select-AssetUrl -release $release -arch $arch
if (-not $assetUrl) {
  throw "No prebuilt binary found for windows/$arch in the latest GitHub release."
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$tmpDir = New-Item -ItemType Directory -Force -Path (Join-Path ([System.IO.Path]::GetTempPath()) ("loongclaw-install-" + [guid]::NewGuid().ToString()))
try {
  $archivePath = Join-Path $tmpDir.FullName 'loongclaw.zip'
  Write-Info "Downloading: $assetUrl"
  Invoke-WebRequest -Uri $assetUrl -OutFile $archivePath

  Expand-Archive -Path $archivePath -DestinationPath $tmpDir.FullName -Force
  $bin = Get-ChildItem -Path $tmpDir.FullName -Filter $BinName -Recurse | Select-Object -First 1
  if (-not $bin) {
    throw "Could not find $BinName in archive"
  }

  $targetPath = Join-Path $InstallDir $BinName
  Copy-Item -Path $bin.FullName -Destination $targetPath -Force

  $pathUpdated = Ensure-UserPathContains $InstallDir

  Write-Info "Installed loongclaw to: $targetPath"
  if ($pathUpdated) {
    Write-Info "Added '$InstallDir' to your user PATH."
    Write-Info "Open a new terminal if command lookup does not refresh immediately."
  } else {
    Write-Info "PATH already contains '$InstallDir'."
  }

  Write-Info "loongclaw"
  if (Get-Command loongclaw -ErrorAction SilentlyContinue) {
    Write-Info "Running: loongclaw"
    try {
      & loongclaw
    } catch {
      Write-Info "Auto-run failed. Try running: loongclaw"
    }
  } else {
    Write-Info "Could not find 'loongclaw' in PATH."
    Write-Info "Add this directory to PATH: $InstallDir"
    Write-Info "Then run: $targetPath"
  }

  # Check for browser executable
  $browserFound = $false
  $browserPaths = @(
    "C:\Program Files\Google\Chrome\chrome.exe",
    "chrome.exe"
  )
  
  foreach ($path in $browserPaths) {
    if (Get-Command $path -ErrorAction SilentlyContinue) {
      $browserFound = $true
      break
    }
  }
  
  if (-not $browserFound) {
    Write-Info "Optional: browser automation requires Chromium or Google Chrome."
    Write-Info "Please install a browser and ensure it's in PATH or configure browser_executable_path in config."
  }
} finally {
  Remove-Item -Recurse -Force $tmpDir.FullName -ErrorAction SilentlyContinue
}
