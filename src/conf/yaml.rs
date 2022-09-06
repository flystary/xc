extern crate serde_yaml;
extern crate serde;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
	pub url:       String,
        pub token:     String,
	pub operation: String,
        modes:      Vec<String>,
        valor:      Valor,
        tassadar:   Tassadar,
        nexus:      Nexus,
        watsons:    Watsons,
        watsons_ha: WatsonsHa,
}
#[derive(Debug, Serialize, Deserialize)]
struct Tassadar{
	pop: String,
	cpe: String,
    dvc: String
}
#[derive(Debug, Serialize, Deserialize)]
struct Nexus{
	pop: String,
	cpe: String,
    dvc: String
}
#[derive(Debug, Serialize, Deserialize)]
struct Watsons{
	pop: String,
	cpe: String,
    dvc: String,
}
#[warn(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
struct WatsonsHa{
	cpe: String,
	pop: String,
    dvc: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Valor{
	cpe: String,
	pop: String,
    dvc: String,
}

impl Url {
    pub fn get_pop_string(self,mode: &str) -> Option<String> {

        match mode {
            "tassadar" => Some(format!("{}/{}/{}",self.url,mode, self.tassadar.pop)),
            "nexus"    => Some(format!("{}/{}/{}",self.url,mode, self.nexus.pop)),
            "watsons"  => Some(format!("{}/{}/{}",self.url,mode, self.watsons.pop)),
            "watsons_ha" => Some(format!("{}/{}/{}",self.url,mode, self.watsons_ha.pop)),
            "valor"    => Some(format!("{}/{}/{}",self.url,mode, self.valor.pop)),
            _    => None,
        }
    }
    pub fn get_cpe_string(self, mode: &str) -> Option<String> {
        match mode {
            "tassadar" => Some(format!("{}/{}/{}",self.url, mode, self.tassadar.cpe)),
            "nexus"    => Some(format!("{}/{}/{}",self.url, mode, self.nexus.cpe)),
            "watsons"  => Some(format!("{}/{}/{}",self.url, mode, self.watsons.cpe)),
            "watsons_ha" => Some(format!("{}/{}/{}",self.url, mode, self.watsons_ha.cpe)),
            "valor"    => Some(format!("{}/{}/{}",self.url, mode, self.valor.cpe)),
            _    => None,
        }
    }
    pub fn get_device_string(self, mode: &str) -> Option<String> {
        match mode {
            "tassadar" => Some(format!("{}/{}/{}",self.url, mode, self.tassadar.dvc)),
            "nexus"    => Some(format!("{}/{}/{}",self.url, mode, self.nexus.dvc)),
            "watsons"  => Some(format!("{}/{}/{}",self.url, mode, self.watsons.dvc)),
            "watsons_ha" => Some(format!("{}/{}/{}",self.url, mode, self.watsons_ha.dvc)),
            "valor"    => Some(format!("{}/{}/{}",self.url, mode, self.valor.dvc)),
            _    => None,
        }
    }

}


pub fn load_url(path: String) -> Url {
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", path, e)
    };
    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    serde_yaml::from_str(&str).unwrap()
}
