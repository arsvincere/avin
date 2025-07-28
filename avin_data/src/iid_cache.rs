use polars::frame::DataFrame;
use source::Source;
use category::Category;

use avin_utils::Cmd;

pub struct IidCache {
    source: Source,
    category: Category,
    iid_df: DataFrame,
}

impl IidCache {

    pub fn load(source: Source, category: Category) -> Self {
        let path = ();
        let df = match Cmd::read_pqt() {
                Ok(df) => df,
                Err(err) => panic!("{err}"),
        }
        Self { source, category, iid_df: df }
    }

    pub fn save(cache: IidCache) {
        let path = ();
        let mut df = cache.df;
        Cmd::write_pqt(&mut df, path);
    }
}