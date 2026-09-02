# AVIN Architecture

```text
avin    - public package interface / main cli entry point
tools   - trader toolkit: tester, pattern search, simulator, analyse etc
gui     - lib, generic widgets for avin tools

service - internal operations and orchestration

connect - broker and crypto exchange connectors
data    - historical market data providers and normalization
storage - local persistence, storage layout and codecs
system  - workspace, configuration and process environment

domain  - trading abstractions
core    - low-level type defenitions

utils   - generic helpers
```

# Dependency rules

Упрощенная иерархия зависимостей:

```text
avin
    ↓
tools / gui
    ↓
service
    ↓
connect / data / storage / system
    ↓
domain
    ↓
core
    ↓
utils
```

Допустимые направления зависимостей:

```text
avin    -> re-exports public api

service -> utils core domain system storage data connect

connect ->
data    ->
storage -> utils core domain system
system  -> utils core domain

domain  -> utils core
core    -> utils

utils   ->
```

# Modules

## avin

Public package interface, only re-exports.

Должен быть самодостаточным для повседневной работы трейдера. Формируется по мере развития public API.

### cli

Основная точка входа к функциям AVIN через командную строку. Примеры использования:

```bash
avin instrument cache
avin data sync
avin gui
```

## api

Высокоуровневые интерфейсы пользователя.

- интуитивно понятны трейдеру;
- скрывают internal orchestration;
- предоставляют удобные defaults;
- маленькие;

Интерфейсы `api` публикуются через package interface `avin`.

Сделаны:
- `Data` - операции с биржевыми данными;
- `Asset` - фабрика типизированных инструментов (`Share`, ...);
- `Loader` - загрузка runtime data в инструменты.

## service

Внутренние операции и orchestration над domain-объектами и данными.

Операции:

- loading;
- building;
- ensuring;
- syncing;
- создание и подготовка domain-объектов;
- преобразование raw data в готовое runtime state;

Координация между:

- `domain`;
- `system`;
- `storage`;
- `source`;

## source

Получает provider-specific historical data и приводит их к единому формату AVIN DataFrame.

- использует provider SDK и HTTP requests;
- управляет provider-specific authentication;
- знает provider schemas, formats и identifiers;
- получает instrument reference data;
- преобразует provider data в canonical AVIN DataFrames.

Provider-specific код остается изолированным. Универсальный provider framework не создается до появления второго реального provider-а.

## storage

Локальное хранение данных, структура хранилища, преобразование DataFrame <-> domain object.

Пример пути к файлу внутри `workspace`:

`data/MOEX/SHARE/GAZP/TINKOFF/BAR_1M/2026/2026-01-01.parquet`

- Данные хранятся в `.parquet`.
- При сохранении выполняет дополнительные проверки DataFrame: schema, sorting...

Содержит `StorageCodec` для преобразования market data:

```text
DataFrame <-> list[Bar]
DataFrame <-> list[Tick]
```

## system

Рабочее окружение, конфигурация и process-level state.

Основные components:

- `Workspace` - рабочее пространство, root directory с `AVIN.toml`.
- `Configuration` - конфигурация пользователя.
- `DataManifest` - desired state of market data.
- `ws()` - доступ к текущему `Workspace` процесса.
- `log` - сконфигурированный логгер приложения.

## domain

Основные трейдерские абстракции и их собственное состояние и поведение:
`Iid`, `Tick`, `Bar`, `Chart`, `TimeFrame`, `Footprint`, `BaseAsset`, `Share`, ...

Не занимается I/O, orchestration и взаимодействием с внешними системами.

## errors

Project-specific exceptions.

Generic built-in exceptions используются там, где отдельная project exception
не добавляет смысла.

## utils

Вспомогательные инструменты.

Могут быть поняты отдельно от AVIN business model.

`utils` не должен становиться складом кода с неопределенной responsibility.

## analyse

Исследование рынка, производные данные и статистический анализ.

Модуль запланирован, реализация не начата.

## gui

Визуализация данных и исследований.

Реализация начата, но пока это только эксперименты с Elm-подобной архитектурой на PyQt6.

Что и как реально будет работать - предмет будущих обсуждений.

Текущий экспериментальный GUI flow:

```text
Widget
    -> Event
    -> AppController
    -> service
    -> AppState / domain state
    -> state_changed
    -> Widgets
```

AvinApp создаёт:

```
AppState
AppController
MainWindow
```

AppController владеет AppState.

MainWindow и widgets получают controller и передают ему user events.

## connect

Коннекторы к брокерам и криптовалютным биржам.

Модуль запланирован, реализация не начата.

# Flows

## Historical data

Принципиальная схема:

```text
external historical market data, provider specific format
    ↓
data provider -> normalization
    ↓
storage -> .parquet raw data
    ↓
domain objects / derived objects
    ↓
public user API / GUI / CLI
```

## Asset

Принципиальная схема:

```text
InstrumentId                - identity
    ↓
InstrumentInfo              - reference data
    ↓
Share / Future / Bond / ... - concrete types, runtime container for loaded data
    ↓
Asset                       - wrapper for all tradable instrument kind
    ↓
Whatchlist                  - ordered user instruments collection
```

# File formats

## .parquet data

Основной формат хранения рыночных данных, локального справочника инструментов, производных данных (Footprint, large timeframes) и результатов анализа пользователя.

Почему `.parquet`:

* проще в работе и быстрее чем базы данных;
* простой доступ из python/rust благодаря polars;
* возможность быстрого просмотра содержимого сторонними утилитами (tabiew и др);

Это почти так же просто как работать с `.csv` файлами, но занимают меньше места на диске и загружаются гораздо быстрее.

## .toml configs

Основной формат для кофигурации.

Почему `.toml`:

* синтаксис чище чем json, yaml, ron;
* родной и для rust, и для python разработчиков (Cargo.toml / pyproject.toml)
* простой парсинг из python/rust;
* все его знают;
* интуитивно понятен даже когда первый раз видишь этот формат;

# Workspace

Один процесс должен одновременно работать с одним workspace.

AVIN workspace это директория содержащая файл `AVIN.toml` (или `.AVIN.toml`).

В нем задаются все остальные рабочие директории системы: где лежит конфигурация, дата манифест, логины пароли токены брокеров, GUI настройки.

avin_system::WORKSPACE - singlton

Решить - WORKSPACE.init() явный vs lazylock???
