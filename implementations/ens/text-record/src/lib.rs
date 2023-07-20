pub mod wrap;
use wrap::{
    imported::{ArgsGetResolver, ArgsGetTextRecord},
    *,
};

const DEFAULT_ENS_REGISTRY_ADDRESS: &str = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
const POLYWRAP_TEXT_RECORD_PREFIX: &str = "wrap/";
const TEXT_RECORD_SEPARATOR: &str = ":";
const PATH_SEPARATOR: &str = "/";

struct TextRecordInfo {
    network_name: String,
    carry_over_path: String,
    domain: String,
    text_record_key: String,
}

impl ModuleTrait for Module {
    // wrap://ens/test.eth:v1/wrap.info
    // wrap://ens/goerli/test.eth:v1/wrap.info
    fn try_resolve_uri(
        args: ArgsTryResolveUri,
        env: Option<Env>,
    ) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
        _try_resolve_uri(
            &args,
            env,
            &ENSModule::get_resolver,
            &ENSModule::get_text_record,
        )
    }

    fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Result<Option<Vec<u8>>, String> {
        Ok(None)
    }
}

fn parse_uri(args: &ArgsTryResolveUri) -> Option<TextRecordInfo> {
    if args.authority != "ens" {
        return None;
    }

    let path_parts: Vec<&str> = args.path.split(PATH_SEPARATOR).collect();

    if path_parts.len() < 1 {
        panic!("Invalid URI");
    }

    let domain_or_network = path_parts[0];

    if domain_or_network.is_empty() {
        panic!("Invalid URI");
    }

    let network_name;
    let domain_and_text_record;
    let carry_over_path;

    if domain_or_network.contains(".eth") {
        network_name = "mainnet";
        domain_and_text_record = domain_or_network;
        if path_parts.len() > 1 {
            carry_over_path = path_parts[1..].join(PATH_SEPARATOR);
        } else {
            carry_over_path = "".to_string();
        }
    } else if path_parts.len() < 2 {
        panic!("ENS domain not specified");
    } else {
        network_name = domain_or_network;
        domain_and_text_record = path_parts[1];
        carry_over_path = path_parts[2..].join(PATH_SEPARATOR);
    };

    let domain_parts: Vec<&str> = domain_and_text_record
        .split(TEXT_RECORD_SEPARATOR)
        .collect();

    if domain_parts.len() < 2 {
        return None;
    }

    let domain = domain_parts[0];
    let text_record_key = domain_parts[1];

    Some(TextRecordInfo {
        network_name: network_name.to_string(),
        carry_over_path: carry_over_path,
        domain: domain.to_string(),
        text_record_key: POLYWRAP_TEXT_RECORD_PREFIX.to_string() + text_record_key,
    })
}

fn _try_resolve_uri(
    args: &ArgsTryResolveUri,
    env: Option<Env>,
    get_resolver: &dyn Fn(&ArgsGetResolver) -> Result<String, String>,
    get_text_record: &dyn Fn(&ArgsGetTextRecord) -> Result<String, String>,
) -> Result<Option<UriResolverMaybeUriOrManifest>, String> {
    let text_record_info = parse_uri(args);

    if let None = text_record_info {
        return Ok(None);
    }

    let TextRecordInfo {
        network_name,
        carry_over_path,
        domain,
        text_record_key,
    } = text_record_info.unwrap();

    let registry_address = match env {
        Some(vars) => vars
            .registry_address
            .unwrap_or(DEFAULT_ENS_REGISTRY_ADDRESS.to_string())
            .clone(),
        None => DEFAULT_ENS_REGISTRY_ADDRESS.to_string(),
    };

    let resolver_address = match get_resolver(&ArgsGetResolver {
        registry_address: registry_address.to_string(),
        domain: domain.to_string(),
        connection: network_to_connection(network_name.clone()),
    }) {
        Ok(value) => value,
        Err(_) => panic!(
            "Error getting resolver address for registry: {}",
            registry_address
        ),
    };

    let text_record = match get_text_record(&ArgsGetTextRecord {
        domain: domain.to_string(),
        resolver_address,
        key: text_record_key.to_string(),
        connection: network_to_connection(network_name.clone()),
    }) {
        Ok(value) => value,
        Err(_) => panic!(
            "Error getting text record({}) for domain: {}",
            text_record_key, domain
        ),
    };

    if carry_over_path.is_empty() {
        Ok(redirect(text_record))
    } else {
        Ok(redirect(text_record + "/" + &carry_over_path))
    }
}

fn network_to_connection<T: Into<String>>(network_name: T) -> Option<ENSEthersConnection> {
    Some(ENSEthersConnection {
        network_name_or_chain_id: Some(network_name.into()),
        node: None,
    })
}

fn redirect<T: Into<String>>(uri: T) -> Option<UriResolverMaybeUriOrManifest> {
    Some(UriResolverMaybeUriOrManifest {
        uri: Some(uri.into()),
        manifest: None,
    })
}
#[cfg(test)]
mod tests {
    pub use crate::wrap::*;
    use crate::{parse_uri, TextRecordInfo, _try_resolve_uri};

    use self::imported::ArgsGetTextRecord;

