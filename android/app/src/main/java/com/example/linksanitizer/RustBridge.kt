package com.example.linksanitizer

import android.content.Context
import android.util.Log

/**
 * JNI Bridge to the Rust LinkSanitizer backend compiled with Rivium & Apalis.
 */
object RustBridge {
    private const val TAG = "RustBridge"
    private var isInitialized = false

    init {
        try {
            System.loadLibrary("link_sanitizer")
            Log.i(TAG, "Successfully loaded liblink_sanitizer.so")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Failed to load liblink_sanitizer.so. Ensure native libraries are packaged: ${e.message}")
        }
    }

    /**
     * Initializes the background Rivium service & Apalis SQLite database.
     */
    fun init(context: Context) {
        if (!isInitialized) {
            val storagePath = context.filesDir.absolutePath
            val success = initService(storagePath)
            Log.i(TAG, "Rust service init: $success at $storagePath")
            isInitialized = success
        }
    }

    // Native JNI methods implemented in Rust (src/lib.rs)
    private external fun initService(storageDir: String): Boolean
    external fun sanitizeSync(rawUrl: String): String
}
