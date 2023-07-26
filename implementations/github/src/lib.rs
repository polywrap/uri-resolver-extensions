pub mod wrap;

use base64::decode;
use std::str::from_utf8;
use wrap::{imported::ArgsGet, *};

const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com";
const URI_SEARCH_PATTERN: &str = "URI.txt";

impl ModuleTrait for Module {
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        _env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        if args.authority != "github.com" && args.authority != "http" && args.authority != "https" {
            return Ok(None);
        }

        // http url must point to github.com
        let mut path: &str = args
            .path
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let path_has_gh = path.starts_with("github.com/");
        if args.authority != "github.com" && !path_has_gh {
            return Ok(None);
        }
        path = path.trim_start_matches("github.com/");

        // build request url
        let mut path_parts = path.split("/");
        let org = match path_parts.nth(0) {
            Some(org) => org,
            None => return Ok(None),
        };
        let repo = match path_parts.nth(0) {
            Some(org) => org,
            None => return Ok(None),
        };
        let branch = match path_parts.nth(1) {
            Some(org) => org,
            None => return Ok(None),
        };
        let mut dir_parts: Vec<&str> = path_parts.collect();
        if dir_parts[dir_parts.len() - 1].is_empty() {
            dir_parts.pop();
        } // if path ends with /
        let dir = dir_parts.join("/");
        let request_url = format!(
            "{}/{}/{}/{}/{}/{}",
            GITHUB_RAW_URL, org, repo, branch, dir, URI_SEARCH_PATTERN
        );

        let result = HttpModule::get(&ArgsGet {
            url: request_url,
            request: Some(HttpRequest {
                response_type: HttpResponseType::BINARY,
                headers: None,
                url_params: None,
                body: None,
                timeout: None,
                form_data: None,
            }),
        });

        let response = match result {
            Ok(Some(x)) => x,
            _ => {
                return Ok(Some(UriResolverMaybeUriOrManifest {
                    uri: None,
                    manifest: None,
                }))
            }
        };

        if let Some(body) = response.body {
            let content = decode(body).map_err(|e| {
                format!(
                    "GitHub URI Resolver Extension: failed to decode URI.txt from base64: {}",
                    e.to_string()
                )
            })?;
            let uri = from_utf8(content.as_slice())
                .map_err(|e| {
                    format!(
                        "GitHub URI Resolver Extension: failed to parse URI.txt from bytes: {}",
                        e.to_string()
                    )
                })?
                .to_string();
            return Ok(Some(UriResolverMaybeUriOrManifest {
                uri: Some(uri),
                manifest: None,
            }));
        }

        Ok(Some(UriResolverMaybeUriOrManifest {
            uri: None,
            manifest: None,
        }))
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}