    #[test]
    fn path_parse() {
        assert_parse_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "domain.eth:some_key".to_string(),
            },
            &Some(TextRecordInfo {
                network_name: "mainnet".to_string(),
                carry_over_path: "".to_string(),
                domain: "domain.eth".to_string(),
                text_record_key: "wrap/some_key".to_string(),
            }),
        );
    }

    #[test]
    fn path_parse_netowork() {
        assert_parse_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "domain.eth:some_key".to_string(),
            },
            &Some(TextRecordInfo {
                network_name: "mainnet".to_string(),
                carry_over_path: "".to_string(),
                domain: "domain.eth".to_string(),
                text_record_key: "wrap/some_key".to_string(),
            }),
        );
    }

    #[test]
    fn network_and_carry_over_path() {
        assert_parse_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "goerli/domain.eth:some_key/dir/wrap.info".to_string(),
            },
            &Some(TextRecordInfo {
                network_name: "goerli".to_string(),
                carry_over_path: "dir/wrap.info".to_string(),
                domain: "domain.eth".to_string(),
                text_record_key: "wrap/some_key".to_string(),
            }),
        );
    }

    #[test]
    fn invalid_authority() {
        assert_parse_uri(
            &ArgsTryResolveUri {
                authority: "non_matched_authority".to_string(),
                path: "goerli/domain.eth:some_key/dir/wrap.info".to_string(),
            },
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "Error getting text record(wrap/invalid_key) for domain: domain.eth")]
    fn not_found() {
        assert_resolve_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "domain.eth:invalid_key".to_string(),
            },
            "wrap/some_key".to_string(),
            "wrap://ens/test.eth".to_string(),
            &None,
        );
    }

    #[test]
    fn valid_redirect() {
        assert_resolve_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "goerli/domain.eth:some_key".to_string(),
            },
            "wrap/some_key".to_string(),
            "ipfs/Qmdasd".to_string(),
            &Some(UriResolverMaybeUriOrManifest {
                uri: Some("ipfs/Qmdasd".to_string()),
                manifest: None,
            }),
        );
    }

    #[test]
    fn valid_redirect_carry_over_path() {
        assert_resolve_uri(
            &ArgsTryResolveUri {
                authority: "ens".to_string(),
                path: "goerli/domain.eth:some_key/dir/wrap.info".to_string(),
            },
            "wrap/some_key".to_string(),
            "ipfs/Qmdasd".to_string(),
            &Some(UriResolverMaybeUriOrManifest {
                uri: Some("ipfs/Qmdasd/dir/wrap.info".to_string()),
                manifest: None,
            }),
        );
    }

    fn assert_resolve_uri(
        args: &ArgsTryResolveUri,
        text_record_key: String,
        text_record_value: String,
        expected_res: &Option<UriResolverMaybeUriOrManifest>,
    ) {
        let result = _try_resolve_uri(&args, None, &|_| Ok("0x123".to_string()), &|args| {
            let ArgsGetTextRecord {
                domain: _,
                resolver_address: _,
                key,
                connection: _,
            } = args;

            if key.to_string() == text_record_key {
                Ok(text_record_value.clone())
            } else {
                Err("".to_string())
            }
        });

        match result {
            Ok(uri) => match uri {
                Some(UriResolverMaybeUriOrManifest {
                    uri: Some(uri),
                    manifest: None,
                }) => match expected_res {
                    Some(expected_res) => match expected_res {
                        UriResolverMaybeUriOrManifest {
                            uri: Some(expected_res),
                            manifest: None,
                        } => {
                            assert_eq!(&uri, expected_res);
                        }
                        _ => {
                            panic!("Unexpected uri");
                        }
                    },
                    None => {
                        panic!("Unexpected uri");
                    }
                },
                Some(UriResolverMaybeUriOrManifest {
                    uri: None,
                    manifest: None,
                }) => match expected_res {
                    Some(UriResolverMaybeUriOrManifest {
                        uri: None,
                        manifest: None,
                    }) => (),
                    _ => panic!("Expected a uri or manifest to be returned"),
                },
                None => match expected_res {
                    None => (),
                    _ => panic!("Expected a null response"),
                },
                _ => assert!(false),
            },
            Err(_) => {
                panic!("Expect an Ok response")
            }
        }
    }

    fn assert_parse_uri(args: &ArgsTryResolveUri, expected_info: &Option<TextRecordInfo>) {
        let text_record_info = parse_uri(&args);

        match text_record_info {
            Some(text_record_info) => match expected_info {
                Some(expected_info) => {
                    let TextRecordInfo {
                        network_name,
                        carry_over_path,
                        domain,
                        text_record_key,
                    } = text_record_info;

                    assert_eq!(network_name, expected_info.network_name);
                    assert_eq!(domain, expected_info.domain);
                    assert_eq!(text_record_key, expected_info.text_record_key);
                    assert_eq!(carry_over_path, expected_info.carry_over_path);
                }
                None => {
                    panic!("expected None, got Some");
                }
            },
            None => match expected_info {
                Some(_) => {
                    panic!("expected Some, got None");
                }
                None => {}
            },
        }
    }
}
