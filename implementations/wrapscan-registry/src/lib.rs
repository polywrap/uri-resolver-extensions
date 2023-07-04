pub mod wrap;
pub use wrap::*;

const WRAPSCAN_RESOLVE_URL: &str = "http/wraps.wrapscan.io/r/";

impl ModuleTrait for Module {
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        let wrapscan_resolve_url = match env {
            Some(env) => env.resolve_url.unwrap_or(WRAPSCAN_RESOLVE_URL.to_string()),
            None => WRAPSCAN_RESOLVE_URL.to_string(),
        };

        if args.authority != "wrapscan" {
            return Ok(None);
        }

        let wrapscan_wrap_url = wrapscan_resolve_url + &args.path;

        Ok(Some(UriResolverMaybeUriOrManifest {
            uri: Some(wrapscan_wrap_url),
            manifest: None,
        }))
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}
