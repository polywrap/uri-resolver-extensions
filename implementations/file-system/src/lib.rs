pub mod wrap;
use wrap::{
    *,
    imported::{
        ArgsGetFile,
        ArgsExists,
        ArgsReadFile
    }
};
use std::path::Path;

const MANIFEST_SEARCH_PATTERN: &str = "wrap.info";

pub fn try_resolve_uri(args: ArgsTryResolveUri, _env: Option<Env>) -> Option<UriResolverMaybeUriOrManifest> {
    if args.authority != "fs" && args.authority != "file" {
        return None;
    }

    let base_path = Path::new(&args.path);
    let manifest_path = base_path
        .join(MANIFEST_SEARCH_PATTERN)
        .as_path()
        .to_str()
        .unwrap()
        .to_string();

    let exists_result = FileSystemModule::exists(&ArgsExists {
        path: manifest_path.clone()
    });

    match exists_result {
        Ok(exists) => {
            match exists {
                false => panic!("Manifest not found"),
                true => {
                    let bytes = FileSystemModule::read_file(&ArgsReadFile {
                        path: manifest_path
                    });
                
                    match bytes {
                        Ok(bytes) => {
                            Some(UriResolverMaybeUriOrManifest {
                                manifest: Some(bytes),
                                uri: None
                            })
                        },
                        Err(_) => {
                            panic!("Error reading manifest");
                        }
                    }
                }
            }
        },
        Err(_) => {
            panic!("Error checking if manifest exists");
        }
    }
}

pub fn get_file(args: ArgsGetFile, _env: Option<Env>) -> Option<Vec<u8>> {
    let res = FileSystemModule::read_file(&ArgsReadFile {
        path: args.path
    });

    if res.is_err() {
        return None;
    }

    Some(res.unwrap())
}
