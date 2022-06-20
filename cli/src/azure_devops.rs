use serde::Deserialize;

#[derive(Deserialize)]
pub struct AzureDevOpsResponse<VALUE> {
    pub value: VALUE,
}

#[derive(Deserialize)]
pub struct AzureDevOpsBuild {
    pub id: i32,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct AzureDevOpsRetentionLease {
    pub leaseId: i32,
}