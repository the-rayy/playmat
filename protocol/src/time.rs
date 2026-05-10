use time::OffsetDateTime;

#[cfg(not(target_arch = "wasm32"))]
pub fn now_utc() -> OffsetDateTime {
  OffsetDateTime::now_utc()
}

#[cfg(target_arch = "wasm32")]
#[expect(clippy::arithmetic_side_effects, reason = "we know its timestamp")]
pub fn now_utc() -> OffsetDateTime {
  let millis = js_sys::Date::now() as i128;
  OffsetDateTime::from_unix_timestamp_nanos(millis * 1_000_000).expect("valid timestamp")
}
