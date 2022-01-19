use std::fs::File;
use serde::{Deserialize, Serialize};


pub fn net(file: String, m_id: i64, b_id: i64)  -> (String, String) {
    #[derive(Serialize, Deserialize)]
    pub struct Net {
        id: i64,
        name: String,
        entry_ip: String,
        entry_type: String,
        need_sync: bool,
    }

    let f = File::open(file).unwrap();
        // let f = File::open("/etc/xc/nexus_net_kv.json").unwrap();
    let u: Vec<Net> = serde_json::from_reader(f).unwrap();
    let mut ip_1 = String::new();
    let mut ip_2 = String::new();
    for n  in &u {
        // let data = u.get(i).expect("No find data");
        if n.id == m_id {
            ip_1 = n.entry_ip.to_string();
            break
        } else {
            continue
        }
    }

    for n in u {
        if n.id == b_id {
            ip_2 = n.entry_ip.to_string();
            break
        } else {
            continue
        } 
    }
    return (ip_1, ip_2)

}



