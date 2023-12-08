# thing.sdk
## Prequisites
```shell
brew install swiftformat ktlint
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-linux-android x86_64-linux-android armv7-linux-androideabi
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
// sign a p2pkh tx
p2pkhSign(coinType: 0, privHex: "", txHex: "")
// sign a p2tr tx
p2trSign(coinType: 0, privHex: "", txHex: "", txPrevoutsJson: "[]")
```

## Android
It will generate the following files:
- `projects/android/app/src/main/jniLibs/x86_64/libuniffi_thing.so` - Android x86_64 shared library
- `projects/android/app/src/main/jniLibs/arm64-v8a/libuniffi_thing.so` - Android arm64-v8a shared library
- `projects/android/app/src/main/jniLibs/armeabi-v7a/libuniffi_thing.so` - Android armeabi-v7a shared library

### Back to Android Studio
Open the Android Studio project in the `projects/android` folder. Change the code:
```kotlin
val wallet = HdWallet(0u, "92ff6cd1fc51db4fd09d4204750c3e72a117488ce893d08811833ecca502e333d149ead97d80f7cb5f347ba9cf5cecb4745cd7dcd4c6dd8d528997086f445a3c")
val masterPriv = wallet.exportMasterPriv()
val wallet = HdWallet.fromMasterPriv(masterPriv)
wallet.bip44Address()
wallet.bip86Address()
// sign a p2pkh tx
p2pkhSign(0u, "", "")
// sign a p2tr tx
p2trSign(0u, "", "", "[]")
```