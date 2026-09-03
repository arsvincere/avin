// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use crate::DomainError;

use crate::{InstrumentId, InstrumentInfo};

/// A list of instrument reference data.
///
/// `InstrumentList` stores a collection of `InstrumentInfo` values unique by
/// `InstrumentId`.
///
/// A list cannot contain two instruments with the same `InstrumentId`, even
/// if their remaining metadata differs.
///
/// Instrument order is not part of the list contract.
pub struct InstrumentList {
    instruments: HashMap<InstrumentId, InstrumentInfo>,
}

impl InstrumentList {
    pub fn new() -> Self {
        Self {
            instruments: HashMap::new(),
        }
    }

    /// Returns `true` if the list contains no instruments.
    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    /// Returns the number of instruments in the list.
    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    /// Returns an iterator over instrument reference data.
    pub fn iter(&self) -> impl Iterator<Item = &InstrumentInfo> {
        self.instruments.values()
    }

    /// Adds an instrument to the list.
    ///
    /// # Errors
    ///
    /// Returns an error if the list already contains an instrument with the
    /// same `InstrumentId`.
    pub fn add(
        &mut self,
        instrument: InstrumentInfo,
    ) -> Result<(), DomainError> {
        let iid = instrument.iid();

        match self.instruments.entry(iid) {
            // if not in hash map
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(instrument);
                Ok(())
            }
            // if in hash map
            std::collections::hash_map::Entry::Occupied(entry) => {
                Err(DomainError::Value(format!(
                    "Duplicate InstrumentId in InstrumentList: {}",
                    entry.key()
                )))
            }
        }
    }

    /// Returns instrument information by its `InstrumentId`.
    ///
    /// Returns `None` if the instrument is not present in the list.
    pub fn find(&self, iid: &InstrumentId) -> Option<&InstrumentInfo> {
        self.instruments.get(iid)
    }
}

impl Default for InstrumentList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_info(ticker: &str, name: &str) -> InstrumentInfo {
        let info = HashMap::from([
            ("exchange".to_string(), "MOEX".to_string()),
            ("category".to_string(), "SHARE".to_string()),
            ("ticker".to_string(), ticker.to_string()),
            ("figi".to_string(), "TEST_FIGI".to_string()),
            ("name".to_string(), name.to_string()),
            ("lot".to_string(), "1".to_string()),
            ("step".to_string(), "0.01".to_string()),
        ]);

        InstrumentInfo::new(info).unwrap()
    }

    #[test]
    fn empty_and_len() {
        let mut list = InstrumentList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.add(get_info("SBER", "Сбер Банк")).unwrap();

        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.add(get_info("GAZP", "Газпром")).unwrap();

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn iter() {
        let mut list = InstrumentList::new();
        list.add(get_info("SBER", "Сбер Банк")).unwrap();
        list.add(get_info("GAZP", "Газпром")).unwrap();

        assert_eq!(list.iter().count(), 2);
        assert!(list.iter().any(|info| info.ticker().to_string() == "SBER"));
        assert!(list.iter().any(|info| info.ticker().to_string() == "GAZP"));
    }

    #[test]
    fn add_and_find() {
        let info = get_info("SBER", "Sber Bank");
        let iid = info.iid();

        let mut list = InstrumentList::new();
        list.add(info).unwrap();

        let found = list.find(&iid).unwrap();

        assert_eq!(found.iid(), iid);
        assert_eq!(found.name(), "Sber Bank");
    }

    #[test]
    fn find_missing() {
        let info = get_info("SBER", "Sber Bank");
        let iid = info.iid();

        let list = InstrumentList::new();

        assert!(list.find(&iid).is_none());
    }

    #[test]
    fn duplicate_iid() {
        let first = get_info("SBER", "Sber Bank");
        let second = get_info("SBER", "Сбер Банк");

        assert_eq!(first.iid(), second.iid());

        let mut list = InstrumentList::new();

        let result = list.add(first);

        assert!(result.is_ok());

        let result = list.add(second);

        assert!(matches!(result, Err(DomainError::Value(_))));
    }
}
