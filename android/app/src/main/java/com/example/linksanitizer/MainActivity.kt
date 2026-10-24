package com.example.linksanitizer

import android.Manifest
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.Color
import android.graphics.Typeface
import android.os.Build
import android.os.Bundle
import android.view.Gravity
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat

class MainActivity : AppCompatActivity() {

    private lateinit var statusText: TextView
    private lateinit var inputEdit: EditText
    private lateinit var outputText: TextView
    private lateinit var copyButton: Button
    private lateinit var shareButton: Button

    private val requestPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { isGranted: Boolean ->
        if (!isGranted) {
            Toast.makeText(this, "Notification permission required for alert badges", Toast.LENGTH_SHORT).show()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Initialize Rust backend
        RustBridge.init(applicationContext)

        // Request notification permission on Android 13+
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
                requestPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
            }
        }

        buildUi()

        // Check if opened with an existing result from a notification
        val lastUrl = intent?.getStringExtra("last_cleaned_url")
        if (!lastUrl.isNullOrEmpty()) {
            displayResult(lastUrl)
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
        val lastUrl = intent.getStringExtra("last_cleaned_url")
        if (!lastUrl.isNullOrEmpty()) {
            displayResult(lastUrl)
        }
    }

    private fun displayResult(cleanedUrl: String) {
        outputText.text = cleanedUrl
        copyButton.visibility = View.VISIBLE
        shareButton.visibility = View.VISIBLE
    }

    private fun buildUi() {
        val rootScrollView = ScrollView(this).apply {
            isFillViewport = true
            setBackgroundColor(Color.parseColor("#F5F7FA"))
        }

        val container = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(48, 64, 48, 64)
        }

        // Header Title
        val titleView = TextView(this).apply {
            text = "LinkSanitizer"
            textSize = 26f
            typeface = Typeface.DEFAULT_BOLD
            setTextColor(Color.parseColor("#1A202C"))
        }

        val subtitleView = TextView(this).apply {
            text = "Embedded Rust & Apalis Privacy Engine"
            textSize = 14f
            setTextColor(Color.parseColor("#718096"))
            setPadding(0, 8, 0, 32)
        }

        // Engine Status Badge
        statusText = TextView(this).apply {
            val loaded = RustBridge.isReady()
            text = if (loaded) " Rust Native Core: ACTIVE" else " Rust Native Core: PENDING/FALLBACK"
            textSize = 13f
            setTextColor(if (loaded) Color.parseColor("#2E7D32") else Color.parseColor("#D32F2F"))
            setBackgroundColor(if (loaded) Color.parseColor("#E8F5E9") else Color.parseColor("#FFEBEE"))
            setPadding(24, 12, 24, 12)
        }

        // Instructions
        val instructions = TextView(this).apply {
            text = "Share links from Chrome, Twitter, or YouTube directly to LinkSanitizer to strip tracking parameters automatically, or test below:"
            textSize = 14f
            setTextColor(Color.parseColor("#4A5568"))
            setPadding(0, 32, 0, 16)
        }

        // Input Field
        inputEdit = EditText(this).apply {
            hint = "Paste tracking URL or shared text..."
            setText("https://news.ycombinator.com?utm_source=twitter&utm_medium=social&fbclid=abc123xyz")
            setBackgroundColor(Color.WHITE)
            setPadding(32, 32, 32, 32)
            textSize = 14f
            setTextColor(Color.parseColor("#2D3748"))
        }

        // Sanitize Button
        val sanitizeButton = Button(this).apply {
            text = "Sanitize Link"
            setBackgroundColor(Color.parseColor("#1E88E5"))
            setTextColor(Color.WHITE)
            setPadding(0, 24, 0, 24)
        }

        // Output Result Header
        val resultLabel = TextView(this).apply {
            text = "Cleaned Output:"
            textSize = 14f
            typeface = Typeface.DEFAULT_BOLD
            setTextColor(Color.parseColor("#2D3748"))
            setPadding(0, 32, 0, 8)
        }

        // Output Result Box
        outputText = TextView(this).apply {
            text = "No URL sanitized yet."
            setBackgroundColor(Color.WHITE)
            setPadding(32, 32, 32, 32)
            textSize = 14f
            setTextColor(Color.parseColor("#2D3748"))
            setTextIsSelectable(true)
        }

        // Action Buttons Row (Copy & Share)
        val buttonRow = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            setPadding(0, 16, 0, 0)
        }

        copyButton = Button(this).apply {
            text = "Copy"
            visibility = View.GONE
            setOnClickListener {
                val cleaned = outputText.text.toString()
                val clipboard = getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                clipboard.setPrimaryClip(ClipData.newPlainText("Cleaned URL", cleaned))
                Toast.makeText(this@MainActivity, "Copied to clipboard!", Toast.LENGTH_SHORT).show()
            }
        }

        shareButton = Button(this).apply {
            text = "Share"
            visibility = View.GONE
            setOnClickListener {
                val cleaned = outputText.text.toString()
                val share = Intent(Intent.ACTION_SEND).apply {
                    type = "text/plain"
                    putExtra(Intent.EXTRA_TEXT, cleaned)
                }
                startActivity(Intent.createChooser(share, "Share Cleaned Link"))
            }
        }

        buttonRow.addView(copyButton)
        buttonRow.addView(shareButton)

        sanitizeButton.setOnClickListener {
            val raw = inputEdit.text.toString()
            if (raw.isNotBlank()) {
                val cleaned = RustBridge.sanitizeSync(raw)
                displayResult(cleaned)
                NotificationHelper.showCleanedLinkNotification(this, raw, cleaned)
            }
        }

        container.addView(titleView)
        container.addView(subtitleView)
        container.addView(statusText)
        container.addView(instructions)
        container.addView(inputEdit)
        container.addView(sanitizeButton)
        container.addView(resultLabel)
        container.addView(outputText)
        container.addView(buttonRow)

        rootScrollView.addView(container)
        setContentView(rootScrollView)
    }
}
