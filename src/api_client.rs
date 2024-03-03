/*
    apiClient should be able to send GET, POST, PATCH, DELETE request
    apiClient should be able to send request with a Authorization header 'Bearer <API_KEY>'
    apiClient should be accept a API Key and Base Path as constructor
*/

/*
    @Todo Review all string types
    @Todo Review how to reference self
    @Todo Review the difference between lib crate and bin crate
    @Todo Enable a test script for apiClient
*/

use reqwest::{header::{self, HeaderValue, HeaderMap}, Client, StatusCode, Response, Error};
use serde::de::DeserializeOwned;

pub struct ApiClient<'a> {
    http_client: Client,
    base_path: &'a str,
}

impl<'a> ApiClient<'a> {
    pub fn new(api_key: &str) -> ApiClient {
        const LUDEX_API: &str = "https://staging-ludex-protocol-api.herokuapp.com/api/v2";
        // const LUDEX_API: &str = "https://api.ludex.gg/api/v2/";
        
        let mut headers = HeaderMap::new();

        let header_value =  HeaderValue::from_str(&format!("Bearer {}", &api_key)).expect("Invalid API Key");

        headers.insert(
            header::AUTHORIZATION,
            header_value,
        );

        // @note using unwrap to handle error
        let http_auth_client: Client = Client::builder().default_headers(headers).build().unwrap();

        ApiClient {
            http_client: http_auth_client,
            base_path: LUDEX_API,
        }
    }

    pub async fn issue_get_request<T>(&self, url: &str) -> Result<T, StatusCode> where
    // to make sure the response from the api call is a JSON deserializable and can be fully owned
    T: DeserializeOwned, {
        let full_url: String = format!("{}{}", &self.base_path, &url);

        let response: Result<Response, Error> = self.http_client.get(&full_url).send().await;

        match &response {
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    // returns a Result enum variant Err 
                    return Err(r.status());
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                return Err(e.status().unwrap());
            }
        }

        // Parse the response body as Json
        let content: Result<T, Error> = response.unwrap().json::<T>().await;

        match content {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("{}", e.to_string());
                Err(e.status().unwrap())
            } 
        }
    }
}
