# AVIN CLI — модульная архитектура и установка

## Цель

AVIN должен быть модульной системой в духе XFCE:

- пользователь может установить только нужную функциональность;
- все модули используют единый CLI namespace `avin ...`;
- основной бинарник `avin` остаётся маленьким dispatcher'ом;
- функциональные бинарники собираются отдельно;
- весь CLI-код хранится централизованно в `crates/avin/src/cli/`;
- нижние crates (`avin_data`, `avin_storage`, `avin_service`, `avin_tester` и т.д.) ничего не знают про CLI и Clap;
- Rust API остаётся доступным через `use avin`;
- Python API остаётся доступным через `import avin`;
- Cargo и системные package managers могут устанавливать AVIN по-разному — это нормально.

Желаемый пользовательский интерфейс:

```bash
avin new my_workspace
avin init

avin data sync
avin instruments cache
avin tester run ...
```

Если модуль не установлен:

```text
AVIN module `tester` is not installed.

Cargo:
    cargo install avin --features tester

Arch Linux:
    pacman -S avin-tester
```

Главная идея dispatch:

```text
avin <module> <args...>
        ↓
avin-<module> <args...>
```

Например:

```text
avin data sync
    ↓
avin-data sync
```

Физические `avin-data`, `avin-tester` и т.д. являются implementation detail. Пользовательский CLI всегда выглядит как `avin ...`.

---

# 1. Основное архитектурное решение

Весь user-facing CLI-код хранится в одном месте:

```text
crates/avin/src/cli/
```

Пример:

```text
crates/avin/src/cli/
├── mod.rs
├── dispatch.rs
├── new.rs
├── init.rs
│
├── data/
│   ├── mod.rs
│   └── sync.rs
│
├── instruments/
│   ├── mod.rs
│   └── cache.rs
│
└── tester/
    ├── mod.rs
    └── run.rs
```

Это сознательное решение.

`avin` — верхний user-facing facade crate, поэтому CLI является его adapter layer.

Нижние crates предоставляют функциональность:

```text
avin_data
avin_service
avin_storage
avin_domain
avin_tester
...
```

но не знают:

- про Clap;
- про CLI arguments;
- про `avin data`;
- про dispatcher;
- про способ установки;
- про `pacman` или `cargo install`.

CLI только преобразует пользовательские аргументы в вызовы нижних API.

Если `avin/src/cli/` когда-нибудь реально станет слишком большим, его всегда можно разнести позже. Заранее дробить его по другим crates нет необходимости.

---

# 2. Почему CLI не лежит в `avin_data`, `avin_tester` и т.д.

Альтернативный вариант:

```text
avin_data/src/cli/
avin_tester/src/cli/
```

технически тоже возможен, но пока не даёт AVIN полезных преимуществ.

Централизованный CLI проще воспринимать:

```text
avin/
└── cli/
    ├── data/
    ├── instruments/
    ├── tester/
    └── ...
```

При этом модульность установки не теряется.

То, где лежат исходники CLI, не определяет, что попадёт в конкретный binary.

Это определяют:

- Cargo features;
- optional dependencies;
- `required-features`;
- отдельные binary targets;
- dependency graph.

Поэтому можно держать весь CLI рядом и всё равно собирать отдельно только:

```text
avin
avin-data
avin-tester
...
```

---

# 3. Общая структура repo

Предварительно:

```text
avin/
├── Cargo.toml
│
└── crates/
    ├── avin/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── prelude.rs
    │       ├── main.rs
    │       │
    │       ├── cli/
    │       │   ├── mod.rs
    │       │   ├── dispatch.rs
    │       │   ├── new.rs
    │       │   ├── init.rs
    │       │   │
    │       │   ├── data/
    │       │   │   ├── mod.rs
    │       │   │   └── sync.rs
    │       │   │
    │       │   ├── instruments/
    │       │   │   ├── mod.rs
    │       │   │   └── cache.rs
    │       │   │
    │       │   └── tester/
    │       │       ├── mod.rs
    │       │       └── run.rs
    │       │
    │       └── bin/
    │           ├── avin-data.rs
    │           ├── avin-tester.rs
    │           └── ...
    │
    ├── avin_data/
    ├── avin_service/
    ├── avin_storage/
    ├── avin_domain/
    ├── avin_tester/
    └── ...
```

Ключевое разделение:

```text
avin/src/cli/*
    = user-facing CLI adapters

avin_data / avin_service / avin_storage / ...
    = функциональность

avin/src/bin/*
    = thin binary entry points
```

---

# 4. Cargo workspace

