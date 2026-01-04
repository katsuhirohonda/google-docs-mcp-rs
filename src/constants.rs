/// Google Docs API base URL
pub const GOOGLE_DOCS_API_URL: &str = "https://docs.googleapis.com/v1";

/// Google Drive API base URL
pub const GOOGLE_DRIVE_API_URL: &str = "https://www.googleapis.com/drive/v3";

/// Google OAuth2 token endpoint
pub const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// Google Docs API scope
pub const GOOGLE_DOCS_SCOPE: &str = "https://www.googleapis.com/auth/documents";

/// Google Drive API scope
pub const GOOGLE_DRIVE_SCOPE: &str = "https://www.googleapis.com/auth/drive";

/// JWT expiration time in seconds (1 hour)
pub const JWT_EXPIRATION_SECS: i64 = 3600;

/// MIME type for Google Docs documents
pub const GOOGLE_DOCS_MIME_TYPE: &str = "application/vnd.google-apps.document";
