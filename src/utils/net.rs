#[warn(unused_imports)]
use serde_json::Value;
use std::collections::HashMap;
use futures::executor::block_on;
use crate::conf::yaml::{
    Url,
    load_url,
};
use crate::conf::toml::{
    Conf,
    load_conf,
};
use crate::utils::cpe::{
    Cpe
};

pub fn init_toml() -> Conf {
    let path = String::from("/etc/xc/xc.toml");
    load_conf(path)
}

pub fn init_yaml() -> Url {
    load_url(init_toml().sys.rulepath)
}

pub fn get_unixtime() -> i64 {
    let times = time::get_time();
    times.sec * 1000 + (times.nsec as f64 / 1000.0 / 1000.0) as i64
}

async fn do_get_resp() -> Result<HashMap<std::string::String, Value>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/matrix/oauth/token?client_id={}&client_secret={}&grant_type={}&password={}&username={}",
        CLENT_HOST = String::from("http://xxxx.xxx"),
        CLIENT_ID = String::from("browser"),
        CLIENT_SECRET = String::from("b7n3i7kzg22y3p035rw3rd9sfzvs4cv0"),
        GRANT_TYPE = String::from("password"),
        PASSWORD = String::from("c8d064e2ad4670f418ba02ef342b33d1"),
        USERNAME = String::from("matrix")
    );

    client
        .post(url.as_str())
        .send()
        .unwrap()
        .json::<HashMap<String, Value>>()
}

async fn get_token_by_resp() -> Option<String> {
    let result = do_get_resp().await;
    match result {
        Ok(v) => {
            if let Some(token) = v.get("access_token") {
                if let Value::String(token) = token {
                    return Some(token.to_string());
                }
            }
        },
        Err(e) => {
            println!("get token error:{}",e);
            return None
        }
    }
    None
}

pub async fn get_pops(base: String) -> String {
    let mut token = String::new();
    let resp_token = get_token_by_resp().await;
    if let Some(tk) = resp_token {
        token = tk
    }
    let url = format!(
        "{}?&access_token={}&_={}",
        BASE = base,
        ACCESS_TOKEN = token,
        CLENT_TIME   = get_unixtime(),
    );
    reqwest::blocking::get(url.as_str()).unwrap().text().unwrap()
}

pub async fn get_cpes(base: String) -> String{
    let mut token = String::new();
    let resp_token = get_token_by_resp().await;
    if let Some(tk) = resp_token {
        token = tk
    }
    let url = format!(
        "{}?&access_token={}&_={}",
        BASE = base,
        ACCESS_TOKEN = token,
        CLENT_TIME   = get_unixtime(),
    );
    reqwest::blocking::get(url.as_str()).unwrap().text().unwrap()
}

pub async fn get_devices(base: String) -> String{
    let mut token = String::new();
    let resp_token = get_token_by_resp().await;
    if let Some(tk) = resp_token {
        token = tk
    }
    let url = format!(
        "{}?&access_token={}&_={}",
        BASE = base,
        ACCESS_TOKEN = token,
        CLENT_TIME   = get_unixtime(),
    );
    reqwest::blocking::get(url.as_str()).unwrap().text().unwrap()
}


pub fn get_cpe(mode: &str, sn: &str) ->Option<Value> {
    if let Some(base) = get_cpe_url_by_mode(mode) {
        let text = block_on(get_cpes(base));
        let v: Vec<Value> = serde_json::from_str(text.as_str()).unwrap();
        for cpe in v {
            if cpe["sn"] == *sn {
                return Some(cpe);
            }
        }
        return None
    }
    None
}

pub fn get_pop(mode: &str, id: i64) ->Option<Value> {
    if let Some(base) = get_pop_url_by_mode(mode) {
        let text = block_on(get_pops(base));
        let v: Vec<Value> = serde_json::from_str(text.as_str()).unwrap();
        for pop in v {
            if pop["id"] == id {
                return Some(pop);
            }
        }
        return None
    }
    None
}

pub fn get_device(mode: &str, sn: &str) ->Option<Value> {
    if let Some(base) = get_device_url_by_mode(mode) {
        let text = block_on(get_devices(base));
        let v: Vec<Value>  = serde_json::from_str(text.as_str()).unwrap();
        for cpe in v {
            if cpe["sn"] == *sn {
                return Some(cpe);
            }
        }
        return None
    }
    None
}

fn get_cpe_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(cpe) = u.get_cpe_string(mode) {
        return Some(cpe)
    }
    None
}

fn get_pop_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(pop) = u.get_pop_string(mode) {
        return Some(pop)
    }
    None
}

fn get_device_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(cpe) = u.get_device_string(mode) {
        return Some(cpe)
    }
    None
}