Корневой `Cargo.toml` задаёт реальные версии и workspace dependencies.

Пример:

```toml
[workspace]
resolver = "3"
members = [
    "crates/*",
]

[workspace.package]
version = "0.5.0"
edition = "2024"

[workspace.dependencies]
clap = {
    version = "4",
    features = ["derive"],
}

avin_data = {
    version = "0.5.0",
    path = "crates/avin_data",
}

avin_service = {
    version = "0.5.0",
    path = "crates/avin_service",
}

avin_tester = {
    version = "0.5.0",
    path = "crates/avin_tester",
}
```

Важно:

в корневом `[workspace.dependencies]` указываются конкретные версии/path.

`workspace = true` используется уже в дочерних `Cargo.toml`.

---

# 5. `crates/avin/Cargo.toml`

Пример:

```toml
[package]
name = "avin"
version.workspace = true
edition.workspace = true

[features]
default = []

data = [
    "dep:avin_data",
    "dep:avin_service",
]

tester = [
    "dep:avin_tester",
]

full = [
    "data",
    "tester",
]

[dependencies]
clap.workspace = true

avin_data = {
    workspace = true,
    optional = true,
}

avin_service = {
    workspace = true,
    optional = true,
}

avin_tester = {
    workspace = true,
    optional = true,
}

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
```

Идея:

```bash
cargo install avin
```

ставит только минимальный:

```text
avin
```

А:

```bash
cargo install avin --features data
```

ставит:

```text
avin
avin-data
```

А:

```bash
cargo install avin --features full
```

ставит все enabled binary targets.

`full` не должен быть default feature, иначе обычный library user:

```toml
avin = "0.5"
```

начнёт тянуть весь application stack.

---

# 6. Feature-gating CLI-модулей

Так как весь CLI лежит внутри crate `avin`, optional функциональность можно ограничивать через `cfg(feature = "...")`.

Например:

```rust
pub mod dispatch;
pub mod init;
pub mod new;

#[cfg(feature = "data")]
pub mod data;

#[cfg(feature = "data")]
pub mod instruments;

#[cfg(feature = "tester")]
pub mod tester;
```

Минимальная сборка `avin` не компилирует CLI-код data/tester модулей и не тянет их optional dependencies.

---

# 7. Минимальный Hello World

Ниже синтетический пример двух команд:

```bash
avin new workspace_name
avin data sync
```

Первая выполняется внутри главного `avin`.

Вторая dispatch'ится в отдельный binary `avin-data`, но сама реализация команды всё равно лежит в `avin/src/cli/data/`.

---

# 8. `avin new workspace_name`

## `crates/avin/src/main.rs`

```rust
fn main() {
    if let Err(error) = avin::cli::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
```

---

## `crates/avin/src/cli/mod.rs`

```rust
pub mod dispatch;
pub mod new;

#[cfg(feature = "data")]
pub mod data;

use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "avin")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New {
        workspace_name: String,
    },

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::New { workspace_name } => {
            new::run(&workspace_name)?;
        }

        Command::External(args) => {
            dispatch::run(args)?;
        }
    }

    Ok(())
}
```

Ключевой элемент:

```rust
#[command(external_subcommand)]
External(Vec<OsString>)
```

Неизвестная основному `avin` команда не считается сразу ошибкой.

Например:

```bash
avin data sync
```

превращается примерно в:

```rust
External([
    "data",
    "sync",
])
```

---

## `crates/avin/src/cli/new.rs`

```rust
use std::fs;
use std::path::Path;

pub fn run(workspace_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(workspace_name);

    fs::create_dir_all(path)?;

    println!("Created AVIN workspace: {workspace_name}");

    Ok(())
}
```

Вызов:

```bash
avin new research
```

результат:

```text
Created AVIN workspace: research
```

Здесь никакого дочернего процесса нет.

---

# 9. Dispatcher

## `crates/avin/src/cli/dispatch.rs`

Смысл:

```text
avin data sync
     │    │
     │    └── args дочернего CLI
     │
     └─────── module = data
```

Dispatcher превращает это в:

```text
avin-data sync
```

Синтетический вариант:

```rust
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{self, Command};

pub fn run(args: Vec<OsString>) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args.into_iter();

    let module = args
        .next()
        .ok_or("missing AVIN module")?;

    let module = module
        .into_string()
        .map_err(|_| "invalid module name")?;

    let binary_name = format!("avin-{module}");

    let binary = find_binary(&binary_name)?;

    let status = Command::new(binary)
        .args(args)
        .status()?;

    process::exit(status.code().unwrap_or(1));
}

fn find_binary(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;

    let bin_dir = current_exe
        .parent()
        .ok_or("cannot determine AVIN binary directory")?;

    let filename = format!(
        "{name}{}",
        std::env::consts::EXE_SUFFIX
    );

    let candidate = bin_dir.join(filename);

    if candidate.exists() {
        return Ok(candidate);
    }

    Err(format!(
        "AVIN module `{name}` is not installed"
    )
    .into())
}
```

