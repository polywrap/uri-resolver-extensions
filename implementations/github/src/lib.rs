pub mod wrap;

use std::str::from_utf8;
use wrap::{*, imported::{ArgsGet}};
use base64::{decode};

const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com";
const URI_SEARCH_PATTERN: &str = "deployment.txt";

pub fn try_resolve_uri(args: ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "github.com" {
        return None;
    }

    let request_url = GITHUB_RAW_URL.to_string() + "/" + &args.path + "/" + URI_SEARCH_PATTERN;
    let result = HttpModule::get(&ArgsGet {
        url: request_url,
        request: Some(HttpRequest{
            response_type: HttpResponseType::BINARY,
            headers: None,
            url_params: None,
            body: None,
            timeout: None,
            form_data: None,
        })
    });

    let response = match result {
        Ok(Some(x)) => x,
        _ => return Some(UriResolverMaybeUriOrManifest {
            uri: None,
            manifest: None
        })
    };

    if let Some(body) = response.body {
        let content = decode(body).unwrap();
        let uri = from_utf8(content.as_slice()).unwrap().to_string();
        return Some(UriResolverMaybeUriOrManifest {
            uri: Some(uri),
            manifest: None
        });
    }

    Some(UriResolverMaybeUriOrManifest {
        uri: None,
        manifest: None
    })
}

pub fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Option<Vec<u8>> {
    None
}
