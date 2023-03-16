use crate::utils::cpe::*;
use crate::utils::dve::*;
use crate::utils::pop::*;

#[warn(unused_imports)]
use serde_json::Value;
use std::collections::HashMap;

use tokio::task;
use tokio::runtime::Runtime;
use super::init::init_conf;
use super::ucpe::Ucpe;
use super::ucpes::Ucpes;

static mut CPES: Vec<Value> = Vec::new();
static mut POPS: Vec<Value> = Vec::new();
static mut DVES: Vec<Value> = Vec::nwe();

async fn handle(mode: &str) {

    let rt = Runtime::new().unwrap();
    let task1 = task::spawn(async move {
        if let Some(data) = get_cpes(&mode).await {
            CPES = data
        }
    });

    let task2 = task::spawn(async move {
        if let Some(data) = get_pops(&mode).await {
            POPS = data
        }
    });

    let task3 = task::spawn(async move {
        if let Some(data) = get_dves(&mode).await {
            DVES = data
        }
    });

    tokio::select! {
        _ = task1 => {},
        _ = task2 => {},
        _ = task3 => {},
        _ = tokio::task::spawn_blocking(||super::init::init_token()).await.unwrap() => {},
    }
}

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
        .post(url.as_str())
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, Value>>()
        .await
}

pub async fn get_token_by_resp() -> Option<String> {
    if let Ok(res) = do_get_resp().await {
        if let Some(Value::String(token)) = res.get("access_token") {
            return Some(token.to_string());
        }
    }
    None
}


