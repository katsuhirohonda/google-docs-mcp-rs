use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Output format for responses
#[derive(Debug, Default, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
    /// Human-readable markdown format
    #[default]
    Markdown,
    /// Machine-readable JSON format
    Json,
}

/// A single update request for a document
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DocumentRequest {
    /// Insert text at a specific location
    InsertText {
        /// The text to insert
        text: String,
        /// The index where to insert (1 = beginning of document body)
        index: i32,
    },
    /// Delete content in a range
    DeleteContentRange {
        /// Start index of the range to delete
        start_index: i32,
        /// End index of the range to delete
        end_index: i32,
    },
    /// Replace all occurrences of text
    ReplaceAllText {
        /// The text to find
        find_text: String,
        /// The text to replace with
        replace_text: String,
        /// Whether to match case
        #[serde(default)]
        match_case: bool,
    },
}

// =============================================================================
// Google Docs API Response Models
// =============================================================================

/// Google Document response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// The document ID
    pub document_id: String,

    /// The document title
    pub title: String,

    /// The document body
    #[serde(default)]
    pub body: Option<DocumentBody>,

    /// The revision ID of the document
    #[serde(default)]
    pub revision_id: Option<String>,
}

/// Document body structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBody {
    /// The content elements in the document body
    #[serde(default)]
    pub content: Vec<StructuralElement>,
}

/// A structural element in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralElement {
    /// Start index of this element
    #[serde(default)]
    pub start_index: Option<i32>,

    /// End index of this element
    #[serde(default)]
    pub end_index: Option<i32>,

    /// Paragraph content
    #[serde(default)]
    pub paragraph: Option<Paragraph>,
}

/// A paragraph in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    /// The paragraph elements
    #[serde(default)]
    pub elements: Vec<ParagraphElement>,
}

/// An element within a paragraph
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphElement {
    /// Start index of this element
    #[serde(default)]
    pub start_index: Option<i32>,

    /// End index of this element
    #[serde(default)]
    pub end_index: Option<i32>,

    /// Text run content
    #[serde(default)]
    pub text_run: Option<TextRun>,
}

/// A run of text with the same styling
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRun {
    /// The actual text content
    #[serde(default)]
    pub content: Option<String>,
}

// =============================================================================
// Google Docs API Request Models
// =============================================================================

/// Request body for creating a document
#[derive(Debug, Serialize)]
pub struct CreateDocumentRequest {
    /// The title of the document
    pub title: String,
}

/// Request body for batch updating a document
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequest {
    /// The list of requests to apply
    pub requests: Vec<GoogleDocsRequest>,
}

/// A single request in a batch update
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleDocsRequest {
    /// Insert text request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<InsertTextRequest>,

    /// Delete content range request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_content_range: Option<DeleteContentRangeRequest>,

    /// Replace all text request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_all_text: Option<ReplaceAllTextRequest>,
}

/// Insert text request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextRequest {
    /// The text to insert
    pub text: String,
    /// The location to insert at
    pub location: Location,
}

/// A location in the document
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// The index in the document
    pub index: i32,
}

/// Delete content range request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteContentRangeRequest {
    /// The range to delete
    pub range: Range,
}

/// A range in the document
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    /// Start index
    pub start_index: i32,
    /// End index
    pub end_index: i32,
}

/// Replace all text request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAllTextRequest {
    /// The text to find
    pub contains_text: ContainsText,
    /// The replacement text
    pub replace_text: String,
}

/// Text search criteria
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainsText {
    /// The text to search for
    pub text: String,
    /// Whether to match case
    pub match_case: bool,
}

/// Response from batch update
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateResponse {
    /// The updated document ID
    pub document_id: String,

    /// Individual replies for each request
    #[serde(default)]
    pub replies: Vec<serde_json::Value>,
}

// =============================================================================
// Service Account Credentials
// =============================================================================

/// Service account credentials from JSON key file
#[derive(Debug, Clone, Deserialize)]
pub struct ServiceAccountCredentials {
    /// The type of credentials (should be "service_account")
    #[serde(rename = "type")]
    pub credential_type: String,

    /// The project ID
    pub project_id: String,

    /// The private key ID
    pub private_key_id: String,

    /// The private key in PEM format
    pub private_key: String,

    /// The client email (service account email)
    pub client_email: String,

    /// The client ID
    pub client_id: String,

    /// The auth URI
    pub auth_uri: String,

    /// The token URI
    pub token_uri: String,
}

/// OAuth2 token response
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    /// The access token
    pub access_token: String,

    /// Token type (usually "Bearer")
    pub token_type: String,

    /// Expiration time in seconds
    pub expires_in: i64,
}
