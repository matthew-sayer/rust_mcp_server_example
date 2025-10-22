//Imports
use prism_mcp_rs::prelude::*; 
use std::collections::HashMap; 

#[derive(Clone)] //This allows us to create copies, new instances, of our toolhandler - so we can use it for multiple tools
struct ToolHandlerCore; // Defines the ToolHandler data structure (struct)

#[async_trait]
impl ToolHandler for ToolHandlerCore {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let name = arguments.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("World");
            
        Ok(ToolResult {
            content: vec![ContentBlock::text(format!("Hello, {}!", name))],
            is_error: Some(false),
            meta: None,
            structured_content: None,
        })
    }
}

#[tokio::main] //The :: indicates we're using the main function from Tokio crate. The crate is the library.
async fn main() -> McpResult<()> {
    //First we will create the mcp server. It is mut for mutable, so we can add tools to it.
    let mut server = McpServer::new(
        "HelloToolServer".to_string(), //server name returned as string
        "1.0.0".to_string() //server version returned as string
    );


    server.add_tool(
        "hello".to_string(), // Tool name
        Some("Says hello to the specified user.".to_string()), // Tool description
        json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the person to greet."
                }
            }
        }), // Input schema
        ToolHandlerCore, // Tool handler
    ).await?; //await the async operation, ? propagates errors if any

    //Now, we start the server asynchronously
    
    let transport = StdioServerTransport::new();
    server.start(transport).await

}

