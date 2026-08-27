// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use crate::{InstrumentId, WatchlistGroup};

/// A top-level entry in a [`Watchlist`](crate::Watchlist).
///
/// A watchlist item is either a single instrument or a named group of
/// instruments. This allows a watchlist to contain instruments and groups
/// together while preserving their top-level order.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::{InstrumentId, WatchlistGroup, WatchlistItem};
///
/// let instrument = WatchlistItem::Instrument(
///     InstrumentId::from_str("MOEX.SHARE.SBER").unwrap(),
/// );
///
/// let group = WatchlistItem::Group(
///     WatchlistGroup::new("futures"),
/// );
///
/// assert!(matches!(instrument, WatchlistItem::Instrument(_)));
/// assert!(matches!(group, WatchlistItem::Group(_)));
/// ```
#[derive(Debug, Clone)]
pub enum WatchlistItem {
    Instrument(InstrumentId),
    Group(WatchlistGroup),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn instrument_variant_keeps_instrument() {
        let iid = InstrumentId::from_str("MOEX.SHARE.SBER").unwrap();

        let item = WatchlistItem::Instrument(iid.clone());

        match item {
            WatchlistItem::Instrument(instrument) => {
                assert_eq!(instrument, iid);
            }
            WatchlistItem::Group(_) => {
                panic!("expected instrument");
            }
        }
    }

    #[test]
    fn group_variant_keeps_group() {
        let item = WatchlistItem::Group(WatchlistGroup::new("shares"));

        match item {
            WatchlistItem::Group(group) => {
                assert_eq!(group.name(), "shares");
            }
            WatchlistItem::Instrument(_) => {
                panic!("expected group");
            }
        }
    }
}
