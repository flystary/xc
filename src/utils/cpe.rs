use futures::executor::block_on;
use serde_json::Value;

pub fn get_cpe_url_by_mode(mode: &str) -> Option<String> {
    let u = super::init::init_route();
    if let Some(cpe) = u.get_cpe_route(mode) {
        return Some(cpe);
    }
    None
}

pub async fn get_cpe_text(base: String) -> String {
    let url = format!(
        "{}access_token={}&_={}",
        base,
        super::net::TOKEN.lock().unwrap(),
        super::tools::get_unixtime(),
    );
    reqwest::blocking::get(url.as_str())
        .unwrap()
        .text()
        .unwrap()
}

fn decode(mode: &str) -> Option<Value> {
    if let Some(base) = get_cpe_url_by_mode(mode) {
        let text = block_on(get_cpe_text(base));
        let v = serde_json::from_str(text.as_str()).unwrap();
        return Some(v);
    }
    None
}

pub fn get_cpes(mode: &str) -> Option<Vec<Value>> {
    if let Some(value) = decode(mode) {
        match value {
            Value::Array(vs) => return Some(vs),
            // Value::Object(map) => {
            //     if let Value::Array(vs) = map["data"].clone() {
            //         return Some(vs)
            //     }
            // },
            Value::Object(map) => {
                let vs = map["data"].as_array().unwrap().to_vec();
                return Some(vs);
            }
            _ => return None,
        }
    }
    None
}
