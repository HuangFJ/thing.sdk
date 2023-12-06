uniffi_macros::include_scaffolding!("thing");

use std::sync::Arc;

pub use hd_wallet::MyClass;

pub fn create_myclass() -> Arc<MyClass> {
    Arc::new(MyClass {})
}
