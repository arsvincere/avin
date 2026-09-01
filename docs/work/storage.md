# cli

## Commands overview

```bash
avin data instruments cache
avin data instruments cache --provider tbank

avin data instruments clear
avin data instruments clear --provider tbank

avin data sync
avin data sync --dry-run
avin data sync --resume
avin data sync --abort
avin data sync --status

avin data sync --force
avin data sync --force --provider tbank
avin data sync --force --provider tbank --instrument moex.share.sber
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m --year 2025

avin data delete --provider tbank
avin data delete --provider tbank --instrument moex.share.sber
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m --year 2025

avin data prune

avin data compact
```

## Principle

CLI для работы с локальными market data и instrument reference data.

Основной namespace:

```text
avin data ...
```

Первое слово после `avin` выбирает tool/module. В будущем рядом будут другие команды:

```text
avin data ...
avin tester ...
avin search ...
avin analyse ...
...
```

Поэтому `data` не поднимается в root-команды вроде `avin sync`.

## Общие решения

* CLI должен оставаться тонкой внешней boundary.
* `avin_service` не принимает пользовательский `code: &str`; внутренние сервисы работают с валидным `InstrumentId`.
* В CLI допустим локальный helper для преобразования:

```text
code
→ InstrumentCatalog
→ InstrumentInfo
→ InstrumentId
```

Например:

```text
avin/src/cli/
├── mod.rs
└── helpers.rs
```

Если `helpers.rs` начнёт заметно разрастаться, это будет сигналом пересмотреть boundary/layout.

* Не требуется полная идентичность CLI и public Rust/Python API.
* CLI может обращаться к service API напрямую, если это не приводит к размазыванию business logic по CLI.
* Редкие операции могут быть длиннее, если синтаксис от этого становится очевиднее.
* Для scope используются named options, а не positional arguments.

Scope имеет единый порядок:

```text
provider
→ instrument
→ data
→ year
```

## Instruments

InstrumentInfo cache относится к `data` subsystem.

```text
avin data instruments cache
avin data instruments cache --provider tbank

avin data instruments clear
avin data instruments clear --provider tbank
```

`cache` сохраняет полный справочник выбранного provider, а не только инструменты из `data.toml`.

Обычному пользователю вручную вызывать `cache` обычно не требуется: `sync` автоматически обеспечивает наличие нужных provider catalogs.

## Sync

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

### Syntax

```text
avin data sync
avin data sync --dry-run
avin data sync --resume
avin data sync --abort
avin data sync --status

avin data sync --force
avin data sync --force --provider tbank
avin data sync --force --provider tbank --instrument moex.share.sber
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m
avin data sync --force --provider tbank --instrument moex.share.sber --data bar_1m --year 2025
```

Обычный `sync` всегда синхронизирует manifest целиком и сам пропускает уже актуальные данные.

Уточнение scope вручную имеет смысл прежде всего вместе с `--force`, когда пользователь хочет пересинхронизировать конкретную часть dataset.

### Recovery

```text
avin data sync --status
avin data sync --resume
avin data sync --abort
```

Точное поведение recovery, конфликтов с `--force` и интерактивных подтверждений пока не фиксируется.

Для неоднозначных destructive/recovery ситуаций допустим CLI prompt:

```text
[y/N]
```

Storage/service при этом должны оставаться детерминированными и не заниматься пользовательским диалогом.

### Dry run

```text
avin data sync --dry-run
```

Должен показывать план синхронизации без изменения storage.

Точный формат плана пока не определён.

## Delete

```text
avin data delete --provider tbank
avin data delete --provider tbank --instrument moex.share.sber
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m
avin data delete --provider tbank --instrument moex.share.sber --data bar_1m --year 2025
```

Scope последовательно сужается:

```text
provider
→ instrument
→ data
→ year
```

`delete` относится к market data.

InstrumentInfo cache очищается отдельно через:

```text
avin data instruments clear
```

Для destructive operations поведение confirmation пока отдельно не проектировалось.

## Prune

```text
avin data prune
```

Семантика:

> удалить из локального market data storage всё, что больше не входит в текущий `data.toml`.

`prune` приводит фактическое содержимое storage к declarative manifest со стороны удаления лишних данных.

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
    dry_run: bool,

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

Точные `requires`, `conflicts_with`, argument groups и правила комбинации с `--dry-run` / `--force` будут определены при реализации.

## Пока не решено

* точная семантика `--force` при наличии pending operation;
* точное взаимодействие `--force`, `--resume`, `--abort`, `--status`;
* формат `--dry-run`;
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

# Architecture

canonical historical data storage = Parquet
research data representation = Polars DataFrame
call: service -> storage -> domain

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

# Operations
Storage:
    CREATE
    REPLACE
    APPEND
    DELETE
    --
    COMPACT
        current year:
        base + tail → base
    FINALIZE
        stage year → YYYY.parquet
        или current open year → YYYY.parquet

Service:
    backfill    -> CREATE year
    force       -> REPLACE year
    sync        -> APPEND days
    delete      -> DELETE

# Vocabularity

normal workflow:
    start
    add
    finalize

recovery:
    status
    abort
    continue

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

# Api
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
