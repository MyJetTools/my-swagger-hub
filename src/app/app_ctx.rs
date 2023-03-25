use std::sync::Arc;

use rust_extensions::AppStates;

pub struct AppContext {
    pub settings: crate::settings::SettingsReader,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub fn new(settings: crate::settings::SettingsReader) -> Self {
        Self {
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
