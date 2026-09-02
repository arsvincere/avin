# Time

```rust
struct Time(i64)

impl Time {
    pub fn new(timestamp_nanos: i64) -> Self
    pub fn dt(&self) -> DateTime<Utc>
    pub fn ts(&self) -> i64
}

impl Display for Time {
    fn fmt(...)
}

impl FromStr {
    // available formats
    // from_str("2026-01-01 12:55:00")
    // from_str("2026-01-01 12:55")
    // from_str("2026-01-01")
    fn from_str(s: &str) -> Result<Time, CoreError>
}
```
