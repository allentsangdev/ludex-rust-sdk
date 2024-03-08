#![allow(dead_code)]

pub enum Chain {
    Solana,
    Avalanche,
}

 impl Chain {
    fn as_str(&self) -> &'static str {
        match self {
            Chain::Solana => "SOLANA",
            Chain::Avalanche => "AVALANCHE",
        }
    }
}

pub enum PayoutType {
    NATIVE,
    FT,
    NFT,
}

 impl PayoutType {
    fn as_str(&self) -> &'static str {
        match self {
            PayoutType::NATIVE => "NATIVE",
            PayoutType::FT => "FT",
            PayoutType::NFT => "NFT",
        }
    }
}

// Request response typess
#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct PayoutResponse { 
    id: i32,
    chain: String
}

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct PayoutListResponse {
    pub payouts: Vec<PayoutResponse>,
}