use time::OffsetDateTime;

pub fn now_utc() -> OffsetDateTime {
  now_utc_impl()
}

#[cfg(not(target_arch = "wasm32"))]
fn now_utc_impl() -> OffsetDateTime {
  OffsetDateTime::now_utc()
}

#[cfg(target_arch = "wasm32")]
fn now_utc_impl() -> OffsetDateTime {
  let millis = js_sys::Date::now() as i64;
  OffsetDateTime::from_unix_timestamp_nanos((millis * 1_000_000) as i128).expect("valid timestamp")
}
