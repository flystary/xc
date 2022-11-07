use crate::utils::net::get_token_by_resp;
use crate::utils::net::init_yaml;
use futures::executor::block_on;
use serde_json::Value;

pub fn get_dve_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(cpe) = u.get_dve_string(mode) {
        return Some(cpe);
    }
    None
}

pub async fn get_dve_text(base: String) -> String {
    let mut token = String::new();
    let resp_token = get_token_by_resp().await;
    if let Some(tk) = resp_token {
        token = tk
    }
    let url = format!(
        "{}?&access_token={}&_={}",
        base,
        token,
        super::tools::get_unixtime(),
    );
    reqwest::blocking::get(url.as_str())
        .unwrap()
        .text()
        .unwrap()
}

pub fn get_dve(mode: &str, sn: &str) -> Option<Value> {
    if let Some(base) = get_dve_url_by_mode(mode) {
        let text = block_on(get_dve_text(base));
        let v: Vec<Value> = serde_json::from_str(text.as_str()).unwrap();
        for cpe in v {
            if cpe["sn"] == *sn {
                return Some(cpe);
            }
        }
        return None;
    }
    None
}
