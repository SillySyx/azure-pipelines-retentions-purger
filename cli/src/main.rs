mod azure_devops;
mod config;

use azure_devops::{AzureDevOpsResponse, AzureDevOpsBuild, AzureDevOpsRetentionLease};
use tokio::main;
use http_client::{HttpRequest, HttpMethod, send_request};

use config::Config;

#[main]
async fn main() {
    let config = Config::from_env_vars()
        .expect("Failed to read config from env vars");

    let header = config.authorization_header();

    let builds = load_builds(&header, &config).await
        .expect("Failed to load builds");

    let amount = builds.len();

    println!("loaded {} retained builds", builds.len());

    let mut current_build = 0usize;
    for build in builds {
        current_build += 1;

        let leases = match load_build_leases(&header, &config, &build).await {
            Some(value) => value,
            None => continue,
        };

        if leases.is_empty() {
            println!("[{current_build}/{amount}] build {} has no leases", build);
            continue;
        }

        let leases = leases.join(",");

        delete_build_lease(&header, &config, &leases).await;

        println!("[{current_build}/{amount}] removed leases for build {}", build);
    }
}

async fn load_builds(auth_header: &str, config: &Config) -> Option<Vec<String>> {
    let request = HttpRequest::new()
        .with_method(HttpMethod::Get)
        .with_url(format!("https://dev.azure.com/{}/{}/_apis/build/builds", &config.organization, &config.project))
        .with_header("Authorization", auth_header)
        .with_query_param("api-version", "6.0")
        .with_query_param("definitions", &config.pipeline);

    let response = send_request(request).await
        .expect("Failed to send request");

    let builds = response.body_as::<AzureDevOpsResponse<Vec<AzureDevOpsBuild>>>().expect("Failed to deserialize response");

    let ids = builds
        .value
        .iter()
        .map(|build| build.id.to_string())
        .collect();

    Some(ids)
}

async fn load_build_leases(auth_header: &str, config: &Config, build_id: &str) -> Option<Vec<String>> {
    let request = HttpRequest::new()
        .with_method(HttpMethod::Get)
        .with_url(format!("https://dev.azure.com/{}/{}/_apis/build/builds/{}/leases", &config.organization, &config.project, build_id))
        .with_header("Authorization", auth_header)
        .with_query_param("api-version", "7.1-preview.1");

    let response = send_request(request).await
        .expect("Failed to send request");
    
    let leases = response.body_as::<AzureDevOpsResponse<Vec<AzureDevOpsRetentionLease>>>().expect("Failed to deserialize response");

    let ids = leases
        .value
        .iter()
        .map(|lease| lease.leaseId.to_string())
        .collect();

    Some(ids)
}

async fn delete_build_lease(auth_header: &str, config: &Config, lease_id: &str) {
    let request = HttpRequest::new()
        .with_method(HttpMethod::Delete)
        .with_url(format!("https://dev.azure.com/{}/{}/_apis/build/retention/leases", &config.organization, &config.project))
        .with_header("Authorization", auth_header)
        .with_query_param("api-version", "6.0")
        .with_query_param("ids", lease_id);

    send_request(request).await
        .expect("Failed to send request");
}