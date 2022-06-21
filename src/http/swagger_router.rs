use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow, WebContentType,
};

use crate::app::AppContext;

pub struct SwaggerRouterMiddleware {
    app: Arc<AppContext>,
}

impl SwaggerRouterMiddleware {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for SwaggerRouterMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path_lower_case();

        for route in &self.app.settings.routes {
            if route.url == path {
                match flurl::FlUrl::new(route.remote_url.as_str())
                    .with_header("Host", route.host.as_str())
                    .with_header("X-Forwarded-Proto", route.scheme.as_str())
                    .get()
                    .await
                {
                    Ok(result) => {
                        let body = result.receive_body().await.unwrap();

                        let output = HttpOutput::Content {
                            headers: None,
                            content_type: Some(WebContentType::Json),
                            content: body,
                        };
                        return Ok(HttpOkResult {
                            write_telemetry: false,
                            output,
                        });
                    }
                    Err(err) => {
                        return Err(HttpFailResult {
                            write_telemetry: false,
                            status_code: 503,
                            content_type: WebContentType::Text,
                            content: format!("{:?}", err).into_bytes(),
                        });
                    }
                }
            }
        }

        get_next.next(ctx).await
    }
}
