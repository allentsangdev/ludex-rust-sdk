// Import the ApiClient
// use api_client::ApiClient;
use ludex_rust_sdk::api_client::ApiClient;
use reqwest::StatusCode;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct Payout { 
    id: i32,
    chain: String
}


#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct PayoutResponse {
    pub payouts: Vec<Payout>,
}

#[tokio::main]
async fn main() {
    // Example usage of ApiClient constructor/new function
    let org_api_key = "b91326ef-a39d-49c3-bf7d-3a6eec416294";
    let client = ApiClient::new(org_api_key);

    // Now, you can use `client` to make API calls
    let payout: Result<PayoutResponse, StatusCode> = client.issue_get_request("/payout").await;

    match payout {
        Ok(r) => {
            println!("Success: {:?}", r);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }






}
