use std::env::var;

pub struct Config {
    pub organization: String,
    pub project: String,
    pub pipeline: String,
    pub username: String,
    pub pat: String,
}

impl Config {
    pub fn from_env_vars() -> Option<Self> {
        let organization = match var("AZURE_ORG") {
            Ok(value) => value,
            Err(_) => return None,
        };
        let project = match var("AZURE_PROJECT") {
            Ok(value) => value,
            Err(_) => return None,
        };
        let pipeline = match var("AZURE_PIPELINE") {
            Ok(value) => value,
            Err(_) => return None,
        };
        let username = match var("AZURE_USERNAME") {
            Ok(value) => value,
            Err(_) => return None,
        };
        let pat = match var("AZURE_PAT") {
            Ok(value) => value,
            Err(_) => return None,
        };

        Some(Self {
            organization,
            project,
            pipeline,
            username,
            pat,
        })
    }

    pub fn authorization_header(&self) -> String {
        let value = format!("{}:{}", &self.username, &self.pat);
        let bytes = value.as_bytes();
        let encoded_value = crypto::base64::to_base64(bytes);
        
        format!("Basic {}", encoded_value)
    }
}