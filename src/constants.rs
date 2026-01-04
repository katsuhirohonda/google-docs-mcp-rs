/// Google Docs API base URL
pub const GOOGLE_DOCS_API_URL: &str = "https://docs.googleapis.com/v1";

/// Google OAuth2 token endpoint
pub const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// Google Docs API scope
pub const GOOGLE_DOCS_SCOPE: &str = "https://www.googleapis.com/auth/documents";

/// Maximum character limit for responses
pub const CHARACTER_LIMIT: usize = 25_000;

/// JWT expiration time in seconds (1 hour)
pub const JWT_EXPIRATION_SECS: i64 = 3600;
