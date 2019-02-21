+use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::error::HolochainError,
    holochain_core_types::json::JsonString,
};

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    cas::content::Address,
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Invite {
    pub name: String,
    pub description: String
    pub host: String
}


pub fn private_invite_definition() -> ValidatingEntryType {
    entry!(
        name: "private_invite",
        description: "Sends private invite to members of choice",
        host: //References Sender - host of event
        sharing: Sharing::Private,
        native_type: Invite,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_invite: Invite, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [    //I need help better understanding links
            to!(
                "%agent_id",
                tag: "has_member",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                tag: "member_of",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            ),
            to!(
                "message",
                tag: "message_in",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}