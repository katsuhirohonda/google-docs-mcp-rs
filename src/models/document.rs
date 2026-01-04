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
#[serde(rename_all = "camelCase")]
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
        #[serde(rename = "startIndex")]
        start_index: i32,
        /// End index of the range to delete
        #[serde(rename = "endIndex")]
        end_index: i32,
    },
    /// Replace all occurrences of text
    ReplaceAllText {
        /// The text to find
        #[serde(rename = "findText")]
        find_text: String,
        /// The text to replace with
        #[serde(rename = "replaceText")]
        replace_text: String,
        /// Whether to match case
        #[serde(default, rename = "matchCase")]
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

// =============================================================================
// Google Drive API Request/Response Models
// =============================================================================

/// Request body for creating a file via Drive API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveCreateFileRequest {
    /// The name of the file
    pub name: String,
    /// The MIME type of the file
    pub mime_type: String,
    /// Parent folder IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
}

/// Response from Drive API file creation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    /// The file ID
    pub id: String,
    /// The file name
    pub name: String,
    /// The MIME type
    pub mime_type: String,
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ResponseFormat Tests
    // -------------------------------------------------------------------------

    #[test]
    fn response_format_defaults_to_markdown() {
        // Given: No explicit format specified
        // When: Creating a default ResponseFormat
        let format = ResponseFormat::default();

        // Then: It should be Markdown
        assert!(matches!(format, ResponseFormat::Markdown));
    }

    #[test]
    fn response_format_serializes_to_lowercase() {
        // Given: ResponseFormat variants
        let markdown = ResponseFormat::Markdown;
        let json = ResponseFormat::Json;

        // When: Serializing to JSON
        let markdown_str = serde_json::to_string(&markdown).unwrap();
        let json_str = serde_json::to_string(&json).unwrap();

        // Then: Values should be lowercase strings
        assert_eq!(markdown_str, r#""markdown""#);
        assert_eq!(json_str, r#""json""#);
    }

    #[test]
    fn response_format_deserializes_from_lowercase() {
        // Given: Lowercase JSON strings
        let markdown_json = r#""markdown""#;
        let json_json = r#""json""#;

        // When: Deserializing from JSON
        let markdown: ResponseFormat = serde_json::from_str(markdown_json).unwrap();
        let json: ResponseFormat = serde_json::from_str(json_json).unwrap();

        // Then: Correct variants should be created
        assert!(matches!(markdown, ResponseFormat::Markdown));
        assert!(matches!(json, ResponseFormat::Json));
    }

    // -------------------------------------------------------------------------
    // DocumentRequest Tests
    // -------------------------------------------------------------------------

    #[test]
    fn document_request_insert_text_serializes_correctly() {
        // Given: An InsertText request
        let request = DocumentRequest::InsertText {
            text: "Hello, World!".to_string(),
            index: 1,
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: It should have camelCase format with correct fields
        assert_eq!(json["insertText"]["text"], "Hello, World!");
        assert_eq!(json["insertText"]["index"], 1);
    }

    #[test]
    fn document_request_delete_content_range_serializes_correctly() {
        // Given: A DeleteContentRange request
        let request = DocumentRequest::DeleteContentRange {
            start_index: 5,
            end_index: 10,
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: It should have camelCase format with correct range fields
        assert_eq!(json["deleteContentRange"]["startIndex"], 5);
        assert_eq!(json["deleteContentRange"]["endIndex"], 10);
    }

    #[test]
    fn document_request_replace_all_text_serializes_correctly() {
        // Given: A ReplaceAllText request with match_case
        let request = DocumentRequest::ReplaceAllText {
            find_text: "old".to_string(),
            replace_text: "new".to_string(),
            match_case: true,
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: It should have camelCase format with all fields including matchCase
        assert_eq!(json["replaceAllText"]["findText"], "old");
        assert_eq!(json["replaceAllText"]["replaceText"], "new");
        assert_eq!(json["replaceAllText"]["matchCase"], true);
    }

    #[test]
    fn document_request_replace_all_text_defaults_match_case_to_false() {
        // Given: JSON without matchCase field (camelCase format)
        let json = r#"{"replaceAllText":{"findText":"old","replaceText":"new"}}"#;

        // When: Deserializing the request
        let request: DocumentRequest = serde_json::from_str(json).unwrap();

        // Then: match_case should default to false
        if let DocumentRequest::ReplaceAllText { match_case, .. } = request {
            assert!(!match_case);
        } else {
            panic!("Expected ReplaceAllText variant");
        }
    }

    // -------------------------------------------------------------------------
    // Document Model Tests
    // -------------------------------------------------------------------------

    #[test]
    fn document_deserializes_from_api_response() {
        // Given: A Google Docs API response JSON
        let json = r#"{
            "documentId": "1abc123",
            "title": "Test Document",
            "revisionId": "rev123"
        }"#;

        // When: Deserializing the response
        let doc: Document = serde_json::from_str(json).unwrap();

        // Then: Fields should be correctly mapped from camelCase
        assert_eq!(doc.document_id, "1abc123");
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.revision_id, Some("rev123".to_string()));
        assert!(doc.body.is_none());
    }

    #[test]
    fn document_deserializes_with_body_content() {
        // Given: A document with body content
        let json = r#"{
            "documentId": "doc123",
            "title": "Doc with Body",
            "body": {
                "content": [
                    {
                        "startIndex": 1,
                        "endIndex": 14,
                        "paragraph": {
                            "elements": [
                                {
                                    "startIndex": 1,
                                    "endIndex": 14,
                                    "textRun": {
                                        "content": "Hello, World!"
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        }"#;

        // When: Deserializing the document
        let doc: Document = serde_json::from_str(json).unwrap();

        // Then: Body content should be correctly parsed
        let body = doc.body.expect("Body should exist");
        assert_eq!(body.content.len(), 1);

        let element = &body.content[0];
        assert_eq!(element.start_index, Some(1));
        assert_eq!(element.end_index, Some(14));

        let paragraph = element.paragraph.as_ref().expect("Paragraph should exist");
        assert_eq!(paragraph.elements.len(), 1);

        let text_run = paragraph.elements[0]
            .text_run
            .as_ref()
            .expect("TextRun should exist");
        assert_eq!(text_run.content, Some("Hello, World!".to_string()));
    }

    // -------------------------------------------------------------------------
    // API Request Models Tests
    // -------------------------------------------------------------------------

    #[test]
    fn create_document_request_serializes_correctly() {
        // Given: A create document request
        let request = CreateDocumentRequest {
            title: "New Document".to_string(),
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: It should have the title field
        assert_eq!(json["title"], "New Document");
    }

    #[test]
    fn google_docs_request_serializes_insert_text_only() {
        // Given: A GoogleDocsRequest with only insert_text
        let request = GoogleDocsRequest {
            insert_text: Some(InsertTextRequest {
                text: "Hello".to_string(),
                location: Location { index: 1 },
            }),
            delete_content_range: None,
            replace_all_text: None,
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: Only insert_text should be present (skip_serializing_if)
        assert!(json.get("insertText").is_some());
        assert!(json.get("deleteContentRange").is_none());
        assert!(json.get("replaceAllText").is_none());
    }

    #[test]
    fn batch_update_request_serializes_with_camel_case() {
        // Given: A batch update request with multiple operations
        let request = BatchUpdateRequest {
            requests: vec![
                GoogleDocsRequest {
                    insert_text: Some(InsertTextRequest {
                        text: "New text".to_string(),
                        location: Location { index: 1 },
                    }),
                    delete_content_range: None,
                    replace_all_text: None,
                },
            ],
        };

        // When: Serializing to JSON
        let json = serde_json::to_string(&request).unwrap();

        // Then: Field names should be camelCase for API compatibility
        assert!(json.contains("insertText"));
        assert!(!json.contains("insert_text"));
    }

    #[test]
    fn replace_all_text_request_serializes_correctly() {
        // Given: A replace all text request
        let request = ReplaceAllTextRequest {
            contains_text: ContainsText {
                text: "find me".to_string(),
                match_case: true,
            },
            replace_text: "replacement".to_string(),
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: All fields should be in camelCase
        assert_eq!(json["containsText"]["text"], "find me");
        assert_eq!(json["containsText"]["matchCase"], true);
        assert_eq!(json["replaceText"], "replacement");
    }

    // -------------------------------------------------------------------------
    // BatchUpdateResponse Tests
    // -------------------------------------------------------------------------

    #[test]
    fn batch_update_response_deserializes_correctly() {
        // Given: A batch update response from the API
        let json = r#"{
            "documentId": "doc123",
            "replies": [{}]
        }"#;

        // When: Deserializing the response
        let response: BatchUpdateResponse = serde_json::from_str(json).unwrap();

        // Then: Fields should be correctly mapped
        assert_eq!(response.document_id, "doc123");
        assert_eq!(response.replies.len(), 1);
    }

    #[test]
    fn batch_update_response_handles_empty_replies() {
        // Given: A response without replies field
        let json = r#"{"documentId": "doc456"}"#;

        // When: Deserializing the response
        let response: BatchUpdateResponse = serde_json::from_str(json).unwrap();

        // Then: replies should default to empty vector
        assert_eq!(response.document_id, "doc456");
        assert!(response.replies.is_empty());
    }

    // -------------------------------------------------------------------------
    // Google Drive API Models Tests
    // -------------------------------------------------------------------------

    #[test]
    fn drive_create_file_request_serializes_with_parents() {
        // Given: A Drive API create file request with parents
        let request = DriveCreateFileRequest {
            name: "Test Document".to_string(),
            mime_type: "application/vnd.google-apps.document".to_string(),
            parents: Some(vec!["folder123".to_string()]),
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: Fields should be camelCase and parents should be an array
        assert_eq!(json["name"], "Test Document");
        assert_eq!(json["mimeType"], "application/vnd.google-apps.document");
        assert_eq!(json["parents"][0], "folder123");
    }

    #[test]
    fn drive_create_file_request_omits_parents_when_none() {
        // Given: A Drive API create file request without parents
        let request = DriveCreateFileRequest {
            name: "Test Document".to_string(),
            mime_type: "application/vnd.google-apps.document".to_string(),
            parents: None,
        };

        // When: Serializing to JSON
        let json = serde_json::to_value(&request).unwrap();

        // Then: parents field should be omitted
        assert_eq!(json["name"], "Test Document");
        assert_eq!(json["mimeType"], "application/vnd.google-apps.document");
        assert!(json.get("parents").is_none());
    }

    #[test]
    fn drive_file_deserializes_from_api_response() {
        // Given: A Drive API file response
        let json = r#"{
            "id": "doc123abc",
            "name": "My Document",
            "mimeType": "application/vnd.google-apps.document"
        }"#;

        // When: Deserializing the response
        let file: DriveFile = serde_json::from_str(json).unwrap();

        // Then: Fields should be correctly mapped from camelCase
        assert_eq!(file.id, "doc123abc");
        assert_eq!(file.name, "My Document");
        assert_eq!(file.mime_type, "application/vnd.google-apps.document");
    }

    // -------------------------------------------------------------------------
    // ServiceAccountCredentials Tests
    // -------------------------------------------------------------------------

    #[test]
    fn service_account_credentials_deserializes_from_json_key() {
        // Given: A service account JSON key file content
        let json = r#"{
            "type": "service_account",
            "project_id": "my-project",
            "private_key_id": "key123",
            "private_key": "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----\n",
            "client_email": "test@my-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }"#;

        // When: Deserializing the credentials
        let creds: ServiceAccountCredentials = serde_json::from_str(json).unwrap();

        // Then: All fields should be correctly mapped
        assert_eq!(creds.credential_type, "service_account");
        assert_eq!(creds.project_id, "my-project");
        assert_eq!(creds.private_key_id, "key123");
        assert_eq!(
            creds.client_email,
            "test@my-project.iam.gserviceaccount.com"
        );
        assert_eq!(creds.client_id, "123456789");
    }

    // -------------------------------------------------------------------------
    // TokenResponse Tests
    // -------------------------------------------------------------------------

    #[test]
    fn token_response_deserializes_correctly() {
        // Given: An OAuth2 token response
        let json = r#"{
            "access_token": "ya29.abc123",
            "token_type": "Bearer",
            "expires_in": 3600
        }"#;

        // When: Deserializing the response
        let token: TokenResponse = serde_json::from_str(json).unwrap();

        // Then: All fields should be correctly mapped
        assert_eq!(token.access_token, "ya29.abc123");
        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.expires_in, 3600);
    }
}
