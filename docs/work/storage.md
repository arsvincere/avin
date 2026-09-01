# CLI commands

```bash
avin data instruments cache
avin data instruments cache --provider tbank

avin data instruments clear
avin data instruments clear --provider tbank

avin data sync --resume
avin data sync --abort
avin data sync --status

avin data sync
avin data sync --force
avin data sync --force --provider tbank
avin data sync --force --provider tbank --instrument moex.share.sber
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m --year 2025

avin data delete
avin data delete --provider tbank
avin data delete --provider tbank --instrument moex.share.sber
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m --year 2025

avin data prune

avin data compact
```

## clap

Предпочтительный стиль — typed named options:

```rust
#[derive(Args)]
struct SyncArgs {
    #[arg(long)]
    provider: Option<DataProvider>,

    #[arg(long)]
    instrument: Option<String>,

    #[arg(long)]
    data: Option<MarketData>,

    #[arg(long)]
    year: Option<Year>,

    #[arg(long)]
    force: bool,

    #[arg(long)]
    resume: bool,

    #[arg(long)]
    abort: bool,

    #[arg(long)]
    status: bool,
}
```

Named options не позиционные, поэтому их порядок в команде не должен иметь значения.

Зависимости аргументов желательно описывать декларативно через возможности `clap`.

Например:

```text
--instrument requires --provider
--data       requires --instrument
--year       requires --data
```

Для `sync` scoped options:

```text
--provider
--instrument
--data
--year
```

предполагаются прежде всего для режима `--force`.

Recovery options:

```text
--resume
--abort
--status
```

должны быть взаимоисключающими.

# Architecture

canonical historical data storage = Parquet
research data representation = Polars DataFrame
call: service -> storage -> domain

# Happy path

Главный happy path:

```text
avin data sync
```

`sync` работает с декларативным `data.toml`:

1. читает `data.toml`;
2. определяет используемые providers;
3. для каждого provider проверяет InstrumentInfo cache;
4. при отсутствии cache загружает полный provider catalog;
5. разрешает instrument codes из manifest;
6. синхронизирует заявленные market data.

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

# Api

## InstrumentInfoStorage

```rust
InstrumentInfoStorage::exists(
    provider: DataProvider,
    category: Category,
) -> Result<bool, AvinError>

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

MarketDataStorage::exists(
    key: StorageKey,
) -> Result<bool, AvinError>

MarketDataStorage::delete(
    key: StorageKey,
) -> Result<(), AvinError>

MarketDataStorage::write(
    key: StorageKey,
) -> Result<WriteOperation, AvinError>

MarketDataStorage::compact() -> Result<(), AvinError>
MarketDataStorage::status() -> Result<StorageStatus, AvinError>
MarketDataStorage::inventory() -> Result<Vec<StorageKey>, AvinError>
```

### Impl

```rust
enum StorageStatus {
    Clean,
    Dirty(WriteOperation),
}

struct WriteOperation {
    provider: DataProvider,
    iid: InstrumentId,
    md: MarketData,
    year: Year,
}

impl WriteOperation {
    pub fn add(&self, chunk: DataChunk) -> Result<(), AvinError>
    pub fn finalize(self) -> Result<(), AvinError>
    pub fn abort(self) -> Result<(), AvinError>
    pub fn next_time(&self) -> Result<Option<Time>, AvinError>
}

struct DataChunk {
    coverage_range: TimeRange,
    df: DataFrame,
}

// лучший вариант пока такой!
struct StorageKey {
    provider: DataProvider,
    iid: Option<InstrumentId>,
    md: Option<MarketData>,
    year: Option<Year>,
}


// how to use
let operation = MarketDataStorage::write(
    provider,
    iid,
    md,
    year,
)?;
for chunk in provider {
    operation.add(chunk)?;
}
operation.finalize()?;
```

# Пока не решено

## хранить ли незавершенные бары
основной трабл - 1М есть до середины месяца, скачали еще пару дней, упали...
Хранилище рассинхронизировано...
хотя если только завершенные таже проблема скачали вторую половину 1М и еще пару
дней и потом упали - хранилище рассинхронизировано...
Но инвариант - только исторический завершенный бар - проще поддерживать.
Незавершенный в реал тайме - все равно качается с брокера.
В тестере - он просто не нужен.

Склоняюсь к "не хранить незавершенный бар".
Не сохранять сконвертированный незавершенный бар.

## Как хранить InstrumentInfo, как устроен InstrumentInfo внутри
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

## Разное
* точная семантика `--force` при наличии pending operation;
* точное взаимодействие `--force`, `--resume`, `--abort`, `--status`;
* confirmation policy для destructive/recovery операций;
* нужен ли дополнительный scope для `prune`;
* short options — отдельно после окончательного утверждения long-form CLI.

## Imperative mode - будущая feature

На будущее можно добавить императивный режим.

Что качать задается непосредственно в cli command args, а не в data.toml.

А `update` просматривает текущий storage и запрашивает append только для существующих данных.

```
avin data download --provider tbank --instrument moex.share.sber
avin data download --provider tbank --instrument moex.share.sber --data bar_1m
avin data download --provider tbank --instrument moex.share.sber --data bar_1m --year 2025

avin data update
```

# release build
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