На первом этапе этого достаточно для Cargo installation, где бинарники лежат рядом:

```text
~/.cargo/bin/
├── avin
└── avin-data
```

Позже `find_binary()` можно адаптировать под system packaging.

---

# 10. `avin-data` binary

## `crates/avin/src/bin/avin-data.rs`

Binary target очень маленький:

```rust
fn main() {
    if let Err(error) = avin::cli::data::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
```

Он не содержит бизнес-логику и практически не содержит CLI implementation.

Он только выбирает соответствующее CLI-поддерево.

---

# 11. Реальный data CLI

## `crates/avin/src/cli/data/mod.rs`

```rust
mod sync;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "avin-data",
    bin_name = "avin data"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Sync,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Sync => {
            sync::run()?;
        }
    }

    Ok(())
}
```

Важно:

```rust
bin_name = "avin data"
```

Физический executable называется:

```text
avin-data
```

но публичное CLI-имя:

```text
avin data
```

Поэтому help должен выглядеть примерно так:

```text
Usage: avin data <COMMAND>

Commands:
  sync
```

---

## `crates/avin/src/cli/data/sync.rs`

```rust
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // В реальном AVIN здесь будет вызов service/API слоя.
    println!("Hello from AVIN Data sync!");

    Ok(())
}
```

В реальной реализации этот adapter будет вызывать что-то уровня:

```text
CLI args
    ↓
DataManager
    ↓
InstrumentCatalog / MarketDataStorage / providers
```

Но CLI не должен сам реализовывать storage/data logic.

---

# 12. Полный flow `avin data sync`

Shell запускает:

```text
avin ["data", "sync"]
```

Главный parser получает:

```text
External([
    "data",
    "sync",
])
```

Dispatcher отделяет:

```text
module = data
args   = ["sync"]
```

и строит имя binary:

```text
avin-data
```

Дальше запускается:

```text
avin-data ["sync"]
```

Binary:

```text
crates/avin/src/bin/avin-data.rs
```

вызывает:

```rust
avin::cli::data::run()
```

Data CLI parser получает:

```text
Command::Sync
```

и вызывает:

```text
cli/data/sync.rs
```

Вся CLI implementation остаётся внутри `avin`.

---

# 13. Почему отдельный process всё равно нужен

Даже если весь CLI-код находится в одном crate, отдельный binary нужен ради модульной установки.

Минимальный пользователь может иметь только:

```text
avin
```

и не иметь:

```text
avin-data
avin-tester
```

Поэтому главный `avin` dispatcher физически проверяет наличие нужного module binary.

Это позволяет одному CLI namespace работать поверх независимо установленных компонентов.

---

# 14. Несколько top-level команд у одного модуля

Есть нюанс:

```bash
avin data sync
avin instruments cache
```

Обе команды логически могут относиться к data subsystem.

Generic rule:

```text
data → avin-data
```

работает сразу.

Но:

```text
instruments → ?
```

тоже желательно направить в `avin-data`.

Значит eventually dispatcher может иметь небольшой registry:

```text
data        → avin-data
instruments → avin-data
tester      → avin-tester
```

Это не проблема архитектуры.

Пример:

```rust
fn binary_for(command: &str) -> String {
    match command {
        "data" | "instruments" => "avin-data".into(),
        "tester" => "avin-tester".into(),
        other => format!("avin-{other}"),
    }
}
```

На первом этапе можно реализовать только фактически существующие команды.

Не надо заранее строить сложную plugin registry system.

---

# 15. Ошибка при отсутствующем модуле

После:

```bash
cargo install avin
```

есть только:

```text
avin
```

Если пользователь вызовет:

```bash
avin data sync
```

dispatcher не найдёт `avin-data`.

Желаемый UX:

```text
AVIN Data is not installed.

Cargo:
    cargo install avin --features data

Arch Linux:
    pacman -S avin-data
```

Для начала достаточно generic ошибки:

```text
AVIN module `data` is not installed
```

Красивые package hints можно добавить позже.

---

# 16. Cargo installation model

Cargo и Linux package manager решают разные задачи, поэтому не обязаны устанавливать AVIN одинаково.

