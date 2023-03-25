use std::sync::Arc;

use crate::app::AppContext;

mod app;
mod http;
mod resources;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::SettingsReader::new(".my-swagger-hub").await;

    let app = AppContext::new(settings);

    let app = Arc::new(app);

    crate::http::start_up::setup_server(&app);

    app.app_states.wait_until_shutdown().await;
}
