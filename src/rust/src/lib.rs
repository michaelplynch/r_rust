use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello World from Rust"
}

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_worldd() {
    "Hello worldd!";
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rrust;
    fn hello_world;
    fn hello_worldd;
}
