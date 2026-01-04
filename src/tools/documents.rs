use crate::api::GoogleDocsClient;
use crate::models::{
    ContainsText, DeleteContentRangeRequest, Document, DocumentRequest,
    GoogleDocsRequest, InsertTextRequest, Location, Range,
    ReplaceAllTextRequest, ResponseFormat,
};
use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::tool::Parameters,
    model::*,
    tool, tool_handler, tool_router,
    ErrorData as McpError,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

/// Google Docs MCP Server
#[derive(Clone)]
pub struct GoogleDocsMcpServer {
    client: Arc<GoogleDocsClient>,
    tool_router: ToolRouter<Self>,
}

/// Input for getting a Google Document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentParams {
    /// The document ID to retrieve
    pub document_id: String,

    /// Output format: "markdown" (default) or "json"
    #[serde(default)]
    pub response_format: ResponseFormat,
}

/// Input for updating a Google Document
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateDocumentParams {
    /// The document ID to update
    pub document_id: String,

    /// List of update operations to apply
    pub requests: Vec<DocumentRequest>,

    /// Output format: "markdown" (default) or "json"
    #[serde(default)]
    pub response_format: ResponseFormat,
}

#[tool_router]
impl GoogleDocsMcpServer {
    /// Create a new Google Docs MCP server
    pub fn new(client: GoogleDocsClient) -> Self {
        Self {
            client: Arc::new(client),
            tool_router: Self::tool_router(),
        }
    }

    /// Get a Google Document by its ID.
    #[tool(description = "Get a Google Document by its ID. Returns the document title and full text content from all tabs (including nested child tabs).")]
    async fn google_docs_get_document(
        &self,
        Parameters(params): Parameters<GetDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        if params.document_id.trim().is_empty() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Document ID cannot be empty",
            )]));
        }

        match self.client.get_document(&params.document_id).await {
            Ok(document) => {
                let response = format_get_response(&document, &params.response_format);
                Ok(CallToolResult::success(vec![Content::text(response)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to get document: {:?}",
                e
            ))])),
        }
    }

    /// Update a Google Document with various operations.
    #[tool(description = r#"Update a Google Document with batch operations.

## Supported Operations

### 1. insertText
Insert text at a specific position in the document.
- `text` (string, required): The text to insert
- `index` (integer, required): Position to insert at (1 = beginning of document body)

### 2. deleteContentRange
Delete content within a specified range.
- `startIndex` (integer, required): Start position of the range to delete
- `endIndex` (integer, required): End position of the range to delete

### 3. replaceAllText
Replace all occurrences of a text string.
- `findText` (string, required): The text to search for
- `replaceText` (string, required): The replacement text
- `matchCase` (boolean, optional): Whether to match case (default: false)

## Example Request

```json
{
  "document_id": "your-document-id",
  "requests": [
    {
      "insertText": {
        "text": "Hello, World!",
        "index": 1
      }
    },
    {
      "deleteContentRange": {
        "startIndex": 10,
        "endIndex": 20
      }
    },
    {
      "replaceAllText": {
        "findText": "old text",
        "replaceText": "new text",
        "matchCase": true
      }
    }
  ]
}
```

## Notes
- Index 1 is the beginning of the document body
- To append text at the end, first get the document to find the last index
- Operations are applied in order"#)]
    async fn google_docs_update_document(
        &self,
        Parameters(params): Parameters<UpdateDocumentParams>,
    ) -> Result<CallToolResult, McpError> {
        if params.document_id.trim().is_empty() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Document ID cannot be empty",
            )]));
        }

        if params.requests.is_empty() {
            return Ok(CallToolResult::error(vec![Content::text(
                "At least one update request is required",
            )]));
        }

        // Convert user-friendly requests to Google Docs API format
        let google_requests = match convert_requests(&params.requests) {
            Ok(r) => r,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(e)]));
            }
        };

        match self
            .client
            .batch_update(&params.document_id, google_requests)
            .await
        {
            Ok(result) => {
                let response =
                    format_update_response(&result.document_id, &params.requests, &params.response_format);
                Ok(CallToolResult::success(vec![Content::text(response)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to update document: {:?}",
                e
            ))])),
        }
    }
}

#[tool_handler]
impl rmcp::ServerHandler for GoogleDocsMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Google Docs MCP Server - Read and update Google Documents using Service Account authentication".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

