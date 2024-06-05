use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpRequestHeaders,
    HttpServerMiddleware, HttpServerRequestFlow, WebContentType,
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
        let path = ctx.request.get_path();

        for route in self.app.settings.get_routes().await {
            if route.url == path.as_str() {
                let mut fl_url = flurl::FlUrl::new(route.remote_url.as_str());

                if let Some(host) = ctx.request.get_headers().try_get_case_insensitive("host") {
                    let host = host.as_str().unwrap();
                    println!("Overridden host: {}", host);
                    fl_url = fl_url.with_header("Host", host);
                }

                fl_url = fl_url.with_header("X-Forwarded-Proto", get_scheme(&ctx));

                match fl_url.get().await {
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
                            write_to_log: true,
                        });
                    }
                }
            }
        }

        get_next.next(ctx).await
    }
}

fn get_scheme(header_map: &HttpContext) -> &str {
    if let Some(value) = header_map
        .request
        .get_headers()
        .try_get_case_insensitive("x-forwarded-proto")
    {
        return value.as_str().unwrap();
    }

    if let Some(value) = header_map
        .request
        .get_headers()
        .try_get_case_insensitive("scheme")
    {
        return value.as_str().unwrap();
    }

    return "http";
}
