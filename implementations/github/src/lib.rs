pub mod wrap;

use std::str::from_utf8;
use wrap::{*, imported::{ArgsGet}};
use base64::{decode};

// polywrap/ipfs/tree/main/http-client/ipfs-http-client
// https://raw.githubusercontent.com/polywrap/ipfs/main/http-client/ipfs-http-client/polywrap.deployment.txt

const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com";
const URI_SEARCH_PATTERN: &str = "polywrap.deployment.txt";

pub fn try_resolve_uri(args: ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "github.com" && args.authority != "http" && args.authority != "https" {
        return None;
    }

    // http url must point to github.com
    let mut path: &str = args.path.trim_start_matches("http://").trim_start_matches("https://");
    let path_has_gh = path.starts_with("github.com/");
    if args.authority != "github.com" && !path_has_gh {
        return None;
    }
    path = path.trim_start_matches("github.com/");

    // build request url
    let mut path_parts = path.split("/");
    let org = path_parts.nth(0).unwrap();
    let repo = path_parts.nth(0).unwrap();
    let branch = path_parts.nth(1).unwrap();
    let mut dir_parts: Vec<&str> = path_parts.collect();
    if dir_parts[dir_parts.len() - 1].is_empty() { dir_parts.pop(); } // if path ends with /
    let dir = dir_parts.join("/");
    let request_url = format!("{}/{}/{}/{}/{}/{}", GITHUB_RAW_URL, org, repo, branch, dir, URI_SEARCH_PATTERN);

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
