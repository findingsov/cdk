#[cfg(all(target_arch = "wasm32", feature = "blocking"))]
compile_error!("`blocking` feature can't be enabled for WASM targets");

#[cfg(feature = "blocking")]
use futures_util::Future;
#[cfg(feature = "blocking")]
use once_cell::sync::Lazy;
#[cfg(feature = "blocking")]
use tokio::runtime::Runtime;

#[cfg(feature = "wallet")]
pub mod client;

mod localstore;
#[cfg(feature = "mint")]
pub mod mint;
pub mod utils;
#[cfg(feature = "wallet")]
pub mod wallet;

pub use bip39::Mnemonic;
pub use cashu::{self, *};

#[cfg(feature = "blocking")]
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

#[cfg(feature = "blocking")]
pub fn block_on<F: Future>(future: F) -> F::Output {
    RUNTIME.block_on(future)
}
