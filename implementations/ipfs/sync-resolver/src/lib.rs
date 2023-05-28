pub mod wrap;
use wrap::*;
mod util;
use util::*;
use cid::Cid;

impl ModuleTrait for Module {
    fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        let env = env.expect("IPFS uri resolver requires a configured Env");

        if args.authority != "ipfs" && args.authority != "wrap" {
            return Ok(None);
        }

        if !is_cid(&args.path) {
            return Err(format!("Invalid CID: {}", &args.path));
        }

        let path = format!("{}/wrap.info", &args.path);
        let options: Options = get_options(&env, false);
        let manifest: Vec<u8> = exec_with_options(&path, &options);

        return Ok(Some(UriResolverMaybeUriOrManifest { 
            manifest: Some(manifest), 
            uri: None 
        }));
    }

    fn get_file(args: ArgsGetFile, env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        let env = env.expect("IPFS URI resolver requires a configured Env");
        let options: Options = get_options(&env, true);
        Ok(Some(exec_with_options(&args.path, &options)))
    }
}

fn exec_with_options(path: &str, options: &Options) -> Vec<u8> {
    let mut attempts = options.retries + 1;
    loop {
        let result = exec_sequential(&options.providers, &path, options.timeout);
        
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
