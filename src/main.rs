// Import the ApiClient
// use api_client::ApiClient;
use ludex_rust_sdk::api_client::ApiClient;
use ludex_rust_sdk::types::PayoutListResponse;
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    let org_api_key = "b91326ef-a39d-49c3-bf7d-3a6eec416294";
    let client = ApiClient::new(org_api_key);

    // Test for payout mod
    let payouts: Result<PayoutListResponse, StatusCode> = client.issue_get_request("/payout").await;
    assert!(payouts.is_ok()); // Check if the result is Ok
}
