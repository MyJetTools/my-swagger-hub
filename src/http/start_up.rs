use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::app::AppContext;

use super::SwaggerRouterMiddleware;

pub fn setup_server(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    let swagger_middleware = crate::http::SwaggerMiddleware::new(app.clone());

    http_server.add_middleware(Arc::new(SwaggerRouterMiddleware::new(app.clone())));

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.start(app.clone());
}
