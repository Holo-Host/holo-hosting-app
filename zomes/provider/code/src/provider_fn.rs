use crate::app_config::AppConfig;
use hdk::{
    holochain_core_types::{
        entry::Entry,
        hash::HashString,
        cas::content::Address,
    },
    error::ZomeApiResult,
};

pub fn handle_register_app(ui_hash:HashString,dna_list:Vec<HashString>) -> ZomeApiResult<Address>{
    // Validation before commiting to the DHT
    // Check if user is verified
    // Check if all the hashes exist in the HCHC
    let post_entry = Entry::App("app_config".into(), AppConfig{
        ui_hash,
        dna_list,
        }.into());

    let address = hdk::commit_entry(&post_entry)?;
    
    Ok(address)
}