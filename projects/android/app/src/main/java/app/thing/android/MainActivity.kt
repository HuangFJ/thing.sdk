package app.thing.android

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import uniffi.thing.HdWallet

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val wallet = HdWallet(0u, "92ff6cd1fc51db4fd09d4204750c3e72a117488ce893d08811833ecca502e333d149ead97d80f7cb5f347ba9cf5cecb4745cd7dcd4c6dd8d528997086f445a3c")
        setContentView(R.layout.activity_main)
        val textView = findViewById<TextView>(R.id.helloTextView)
        textView.text = wallet.bip44Address()
    }
}