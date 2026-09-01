# cli

avin instruments cache
avin instruments cache --provider tbank
avin instruments clear ?
avin instruments delete ?

avin data sync
avin data sync --force
avin data sync --continue
avin data sync --abort
avin data sync --provider tbank

avin data prune

avin data delete --provider tbank --iid "moex.share.sber" --type bar_1m
avin data delete moex.share.sber bar_1m --provider tbank

avin data download
avin data update


# Architecture

canonical historical data storage = Parquet
research data representation = Polars DataFrame
call: service -> storage -> domain

## InstrumentInfoStorage

```rust
InstrumentInfoStorage::save(
    provider: DataProvider,
    category: Category,
    df: DataFrame,
) -> Result<(), AvinError>

InstrumentInfoStorage::load(
    provider: DataProvider,
    category: Category,
) -> Result<DataFrame, AvinError>

InstrumentInfoStorage::delete(
    provider: DataProvider,
    category: Category,
) -> Result<(), AvinError>
```

## MarketDataStorage

```rust
// этот метод уже не нужем см api v2 там вырисовывается новая картинка
MarketDataStorage::save(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    df: DataFrame,
) -> Result<(), AvinError>

MarketDataStorage::load_range(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    range: TimeRange,
) -> Result<DataFrame, AvinError>

MarketDataStorage::load_latest(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
    quantity: Quantity,
) -> Result<DataFrame, AvinError>

MarketDataStorage::delete(
    provider: DataProvider,
    iid: &InstrumentId,
    md: MarketData,
) -> Result<(), AvinError>
```

# File system

## directory

```text
data/moex/share/SBER/tbank/bar_1m
data/moex/share/SBER/tbank/bar_5m
data/moex/share/SBER/tbank/tick
data/moex/share/SBER/tbank/...
```

## files
```text
# v6 — base + tail + stage/
tbank/
├── bar_1m/
│   ├── 2024.parquet
│   ├── 2025.parquet
│   ├── 2026/
│   │   ├── base.parquet
│   │   └── tail/
│   │       ├── 2026-09-29.parquet
│   │       └── 2026-09-30.parquet
│   ├── stage/
│   │   ├── 2022
│   │   │   ├── 2022-01-01.parquet
│   │   │   ├── 2022-01-02.parquet
│   │   │   └── ...
│   │   ├── 2025
│   │   │   ├── 2025-01-01.parquet
│   │   │   ├── 2025-01-02.parquet
│   │   │   └── ...
│   └── metadata.toml
│
└── tick/
    ├── 2024.parquet
    ├── 2025.parquet
    ├── 2026/
    │   ├── base.parquet
    │   └── tail/
    │       ├── 2026-09-29.parquet
    │       └── 2026-09-30.parquet
    ├── stage/
    │   ├── 2022
    │   │   ├── 2022-01-01.parquet
    │   │   ├── 2022-01-02.parquet
    │   │   └── ...
    │   ├── 2025
    │   │   ├── 2025-01-01.parquet
    │   │   ├── 2025-01-02.parquet
    │   │   └── ...
    └── metadata.toml
```

## metadata

тут пока не понятно что и как хранить, из разумных идей - не дублировать то что и так понятно из файловой системы. То есть вот это в текущей раскладке файлов не нужно:

```toml
[years.2025]
complete = true
```

## плохие схемы (архив)
```text
# v4 — год как отдельный partition
отдельная папка на год - сначала выглядит хорошо... но тогда чтобы увидеть
есть ли незавершенный оверлай нужно просмотреть все дерево. А глобальный стейдж
сразу показывает - в этой папке не порядок. Поэтому схема вот эта с отдельными
папками по годам - нафиг не нужна.
tbank/
├── bar_1m/
│   ├── base.parquet
│   ├── stage/
│   │   ├── 2024-01-01.parquet
│   │   ├── 2024-01-02.parquet
│   │   ├── ...
│   │   ├── 2025-01-01.parquet
│   │   ├── 2025-01-02.parquet
│   │   └── ...
│   ├── tail/
│   │   ├── 2026-09-29.parquet
│   │   └── 2026-09-30.parquet
│   └── metadata.toml
│
└── tick/
    ├── 2024/
    │   ├── data.parquet
    │   └── stage/
    │       ├── 2024-01-01.parquet
    │       └── 2024-01-02.parquet
    ├── 2025/
    │   ├── data.parquet
    │   └── stage/
    │       ├── 2025-01-01.parquet
    │       └── 2025-01-02.parquet
    ├── 2026/
    │   ├── base.parquet
    │   └── tail/
    │       ├── 2026-09-29.parquet
    │       └── 2026-09-30.parquet
    └── metadata.toml

# v2 — compact/ + tail/
лишние сложные пути... мне не навится... как строить список общих файлов для
поиска? не очевидно. Возьми все компакт, возьми тейл...
tbank/
├── bar_1m/
│   ├── compact/
│   │   └── data.parquet
│   ├── tail/
│   │   ├── 2026-09-29.parquet
│   │   └── 2026-09-30.parquet
│   └── metadata.toml
│
└── tick/
    ├── compact/
    │   ├── 2024.parquet
    │   ├── 2025.parquet
    │   └── 2026.parquet
    ├── tail/
    │   ├── 2026-09-29.parquet
    │   └── 2026-09-30.parquet
    └── metadata.toml


# v3 — плоско, роль в имени файла
Мешанина из файлов, глазами не разберешься.
tbank/
├── bar_1m/
│   ├── base.parquet
│   ├── tail.2026-09-29.parquet
│   ├── tail.2026-09-30.parquet
│   └── metadata.toml
│
└── tick/
    ├── base.2024.parquet
    ├── base.2025.parquet
    ├── base.2026.parquet
    ├── tail.2026-09-29.parquet
    ├── tail.2026-09-30.parquet
    └── metadata.toml

```

