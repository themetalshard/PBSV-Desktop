use chrono::{DateTime, Local, TimeZone, Utc};

pub fn unix_to_local(epoch: i64) -> DateTime<Local> {
    let utc = Utc.timestamp_opt(epoch, 0).unwrap();
    utc.with_timezone(&Local)
}

pub fn unix_to_utc(epoch: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(epoch, 0).unwrap()
}
