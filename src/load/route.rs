extern crate serde;
extern crate serde_yaml;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub url: String,
    pub token: String,
    pub operation: String,
    modes: Vec<String>,
    valor: Valor,
    tassadar: Tassadar,
    nexus: Nexus,
    watsons: Watsons,
    watsonsha: WatsonsHa,
}
#[derive(Debug, Serialize, Deserialize)]
struct Valor {
    cpe: String,
    pop: String,
    dvc: String,
    pse: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Tassadar {
    pop: String,
    cpe: String,
    dvc: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Watsons {
    pop: String,
    cpe: String,
    dvc: String,
    pse: String,
}
#[warn(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
struct WatsonsHa {
    cpe: String,
    pop: String,
    dvc: String,
    pse: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Nexus {
    pop: String,
    cpe: String,
    dvc: String,
}

impl Route {
    pub fn get_cpe_route(self, mode: &str) -> Option<String> {
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
            "valor" => Some(format!("{}/valor/{}?", self.url, self.valor.dvc)),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.url, self.tassadar.dvc)),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.url, self.watsons.dvc, self.watsons.pse
            )),
            "watsonsha" => Some(format!("{}/watsons_ha/{}?", self.url, self.watsonsha.dvc)),
            "nexus" => Some(format!("{}/nexus/{}?", self.url, self.nexus.dvc)),
            _ => None,
        }
    }
}

pub fn load_route(path: String) -> Route {
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", path, e),
    };
    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    serde_yaml::from_str(&str).unwrap()
}
