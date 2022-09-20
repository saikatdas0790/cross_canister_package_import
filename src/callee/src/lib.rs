use candid::{CandidType, Deserialize};

#[derive(Deserialize, CandidType)]
pub struct UserDetails {
    pub name: String,
    pub power: u32,
}

#[ic_cdk_macros::query]
#[candid::candid_method(query)]
fn greet(details: UserDetails) -> String {
    format!("Hello, {}! Your power is {}", details.name, details.power)
}

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
