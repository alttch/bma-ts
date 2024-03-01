use crate::Timestamp;
#[test]
fn test_time() {
    let timestamp = 1632093707.1893349;
    let time = Timestamp::from_secs_f64(timestamp);
    assert_eq!(time.as_secs_f64(), timestamp);
    assert_eq!(time.as_nanos(), 1632093707189334869);
    let timestamp_ns = 1632093707123456789;
    let time = Timestamp::from_nanos(timestamp_ns);
    assert_eq!(time.as_nanos() as u64, timestamp_ns);
    assert_eq!(time.as_secs_f64(), 1_632_093_707.123_456_7);
    assert_eq!(time.as_millis() as u64, timestamp_ns / 1_000_000);
    assert_eq!(time.as_micros() as u64, timestamp_ns / 1_000);
    let timestamp_us = 1632093707123456;
    let time = Timestamp::from_micros(timestamp_us);
    assert_eq!(time.as_secs_f64(), 1632093707.123456);
    assert_eq!(time.as_millis() as u64, timestamp_us / 1_000);
    assert_eq!(time.as_micros() as u64, timestamp_us);
    assert_eq!(time.as_nanos() as u64, timestamp_us * 1_000);
    let timestamp_ms = 1632093707123u128;
    let time = Timestamp::from_millis(timestamp_ms as u64);
    assert_eq!(time.as_secs_f64(), 1632093707.123);
    assert_eq!(time.as_millis(), timestamp_ms);
    assert_eq!(time.as_micros(), timestamp_ms * 1_000);
    assert_eq!(time.as_nanos(), timestamp_ms * 1_000_000);
}