/// Convert user-friendly requests to Google Docs API format
fn convert_requests(requests: &[DocumentRequest]) -> Result<Vec<GoogleDocsRequest>, String> {
    requests
        .iter()
        .map(|req| match req {
            DocumentRequest::InsertText { text, index } => {
                if *index < 1 {
                    return Err(
                        "Insert index must be at least 1 (1 = beginning of document body)"
                            .to_string(),
                    );
                }
                Ok(GoogleDocsRequest {
                    insert_text: Some(InsertTextRequest {
                        text: text.clone(),
                        location: Location { index: *index },
                    }),
                    delete_content_range: None,
                    replace_all_text: None,
                })
            }
            DocumentRequest::DeleteContentRange {
                start_index,
                end_index,
            } => {
                if *start_index < 1 {
                    return Err("Start index must be at least 1".to_string());
                }
                if *end_index <= *start_index {
                    return Err("End index must be greater than start index".to_string());
                }
                Ok(GoogleDocsRequest {
                    insert_text: None,
                    delete_content_range: Some(DeleteContentRangeRequest {
                        range: Range {
                            start_index: *start_index,
                            end_index: *end_index,
                        },
                    }),
                    replace_all_text: None,
                })
            }
            DocumentRequest::ReplaceAllText {
                find_text,
                replace_text,
                match_case,
            } => {
                if find_text.is_empty() {
                    return Err("Find text cannot be empty".to_string());
                }
                Ok(GoogleDocsRequest {
                    insert_text: None,
                    delete_content_range: None,
                    replace_all_text: Some(ReplaceAllTextRequest {
                        contains_text: ContainsText {
                            text: find_text.clone(),
                            match_case: *match_case,
                        },
                        replace_text: replace_text.clone(),
                    }),
                })
            }
        })
        .collect()
}

/// Extract plain text content from a document body
fn extract_text_from_body(body: &crate::models::DocumentBody) -> String {
    let mut text = String::new();
    for element in &body.content {
        if let Some(ref paragraph) = element.paragraph {
            for para_element in &paragraph.elements {
                if let Some(ref text_run) = para_element.text_run {
                    if let Some(ref content) = text_run.content {
                        text.push_str(content);
                    }
                }
            }
        }
    }
    text
}

/// Extract plain text content from a tab (including nested child tabs)
fn extract_text_from_tab(tab: &crate::models::Tab) -> String {
    let mut text = String::new();

    // Extract content from this tab's document_tab
    if let Some(ref doc_tab) = tab.document_tab {
        if let Some(ref body) = doc_tab.body {
            text.push_str(&extract_text_from_body(body));
        }
    }

    // Recursively extract content from child tabs
    for child_tab in &tab.child_tabs {
        text.push_str(&extract_text_from_tab(child_tab));
    }

    text
}

/// Extract plain text content from a document
fn extract_text_content(document: &Document) -> String {
    // If tabs are present (includeTabsContent=true), extract from tabs
    if !document.tabs.is_empty() {
        let mut text = String::new();
        for tab in &document.tabs {
            text.push_str(&extract_text_from_tab(tab));
        }
        return text;
    }

    // Fallback: extract from body (for documents without tabs or when includeTabsContent=false)
    if let Some(ref body) = document.body {
        return extract_text_from_body(body);
    }

    String::new()
}

/// Format get document response
fn format_get_response(document: &Document, format: &ResponseFormat) -> String {
    let content = extract_text_content(document);

    match format {
        ResponseFormat::Markdown => {
            let url = format!(
                "https://docs.google.com/document/d/{}/edit",
                document.document_id
            );
            format!(
                "# {}\n\n\
                 - **Document ID**: `{}`\n\
                 - **URL**: [Open in Google Docs]({})\n\n\
                 ## Content\n\n\
                 {}",
                document.title, document.document_id, url, content
            )
        }
        ResponseFormat::Json => {
            serde_json::json!({
                "document_id": document.document_id,
                "title": document.title,
                "url": format!("https://docs.google.com/document/d/{}/edit", document.document_id),
                "content": content,
                "revision_id": document.revision_id
            })
            .to_string()
        }
    }
}

/// Format update document response
fn format_update_response(
    document_id: &str,
    requests: &[DocumentRequest],
    format: &ResponseFormat,
) -> String {
    match format {
        ResponseFormat::Markdown => {
            let mut lines = vec![
                "# Document Updated".to_string(),
                String::new(),
                format!("- **Document ID**: `{}`", document_id),
                format!("- **Operations Applied**: {}", requests.len()),
                String::new(),
                "## Operations".to_string(),
                String::new(),
            ];

            for (i, req) in requests.iter().enumerate() {
                let desc = match req {
                    DocumentRequest::InsertText { text, index } => {
                        format!(
                            "{}. Inserted text at index {}: \"{}\"",
                            i + 1,
                            index,
                            truncate_text(text, 50)
                        )
                    }
                    DocumentRequest::DeleteContentRange {
                        start_index,
                        end_index,
                    } => {
                        format!(
                            "{}. Deleted content from index {} to {}",
                            i + 1,
                            start_index,
                            end_index
                        )
                    }
                    DocumentRequest::ReplaceAllText {
                        find_text,
                        replace_text,
                        match_case,
                    } => {
                        format!(
                            "{}. Replaced \"{}\" with \"{}\" (case-sensitive: {})",
                            i + 1,
                            truncate_text(find_text, 30),
                            truncate_text(replace_text, 30),
                            match_case
                        )
                    }
                };
                lines.push(desc);
            }

            lines.join("\n")
        }
        ResponseFormat::Json => {
            serde_json::json!({
                "document_id": document_id,
                "operations_count": requests.len(),
                "success": true
            })
            .to_string()
        }
    }
}

/// Truncate text for display
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}
