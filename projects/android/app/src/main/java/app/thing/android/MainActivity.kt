package app.thing.android

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import uniffi.thing.HdWallet

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val wallet = HdWallet(1u, null)
        setContentView(R.layout.activity_main)
        val textView = findViewById<TextView>(R.id.helloTextView)
        textView.text = wallet.exportMnemonic()
    }
}