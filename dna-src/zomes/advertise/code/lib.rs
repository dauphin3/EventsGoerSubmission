#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
    hash::HashString,
    cas::content::Address,
    json::{JsonString},
    error::HolochainError,
};

mod anchor;
mod message;
mod invite;
mod member;
mod utils;

define_zome! {

	entries: [
		message::message_definition(),
    	invite::public_event_definition(),
        member::profile_definition(),
        anchor::anchor_definition()
	]

    genesis: || {
        {
    		Ok(())
        }
    }

	functions: [
		send: {
			inputs: | name: String, invitee: String |,
			outputs: |result: ZomeApiResult<Address>|,
			handler: member::handlers::handle_send_nvite
		}
		
		get_members: {
			inputs: |event_address: HashString|,
			outputs: |result: ZomeApiResult<Vec<Address>>|,
			handler: event::handlers::handle_get_members
		}
		
		write_message: {
			inputs: |event_address: HashString, message: message::MessageSpec|,
			outputs: |result: ZomeApiResult<()>|,
			handler: event::handlers::handle_write_message
		}

	]

	 traits: {
	        hc_public [
	        	send,
	        	get_members,
	        	write_message,
	        ]
	}
 }