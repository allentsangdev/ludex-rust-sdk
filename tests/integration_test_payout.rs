use ludex_rust_sdk::payout::Payout;
use ludex_rust_sdk::OrganizationScoped;
use once_cell::sync::Lazy;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_ORG_SCOPED: Lazy<OrganizationScoped> = Lazy::new(|| {
    let api_key = "b91326ef-a39d-49c3-bf7d-3a6eec416294".to_string();
    OrganizationScoped::new(api_key)
});

#[tokio::test]
async fn get_payouts() {
    let payout_scoped: &Payout = &LUDEX_ORG_SCOPED.payout;
    let response = payout_scoped.get_payouts().await;

    // run cargo test -- --nocapture to print during Rust tests
    match &response {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{}", e)
    }

    assert!(response.is_ok());
}

#[tokio::test]
async fn get_payout() {
    let payout_scoped: &Payout = &LUDEX_ORG_SCOPED.payout;
    let response = payout_scoped.get_payouts().await;

    // run cargo test -- --nocapture to print during Rust tests
    match &response {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{}", e)
    }

    assert!(response.is_ok());
}