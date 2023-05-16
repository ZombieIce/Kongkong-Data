use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub port: String,
    pub host: String,
}

impl Server {
    fn generate_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn generate_url(&self) -> String {
        self.server.generate_url()
    }
}

pub fn load_server() -> Settings {
    let yaml_str = include_str!("../../local_settings.yaml");
    let settings = serde_yaml::from_str(yaml_str).expect("local_settings.yaml read failed!");
    settings
}
