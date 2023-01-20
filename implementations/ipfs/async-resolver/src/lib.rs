pub mod wrap;
pub use wrap::*;
pub mod util;
pub use util::*;
use cid::Cid;

pub fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    let env = env.expect("Ipfs uri resolver requires a configured Env");

    if args.authority != "ipfs" {
        return None;
    }

    if !is_cid(&args.path) {
        // Not a valid CID
        return Some(UriResolverMaybeUriOrManifest { manifest: None, uri: None });
    }

    let path = format!("{}/wrap.info", &args.path);
    let options: Options = get_options(&env, false);
    let manifest: Option<Vec<u8>> = exec_with_options(&path, &options);

    return Some(UriResolverMaybeUriOrManifest { manifest, uri: None });
}

pub fn get_file(args: ArgsGetFile, env: Option<Env>) -> Option<Vec<u8>> {
    let env = env.expect("Ipfs uri resolver requires a configured Env");
    let options: Options = get_options(&env, true);
    exec_with_options(&args.path, &options)
}

fn exec_with_options(path: &str, options: &Options) -> Option<Vec<u8>> {
    let synchronous = options.disable_parallel_requests || options.providers.len() == 1;
    let mut attempts = options.retries + 1;
    while attempts > 0 {
        let result: Result<Vec<u8>, String>;
        if synchronous {
            result = exec_sequential(&options.providers, &path, options.timeout);
        } else {
            result = exec_parallel(&options.providers, &path, options.timeout);
        };
        if result.is_ok() {
            return result.ok();
        }
        attempts = attempts - 1;
    }
    None
}

fn is_cid(maybe_cid: &str) -> bool {
    return Cid::try_from(maybe_cid).is_ok();
}
