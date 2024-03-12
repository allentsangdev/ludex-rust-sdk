mod api_client;
pub mod payout;
pub mod client;
pub mod types;
pub mod challenge;
pub mod vault;


pub struct OrganizationScoped<'a> {
    pub client: client::Client<'a>,
    pub payout: payout::Payout<'a>,
}

impl<'a> OrganizationScoped<'a> {
    pub fn new(org_api_key: String) -> OrganizationScoped<'a> {
        let client = client::Client::new(org_api_key.clone());
        let payout = payout::Payout::new(org_api_key.clone());
        
        OrganizationScoped {
            client,
            payout,
        }
    }
}

pub struct ClientScoped<'a> {
    pub challenge: challenge::Challenge<'a>,
    pub vault: vault::Vault<'a>,
}

impl<'a> ClientScoped<'a> {
    pub fn new(client_api_key: String) -> ClientScoped<'a> {
        let challenge = challenge::Challenge::new(client_api_key.clone());
        let vault = vault::Vault::new(client_api_key.clone());
        
        ClientScoped {
            challenge,
            vault,
        }
    }
}