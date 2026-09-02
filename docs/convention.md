# Instrument ID

`InstrumentId` — canonical instrument identifier used by AVIN.

Содержит биржу, категорию инструмента, тикер. Имеет человеко-читаему форму записи, например: `MOEX.SHARE.SBER`. Общее правило: `EXCHANGE.CATEGORY.TICKER`. Пользователь в основном создает конкретные активы через текстовое представление идентификатора.

Wrapped Asset требует полной формы записи:

`Asset::new("MOEX.SHARE.LKOH")`.

Concrete type constructor принимаю так же сокращенную форму:

`Share::new("MOEX.SBER") == Share::new("MOEX.SHARE.SBER")`
`Future::new("MOEX.GLDRUBF") == Future::new("MOEX.FUTURE.GLDRUBF")`

Case insensitive:
`Asset::new("MOEX.SHARE.LKOH") == Asset::new("Moex.ShaRE.lkoh")`

# InstrumentInfo

`InstrumentInfo` — справочная информация по инструменту: биржа, категория, тикер, размер лота, минимальный шаг цены, FIGI и другие metadata.

В domain InstrumentInfo хранит данные как: `HashMap<String, String>` и предоставляет typed getters.

```rust
InstrumentInfo::iid() -> InstrumentId
InstrumentInfo::exchange() -> Exchange
InstrumentInfo::lot() -> u32
```

В storage данные сохраняются по единой schema: `avin_storage::Schema::InstrumentInfo`. Все значения хранятся как `pl.String` и при загрузке преобразуются в `HashMap<String, String>`.

Это намеренный contract: data providers передают instrument metadata с разными именами полей / типами значений, поэтому перед сохранением данные нормализуются до avin_storage::Schema::InstrumentInfo.

# Time

Используются сокращения в именах переменных:

* `ts` — timestamp, всегда nanoseconds; основная форма времени в AVIN.
* `dt` — datetime, всегда UTC; используется в public API для удобства.

Naive local datetime не используется как внутреннее представление времени, но может показываться пользователю в GUI / CLI.

Для работы со временем определены утилиты и константы, смотреть:

- `avin_utils::const`
- `avin_utils::dt`
- `avin_utils::week_days`

Использовать их, а не плодить реализацию в каждом helper-е.

Встречающиеся в Python коде:

- `Date`
- `DateTime`
- `Time`
- `TimeDelta`
- `TimeZone`
- `UTC`

Являются простыми алиасами над стандартными python datetime классами. Определены в `avin.utils.alias`.

```python
from avin.utils.alias import Date, DateTime, Time, TimeDelta, TimeZone, UTC
```

# Ranges

Использовать установленную семантику пар имен, не переопределяя ее локально.

```text
[begin, end)    - полуоткрытый диапазон
[from, till]    - закрытый диапазон
[start, finish] - закрытый диапазон
[low, high]     - закрытый диапазон
```

# Python - fields, properties and methods

## Non-private field

Только для простых immutable value objects. Не должен обходить domain invariants.

## Property

`@property` используется для дешевого read-only access без parameters и side effects.

```text
asset.exchange
asset.category
asset.ticker
```

## Method

Используется, если нужны parameters, validation, domain logic, exception или optional result.

Для доступа к data используются следующие contracts:

```text
strict accessor
    -> object или exception
    -> no side effects

find_*
    -> object | None
    -> no side effects

has_*
    -> bool
    -> no side effects
```

Strict accessor используется, когда data должны существовать:

```text
asset.chart(tf)
asset.ticks()
asset.time_footprint(tf)
```

Accessors не загружают, не строят и не изменяют state.
