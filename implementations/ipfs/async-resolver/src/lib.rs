pub mod wrap;
use wrap::*;
mod util;
use util::*;
use cid::Cid;

pub fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    let env = env.expect("Ipfs uri resolver requires a configured Env");

    if args.authority != "ipfs" {
        return None;
    }

    if !is_cid(&args.path) {
        panic!("Invalid CID: {}", &args.path);
    }

    let path = format!("{}/wrap.info", &args.path);
    let options: Options = get_options(&env, false);
    let manifest: Vec<u8> = exec_with_options(&path, &options);

    return Some(UriResolverMaybeUriOrManifest { 
        manifest: Some(manifest), 
        uri: None 
    });
}

pub fn get_file(args: ArgsGetFile, env: Option<Env>) -> Option<Vec<u8>> {
    let env = env.expect("Ipfs uri resolver requires a configured Env");
    let options: Options = get_options(&env, true);
    Some(exec_with_options(&args.path, &options))
}

fn exec_with_options(path: &str, options: &Options) -> Vec<u8> {
    let synchronous = options.disable_parallel_requests || options.providers.len() == 1;
    let mut attempts = options.retries + 1;

    loop {
        let result = if synchronous {
            exec_sequential(&options.providers, &path, options.timeout)
        } else {
            exec_parallel(&options.providers, &path, options.timeout)
        };
        
        match result {
            Ok(result) => return result,
            Err(err) => {
                attempts = attempts - 1;

                if attempts == 0 {
                    panic!("Failed to resolve IPFS URI with error: {}", err);
                }
            }
        };
    }
}

fn is_cid(maybe_cid: &str) -> bool {
    return Cid::try_from(maybe_cid).is_ok();
}
