//! # ludex-rust-sdk
//!
//! The `ludex-rust-sdk` crate provides a Rust wrapper (SDK) for the [Ludex Protocol API](https://docs.ludex.gg/).
//!
//! ## Modules
//!
//! - `payout`: Module for interacting with payout-related functionality.
//! - `client`: Module for managing client-related operations.
//! - `challenge`: Module for handling challenge-related operations.
//! - `vault`: Module for managing vault-related operations.
//! - `types`: Module containing various types and structs used throughout the SDK.
//!
//! ## Structs
//!
//! ### `OrganizationScoped<'a>`
//!
//! A struct representing an organization-scoped context for interacting with the Ludex Protocol API.
//!
//! - `client`: Instance of `client::Client<'a>` for making client-related API calls.
//! - `payout`: Instance of `payout::Payout<'a>` for handling payout-related functionality.
//!
//! #### Methods
//!
//! - `new(org_api_key: String) -> OrganizationScoped<'a>`: Constructs a new `OrganizationScoped` instance.
//!
//! ### `ClientScoped<'a>`
//!
//! A struct representing a client-scoped context for interacting with the Ludex Protocol API.
//!
//! - `challenge`: Instance of `challenge::Challenge<'a>` for managing challenge-related operations.
//! - `vault`: Instance of `vault::Vault<'a>` for managing vault-related functionality.
//!
//! #### Methods
//!
//! - `new(client_api_key: String) -> ClientScoped<'a>`: Constructs a new `ClientScoped` instance.
//!
//! ## Examples
//!
//! ```rust
//! use ludex_rust_sdk::{OrganizationScoped, ClientScoped};
//!
//! fn main() {
//!     // Initialize an OrganizationScoped instance with an organization API key
//!     let org_scoped = OrganizationScoped::new("ORG_API_KEY".to_string());
//!
//!     // Initialize a ClientScoped instance with a client API key
//!     let client_scoped = ClientScoped::new("CLIENT_API_KEY".to_string());
//! }
//! ```

#[doc(hidden)]
mod api_client;
pub mod payout;
pub mod client;
pub mod challenge;
pub mod vault;
pub mod types;

pub struct OrganizationScoped<'a> {
    pub client: client::Client<'a>,
    pub payout: payout::Payout<'a>,
}

impl<'a> OrganizationScoped<'a> {
    pub fn new(org_api_key: String) -> OrganizationScoped<'a> {
        let client = client::Client::new(org_api_key.clone());
        let payout = payout::Payout::new(org_api_key.clone());
        
        OrganizationScoped {
            client,
            payout,
        }
    }
}

pub struct ClientScoped<'a> {
    pub challenge: challenge::Challenge<'a>,
    pub vault: vault::Vault<'a>,
}

impl<'a> ClientScoped<'a> {
    pub fn new(client_api_key: String) -> ClientScoped<'a> {
        let challenge = challenge::Challenge::new(client_api_key.clone());
        let vault = vault::Vault::new(client_api_key.clone());
        
        ClientScoped {
            challenge,
            vault,
        }
    }
}