//
//  ContentView.swift
//  ios
//
//  Created by Jon on 2023/12/7.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            let wallet = HdWallet.init(coinType: 0, seedHex: "92ff6cd1fc51db4fd09d4204750c3e72a117488ce893d08811833ecca502e333d149ead97d80f7cb5f347ba9cf5cecb4745cd7dcd4c6dd8d528997086f445a3c")
            Text(wallet.bip44Address())
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
