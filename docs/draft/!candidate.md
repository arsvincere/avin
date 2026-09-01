Записи здесь:

- не считаются отсутствующей функциональностью;
- не реализуются и не удаляются без отдельного решения;

# Loader

не утверждено, может и не надо

## Api

```rust
Loader::chart(code: &str, tf: TimeFrame) -> Result<Chart, AvinError>
Loader::chart_range(
    code: &str,
    tf: TimeFrame,
    range: TimeRange
) -> Result<Chart, AvinError>
Loader::ticks(code: &str) -> Result<Vec<Tick>, AvinError>
Loader::footprint(code: &str, tf: TimeFrame) -> Result<Footprint, AvinError>

// и то же самое со scoped provider
```

## Call

продумать позже если понадобится

# Storage

## Api

```rust
Storage::cache() -> Result<(), AvinError>
Storage::sync() -> Result<(), AvinError>
Storage::delete(code: &str, md: MarketData) -> Result<(), AvinError>

// и то же самое со scoped provider
```

## Call
cache -> InstrumentCatalog -> InstrumentInfoStorage
sync -> DataManager -> MarketDataStorage
delete -> DataManager -> MarketDataStorage

