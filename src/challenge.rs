use crate::api_client::ApiClient;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

// Response Types
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResponse {
    pub id: i32,
    pub limit: i32,
    pub payout: Option<PayoutResponse>,
    pub state: String,
    pub blockchain_address: Option<String>,
    pub contract_address: String,
    pub total_pot: Vec<Pot>,
    pub players: Vec<String>, // @todo look for a way to inculde nft players
    pub winnings: Option<Vec<WinningResponse>>,
    pub signatures: Vec<Signature>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChallengeListResponse {
    pub challenges: Vec<ChallengeResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallengeResponse {
    pub challenge_id: i32,
    pub blockchain_address: String,
    pub environment: String,
    pub payout_id: i32,
    pub limit: i32,
    pub chain: String,
    #[serde(rename = "type")]
    pub challenge_type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LockChallengeResponse {
    pub challenge_id: i32,
    pub locking_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelChallengeResponse {
    pub challenge_id: i32,
    pub canceling_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JoinChallengeResponse {
    pub transaction: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LeaveChallengeResponse {
    pub transaction: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeResponse {
    pub challenge_id: i32,
    pub payout: FungibleTokenPayout,
    pub resolving_at: String,
}

// #[derive(Deserialize, Serialize, Clone, Debug)]
// enum ResolveChallengePayout {
//     FungibleTokenPayout(Vec<FungibleTokenPayout>),
//     NonFungibleTokenPayout(Vec<NonFungibleTokenPayout>),
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FungibleTokenPayout {
    pub amount: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NonFungibleTokenPayout {
    pub offering: String,
    pub to: String,
}

// #[derive(Deserialize, Serialize, Clone, Debug)]
// enum Players {
//     StringPlayer(Vec<String>),
//     NftPlayer(Vec<NftPlayer>)
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NftPlayer {
    pub player: String,
    pub offerings: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pot {
    pub mint: String,
    pub amount: String,
    pub ui_amount: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    pub id: i32,
    pub limit: i32,
    pub entry_fee: String,
    pub mediator_rake: String,
    pub provider_rake: String,
    pub chain: String,
    #[serde(rename = "type")]
    pub payout_type: String,
    pub mint: MintResponse,
    pub ui_values: Option<UiValues>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WinningResponse {
    pub to: String,
    pub amount: String,
    pub ui_amount: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub state: String,
    pub signature: String,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UiValues {
    pub entry_fee: String,
    pub mediator_rake: String,
    pub provider_rake: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MintResponse {
    pub blockchain_address: String,
    pub decimal_position: i32,
    pub icon: String,
    pub ticker: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Offering {
    pub mint: String,
    pub amount: i32,
}

// ----------------------------------------- Request Types  ----------------------------------------- //
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallengeRequest {
    pub payout_id: i32,
    pub limit: i32,
    pub is_verified: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinChallengeRequest {
    pub challenge_id: i32,
    pub player_pubkey: String,
    // pub gasless: Option<bool>, @dev should create another struct to include optional fields
    // pub offerings: Option<Offering>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaveChallengeRequest {
    pub challenge_id: i32,
    pub player_pubkey: String,
    // pub gasless: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeRequest {
    pub challenge_id: i32,
    // @dev need to handle nft payout type as well
    pub payout: Vec<FungibleTokenPayout>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResolveChallengeWithOneWinnerRequest {
    pub challenge_id: i32,
    pub winner: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmptyRequestBody<'a> {
    pub placeholder: &'a str,
}

pub struct Challenge {
    api_client: ApiClient,
}

impl Challenge {
    pub fn new(client_api_key: &str) -> Challenge {
        let sub_path: &str = "challenge";
        let api_client: ApiClient = ApiClient::new(client_api_key, &sub_path);
        Challenge { api_client }
    }

    pub async fn get_challenge(&self, challenge_id: i32) -> Result<ChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}", self.api_client.base_path, challenge_id);
        let response: Result<ChallengeResponse, StatusCode> =
            self.api_client.issue_get_request(&full_path).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn get_challenges(&self) -> Result<ChallengeListResponse, StatusCode> {
        let response: Result<ChallengeListResponse, StatusCode> =
            self.api_client.issue_get_request(&self.api_client.base_path).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn create_challenge(
        &self,
        challenge: CreateChallengeRequest,
    ) -> Result<CreateChallengeResponse, StatusCode> {
        let response: Result<CreateChallengeResponse, StatusCode> = self
            .api_client
            .issue_post_request(&self.api_client.base_path, challenge)
            .await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn generate_join(
        &self,
        join_challenge: JoinChallengeRequest,
    ) -> Result<JoinChallengeResponse, StatusCode> {
        let full_path: String = format!(
            "{}/{}/{}",
            self.api_client.base_path, join_challenge.challenge_id, "join"
        );
        let response: Result<JoinChallengeResponse, StatusCode> = self
            .api_client
            .issue_post_request(&full_path, join_challenge)
            .await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn generate_leave(
        &self,
        leave_challenge: LeaveChallengeRequest,
    ) -> Result<LeaveChallengeResponse, StatusCode> {
        let full_path: String = format!(
            "{}/{}/{}",
            self.api_client.base_path, leave_challenge.challenge_id, "leave"
        );

        let response: Result<LeaveChallengeResponse, StatusCode> = self
            .api_client
            .issue_post_request(&full_path, leave_challenge)
            .await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn lock_challenge(
        &self,
        challenge_id: i32,
    ) -> Result<LockChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}/{}", self.api_client.base_path, challenge_id, "lock");
        // @dev look for ways to pass in empty body without using EmptyRequestBody
        // @dev reqwest client.json() now requires a empty struct
        let body: EmptyRequestBody = EmptyRequestBody { placeholder: "" };
        let response: Result<LockChallengeResponse, StatusCode> =
            self.api_client.issue_patch_request(&full_path, body).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn cancel_challenge(
        &self,
        challenge_id: i32,
    ) -> Result<CancelChallengeResponse, StatusCode> {
        let full_path: String = format!("{}/{}/{}", self.api_client.base_path, challenge_id, "cancel");
        // @dev look for ways to pass in empty body without using EmptyRequestBody
        // @dev reqwest client.json() now requires a empty struct
        let body: EmptyRequestBody = EmptyRequestBody { placeholder: "" };

        let response: Result<CancelChallengeResponse, StatusCode> =
            self.api_client.issue_patch_request(&full_path, body).await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn resolve_challenge(
        &self,
        resolve_challenge: ResolveChallengeRequest,
    ) -> Result<ResolveChallengeResponse, StatusCode> {
        let full_path: String = format!(
            "{}/{}/{}",
            self.api_client.base_path, resolve_challenge.challenge_id, "resolve"
        );
        let response: Result<ResolveChallengeResponse, StatusCode> = self
            .api_client
            .issue_patch_request(&full_path, resolve_challenge)
            .await;
        match response {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
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