# Operations
Storage:
    CREATE
    REPLACE
    APPEND
    DELETE
    COMPACT
        current year:
        base + tail → base
    FINALIZE
        stage year → YYYY.parquet
        или current open year → YYYY.parquet

Service:
    first sync  -> CREATE years
    backfill    -> CREATE year
    force       -> REPLACE year
    sync        -> APPEND days
    delete      -> DELETE

# api v2
service-facing:
    add
    finalize
    load_range
    load_latest
    delete

maintenance:
    compact

storage-internal:
    stage
    append
    create
    replace
    compaction policy

```rust
MarketDataStorage::start(provider, iid, md, year)
MarketDataStorage::add(provider, iid, md, chunk)
MarketDataStorage::finalize(provider, iid, md, year)
MarketDataStorage::abort()

MarketDataStorage::status() -> Result<StorageStatus, AvinError>

enum StorageStatus {
    Clean,
    Dirty(PendingOperation),
}

struct PendingOperation {
    provider: DataProvider,
    iid: InstrumentId,
    md: MarketData,
    year: Year,
}

// альтернатива:
let operation = MarketDataStorage::start(
    provider,
    iid,
    md,
    year,
)?;
for chunk in provider {
    operation.add(chunk)?;
}
operation.finalize()?;

// оператион тогда еще должна уметь возвращать last date
operation.latest_date() -> Option<Date>
// или изменить имя... чтобы избавиться от оптион, возвращать не latest date а
// день который нужен следующий... а ну один хер... тогда не сходится если
// 365/365 уже скачано...
```

# vocabularity

normal workflow:
    start
    add
    finalize

recovery:
    status
    abort

read:
    load_range
    load_latest

mutation:
    delete

maintenance:
    compact

internal:
    stage
    append
    create
    replace
    compaction policy

# хранить ли незавершенные бары
основной трабл - 1М есть до середины месяца, скачали еще пару дней, упали...
Хранилище рассинхронизировано...
хотя если только завершенные таже проблема скачали вторую половину 1М и еще пару
дней и потом упали - хранилище рассинхронизировано...
Но инвариант - только исторический завершенный бар - проще поддерживать.
Незавершенный в реал тайме - все равно качается с брокера.
В тестере - он просто не нужен.

Склоняюсь к "не хранить незавершенный бар".
Не сохранять сконвертированный незавершенный бар.

# Как хранить InstrumentInfo, как устроен InstrumentInfo внутри
Провайдер отдает
ti.Instrument
там куча полей
дальше

    info = {
        "exchange": exchange_to_avin_exchange(i.exchange),
        "exchange_specific": i.exchange,  # original exchange name
        "category": "",  # seting below
        "ticker": i.ticker,
        "figi": i.figi,
        "country": i.country_of_risk,
        "currency": i.currency,
        "sector": "",  # seting below
        "class_code": i.class_code,
        "isin": "",  # seting below
        "uid": i.uid,
        "name": i.name,
        "lot": str(i.lot),
        "step": str(float(dec(i.min_price_increment))),
        "long": str(float(dec(i.dlong))),
        "short": str(float(dec(i.dshort))),
        "long_qual": str(float(dec(i.dlong_min))),
        "short_qual": str(float(dec(i.dshort_min))),
        "first_1m": str(dt_to_ts(i.first_1min_candle_date)),
        "first_d": str(dt_to_ts(i.first_1day_candle_date)),
    }

потом я это превращал в словарь, потом в дата фрейм и дата фрейм хранил

--

Сейчас в раст InstrumentInfo содержит HashMap, и он не может содержать
произвольные типы в отличии от питона... поэтому и пришел к тому что
все - String.

Но вообще то можно и усложнить... себе жизнь... и начать типизировать...
и сделать ебейший конструктор на 20 полей...

Или сразу собирать дата фрейм...

Или успокоиться - и так работает. И так достаточно гибко.
Да валидация не такая строгая...
Но кстати ее можно внутри InstrumentInfo сделать более строгой и разбор на
типизированные поля сделать внутри.

Вопрос по большому счету такой - InstrumentInfo создается из HashMap или из DataFrame.
- HashMap - проще
- DataFrame - тянет зависимость в domain

Ответ очевиден - нехуй изобретать колесо... Строки нормальное промежуточное представление.

Проблема тут в другом. Провайдер создает HashMap -> DataFrame -> storage

А должно быть HashMap -> InstrumentInfo -> InstrumentList -> storage

# release
То есть наличие кода внутри зависимого crate не означает, что весь машинный код этого crate окажется в executable.

Особенно хорошо это работает в:

```toml
[profile.release]
lto = true
```

с LTO linker/compiler ещё лучше видит границы между crates и может выкидывать/инлайнить код через них.


```toml
[features]
default = []
data = ["dep:avin_data_app"]
tester = ["dep:avin_tester_app"]
gui = ["dep:avin_gui_app"]
full = ["data", "tester", "gui"]

[[bin]]
name = "avin"
path = "src/main.rs"

[[bin]]
name = "avin-data"
path = "src/bin/avin-data.rs"
required-features = ["data"]

[[bin]]
name = "avin-tester"
path = "src/bin/avin-tester.rs"
required-features = ["tester"]

[[bin]]
name = "avin-gui"
path = "src/bin/avin-gui.rs"
required-features = ["gui"]
```
