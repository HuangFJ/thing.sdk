IOS_PROJECT := ../../projects/ios
IOS_LIB := libffi.a

NDK_TOOLCHAINS_PATH ?= /Users/jon/Library/Android/sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/darwin-x86_64/bin
ANDROID_PROJECT := ../../projects/android
ANDROID_LIB := libffi.so

export PATH := ${NDK_TOOLCHAINS_PATH}:${PATH}

all: ios android-aarch64 android-x86_64

ios: 
	cd bindings/ffi && \
	cargo build --release --target aarch64-apple-ios && \
	cargo build --release --target aarch64-apple-ios-sim && \
	cargo build --release --target x86_64-apple-ios && \
	mkdir -p ../../target/ios-combined && \
	lipo -create \
		../../target/x86_64-apple-ios/release/${IOS_LIB} \
	  	../../target/aarch64-apple-ios-sim/release/${IOS_LIB} \
	  	-output ../../target/ios-combined/${IOS_LIB} && \
	mkdir -p ${IOS_PROJECT} && \
	rm -f ${IOS_PROJECT}/${IOS_LIB} && \
	cp ../../target/ios-combined/${IOS_LIB} ${IOS_PROJECT}/ && \
	uniffi-bindgen \
		generate src/thing.udl \
		--language swift \
		--config uniffi.toml \
		--out-dir ${IOS_PROJECT} && \
	mkdir -p ${IOS_PROJECT}/headers && \
	mkdir -p ${IOS_PROJECT}/Sources && \
	mv ${IOS_PROJECT}/*.h         ${IOS_PROJECT}/headers/ && \
	mv ${IOS_PROJECT}/*.modulemap ${IOS_PROJECT}/headers/ && \
	mv ${IOS_PROJECT}/*.swift     ${IOS_PROJECT}/Sources/ && \
	rm -rf ${IOS_PROJECT}/HDWallet.xcframework && \
	xcodebuild -create-xcframework \
	  -library ../../target/aarch64-apple-ios/release/${IOS_LIB} \
	  -headers ${IOS_PROJECT}/headers \
	  -library ${IOS_PROJECT}/${IOS_LIB} \
	  -headers ${IOS_PROJECT}/headers \
	  -output ${IOS_PROJECT}/HDWallet.xcframework

${NDK_TOOLCHAINS_PATH}/aarch64-linux-android-clang:
	cd ${NDK_TOOLCHAINS_PATH} && ln -sf aarch64-linux-android32-clang aarch64-linux-android-clang
${NDK_TOOLCHAINS_PATH}/aarch64-linux-android-ar:
	cd ${NDK_TOOLCHAINS_PATH} && ln -sf llvm-ar aarch64-linux-android-ar

android-aarch64: ${NDK_TOOLCHAINS_PATH}/aarch64-linux-android-clang ${NDK_TOOLCHAINS_PATH}/aarch64-linux-android-ar
	cd bindings/ffi && \
	cargo build --release --target aarch64-linux-android && \
	mkdir -p ${ANDROID_PROJECT}/app/src/main/jniLibs/aarch64 && \
	cp ../../target/aarch64-linux-android/release/${ANDROID_LIB} \
		${ANDROID_PROJECT}/app/src/main/jniLibs/aarch64/${ANDROID_LIB} && \
	mkdir -p ${ANDROID_PROJECT}/app/src/main/jniLibs/arm64-v8a && \
	cp ../../target/aarch64-linux-android/release/${ANDROID_LIB} \
		${ANDROID_PROJECT}/app/src/main/jniLibs/arm64-v8a/${ANDROID_LIB}

${NDK_TOOLCHAINS_PATH}/x86_64-linux-android-clang:
	cd ${NDK_TOOLCHAINS_PATH} && ln -sf x86_64-linux-android32-clang x86_64-linux-android-clang
${NDK_TOOLCHAINS_PATH}/x86_64-linux-android-ar:
	cd ${NDK_TOOLCHAINS_PATH} && ln -sf llvm-ar x86_64-linux-android-ar

android-x86_64: ${NDK_TOOLCHAINS_PATH}/x86_64-linux-android-clang ${NDK_TOOLCHAINS_PATH}/x86_64-linux-android-ar
	cd bindings/ffi && \
	cargo build --release --target x86_64-linux-android && \
	mkdir -p ${ANDROID_PROJECT}/app/src/main/jniLibs/x86_64 && \
	cp ../../target/x86_64-linux-android/release/${ANDROID_LIB} \
		${ANDROID_PROJECT}/app/src/main/jniLibs/x86_64/${ANDROID_LIB}
