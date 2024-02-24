use types::{Chain, PayoutType};


pub struct PayoutResponse {
    pub id: f64,
    pub chain: Chain,
    pub entryFee: String,
    pub mediatorRake: String,
    pub providerRake: String,
    pub payoutType: PayoutType 
}

pub async fn getPayouts() -> Result<Vec<PayoutResponse>, StatusCode> {

}