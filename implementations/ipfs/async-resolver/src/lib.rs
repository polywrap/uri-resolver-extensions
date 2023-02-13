pub mod wrap;
use wrap::*;
mod util;
use util::*;
use cid::Cid;

pub fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if env.is_none() {
        panic!("Ipfs uri resolver requires a configured Env")
    }
    if args.authority != "ipfs" {
        return None;
    }

    if !is_cid(&args.path) {
        // Not a valid CID
        return Some(UriResolverMaybeUriOrManifest { manifest: None, uri: None });
    }

    let path = format!("{}/wrap.info", &args.path);
    let manifest: Option<Vec<u8>> = get_file(ArgsGetFile { path }, env);

    return Some(UriResolverMaybeUriOrManifest { manifest, uri: None });
}

pub fn get_file(args: ArgsGetFile, env: Option<Env>) -> Option<Vec<u8>> {
    let env = env.expect("Ipfs uri resolver requires a configured Env");
    let options: Options = get_options(&env);
    if options.disable_parallel_requests || options.providers.len() == 1 {
        return exec_sequential(&options.providers, &args.path, options.timeout).ok();
    }
    return exec_parallel(&options.providers, &args.path, options.timeout).ok();
}

fn is_cid(maybe_cid: &str) -> bool {
    return Cid::try_from(maybe_cid).is_ok();
}
