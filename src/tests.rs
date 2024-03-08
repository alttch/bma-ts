use crate::Timestamp;
#[test]
#[allow(
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]
fn test_time() {
    let timestamp = 1_632_093_707.189_334_9;
    let time = Timestamp::from_secs_f64(timestamp);
    assert_eq!(time.as_secs_f64(), timestamp);
    assert_eq!(time.as_nanos(), 1_632_093_707_189_334_869);
    let timestamp_ns = 1_632_093_707_123_456_789;
    let time = Timestamp::from_nanos(timestamp_ns);
    assert_eq!(time.as_nanos() as u64, timestamp_ns);
    assert_eq!(time.as_secs_f64(), 1_632_093_707.123_456_7);
    assert_eq!(time.as_millis() as u64, timestamp_ns / 1_000_000);
    assert_eq!(time.as_micros() as u64, timestamp_ns / 1_000);
    let timestamp_us = 1_632_093_707_123_456;
    let time = Timestamp::from_micros(timestamp_us);
    assert_eq!(time.as_secs_f64(), 1_632_093_707.123_456);
    assert_eq!(time.as_millis() as u64, timestamp_us / 1_000);
    assert_eq!(time.as_micros() as u64, timestamp_us);
    assert_eq!(time.as_nanos() as u64, timestamp_us * 1_000);
    let timestamp_ms = 1_632_093_707_123_u128;
    let time = Timestamp::from_millis(timestamp_ms as u64);
    assert_eq!(time.as_secs_f64(), 1_632_093_707.123);
    assert_eq!(time.as_millis(), timestamp_ms);
    assert_eq!(time.as_micros(), timestamp_ms * 1_000);
    assert_eq!(time.as_nanos(), timestamp_ms * 1_000_000);
}
