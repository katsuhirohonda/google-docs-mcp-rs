use google_docs_mcp_server::{GoogleDocsClient, GoogleDocsMcpServer};
use rmcp::transport::stdio;
use rmcp::ServiceExt;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing (logs to stderr so it doesn't interfere with stdio transport)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    // Get service account credentials path from environment
    let credentials_path = env::var("GOOGLE_SERVICE_ACCOUNT_KEY").unwrap_or_else(|_| {
        eprintln!("Error: GOOGLE_SERVICE_ACCOUNT_KEY environment variable is required.");
        eprintln!("Set it to the path of your service account JSON key file.");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  export GOOGLE_SERVICE_ACCOUNT_KEY=/path/to/service-account.json");
        std::process::exit(1);
    });

    // Create Google Docs API client
    let client = GoogleDocsClient::from_json_file(&credentials_path).map_err(|e| {
        eprintln!("Failed to initialize Google Docs client: {:?}", e);
        anyhow::anyhow!("Failed to initialize client")
    })?;

    // Create MCP server
    let server = GoogleDocsMcpServer::new(client);

    eprintln!("Google Docs MCP Server starting...");
    eprintln!("Using service account credentials from: {}", credentials_path);

    // Run with stdio transport
    let service = server.serve(stdio()).await?;

    // Wait for service to complete
    service.waiting().await?;

    Ok(())
}
