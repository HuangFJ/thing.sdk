# thing.sdk
## Prequisites
```shell
brew install swiftformat
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-linux-android x86_64-linux-android
cargo install --path uniffi-bindgen 

# specific NDK path
NDK_TOOLCHAINS_PATH=/Users/jon/Library/Android/sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/darwin-x86_64/bin
```

## iOS
### Add generated headers and xcframework to the project
Drag and drop the `headers`, `Sources` folders and `thing.xcframework` to the Xcode project.

### Tell swiftc where to find the generated headers
In Xcode, select app's main target -> build settings -> Objective-C Bridging Header -> add the path to the bridging header file: `headers/Bridging-Header.h`.

