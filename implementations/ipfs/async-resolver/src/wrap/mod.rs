pub mod entry;
pub mod retries;
pub use retries::Retries;
pub mod env;
pub use env::Env;
pub mod imported;
pub use imported::uri_resolver_maybe_uri_or_manifest::UriResolverMaybeUriOrManifest;
pub use imported::client_cat_options::ClientCatOptions;
pub use imported::client_resolve_options::ClientResolveOptions;
pub use imported::client_resolve_result::ClientResolveResult;
pub use imported::client_file_entry::ClientFileEntry;
pub use imported::client_add_options::ClientAddOptions;
pub use imported::client_add_result::ClientAddResult;
pub use imported::client_directory_entry::ClientDirectoryEntry;
pub use imported::client_blob::ClientBlob;
pub use imported::concurrent_task_result::ConcurrentTaskResult;
pub use imported::concurrent_task::ConcurrentTask;
pub use imported::concurrent_return_when::{
    get_concurrent_return_when_key,
    get_concurrent_return_when_value,
    sanitize_concurrent_return_when_value,
    ConcurrentReturnWhen
};
pub use imported::concurrent_task_status::{
    get_concurrent_task_status_key,
    get_concurrent_task_status_value,
    sanitize_concurrent_task_status_value,
    ConcurrentTaskStatus
};
pub use imported::uri_resolver_module::UriResolverModule;
pub use imported::client_module::ClientModule;
pub use imported::concurrent_module::ConcurrentModule;
pub mod concurrent;
pub use concurrent::Concurrent;
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
