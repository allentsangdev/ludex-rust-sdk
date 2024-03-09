use ludex_rust_sdk::client::Client;
use ludex_rust_sdk::OrganizationScoped;
use once_cell::sync::Lazy;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_ORG_SCOPED: Lazy<OrganizationScoped> = Lazy::new(|| {
    let api_key = "b91326ef-a39d-49c3-bf7d-3a6eec416294".to_string();
    OrganizationScoped::new(api_key)
});

#[tokio::test]
async fn get_client() {
    
    let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;
    let response = client_scoped.get_client(1072).await; // <-- should return coinflip V2
    
    // run cargo test -- --nocapture to print during Rust tests
    match &response {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{}", e)
    }

    assert!(response.is_ok()); 
}

#[tokio::test]
async fn get_clients() {
    
    let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;
    let response = client_scoped.get_clients().await; // Await the asynchronous response
    
    // run cargo test -- --nocapture to print during Rust tests
    // match &response {
    //     Ok(r) => println!("{:?}",r),
    //     Err(e) => println!("{}", e)
    // }

    assert!(response.is_ok()); 
}
