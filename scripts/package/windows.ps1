# PSOC Windows Packaging Script
# This script creates Windows installers and portable packages

param(
    [string]$Version = "0.8.6",
    [string]$Configuration = "release",
    [switch]$SkipBuild = $false,
    [switch]$CreateMSI = $true,
    [switch]$CreatePortable = $true
)

$ErrorActionPreference = "Stop"

# Configuration
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$BuildDir = Join-Path $ProjectRoot "target\release"
$PackageDir = Join-Path $ProjectRoot "packages\windows"
$ResourcesDir = Join-Path $ProjectRoot "resources"

Write-Host "PSOC Windows Packaging Script" -ForegroundColor Green
Write-Host "Version: $Version" -ForegroundColor Yellow
Write-Host "Configuration: $Configuration" -ForegroundColor Yellow

# Create package directory
if (!(Test-Path $PackageDir)) {
    New-Item -ItemType Directory -Path $PackageDir -Force | Out-Null
}

# Build the application
if (!$SkipBuild) {
    Write-Host "Building PSOC for Windows..." -ForegroundColor Blue
    Set-Location $ProjectRoot
    
    # Install required dependencies
    if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
        Write-Error "Cargo not found. Please install Rust toolchain."
    }
    
    # Build release version
    cargo build --release --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed with exit code $LASTEXITCODE"
    }
    
    Write-Host "Build completed successfully!" -ForegroundColor Green
}

# Verify executable exists
$ExePath = Join-Path $BuildDir "psoc.exe"
if (!(Test-Path $ExePath)) {
    Write-Error "Executable not found at $ExePath"
}

# Create portable package
if ($CreatePortable) {
    Write-Host "Creating portable package..." -ForegroundColor Blue
    
    $PortableDir = Join-Path $PackageDir "psoc-$Version-windows-portable"
    if (Test-Path $PortableDir) {
        Remove-Item $PortableDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $PortableDir -Force | Out-Null
    
    # Copy executable
    Copy-Item $ExePath $PortableDir
    
    # Copy resources
    if (Test-Path $ResourcesDir) {
        Copy-Item $ResourcesDir $PortableDir -Recurse
    }
    
    # Copy documentation
    $DocsToInclude = @("README.md", "LICENSE-MIT", "LICENSE-APACHE", "CHANGELOG.md")
    foreach ($doc in $DocsToInclude) {
        $docPath = Join-Path $ProjectRoot $doc
        if (Test-Path $docPath) {
            Copy-Item $docPath $PortableDir
        }
    }
    
    # Create portable marker file
    "This is a portable installation of PSOC Image Editor." | Out-File -FilePath (Join-Path $PortableDir "PORTABLE.txt")
    
    # Create ZIP archive
    $ZipPath = Join-Path $PackageDir "psoc-$Version-windows-portable.zip"
    if (Test-Path $ZipPath) {
        Remove-Item $ZipPath -Force
    }
    
    Compress-Archive -Path "$PortableDir\*" -DestinationPath $ZipPath
    Write-Host "Portable package created: $ZipPath" -ForegroundColor Green
}

# Create MSI installer
if ($CreateMSI) {
    Write-Host "Creating MSI installer..." -ForegroundColor Blue
    
    # Check if cargo-wix is installed
    if (!(Get-Command "cargo-wix" -ErrorAction SilentlyContinue)) {
        Write-Host "Installing cargo-wix..." -ForegroundColor Yellow
        cargo install cargo-wix
    }
    
    # Create WiX configuration if it doesn't exist
    $WixConfigPath = Join-Path $ProjectRoot "wix\main.wxs"
    if (!(Test-Path $WixConfigPath)) {
        Write-Host "Creating WiX configuration..." -ForegroundColor Yellow
        New-Item -ItemType Directory -Path (Split-Path $WixConfigPath) -Force | Out-Null
        
        # Generate basic WiX configuration
        Set-Location $ProjectRoot
        cargo wix init
    }
    
    # Build MSI
    Set-Location $ProjectRoot
    cargo wix --no-build --nocapture
    
    # Move MSI to package directory
    $MsiSource = Join-Path $ProjectRoot "target\wix\psoc-$Version-x86_64.msi"
    $MsiDest = Join-Path $PackageDir "psoc-$Version-windows-installer.msi"
    
    if (Test-Path $MsiSource) {
        Move-Item $MsiSource $MsiDest -Force
        Write-Host "MSI installer created: $MsiDest" -ForegroundColor Green
    } else {
        Write-Warning "MSI installer not found at expected location"
    }
}

Write-Host "Windows packaging completed!" -ForegroundColor Green
Write-Host "Packages created in: $PackageDir" -ForegroundColor Yellow
