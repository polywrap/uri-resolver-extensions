pub mod entry;
pub mod imported;
pub use imported::uri_resolver_maybe_uri_or_manifest::UriResolverMaybeUriOrManifest;
pub use imported::uri_resolver_module::UriResolverModule;
pub mod module;
pub use module::{
    deserialize_try_resolve_uri_args,
    serialize_try_resolve_uri_result,
    try_resolve_uri_wrapped,
    ArgsTryResolveUri,
    deserialize_get_file_args,
    serialize_get_file_result,
    get_file_wrapped,
    ArgsGetFile
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
