use chrono::Local;

pub fn now_timestamp() -> i64 {
    let now = Local::now();
    return now.timestamp();
}

