use rust_extensions::ApplicationStates;

use crate::settings::SettingsModel;

pub struct AppContext {
    pub settings: SettingsModel,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self { settings }
    }
}

impl ApplicationStates for AppContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn is_shutting_down(&self) -> bool {
        false
    }
}
