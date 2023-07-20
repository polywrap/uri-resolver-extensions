pub mod wrap;

use base58::ToBase58;
use hex;
use wrap::*;

impl ModuleTrait for Module {
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        _try_resolve_uri(&args, env)
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}

pub fn _try_resolve_uri(
    args: &ArgsTryResolveUri,
    _env: Option<Env>,
) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
    if args.authority != "ens-contenthash" {
        return Ok(None);
    }

    if args.path.starts_with("0xe3010170") && is_hex_string(&args.path, 38) {
        let hex_data = &args.path[10..];
        let cid = match hex::decode(hex_data) {
            Ok(bytes) => bytes.to_base58(),
            Err(err) => return Err(format!("Error decoding hex({}): {}", hex_data, err)),
        };

        return Ok(Some(UriResolverMaybeUriOrManifest {
            uri: Some(format!("wrap://ipfs/{cid}")),
            manifest: None,
        }));
    }

    return Ok(None);
}

fn is_hex_string(value: &str, length: i32) -> bool {
    if !value.starts_with("0x") {
        return false;
    }
    for c in value[2..].chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    if value.len() != (2 + 2 * length) as usize {
        return false;
    }
    return true;
}
