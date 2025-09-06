#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use tokio;
use tracing::{info, Level};
use tracing_subscriber;
use rmcp::{
    ErrorData as McpError, model::*, tool, tool_router, handler::server::tool::ToolRouter,
};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::service::{RoleServer, serve_server};
mod gitlab {
    use rmcp::{
        ErrorData as McpError, RoleServer, ServerHandler,
        handler::server::{
            router::{prompt::PromptRouter, tool::ToolRouter},
            wrapper::Parameters,
        },
        model::*, prompt_handler, prompt_router, schemars, service::RequestContext, tool,
        tool_handler, tool_router,
    };
    pub struct GitLabTool {
        base_url: String,
        private_token: Option<String>,
        tool_router: ToolRouter<GitLabTool>,
        prompt_router: PromptRouter<GitLabTool>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GitLabTool {
        #[inline]
        fn clone(&self) -> GitLabTool {
            GitLabTool {
                base_url: ::core::clone::Clone::clone(&self.base_url),
                private_token: ::core::clone::Clone::clone(&self.private_token),
                tool_router: ::core::clone::Clone::clone(&self.tool_router),
                prompt_router: ::core::clone::Clone::clone(&self.prompt_router),
            }
        }
    }
    impl GitLabTool {
        pub fn new() -> Self {
            Self {
                base_url: "https://gitlab.com/api/v4".to_string(),
                private_token: None,
                tool_router: Self::tool_router(),
                prompt_router: Self::prompt_router(),
            }
        }
        pub fn get_user_tool_attr() -> rmcp::model::Tool {
            rmcp::model::Tool {
                name: "get_user".into(),
                description: Some("Get user details by username".into()),
                input_schema: rmcp::handler::server::common::cached_schema_for_type::<
                    rmcp::model::EmptyObject,
                >(),
                output_schema: None,
                annotations: None,
            }
        }
        fn get_user(
            &self,
        ) -> ::std::pin::Pin<
            Box<
                dyn ::std::future::Future<
                    Output = Result<CallToolResult, McpError>,
                > + Send + '_,
            >,
        > {
            Box::pin(async move {
                Ok(
                    CallToolResult::success(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([Content::text("hello")]),
                        ),
                    ),
                )
            })
        }
        pub fn say_hello_tool_attr() -> rmcp::model::Tool {
            rmcp::model::Tool {
                name: "say_hello".into(),
                description: Some("Say hello".into()),
                input_schema: rmcp::handler::server::common::cached_schema_for_type::<
                    rmcp::model::EmptyObject,
                >(),
                output_schema: None,
                annotations: None,
            }
        }
        fn say_hello(&self) -> Result<CallToolResult, McpError> {
            Ok(
                CallToolResult::success(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([Content::text("Hello from GitLab!")]),
                    ),
                ),
            )
        }
        fn tool_router() -> rmcp::handler::server::router::tool::ToolRouter<Self> {
            rmcp::handler::server::router::tool::ToolRouter::<Self>::new()
                .with_route((Self::get_user_tool_attr(), Self::get_user))
                .with_route((Self::say_hello_tool_attr(), Self::say_hello))
        }
    }
    impl GitLabTool {
        fn prompt_router() -> rmcp::handler::server::router::prompt::PromptRouter<
            GitLabTool,
        > {
            rmcp::handler::server::router::prompt::PromptRouter::new()
        }
    }
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
                instructions: Some(
                    "This server provides GitLab tools and prompts.".to_string(),
                ),
            }
        }
        async fn initialize(
            &self,
            _request: InitializeRequestParam,
            context: RequestContext<RoleServer>,
        ) -> Result<InitializeResult, McpError> {
            if let Some(http_request_part) = context
                .extensions
                .get::<axum::http::request::Parts>()
            {
                let initialize_headers = &http_request_part.headers;
                let initialize_uri = &http_request_part.uri;
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event mcp-gitlab-server/src/gitlab.rs:70",
                                "mcp_gitlab_server::gitlab",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "mcp-gitlab-server/src/gitlab.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(70u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "mcp_gitlab_server::gitlab",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message", "initialize_headers", "initialize_uri"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &value_set,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("initialize from http server") as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &debug(&initialize_headers) as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &display(&initialize_uri) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                        if match ::tracing::Level::INFO {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &{
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!("initialize from http server") as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &debug(&initialize_headers) as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &display(&initialize_uri) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                },
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    }
                };
            }
            Ok(self.get_info())
        }
        async fn call_tool(
            &self,
            request: rmcp::model::CallToolRequestParam,
            context: rmcp::service::RequestContext<rmcp::RoleServer>,
        ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
            let tcc = rmcp::handler::server::tool::ToolCallContext::new(
                self,
                request,
                context,
            );
            self.tool_router.call(tcc).await
        }
        async fn list_tools(
            &self,
            _request: Option<rmcp::model::PaginatedRequestParam>,
            _context: rmcp::service::RequestContext<rmcp::RoleServer>,
        ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
            Ok(rmcp::model::ListToolsResult::with_all_items(self.tool_router.list_all()))
        }
        async fn get_prompt(
            &self,
            request: GetPromptRequestParam,
            context: RequestContext<RoleServer>,
        ) -> Result<GetPromptResult, rmcp::ErrorData> {
            let prompt_context = rmcp::handler::server::prompt::PromptContext::new(
                self,
                request.name,
                request.arguments,
                context,
            );
            self.prompt_router.get_prompt(prompt_context).await
        }
        async fn list_prompts(
            &self,
            _request: Option<PaginatedRequestParam>,
            _context: RequestContext<RoleServer>,
        ) -> Result<ListPromptsResult, rmcp::ErrorData> {
            let prompts = self.prompt_router.list_all();
            Ok(ListPromptsResult {
                prompts,
                next_cursor: None,
            })
        }
    }
}
const BIND_ADDRESS: &str = "127.0.0.1:7000";
fn main() -> anyhow::Result<()> {
    let body = async {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
        let service = StreamableHttpService::new(
            || Ok(gitlab::GitLabTool::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        );
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event mcp-gitlab-server/src/main.rs:28",
                        "mcp_gitlab_server",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some(
                            "mcp-gitlab-server/src/main.rs",
                        ),
                        ::tracing_core::__macro_support::Option::Some(28u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "mcp_gitlab_server",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Starting GitLab MCP Server.") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!("Starting GitLab MCP Server.") as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        let router = axum::Router::new().nest_service("/mcp", service);
        let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;
        let _ = axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
            .await;
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
