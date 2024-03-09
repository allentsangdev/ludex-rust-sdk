mod api_client;
pub mod payout;
pub mod client;
pub mod types;

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