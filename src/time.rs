//! Time-Related Functions and Types

pub use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
pub use prost_types::Timestamp;

/// Converts a prost Timestamp value to a chrono DateTime<Utc> value.
/// Will return None if Timestamp `nanos` value is negative
pub fn timestamp_to_datetime(ts: &Timestamp) -> Option<DateTime<Utc>> {
    let nanos: u32 = ts.nanos.try_into().ok()?;
    let ndt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(ts.seconds, nanos)?;
    Some(DateTime::<Utc>::from_utc(ndt, Utc))
}

/// Converts a chrono DateTime<Utc> value to a prost Timestamp value.
pub fn datetime_to_timestamp(dt: &DateTime<Utc>) -> Option<Timestamp> {
    let seconds = dt.timestamp();

    let nanos: i32 = dt.timestamp_subsec_nanos().try_into().ok()?;

    Some(Timestamp { seconds, nanos })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn ut_timestamp_to_datetime() {
        let mut ts: Timestamp = Timestamp {
            seconds: 0,
            nanos: 0,
        };
        assert!(timestamp_to_datetime(&ts).is_some());

        // Handles negative seconds
        ts.seconds = -1;
        assert!(timestamp_to_datetime(&ts).is_some());

        // Returns None on negative nanos
        ts.nanos = -1;
        assert!(timestamp_to_datetime(&ts).is_none());
    }

    #[test]
    fn ut_datetime_to_timestamp() {
        let dt: DateTime<Utc> = Utc::now();
        assert!(datetime_to_timestamp(&dt).is_some());

        // DateTime stores subsec_nanos as u32
        // Timestamp stores subsec_nanos as i32
        //
        // Utc panics if the subsec nanos field is invalid
        // Invalid nanos value is anything greater than 1_500_000_000 (1e9 + leap second)
        // Should be impossible to create a subsec_nanos value in DateTime<Utc> that
        //  exceeds i32::MAX == 2_147_483_647
    }
}
