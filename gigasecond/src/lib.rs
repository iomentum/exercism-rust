use chrono::{{DateTime, Utc},TimeZone};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.timestamp(start.timestamp() + 1_000_000_000, 0)
}
