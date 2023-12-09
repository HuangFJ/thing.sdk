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
            let wallet = HdWallet.init(isTestnet: 0, mnemonic: nil)
            Text(wallet.bip44Address())
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
