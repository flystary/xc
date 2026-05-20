use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpeBase {
    pub sn: String,
    pub model: String,
    pub software_version: String,
    pub alias: String,
    #[serde(alias = ["popUpdateTime", "EntryUpdateTime"])]
    pub pop_update_time: String,
    #[serde(alias = ["masterPopId", "masterEntryId"])]
    pub master_pop_id: u64,
    #[serde(alias = ["backupPopId", "backupEntryId"])]
    pub backup_pop_id: u64,
    #[serde(alias = ["masterPopIp", "masterEntryIp"])]
    pub master_pop_ip: String,
    #[serde(alias = ["backupPopIp", "backupEntryIp"])]
    pub backup_pop_ip: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PopBase {
    pub id: u64,
    pub name: String,
    #[serde(alias = ["popIp", "entryIp"])]
    pub pop_ip: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Customer {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DveBase {
    pub sn: String,
    pub software_version: String,
    pub status: String,
    pub customer_name: String,
    pub server_port: u64,
    pub customer: Customer,
}
