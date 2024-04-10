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
    yifeng: Valor,
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

impl Route {
    pub fn get_cpe_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!(
                "{}/valor/{}?page=1&pageSize={}&",
                self.initurl, self.valor.cpe, self.valor.pse
            )),
            "yifeng" => Some(format!(
                "{}/yifeng/valor/{}?page=1&pageSize={}&",
                self.initurl, self.yifeng.cpe, self.yifeng.pse
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
            _ => None,
        }
    }

    pub fn get_pop_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}", self.initurl, self.valor.pop)),
            "yifeng" => Some(format!("{}/yifeng/valor/{}", self.initurl, self.yifeng.pop)),
            "tassadar" => Some(format!("{}/tassadar/{}", self.initurl, self.tassadar.pop)),
            "watsons" => Some(format!("{}/watsons/{}", self.initurl, self.watsons.pop)),
            "watsonsha" => Some(format!("{}/watsons_ha/{}", self.initurl, self.watsonsha.pop)),
            _ => None,
        }
    }

    pub fn get_dve_route(self, mode: &str) -> Option<String> {
        match mode {
            "valor" => Some(format!("{}/valor/{}?", self.initurl, self.valor.dve)),
            "yifeng" => Some(format!("{}/yifeng/valor/{}?", self.initurl, self.yifeng.dve)),
            "tassadar" => Some(format!("{}/tassadar/{}?", self.initurl, self.tassadar.dve)),
            "watsons" => Some(format!(
                "{}/watsons/{}?page=1&pageSize={}&",
                self.initurl, self.watsons.dve, self.watsons.pse
            )),
            "watsonsha" => Some(format!("{}/watsons_ha/{}?", self.initurl, self.watsonsha.dve)),
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
