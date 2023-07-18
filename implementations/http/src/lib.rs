pub mod wrap;
use polywrap_wasm_rs::Map;
use base64::decode;
use wrap::{*, imported::ArgsGet};

const MANIFEST_SEARCH_PATTERN: &str = "wrap.info";
const URI_HEADER_KEY: &str = "x-wrap-uri";

impl ModuleTrait for Module {
    fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        Ok(_try_resolve_uri(&args, env))
    } 

    fn get_file(args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        let result = HttpModule::get(&ArgsGet {
            url: args.path,
            request: Some(HttpRequest{
                response_type: HttpResponseType::BINARY,
                headers: None,
                url_params: None,
                body: None,
                timeout: None,
                form_data: None,
            })
        });
    
        if let Err(err) = result {
            return Err(format!("Error during HTTP request: {}", err));
        }
    
        let response = match result.unwrap() {
            None => return Ok(None),
            Some(x) => x
        };
    
        match response.body {
            None => Ok(None),
            Some(body) => match decode(body) {
                Ok(body) => Ok(Some(body)),
                Err(err) => {
                    Err(format!("Error during base64 decoding of body: {}", err.to_string()))
                }
            }
        }
    }
}

pub fn _try_resolve_uri(args: &ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
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
        url: url.clone(),
        request: Some(HttpRequest{
            response_type: HttpResponseType::BINARY,
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

    let redirect_uri = get_redirect_uri_from_headers(&response.headers);

    let manifest = get_manifest_from_body(&response.body);

    if redirect_uri.is_none() && manifest.is_none() {
        panic!("No URI or manifest found at {}", &url);
    } 
    
    Some(UriResolverMaybeUriOrManifest {
        uri: redirect_uri,
        manifest
    })
}

fn get_redirect_uri_from_headers(headers: &Option<Map<String, String>>) -> Option<String> {
    if let Some(headers) = headers {
        if let Some(uri) = headers.get(URI_HEADER_KEY) {
            return Some(uri.clone())
        }
    }

    None
}

fn get_manifest_from_body(body: &Option<String>) -> Option<Vec<u8>> {
    match body {
        None => None,
        Some(body) => if body.len() == 0 {
            None
        } else {
            match decode(body) {
                Ok(body) => Some(body),
                Err(err) => {
                    panic!("Error during base64 decoding of body: {}", err.to_string());
                }
            }
        }
    }
}
