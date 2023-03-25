use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Clone)]
pub struct SettingsModel {
    pub routes: Vec<Route>,
}

impl SettingsReader {
    pub async fn get_routes(&self) -> Vec<Route> {
        let read_access = self.settings.read().await;
        read_access.routes.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Route {
    pub name: String,
    pub url: String,
    pub remote_url: String,
}
