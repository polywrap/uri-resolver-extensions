pub mod wrap;
pub use wrap::*;

const DEFAULT_PROVIDER_URL: &str = "http/wraps.wrapscan.io";
const RESOLVE_PATH: &str = "/r/";

impl ModuleTrait for Module {
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        if args.authority != "wrapscan" {
            return Ok(None);
        }

        let provider_url = match env {
            Some(env) => env.provider_url.unwrap_or(DEFAULT_PROVIDER_URL.to_string()),
            None => DEFAULT_PROVIDER_URL.to_string(),
        };

        let wrap_url = provider_url + RESOLVE_PATH + &args.path;

        Ok(Some(UriResolverMaybeUriOrManifest {
            uri: Some(wrap_url),
            manifest: None,
        }))
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}
