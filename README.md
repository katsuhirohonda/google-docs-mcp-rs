# Google Docs MCP Server

A Rust-based Model Context Protocol (MCP) server for Google Docs API integration using Service Account authentication.

## Features

- **Read Documents**: Retrieve document content and metadata
- **Update Documents**: Modify documents with insert, delete, and replace operations
- **Service Account Auth**: Secure authentication using Google Service Account credentials

## Prerequisites

1. A Google Cloud Project
2. Google Docs API enabled
3. A Service Account with appropriate permissions
4. Service Account JSON key file

## Setup

### 1. Create a Service Account

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create or select a project
3. Enable the Google Docs API:
   - Go to **APIs & Services** > **Library**
   - Search for and enable **Google Docs API**
4. Go to **IAM & Admin** > **Service Accounts**
5. Create a new service account
6. Download the JSON key file

### 2. Grant Document Access

For the service account to access documents:
- Share documents with the service account email (found in the JSON key file as `client_email`)
- Or use Google Workspace domain-wide delegation for organization-wide access

### 3. Build the Server

```bash
cargo build --release
```

## Usage

### Environment Variables

Set the path to your service account JSON key file:

```bash
export GOOGLE_SERVICE_ACCOUNT_KEY=/path/to/service-account.json
```

### Running the Server

```bash
./target/release/google-docs-mcp-server
```

### Claude Code Configuration

Add to your Claude Code MCP settings:

```json
{
  "mcpServers": {
    "google-docs": {
      "command": "/path/to/google-docs-mcp-server",
      "env": {
        "GOOGLE_SERVICE_ACCOUNT_KEY": "/path/to/service-account.json"
      }
    }
  }
}
```

## Tools

### google_docs_get_document

Retrieve a Google Document by ID.

**Parameters:**
- `document_id` (required): The document ID
- `response_format` (optional): "markdown" (default) or "json"

### google_docs_update_document

Update a Google Document with various operations.

**Parameters:**
- `document_id` (required): The document ID
- `requests` (required): Array of update operations
- `response_format` (optional): "markdown" (default) or "json"

**Update Operations:**

```json
{
  "insert_text": {
    "text": "Hello World",
    "index": 1
  }
}
```

```json
{
  "delete_content_range": {
    "start_index": 1,
    "end_index": 10
  }
}
```

```json
{
  "replace_all_text": {
    "find_text": "old text",
    "replace_text": "new text",
    "match_case": true
  }
}
```

## License

MIT
