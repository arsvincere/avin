# Time

Основной тип для представления времени в AVIN.

Технически - Unix timestamp в наносекундах, обертка над `i64`.

Допустимый интервал значений от `1677-09-21 00:12:43` до `2262-04-11 23:47:16`.

**ВАЖНО!!!**
Метод `from_str` всегда интерпретирует исходную строку как UTC.
Локальная тайм-зона не применяется автоматически.
Например, если вы имеете ввиду московское время (MSK) `2026-09-02 10:00` нужно писать `2026-09-02 07:00`.

## Примеры:

```rust
use std::str::FromStr;

use chrono::{Utc, TimeZone};

use avin_core::Time;

// Создание из строки (время интерпретируется как UTC)
let time = Time::from_str("2026-01-01 12:55:19").unwrap();
assert_eq!(time.to_string(), "2026-01-01 12:55:19");

// Преобразование в chrono::DateTime<Utc>:
let dt = time.dt();
assert_eq!(dt, Utc.with_ymd_and_hms(2026, 1, 1, 12, 55, 19).unwrap());
```

# TimeRange

Полу-открытый интервал времени `[begin, end)`

`begin` - входит в интервал.
`end`   - **НЕ** входит в интервал.

## Примеры:

```rust
use std::str::FromStr;

use avin_core::{Time, TimeRange};

let begin = Time::from_str("2025-01-01").unwrap();
let end = Time::from_str("2026-01-01").unwrap();
let range = TimeRange::new(begin, end).unwrap();

assert_eq!(range.to_string(), "[2025-01-01, 2026-01-01)");

let inside = Time::from_str("2025-01-02 12:55:03").unwrap();
assert!(range.contains(inside));

let outside = Time::from_str("2020-01-01").unwrap();
assert!(!range.contains(outside));
```
