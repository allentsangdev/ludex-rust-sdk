#![allow(dead_code)]
use crate::api_client::ApiClient;
use reqwest::StatusCode;

/*
    1. Add payout filters
    2. Review derive and traits
*/

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    pub id: i32,
    pub chain: String,
    pub entry_fee: String,
    pub mediator_rake: String,
    pub provider_rake: String,
    #[serde(rename = "type")]
    pub payout_type: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct PayoutListResponse {
    pub payouts: Vec<PayoutResponse>,
}

pub struct Payout {
    api_client: ApiClient,
}

impl Payout {
    pub fn new(org_api_key: &str) -> Payout {
        let sub_path: &str = "payout";
        let client = ApiClient::new(org_api_key, &sub_path);
        Payout { api_client: client }
    }

    pub async fn get_payouts(&self) -> Result<PayoutListResponse, StatusCode> {
        let response: Result<PayoutListResponse, StatusCode> =
            self.api_client.issue_get_request(&self.api_client.base_path).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_payout(&self, id: i32) -> Result<PayoutResponse, StatusCode> {
        let full_url: String = format!("{}{}", self.api_client.base_path, id);

        let response: Result<PayoutResponse, StatusCode> =
            self.api_client.issue_get_request(&full_url).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }
}
