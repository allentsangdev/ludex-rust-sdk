// @todo should not be public, remove pub after testing
pub mod api_client;
pub mod payout;
pub mod client;
pub mod types;

// // let client = api_client::ApiClient::new()


// // struct OrganizationScoped {
// //     client: Client, <-- an api_client with org api key
// //     payout: Payout, <-- an api_client with org api key
// // }

// // impl OrganizationScoped {
// //     fn new(orgainiationApiKey: String) -> OrganizationScoped {
// //         OrganizationScoped {
// //             client: "",
// //             payout: "",
// //         }
// //     }
// // }