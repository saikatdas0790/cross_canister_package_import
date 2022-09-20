// use candid::Principal;
// use ic_cdk::api::call;
use callee::UserDetails;

#[ic_cdk_macros::update]
#[candid::candid_method(update)]
fn greet() {
    let details_1 = callee::UserDetails {
        name: "Alice".to_string(),
        power: 9001,
    };
    let details_2 = utils::UserDetails {
        name: "Bob".to_string(),
        power: 9001,
    };
    // call::call(
    //     Principal::from_text(option_env!("CANISTER_ID_callee").unwrap()).unwrap(),
    //     "greet",
    //     (UserDetails {
    //         name: "Saikat".to_string(),
    //         power: 100,
    //     },),
    // )
}

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
