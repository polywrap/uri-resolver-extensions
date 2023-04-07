pub mod wrap;

use wrap::*;
use ocr_core_rs::ocr_id::decode_ocr_id_from_contenthash;

pub fn try_resolve_uri(args: ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "ens-contenthash" {
        return None;
    }

    let maybe_ocr_id = decode_ocr_id_from_contenthash(&args.path, None);

    let ocr_id = match maybe_ocr_id {
        Ok(ocr_id) => ocr_id,
        Err(_) => return None,
    };

    Some(UriResolverMaybeUriOrManifest {
        uri: Some(format!("wrap://ocr/{}/{}/{}/{}/{}/{}",
            ocr_id.protocol_version,
            ocr_id.chain_id,
            ocr_id.contract_address,
            ocr_id.package_index,
            ocr_id.start_block,
            ocr_id.end_block
        )),
        manifest: None 
    })
}

pub fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Option<Vec<u8>> {
    None
}