# Primary rule

Используется самое короткое имя, сохраняющее однозначный смысл в текущем контексте.

Не добавлять в имя информацию, уже очевидную из module, type.

Более длинное имя используется только для устранения реальной неоднозначности.

# Уточнение имён по мере углубления

Стараться сохранять простые предметные имена на верхнем уровне API и добавлять техническую конкретику только по мере углубления во внутренние слои системы. Пример:

```rust
// user api
Instrument
Data
Storage

// service
InstrumentCatalog
DataManager

// subsystem
InstrumentInfoStorage
MarketDataStorage
```

Условно:

```text
1 слово -> user-facing API
2 слова -> service layer
3 слова -> subsystem / implementation layer
```

Это не жёсткое правило, а ориентир. Если его удаётся соблюдать, внешний API остаётся простым и говорит на языке пользователя, а технические детали появляются только там, где они действительно нужны.

# Abbreviations

Устоявшиеся в ходе разработки сокращения:

```text
iid - Instrument ID
dt  - datetime
ts  - timestamp
tf  - timeframe
md  - market data
fp  - footprint
ws  - workspace
```

# Python Exceptions

Все custom exception classes AVIN используют suffix `Error`.

# Temporary development names

Временные development/debugging elements маркируются:

```text
DEV_*  | DBG_*    - constants
dev_*  | dbg_*    - functions and variables
# dev  | # dbg    - py code blocks
// dev | // dbg   - rs code blocks
```

Маркировка означает, что element не является продакшен кодом, и если не помнишь
нахер он тут остался - скорее всего просто забыл его удалить после работы.
