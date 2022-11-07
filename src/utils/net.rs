use crate::conf::toml::{load_conf, Conf};
use crate::conf::yaml::{load_url, Url};
use crate::utils::cpe::*;
use crate::utils::dve::*;
use crate::utils::pop::*;
use crate::utils::ucpe::{
    Ucpe,
    Ucpes,
    // Ucpe,
};
use futures::executor::block_on;
#[warn(unused_imports)]
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn init_toml() -> Conf {
    let mut path = PathBuf::new();
    if let Ok(buf) = super::conf::get_default_config("xc.toml") {
        path = buf
    }
    load_conf(path)
}

pub fn init_yaml() -> Url {
    load_url(init_toml().sys.path)
}

pub async fn do_get_resp() -> Result<HashMap<std::string::String, Value>, reqwest::Error> {
    let sys = init_toml().sys;
    let client = reqwest::blocking::Client::new();
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
        .unwrap()
        .json::<HashMap<String, Value>>()
}

pub async fn get_token_by_resp() -> Option<String> {
    let result = do_get_resp().await;
    match result {
        Ok(v) => {
            if let Some(Value::String(token)) = v.get("access_token") {
                return Some(token.to_string());
            }
        }
        Err(e) => {
            println!("get token error:{}", e);
            return None;
        }
    }
    None
}

pub fn get_cpes_by_sn_mode(mode: &str, cpesns: Vec<&str>) -> Option<Ucpes> {
    let mut dtext = String::new();
    let mut ptext = String::new();
    let mut ucpes = Vec::new(); //table
    let mut cpes: Vec<Value> = Vec::new(); //http

    if let Some(data) = get_cpes(mode) {
        cpes = data
    }

    if let Some(base) = get_dve_url_by_mode(mode) {
        dtext = block_on(get_dve_text(base));
    }
    let d: Vec<Value> = serde_json::from_str(dtext.as_str()).unwrap();

    if let Some(base) = get_pop_url_by_mode(mode) {
        ptext = block_on(get_pop_text(base));
    }
    let p: Vec<Value> = serde_json::from_str(ptext.as_str()).unwrap();
    for cpesn in cpesns {
        let mut mid = 0;
        let mut bid = 0;
        let mut sn = String::new();
        let mut model = String::new();
        let mut version = String::new();
        let mut remoteport = String::new();

        let mut updatetime = String::new();
        let mut masterpopip = String::new();
        let mut mastercpeip = String::new();
        let mut backuppopip = String::new();
        let mut backupcpeip = String::new();

        for cpe in &cpes {
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
                match mode {
                    "nexus" => {
                        if let Value::String(t) = &cpe["entryUpdateTime"] {
                            updatetime = t.to_string();
                        }
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
                    "watsons" => {
                        if let Value::String(t) = &cpe["entryUpdateTime"] {
                            updatetime = t.to_string();
                        }
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
                    "watsonsha" => {
                        if let Value::String(t) = &cpe["entryUpdateTime"] {
                            updatetime = t.to_string();
                        }
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
                    "valor" => {
                        if let Value::String(t) = &cpe["entryUpdateTime"] {
                            updatetime = t.to_string();
                        }
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
                        if let Value::String(t) = &cpe["popUpdateTime"] {
                            updatetime = t.to_string();
                        }
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
                        println!("Unknown mode: {}", mode);
                    }
                }
                break;
            }
        }

        for device in &d {
            if device["sn"] == *cpesn {
                if let Value::Number(p) = &device["serverPort"] {
                    remoteport = p.to_string();
                    break;
                }
            }
        }

        match mode {
            "nexus" => {
                for pop in &p {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["entryIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &p {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["entryIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            "watsons" => {
                for pop in &p {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["entryIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &p {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["entryIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            "watsonsha" => {
                for pop in &p {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["entryIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &p {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["entryIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            "valor" => {
                for pop in &p {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["popIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &p {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["popIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            "tassadar" => {
                for pop in &p {
                    if pop["id"] == mid {
                        if let Value::String(m) = &pop["entryIp"] {
                            masterpopip = m.to_string();
                            break;
                        }
                    }
                }
                for pop in &p {
                    if pop["id"] == bid {
                        if let Value::String(m) = &pop["entryIp"] {
                            backuppopip = m.to_string();
                            break;
                        }
                    }
                }
            }
            _ => {
                println!("Unknown mode: {}", mode);
            }
        }

        ucpes.push(Ucpe {
            sn,
            model,
            version,
            updatetime,
            masterpopip,
            mastercpeip,
            backupcpeip,
            backuppopip,
            remoteport,
        })
    }
    Some(ucpes)
}

pub fn get_cpe_by_sn_and_mode(cpesn: &str, mode: &str) -> Option<Ucpe> {
    let mut mid = 0;
    let mut bid = 0;

    let mut cpe = Value::Null;

    let mut sn = String::new();
    let mut model = String::new();
    let mut version = String::new();
    let mut remoteport = String::new();

    let mut updatetime = String::new();
    let mut masterpopip = String::new();
    let mut mastercpeip = String::new();
    let mut backuppopip = String::new();
    let mut backupcpeip = String::new();

    if let Some(values) = get_cpes(mode) {
        for value in values {
            if value["sn"] == cpesn {
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
    match mode {
        "nexus" => {
            if let Value::String(t) = &cpe["entryUpdateTime"] {
                updatetime = t.to_string();
            }
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

            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        "watsons" => {
            if let Value::String(t) = &cpe["entryUpdateTime"] {
                updatetime = t.to_string();
            }
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

            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        "watsonsha" => {
            if let Value::String(t) = &cpe["entryUpdateTime"] {
                updatetime = t.to_string();
            }
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

            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        "valor" => {
            if let Value::String(t) = &cpe["entryUpdateTime"] {
                updatetime = t.to_string();
            }
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

            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["popIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["popIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        "tassadar" => {
            if let Value::String(t) = &cpe["popUpdateTime"] {
                updatetime = t.to_string();
            }
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

            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    masterpopip = m.to_string();
                }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backuppopip = b.to_string();
                }
            }
        }
        _ => {
            println!("Unknown mode: {}", mode);
        }
    }
    if let Some(device) = get_dve(mode, cpesn) {
        if let Value::Number(p) = &device["serverPort"] {
            remoteport = p.to_string();
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
        remoteport,
    })
}
