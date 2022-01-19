use std::fs::File;
use serde::{Deserialize, Serialize};


pub fn oss(file: String, sn: String) -> (String, String, i64, String, i64, String){
    #[derive(Serialize, Deserialize)]
    pub struct Oss {
        sn: String,
        model: String,
        software_version: String,
        alias: Option<String>,
        master_entry_id: Option<i64>,
        master_entry_ip: String,
        backup_entry_id: Option<i64>,
        backup_entry_ip: String,
        //   server_ip: Option<String>,
    }
      // let f = File::open("/etc/xc/nexus_cpe_kv.json").unwrap();
    let f = File::open(file).unwrap();
    let u: Vec<Oss> = serde_json::from_reader(f).unwrap();
    
    if sn.len() <= 10 && sn.len() >= 25 {
        println!("{} error", sn);
    }

    let mut model = String::new();
    let mut version = String::new();
    let mut master_id: i64 = 0;
    let mut master_ip = String::new();
    let mut backup_id: i64 = 0;
    let mut backup_ip = String::new();

    for o in u {
    // let data = u.get(i).expect("No find data");
        if o.sn.to_string() == sn.to_string() {
            model = o.model;
            version = o.software_version;
            master_id = o.master_entry_id.unwrap() as i64;
            master_ip = o.master_entry_ip;
            backup_id = o.backup_entry_id.unwrap() as i64;
            backup_ip = o.backup_entry_ip;
            break
        } else {
            continue;
        }
    };

    return (model, version, master_id, master_ip, backup_id, backup_ip);

}
