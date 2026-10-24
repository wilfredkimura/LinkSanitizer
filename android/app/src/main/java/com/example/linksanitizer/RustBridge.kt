package com.example.linksanitizer

import android.content.Context
import android.net.Uri
import android.util.Log

/**
 * JNI Bridge to the Rust LinkSanitizer backend compiled with Rivium & Apalis.
 * Includes graceful Kotlin fallback in case the native .so has not yet been built.
 */
object RustBridge {
    private const val TAG = "RustBridge"
    private var nativeLibraryLoaded = false
    private var isInitialized = false

    private val FALLBACK_TRACKING_KEYS = setOf(
        "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content",
        "utm_id", "fbclid", "gclid", "gbraid", "wbraid", "_gl", "igshid", "igsh",
        "si", "feature", "ref", "ref_src", "s", "t", "tag", "linkcode"
    )

    init {
        try {
            System.loadLibrary("link_sanitizer")
            nativeLibraryLoaded = true
            Log.i(TAG, "Successfully loaded liblink_sanitizer.so")
        } catch (e: UnsatisfiedLinkError) {
            Log.w(TAG, "liblink_sanitizer.so not found; running in fallback mode: ${e.message}")
            nativeLibraryLoaded = false
        }
    }

    fun isReady(): Boolean = nativeLibraryLoaded

    /**
     * Initializes the background Rivium service & Apalis SQLite database.
     */
    fun init(context: Context) {
        if (nativeLibraryLoaded && !isInitialized) {
            try {
                val storagePath = context.filesDir.absolutePath
                val success = initService(storagePath)
                Log.i(TAG, "Rust service init: $success at $storagePath")
                isInitialized = success
            } catch (e: Throwable) {
                Log.e(TAG, "Error initializing native service: ${e.message}")
            }
        }
    }

    /**
     * Sanitizes URL or mixed text via native Rust core, or fallback if native lib is unavailable.
     */
    fun sanitizeSync(rawUrl: String): String {
        if (nativeLibraryLoaded) {
            try {
                return sanitizeNative(rawUrl)
            } catch (e: Throwable) {
                Log.e(TAG, "Native sanitize failed, using fallback: ${e.message}")
            }
        }
        return fallbackSanitize(rawUrl)
    }

    private fun fallbackSanitize(input: String): String {
        return try {
            val uri = Uri.parse(input.trim())
            if (uri.scheme != null && uri.host != null) {
                val builder = uri.buildUpon().clearQuery()
                for (param in uri.queryParameterNames) {
                    val lower = param.lowercase()
                    if (lower !in FALLBACK_TRACKING_KEYS && !lower.startsWith("utm_")) {
                        for (value in uri.getQueryParameters(param)) {
                            builder.appendQueryParameter(param, value)
                        }
                    }
                }
                builder.build().toString()
            } else {
                input
            }
        } catch (e: Exception) {
            input
        }
    }

    // Native JNI methods implemented in Rust (src/lib.rs)
    @JvmStatic
    private external fun initService(storageDir: String): Boolean

    @JvmStatic
    private external fun sanitizeNative(rawUrl: String): String
}
