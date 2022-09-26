use std::sync::Arc;

use rust_extensions::AppStates;

use crate::settings::SettingsModel;

pub struct AppContext {
    pub settings: SettingsModel,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self {
            settings,
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
