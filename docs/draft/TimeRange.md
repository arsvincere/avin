# TimeRange

`TimeRange` — доменный тип для представления временного диапазона AVIN.

Основная цель — не плодить отдельные методы вроде `load_year`, `load_month`, `load_day` в высокоуровневом API. Варианты задания периода должны инкапсулироваться самим `TimeRange`.

## Основной API

```rust
TimeRange::new(begin, end)
TimeRange::year(year)
```

Пример:

```rust
Data::load(
    "moex.share.sber",
    MarketData::Tick,
    TimeRange::year(2025),
)?;
```

Вместо:

```rust
Data::load_year(...)
Data::load_month(...)
Data::load_day(...)
```

## Семантика

Диапазон должен использовать единую семантику:

```text
[begin, end)
```

- `begin` включён;
- `end` не включён.

Это позволяет однозначно стыковать соседние периоды без пересечений.

## Возможное расширение

Добавлять convenience-конструкторы только при появлении реальных сценариев:

```rust
TimeRange::month(year, month)
TimeRange::day(year, month, day)
```

Не создавать отдельные конструкторы заранее без необходимости.

## Идея границы API

`Data` и другие высокоуровневые сервисы работают с одним универсальным `TimeRange`:

```rust
Data::load(code, market_data, range)
```

А логика формирования диапазона за год, месяц, день или произвольный период остаётся внутри `TimeRange`.
