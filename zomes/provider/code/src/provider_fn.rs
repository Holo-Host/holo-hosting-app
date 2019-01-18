use crate::entry::app_config::AppConfig;
use crate::entry::app_details::AppDetails;

use hdk::{
    self,
    holochain_core_types::{
        entry::Entry,
        hash::HashString,
        cas::content::Address,
    },
    error::ZomeApiResult,
};

pub fn handle_register_app(ui_hash:HashString,dna_list:Vec<HashString>) -> ZomeApiResult<Address>{
    // TODO
    // Validation before commiting to the DHT
    // Check if user is verified
    // Check if all the hashes exist in the HCHC

    let app_entry = Entry::App("app_config".into(), AppConfig{
        ui_hash,
        dna_list
        }.into());

    utils::commit_and_link(&app_entry, &hdk::AGENT_ADDRESS, "registered_tag")
}

pub fn handle_add_app_details(app_details:AppDetails,app_hash:Address) -> ZomeApiResult<Address>{

    let app_details_entry = Entry::App("app_details".into(), app_details.into());

    utils::commit_and_link(&app_details_entry, &app_hash, "details_tag")
}
