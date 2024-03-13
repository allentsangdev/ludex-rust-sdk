use ludex_rust_sdk::vault::Vault;
use ludex_rust_sdk::ClientScoped;
use once_cell::sync::Lazy;
use std::env;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_CLIENT_SCOPED: Lazy<ClientScoped> = Lazy::new(|| {
    let api_key = env::var("LUDEX_CLIENT_API_KEY").expect("LUDEX_CLIENT_API_KEY environment variable not set");
    ClientScoped::new(api_key)
});