## Минимальный AVIN

```bash
cargo install avin
```

Устанавливается:

```text
avin
```

Доступно:

```bash
avin new ...
avin init
```

Плюс dispatcher.

---

## Data module

```bash
cargo install avin --features data
```

Устанавливаются enabled binaries package `avin`:

```text
avin
avin-data
```

Пользователь работает:

```bash
avin data sync
avin instruments cache
```

а не напрямую через:

```bash
avin-data ...
```

---

## Tester

```bash
cargo install avin --features tester
```

Устанавливаются:

```text
avin
avin-tester
```

Публичный интерфейс:

```bash
avin tester ...
```

---

## Полная система

```bash
cargo install avin --features full
```

Feature:

```toml
full = [
    "data",
    "tester",
    "gui",
    ...
]
```

включает все нужные optional dependencies и binary targets.

---

# 17. Можно ли собрать только один binary

Да.

Например только `avin-data`:

```bash
cargo build \
    -p avin \
    --bin avin-data \
    --features data \
    --release
```

Результат:

```text
target/release/avin-data
```

Точно так же:

```bash
cargo build \
    -p avin \
    --bin avin-tester \
    --features tester \
    --release
```

Это важно для системных package managers: им не важно, что несколько executables принадлежат одному Cargo package.

---

# 18. `cargo install avin-data`

В текущей модели такой команды нет.

`avin-data` — binary target package `avin`, а не самостоятельный Cargo package.

Поэтому Cargo installation делается через:

```bash
cargo install avin --features data
```

Это сознательное решение.

Оно гарантирует, что вместе с module binary устанавливается основной dispatcher `avin`.

---

# 19. Rust и Python API

CLI packaging не влияет на библиотечный API.

Rust:

```rust
use avin::prelude::*;
```

Python:

```python
import avin
```

CLI:

```bash
avin data sync
```

Это разные user-facing surfaces поверх общей внутренней функциональности.

---

# 20. Arch Linux / pacman

Системный package manager может иметь настоящую component dependency model.

Желаемые packages:

```text
avin
avin-data
avin-tester
avin-gui
...
```

---

## `avin`

Минимальный package:

```text
/usr/bin/avin
```

Содержит:

- dispatcher;
- `avin new`;
- `avin init`.

Установка:

```bash
pacman -S avin
```

---

## `avin-data`

Package:

```text
avin-data
```

зависит от:

```text
avin
```

и устанавливает module binary.

Например:

```text
/usr/bin/avin-data
```

или:

```text
/usr/lib/avin/avin-data
```

Если используется `/usr/lib/avin/`, dispatcher должен знать system module directory.

Публичный интерфейс всё равно:

```bash
avin data ...
```

Установка:

```bash
pacman -S avin-data
```

автоматически подтягивает `avin`.

После этого:

```bash
avin data sync
```

работает.

---

# 21. Split packages в Arch

Один AVIN source release может породить несколько Arch packages.

Пример:

```bash
pkgbase=avin

pkgname=(
    avin
    avin-data
    avin-tester
)
```

Дальше:

```bash
package_avin() {
    ...
}

package_avin-data() {
    ...
}

package_avin-tester() {
    ...
}
```

Сборка может получать бинарники отдельно:

```bash
cargo build -p avin --bin avin --release
cargo build -p avin --bin avin-data --features data --release
cargo build -p avin --bin avin-tester --features tester --release
```

и раскладывать их по соответствующим system packages.

Отдельный upstream repo/release для каждого модуля не нужен.

---

# 22. Полная система через pacman

Минимальная модель:

```bash
pacman -S avin
```

ставит только dispatcher/core CLI.

Добавление модулей:

```bash
pacman -S avin-data
pacman -S avin-tester
pacman -S avin-gui
```

Если позже понадобится команда «поставить всё», можно добавить meta package, например:

```text
avin-full
```

с dependencies:

```text
avin
avin-data
avin-tester
avin-gui
...
```

Тогда:

```bash
pacman -S avin-full
```

установит всю систему.

Это можно решить позже. Для архитектуры CLI это несущественно.

---

# 23. Что пока не решено

Это оставляется на момент реальной реализации.

## Lookup module binaries

Для Cargo естественно искать sibling binary:

```text
~/.cargo/bin/avin
~/.cargo/bin/avin-data
```

Для system packages может быть:

```text
/usr/bin/avin-data
```

или:

```text
/usr/lib/avin/avin-data
```

Нужно будет сделать небольшой:

```text
find_module_binary(...)
```

и спрятать packaging details внутри dispatcher.

