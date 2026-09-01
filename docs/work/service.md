# InstrumentService

```rust
InstrumentService::cache(scope: InstrumentScope) -> Result<(), AvinError>
InstrumentService::clear(scope: InstrumentScope) -> Result<(), AvinError>

InstrumentService::cache() -> Result<(), AvinError>
InstrumentService::cache_provider(provider: DataProvider) -> Result<(), AvinError>
InstrumentService::clear() -> Result<(), AvinError>
InstrumentService::clear_provider(provider: DataProvider) -> Result<(), AvinError>

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

### Impl

```rust
enum InstrumentScope {
    __Manifest, // User? Default? UserDefault? UserManifest?
    Provider(DataProvider),
}
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
DataService::sync(
    scope: DataSelector,
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
DataService::delete(scope: DataSelector) -> Result<(), AvinError>
```

## DataSyncer

```rust
DataSyncer::sync(scope: DataSelector, force: bool) -> Result<(), AvinError>
DataSyncer::resume() -> Result<(), AvinError>
DataSyncer::abort() -> Result<(), AvinError>
DataSyncer::status() -> Result<StorageStatus, AvinError>
```

### Impl
```rust
enum DataSelector {
    Providers(Vec<DataProvider>),

    Instruments {
        provider: DataProvider,
        code: Vec<&str>,
    },

    MarketData {
        provider: DataProvider,
        code: &str,
        md: Vec<MarketData>,
    },

    Years {
        provider: DataProvider,
        code: &str,
        md: MarketData,
        year: Vec<Year>,
    },
}

impl DataSelector {
    pub fn providers(
        providers: Vec<DataProvider>,
    ) -> Result<Self, AvinError>

    pub fn instruments(
        provider: DataProvider,
        iids: Vec<InstrumentId>,
    ) -> Result<Self, AvinError>

    pub fn data(
        provider: DataProvider,
        iid: InstrumentId,
        data: Vec<MarketData>,
    ) -> Result<Self, AvinError>

    pub fn years(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        years: Vec<Year>,
    ) -> Result<Self, AvinError>
}

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
DataManager::delete(scope: DataSelector) -> Result<(), AvinError>
```
