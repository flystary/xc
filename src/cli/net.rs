use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json::value::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::cmp::min;
use std::thread;
use std::time::Duration;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("net").about("update net vbox").arg(
        Arg::with_name("updata")
            .required(true)
            .help("Name of the net"),
    )
}

pub fn run(args: &ArgMatches) {
    let arg = args.value_of("updata").unwrap();
    // let sn = args.value_of("sn").unwrap();

    if arg == "updata" {
        // println!("{}", arg);
        let mut down_and_write = 0;
        let total_size = 925696;
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        // .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
        .progress_chars("#>-"));

        while down_and_write < total_size {
            let new = min(down_and_write + 520, total_size);
            down_and_write = new;
            pb.set_position(new);
            thread::sleep(Duration::from_millis(4));
        }
        // 序列化json数据
        futures::executor::block_on(write());
        pb.finish_with_message("down_and_write");
        thread::sleep(Duration::from_millis(1));
        init_cpe_json_file();
        init_net_json_file()
    } else {
        println!("不支持此命令 {}", arg)
    }

}

async fn do_get_resp() -> Result<HashMap<std::string::String, Value>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/matrix/oauth/token?client_id={}&client_secret={}&grant_type={}&password={}&username={}",
        CLENT_HOST = String::from("http://internal.oss.7x-networks.net"),
        CLIENT_ID = String::from("browser"),
        CLIENT_SECRET = String::from("b7n3i7kzg22y3p035rw3rd9sfzvs4cv0"),
        GRANT_TYPE = String::from("password"),
        PASSWORD = String::from("c8d064e2ad4670f418ba02ef342b33d1"),
        USERNAME = String::from("matrix")
    );

    Ok(client
        .post(url.as_str())
        .send()
        .unwrap()
        .json::<HashMap<String, Value>>()?)
}

async fn get_token() -> (String, i64) {
    let mut s = String::new();
    let time = time::get_time();
    let times = time.sec * 1000 + (time.nsec as f64 / 1000.0 / 1000.0) as i64;
    let result = do_get_resp().await;
    match result {
        Ok(v) => {
            // println!("{:#?}", v.get("access_token"));
            let token = v.get("access_token");
            for tk in token.unwrap().to_string().split('"') {
                if tk.len() != 0 {
                    s = tk.to_string().clone()
                };
            } // Some(b) => let tk = b.unwrap().to_string()
        }
        Err(e) => println!("Error is {}", e),
    };
    return (s, times);
}

async fn write_nexus_cpe_file(mode: String) {
    let (token, time) = get_token().await;
    let mut cpe_mode = format!("{}/", &mode);
    if mode == "nexus" {
        cpe_mode = String::from("").to_string();
    } 
    let url = format!(
        "{}/matrix/{}nexus/business/boxes?&access_token={}&_={}",
        CLENT_HOST = String::from("http://internal.oss.7x-networks.net"),
        CPE_MODE = cpe_mode,
        ACCESS_TOKEN = token,
        CLENT_TIME = time,
    );
    let res = reqwest::blocking::get(url.as_str()).unwrap();
    // println!("Status: {}", res.status());
    let body = res.text().unwrap();
    let xc = String::from("/etc/xc/cpe/");
    let path = Path::new(&xc);
    if !path.is_dir() {
        fs::create_dir(path).unwrap();
    };
    let  cpe = format!("{}/{}_cpe.json",xc, mode);
    // let  cpe = "/etc/xc/nexus_cpe.json";
    let mut file = fs::File::create(cpe).unwrap();
    file.write(&body.to_string().as_bytes()).unwrap();
}

async fn write_nexus_net_file(mode: String) {
    let (token, time) = get_token().await;
    let mut net_mode = format!("{}/", &mode);
    let mut vpn_mode = String::from("open");
    if mode == "nexus" {
        net_mode = String::from("").to_string();
        vpn_mode = String::from("").to_string();
    }
    let url = format!(
        "{}/matrix/{}nexus/{}vpn/entries?&access_token={}&_={}",
        CLENT_HOST = String::from("http://internal.oss.7x-networks.net"),
        NET_MODE =  net_mode,
        VPN_MODE = vpn_mode,
        ACCESS_TOKEN = token,
        CLENT_TIME = time,
    );
    // println!("{}", url);
    let res = reqwest::blocking::get(url.as_str()).unwrap();
    // println!("Status: {}", res.status());
    let body = res.text().unwrap();
    let xc = String::from("/etc/xc/net/");
    let path = Path::new(&xc);
    if !path.is_dir() {
        fs::create_dir(path).unwrap();
    };
    let  net = format!("{}/{}_net.json",xc, mode);

    let mut file = fs::File::create(net).unwrap();
    file.write(&body.to_string().as_bytes()).unwrap();
}


async fn write() {
    let cpe_mode = vec!("watsons", "nexus", "watsons_ha");
    for cpe in cpe_mode {
        write_nexus_cpe_file(cpe.to_string()).await;
        write_nexus_net_file(cpe.to_string()).await;
    }
}
// minicom


fn init_cpe_json_file() {
    Command::new("python3")
    .arg("/etc/xc/bin/cpe")
    .arg("-f")
    .arg("/etc/xc/cpe/nexus_cpe.json")
    .arg("/etc/xc/cpe/watsons_cpe.json")
    .arg("/etc/xc/cpe/watsons_ha_cpe.json")
    // .arg(format!("{}@{}", user_name, ip_name))
    .status()
    .expect("init cpe 数据失败!");
}


fn init_net_json_file() {
    Command::new("python3")
    .arg("/etc/xc/bin/net")
    .arg("-f")
    .arg("/etc/xc/net/nexus_net.json")
    .arg("/etc/xc/net/watsons_net.json")
    .arg("/etc/xc/net/watsons_ha_net.json")
    // .arg(format!("{}@{}", user_name, ip_name))
    .status()
    .expect("init net 数据失败!");

}