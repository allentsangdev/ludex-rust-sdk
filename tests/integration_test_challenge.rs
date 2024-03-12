use ludex_rust_sdk::challenge::Challenge;
use ludex_rust_sdk::ClientScoped;
use once_cell::sync::Lazy;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_CLIENT_SCOPED: Lazy<ClientScoped> = Lazy::new(|| {
    let api_key = "b079c93f-3afc-42f5-bd5a-7fe131a03e95".to_string();
    ClientScoped::new(api_key)
});

#[tokio::test]
async fn get_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;
    let response = challenge_scoped.get_challenge(88302).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{}", e)
    }

    assert!(response.is_ok());
}
