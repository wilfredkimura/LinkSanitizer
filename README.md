# LinkSanitizer (Rivium + Apalis Pocket Backend)

LinkSanitizer is a privacy utility running **on Android** powered by an embedded Rust engine (using Rivium & Apalis) that cleans tracking metadata, analytics parameters, and surveillance IDs (`utm_*`, `fbclid`, `igshid`, `gclid`, `si`, `tt_*`, Amazon `tag`, etc.) from shared URLs.

---

## Architecture

```
[User shares link from YouTube / Chrome / Twitter / TikTok]
                           │
                           ▼
          [Android Share Sheet Intent (ACTION_SEND)]
                           │
                           ▼
       [ShareActivity.kt / MainActivity.kt (Android UI)]
                           │
                  (JNI Call via RustBridge.kt)
                           ▼
             [Rust Engine: liblink_sanitizer.so]
            ├── sanitizer.rs (URL & Text Scanner)
            ├── job.rs (Apalis Job Handler)
            └── service.rs (Rivium + Apalis SQLite Persistence)
                           │
                           ▼
           [Clean Sanitized URL Output]
          ├── Automatically Copied to Clipboard
          └── Notification with 1-Tap "Share" Action
```

* **Rust Engine (`src/`):**
  * `sanitizer.rs`: High-performance URL scanner and query stripper. Handles standalone URLs and mixed text containing URLs.
  * `job.rs`: Apalis job payload (`SanitizeLinkJob`) and worker handler.
  * `service.rs`: Rivium service wrapper managing SQLite persistence and background workers.
  * `lib.rs`: JNI bindings (`Java_com_example_linksanitizer_RustBridge_*`).
  * `main.rs`: Desktop CLI runner for rapid local testing without a phone.
* **Android Client (`android/`):**
  * `RustBridge.kt`: Loads `liblink_sanitizer.so`, manages JNI calls, and includes a fallback engine.
  * `ShareActivity.kt`: Captures Android Share Sheet intents silently without UI flicker.
  * `NotificationHelper.kt`: Posts actionable notifications and copies clean URLs to clipboard.
  * `MainActivity.kt`: Interactive dashboard to test links, check engine status, and manage permissions.

---

## Running on Desktop (CLI Runner)

You can test the core sanitization and queue logic on your computer without Android:

```bash
cargo run --bin link_sanitizer_cli
```

---

## Building the Android APK

You can build the APK using PowerShell or command line without needing to open Android Studio:

### 1. Compile Rust Native Library for Android (via `cargo ndk`)
```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
cargo ndk -t arm64-v8a -o ./android/app/src/main/jniLibs build --release
```

### 2. Assemble Debug APK with Gradle
```powershell
cd android
./gradlew assembleDebug
```

The resulting APK will be at:
`android/app/build/outputs/apk/debug/app-debug.apk`

---

## Installing and Running the APK

### On a Real Android Device:
1. Enable **Developer Options** and **USB Debugging** on your phone.
2. Plug your phone into your computer via USB.
3. Install via ADB:
   ```powershell
   adb install android/app/build/outputs/apk/debug/app-debug.apk
   ```
*(Alternatively, copy `app-debug.apk` to your phone's storage and tap it to install).*

### On PC via Android Emulator:
Drag and drop `app-debug.apk` directly into any running Android emulator (BlueStacks, LDPlayer, or Android SDK emulator).
