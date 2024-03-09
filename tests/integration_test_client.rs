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
    let response = client_scoped.get_client(1072).await; // should return coinflip V2

    // run cargo test -- --nocapture to print during Rust tests
    // match &response {
    //     Ok(r) => println!("{:?}",r),
    //     Err(e) => println!("{}", e)
    // }

    assert!(response.is_ok());
}

#[tokio::test]
async fn get_clients() {
    let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;
    let response = client_scoped.get_clients().await;

    // run cargo test -- --nocapture to print during Rust tests
    // match &response {
    //     Ok(r) => println!("{:?}",r),
    //     Err(e) => println!("{}", e)
    // }

    assert!(response.is_ok());
}

// #[tokio::test]
// async fn create_client() {

//     let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;
//     let client_request: CreateClientRequest = CreateClientRequest {
//         name: String::from("Test 3")
//     };

//     let response = client_scoped.create_client(client_request).await;

//     // run cargo test -- --nocapture to print during Rust tests
//     match &response {
//         Ok(r) => println!("{:?}",r),
//         Err(e) => println!("{}", e)
//     }

//     assert!(response.is_ok());
// }

#[tokio::test]
async fn get_open_challenge_count() {
    let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;

    let response = client_scoped.get_open_challenge_count(1072).await;

    // run cargo test -- --nocapture to print during Rust tests
    // match &response {
    //     Ok(r) => println!("{:?}",r),
    //     Err(e) => println!("{}", e)
    // }

    assert!(response.is_ok());
}

// #[tokio::test]
// async fn update_client_wallet() {

//     let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;
//     let wallet: ClientWallet = ClientWallet {
//         chain: String::from("SOLANA"),
//         address: String::from("6ofyGkHGZzFvLqeYuLoV5MVnNGT9yNqkgicCfZpAYGRa")
//     };

//     let response = client_scoped.update_client_wallet(1169, wallet).await;

//     // run cargo test -- --nocapture to print during Rust tests
//     match &response {
//         Ok(r) => println!("{:?}",r),
//         Err(e) => println!("{}", e)
//     }

//     assert!(response.is_ok());
// }

// #[tokio::test]
// async fn delete_client() {

//     let client_scoped: &Client = &LUDEX_ORG_SCOPED.client;

//     let response = client_scoped.delete_client(1169).await;

//     // run cargo test -- --nocapture to print during Rust tests
//     match &response {
//         Ok(r) => println!("{:?}",r),
//         Err(e) => println!("{}", e)
//     }

//     assert!(response.is_ok());
// }
