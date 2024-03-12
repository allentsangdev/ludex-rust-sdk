use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::api_client::ApiClient;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Chain {
    SOLANA,
    AVALANCHE
}

impl Chain {
    fn to_string(&self) -> String {
        match self {
            Chain::SOLANA => String::from("SOLANA"),
            Chain::AVALANCHE => String::from("AVALANCHE")
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultResponse {
   name: String,
   blockchain_address: Option<String>,
   chain: Chain,
   fee_recipient: String

}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTransactionResponse {
    transaction_id: i32,
    transaction: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TransactionResponse {
    id: i32,
    signature: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultRequest {
    name: Option<String>,
    fee_recipient: Option<String>,
    chain: Chain
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVaultRequest {
    name: Option<String>,
    fee_recipient: Option<String>,
    chain: Chain
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTransactionRequest {
    chain: Chain,
    #[serde(rename = "type")]
    reedeem_type:  RedeemType,
    gasless: bool,
    player_public_key: String,
    amount_given: Option<i32>,
    amount_redeemed: i32, 
    overide_fee_recipient_pubkey: Option<String>,
    pay_mint: Option<String>,
    receive_mint: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]

pub enum RedeemType {
    Native,
    NativeForTokens,
    TokensForNative,
    TokensForTokens
}

impl RedeemType {
    fn to_string(&self) -> String {
        match self {
            RedeemType::Native => String::from("native"),
            RedeemType::NativeForTokens => String::from("nativeForTokens"),
            RedeemType::TokensForNative => String::from("tokensForNative"),
            RedeemType::TokensForTokens  => String::from("tokensForTokens")
        }
    }
}

pub struct Vault<'a> {
    api_client: ApiClient<'a>,
    base_path: &'a str
}

impl<'a> Vault<'a> {
    pub fn new(client_api_key: String) -> Vault<'a> {
        let api_client: ApiClient = ApiClient::new(client_api_key);
        Vault {
            api_client,
            base_path: "/vault"
        }
    }
    
    pub async fn get_vault(&self, chain: Chain) -> Result<VaultResponse, StatusCode> {
        let full_path: String = format!("{}/{}", self.base_path, chain.to_string());
        let response: Result<VaultResponse, StatusCode> = self.api_client.issue_get_request(&full_path).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn create_vault(&self, vault: CreateVaultRequest) -> Result<VaultResponse, StatusCode> {
        
        
    }

    pub async fn update_vault(&self, vault: UpdateVaultRequest) -> Result<VaultResponse, StatusCode> {
        
    }

    pub async fn generate_transaction(&self, transaction: GenerateTransactionRequest) -> Result<GenerateTransactionResponse, StatusCode> {
        
    }

    pub async fn get_transactions(&self, chain: Chain) -> Result<Vec<TransactionResponse>, StatusCode> {
        
    }

    pub async fn get_transaction(chain: Chain, transaction_id: &str) -> Result<TransactionResponse, StatusCode> {
        
    }
}