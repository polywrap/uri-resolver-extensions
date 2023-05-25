mod ipfs;
use ipfs::{is_ipfs_cid};

pub mod wrap;
use wrap::{
    *,
    imported::{
        ArgsTryResolveUri,
        ArgsGetFile,
        UriResolverMaybeUriOrManifest
    }
};

impl ModuleTrait for Module {
    fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        if args.authority.to_lowercase() != "wrap" {
            return Ok(None);
        }

        let id = args.path;

        // If id is an IPFS CID
        if is_ipfs_cid(&id) {
            return Ok(redirect(format!("wrap://ipfs/{}", id)));
        }

        Ok(None)
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}

fn redirect<T: Into<String>>(uri: T) -> Option<UriResolverMaybeUriOrManifest> {
    Some(UriResolverMaybeUriOrManifest {
        uri: Some(uri.into()),
        manifest: None
    })
}
