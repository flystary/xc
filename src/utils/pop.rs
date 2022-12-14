use serde_json::Value;

pub fn get_pop_url_by_mode(mode: &str) -> Option<String> {
    let u = super::init::init_route();
    if let Some(pop) = u.get_pop_route(mode) {
        return Some(pop);
    }
    None
}

pub async fn get_pop_text(base: String) -> String {
    let url = format!(
        "{}?access_token={}&_={}",
        base,
        super::init::TOKEN.to_string(),
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
    if let Some(base) = get_pop_url_by_mode(mode) {
        let text = get_pop_text(base).await;
        let v = serde_json::from_str(text.as_str()).unwrap();
        return Some(v);
    }
    None
}

pub async fn get_pops(mode: &str) -> Option<Vec<Value>> {
    if let Some(value) = decode(mode).await {
        match value {
            Value::Array(vs) => return Some(vs),
            _ => return None,
        }
    }
    None
}

pub async fn get_pop(mode: &str, id: i64) -> Option<Value> {
    if let Some(pops) = get_pops(mode).await {
        for pop in pops {
            if pop["id"] == id {
                return Some(pop);
            }
        }
        return None;
    }
    None
}
