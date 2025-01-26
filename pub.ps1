param(
    [string]$version
)

# Check if the argument is provided
if (-not $version) {
    Write-Host "Please provide a version argument, e.g., .\pub.ps1 '3.0.0_1'"
    exit 1
}

# Run cargo build commands
Write-Host "Building game-server..."
cargo build --release --bin game-server
Write-Host "Building sdk-server..."
cargo build --release --bin sdk-server
Write-Host "Building cfg-manager..."
cargo build --release --bin cfg-manager

# Define the target directory path
$targetDir = ".\target\release"

# Check if the target directory exists
if (-not (Test-Path $targetDir)) {
    Write-Host "Target directory not found. Make sure the build completed successfully."
    exit 1
}

# Find the .exe files in the release directory
$gameServerExe = Join-Path $targetDir "game-server.exe"
$sdkServerExe = Join-Path $targetDir "sdk-server.exe"
$cfgManagerExe = Join-Path $targetDir "cfg-manager.exe"

# Check if the required .exe files exist
if (-not (Test-Path $gameServerExe)) {
    Write-Host "game-server.exe not found."
    exit 1
}
if (-not (Test-Path $sdkServerExe)) {
    Write-Host "sdk-server.exe not found."
    exit 1
}
if (-not (Test-Path $cfgManagerExe)) {
    Write-Host "cfg-manager.exe not found."
    exit 1
}

# Define the release directory
$releaseDir = ".\release"

# Check if the release directory exists
if (-not (Test-Path $releaseDir)) {
    Write-Host "Release directory not found. Creating it..."
    New-Item -Path $releaseDir -ItemType Directory
}

# Create the new directory based on the version argument
$newDir = Join-Path $releaseDir "$version-Windows-X86_64"
if (-not (Test-Path $newDir)) {
    Write-Host "Creating directory $newDir..."
    New-Item -Path $newDir -ItemType Directory
}

# Copy the .exe files to the new directory
Write-Host "Copying .exe files..."
Copy-Item $gameServerExe -Destination $newDir
Copy-Item $sdkServerExe -Destination $newDir
Copy-Item $cfgManagerExe -Destination $newDir

# Copy the LICENSE file and _cfg directory
$licenseFile = ".\LICENSE"
$cfgDir = ".\_cfg"

# Check if LICENSE file exists
if (Test-Path $licenseFile) {
    Write-Host "Copying LICENSE file..."
    Copy-Item $licenseFile -Destination $newDir
} else {
    Write-Host "LICENSE file not found."
}

# Check if _cfg directory exists
if (Test-Path $cfgDir) {
    Write-Host "Copying _cfg directory..."
    Copy-Item $cfgDir -Destination $newDir -Recurse
} else {
    Write-Host "_cfg directory not found."
}

Write-Host "All files have been copied to $newDir."
