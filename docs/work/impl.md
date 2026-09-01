# Implementation plan

1. Добавить в DataManifest метод:

```rust
fn contains(
    &self,
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    year: Year,
) -> bool
```

2. avin_core реализовать объекты:

Time
TimeRange
