pub mod entry;
pub mod env;
pub use env::Env;
pub mod imported;
pub use imported::uri_resolver_maybe_uri_or_manifest::UriResolverMaybeUriOrManifest;
pub use imported::http_request::HttpRequest;
pub use imported::http_response::HttpResponse;
pub use imported::http_response_type::{
    get_http_response_type_key,
    get_http_response_type_value,
    sanitize_http_response_type_value,
    HttpResponseType
};
pub use imported::uri_resolver_module::UriResolverModule;
pub use imported::http_module::HttpModule;
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
