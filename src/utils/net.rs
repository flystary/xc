use crate::utils::{cpe::*, dve::*, pop::*};
use serde_json::Value;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tokio::task;

use super::init::init_conf;
use super::ucpe::Ucpe;
use super::ucpes::Ucpes;

/// 全局共享状态（线程安全）
pub static CPES: Lazy<RwLock<Vec<Value>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub static POPS: Lazy<RwLock<Vec<Value>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub static DVES: Lazy<RwLock<Vec<Value>>> = Lazy::new(|| RwLock::new(Vec::new()));

async fn handle(mode: &str) {
    let (cpes, pops, dves, _) = tokio::join!(
        get_cpes(mode),
        get_pops(mode),
        get_dves(mode),
        task::spawn_blocking(|| super::init::init_token())
    );

    if let Some(data) = cpes {
        *CPES.write().await = data;
    }
    if let Some(data) = pops {
        *POPS.write().await = data;
    }
    if let Some(data) = dves {
        *DVES.write().await = data;
    }
}

/// 请求认证服务器获取 token 响应
pub async fn do_get_resp() -> Result<HashMap<std::string::String, Value>, reqwest::Error> {
    let sys = init_conf().sys;
    let client = reqwest::Client::new();
    let url = format!(
        "{}/matrix/oauth/token?client_id=browser&client_secret={}&grant_type=password&password={}&username={}",
        sys.loginurl,
        sys.secret,
        super::tools::md5(super::tools::md5(sys.password)),
        sys.username
    );

    client
        .post(url)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await
}

/// 从响应中提取 token
pub async fn get_token_by_resp() -> Option<String> {
    do_get_resp().await.ok().and_then(|res| {
        res.get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    })
}

/// 获取 CPES 的只读副本
pub async fn get_cpes_data() -> Vec<Value> {
    CPES.read().await.clone()
}

/// 获取 POPS 的只读副本
pub async fn get_pops_data() -> Vec<Value> {
    POPS.read().await.clone()
}

/// 获取 DVES 的只读副本
pub async fn get_dves_data() -> Vec<Value> {
    DVES.read().await.clone()
}

