use hdk::error::ZomeApiResult;
use hdk::AGENT_ADDRESS;
use hdk::holochain_core_types::{
    hash::HashString,
    entry::Entry,
    cas::content::Address,
    json::RawString,
};

use crate::invite::{
    Invite,
};

use crate::utils;
use crate::message;

pub fn handle_send_invite(
    name: String,
    description: String,
    invitees: Vec<Address>, //Need a way to select invitees from a list
) -> ZomeApiResult<Address> {

    let invite = invite{event_name, description, host};
}
