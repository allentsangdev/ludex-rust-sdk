use crate::api_client::ApiClient;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;

// Response Types
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResponse {
    id: i32,
    limit: i32,
    payout: PayoutResponse,
    state: String,
    blockchain_address: Option<String>,
    contract_address: String,
    total_pot: Vec<Pot>,
    players: Players,
    winning: Option<Vec<WinningResponse>>,
    signatures: Vec<Signature>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallengeResponse {
    challenge_id: i32,
    creating_at: String,
    payout_id: i32,
    limit: i32,
    chain: String,
    #[serde(rename = "type")]
    challenge_type: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LockChallengeResponse {
    challenge_id: i32,
    locking_at: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelChallengeResponse {
    challenge_id: i32,
    canceling_at: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JoinChallengeResponse {
    transaction: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaveChallengeResponse {
    transaction: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeResponse {
    challenge_id: i32,
    payout: ResolveChallengePayout,
    resolving_at: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
enum ResolveChallengePayout {
    FungibleTokenPayout(Vec<FungibleTokenPayout>),
    NonFungibleTokenPayout(Vec<NonFungibleTokenPayout>)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FungibleTokenPayout {
    amount: String,
    to: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NonFungibleTokenPayout {
    offering: String,
    to: String
}



#[derive(Deserialize, Serialize, Clone, Debug)]
enum Players {
    StringPlayer(Vec<String>),
    NftPlayer(Vec<NftPlayer>)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftPlayer {
    player: String,
    offerings: Vec<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pot {
    mint: String,
    amount: String,
    ui_amount: String
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    id: i32,
    entry_fee: String,
    mediator_rake: String,
    provider_rake: String,
    chain: String,
    #[serde(rename = "type")]
    payout_type: String,
    mint: MintResponse,
    ui_values: Option<UiValues>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct  WinningResponse {
    player: String,
    amount: String,
    ui_amount: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    state: String,
    signature: String,
    time_stamp: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UiValues {
    entry_fee: String,
    mediator_rake: String,
    provider_rake: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MintResponse {
    blockchain_response: String,
    decimal_position: i32,
    icon: String,
    ticker: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Offering {
    mint: String,
    amount: i32
}

// ----------------------------------------- Request Types  ----------------------------------------- //
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct  CreateChallengeRequest {
    payout: i32,
    limit: i32,
    is_verified: bool
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinChallengeRequest {
    challenge_id: i32,
    player_pubkey: String,
    gasless: Option<bool>,
    offerings: Option<Offering>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaveChallengeRequest {
    challenge_id: i32,
    player_pubkey: String,
    gasless: Option<bool>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeRequest {
    challenge_id: i32,
    payout: ResolveChallengePayout
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeWithOneWinnerRequest {
    challenge_id: i32,
    winner: String
}



pub struct Challenge<'a> {
    api_client: ApiClient<'a>,
    base_path: &'a str,
}

impl<'a> Challenge<'a> {
    pub fn new(client_api_key: String) -> Challenge<'a> {
        let api_client: ApiClient = ApiClient::new(client_api_key);
        Challenge {
            api_client,
            base_path: "/challenge",
        }
    }

    pub async fn get_challenge(&self, challenge_id: i32) -> Result<ChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}", self.base_path, challenge_id);
        let response: Result<ChallengeResponse, StatusCode> = self.api_client.issue_get_request(&full_path).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn get_challenges(&self) -> Result<Vec<ChallengeResponse>, StatusCode> {
        let response: Result<Vec<ChallengeResponse>, StatusCode> = self.api_client.issue_get_request(self.base_path).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn create_challenge(&self, challenge: CreateChallengeRequest) -> Result<CreateChallengeResponse, StatusCode> {
        let response: Result<CreateChallengeResponse, StatusCode> = self.api_client.issue_post_request(self.base_path, challenge).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn generate_join(&self, join_challenge: JoinChallengeRequest) -> Result<JoinChallengeResponse, StatusCode> {
        let response: Result<JoinChallengeResponse, StatusCode> = self.api_client.issue_post_request(self.base_path, join_challenge).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn generate_leave(&self, leave_challenge: LeaveChallengeRequest) -> Result<LeaveChallengeResponse, StatusCode> {
        let response: Result<LeaveChallengeResponse, StatusCode> = self.api_client.issue_post_request(self.base_path, leave_challenge).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn lock_challenge(&self, challenge_id: i32) -> Result<LockChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}/{}", self.base_path, challenge_id, "lock");
        let response: Result<LockChallengeResponse, StatusCode> = self.api_client.issue_patch_request(&full_path, "").await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn cancel_challenge(&self, challenge_id: i32) -> Result<CancelChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}/{}", self.base_path, challenge_id, "cancel");
        let response: Result<CancelChallengeResponse, StatusCode> = self.api_client.issue_patch_request(&full_path, "").await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    pub async fn resolve_challenge(&self, resolve_challenge: ResolveChallengeRequest) -> Result<ResolveChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}/{}", self.base_path, resolve_challenge.challenge_id, "resolve");
        let response: Result<ResolveChallengeResponse, StatusCode> = self.api_client.issue_patch_request(&full_path, resolve_challenge).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }

    // pub async fn resolve_challenge_with_one_player(&self, resolve_challenge: ResolveChallengeWithOnePlayerRequest) -> Result<ResolveChallengeResponse, StatusCode> {
    //     let full_path: String = format!("{}/{}/{}", self.base_path, resolve_challenge.challenge_id, "resolve");
    //     let response: Result<ResolveChallengeResponse, StatusCode> = self.api_client.issue_patch_request(&full_path, resolve_challenge).await;
    //     match response {
    //         Ok(r) => Ok(r),
    //         Err(e) => Err(e)
    //     }
    // }
}
