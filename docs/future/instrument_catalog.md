# InstrumentCatalog — отложенный дизайн

`InstrumentCatalog` может понадобиться для работы сразу с несколькими provider-specific `InstrumentList`:

```text
TBank   InstrumentList
Finam   InstrumentList
MOEX    InstrumentList
Binance InstrumentList
Bybit   InstrumentList
```

У разных providers могут быть разные `InstrumentInfo` для одного `InstrumentId`, поэтому объединить их в один `InstrumentList` нельзя.

Концептуально:

```rust
pub struct InstrumentCatalog {
    providers: HashMap<Provider, InstrumentList>,
}
```

`InstrumentCatalog` может отвечать за:

- multi-provider search;
- фильтрацию по provider;
- provenance результатов;
- общий fuzzy/search index;
- GUI/CLI поиск сразу по нескольким источникам.

Например:

```text
avin instrument --find SBER

TBank      MOEX.SHARE.SBER
Finam      MOEX.SHARE.SBER
MOEX Algo  MOEX.SHARE.SBER
```

При этом отдельный `InstrumentCatalog` не обязателен: пока multi-provider orchestration остается простой, service может работать напрямую с несколькими `InstrumentList`.

Вводить `InstrumentCatalog` стоит только когда эта логика станет самостоятельной и начнет разрастаться.

```text
InstrumentList
    один InstrumentId -> максимум один InstrumentInfo

InstrumentCatalog
    несколько provider-specific InstrumentList
    допускает несколько InstrumentInfo для одного InstrumentId
```
