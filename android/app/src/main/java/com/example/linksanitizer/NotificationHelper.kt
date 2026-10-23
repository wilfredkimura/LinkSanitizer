package com.example.linksanitizer

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.os.Build
import android.widget.Toast
import androidx.core.app.NotificationCompat

object NotificationHelper {
    private const val CHANNEL_ID = "link_sanitizer_channel"
    private const val CHANNEL_NAME = "Cleaned Links"

    fun showCleanedLinkNotification(context: Context, originalUrl: String, cleanedUrl: String) {
        val notificationManager = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager

        // Create channel for Android 8.0+
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                CHANNEL_NAME,
                NotificationManager.IMPORTANCE_HIGH
            ).apply {
                description = "Notifications for sanitized URLs"
            }
            notificationManager.createNotificationChannel(channel)
        }

        // Copy cleaned link directly to clipboard
        val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
        val clip = ClipData.newPlainText("Cleaned URL", cleanedUrl)
        clipboard.setPrimaryClip(clip)
        Toast.makeText(context, "Cleaned link copied to clipboard!", Toast.LENGTH_SHORT).show()

        // Notification builder
        val notification = NotificationCompat.Builder(context, CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_menu_share)
            .setContentTitle("Link Sanitized & Copied!")
            .setContentText(cleanedUrl)
            .setStyle(NotificationCompat.BigTextStyle().bigText("Cleaned:\n$cleanedUrl\n\nOriginal:\n$originalUrl"))
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setAutoCancel(true)
            .build()

        notificationManager.notify(System.currentTimeMillis().toInt(), notification)
    }
}
