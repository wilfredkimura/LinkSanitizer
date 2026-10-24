package com.example.linksanitizer

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.util.Log

/**
 * Handles incoming intent when a user shares a link via Android Share Sheet.
 */
class ShareActivity : Activity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Initialize Rust backend if not already done
        RustBridge.init(applicationContext)

        if (intent?.action == Intent.ACTION_SEND && intent.type?.startsWith("text/") == true) {
            val sharedText = intent.getStringExtra(Intent.EXTRA_TEXT)
                ?: intent.getCharSequenceExtra(Intent.EXTRA_TEXT)?.toString()
                ?: ""
            Log.d("ShareActivity", "Received shared text: $sharedText")

            if (sharedText.isNotBlank()) {
                // Call Rust logic via JNI
                val cleanedUrl = RustBridge.sanitizeSync(sharedText)

                // Show notification with the cleaned URL and copy to clipboard
                NotificationHelper.showCleanedLinkNotification(
                    context = this,
                    originalUrl = sharedText,
                    cleanedUrl = cleanedUrl
                )
            }
        }

        // Close activity immediately so user stays in their previous app seamlessly
        finish()
        overridePendingTransition(0, 0)
    }
}
