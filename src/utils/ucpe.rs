use colored::*;
use std::process::{Command, ExitStatus};
use tabled::{settings::Style, Table, Tabled};

const CONNECT_SCRIPT: &str = "/etc/xc/bin/connet";
const EXPECT_BIN: &str = "/usr/bin/expect";

#[derive(Tabled, Clone)]
pub struct Ucpe {
    pub sn: String,
    pub model: String,
    pub version: String,
    pub updatetime: String,
    pub masterpopip: String,
    pub mastercpeip: String,
    pub backuppopip: String,
    pub backupcpeip: String,
    pub port: String,
    pub enterprise: String,
    pub alias: String,
}

pub trait Dis {
    fn display(&self);
}

impl Ucpe {
    pub fn display(&self) {
        // let v = vec![self];
        let table = Table::new(vec![self])
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            // .with(Style::NO_BORDER)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            //.with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Modify::new(Row(0..1)).with(Format(|s| s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s| s.to_string())));

        println!("{}", table);
    }
}

pub trait Con {
    fn check_master(&self) -> bool;
    fn check_backup(&self) -> bool;
    fn conn_master(&self);
    fn conn_backup(&self);

    // fn open_report(&self)  -> bool;
    // fn close_report(&self) -> bool;
    // fn conn_report(&self);
}

impl Con for Ucpe {
    fn check_master(&self) -> bool {
        if self.mastercpeip.is_empty() && self.masterpopip.is_empty() {
            return false;
        }
        true
    }
    fn check_backup(&self) -> bool {
        if self.backupcpeip.is_empty() && self.backuppopip.is_empty() {
            return false;
        }
        true
    }
    fn conn_master(&self) {
        self.perform_connect(&self.masterpopip, &self.mastercpeip, "Master");
    }

    fn conn_backup(&self) {
        self.perform_connect(&self.backuppopip, &self.backupcpeip, "Backup");
    }
}

impl Ucpe {
    fn perform_connect(&self, pop_ip: &str, cpe_ip: &str, label: &str) {
        let conf = super::init::init_conf();

        if pop_ip == "0.0.0.0" || cpe_ip == "0.0.0.0" {
            eprintln!("{}", format!("Error: {} IP is 0.0.0.0", label).red().bold());
            return;
        }

        if pop_ip.is_empty() || cpe_ip.is_empty() || conf.jump.username.is_empty() {
            eprintln!(
                "{}",
                format!("Error: {} info or credentials missing", label)
                    .red()
                    .bold()
            );
            return;
        }

        if cfg!(target_os = "linux") {
            let mut cmd = Command::new(EXPECT_BIN);
            cmd.arg(CONNECT_SCRIPT)
                .arg(pop_ip)
                .arg(cpe_ip)
                .arg(&conf.jump.username)
                .arg(&conf.jump.password);

            if matches!(self.model.as_str(), "7XEC2000-260" | "7XEC2000-100") {
                cmd.arg("ucpe");
            }

            match cmd.status() {
                Ok(status) if status.success() => println!("{} login successful.", label),
                Ok(status) => eprintln!("{}", format!("{} login failed: {}", label, status).red()),
                Err(e) => eprintln!("{}", format!("Failed to execute expect: {}", e).red()),
            }
        } else {
            println!(
                "{}",
                "Warning: Connection logic only supports Linux.".yellow()
            );
        }
    }
}
