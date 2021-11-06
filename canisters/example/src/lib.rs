use ic_certified_map::{fork, fork_hash, AsHashTree, HashTree};
use ic_kit::candid::{candid_method, export_service};
use ic_kit::macros::*;
use ic_kit::{ic, Principal};
use serde::Serialize;
use std::collections::BTreeSet;

mod upgrade;

// required by crate::Data;
#[derive(Serialize)]
struct Data {
}

impl Default for Data {
    fn default() -> Self
{
        Self {
        }
    }
}

// needed to export candid on save
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("candid");
        write(dir.join("root.did"), export_candid()).expect("Write failed.");
    }
}