/// 根据 SN 列表和 mode 获取 Ucpes
pub async fn get_cpes_by_sn_mode(mode: &str, cpesns: Vec<&str>) -> Option<Ucpes> {
    // 确保数据最新
    handle(mode).await;

    let cpes = CPES.read().await.clone();
    let pops = POPS.read().await.clone();
    let dves = DVES.read().await.clone();

    let mut ucpes: Ucpes = Vec::new();

    // 预构建 pop 映射，加速查找
    let mut pop_map = HashMap::new();
    for pop in pops {
        if let Some(id) = pop["id"].as_i64() {
            pop_map.insert(id, pop);
        }
    }

    // 遍历目标 SN
    for sn_filter in cpesns {
        let mut sn = String::new();
        let mut model = String::new();
        let mut version = String::new();
        let mut port = String::new();
        let mut enterprise = String::new();
        let mut alias = String::new();
        let mut updatetime = String::new();
        let mut masterpopip = String::new();
        let mut mastercpeip = String::new();
        let mut backuppopip = String::new();
        let mut backupcpeip = String::new();
        let mut mid: i64 = 0;
        let mut bid: i64 = 0;

        // ---- 查找 CPE ----
        if let Some(cpe) = cpes.iter().find(|c| c["sn"].as_str() == Some(sn_filter)) {
            sn = cpe["sn"].as_str().unwrap_or_default().to_string();
            model = cpe["model"].as_str().unwrap_or_default().to_string();
            version = cpe["softwareVersion"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            alias = cpe["alias"].as_str().unwrap_or_default().to_string();

            updatetime = match mode {
                "tassadar" => cpe["popUpdateTime"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                _ => cpe["entryUpdateTime"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            };

            match mode {
                "valor" | "tassadar" => {
                    mastercpeip = cpe["masterPopIp"].as_str().unwrap_or_default().to_string();
                    backupcpeip = cpe["backupPopIp"].as_str().unwrap_or_default().to_string();
                    mid = cpe["masterPopId"].as_i64().unwrap_or(0);
                    bid = cpe["backupPopId"].as_i64().unwrap_or(0);
                }
                _ => {
                    mastercpeip = cpe["masterEntryIp"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    backupcpeip = cpe["backupEntryIp"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    mid = cpe["masterEntryId"].as_i64().unwrap_or(0);
                    bid = cpe["backupEntryId"].as_i64().unwrap_or(0);
                }
            }
        }

        // ---- 查找 DVE ----
        if let Some(device) = dves.iter().find(|d| d["sn"].as_str() == Some(sn_filter)) {
            if let Some(p) = device["serverPort"].as_i64() {
                port = p.to_string();
            }

            enterprise = match mode {
                "watsons" => "watsons".to_string(),
                "watsonsha" => "watsonsha".to_string(),
                _ => device["customer"]["name"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            };
        }

        // ---- 查找 POP ----
        if let Some(pop) = pop_map.get(&mid) {
            masterpopip = match mode {
                "valor" => pop["popIp"].as_str().unwrap_or_default().to_string(),
                _ => pop["entryIp"].as_str().unwrap_or_default().to_string(),
            };
        }
        if let Some(pop) = pop_map.get(&bid) {
            backuppopip = match mode {
                "valor" => pop["popIp"].as_str().unwrap_or_default().to_string(),
                _ => pop["entryIp"].as_str().unwrap_or_default().to_string(),
            };
        }

        // ---- 组装 Ucpe ----
        let ucpe = Ucpe {
            sn,
            model,
            version,
            updatetime,
            masterpopip,
            mastercpeip,
            backupcpeip,
            backuppopip,
            port,
            enterprise,
            alias,
        };

        ucpes.push(ucpe);
    }

    Some(ucpes)
}

pub async fn get_cpe_by_sn_and_mode(cpesn: &str, mode: &str) -> Option<Ucpe> {
    // 先找出目标 CPE
    let cpe = get_cpes(mode)
        .await?
        .into_iter()
        .find(|c| c["sn"].as_str() == Some(cpesn))?;

    let sn = cpe["sn"].as_str().unwrap_or_default().to_string();
    let model = cpe["model"].as_str().unwrap_or_default().to_string();
    let version = cpe["softwareVersion"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let alias = cpe["alias"].as_str().unwrap_or_default().to_string();

    let updatetime = match mode {
        "tassadar" => cpe["popUpdateTime"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        _ => cpe["entryUpdateTime"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    };

    let (mut mid, mut bid) = (0, 0);
    let (mut mastercpeip, mut backupcpeip) = (String::new(), String::new());

    match mode {
        "valor" | "tassadar" => {
            mastercpeip = cpe["masterPopIp"].as_str().unwrap_or_default().to_string();
            backupcpeip = cpe["backupPopIp"].as_str().unwrap_or_default().to_string();
            mid = cpe["masterPopId"].as_i64().unwrap_or(0);
            bid = cpe["backupPopId"].as_i64().unwrap_or(0);
        }
        _ => {
            mastercpeip = cpe["masterEntryIp"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            backupcpeip = cpe["backupEntryIp"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            mid = cpe["masterEntryId"].as_i64().unwrap_or(0);
            bid = cpe["backupEntryId"].as_i64().unwrap_or(0);
        }
    }

    // 查 POP
    let mut masterpopip = String::new();
    let mut backuppopip = String::new();

    if let Some(p) = get_pop(mode, mid).await {
        masterpopip = match mode {
            "valor" => p["popIp"].as_str().unwrap_or_default().to_string(),
            _ => p["entryIp"].as_str().unwrap_or_default().to_string(),
        };
    }
    if let Some(p) = get_pop(mode, bid).await {
        backuppopip = match mode {
            "valor" => p["popIp"].as_str().unwrap_or_default().to_string(),
            _ => p["entryIp"].as_str().unwrap_or_default().to_string(),
        };
    }

    // 查 DVE
    let mut port = String::new();
    let mut enterprise = String::new();

    if let Some(device) = get_dve(mode, cpesn).await {
        if let Some(p) = device["serverPort"].as_i64() {
            port = p.to_string();
        }
        enterprise = match mode {
            "watsons" => "watsons".to_string(),
            "watsonsha" => "watsonsha".to_string(),
            _ => device["customer"]["name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        };
    }

    Some(Ucpe {
        sn,
        model,
        version,
        updatetime,
        masterpopip,
        mastercpeip,
        backupcpeip,
        backuppopip,
        port,
        enterprise,
        alias,
    })
}
