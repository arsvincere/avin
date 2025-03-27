#[derive(Debug)]
pub enum MarketData {
    BAR_1M,
    BAR_D,
}
impl MarketData {
    pub fn name(&self) -> String {
        match self {
            MarketData::BAR_1M => String::from("BAR_1M"),
            MarketData::BAR_D => String::from("BAR_D"),
        }
    }
    pub fn clone(&self) -> Self {
        match self {
            MarketData::BAR_1M => MarketData::BAR_1M,
            MarketData::BAR_D => MarketData::BAR_D,
        }
    }
    pub fn from(s: &String) -> Result<MarketData, &'static str> {
        let t = s.to_lowercase();

        if t == "d" || t == "bar_d" {
            return Ok(MarketData::BAR_D);
        } else if t == "1m" || t == "bar_1m" {
            return Ok(MarketData::BAR_1M);
        }

        return Err("Invalid data type");
    }
}
