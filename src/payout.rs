use crate::api_client:: ApiClient;
use reqwest::StatusCode;

/* 
    1. Add payout filters
    2. Review derive and traits
*/

#[derive(serde::Deserialize)]
#[derive(Clone)]
struct PayoutResponse {
    pub id: i32,
    pub chain: String,
    pub entry_fee: String,
    pub mediator_rake: String,
    pub provide_rake: String,
    pub payout_type: String
}

pub struct PayoutListResponse {
    pub payouts: Vec<PayoutResponse>,
}

pub struct Payout<'a> {
    api_client: ApiClient<'a>,
    base_path: &'a str
}

impl<'a> Payout<'a> {
    fn new(org_api_key:&str) -> Payout {
        let client = ApiClient::new(org_api_key);
        Payout {
            api_client: client,
            base_path: "/payout"
        }
    }

    async fn get_payouts(&self) -> Result<Vec<PayoutResponse>, StatusCode> {
        let response : Result<Vec<PayoutResponse>, StatusCode> = self.api_client.issue_get_request("/payout").await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    async fn get_payout(&self, id: i32) -> Result<PayoutResponse, StatusCode> {
        let full_url: String = format!("{}{}", &self.base_path, id);

        let response : Result<PayoutResponse, StatusCode> = self.api_client.issue_get_request(&full_url).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }
    
}