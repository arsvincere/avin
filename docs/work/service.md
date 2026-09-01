# InstrumentCatalog

```rust
InstrumentCatalog::cache(provider: DataProvider) -> Result<(), AvinError>

InstrumentCatalog::find(
    provider: DataProvider,
    code: &str,
) -> Result<InstrumentInfo, AvinError>

InstrumentCatalog::find_iid(
    provider: DataProvider,
    iid: &InstrumentId,
) -> Result<InstrumentInfo, AvinError>

InstrumentCatalog::find_figi(
    provider: DataProvider,
    figi: &str,
) -> Result<InstrumentInfo, AvinError>

InstrumentCatalog::list(
    provider: DataProvider,
    category: Category,
) -> Result<InstrumentList, AvinError>
```

# DataManager

```rust
DataManager::sync(provider: DataProvider) -> Result<(), AvinError>

DataManager::load(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    range: TimeRange,
) -> Result<DataFrame, AvinError>

DataManager::delete(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
) -> Result<(), AvinError>
```
