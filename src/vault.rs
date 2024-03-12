use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::api_client::ApiClient;

enum Chain {
    SOLANA,
    AVALANCHE
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

pub enum RedeemType {
    Native,
    NativeForTokens,
    TokensForNative,
    TokensForTokens
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
    pub fn get_vault(chain: Chain) -> Result<VaultResponse, StatusCode> {

    }

    pub fn create_vault(vault: CreateVaultRequest) -> Result<VaultResponse, StatusCode> {
        
    }

    pub fn update_vault(vault: UpdateVaultRequest) -> Result<VaultResponse, StatusCode> {
        
    }

    pub fn generate_transaction(transaction: GenerateTransactionRequest) -> Result<GenerateTransactionResponse, StatusCode> {
        
    }

    pub fn get_transactions(chain: Chain) -> Result<Vec<TransactionResponse>, StatusCode> {
        
    }

    pub fn get_transaction(chain: Chain, transaction_id: &str) -> Result<TransactionResponse, StatusCode> {
        
    }
}