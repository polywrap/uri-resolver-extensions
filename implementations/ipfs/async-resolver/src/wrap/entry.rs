use crate::{
    try_resolve_uri_wrapped,
    get_file_wrapped
};
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);

    match args.method.as_str() {
        "tryResolveUri" => invoke::wrap_invoke(args, env_size, Some(try_resolve_uri_wrapped)),
        "getFile" => invoke::wrap_invoke(args, env_size, Some(get_file_wrapped)),
        _ => invoke::wrap_invoke(args, env_size, None),
    }
}
