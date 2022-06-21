use std::sync::Arc;

use hyper::HeaderMap;
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
                let headers = ctx.request.get_headers();

                let mut fl_url = flurl::FlUrl::new(route.remote_url.as_str());

                if let Some(host) = get_host(&headers) {
                    println!("Overrided host: {}", host);
                    fl_url = fl_url.with_header("Host", host);
                }

                fl_url = fl_url.with_header("X-Forwarded-Proto", get_scheme(&headers));

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
                        });
                    }
                }
            }
        }

        get_next.next(ctx).await
    }
}

fn get_host(header_map: &HeaderMap) -> Option<&str> {
    let result = header_map.get("host")?;
    result.to_str().ok()
}

fn get_scheme(header_map: &HeaderMap) -> &str {


    if header_map.contains_key("X-Forwarded-Proto") {
        let result = header_map.get("X-Forwarded-Proto").unwrap();
        return result.to_str().unwrap();
    }

    if header_map.contains_key("Scheme") {
       let result = header_map.get("Scheme").unwrap();
       return result.to_str().unwrap();
    }

    return "http"


}
