use serde_json::Value;

pub fn get_dve_url_by_mode(mode: &str) -> Option<String> {
    let u = super::init::init_route();
    if let Some(cpe) = u.get_dve_route(mode) {
        return Some(cpe);
    }
    None
}

pub async fn get_dve_text(base: String) -> String {
    let url = format!(
        "{}access_token={}&_={}",
        base,
        *super::init::TOKEN,
        super::tools::get_unixtime(),
    );
    reqwest::get(url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn decode(mode: &str) -> Option<Value> {
    if let Some(base) = get_dve_url_by_mode(mode) {
        let text = get_dve_text(base).await;
        let v = serde_json::from_str(text.as_str()).unwrap();
        return Some(v);
    }
    None
}

pub async fn get_dves(mode: &str) -> Option<Vec<Value>> {
    if let Some(value) = decode(mode).await {
        match value {
            Value::Array(vs) => return Some(vs),
            Value::Object(map) => {
                let vs = map["data"].as_array().unwrap().to_vec();
                return Some(vs);
            }
            _ => return None,
        }
    }
    None
}

pub async fn get_dve(mode: &str, sn: &str) -> Option<Value> {
    if let Some(dves) = get_dves(mode).await {
        for dve in dves {
            if dve["sn"] == *sn {
                return Some(dve);
            }
        }
        return None;
    }
    None
}
