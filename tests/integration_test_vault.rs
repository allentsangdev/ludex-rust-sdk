use ludex_rust_sdk::vault::Vault;
use ludex_rust_sdk::ClientScoped;
use once_cell::sync::Lazy;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_CLIENT_SCOPED: Lazy<ClientScoped> = Lazy::new(|| {
    let api_key = "b079c93f-3afc-42f5-bd5a-7fe131a03e95".to_string();
    ClientScoped::new(api_key)
});