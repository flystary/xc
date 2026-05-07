extern crate serde;
extern crate serde_yaml;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FullService {
    pub cpe: String,
    pub pop: String,
    pub dve: String,
    pub pse: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub url: String,
    pub token: String,
    pub operation: String,
    pub modes: Vec<String>,
    pub valor: FullService,
    pub tassadar: FullService,
    pub nexus: FullService,
    pub watsons: FullService,
    #[serde(rename = "watsonsha")] // 映射 YAML 中的字段名
    pub watsons_ha: FullService,
}

impl Route {
    pub fn get_cpe_route(&self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!(
                "{}/valor/{}?page=1&pageSize={}&",
                self.url, self.valor.cpe, self.valor.pse
            )),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.url, self.watsons.cpe, self.watsons.pse
            )),
            "watsonsha" => Some(format!(
                "{}/watsons_ha/{}?page=1&pageSize={}&",
                self.url, self.watsonsha.cpe, self.watsonsha.pse
            )),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.url, self.tassadar.cpe)),
            "nexus" => Some(format!("{}/nexus/{}?", self.url, self.nexus.cpe)),
            _ => None,
        }
    }

    pub fn get_pop_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}", self.url, self.valor.pop)),
            "tassadar" => Some(format!("{}/tassadar/{}", self.url, self.tassadar.pop)),
            "watsons" => Some(format!("{}/watsons/{}", self.url, self.watsons.pop)),
            "watsonsha" => Some(format!("{}/watsons_ha/{}", self.url, self.watsonsha.pop)),
            "nexus" => Some(format!("{}/nexus/{}", self.url, self.nexus.pop)),
            _ => None,
        }
    }

    pub fn get_dve_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}?", self.url, self.valor.dve)),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.url, self.tassadar.dve)),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.url, self.watsons.dve, self.watsons.pse
            )),
            "watsonsha" => Some(format!("{}/watsons_ha/{}?", self.url, self.watsonsha.dve)),
            "nexus" => Some(format!("{}/nexus/{}?", self.url, self.nexus.dve)),
            _ => None,
        }
    }
}

pub fn load_route<P: AsRef<Path>>(path: P) -> Route {
    let file = File::open(path).unwrap_or_else(|e| {
        panic!("no open file: {:?}", e);
    });

    // 直接从 Reader 反序列化，效率更高
    serde_yaml::from_reader(file).expect("YAML 格式错误")
}
