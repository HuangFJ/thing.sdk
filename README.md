# thing.sdk
## Prequisites
```shell
brew install swiftformat
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-linux-android x86_64-linux-android
cargo install --path uniffi-bindgen 

# specific NDK path
NDK_TOOLCHAINS_PATH=/Users/jon/Library/Android/sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/darwin-x86_64/bin
```

## Build
```shell
make
```

## iOS
It will generate the following files:
- `projects/ios/headers/thingFFI.h` - C header file
- `projects/ios/headers/thingFFI.modulemap` - C modulemap file
- `projects/ios/Sources/thing.swift` - Swift wrapper
- `projects/ios/thing.xcframework` - XCFramework

### Add generated headers and xcframework to the project
Open the Xcode project in the `projects/ios` folder. Add the `headers/thingFFI.*`, `Sources/thing.swift` and `thing.xcframework` files to the responding folders in the Xcode project.

### Back to the code
```swift
let wallet = HdWallet.init(coinType: 0, seedHex: "92ff6cd1fc51db4fd09d4204750c3e72a117488ce893d08811833ecca502e333d149ead97d80f7cb5f347ba9cf5cecb4745cd7dcd4c6dd8d528997086f445a3c")
let masterPriv = wallet.exportMasterPriv()
let wallet = HdWallet.fromMasterPriv(masterPriv: masterPriv)
wallet.bip44Address()
wallet.bip86Address()
```
