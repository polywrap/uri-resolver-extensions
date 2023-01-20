pub mod wrap;
pub use wrap::*;
pub mod util;
pub use util::*;
use cid::Cid;

pub fn try_resolve_uri(args: ArgsTryResolveUri, env: Env) -> Option<UriResolverMaybeUriOrManifest> {
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

pub fn get_file(args: ArgsGetFile, env: Env) -> Option<Vec<u8>> {
    let options: Options = get_options(&env);
    return exec_sequential(&options.providers, &args.path, options.timeout).ok();
}

fn is_cid(maybe_cid: &str) -> bool {
    return Cid::try_from(maybe_cid).is_ok();
}
