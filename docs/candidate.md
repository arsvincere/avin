Записи здесь:

- не считаются отсутствующей функциональностью;
- не реализуются и не удаляются без отдельного решения;

# avin data sync
Если понадобится CLI вызов в духе:
```bash
avin data sync MOEX_SHARE_SBER BAR_1M --source TINKOFF
```

Data.sync() сохраняет цельный manifest-driven contract и синхронизирует весь
desired persistent data state Workspace.

Не следует расширять Data.sync() набором optional filters для code, source,
market-data type и range.

Вариант решения - сделать отдельный метод:

```python
Data.sync_instrument(...)
```

Предпочтительный internal execution boundary:

```python
DataSyncer.sync_task(...)
```

# Storage: raw / derived layout

Сейчас market data хранятся без физического разделения `raw / derived`:

```text
data/MOEX/SHARE/GAZP/TINKOFF/BAR_1M/...
data/MOEX/SHARE/GAZP/TINKOFF/BAR_5M/...
data/MOEX/SHARE/GAZP/TINKOFF/BAR_10M/...
```

Для TINKOFF этого достаточно: `BAR_1M` — raw data, остальные timeframe bars — derived data AVIN.

Когда появится `MOEX_ALGO`, пересмотреть layout. MOEX может предоставлять несколько timeframe напрямую: 1M 10M 1H D W M, поэтому `BAR_1H` может быть как raw, так и derived. Предварительный вариант:

```text
data/MOEX/SHARE/GAZP/MOEX_ALGO/RAW/BAR_1H/...
data/MOEX/SHARE/GAZP/MOEX_ALGO/DERIVED/BAR_1H/...
```

Переходить на такой layout только если различие raw / derived реально понадобится.

# InstrumentCatalog

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
