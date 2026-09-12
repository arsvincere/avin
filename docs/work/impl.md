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

```rust
impl TBankClient {
    pub fn exchanges() -> &'static [Exchange];
    pub fn categories() -> &'static [Category];
    pub fn instruments() -> Vec<InstrumentPack>;
}

InstrumentPack {
    provider: Provider,
    exchange: Exchange,
    category: Category,
    instruments: InstrumentList,
}
```
