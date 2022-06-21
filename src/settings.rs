use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

const SETTINGS_FILE_NAME: &str = ".my-swagger-hub";
#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub routes: Vec<Route>,
}

impl SettingsModel {
    pub async fn read_settings() -> Self {
        let file_name = get_settings_filename();

        let mut file = File::open(file_name).await.unwrap();

        let mut file_content: Vec<u8> = vec![];
        file.read_to_end(&mut file_content).await.unwrap();

        let result: SettingsModel = serde_yaml::from_slice(file_content.as_slice()).unwrap();

        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub url: String,
    pub remote_url: String,
    pub host: String,
    pub scheme: String,
}

fn get_settings_filename() -> String {
    let path = env!("HOME");

    if path.ends_with('/') {
        return format!("{}{}", path, SETTINGS_FILE_NAME);
    }

    return format!("{}{}{}", path, "/", SETTINGS_FILE_NAME);
}
