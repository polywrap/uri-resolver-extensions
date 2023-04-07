pub mod wrap;
use wrap::{*, imported::{ArgsGet}};
use base64::decode;

const MANIFEST_SEARCH_PATTERN: &str = "wrap.info";

pub fn try_resolve_uri(args: ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "http" && args.authority != "https" {
        return None;
    }

    let path = if args.path.starts_with("https://") {
        args.path.clone()
    } else if args.path.starts_with("http://") {
        if args.authority == "https" {
            "https://".to_string() + &args.path[7..]
        } else {
            args.path.clone()
        }
    } else {
        let authority_prefix = format!("{}://", args.authority);
        authority_prefix + &args.path
    };

    let url = if path.ends_with("/") {
        path + MANIFEST_SEARCH_PATTERN
    } else {
        path + "/" + MANIFEST_SEARCH_PATTERN
    };

    let result = HttpModule::get(&ArgsGet {
        url,
        request: Some(HttpHttpRequest{
            response_type: HttpHttpResponseType::BINARY,
            headers: None,
            url_params: None,
            body: None,
            timeout: None,
            form_data: None,
        })
    });

    if let Err(err) = result {
        panic!("Error during HTTP request: {}", err);
    }

    let response = match result.unwrap() {
        None => return None,
        Some(x) => x,
    };

    match response.body {
        None => return None,
        Some(body) => match decode(body) {
            Ok(body) => return Some(UriResolverMaybeUriOrManifest {
                uri: None,
                manifest: Some(body)
            }),
            Err(err) => {
                panic!("Error during base64 decoding of body: {}", err.to_string());
            }
        }
    };
}

pub fn get_file(args: ArgsGetFile, _env: Option<Env>) -> Option<Vec<u8>> {
    let result = HttpModule::get(&ArgsGet {
        url: args.path,
        request: Some(HttpHttpRequest{
            response_type: HttpHttpResponseType::BINARY,
            headers: None,
            url_params: None,
            body: None,
            timeout: None,
            form_data: None,
        })
    });

    if let Err(err) = result {
        panic!("Error during HTTP request: {}", err);
    }

    let response = match result.unwrap() {
        None => return None,
        Some(x) => x
    };

    match response.body {
        None => return None,
        Some(body) => match decode(body) {
            Ok(body) => return Some(body),
            Err(err) => {
                panic!("Error during base64 decoding of body: {}", err.to_string());
            }
        }
    };
}
