# InstrumentService

```rust
// разрешение провайдера - дело cli
InstrumentService::cache(provider: DataProvider) -> Result<(), AvinError>
InstrumentService::clear(provider: DataProvider) -> Result<(), AvinError>

InstrumentService::find(
    provider: DataProvider,
    code: &str
) -> Result<InstrumentInfo, AvinError>

InstrumentService::find_figi(
    provider: DataProvider,
    figi: &str
) -> Result<InstrumentInfo, AvinError>

InstrumentService::list(
    provider: DataProvider,
    category: Category
) -> Result<InstrumentList, AvinError>
```

## InstrumentCatalog

```rust
InstrumentCatalog::cache(provider: DataProvider) -> Result<(), AvinError>
InstrumentCatalog::clear(provider: DataProvider) -> Result<(), AvinError>

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

# DataService

```rust
// простой и тупой вариант, service собирает avin_storage::StorageKey
DataService::sync(
    provider: DataProvider,
    code: Option<String>,
    md: Option<MarketData>,
    year: Option<Year>,
    force: bool,
) -> Result<(), AvinError>

DataService::resume() -> Result<(), AvinError>
DataService::abort() -> Result<(), AvinError>
DataService::status() -> Result<StorageStatus, AvinError>

DataService::load(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    range: TimeRange,
) -> Result<DataFrame, AvinError>
DataService::compact() -> Result<(), AvinError>
DataService::prune() -> Result<(), AvinError>
DataService::delete(key: StorageKey) -> Result<(), AvinError>
```

## DataSyncer

```rust
DataSyncer::sync(key: StorageKey, force: bool) -> Result<(), AvinError>
DataSyncer::resume() -> Result<(), AvinError>
DataSyncer::abort() -> Result<(), AvinError>
DataSyncer::status() -> Result<StorageStatus, AvinError>
```

### Impl
```rust
struct DataSyncTask {
    provider: Provider,
    iid: InstrumentId,
    market_data: MarketData,
    year: Year,
}

struct DataSyncPlan {
    tasks: Vec<DataSyncTask>,
}
DataSyncPlan::try_from(manifest: &DataManifest) -> Result<Self, AvinError>

```

## DataManager

```rust
DataManager::load(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    range: TimeRange,
) -> Result<DataFrame, AvinError>
DataManager::compact() -> Result<(), AvinError>
DataManager::prune() -> Result<(), AvinError>
DataManager::delete(key: DataSelector) -> Result<(), AvinError>
```
