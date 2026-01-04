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

    /// The document body (may be empty when tabs are used; content is in tabs instead)
    #[serde(default)]
    pub body: Option<DocumentBody>,

    /// The tabs in the document (populated when includeTabsContent=true)
    #[serde(default)]
    pub tabs: Vec<Tab>,

    /// The revision ID of the document
    #[serde(default)]
    pub revision_id: Option<String>,
}

/// A tab in a Google Document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    /// Properties of the tab
    #[serde(default)]
    pub tab_properties: Option<TabProperties>,

    /// The document content within this tab
    #[serde(default)]
    pub document_tab: Option<DocumentTab>,

    /// Child tabs nested under this tab
    #[serde(default)]
    pub child_tabs: Vec<Tab>,
}

/// Properties of a tab
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabProperties {
    /// The ID of the tab
    #[serde(default)]
    pub tab_id: Option<String>,

    /// The title of the tab
    #[serde(default)]
    pub title: Option<String>,

    /// The index of the tab
    #[serde(default)]
    pub index: Option<i32>,
}

/// The document content within a tab
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTab {
    /// The body content of the tab
    #[serde(default)]
    pub body: Option<DocumentBody>,
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

    #[test]
    fn document_deserializes_with_tabs_content() {
        // Given: A document with tabs (includeTabsContent=true response)
        let json = r#"{
            "documentId": "doc456",
            "title": "Doc with Tabs",
            "tabs": [
                {
                    "tabProperties": {
                        "tabId": "tab1",
                        "title": "First Tab",
                        "index": 0
                    },
                    "documentTab": {
                        "body": {
                            "content": [
                                {
                                    "startIndex": 1,
                                    "endIndex": 20,
                                    "paragraph": {
                                        "elements": [
                                            {
                                                "textRun": {
                                                    "content": "Content in tab 1"
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    "childTabs": []
                },
                {
                    "tabProperties": {
                        "tabId": "tab2",
                        "title": "Second Tab",
                        "index": 1
                    },
                    "documentTab": {
                        "body": {
                            "content": [
                                {
                                    "paragraph": {
                                        "elements": [
                                            {
                                                "textRun": {
                                                    "content": "Content in tab 2"
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    },
                    "childTabs": []
                }
            ]
        }"#;

        // When: Deserializing the document
        let doc: Document = serde_json::from_str(json).unwrap();

        // Then: Tabs should be correctly parsed
        assert_eq!(doc.document_id, "doc456");
        assert_eq!(doc.title, "Doc with Tabs");
        assert_eq!(doc.tabs.len(), 2);

        // Verify first tab
        let tab1 = &doc.tabs[0];
        let tab1_props = tab1.tab_properties.as_ref().expect("Tab properties should exist");
        assert_eq!(tab1_props.tab_id, Some("tab1".to_string()));
        assert_eq!(tab1_props.title, Some("First Tab".to_string()));
        assert_eq!(tab1_props.index, Some(0));

        let doc_tab1 = tab1.document_tab.as_ref().expect("DocumentTab should exist");
        let body1 = doc_tab1.body.as_ref().expect("Body should exist");
        assert_eq!(body1.content.len(), 1);

        // Verify second tab
        let tab2 = &doc.tabs[1];
        let tab2_props = tab2.tab_properties.as_ref().expect("Tab properties should exist");
        assert_eq!(tab2_props.tab_id, Some("tab2".to_string()));
        assert_eq!(tab2_props.title, Some("Second Tab".to_string()));
    }

    #[test]
    fn document_deserializes_with_nested_child_tabs() {
        // Given: A document with nested child tabs
        let json = r#"{
            "documentId": "doc789",
            "title": "Doc with Nested Tabs",
            "tabs": [
                {
                    "tabProperties": {
                        "tabId": "parent",
                        "title": "Parent Tab"
                    },
                    "documentTab": {
                        "body": {
                            "content": []
                        }
                    },
                    "childTabs": [
                        {
                            "tabProperties": {
                                "tabId": "child1",
                                "title": "Child Tab 1"
                            },
                            "documentTab": {
                                "body": {
                                    "content": [
                                        {
                                            "paragraph": {
                                                "elements": [
                                                    {
                                                        "textRun": {
                                                            "content": "Child content"
                                                        }
                                                    }
                                                ]
                                            }
                                        }
                                    ]
                                }
                            },
                            "childTabs": []
                        }
                    ]
                }
            ]
        }"#;

        // When: Deserializing the document
        let doc: Document = serde_json::from_str(json).unwrap();

        // Then: Nested tabs should be correctly parsed
        assert_eq!(doc.tabs.len(), 1);
        let parent_tab = &doc.tabs[0];
        assert_eq!(parent_tab.child_tabs.len(), 1);

        let child_tab = &parent_tab.child_tabs[0];
        let child_props = child_tab.tab_properties.as_ref().unwrap();
        assert_eq!(child_props.tab_id, Some("child1".to_string()));
        assert_eq!(child_props.title, Some("Child Tab 1".to_string()));
    }

    // -------------------------------------------------------------------------
    // API Request Models Tests
    // -------------------------------------------------------------------------

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
