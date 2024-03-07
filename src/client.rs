use crate::api_client::ApiClient;
use reqwest::StatusCode;

#[derive(serde::Deserialize, Clone)]
struct ClientResponse {
    /** client id */
    id: i32,
    /** name of client */
    name: String,
    /** Open challenge limit */
    open_challenge_limit: i32,
    //Clients wallets (one per chain)
    //   wallets: ClientWallet[];
}

pub struct Client<'a> {
    api_client: ApiClient<'a>,
    base_path: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(org_api_key: &str) -> Client {
        let client = ApiClient::new(org_api_key);
        Client {
            api_client: client,
            base_path: "/client",
        }
    }

    pub async fn get_client(&self, client_id: i32) -> Result<Vec<ClientResponse>, StatusCode> {
        let full_url: String = format!("{}{}", self.base_path, client_id);
        let response: Result<Vec<ClientResponse>, StatusCode> =
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
