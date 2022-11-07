use crate::utils::net::init_yaml;
use serde_json::Value;
use futures::executor::block_on;
use crate::utils::net::get_token_by_resp;

pub async fn get_cpes(base: String) -> String {
    let mut token = String::new();
    let resp_token = get_token_by_resp().await;
    if let Some(tk) = resp_token {
        token = tk
    }
    let url = format!(
        "{}?pageSize=1000&access_token={}&_={}",
        base,
        token,
        super::tools::get_unixtime(),
    );
    reqwest::blocking::get(url.as_str())
        .unwrap()
        .text()
        .unwrap()
}

pub fn get_cpe(mode: &str, sn: &str) -> Option<Value> {
    if let Some(base) = get_cpe_url_by_mode(mode) {
        let text = block_on(get_cpes(base));
        if let Value::Object(object) = serde_json::from_str(text.as_str()).unwrap() {
            if let Value::Array(value) = object["data"].clone() {
                for cpe in value {
                    if cpe["sn"] == sn {
                        return Some(cpe);
                    }
                }
            }
            return None;
        }
    }
    None
}

pub fn get_cpe_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(cpe) = u.get_cpe_string(mode) {
        return Some(cpe);
    }
    None
}