pub mod wrap;
use wrap::{*, imported::{ArgsGetResolver, ArgsGetContentHash}};

const DEFAULT_ENS_REGISTRY_ADDRESS: &str = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
const PATH_SEPARATOR: &str = "/";

struct DomainInfo {
    network_name: String,
    carry_over_path: String,
    domain: String
}

fn parse_uri(args: &ArgsTryResolveUri) -> Option<DomainInfo> {
    if args.authority != "ens" {
        return None;
    }

    let path_parts: Vec<&str> = args.path.split(PATH_SEPARATOR).collect();

    if path_parts.len() < 1 {
        return None;
    }

    let domain_or_network = path_parts[0];

    if domain_or_network.is_empty() {
        return None;
    }

    let network_name;
    let domain;
    let carry_over_path;

    if domain_or_network.contains(".eth") {
        network_name = "mainnet";
        domain = domain_or_network;
        carry_over_path = path_parts[1..].join(PATH_SEPARATOR);
    } else if path_parts.len() < 2 {
        return None;
    } else {
        network_name = domain_or_network;
        domain = path_parts[1];
        carry_over_path = path_parts[2..].join(PATH_SEPARATOR);
    }

    Some(
        DomainInfo {
            network_name: network_name.to_string(),
            carry_over_path: carry_over_path,
            domain: domain.to_string()
        }
    )
}

pub fn try_resolve_uri(args: ArgsTryResolveUri, env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    _try_resolve_uri(&args,env, &ENSModule::get_resolver, &ENSModule::get_content_hash)
}

fn _try_resolve_uri(
    args: &ArgsTryResolveUri,
    env: Option<Env>,
    get_resolver: &dyn Fn(&ArgsGetResolver) -> Result<String, String>,
    get_content_hash: &dyn Fn(&ArgsGetContentHash) -> Result<String, String>
) -> Option<UriResolverMaybeUriOrManifest> {
    let domain_info = parse_uri(args);

    if let None = domain_info {
        return None;
    }

    let DomainInfo {
        network_name,
        carry_over_path,
        domain
    } = domain_info.unwrap();

    let registry_address = match env {
        Some(vars) => vars.registry_address.unwrap_or(DEFAULT_ENS_REGISTRY_ADDRESS.to_string()).clone(),
        None => DEFAULT_ENS_REGISTRY_ADDRESS.to_string()
    };

    let resolver_address = match get_resolver(&ArgsGetResolver {
        registry_address,
        domain: domain.to_string(),
        connection: network_to_connection(network_name.clone())
    }) {
        Ok(value) => value,
        Err(_) => return not_found()
    };

    let content_hash = match get_content_hash(&ArgsGetContentHash {
        domain: domain.to_string(),
        resolver_address,
        connection: network_to_connection(network_name.clone())
    }) {
        Ok(value) => value,
        Err(_) => return not_found()
    };

    if content_hash == "0x" {
        return not_found();
    }

    if carry_over_path.is_empty() {
        redirect("ens-contenthash/".to_owned() + &content_hash)
    } else {
        redirect("ens-contenthash/".to_owned() + &content_hash + "/" + &carry_over_path)
    }
}

fn not_found() -> Option<UriResolverMaybeUriOrManifest> {
    Some(UriResolverMaybeUriOrManifest {
        uri: None,
        manifest: None
    })
}

fn network_to_connection<T: Into<String>>(network_name: T) -> Option<ENSEthereumConnection> {
    Some(ENSEthereumConnection {
        network_name_or_chain_id: Some(network_name.into()),
        node: None
    })
}

fn redirect<T: Into<String>>(uri: T) -> Option<UriResolverMaybeUriOrManifest> {
    Some(UriResolverMaybeUriOrManifest {
        uri: Some(uri.into()),
        manifest: None
    })
} 

pub fn get_file(_args: ArgsGetFile, _env: Option<Env>) -> Option<Vec<u8>> {
    None
}
