#![warn(
    missing_debug_implementations,
    // missing_docs,
    // clippy::missing_docs_in_private_items,
    // clippy::missing_errors_doc,
    // clippy::missing_panics_doc,
    clippy::missing_const_for_fn
)]
#![deny(unsafe_code, unreachable_pub)]

pub use config::*;
pub use lmm::SupplierInfo;

mod lmm;

pub mod config;
