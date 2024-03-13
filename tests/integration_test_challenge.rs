use ludex_rust_sdk::challenge::Challenge;
use ludex_rust_sdk::ClientScoped;
use once_cell::sync::Lazy;
use std::env;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_CLIENT_SCOPED: Lazy<ClientScoped> = Lazy::new(|| {
    let api_key = env::var("LUDEX_CLIENT_API_KEY").expect("LUDEX_CLIENT_API_KEY environment variable not set");
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

#[tokio::test]
async fn get_challenges() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;
    let response = challenge_scoped.get_challenges().await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{}", e)
    }

    assert!(response.is_ok());
}
