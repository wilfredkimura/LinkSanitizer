package com.example.linksanitizer

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Initialize Rust backend
        RustBridge.init(applicationContext)

        // Simple programmatic UI for testing on device
        val layout = android.widget.LinearLayout(this).apply {
            orientation = android.widget.LinearLayout.VERTICAL
            setPadding(48, 48, 48, 48)
        }

        val title = TextView(this).apply {
            text = "Rivium + Apalis LinkSanitizer"
            textSize = 20f
            setPadding(0, 0, 0, 32)
        }

        val input = EditText(this).apply {
            hint = "Paste tracking URL here..."
            setText("https://news.ycombinator.com?utm_source=twitter&utm_medium=social&fbclid=123")
        }

        val button = Button(this).apply {
            text = "Sanitize URL with Rust"
        }

        val output = TextView(this).apply {
            textSize = 16f
            setPadding(0, 32, 0, 0)
        }

        button.setOnClickListener {
            val raw = input.text.toString()
            val cleaned = RustBridge.sanitizeSync(raw)
            output.text = "Result:\n$cleaned"
            NotificationHelper.showCleanedLinkNotification(this, raw, cleaned)
        }

        layout.addView(title)
        layout.addView(input)
        layout.addView(button)
        layout.addView(output)

        setContentView(layout)
    }
}
