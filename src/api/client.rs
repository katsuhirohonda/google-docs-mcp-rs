use crate::constants::{GOOGLE_DOCS_API_URL, GOOGLE_DOCS_SCOPE, GOOGLE_TOKEN_URL, JWT_EXPIRATION_SECS};
use crate::models::{
    BatchUpdateRequest, BatchUpdateResponse, Document, GoogleDocsRequest,
    ServiceAccountCredentials, TokenResponse,
};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use rmcp::ErrorData as McpError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// JWT claims for Service Account authentication
#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    /// Issuer (service account email)
    iss: String,
    /// Scope
    scope: String,
    /// Audience (token endpoint)
    aud: String,
    /// Issued at timestamp
    iat: i64,
    /// Expiration timestamp
    exp: i64,
}

/// Cached access token with expiration
#[derive(Debug, Clone)]
struct CachedToken {
    access_token: String,
    expires_at: i64,
}

/// Google Docs API client with Service Account authentication
#[derive(Clone)]
pub struct GoogleDocsClient {
    client: Client,
    credentials: ServiceAccountCredentials,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl GoogleDocsClient {
    /// Create a new Google Docs API client from service account credentials
    pub fn new(credentials: ServiceAccountCredentials) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            credentials,
            cached_token: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a new client from a JSON key file path
    pub fn from_json_file(path: &str) -> Result<Self, McpError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            McpError::internal_error(
                format!("Failed to read service account key file: {}", e),
                None,
            )
        })?;

        let credentials: ServiceAccountCredentials =
            serde_json::from_str(&content).map_err(|e| {
                McpError::internal_error(
                    format!("Failed to parse service account key file: {}", e),
                    None,
                )
            })?;

        Ok(Self::new(credentials))
    }

    /// Get a valid access token, refreshing if necessary
    async fn get_access_token(&self) -> Result<String, McpError> {
        // Check if we have a valid cached token
        {
            let cached = self.cached_token.read().await;
            if let Some(ref token) = *cached {
                let now = Utc::now().timestamp();
                // Use token if it has more than 60 seconds of validity
                if token.expires_at > now + 60 {
                    return Ok(token.access_token.clone());
                }
            }
        }

        // Need to refresh the token
        let new_token = self.fetch_new_token().await?;

        // Cache the new token
        {
            let mut cached = self.cached_token.write().await;
            *cached = Some(new_token.clone());
        }

        Ok(new_token.access_token)
    }

    /// Fetch a new access token using Service Account JWT
    async fn fetch_new_token(&self) -> Result<CachedToken, McpError> {
        let now = Utc::now().timestamp();
        let exp = now + JWT_EXPIRATION_SECS;

        let claims = JwtClaims {
            iss: self.credentials.client_email.clone(),
            scope: GOOGLE_DOCS_SCOPE.to_string(),
            aud: GOOGLE_TOKEN_URL.to_string(),
            iat: now,
            exp,
        };

        let header = Header::new(Algorithm::RS256);
        let key = EncodingKey::from_rsa_pem(self.credentials.private_key.as_bytes())
            .map_err(|e| {
                McpError::internal_error(format!("Failed to parse private key: {}", e), None)
            })?;

        let jwt = encode(&header, &claims, &key).map_err(|e| {
            McpError::internal_error(format!("Failed to create JWT: {}", e), None)
        })?;

        // Exchange JWT for access token
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let response = self
            .client
            .post(GOOGLE_TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| handle_api_error(e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(McpError::internal_error(
                format!(
                    "Failed to obtain access token: {} - {}",
                    status, body
                ),
                None,
            ));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            McpError::internal_error(format!("Failed to parse token response: {}", e), None)
        })?;

        Ok(CachedToken {
            access_token: token_response.access_token,
            expires_at: now + token_response.expires_in,
        })
    }

    /// Get a Google Document by ID
    pub async fn get_document(&self, document_id: &str) -> Result<Document, McpError> {
        let token = self.get_access_token().await?;

        let response = self
            .client
            .get(format!("{}/documents/{}", GOOGLE_DOCS_API_URL, document_id))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(handle_api_error)?;

        handle_response(response).await
    }

    /// Update a Google Document with batch requests
    pub async fn batch_update(
        &self,
        document_id: &str,
        requests: Vec<GoogleDocsRequest>,
    ) -> Result<BatchUpdateResponse, McpError> {
        let token = self.get_access_token().await?;

        let request_body = BatchUpdateRequest { requests };

        let response = self
            .client
            .post(format!(
                "{}/documents/{}:batchUpdate",
                GOOGLE_DOCS_API_URL, document_id
            ))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(handle_api_error)?;

        handle_response(response).await
    }
}

/// Handle API response and convert to result
async fn handle_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, McpError> {
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(match status.as_u16() {
            404 => McpError::invalid_params(
                format!("Document not found. Please check the document ID. Details: {}", body),
                None,
            ),
            403 => McpError::invalid_params(
                format!("Permission denied. Ensure the service account has access to this document. Details: {}", body),
                None,
            ),
            401 => McpError::internal_error(
                format!("Authentication failed. Check service account credentials. Details: {}", body),
                None,
            ),
            429 => McpError::internal_error(
                "Rate limit exceeded. Please wait before making more requests.".to_string(),
                None,
            ),
            _ => McpError::internal_error(
                format!("API request failed with status {}: {}", status, body),
                None,
            ),
        });
    }

    response.json().await.map_err(|e| {
        McpError::internal_error(format!("Failed to parse API response: {}", e), None)
    })
}

/// Convert reqwest errors to MCP errors with clear messages
pub fn handle_api_error(error: reqwest::Error) -> McpError {
    if error.is_timeout() {
        McpError::internal_error("Request timed out. Please try again.".to_string(), None)
    } else if error.is_connect() {
        McpError::internal_error(
            "Failed to connect to Google API. Please check network connectivity.".to_string(),
            None,
        )
    } else {
        McpError::internal_error(format!("Network error: {}", error), None)
    }
}
