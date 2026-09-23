// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use polars::prelude::DataFrame;

use crate::StorageError;

/// Conversion between AVIN domain objects and Polars DataFrames.
pub trait DataFrameExt: Sized {
    /// Converts a slice of domain objects into a DataFrame.
    fn to_df(data: &[Self]) -> Result<DataFrame, StorageError>;

    /// Converts a DataFrame into domain objects.
    fn from_df(df: DataFrame) -> Result<Vec<Self>, StorageError>;
}
