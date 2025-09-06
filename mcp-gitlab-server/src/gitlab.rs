use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
    },
    model::*,
    prompt_handler, prompt_router,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

#[derive(Clone)]
pub struct GitLabTool {
    base_url: String,
    private_token: Option<String>,
    tool_router: ToolRouter<GitLabTool>,
    prompt_router: PromptRouter<GitLabTool>,
}

#[tool_router]
impl GitLabTool {
    pub fn new() -> Self {
        Self {
            base_url: "https://gitlab.com/api/v4".to_string(),
            private_token: None,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }
    #[tool(description = "Get user details by username")]
    async fn get_user(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("hello")]))
    }
    #[tool(description = "Say hello")]
    fn say_hello(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("Hello from GitLab!")]))
    }
}

#[prompt_router]
impl GitLabTool {
    
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for GitLabTool {    
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides GitLab tools and prompts.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}