fn main() {
    uniffi_build::generate_scaffolding("./src/thing.udl")
        .expect("Building the UDL file failed");
}