## Command → binary registry

Generic case:

```text
data → avin-data
tester → avin-tester
```

Но для случаев:

```text
data        → avin-data
instruments → avin-data
```

понадобится маленькое mapping.

Не надо проектировать сложную plugin architecture заранее.

## Help

Позже нужно проверить, как лучше объединять:

```bash
avin --help
```

с командами установленных/неустановленных модулей.

На первом этапе dispatcher может показывать только встроенные команды либо небольшой статический список известных модулей.

---

# 24. Итоговая архитектура

```text
                         AVIN CLI source

                     crates/avin/src/cli/
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
       new/init              data               tester
          │                   │                   │
          │                   │                   │
          │          avin-data binary     avin-tester binary
          │                   │                   │
          └───────────────────┴───────────────────┘
                              │
                    lower AVIN libraries
```

Во время выполнения:

```text
                         /usr/bin/avin
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          new/init           data            tester
             │                │                │
          internal       avin-data        avin-tester
                            process           process
```

Основные правила:

1. Весь CLI source code хранится в `crates/avin/src/cli/`.
2. `avin` — верхний user-facing CLI/facade layer.
3. Нижние crates ничего не знают про CLI.
4. `avin data sync` dispatch'ится в `avin-data sync`.
5. `avin-data` вызывает `avin::cli::data::run()`.
6. Features определяют, какой CLI-код и какие dependencies вообще компилируются.
7. `required-features` определяет доступность отдельных binary targets.
8. `cargo install avin` ставит минимальный dispatcher.
9. `cargo install avin --features data` добавляет data functionality.
10. `cargo build -p avin --bin avin-data --features data` позволяет собрать только data binary.
11. `pacman -S avin` ставит dispatcher.
12. `pacman -S avin-data` добавляет data module и зависит от `avin`.
13. Пользовательский CLI всегда остаётся единым: `avin ...`.
14. Если `avin/src/cli/` когда-нибудь реально станет слишком большим, его можно разнести позже без изменения внешнего CLI.

Этого достаточно как исходной архитектуры для начала реализации CLI после завершения текущей работы над storage.

# 25. FIX — dispatcher передаёт дочернему binary полное дерево команды

В предыдущих примерах использовалась упрощённая схема:

```text
avin data sync
    ↓
avin-data sync
```

Для реального AVIN этого недостаточно, потому что один функциональный binary может обслуживать несколько top-level веток CLI.

Например `avin-data` должен владеть обеими ветками:

```bash
avin data sync
avin instruments cache
```

При этом внутри них могут существовать одинаковые имена команд:

```bash
avin data status
avin instruments status
```

Поэтому дочерний binary должен получать **полное дерево команды после `avin`**, включая первый аргумент.

Правильный flow:

```text
avin data sync
```

dispatcher получает:

```text
args = ["data", "sync"]
```

Первый аргумент используется только для выбора binary:

```text
data → avin-data
```

После этого запускается:

```text
avin-data data sync
```

Аналогично:

```text
avin instruments cache
```

dispatcher получает:

```text
args = ["instruments", "cache"]
```

registry:

```text
instruments → avin-data
```

и запускается:

```text
avin-data instruments cache
```

Таким образом dispatcher **не удаляет первый аргумент**. Он использует его для routing, но передаёт дочернему process весь command tree целиком.

Концептуально:

```text
avin <command-tree...>
        ↓
first command
        ↓
command → binary registry
        ↓
child binary <command-tree...>
```

Пример registry:

```text
data        → avin-data
instruments → avin-data
tester      → avin-tester
```

Dispatcher остаётся максимально простым:

```rust
let args = external_args;

let command = args
    .first()
    .ok_or("missing command")?;

let binary = match command.to_str() {
    Some("data") | Some("instruments") => "avin-data",
    Some("tester") => "avin-tester",
    _ => return Err("unknown AVIN command".into()),
};

Command::new(binary)
    .args(args)
    .status()?;
```

А `avin-data` уже самостоятельно парсит своё полное CLI-поддерево:

```text
avin-data
├── data
│   ├── sync
│   └── status
│
└── instruments
    ├── cache
    └── status
```

То есть физический вызов:

```bash
avin-data instruments status
```

в пользовательском интерфейсе остаётся:

```bash
avin instruments status
```

Главный принцип:

> Dispatcher отвечает только за выбор установленного функционального binary.
> Дочерний binary получает полную команду и сам отвечает за её дальнейший разбор.

Это позволяет одному binary обслуживать несколько top-level команд AVIN и не требует уникальности имён команд на более глубоких уровнях CLI.
