<#
.SYNOPSIS
    Build script for LinkSanitizer Android APK and Rust native library.
#>

param(
    [string]$TargetAbi = "arm64-v8a", # e.g. arm64-v8a, armeabi-v7a, x86_64
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " LinkSanitizer Android Build Pipeline   " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# 1. Check Cargo & Rust
$cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargoCmd) {
    Write-Host "`n[1/3] Building Rust Native Library for target: $TargetAbi..." -ForegroundColor Green
    $buildMode = if ($Release) { "--release" } else { "" }
    
    # Check cargo-ndk
    $cargoNdk = Get-Command cargo-ndk -ErrorAction SilentlyContinue
    if (-not $cargoNdk) {
        Write-Host "cargo-ndk not found. Installing via 'cargo install cargo-ndk'..." -ForegroundColor Yellow
        cargo install cargo-ndk
    }

    Write-Host "Running: cargo ndk -t $TargetAbi -o ./android/app/src/main/jniLibs build $buildMode"
    cargo ndk -t $TargetAbi -o ./android/app/src/main/jniLibs build $buildMode
} else {
    Write-Host "`n[!] Rust toolchain (cargo) not detected in PATH." -ForegroundColor Yellow
    Write-Host "    LinkSanitizer will build using the high-performance Kotlin fallback engine." -ForegroundColor Yellow
    Write-Host "    (To enable native Rust compilation, install Rust from https://rustup.rs)" -ForegroundColor Yellow
}

# 2. Build Android APK
Write-Host "`n[2/3] Assembling Android APK with Gradle..." -ForegroundColor Green
Set-Location -Path "./android"

$gradleTask = if ($Release) { "assembleRelease" } else { "assembleDebug" }

if (Test-Path "./gradlew.bat") {
    & .\gradlew.bat $gradleTask
} else {
    gradle $gradleTask
}

Set-Location -Path ".."

Write-Host "`n[3/3] Build complete!" -ForegroundColor Green
Write-Host "Output APK located at: android/app/build/outputs/apk/debug/app-debug.apk" -ForegroundColor Cyan
