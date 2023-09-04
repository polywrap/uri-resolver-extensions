pub mod wrap;
pub use wrap::*;

const DEFAULT_PROVIDER_URL: &str = "https/wraps.wrapscan.io";
const RESOLVE_PATH: &str = "/r/";
const AUTHORITY: &str = "wrapscan.io";

impl ModuleTrait for Module {
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        if args.authority != AUTHORITY {
            return Ok(None);
        }

        let provider_url = env
            .and_then(|x| x.provider_url)
            .unwrap_or(DEFAULT_PROVIDER_URL.to_string());

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