pub fn get_cpe_by_sn_and_mode(sn: &str, mode: &str) -> Cpe {
    let mut mid = 0;
    let mut bid = 0;

    let mut cpesn = String::new();
    let mut model = String::new();
    let mut version = String::new();
    let mut cpeport = String::new();
    let mut synctime = String::new();

    let mut master_pop_ip = String::new();
    let mut master_cpe_ip = String::new();
    let mut backup_pop_ip = String::new();
    let mut backup_cpe_ip = String::new();
    match mode {
        "nexus"    => {
            if let Some(s) = get_cpe(mode, sn) {
                if let Value::String(s) = &s["sn"] {
                    cpesn = s.to_string();
                }
                if let Value::String(m) = &s["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &s["softwareVersion"] {
                    version = v.to_string();
                }
                if let Value::String(t) = &s["entryUpdateTime"] {
                    synctime = t.to_string();
                }
                if let Value::String(m) = &s["masterEntryIp"] {
                    master_cpe_ip = m.to_string();
                }
                if let Value::String(b) = &s["backupEntryIp"] {
                    backup_cpe_ip = b.to_string();
                }
                if let Value::Number(id) =  &s["masterEntryId"] {
                    mid = id.as_i64().unwrap();
                }
                if let Value::Number(id) = &s["backupEntryId"] {
                    bid = id.as_i64().unwrap();
                }
            }
            if let Some(d) = get_device(mode, sn) {
                if let Value::Number(p) = &d["serverPort"] {
                    cpeport = p.to_string();
                }
            }
            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    master_pop_ip = m.to_string();
              }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backup_pop_ip = b.to_string();
              }
            }
        },
        "watsons"  => {
            if let Some(s) = get_cpe(mode, sn) {
                if let Value::String(s) = &s["sn"] {
                    cpesn = s.to_string();
                }
                if let Value::String(m) = &s["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &s["softwareVersion"] {
                    version = v.to_string();
                }
                if let Value::String(t) = &s["entryUpdateTime"] {
                    synctime = t.to_string();
                }
                if let Value::String(m) = &s["masterEntryIp"] {
                    master_cpe_ip = m.to_string();
                }
                if let Value::String(b) = &s["backupEntryIp"] {
                    backup_cpe_ip = b.to_string();
                }
                if let Value::Number(id) = &s["masterEntryId"] {
                    mid = id.as_i64().unwrap();
                }
                if let Value::Number(id) = &s["backupEntryId"] {
                    bid = id.as_i64().unwrap();
                }
            }
            if let Some(d) = get_device(mode, sn) {
                if let Value::Number(p) = &d["serverPort"] {
                    cpeport = p.to_string();
                }
            }
            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    master_pop_ip = m.to_string();
              }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backup_pop_ip = b.to_string();
              }
            }
        },
        "watsons_ha" => {
            if let Some(s) = get_cpe(mode, sn) {
                if let Value::String(s) = &s["sn"] {
                    cpesn = s.to_string();
                }
                if let Value::String(m) = &s["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &s["softwareVersion"] {
                    version = v.to_string();
                }
                if let Value::String(t) = &s["entryUpdateTime"] {
                    synctime = t.to_string();
                }
                if let Value::String(m) = &s["masterEntryIp"] {
                    master_cpe_ip = m.to_string();
                }
                if let Value::String(b) = &s["backupEntryIp"] {
                    backup_cpe_ip = b.to_string();
                }
                if let Value::Number(id) =  &s["masterEntryId"] {
                    mid = id.as_i64().unwrap();
                }
                if let Value::Number(id) = &s["backupEntryId"] {
                    bid = id.as_i64().unwrap();
                }
            }
            if let Some(d) = get_device(mode, sn) {
                if let Value::Number(p) = &d["serverPort"] {
                    cpeport = p.to_string();
                }
            }
            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    master_pop_ip = m.to_string();
              }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backup_pop_ip = b.to_string();
              }
            }
        },
        "valor"    => {
            if let Some(s) = get_cpe(mode, sn) {
                if let Value::String(s) = &s["sn"] {
                    cpesn = s.to_string();
                }
                if let Value::String(m) = &s["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &s["softwareVersion"] {
                    version = v.to_string();
                }
                if let Value::String(t) = &s["entryUpdateTime"] {
                    synctime = t.to_string();
                }
                if let Value::String(m) = &s["masterPopIp"] {
                    master_cpe_ip = m.to_string();
                }
                if let Value::String(b) = &s["backupPopIp"] {
                    backup_cpe_ip = b.to_string();
                }
                if let Value::Number(id) =  &s["masterPopId"] {
                    mid = id.as_i64().unwrap();
                }
                if let Value::Number(id) = &s["backupPopId"] {
                    bid = id.as_i64().unwrap();
                }
            }
            if let Some(d) = get_device(mode, sn) {
                if let Value::Number(p) = &d["serverPort"] {
                    cpeport = p.to_string();
                }
            }
            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["popIp"]{
                    master_pop_ip = m.to_string();
              }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["popIp"] {
                    backup_pop_ip = b.to_string();
              }
            }
        },
        "tassadar" => {
            if let Some(s) = get_cpe(mode, sn) {
                if let Value::String(s) = &s["sn"] {
                    cpesn = s.to_string();
                }
                if let Value::String(m) = &s["model"] {
                    model = m.to_string();
                }
                if let Value::String(v) = &s["softwareVersion"] {
                    version = v.to_string();
                }
                if let Value::String(t) = &s["entryUpdateTime"] {
                    synctime = t.to_string();
                }
                if let Value::String(m) = &s["masterPopIp"] {
                    master_cpe_ip = m.to_string();
                }
                if let Value::String(b) = &s["backupPopIp"] {
                    backup_cpe_ip = b.to_string();
                }
                if let Value::Number(id) = &s["masterPopId"] {
                    mid = id.as_i64().unwrap();
                }
                if let Value::Number(id) = &s["backupPopId"] {
                    bid = id.as_i64().unwrap();
                }
            }
            if let Some(d) = get_device(mode, sn) {
                if let Value::Number(p) = &d["serverPort"] {
                    cpeport = p.to_string();
                }
            }
            if let Some(p) = get_pop(mode, mid) {
                if let Value::String(m) = &p["entryIp"] {
                    master_pop_ip = m.to_string();
              }
            }
            if let Some(p) = get_pop(mode, bid) {
                if let Value::String(b) = &p["entryIp"] {
                    backup_pop_ip = b.to_string();
              }
            }
        },
        _ => {
            println!("Unknown mode: {}", mode);
        }
    }
   Cpe::new(cpesn, model, version, synctime, cpeport, master_pop_ip, master_cpe_ip, backup_pop_ip, backup_cpe_ip)
}
