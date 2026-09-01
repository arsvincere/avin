# Instrument

## Api

```rust
// используется [default] data_provider из пользовательского config.toml
Instrument::find(code: &str) -> Result<InstrumentInfo, AvinError>
Instrument::find_figi(figi: &str) -> Result<InstrumentInfo, AvinError>
Instrument::list(category: Category) -> Result<InstrumentList, AvinError>

// и то же самое со scoped provider
Instrument::provider(provider: DataProvider)
    .find(code: &str) -> Result<InstrumentInfo, AvinError>

Instrument::provider(provider: DataProvider)
    .find_figi(figi: &str) -> Result<InstrumentInfo, AvinError>

Instrument::provider(provider: DataProvider)
    .list(category: Category) -> Result<InstrumentList, AvinError>
```

## Call

find -> InstrumentCatalog -> InstrumentInfoStorage
find_figi -> InstrumentCatalog -> InstrumentInfoStorage
list -> InstrumentCatalog -> InstrumentInfoStorage

## Impl

```rust
pub struct Instrument;

pub struct InstrumentProvider {
    provider: DataProvider,
}

impl Instrument {
    pub fn find(code: &str) -> Result<InstrumentInfo, AvinError> {
        let provider = default_data_provider()?;
        InstrumentCatalog::find(provider, code)
    }

    pub fn find_figi(figi: &str) -> Result<InstrumentInfo, AvinError> {
        let provider = default_data_provider()?;
        InstrumentCatalog::find_figi(provider, figi)
    }

    pub fn list(category: Category) -> Result<InstrumentList, AvinError> {
        let provider = default_data_provider()?;
        InstrumentCatalog::list(provider, category)
    }

    pub fn provider(provider: DataProvider) -> InstrumentProvider {
        InstrumentProvider { provider }
    }
}

impl InstrumentProvider {
    pub fn find(&self, code: &str) -> Result<InstrumentInfo, AvinError> {
        InstrumentCatalog::find(self.provider, code)
    }

    pub fn find_figi(
        &self,
        figi: &str,
    ) -> Result<InstrumentInfo, AvinError> {
        InstrumentCatalog::find_figi(self.provider, figi)
    }

    pub fn list(
        &self,
        category: Category,
    ) -> Result<InstrumentList, AvinError> {
        InstrumentCatalog::list(self.provider, category)
    }
}
```

# Data

## Api

```rust
Data::load(
    code: &str,
    md: MarketData,
    range: TimeRange
) -> Result<DataFrame, AvinError>

// и то же самое со scoped provider
```

## Call

load -> InstrumentCatalog + DataManager -> MarketDataStorage

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

# Asset

## Api

```rust
Asset::new(code: &str) -> Result<Asset, AvinError>
Share::new(code: &str) -> Result<Share, AvinError>
Future::new(code: &str) -> Result<Future, AvinError>

asset.load_chart(tf: TimeFrame) -> Result<&Chart, AvinError>
asset.load_ticks() -> Result<&[Tick], AvinError>
asset.load_footprint(tf: TimeFrame) -> Result<&Footprint, AvinError>

// то же самое для конкретных типов
share.load_chart(...)
share.load_ticks(...)
share.load_footprint(...)
future.load_chart(...)
future.load_ticks(...)
future.load_footprint(...)

// и то же самое со scoped provider
```

## Call

продумать позже когда дойдем до сервисов загрузки Chart Tick Footprint

## Impl

```rust
// служебный трейт доступа к приватным полям в avin_domain
#[doc(hidden)]
pub mod internal {
    pub trait AssetDataMut {
        fn set_chart(
            &mut self,
            chart: Chart,
        ) -> &Chart;
    }
}

// extension trait в сервисах
pub trait AssetLoad {
    fn load_chart(&mut self, tf: TimeFrame) -> Result<&Chart, AvinError>;
    // ...

    fn provider(
        &mut self,
        provider: DataProvider,
    ) -> AssetProvider<'_>;
}

// scoped
pub struct AssetProvider<'a> {
    asset: &'a mut Asset,
    provider: DataProvider,
}
```

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
