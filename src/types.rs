pub enum Chain {
    Solana,
    Avalanche,
}

pub impl Chain {
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

pub impl PayoutType {
    fn as_str(&self) -> &'static str {
        match self {
            PayoutType::NATIVE => "NATIVE",
            PayoutType::FT => "FT",
            PayoutType::NFT => "NFT",
        }
    }
}
