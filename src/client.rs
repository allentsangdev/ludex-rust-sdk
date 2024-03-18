#![allow(dead_code)]
use crate::api_client::ApiClient;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    id: i32,
    name: String,
    open_challenge_limit: i32,
    wallets: Vec<ClientWallet>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct OpenChallengeCountResponse {
    count: i32,
    limit: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DeleteClientResponse {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientWallet {
    pub chain: String,
    pub address: String,
}

pub struct Client {
    api_client: ApiClient
}

impl Client {
    pub fn new(org_api_key: &str) -> Client {
        let sub_path: &str = "client";
        let client = ApiClient::new(org_api_key, &sub_path);
        Client {
            api_client: client,
        }
    }

    pub async fn get_client(&self, client_id: i32) -> Result<ClientResponse, StatusCode> {
        let full_url: String = format!("{}/{}", self.api_client.base_path, client_id);
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
            self.api_client.issue_get_request(&self.api_client.base_path).await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn create_client(
        &self,
        client: CreateClientRequest,
    ) -> Result<ClientResponse, StatusCode> {
        let response: Result<ClientResponse, StatusCode> = self
            .api_client
            .issue_post_request(&self.api_client.base_path, client)
            .await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_open_challenge_count(
        &self,
        client_id: i32,
    ) -> Result<OpenChallengeCountResponse, StatusCode> {
        let full_path: String = format!(
            "{}/{}{}",
            self.api_client.base_path, client_id, "/open-challenge-count"
        );

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

    pub async fn update_client_wallet(
        &self,
        client_id: i32,
        wallet: ClientWallet,
    ) -> Result<ClientResponse, StatusCode> {
        let full_path: String = format!("{}/{}{}", self.api_client.base_path, client_id, "/wallet");

        let response: Result<ClientResponse, StatusCode> = self
            .api_client
            .issue_patch_request(&full_path, wallet)
            .await;

        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn delete_client(&self, client_id: i32) -> Result<DeleteClientResponse, StatusCode> {
        let full_path: String = format!("{}/{}", self.api_client.base_path, client_id);

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
