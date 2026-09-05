# Суть
- возможно текущий InstrumentInfo - никуда не годится.
  И нужны конкретные ShareInfo, FutureInfo...
- возможно нужен общий враппер над ShareInfo, FutureInfo...
  а возможно и не нужен вообще, будет зависеть от того
  как будет устроен avin_data. Конечному потребителю
  Share Future - врапперы не нужны, он будет брать типизированный
  объект.
- в целом при переходе на ShareInfo, FutureInfo - этот InstrumentInfo все
  legacy код который надо выпиливать и адаптировать Share Future
  Asset к новым реалиям.
- отдельный вопрос InstrumentList - как контейнер для передачи
  от avin_data -> service -> storage. Сейчас не делает главного
  не проверяет что внутри только Share или только Future, а
  именно для этого он и нужен... иначе вообще не нужен... можно
  просто Vec<ShareInfo> инвозвращать... Ну плюс проверка уникальности
  нужна... возможное решение InstrumentList::new(category: Category).
- отдельный вопрос - InstrumentInfoView трейт... Что ему теперь показывать?
  exchange, category, ticker, iid, name... и все? больше он ничего
  гарантировать не может.. а в таком виде он бесполезен без lot() step()
  или делать обобщенный price_step quantity_step который подходит для
  любого инструмента но криво звучит. В целом - это онли эргономик, и пока
  его проще всего просто выпилить. А добавить когда уже устоится реально
  устойчивая картинка общего чего-то.

# ShareInfo

```rust
// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use avin_core::Quantity;

use crate::{Category, DomainError, Exchange, InstrumentId, Ticker};

// TODO: docstring & tests

#[derive(Debug, Clone)]
pub struct ShareInfo {
    iid: InstrumentId,
    name: String,
    lot: Quantity,
    step: f64,
    extra: HashMap<String, String>,
}

impl ShareInfo {
    pub fn new(
        exchange: Exchange,
        ticker: Ticker,
        name: String,
        lot: Quantity,
        step: f64,
        extra: HashMap<String, String>,
    ) -> Result<Self, DomainError> {
        let iid = InstrumentId::new(exchange, Category::Share, ticker);

        if name.trim().is_empty() {
            return Err(DomainError::ShareInfo(
                "missing value for name".to_string(),
            ));
        }

        if lot.is_zero() {
            return Err(DomainError::ShareInfo(
                "lot must be greater than zero".to_string(),
            ));
        }

        if !step.is_finite() || step <= 0.0 {
            let err = DomainError::ShareInfo(
                "step must be finite and greater than zero".to_string(),
            );
            return Err(err);
        }

        Ok(Self {
            iid,
            name,
            lot,
            step,
            extra,
        })
    }

    pub fn iid(&self) -> &InstrumentId {
        &self.iid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lot(&self) -> Quantity {
        self.lot
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn extra(&self) -> &HashMap<String, String> {
        &self.extra
    }
}
```

# FutureInfo

```rust
// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use avin_core::Quantity;

use crate::{Category, DomainError, Exchange, InstrumentId, Ticker};

// TODO: docstring & tests

#[derive(Debug, Clone)]
pub struct FutureInfo {
    iid: InstrumentId,
    name: String,
    lot: Quantity,
    step: f64,
    extra: HashMap<String, String>,
}

impl FutureInfo {
    pub fn new(
        exchange: Exchange,
        ticker: Ticker,
        name: String,
        lot: Quantity,
        step: f64,
        extra: HashMap<String, String>,
    ) -> Result<Self, DomainError> {
        let iid = InstrumentId::new(exchange, Category::Share, ticker);

        if name.trim().is_empty() {
            return Err(DomainError::FutureInfo(
                "missing value for name".to_string(),
            ));
        }

        if lot.is_zero() {
            return Err(DomainError::FutureInfo(
                "lot must be greater than zero".to_string(),
            ));
        }

        if !step.is_finite() || step <= 0.0 {
            let err = DomainError::FutureInfo(
                "step must be finite and greater than zero".to_string(),
            );
            return Err(err);
        }

        Ok(Self {
            iid,
            name,
            lot,
            step,
            extra,
        })
    }

    pub fn iid(&self) -> &InstrumentId {
        &self.iid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lot(&self) -> Quantity {
        self.lot
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn extra(&self) -> &HashMap<String, String> {
        &self.extra
    }
}
```
