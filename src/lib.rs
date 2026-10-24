pub mod job;
pub mod sanitizer;
pub mod service;

use std::sync::Mutex;
use tracing::info;

// Global service instance handle for JNI calls
static SERVICE: Mutex<Option<tokio::sync::mpsc::Sender<String>>> = Mutex::new(None);

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android_jni {
    use super::*;
    use jni::objects::{JClass, JString};
    use jni::sys::{jboolean, jstring};
    use jni::JNIEnv;

    #[no_mangle]
    pub extern "system" fn Java_com_example_linksanitizer_RustBridge_initService(
        mut env: JNIEnv,
        _class: JClass,
        storage_dir: JString,
    ) -> jboolean {
        let storage_path: String = match env.get_string(&storage_dir) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };

        let db_file = format!("{}/link_sanitizer_jobs.db", storage_path);

        // Start background Tokio runtime for Rivium/Apalis
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");

            rt.block_on(async move {
                let service = match service::LinkSanitizerService::init(&db_file).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to init LinkSanitizerService: {e}");
                        return;
                    }
                };

                // Run Apalis worker
                service.run_worker().await;
            });
        });

        1 // Success
    }

    #[no_mangle]
    pub extern "system" fn Java_com_example_linksanitizer_RustBridge_sanitizeNative(
        mut env: JNIEnv,
        _class: JClass,
        raw_url: JString,
    ) -> jstring {
        let input: String = match env.get_string(&raw_url) {
            Ok(s) => s.into(),
            Err(_) => return env.new_string("").unwrap().into_raw(),
        };

        let res = sanitizer::sanitize_text(&input);
        let cleaned = res.sanitized_url;

        env.new_string(cleaned).unwrap().into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_com_example_linksanitizer_RustBridge_sanitizeSync(
        env: JNIEnv,
        class: JClass,
        raw_url: JString,
    ) -> jstring {
        Java_com_example_linksanitizer_RustBridge_sanitizeNative(env, class, raw_url)
    }
}
