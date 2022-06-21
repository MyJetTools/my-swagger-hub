use std::sync::Arc;

use crate::app::AppContext;

mod app;
mod http;
mod resources;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::SettingsModel::read_settings().await;

    let app = AppContext::new(settings);

    let app = Arc::new(app);

    crate::http::start_up::setup_server(&app);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
