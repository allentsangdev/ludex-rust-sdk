#![allow(dead_code)]
use crate::api_client::ApiClient;
use reqwest::{Body, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    id: i32,
    name: String,
    open_challenge_limit: i32,
    wallets: Vec<ClientWallet>
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct OpenChallengeCountResponse {
    count: i32,
    limit: i32
}

#[derive(serde::Deserialize, Clone)]
pub struct DeleteClientResponse {
    id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CreateClientRequest {
    pub name: String
}

impl CreateClientRequest {
    // a helper function to parse the struct to a reqwest Body type
    pub fn to_request_body(&self) -> Body {
        let json_string = serde_json::to_string(&self).unwrap();
        Body::from(json_string)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientWallet {
    chain: String,
    address: String
}

impl ClientWallet {
    fn to_request_body(&self) -> Body {
        let json_string = serde_json::to_string(&self).unwrap();
        Body::from(json_string)
}
}


pub struct Client<'a> {
    api_client: ApiClient<'a>,
    base_path: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(org_api_key: String) -> Client<'a> {
        let client = ApiClient::new(org_api_key);
        Client {
            api_client: client,
            base_path: "/client",
        }
    }

    pub async fn get_client(&self, client_id: i32) -> Result<ClientResponse, StatusCode> {
        let full_url: String = format!("{}/{}", self.base_path, client_id);
        let response: Result<ClientResponse, StatusCode> =
            self.api_client.issue_get_request(&full_url).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_clients(&self) -> Result<Vec<ClientResponse>, StatusCode> {
        let response: Result<Vec<ClientResponse>, StatusCode> =
            self.api_client.issue_get_request(&self.base_path).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn create_client(&self, client: CreateClientRequest) -> Result<ClientResponse, StatusCode> {
        let request_body: Body = client.to_request_body();
        
        let response: Result<ClientResponse, StatusCode> =
            self.api_client.issue_post_request(&self.base_path, request_body).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_open_challenge_count(&self, client_id: i32) -> Result<OpenChallengeCountResponse, StatusCode> {
        let full_path: String = format!("{}/{}{}", self.base_path, client_id, "/open-challenge-count");
        
        let response: Result<OpenChallengeCountResponse, StatusCode> =
            self.api_client.issue_get_request(&full_path).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }


    pub async fn update_client_wallet(&self, client_id: i32, wallet: ClientWallet) -> Result<ClientResponse, StatusCode> {
        let full_path: String = format!("{}{}{}", self.base_path, client_id, "/wallet");
        let request_body: Body = wallet.to_request_body();
        
        let response: Result<ClientResponse, StatusCode> =
            self.api_client.issue_patch_request(&full_path, request_body).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn delete_client(&self, client_id: i32) -> Result<DeleteClientResponse, StatusCode> {
        let full_path: String = format!("{}{}", self.base_path, client_id);
        
        let response: Result<DeleteClientResponse, StatusCode> =
            self.api_client.issue_delete_request(&full_path).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }
}
