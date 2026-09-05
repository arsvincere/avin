# Watchlist / Universe — текущий дизайн

## Базовое разделение

### Watchlist

Пользовательская persistent-коллекция инструментов.

Хранит идентификаторы, а не runtime market objects.

```rust
pub struct Watchlist {
    name: String,
    items: Vec<InstrumentId>,
}
```

Будущие обязанности:

- пользовательский порядок инструментов;
- произвольные пользовательские группы;
- persistence;
- возможно UI-состояние вроде collapsed/expanded групп, заметок, favorites.

Группировка задается пользователем и не обязана соответствовать доменным категориям вроде `Share / Future / Index`.

`Watchlist` отвечает на вопрос:

> Какие инструменты пользователь хочет видеть / использовать?

Связь с runtime-объектами идет через `InstrumentId`.

---

### Universe

Runtime/application state с материализованными рыночными объектами.

Концептуально:

```rust
pub struct Universe {
    assets: Vec<Asset>,
    indices: Vec<Index>,
}
```

Точное имя и расположение пока не финальные. В будущем сущность может получить более техническое/internal имя.

Обязанности:

- хранить runtime-объекты, уже загруженные в память;
- lazy population;
- lookup по `InstrumentId`;
- давать общий runtime context для GUI и стратегий;
- получать и обновлять market data в realtime и backtest.

`Universe` отвечает на вопрос:

> Какие runtime market objects сейчас доступны в текущей session/run?

Скорее всего это **не domain entity**. По смыслу она ближе к service/application/runtime state.

---

## Связь Watchlist и Universe

```text
Watchlist
    │
    │ InstrumentId
    ▼
Universe
    │
    ├── Asset
    │   ├── Share
    │   ├── Future
    │   ├── Bond
    │   └── ...
    │
    └── Index
```

Типичный GUI flow:

```text
загрузить Watchlist
    ↓
показать InstrumentId в UI
    ↓
пользователь выбирает инструмент
    ↓
service загружает / materialize runtime object
    ↓
Universe пополняется и кеширует объект
    ↓
GUI работает с runtime object
```

Переключение watchlist не обязано пересоздавать или очищать `Universe`.

Политика очистки кеша / eviction пока не проектируется.

---

## Использование в стратегии

Стратегии могут быть нужны и tradable assets, и reference market entities вроде индексов.

Runtime context может содержать оба типа:

```text
Asset
Index
```

Но event processing остается типобезопасным:

```rust
fn process(&mut self, asset: &Asset)
```

`Index` не должен попадать в `process(&Asset)`.

Reference data стратегия может получать отдельно из runtime context:

```rust
universe.index(&iid)
```

или через эквивалентный будущий API.

---

## Asset остается без изменений

`Asset` представляет только tradable runtime objects:

```rust
pub enum Asset {
    Share(Share),
    Future(Future),
    // Bond, Option, ETF...
}
```

Поэтому общие capabilities остаются валидными:

```rust
impl HasCharts for Asset
impl HasTicks for Asset
impl HasFootprints for Asset
```

`Index` остается вне `Asset`, потому что у него другой набор capabilities.

---

## Универсальный wrapper для mixed runtime objects

Обертка вида:

```rust
enum UniverseItem {
    Asset(Asset),
    Index(Index),
}
```

**пока не нужна**.

Его стоит вводить только если появится реальный caller, которому нужен generic lookup:

```rust
universe.find(&iid) -> Option<&UniverseItem>
```

Возможно, typed lookup полностью уберет необходимость в такой обертке:

```rust
universe.asset(&iid)
universe.share(&iid)
universe.future(&iid)
universe.index(&iid)
```

Также возможен capability-oriented API:

```rust
universe.chart(&iid, tf)
```

---

## AssetList

Предыдущая модель:

```rust
AssetList {
    assets: Vec<Asset>,
}
```

больше не рассматривается как финальная архитектура.

Причины:

- естественно не содержит `Index`;
- стратегии могут использовать индексы как reference series;
- GUI runtime state тоже должен работать и с tradable, и с non-tradable market entities;
- отдельные глобальные сущности вроде `Market.get_index()` или `Exchange.get_index()` выглядят менее естественно, чем единый runtime context.

Текущее направление:

```text
AssetList → в будущем не нужен
Watchlist → persistent user selection
Universe / runtime state → материализованные market objects
```

Сохранять `AssetList` внутри `Universe` как отдельный тип нет смысла, если у `AssetList` не появится собственная независимая ответственность.

---

## Portfolio

Отдельная задача.

Концептуально может содержать tradable positions и cash:

```rust
Portfolio {
    shares: ...,
    futures: ...,
    cash: Vec<Currency>,
}
```

`Index` в Portfolio не входит.

Дизайн Portfolio пока отложен.
