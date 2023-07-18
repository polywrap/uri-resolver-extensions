pub mod wrap;

use base58::ToBase58;
use wrap::*;
use hex;

impl ModuleTrait for Module {
    fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        Ok(_try_resolve_uri(&args, env))
    } 

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}

pub fn _try_resolve_uri(args: &ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "ens-contenthash" {
        return None;
    }

    if args.path.starts_with("0xe3010170") && is_hex_string(&args.path, 38) {
        let hex_data = &args.path[10..];
        let cid = match hex::decode(hex_data) {
            Ok(bytes) => bytes.to_base58(),
            Err(err) => panic!("Error decoding hex({}): {}", hex_data, err),
        };

        return Some(UriResolverMaybeUriOrManifest {
            uri: Some(format!("wrap://ipfs/{cid}")),
            manifest: None,
        });
    }

    return None;
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