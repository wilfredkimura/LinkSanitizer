# LinkSanitizer (Rivium + Apalis Pocket Backend)

LinkSanitizer is a pocket background service running **inside Android** (via Rivium & Apalis) that cleans tracking metadata (such as `utm_*`, `fbclid`, `igshid`, `gclid`, and `si`) from shared URLs and links.

## Architecture

* **Rust Backend (`src/`):**
  * `sanitizer.rs`: URL parser and query parameter stripper.
  * `job.rs`: Apalis job payload (`SanitizeLinkJob`) and worker handler.
  * `service.rs`: Rivium service wrapper that sets up SQLite storage and persistent worker.
  * `lib.rs`: Exposes JNI native functions (`initService`, `sanitizeSync`) for Android.
  * `main.rs`: Standalone desktop CLI runner to test locally with `cargo run`.
* **Android Client (`android/`):**
  * `RustBridge.kt`: Loads `liblink_sanitizer.so` and bridges calls via JNI.
  * `ShareActivity.kt`: Captures Android **Share Sheet** intents when a user shares a link from Chrome, YouTube, X/Twitter, etc.
  * `NotificationHelper.kt`: Posts a notification with the cleaned URL and copies it to the clipboard.
  * `MainActivity.kt`: Interactive dashboard to test sanitization directly in the app.

## Running on Laptop (Desktop CLI)

No phone needed for testing the core logic:

```bash
cargo run --bin link_sanitizer_cli
```

This will initialize the local SQLite database at `target/link_sanitizer.db`, enqueue test URLs, and run the Apalis worker.

## Building for Android

1. Install the target architecture (e.g. `aarch64-linux-android`):
   ```bash
   rustup target add aarch64-linux-android
   ```
2. Build with `cargo-ndk`:
   ```bash
   cargo ndk -t arm64-v8a -o ./android/app/src/main/jniLibs build --release
   ```
3. Open the `android/` directory in Android Studio and run on an emulator or real device.