pub async fn get_cpes_by_sn_mode(mode: &str, cpesns: Vec<&str>) -> Option<Ucpes> {

    let mut ucpes:Vec<Ucpe> = Vec::new(); //table

    handle(mode).await;

    for cpesn in cpesns {
        let mut mid = 0;
        let mut bid = 0;
        let mut sn = String::new();
        let mut model = String::new();
        let mut version = String::new();
        let mut port = String::new();
        let mut enterprise = String::new();
        let mut alias = String::new();

        let mut updatetime  = String::new();
        let mut masterpopip = String::new();
        let mut mastercpeip = String::new();
        let mut backuppopip = String::new();
        let mut backupcpeip = String::new();

        for cpe in &CPES {
            if cpe["sn"] == *cpesn {
                if let Value::String(s) = &cpe["sn"] {
                    sn = s.to_string();
                }
                if let Value::String(m) = &cpe["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &cpe["softwareVersion"] {
                    version = v.to_string();
                }
                 if let Value::String(a) = &cpe["alias"] {
                    alias = a.to_string();
                }
                //updatetime
                match mode {
                    "tassadar" => {
                        if let Value::String(t) = &cpe["popUpdateTime"] {
                            updatetime = t.to_string();
                        }
                    }
                    _ => {
                        if let Value::String(t) = &cpe["entryUpdateTime"] {
                            updatetime = t.to_string();
                        }
                    }
                }
                // master/backup
                match mode {
                    "valor" => {
                        if let Value::String(m) = &cpe["masterPopIp"] {
                            mastercpeip = m.to_string();
                        }
                        if let Value::String(b) = &cpe["backupPopIp"] {
                            backupcpeip = b.to_string();
                        }
                        if let Value::Number(id) = &cpe["masterPopId"] {
                            mid = id.as_i64().unwrap();
                        }
                        if let Value::Number(id) = &cpe["backupPopId"] {
                            bid = id.as_i64().unwrap();
                        }
                    }
                    "tassadar" => {
                        if let Value::String(m) = &cpe["masterPopIp"] {
                            mastercpeip = m.to_string();
                        }
                        if let Value::String(b) = &cpe["backupPopIp"] {
                            backupcpeip = b.to_string();
                        }
                        if let Value::Number(id) = &cpe["masterPopId"] {
                            mid = id.as_i64().unwrap();
                        }
                        if let Value::Number(id) = &cpe["backupPopId"] {
                            bid = id.as_i64().unwrap();
                        }
                    }
                    _ => {
                        if let Value::String(m) = &cpe["masterEntryIp"] {
                            mastercpeip = m.to_string();
                        }
                        if let Value::String(b) = &cpe["backupEntryIp"] {
                            backupcpeip = b.to_string();
                        }
                        if let Value::Number(id) = &cpe["masterEntryId"] {
                            mid = id.as_i64().unwrap();
                        }
                        if let Value::Number(id) = &cpe["backupEntryId"] {
                            bid = id.as_i64().unwrap();
                        }
                    }
                }
            }
        }

        for device in &DVES {
            if device["sn"] == *cpesn {
                if let Value::Number(p) = &device["serverPort"] {
                    port = p.to_string();
                    //break;
                }
                match mode {
                    "watsons" => {
                         enterprise = "watsons".to_string();
                    }
                    "watsonsha" => {
                         enterprise = "watsonsha".to_string();
                    }
                    _ => {
                        if let Value::String(d) = &device["customer"]["name"] {
                            enterprise = d.to_string();
                        }
                    }
                }
            }
        }
        match mode {
            "valor" => {
                for pop in &POPS {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["popIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &POPS {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["popIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            _ => {
                for pop in &POPS {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["entryIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &POPS {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["entryIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
        }

        let uu = Ucpe {
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

        ucpes.push(uu)
    }
    Some(ucpes.to_vec())
}

pub async fn get_cpe_by_sn_and_mode(cpesn: &str, mode: &str) -> Option<Ucpe> {
    let mut mid = 0;
    let mut bid = 0;

    let mut cpe = Value::Null;

    let mut sn = String::new();
    let mut model = String::new();
    let mut version = String::new();
    let mut port = String::new();
    let mut enterprise = String::new();
    let mut alias = String::new();


    let mut updatetime  = String::new();
    let mut masterpopip = String::new();
    let mut mastercpeip = String::new();
    let mut backuppopip = String::new();
    let mut backupcpeip = String::new();

    if let Some(values) = get_cpes(mode).await {
        for value in values {
            if cpesn == value["sn"] {
                cpe = value
            }
        }
    }
    if let Value::String(s) = &cpe["sn"] {
        sn = s.to_string();
    }
    if let Value::String(m) = &cpe["model"] {
        model = m.to_string();
    }
    if let Value::String(v) = &cpe["softwareVersion"] {
        version = v.to_string();
    }
    if let Value::String(a) = &cpe["alias"] {
        alias = a.to_string();
    }
    //updatetime
    match mode {
        "tassadar" => {
            if let Value::String(t) = &cpe["popUpdateTime"] {
                updatetime = t.to_string();
            }
        }
        _ => {
            if let Value::String(t) = &cpe["entryUpdateTime"] {
                updatetime = t.to_string();
            }
        }
    }
    // master/backup
    match mode {
        "valor" => {
            if let Value::String(m) = &cpe["masterPopIp"] {
                mastercpeip = m.to_string();
            }
            if let Value::String(b) = &cpe["backupPopIp"] {
                backupcpeip = b.to_string();
            }
            if let Value::Number(id) = &cpe["masterPopId"] {
                mid = id.as_i64().unwrap();
            }
            if let Value::Number(id) = &cpe["backupPopId"] {
                bid = id.as_i64().unwrap();
            }
        }
        "tassadar" => {
            if let Value::String(m) = &cpe["masterPopIp"] {
                mastercpeip = m.to_string();
            }
            if let Value::String(b) = &cpe["backupPopIp"] {
                backupcpeip = b.to_string();
            }
            if let Value::Number(id) = &cpe["masterPopId"] {
                mid = id.as_i64().unwrap();
            }
            if let Value::Number(id) = &cpe["backupPopId"] {
                bid = id.as_i64().unwrap();
            }
        }
        _ => {
            if let Value::String(m) = &cpe["masterEntryIp"] {
                mastercpeip = m.to_string();
            }
            if let Value::String(b) = &cpe["backupEntryIp"] {
                backupcpeip = b.to_string();
            }
            if let Value::Number(id) = &cpe["masterEntryId"] {
                mid = id.as_i64().unwrap();
            }
            if let Value::Number(id) = &cpe["backupEntryId"] {
                bid = id.as_i64().unwrap();
            }
        }
    }

    match mode {
        "valor" => {
            if let Some(p) = get_pop(mode, mid).await {
                if let Value::String(m) = &p["popIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid).await {
                if let Value::String(b) = &p["popIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        _ => {
            if let Some(p) = get_pop(mode, mid).await {
                if let Value::String(m) = &p["entryIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid).await {
                if let Value::String(b) = &p["entryIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
    }
    if let Some(device) = get_dve(mode, cpesn).await {
        if let Value::Number(p) = &device["serverPort"] {
            port = p.to_string();
        }
        match mode {
            "watsons" => {
                enterprise = "watsons".to_string();
            }
            "watsonsha" => {
                enterprise = "watsonsha".to_string();
            }
            _ => {
                if let Value::String(d) = &device["customer"]["name"] {
                    enterprise = d.to_string();
                }
           }
       }
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
