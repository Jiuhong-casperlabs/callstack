#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{borrow::ToOwned, collections::BTreeMap, string::{String, ToString}, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, system::CallStackElement};

#[no_mangle]
pub extern "C" fn test1() {
    let named_keys = runtime::list_named_keys();

    for (key, _) in named_keys {
         runtime::remove_key(&key);
    }

    let caller = runtime::get_caller();
    runtime::put_key("caller", caller.into());
    let call_stack = runtime::get_call_stack();
    // let a = call_stack.into_iter().rev().nth(1);
    let b = call_stack.into_iter();
    let mut i = 0;
    for ccc in b {
        let mut owned_string: String = "element".to_owned();
        let ttt = i.to_string();
        let borrowed_string: &str = ttt.as_str();
        
        owned_string.push_str(borrowed_string);
        i = i+1;

        match ccc {
            CallStackElement::Session { account_hash } =>  
                    runtime::put_key(owned_string.as_str(),account_hash.into()),
            CallStackElement::StoredSession {account_hash, contract_package_hash, contract_hash} =>
               {
                let mut map2: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("account_hash");
                let key2 = String::from("contract_package_hash");
                let key3 = String::from("contract_hash");

                //store purse into contract named_keys
                map2.insert(key1, account_hash.into());
                map2.insert(key2, contract_package_hash.into());
                map2.insert(key3, contract_hash.into());
                runtime::put_key(owned_string.as_str(), storage::new_uref(map2).into());
            },

            CallStackElement::StoredContract  { contract_package_hash, contract_hash} => 
            {
                let mut map3: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("contract_package_hash");
                let key2 = String::from("contract_hash");

                //store purse into contract named_keys
                map3.insert(key1, contract_package_hash.into());
                map3.insert(key2, contract_hash.into());
                runtime::put_key(owned_string.as_str(), storage::new_uref(map3).into());
            },
                    
        };  
    }
}

#[no_mangle]
pub extern "C" fn call() {
       let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        "test1",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

      let (stored_contract_hash, _) =
        storage::new_contract(counter_entry_points, None, None, None);
    runtime::put_key("CallStackElement_contract", stored_contract_hash.into());
}
