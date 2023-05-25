use cid::{Cid};

pub fn is_ipfs_cid(id: &String) -> bool {
    if let Ok(cid) = Cid::try_from(id.as_str()) {
        true
    } else {
        false
    }
}
