extern crate serde;
extern crate serde_yaml;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub initurl: String,
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
    dve: String,
    pse: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Tassadar {
    pop: String,
    cpe: String,
    dve: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Watsons {
    pop: String,
    cpe: String,
    dve: String,
    pse: String,
}
#[warn(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
struct WatsonsHa {
    cpe: String,
    pop: String,
    dve: String,
    pse: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Nexus {
    pop: String,
    cpe: String,
    dve: String,
}

impl Route {
    pub fn get_cpe_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!(
                "{}/valor/{}?page=1&pageSize={}&",
                self.initurl, self.valor.cpe, self.valor.pse
            )),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.initurl, self.watsons.cpe, self.watsons.pse
            )),
            "watsonsha" => Some(format!(
                "{}/watsons_ha/{}?page=1&pageSize={}&",
                self.initurl, self.watsonsha.cpe, self.watsonsha.pse
            )),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.initurl, self.tassadar.cpe)),
            "nexus" => Some(format!("{}/nexus/{}?", self.initurl, self.nexus.cpe)),
            _ => None,
        }
    }

    pub fn get_pop_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}", self.initurl, self.valor.pop)),
            "tassadar" => Some(format!("{}/tassadar/{}", self.initurl, self.tassadar.pop)),
            "watsons" => Some(format!("{}/watsons/{}", self.initurl, self.watsons.pop)),
            "watsonsha" => Some(format!("{}/watsons_ha/{}", self.initurl, self.watsonsha.pop)),
            "nexus" => Some(format!("{}/nexus/{}", self.initurl, self.nexus.pop)),
            _ => None,
        }
    }

    pub fn get_dve_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}?", self.initurl, self.valor.dve)),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.initurl, self.tassadar.dve)),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.initurl, self.watsons.dve, self.watsons.pse
            )),
            "watsonsha" => Some(format!("{}/watsons_ha/{}?", self.initurl, self.watsonsha.dve)),
            "nexus" => Some(format!("{}/nexus/{}?", self.initurl, self.nexus.dve)),
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
