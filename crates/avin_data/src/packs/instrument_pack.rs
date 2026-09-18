// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::{Category, DataProvider, Exchange, InstrumentInfo};

use crate::DataError;

/// A non-empty, homogeneous container for instrument reference data.
///
/// Used to transfer instrument reference data from data providers while
/// guaranteeing that all instruments belong to the same provider, exchange,
/// and category.
pub struct InstrumentPack {
    provider: DataProvider,
    exchange: Exchange,
    category: Category,
    instruments: Vec<InstrumentInfo>,
}

impl InstrumentPack {
    /// Creates a pack with the given instruments.
    ///
    /// # Errors
    ///
    /// Returns an error if the instrument list is empty or any instrument
    /// does not match the pack provider, exchange, or category.
    pub fn new(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
        instruments: Vec<InstrumentInfo>,
    ) -> Result<Self, DataError> {
        if instruments.is_empty() {
            let msg = format!(
                "instrument pack [{provider} {exchange} {category}] \
                cannot be empty"
            );
            return Err(DataError::pack(msg, None));
        }

        let mut pack = Self {
            provider,
            exchange,
            category,
            instruments: Vec::with_capacity(instruments.len()),
        };

        for instrument in instruments {
            pack.add(instrument)?;
        }

        Ok(pack)
    }

    /// Returns the data provider.
    pub fn provider(&self) -> DataProvider {
        self.provider
    }

    /// Returns the exchange.
    pub fn exchange(&self) -> Exchange {
        self.exchange
    }

    /// Returns the instrument category.
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the instruments in the pack.
    pub fn instruments(&self) -> &[InstrumentInfo] {
        &self.instruments
    }

    /// Returns the number of instruments in the pack.
    #[allow(clippy::len_without_is_empty)] // pack is always non-empty
    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    /// Adds an instrument to the pack.
    ///
    /// # Errors
    ///
    /// Returns an error if the instrument does not match the pack provider,
    /// exchange, or category.
    pub fn add(
        &mut self,
        instrument: InstrumentInfo,
    ) -> Result<(), DataError> {
        // check provider
        if self.provider != instrument.provider() {
            let msg = format!(
                "instrument provider mismatch: expected {}, got {}",
                self.provider,
                instrument.provider(),
            );
            return Err(DataError::pack(msg, None));
        }

        // check exchange
        if self.exchange != instrument.exchange() {
            let msg = format!(
                "instrument exchange mismatch: expected {}, got {}",
                self.exchange,
                instrument.exchange(),
            );
            return Err(DataError::pack(msg, None));
        }

        // check category
        if self.category != instrument.category() {
            let msg = format!(
                "instrument category mismatch: expected {}, got {}",
                self.category,
                instrument.category(),
            );
            return Err(DataError::pack(msg, None));
        }

        self.instruments.push(instrument);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn instrument(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> InstrumentInfo {
        let info = HashMap::from([
            ("provider".to_string(), provider.key().to_string()),
            ("exchange".to_string(), exchange.key().to_string()),
            ("category".to_string(), category.key().to_string()),
            ("ticker".to_string(), "TEST".to_string()),
            ("name".to_string(), "Test instrument".to_string()),
            ("price_step".to_string(), "0.01".to_string()),
            ("lot_size".to_string(), "1".to_string()),
        ]);

        InstrumentInfo::new_unchecked(info)
    }

    #[test]
    fn new_empty() {
        let result = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            Vec::new(),
        );

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn new_valid() {
        let instruments = vec![
            instrument(DataProvider::TBank, Exchange::Moex, Category::Share),
            instrument(DataProvider::TBank, Exchange::Moex, Category::Share),
        ];

        let pack = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            instruments,
        )
        .unwrap();

        assert_eq!(pack.provider(), DataProvider::TBank);
        assert_eq!(pack.exchange(), Exchange::Moex);
        assert_eq!(pack.category(), Category::Share);
        assert_eq!(pack.len(), 2);
    }

    #[test]
    fn new_provider_mismatch() {
        let instruments = vec![instrument(
            DataProvider::MoexAlgo,
            Exchange::Moex,
            Category::Share,
        )];

        let result = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            instruments,
        );

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn new_exchange_mismatch() {
        let instruments = vec![instrument(
            DataProvider::TBank,
            Exchange::Spb,
            Category::Share,
        )];

        let result = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            instruments,
        );

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn new_category_mismatch() {
        let instruments = vec![instrument(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Future,
        )];

        let result = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            instruments,
        );

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn add() {
        let instruments = vec![
            instrument(DataProvider::TBank, Exchange::Moex, Category::Share),
            instrument(DataProvider::TBank, Exchange::Moex, Category::Share),
        ];

        let mut pack = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
            instruments,
        )
        .unwrap();

        let result = pack.add(instrument(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
        ));

        assert!(result.is_ok());
        assert_eq!(pack.len(), 3);
    }
}
