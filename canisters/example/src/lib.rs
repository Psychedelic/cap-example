use ic_certified_map::{fork, fork_hash, AsHashTree, HashTree};
use ic_kit::candid::{candid_method, export_service};
use ic_kit::interfaces::{management, Method};
use ic_kit::macros::*;
use cap_sdk::{handshake, insert, Event, IndefiniteEventBuilder};
use cap_sdk_core::transaction::EventStatus;
use ic_kit::{ic, Principal};
use serde::Serialize;
use std::collections::BTreeSet;

mod upgrade;

// required by crate::Data;
#[derive(Serialize)]
struct Data {
    cap_root: Principal,
    owner: Principal,
}

impl Default for Data {
    fn default() -> Self
{
        Self {
            cap_root: Principal::management_canister(),
            owner: Principal::management_canister(),
        }
    }
}

#[init]
// can init function be async?
async fn init() {
    let data = ic::get_mut::<Data>();
    data.owner = ic::caller();

    let arg = management::CreateCanisterArgument { settings: None };
    let (res,) = management::CreateCanister::perform_with_payment(
        Principal::management_canister(),
        (arg,),
        0,
    )
    .await
    .expect("Failed to create the canister.");
    let canister_id = res.canister_id;

    data.cap_root = canister_id;

    let cycles_to_give = 1000000000000;
    handshake(cycles_to_give);
}

pub struct TransactionDetails {
    foo: String,
    bar: u64,
}

async fn mint() {
    let transaction_details = TransactionDetails {
        foo: String::from("foo"),
        bar: 42
    };

    let event = IndefiniteEventBuilder::new()
    .caller(Principal::anonymous())
    .operation(String::from("mint"))
    .details(transaction_details)
    .build()
    .unwrap();
}

// async fn transfer() {
//     let insert_res = insert(event_into_indefinite_event(
//         &TryInto::<Event>::try_into(tx_record.clone())
//             .expect("unable to convert TxRecord to Event"),
//     ))
//     .await
//     .map(|tx_id| Nat::from(tx_id))
//     .map_err(|err| TxError::Other);

//     if insert_res.is_err() {
//         tx_log().tx_records.push_back(tx_record);
//     }

//     insert_res
// }

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
