// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::{Category, DataProvider, Exchange, InstrumentInfo};

use crate::DataError;

pub struct InstrumentPack {
    provider: DataProvider,
    exchange: Exchange,
    category: Category,
    instruments: Vec<InstrumentInfo>,
}

impl InstrumentPack {
    pub fn new(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Self {
        Self {
            provider,
            exchange,
            category,
            instruments: Vec::new(),
        }
    }

    pub fn provider(&self) -> DataProvider {
        self.provider
    }

    pub fn exchange(&self) -> Exchange {
        self.exchange
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn instruments(&self) -> &[InstrumentInfo] {
        &self.instruments
    }

    /// Returns `true` if the pack contains no instruments.
    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    /// Returns the number of instruments in the pack.
    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    /// Adds an instrument to the pack.
    ///
    /// # Errors
    ///
    /// Returns an error if ...
    pub fn add(
        &mut self,
        instrument: InstrumentInfo,
    ) -> Result<(), DataError> {
        // TODO: ???
        // а не добавить ли в InstrumentInfo еще и provider? и его тоже чекать?

        // check exchange
        if self.exchange != instrument.exchange() {
            return Err(DataError::TmpError {
                message: "TODO msg".to_string(),
                source: None,
            });
        }

        // check category
        if self.category != instrument.category() {
            return Err(DataError::TmpError {
                message: "TODO msg".to_string(),
                source: None,
            });
        }

        self.instruments.push(instrument);

        Ok(())
    }
}
