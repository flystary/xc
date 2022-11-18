use crate::utils::net::get_token_by_resp;
use crate::utils::net::init_yaml;
use futures::executor::block_on;
use serde_json::Value;

pub fn get_pop_url_by_mode(mode: &str) -> Option<String> {
    let u = init_yaml();
    if let Some(pop) = u.get_pop_route(mode) {
        return Some(pop);
    }
    None
}

pub async fn get_pop_text(base: String) -> String {
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

fn decode(mode: &str) -> Option<Value> {
    if let Some(base) = get_pop_url_by_mode(mode) {
        let text = block_on(get_pop_text(base));
        let v = serde_json::from_str(text.as_str()).unwrap();
        return Some(v);
    }
    None
}

pub fn get_pops(mode: &str) -> Option<Vec<Value>> {
    if let Some(value) = decode(mode) {
        match value {
            Value::Array(vs) => return Some(vs),
            _ => return None,
        }
    }
    None
}

pub fn get_pop(mode: &str, id: i64) -> Option<Value> {
    if let Some(pops) = get_pops(mode) {
        for pop in pops {
            if pop["id"] == id {
                return Some(pop);
            }
        }
        return None;
    }
    None
}
