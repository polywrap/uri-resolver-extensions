use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    try_resolve_uri,
    ArgsTryResolveUri,
    deserialize_try_resolve_uri_args,
    serialize_try_resolve_uri_result,
    get_file,
    ArgsGetFile,
    deserialize_get_file_args,
    serialize_get_file_result
};

use crate::Env;

pub fn try_resolve_uri_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method 'objectMethod'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = Env::from_buffer(&env_buf).unwrap();

    match deserialize_try_resolve_uri_args(args) {
        Ok(args) => {
            let result = try_resolve_uri(ArgsTryResolveUri {
                authority: args.authority,
                path: args.path,
            }, env);
            serialize_try_resolve_uri_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_file_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method 'objectMethod'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = Env::from_buffer(&env_buf).unwrap();

    match deserialize_get_file_args(args) {
        Ok(args) => {
            let result = get_file(ArgsGetFile {
                path: args.path,
            }, env);
            serialize_get_file_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
