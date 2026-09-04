# AVIN Architecture

```text
avin    - public package interface / main cli entry point
tools   - trader toolkit: tester, pattern search, simulator, analyse etc
gui     - lib, generic widgets for AVIN tools

service - internal operations and orchestration

connect - broker and crypto exchange connectors
data    - historical market data providers and normalization
storage - persistence backends, storage layout and data access

system  - workspace, configuration and process environment
domain  - higher-level market and trading models
core    - low-level foundational types
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
connect / data / storage
    ↓
system
    ↓
domain
    ↓
core
```

Допустимые направления зависимостей:

```text
avin    -> re-exports
tools   -> core, domain, service
gui     -> core, domain, service

service -> core, domain, system, storage, data, connect

connect -> core, domain, system
data    -> core, domain, system
storage -> core, domain, system

system  -> core, domain
domain  -> core
core    ->
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

### api

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
- `data`;

## data

Получает provider-specific historical data и приводит их к единому формату AVIN.

- использует provider SDK и HTTP requests;
- управляет provider-specific authentication;
- знает provider schemas, formats и identifiers;
- получает instrument reference data;
- преобразует provider data в canonical AVIN types.

Provider-specific код остается изолированным. Универсальный provider framework не создается до появления второго реального provider-а.

## storage

Локальное хранение данных.

## system

Рабочее окружение, конфигурация и process-level state.

Основные components:

- `Workspace` - рабочее пространство, root directory с `AVIN.toml`.
- `WORKSPACE` - доступ к текущему `Workspace` процесса.
- `Config` - конфигурация пользователя.
- `DataManifest` - desired state of market data.
- `Secret` - token, login/password etc.
- сконфигурированный логгер приложения.

## domain

Основные трейдерские абстракции и их собственное состояние и поведение:
`Bar`, `Chart`, `TimeFrame`, `Tick`, `Footprint`, `Share`, `Future`...

Не занимается I/O, orchestration и взаимодействием с внешними системами.

## core

Общие примитивы для domain объектов.

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

# Flows

## Historical data

Принципиальная схема:

```text
external historical market data, provider specific format
    ↓
data provider -> normalization to AVIN domain objects
    ↓
service -> codec -> .parquet raw data
    ↓
storage
    ↓
service -> codec -> AVIN domain objects
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
```

