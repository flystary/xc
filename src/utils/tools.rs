use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn get_unixtime() -> i64 {
    let times = time::get_time();
    times.sec * 1000 + (times.nsec as f64 / 1000.0 / 1000.0) as i64
}

pub fn md5<S: Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}
