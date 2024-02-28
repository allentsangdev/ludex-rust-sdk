/*
    apiClient should be able to send GET, POST, PATCH, DELETE request
    apiClient should be able to send request with a Authorization header 'Bearer <API_KEY>'
    apiClient should be accept a API Key and Base Path as constructor
*/

/* 
    @Todo Review all string types
    @Todo Review how to reference self

*/

use reqwest::{Client, header};

const LUDEX_API: &str = "https://api.ludex.gg/api/v2/";

pub struct ApiClient {
    httpClient: Client,
    basePath: String
}

pub impl ApiClient {
    fn new(organizationApiKey: String, basePath: String) -> ApiClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", &organizationApiKey)));

        let httpAuthClient: Clietn = Client::builder()
            .default_headers(header)
            .build();
            
        ApiClient {
            httpClient: httpAuthClient
        }
    }
    
    async fn issue_get_request<T>(url: String) -> Result<T, StatusCode> {
        let fullUrl: &str = format!("{}{}", self.basePath, url);
        
        let response: Result<T, Error> = reqwest::get(fullUrl).await;
        
        match &response {
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    return Err(r.status());
                }
            }
            Err(e) => {
                if e.is_status() {
                    return Err(e.status().unwrap());
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    
        // Parse the response body as Json
        let content = response.unwrap().json::<T>().await;
    
        match content {
            Ok(s) => Ok(s),
            Err(e) => {
                println!("{:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

}