use std::sync::Arc;

use crate::{app::AppContext, resources};
use async_trait::async_trait;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow, WebContentType,
};

pub struct SwaggerMiddleware {
    app: Arc<AppContext>,
    index: usize,
}

impl SwaggerMiddleware {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self {
            app,
            index: find_index(),
        }
    }
}

#[async_trait]
impl HttpServerMiddleware for SwaggerMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path().to_lowercase();

        if !path.starts_with("/swagger") {
            return get_next.next(ctx).await;
        }

        if path == "/swagger/index.html" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Html),
                content: get_index_page_content(self.app.as_ref(), self.index).await,
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui.css" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Css),
                content: resources::SWAGGER_UI_CSS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui-bundle.js" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: resources::SWAGGER_UI_BUNDLE_JS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui-standalone-preset.js" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: resources::SWAGGER_UI_STANDALONE_PRESET_JS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/favicon-32x32.png" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: resources::FAVICON_32.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/favicon-16x16.png" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: resources::FAVICON_16.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        let scheme = ctx.request.get_scheme();

        let host = ctx.request.get_host();

        if path == "/swagger" {
            let new_url = format!("{}://{}/swagger/index.html", scheme, host);

            let output = HttpOutput::Redirect {
                url: new_url,
                permanent: false,
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        let result = my_http_server::files::get(format!("./wwwroot{}", path).as_str()).await;

        match result {
            Ok(content) => {
                let output = HttpOutput::Content {
                    headers: None,
                    content_type: None,
                    content,
                };
                return Ok(HttpOkResult {
                    write_telemetry: false,
                    output,
                });
            }
            _ => {
                let new_url = format!("{}://{}/swagger/index.html", scheme, host);
                let output = HttpOutput::Redirect {
                    url: new_url,
                    permanent: false,
                };
                return Ok(HttpOkResult {
                    write_telemetry: false,
                    output,
                });
            }
        }
    }
}

async fn get_index_page_content(app: &AppContext, index: usize) -> Vec<u8> {
    let mut result = Vec::new();

    result.extend_from_slice(&crate::resources::INDEX_PAGE[..index]);
    add_swagger_data(&mut result, app).await;
    result.extend_from_slice(&crate::resources::INDEX_PAGE[index..]);

    result
}

async fn add_swagger_data(result: &mut Vec<u8>, app: &AppContext) {
    let mut no = 0;
    for route in &app.settings.get_routes().await {
        if no > 0 {
            result.push(',' as u8);
        }

        result.extend_from_slice("{ \"url\": \"".as_bytes());
        result.extend_from_slice(route.url.as_bytes());

        result.extend_from_slice("\", \"name\": \"".as_bytes());
        result.extend_from_slice(route.name.as_bytes());
        result.extend_from_slice("\" }".as_bytes());

        no += 1;
    }
}

fn find_index() -> usize {
    for index in 0..crate::resources::INDEX_PAGE.len() - 1 {
        if crate::resources::INDEX_PAGE[index] == '[' as u8
            && crate::resources::INDEX_PAGE[index + 1] == ']' as u8
        {
            return index + 1;
        }
    }

    panic!("Can not find where to put array")
}
